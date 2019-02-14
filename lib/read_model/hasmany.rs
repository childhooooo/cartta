use syn::{DeriveInput, Ident, MetaList, NestedMeta};
use syn::Meta::Word;
use std::iter::FromIterator;

pub struct HasMany {
    relation: String,
    type: Ident,
    schema_to: Ident,
    sync: bool,
    field_order: Option<Ident>,
    order: Option<Ident>
}

impl HasMany {
    pub fn new (metalist: MetaList) -> Self {
        let type = metalist.ident.to_string();
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

        if args.len() < 3 {
            unreachable!()
        } else if args.len() < 4 {
            HasMany {
                type: type,
                schema_to: args[0].to_owned(),
                model_relation: args[1].to_owned(),
                schema_to: args[3].to_owned(),
                sync: match args[4].to_string().to_str() {
                    "sync" => true,
                    "async" => false
                },
                field_order: None,
                order: None
            }
        } else {
            HasMany {
                type: type,
                schema_to: args[0].to_owned(),
                schema_relation: args[1].to_owned(),
                field_from: args[2].to_owned(),
                field_to: args[3].to_owned(),
                sync: match args[4].to_string().to_str() {
                    "sync" => true,
                    "async" => false
                },
                field_order: Some(args[5].to_owned()),
                order: Some(args[6].to_owned())
            }
        };
    }
}