use std::marker::PhantomData;

use cgp_core::prelude::*;
/* NOTE:
* Cgp is a mix of OOP concepts/patterns implemented in rust
* Think of it as a way to have inheritance + dependency injection
* without a need for classes via generics + delegation pattern
*/
fn main() {
    let data = vec![Entity {
        name: "alex".to_string(),
        value: "random".to_string(),
    }];

    let repo = Repository { data };

    repo.has_item("alex".to_string());
    // repo.hello();
}

// NOTE: lets create a struct to consume cgp functionality
struct Repository<T> {
    data: Vec<T>,
}

/* NOTE: Initial wire-up
* HasItem - Top-level *Interface*, shoud be attached to user structs, has no implementation i.e. delegates impl to ItemChecker
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
* GetItem - 0-sized struct to hold implementaiton
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
    // NOTE: HRTB would be hard to wire-up, keep simple
    <Context as CanGetDb>::Item: HasName,
{
    fn has_item(context: &Context, item: String) -> bool {
        let db = context.get_all_items();
        db.iter().any(|f| f.name() == item)
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

/* NOTE: More glue
* Associate exposed functions from RepositoryComponents to impl struct
*/
impl<T> HasComponents for Repository<T>
where
    T: Async,
{
    type Components = RepositoryComponents;
}

/* NOTE: Complete dependency config
* I.e ItemCheckComponent - dependency (HasName is an outer interface)
* GetItem - 0-sized struct with attached implementation
*
* You can associate multiple dependencies wiht one implementation container
* like with GetDbFromMemory
*
* You can also nest everything even more
*/
delegate_components!(RepositoryComponents {
    ItemCheckComponent: GetItem,
    [
        ItemsComponent,
        OrderedItemsComponent,
    ]:
    // NOTE: mixing two implementations
    GetDbFromMemory<Entity>,
});

#[derive_component(OrderedItemsComponent, OrderedItemsGetter<Context>)]
pub trait CanGetOrderedItems {
    type Item;
    fn get_ordered_items(&self) -> Vec<Self::Item>;
}

#[derive_component(ItemsComponent, ItemsGetter<Context>)]
pub trait CanGetDb {
    type Item;
    fn get_all_items(&self) -> Vec<Self::Item>;
}

// NOTE: Concrete implementation for specific type
impl ItemsGetter<Repository<Entity>> for GetDbFromMemory<Entity> {
    type Item = Entity;

    fn get_all_items(context: &Repository<Entity>) -> Vec<Self::Item> {
        context.data.to_vec()
    }
}

// NOTE: As written before, you can place more trait bounds on context
pub struct GetDbFromMemory<ItemContext>(PhantomData<ItemContext>);
impl<Context, ItemContext> OrderedItemsGetter<Context> for GetDbFromMemory<ItemContext>
where
    // NOTE: Async trait is used heavily in CGP, required for associated types
    ItemContext: HasName + Clone + std::cmp::Ord + Async,
    Context: CanGetDb<Item = ItemContext>,
{
    type Item = ItemContext;

    fn get_ordered_items(context: &Context) -> Vec<Self::Item> {
        let mut items = context.get_all_items();
        items.sort();
        items.to_vec()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entity {
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

// NOTE: Works if defined in our repo's
// Doesn't work here, glue missing

// #[derive_component(NestedCallComponent, Nested<Context>)]
// pub trait CanNestedCall {
//     fn hello(&self) -> &'static str;
// }

// // NOTE: Lets define DI container and extend another container
// define_components! {
//     NestedComponents {
//         NestedCallComponent: NestedImpl
//     }
// }

// // NOTE: RepositoryComponents will receive everything from NestedComponents
// // FYI with_nested_components will be autogenerated
// with_nested_components! {
//     delegate_components! {
//         RepositoryComponents {
//             @NestedComponents: NestedComponents
//         }
//     }
// }

// pub struct NestedImpl;

// impl<Context> Nested<Context> for NestedImpl {
//     fn hello(context: &Context) -> &'static str {
//         "hello"
//     }
// }
