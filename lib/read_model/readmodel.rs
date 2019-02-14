use proc_macro2::TokenStream;
use syn::{Ident, MetaList, NestedMeta};
use syn::Meta::{List, Word};
use std::iter::FromIterator;

use crate::helpers::*;
use crate::hasmany::HasMany;

pub struct ReadModel {
    pub name: Ident,
    pub schema: Ident,
    pub error: Ident,
    pub strs: Vec<Ident>,
    pub i32s: Vec<Ident>,
    pub asc: Option<Ident>,
    pub desc: Option<Ident>,
    pub belongs_to = Vec<Ident>,
    pub has_many = Vec<HasMany>
}

impl ReadModel {
    pub fn gen_get () -> TokenStream {
    }

    pub fn gen_all () -> TokenStream {
    }

    pub fn gen_i32s () -> TokenStream {
    }

    pub fn gen_strs () -> TokenStream {
    }

    pub fn gen_has_many () -> TokenStream {
    }

    fn get (&self) -> TokenStream {
        let name = self.name;
        let error = self.error;
        let schema = self.schema;

        if self.sync {
            let query_sync =
                self.has_many
                .iter()
                .map(|hm| {

                });
        }

        quote! {
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
        }
    }

    pub fn new (input: DeriveInput) -> Self {
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
        let mut strs = vec![];
        let mut i32s = vec![];
        let mut asc = None;
        let mut desc = None;
        let mut belongs_to = vec![];
        let mut has_many = vec![];

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
                                strs = get_values(metalist);
                            },
                            "i32s" => {
                                i32s = get_values(metalist);
                            },
                            "asc" => {
                                asc = get_value(metalist);
                            },
                            "desc" => {
                                desc = get_value(metalist);
                            },
                            "belongs_to" => {
                                belongs_to = get_values(metalist);
                            },
                            "has_many" => {
                                has_many =
                                    get_lists(metalist)
                                    .into_iter()
                                    .map(|ml| HasMany::new(ml))
                                    .collect();
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

        Self {
            name: name,
            schema: schema,
            error: error,
            strs: strs,
            i32s: i32s,
            asc: asc,
            desc: desc,
            belongs_to: belongs_to,
            has_many: has_many
        }
    }
}






impl<'a, Parent: 'a, Child, Iter> GroupedBy<'a, Parent> for Iter
where
    Iter: IntoIterator<Item = (Child, Identifiable)>,
    Child: BelongsTo<Parent>,
    &'a Parent: Identifiable,
    Id<&'a Parent>: Borrow<Child::ForeignKey>,
{
    fn grouped_by(self, parents: &'a [Parent]) -> Vec<Vec<Child>> {
        use std::collections::HashMap;

        let id_indices: HashMap<_, _> = parents
            .iter()
            .enumerate()
            .map(|(i, u)| (u.id(), i))
            .collect();
        let mut result = parents.iter().map(|_| Vec::new()).collect::<Vec<_>>();
        for (child, value) in self {
            if let Some(index) = child.foreign_key().map(|i| id_indices[i]) {
                result[index].push(value);
            }
        }
        result
    }
}

preload() {
    let mes =
        mes::table
        .load();


    let relation_tags: Vec<Vec<Other>> =
        Relation::belonging_to(&mes)
        .inner_join(others::table)
        .load::<(Relation, Other)>()
        .grouped_by(&mes);


    let result: Vec<Me, Vec<(Relation, Other)>> =
        mes
        .into_iter()
        .zip(relation_tags)
        .collect();
}

preload() {
    let mes =
        mes::table
        .load();
        
    let others =
        Other::belonging_to(&mes)
        .load::<Other>(conn)
        .grouped_by(&mes);

    let result =
        mes
        .into_iter()
        .zip(others)
        .collect();
}


fn all_from<T> (
    to: &T,
    conn: &PgConnection
) -> Vec<#name> {
    #name::belonging_to(to)
    #order
    .load::<#name>(conn)
}

fn gen_sync (self) -> TokenStream {
    let iter =
        self.hasmany
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let preload = gen_preload(i, h);
            quote! { #preload }
        });
    let push = TokenStream::from_iter(iter);

    let name = self.name;
    let mut synced_types: Ident;
    quote! {
        fn sync (
            self: Vec<#name>,
            conn: &PgConnection
        ) -> Vec<#synced_types> {
            let mut manys = vec![];
            #push

            let mut data = 
                self
                .into_iter()
                .enumerate()
                .map(|(i, d)| (i, d, vec![Vec::new(); manys.len()]));

            for (i, m) in manys.into_iter().enumerate() {
                data
                .map(|(j, d, v)| (j, d, v[i].push(m[j])))
                .collect();
            }

            data
        }
    }
}

fn gen_preload (index: &i32, many: &HasMany) -> TokenStream {
    let num = proc_macro2::ident::new(&index.to_string(), proc_macro2::span::call_site());
    match many.type.as_str() {
        "many_to_many" -> {
            quote! {
                manys.push(
                    #model_relation::belonging_to(self)
                    .inner_join(#schema_to::table)
                    .load::<(#model_relation, #model_to)>(conn)
                    .grouped_by(self)
                );
            }
        },
        "belongs_to" -> {
            quote! {
                manys.push(
                    #model_other::belonging_to(self)
                    .load::<#model_to>(conn)
                    .grouped_by(self)
                );
            }
        },
        _ => unreachable!();
    }
}

/*
usage:
users = User::all_from_Room(&room).sync();
*/