use syn::{Ident, MetaList, NestedMeta};
use syn::Meta::{List, Word};
use std::iter::FromIterator;

pub fn get_values (metalist: MetaList) -> Vec<Ident> {
    metalist.nested
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
    .collect()
}

pub fn get_value (metalist: MetaList) -> Option<Ident> {
    metalist.nested
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
    .last()
}

pub fn get_lists (metalist: MetaList) -> Vec<MetaList> {
    metalist.nested
    .into_iter()
    .map(|nestedmeta| {
        match nestedmeta {
            NestedMeta::Meta(meta) => {
                match meta {
                    List(metalist) => metalist,
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    })
    .collect()
}