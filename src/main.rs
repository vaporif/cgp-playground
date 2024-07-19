use cgp_core::prelude::*;

fn main() {}

pub struct RepositoryComponents;
#[derive_component(ItemCheckComponent, ItemChecker<Context>)]
pub trait HasItem {
    fn has_item(&self, item: String) -> bool;
}

pub struct GetItemFromMemory;

impl<Context> ItemChecker<Context> for GetItemFromMemory
where
    Context: CanGetDb,
{
    fn has_item(context: &Context, item: String) -> bool {
        let db = context.get_db();
        db.contains(&item)
    }
}

delegate_components!(RepositoryComponents {
    ItemCheckComponent: GetItemFromMemory,
});

#[derive_component(DbGetterComponent, DbGetter<Context>)]
pub trait CanGetDb {
    fn get_db(&self) -> Vec<String>;
}
