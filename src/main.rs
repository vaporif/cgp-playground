use cgp_core::component::HasComponents;

fn main() {
    println!("Hello, world!");
}

pub struct Repository;

impl HasComponents for Repository {
    type Components = RepositoryComponents;
}

pub struct RepositoryComponents;
