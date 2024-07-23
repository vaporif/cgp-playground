use cgp_core::prelude::*;

// NOTE: We can have one Components Container
// to call implementation in another Components

pub fn delegate_example() -> &'static str {
    let s = DelegationUserStruct;
    s.delegate()
}

struct DelegationUserStruct;
impl HasComponents for DelegationUserStruct {
    type Components = DelegatedComponents;
}

// NOTE: Top components
struct DelegatedComponents;

#[derive_component(DelegatedComponent, Delegate<Context>)]
pub trait CanDelegate {
    fn delegate(&self) -> &'static str;
}

// NOTE: Lets define DI container and push all implementations
// to nested components
delegate_components! {
    DelegatedComponents {
        DelegatedComponent: DelegateeComponents
    }
}

struct DelegateeComponents;

delegate_components! {
    DelegateeComponents {
        DelegatedComponent: DelegatedImpl
    }
}

// NOTE:
// DelegatedComponents -> CanDelegate -> DelegateeComponents[CanDelegate -> DelegateImpl]

struct DelegatedImpl;
impl<Context> Delegate<Context> for DelegatedImpl {
    fn delegate(_context: &Context) -> &'static str {
        "delegated"
    }
}
