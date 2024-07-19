use cgp_core::prelude::*;
/* NOTE:
* Cgp is a mix of OOP concepts/patterns implemented in rust
* Think of it as a way to have inheritance + dependency injection
* without a need for classes via generics + delegation pattern
*/
fn main() {}

/* NOTE: Initial wire-up
* HasItem - Top-level *Interface*, has no implementation (is a delegator), delegates impl to ItemChecker
* ItemChecker - Trait that implements concrete functionality, a delegatee
* ItemCheckComponent - glue trait, holds inside link between delegator and delegatee
*/
#[derive_component(ItemCheckComponent, ItemChecker<Context>)]
pub trait HasItem {
    fn has_item(&self, item: String) -> bool;
}

/* NOTE: Now we can provide implementation for delegatee
* GetItemFromMemory - 0-sized struct to hold implementaiton
*/
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

/* NOTE: More glue
* RepositoryComponents - Sorta container from dependency injection
*/
pub struct RepositoryComponents;
delegate_components!(RepositoryComponents {
    ItemCheckComponent: GetItemFromMemory,
});

#[derive_component(DbGetterComponent, DbGetter<Context>)]
pub trait CanGetDb {
    fn get_db(&self) -> Vec<String>;
}
