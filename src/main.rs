use cgp_core::prelude::*;

fn main() {
    println!("Hello, world!");
}

pub struct Repository;

impl HasComponents for Repository {
    type Components = RepositoryComponents;
}

pub struct RepositoryComponents;

#[derive_component(DataComponent, DataGetter<Context>)]
pub trait CanGetData {
    fn get_data(self) -> String;
}

// delegate_components! {
//     #[mark_component(IsRepositoryComponent)]
//     RepositoryComponents {
//         DataComponent: GetData,
//     }
// }
