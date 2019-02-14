use syn::{DeriveInput, Ident, MetaList, NestedMeta};
use syn::Meta::Word;
use std::iter::FromIterator;

pub struct HasMany {
    name: Ident,
    schema: Ident,
    model: Ident,
    through: Option<Ident>
}

impl HasMany {
    pub fn new (metalist: MetaList) -> Self {
        let name = metalist.ident;
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

        if args.len() == 2 {
            HasMany {
                name: name,
                schema: args[0],
                model: args[1],
                through: None
            }
        } else if args.len() == 3 {
            HasMany {
                name: name,
                schema: args[0],
                model: args[1],
                through: Some(args[2])
            }
        } else {
            unreachable!();
        };
    }

    pub fn preload (self, parent: &Ident) -> TokenStream {
        let name_target = self.name;
        let model_target = self.model;
        let schema_target = self.schema;
        if let Some(through) = self.through {
            quote! {
                impl<T> #parent {
                    pub fn load_#name_target (
                        base: Vec<#parent>,
                        conn: &PgConnection
                    ) -> Vec<(#parent, Vec<#model_target>)> {
                        #through::belonging_to(&base)
                        .inner_join(#schema_target::table)
                        .load::<(#through, #model_target)>(conn)
                        .grouped_by(&base)
                    }

                    pub fn load_#name_target (
                        base: Vec<(#parent, vec<T>)>,
                        conn: &PgConnection
                    ) -> Vec<(#parent, Vec<T>, Vec<#model_target>)> {
                        let res =
                            #through::belonging_to(&base)
                            .inner_join(#schema_target::table)
                            .load::<(#through, #model_target)>(conn)
                            .grouped_by(base);

                        base
                        .into_iter()
                        .enumerate()
                        .map(|i, b| { (b.0, b.1, res[i].1) })
                        .collect()
                    }
                }
            }
        } else {
            quote! {
                impl<T> #parent {
                    pub fn load_#name_target (
                        base: Vec<#parent>,
                        conn: &PgConnection
                    ) -> Vec<(#parent, Vec<#model_target>)> {
                        #model_target::belonging_to(&base)
                        .load::<#model_target>(conn)
                        .grouped_by(base)
                    }

                    pub fn load_#name_target (
                        base: Vec<(#parent, vec<T>)>,
                        conn: &PgConnection
                    ) -> Vec<(#parent, Vec<T>, Vec<#model_target>)> {
                        let res =
                            #model_target::belonging_to(&base)
                            .load::<#model_target>(conn)
                            .grouped_by(base)

                        base
                        .into_iter()
                        .enumerate()
                        .map(|i, b| { (b.0, b.1, res[i].1) })
                        .collect()
                    }
                }
            }
        }
    }
}