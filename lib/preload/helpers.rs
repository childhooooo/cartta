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