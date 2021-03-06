use std::str::FromStr;

use indradb::Identifier;
use serde::{Deserialize, Serialize};

use crate::extension::Extension;
use crate::PropertyType;

/// A relation type defines the type of an relation instance.
///
/// The relation type defines the entity types of the outbound and inbound entity instances.
/// Also the relation type defines the properties of the relation instance.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationType {
    /// The name of the outbound entity type.
    pub outbound_type: String,

    /// The name of the relation type.
    ///
    /// The name is the unique identifier for relation types.
    #[serde(alias = "name")]
    pub type_name: String,

    /// The full type name of the relation type.
    #[serde(default = "String::new")]
    pub full_name: String,

    /// The name of the inbound entity type.
    pub inbound_type: String,

    /// The relation type belongs to the given group of relation types.
    #[serde(default = "String::new")]
    pub group: String,

    /// Textual description of the relation type.
    #[serde(default = "String::new")]
    pub description: String,

    /// The names of the components of the relation type.
    #[serde(default = "Vec::new")]
    pub components: Vec<String>,

    /// The properties which are defined by the relation type.
    #[serde(default = "Vec::new")]
    pub properties: Vec<PropertyType>,

    /// Relation type specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,

    #[serde(skip)]
    pub t: Identifier,
}

impl RelationType {
    #[allow(clippy::too_many_arguments)]
    pub fn new<S: Into<String>>(
        outbound_type: S,
        type_name: S,
        inbound_type: S,
        group: S,
        description: S,
        components: Vec<String>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> RelationType {
        let type_name = type_name.into();
        let t = Identifier::from_str(type_name.as_str()).unwrap();
        RelationType {
            outbound_type: outbound_type.into(),
            full_name: type_name.clone(),
            type_name,
            inbound_type: inbound_type.into(),
            group: group.into(),
            description: description.into(),
            components,
            properties,
            extensions,
            t,
        }
    }

    /// Returns true, if the relation type is a component with the given name.
    pub fn is_a<S: Into<String>>(&self, component_name: S) -> bool {
        self.components.contains(&component_name.into())
    }

    /// Returns true, if the relation type contains an own property with the given name.
    /// Doesn't respect properties from potential components.
    pub fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool {
        let property_name = property_name.into();
        self.properties.iter().any(|p| p.name == property_name)
    }

    /// Returns true, if the relation type contains an extension with the given name.
    pub fn has_own_extension<S: Into<String>>(&self, extension_name: S) -> bool {
        let extension_name = extension_name.into();
        self.extensions.iter().any(|extension| extension.name == extension_name)
    }
}
