//! # Example
//! ```
//! #[derive(ReadModel)]
//! #[readmodel(
//!     schema(users),
//!     databaseerror(MyError::DatabaseError),
//!     strs(name),
//!     i32s(age),
//!     desc(created_at),
//!     belongs_to(Room),
//!     has_many(
//!         User(users, Relation, name, asc),
//!         Post(posts, created_at, desc)
//!     )
//! )]
//! #[hasmany(
//!     schema(users),
//!     fields(
//!         User(users, Relation),
//!         Post(posts)
//!     )
//! )]
//! pub struct User {
//!     pub id: i32,
//!     pub name: String,
//!     pub age: i32,
//!     pub created_at: Timestamp
//! }
//! 
//! pub struct MyError {
//!     DatabaseError(diesel::result::Error)
//! }
//! 
//! let room = ...;
//! let conn = ...;
//! let users_found = User::find_by_name("hoge", Some(&room), &conn).unwrap();
//! ```

#![recursion_limit = "1024"]

#[macro_use] extern crate quote;
#[macro_use] extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;

mod readmodel;
mod hasmany;
mod helpers;

use proc_macro2::TokenStream;
use syn::{DeriveInput, Ident, MetaList, NestedMeta};
use syn::Meta::{List, Word};
use std::iter::FromIterator;

use readmodel::ReadModel;

#[proc_macro_derive(ReadModel, attributes(readmodel, belongs_to))]
pub fn readmodel (input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_readmodel(input).into()
}

pub fn main (input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let readmodel = ReadModel::new(input);

    let functions_get = readmodel.gen_get();
    let functions_all = readmodel.gen_all();
    let functions_find = readmodel.gen_i32s();
    let functions_search = readmodel.gen_strs();
    let functions_get_many = readmodel.gen_has_many();

    let token_stream_2 = quote! {
        impl #name {
            #functions_get
            #functions_all
            #functions_find
            #functions_search
            #functions_get_many
        }
    }

    token_stream_2.into()
}

fn impl_readmodel (input: DeriveInput) -> TokenStream {
    let name = input.ident;

    let nestedmetas =
        input.attrs
        .into_iter()
        .map(|attr| {
            match attr.parse_meta().unwrap() {
                List(metalist) => metalist.nested.into_iter(),
                _ => unreachable!()
            }
        })
        .flatten(); //Iter<NestedMeta>

    let mut schema = None;
    let mut error = None;
    let mut strs = None;
    let mut i32s = None;
    let mut asc = None;
    let mut desc = None;
    let mut belongs_to = None;
    let mut has_many = None;

    for nestedmeta in nestedmetas {
        if let NestedMeta::Meta(meta) = nestedmeta {
            match meta {
                List(metalist) => {
                    match metalist.ident.to_string().as_str() {
                        "schema" => {
                            schema = get_value(metalist);
                        },
                        "error" => {
                            error = get_value(metalist);
                        },
                        "strs" => {
                            strs = Some(get_values(metalist));
                        },
                        "i32s" => {
                            i32s = Some(get_values(metalist));
                        },
                        "asc" => {
                            asc = get_value(metalist);
                        },
                        "desc" => {
                            desc = get_value(metalist);
                        },
                        "belongs_to" => {
                            belongs_to = Some(get_values(metalist));
                        },
                        "has_many" => {
                            has_many = Some(get_lists(metalist));
                        },
                        _ => {
                            unreachable!();
                        }
                    }
                },
                _ => {}
            }
        }
    }

    let schema = schema.unwrap();
    let error = error.unwrap();

    let fns_get = gen_get(&name, &schema, &error, asc.as_ref(), desc.as_ref(), belongs_to.as_ref());

    let fns_str = match strs.as_ref() {
        Some(ref strs) => gen_strs(strs, &name, &schema, &error, asc.as_ref(), desc.as_ref(), belongs_to.as_ref()),
        _ => quote! {}
    };

    let fns_i32 = match i32s.as_ref() {
        Some(ref i32s) => gen_i32s(i32s, &name, &schema, &error, asc.as_ref(), desc.as_ref(), belongs_to.as_ref()),
        _ => quote! {}
    };

    let fns_search_str = match strs.as_ref() {
        Some(ref strs) => gen_search_strs(strs, &name, &schema, &error, asc.as_ref(), desc.as_ref(), belongs_to.as_ref()),
        _ => quote! {}
    };

    let fns_has_many = match has_many.as_ref() {
        Some(ref has_many) => gen_has_many(has_many, &error),
        _ => quote! {}
    }; 

    quote! {
        impl #name {
            #fns_get
            #fns_str
            #fns_i32
            #fns_search_str
            #fns_has_many
        }
    }
}

