use std::str::FromStr;

use indradb::Identifier;
use serde::{Deserialize, Serialize};

use crate::extension::Extension;
use crate::PropertyType;

/// Entity types defines the type of an entity instance.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntityType {
    /// The name of the entity type.
    ///
    /// The name is the unique identifier for entity types.
    pub name: String,

    /// The entity type belongs to the given group of entity types.
    #[serde(default = "String::new")]
    pub group: String,

    /// Textual description of the entity type.
    #[serde(default = "String::new")]
    pub description: String,

    /// The names of the components of the entity type.
    #[serde(default = "Vec::new")]
    pub components: Vec<String>,

    /// The properties which are defined by the entity type.
    #[serde(default = "Vec::new")]
    pub properties: Vec<PropertyType>,

    /// Entity type specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,

    #[serde(skip)]
    pub t: Identifier,
}

impl EntityType {
    pub fn new<S: Into<String>>(
        name: S,
        group: S,
        description: S,
        components: Vec<String>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> EntityType {
        let name = name.into();
        let t = Identifier::from_str(name.as_str()).unwrap();
        EntityType {
            name,
            group: group.into(),
            description: description.into(),
            components,
            properties,
            extensions,
            t,
        }
    }

    /// Returns true, if the entity type is a component with the given name.
    pub fn is_a<S: Into<String>>(&self, component_name: S) -> bool {
        self.components.contains(&component_name.into())
    }

    /// Returns true, if the entity type contains an own property with the given name.
    /// Doesn't respect properties from potential components.
    pub fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool {
        let property_name = property_name.into();
        self.properties.iter().any(|p| p.name == property_name)
    }

    /// Returns true, if the entity type contains an extension with the given name.
    pub fn has_own_extension<S: Into<String>>(&self, extension_name: S) -> bool {
        let extension_name = extension_name.into();
        self.extensions.iter().any(|extension| extension.name == extension_name)
    }
}
