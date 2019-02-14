use proc_macro2::TokenStream;
use syn::{Ident, MetaList, NestedMeta};
use syn::Meta::{List, Word};
use std::iter::FromIterator;

use crate::helpers::*;
use crate::hasmany::HasMany;

pub struct Preload {
    pub name: Ident,
    pub has_many = Vec<HasMany>
}

impl ReadModel {
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

        let has_many =
            nestedmetas
            .into_iter()
            .map(|nestedmeta| {
                if let NestedMeta::Meta(List(metalist)) = nestedmeta {
                    HasMany::new(metalist)
                }
            })
            .collect();

        Self {
            name: name,
            has_many: has_many
        }
    }
}