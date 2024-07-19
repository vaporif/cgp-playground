use std::u16;

use cgp_core::prelude::*;

fn main() {}

pub struct RepositoryComponents;
#[derive_component(ItemGetterComponent, ItemsGetter<Context>)]
pub trait CanFormatItems {
    fn has_item(&mut self, item: String) -> bool;
}

pub struct GetItemFromMemory;

impl<Context> ItemsGetter<Context> for GetItemFromMemory
where
    Context: CanGetDb,
{
    fn has_item(context: &mut Context, item: String) -> bool {
        let db = context.get_db();
        db.contains(&item)
    }
}

delegate_components!(RepositoryComponents {
    ItemGetterComponent: GetItemFromMemory,
});

#[derive_component(DbGetterComponent, DbGetter<Context>)]
pub trait CanGetDb {
    fn get_db(&mut self) -> Vec<String>;
}