fn gen_strs (
    fields: &Vec<Ident>,
    name: &Ident,
    schema: &Ident,
    error: &Ident,
    asc: Option<&Ident>,
    desc: Option<&Ident>,
    belongs_to: Option<&Vec<Ident>>
) -> TokenStream {
    let iter =
        fields
        .to_owned()
        .into_iter()
        .map(|field| {
            let mut order = quote! {};
            if let Some(f) = asc {
                order = quote! { .order(#schema::#f.asc()) }
            };
            if let Some(f) = desc {
                order = quote! { .order(#schema::#f.desc()) }
            };

            let fnname = proc_macro2::Ident::new(&format!("find_by_{}", &field), proc_macro2::Span::call_site());
            let findby = quote! {
                pub fn #fnname (
                    value: &str,
                    conn: &PgConnection
                ) -> Result<Vec<#name>, #error> {
                    #schema::table
                    .filter(#schema::#field.eq(value))
                    #order
                    .load::<#name>(conn)
                    .map_err(#error::DatabaseError)
                }
            };

            let findbywith = match belongs_to.to_owned() {
                Some(belongs_to) => {
                    let iter =
                        belongs_to
                        .into_iter()
                        .map(|b| {
                            let fnname = proc_macro2::Ident::new(&format!("find_by_{}_with_{}", &field, &b), proc_macro2::Span::call_site());
                            quote! {
                                pub fn #fnname (
                                    with: &#b,
                                    value: &str,
                                    conn: &PgConnection
                                ) -> Result<Vec<#name>, #error> {
                                    #name::belonging_to(with)
                                    #order
                                    .filter(#schema::#field.eq(value))
                                    .load::<#name>(conn)
                                    .map_err(#error::DatabaseError)
                                }
                            }
                        });
                    TokenStream::from_iter(iter)
                },
                None => quote! {}
            };

            quote! {
                #findby
                #findbywith
            }
        });

    TokenStream::from_iter(iter)
}

fn gen_i32s (
    fields: &Vec<Ident>,
    name: &Ident,
    schema: &Ident,
    error: &Ident,
    asc: Option<&Ident>,
    desc: Option<&Ident>,
    belongs_to: Option<&Vec<Ident>>
) -> TokenStream {
    let iter =
        fields
        .to_owned()
        .into_iter()
        .map(|field| {
            let mut order = quote! {};
            if let Some(f) = asc {
                order = quote! { .order(#schema::#f.asc()) }
            };
            if let Some(f) = desc {
                order = quote! { .order(#schema::#f.desc()) }
            };

            let fnname = proc_macro2::Ident::new(&format!("find_by_{}", &field), proc_macro2::Span::call_site());
            let findby = quote! {
                pub fn #fnname (
                    value: &i32,
                    conn: &PgConnection
                ) -> Result<Vec<#name>, #error> {
                    #schema::table
                    .filter(#schema::#field.eq(value))
                    #order
                    .load::<#name>(conn)
                    .map_err(#error::DatabaseError)
                }
            };

            let findbywith = match belongs_to.to_owned() {
                Some(belongs_to) => {
                    let iter =
                        belongs_to
                        .into_iter()
                        .map(|b| {
                            let fnname = proc_macro2::Ident::new(&format!("find_by_{}_with_{}", &field, &b), proc_macro2::Span::call_site());
                            quote! {
                                pub fn #fnname (
                                    with: &#b,
                                    value: &i32,
                                    conn: &PgConnection
                                ) -> Result<Vec<#name>, #error> {
                                    #name::belonging_to(with)
                                    #order
                                    .filter(#schema::#field.eq(value))
                                    .load::<#name>(conn)
                                    .map_err(#error::DatabaseError)
                                }
                            }
                        });
                    TokenStream::from_iter(iter)
                },
                None => quote! {}
            };

            quote! {
                #findby
                #findbywith
            }
        });

    TokenStream::from_iter(iter)
}

fn gen_search_strs (
    fields: &Vec<Ident>,
    name: &Ident,
    schema: &Ident,
    error: &Ident,
    asc: Option<&Ident>,
    desc: Option<&Ident>,
    belongs_to: Option<&Vec<Ident>>
) -> TokenStream {
    let iter =
        fields
        .to_owned()
        .into_iter()
        .map(|field| {
            let mut order = quote! {};
            if let Some(f) = asc {
                order = quote! { .order(#schema::#f.asc()) }
            };
            if let Some(f) = desc {
                order = quote! { .order(#schema::#f.desc()) }
            };

            let fnname = proc_macro2::ident::new(&format!("search_by_{}", &field), proc_macro2::span::call_site());
            let searchby = quote! {
                pub fn #fnname (
                    value: &str,
                    conn: &PgConnection
                ) -> Result<Vec<#name>, #error> {
                    let value = format!("%{}%", value);
                    #schema::table
                    .filter(#schema::#field.like(value))
                    #order
                    .load::<#name>(conn)
                    .map_err(#error::DatabaseError)
                }
            };

            let searchbywith = match belongs_to.to_owned() {
                Some(belongs_to) => {
                    let iter =
                        belongs_to
                        .into_iter()
                        .map(|b| {
                            let fnname = proc_macro2::Ident::new(&format!("search_by_{}_with_{}", &field, &b), proc_macro2::Span::call_site());
                            quote! {
                                pub fn #fnname(
                                    with: &#b,
                                    value: &str,
                                    conn: &PgConnection
                                ) -> Result<Vec<#name>, #error> {
                                    let value = format!("%{}%", value);
                                    #name::belonging_to(with)
                                    #order
                                    .filter(#schema::#field.like(value))
                                    .load::<#name>(conn)
                                    .map_err(#error::DatabaseError)
                                }
                            }
                        });
                    TokenStream::from_iter(iter)
                },
                None => quote! {}
            };

            quote! {
                #searchby
                #searchbywith
            }
        });

    TokenStream::from_iter(iter)
}

fn gen_get (
    name: &Ident,
    schema: &Ident,
    error: &Ident,
    asc: Option<&Ident>,
    desc: Option<&Ident>,
    belongs_to: Option<&Vec<Ident>>
) -> TokenStream {
    let mut order = quote! {};
    if let Some(f) = asc {
        order = quote! { .order(#schema::#f.asc()) }
    };
    if let Some(f) = desc {
        order = quote! { .order(#schema::#f.desc()) }
    };

    let get = quote! {
        pub fn get (
            id: &i32,
            conn: &PgConnection
        ) -> Result<#name, #error> {
            let res =
                #schema::table
                .find(id)
                .first::<#name>(conn)
                .optional()
                .map_err(#error::DatabaseError)?;

            res.ok_or(#error::NotFound)
        }
    };

    let all = quote! {
        pub fn all(
            conn: &PgConnection
        ) -> Result<Vec<#name>, #error> {
            #schema::table
            #order
            .load::<#name>(conn)
            .map_err(#error::DatabaseError)
        }
    };

    let all_from = match belongs_to.to_owned() {
        Some(belongs_to) => {
            let iter =
                belongs_to
                .into_iter()
                .map(|b| {
                    let fnname = proc_macro2::Ident::new(&format!("all_from_{}", &b), proc_macro2::Span::call_site());
                    quote! {
                        pub fn #fnname (
                            with: &#b,
                            conn: &PgConnection
                        ) -> Result<Vec<#name>, #error> {
                            #name::belonging_to(with)
                            #order
                            .load::<#name>(conn)
                            .map_err(#error::DatabaseError)
                        }
                    }
                });

            TokenStream::from_iter(iter)
        },
        None => quote! {}
    };

    quote! {
        #get
        #all
        #all_from
    }
}

fn gen_has_many (
    has_many: &Vec<MetaList>,
    error: &Ident
) -> TokenStream {
    let iter =
        has_many
        .to_owned()
        .into_iter()
        .map(|h| { //MetaList
            let type_return = h.ident;
            let args: Vec<Ident> =
                h.nested
                .into_iter()
                .map(|nestedmeta| {
                    match nestedmeta {
                        NestedMeta::Meta(meta) => {
                            match meta {
                                Word(ident) => ident,
                                _ => unreachable!()
                            }
                        },
                        _ => unreachable!()
                    }
                })
                .collect();

            let (table_to, table_relation, field_from, field_to, field_order, order) = if args.len() < 4 {
                unreachable!()
            } else if args.len() < 6 {
                (args[0].to_owned(), args[1].to_owned(), args[2].to_owned(), args[3].to_owned(), None, None)
            } else {
                (args[0].to_owned(), args[1].to_owned(), args[2].to_owned(), args[3].to_owned(), Some(args[4].to_owned()), Some(args[5].to_owned()))
            };

            let order = match order {
                Some(o) => {
                    let field = field_order.unwrap();
                    quote! { .order(#table_to::#field.#o()) }
                },
                None => quote! {}
            };

            let fnname = proc_macro2::Ident::new(&format!("get_{}s", &type_return), proc_macro2::Span::call_site());
            quote! {
                pub fn #fnname (
                    id: &i32,
                    conn: &PgConnection
                ) -> Result<Vec<#type_return>, #error> {
                    use diesel::pg::expression::dsl::any;
                    let to_ids =
                        #table_relation::table
                        .filter(#table_relation::#field_from.eq(id))
                        .select(#table_relation::#field_to);

                    #table_to::table
                    #order
                    .filter(#table_to::id.eq(any(to_ids)))
                    .load::<#type_return>(conn)
                    .map_err(#error::DatabaseError)
                }
            }
        });

    TokenStream::from_iter(iter)
}