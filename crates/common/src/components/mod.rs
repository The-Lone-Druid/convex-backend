use std::sync::LazyLock;

use cmd_util::env::env_config;
use value::InternalDocumentId;

mod component_definition_path;
mod component_path;
mod function_paths;
mod module_paths;
mod reference;
mod resource;

pub use self::{
    component_definition_path::ComponentDefinitionPath,
    component_path::ComponentPath,
    function_paths::{
        CanonicalizedComponentFunctionPath,
        ComponentDefinitionFunctionPath,
        ComponentFunctionPath,
    },
    module_paths::CanonicalizedComponentModulePath,
    reference::Reference,
    resource::Resource,
};

pub static COMPONENTS_ENABLED: LazyLock<bool> =
    LazyLock::new(|| env_config("COMPONENTS_ENABLED", false));

pub fn require_components_enabled() -> anyhow::Result<()> {
    if !*COMPONENTS_ENABLED {
        anyhow::bail!("Components are not enabled, set COMPONENTS_ENABLED=true to enable them.");
    }
    Ok(())
}

// Globally unique system-assigned ID for a component.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ComponentId {
    Root,
    Child(InternalDocumentId),
}

impl ComponentId {
    pub fn is_root(&self) -> bool {
        matches!(self, ComponentId::Root)
    }
}

#[cfg(any(test, feature = "testing"))]
mod proptests {
    use proptest::prelude::*;

    use super::ComponentId;

    impl Arbitrary for ComponentId {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            Just(ComponentId::Root).boxed()
        }
    }
}
