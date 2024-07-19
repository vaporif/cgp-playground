use std::{marker::PhantomData, task::Context};

use cgp_core::prelude::*;
/* NOTE:
* Cgp is a mix of OOP concepts/patterns implemented in rust
* Think of it as a way to have inheritance + dependency injection
* without a need for classes via generics + delegation pattern
*/
fn main() {}

/* NOTE: Initial wire-up
* HasItem - Top-level *Interface*, shoud be attachesd to user structs, has no implementation i.e. delegates impl to ItemChecker
* ItemChecker - Trait that implements concrete functionality, a delegatee, will copy HasItem fn's
* ItemCheckComponent - glue trait, named wrapper around delegatee. Exists to allow multiple
* implementations of a delegatee.
*
* So in the end the chain of delegation looks like
*
* HasItem -> ItemCheckComponent -> ItemChecker
*/
#[derive_component(ItemCheckComponent, ItemChecker<Context>)]
pub trait HasItem {
    fn has_item(&self, item: String) -> bool;
}

/* NOTE: Now we can provide implementation for delegatee
* GetItemFromMemory - 0-sized struct to hold implementaiton
*/
pub struct GetItem;

/* NOTE: Implementation finally!
* We can add more bounds to context which in the end can require context to implement more
* *Interface*
*
* In terms of OOP/DI try treating Context as a composition class inside ItemChecker
* Context could have *Interface* requirements too, so if we sorta hack representation as OOP
* class ItemChecker: HasItem {
*   Context composiotion;
*
*   ItemChecker(Context injected) {
*       composion = injected
*   }
* }
* class Context: CanGetDB {
* }
*
* You can have more nesting inside CanGetDB context too, making complex injection graphs
* Can limit associated types too
*/
impl<Context> ItemChecker<Context> for GetItem
where
    Context: CanGetDb,
    for<'a> &'a <Context as CanGetDb>::Item: Into<String>,
{
    fn has_item(context: &Context, item: String) -> bool {
        let db = context.get_db();
        let item = db
            .iter()
            .map(|x| {
                let s: String = x.into();
                s
            })
            .any(|x| x == item);

        println!("{:?}", item);

        item
    }
}

/* NOTE: More glue
* RepositoryComponents - Sorta container from dependency injection pattern
* Here we have final connection between GetItemFromMemory delegatee implementation
* to ItemCheckComponent holding top level *Interface* HasItem
*
* In the if you want to attach container to an object the only thing you need to do is
* impl HasComponents for Whatever {
    type Components = RepositoryComponents;
  }
*/
pub struct RepositoryComponents;
delegate_components!(RepositoryComponents {
    ItemCheckComponent: GetItem,
});

#[derive_component(DbGetterComponent, DbGetter<Context>)]
pub trait CanGetDb {
    type Item;
    fn get_db(&self) -> &Vec<Self::Item>;
}

pub struct GetDbFromMemory<ItemContext>(PhantomData<ItemContext>);
impl<Context, ItemContext> DbGetter<Context> for GetDbFromMemory<ItemContext>
where
    // NOTE: Async trait is used heavily in CGP, required almost everywhere
    ItemContext: Async,
{
    type Item = ItemContext;

    fn get_db(context: &Context) -> &Vec<Self::Item> {
        // Imagine you have a connection to db here
        // lets just return default data
        todo!()
    }
}

#[derive(Debug)]
struct Entity {
    pub name: String,
    pub value: String,
}

trait HasName {
    fn name(&self) -> &str;
}

impl HasName for Entity {
    fn name(&self) -> &str {
        &self.name
    }
}
