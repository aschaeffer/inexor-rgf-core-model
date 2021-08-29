use indradb::{EdgeProperties, EdgeKey, Type};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;
use crate::{PropertyInstanceGetter, MutablePropertyInstanceSetter};
use std::str::FromStr;
use async_graphql::scalar;

/// Relation instances are edges from an outbound entity instance to an
/// inbound entity instance.
///
/// The relation instance is of a relation type. The relation type defines
/// the entity types of the outbound entity instance and the inbound entity
/// instance. Furthermore the relation type defines which properties
/// (name, data type, socket type) a relation instance have to have.
///
/// In constrast to the relation type, the relation instance stores values/
/// documents in it's properties.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationInstance {

    /// The id of the outbound vertex.
    pub outbound_id: Uuid,

    /// The name of the relation type
    #[serde(alias = "type")]
    pub type_name: String,

    /// The id of the inbound vertex.
    pub inbound_id: Uuid,

    /// Textual description of the relation instance.
    #[serde(default = "String::new")]
    pub description: String,

    /// The properties of then relation instance.
    ///
    /// Each property is represented by it's name (String) and it's value. The value is
    /// a representation of a JSON. Therefore the value can be boolean, number, string,
    /// array or an object. For more information about the data types please look at
    /// https://docs.serde.rs/serde_json/value/enum.Value.html
    #[serde(default = "HashMap::new")]
    pub properties: HashMap<String, Value>,

}
scalar!(RelationInstance);

impl RelationInstance {
    pub fn new(
        outbound_id: Uuid,
        type_name: String,
        inbound_id: Uuid,
        properties: HashMap<String, Value>
    ) -> RelationInstance {
        RelationInstance {
            outbound_id,
            type_name,
            inbound_id,
            description: String::from(""),
            properties,
        }
    }

    pub fn get_key(&self) -> Option<EdgeKey> {
        let t = Type::from_str(self.type_name.as_str());
        if t.is_ok() {
            return Some(EdgeKey::new(self.outbound_id, t.unwrap(), self.inbound_id));
        }
        None
    }
}

impl From<EdgeProperties> for RelationInstance {
    fn from(properties: EdgeProperties) -> Self {
        let outbound_id = properties.edge.key.outbound_id.clone();
        let type_name = properties.edge.key.t.0.clone();
        let inbound_id = properties.edge.key.inbound_id.clone();
        let properties: HashMap<String, Value> = properties.props.iter()
            .map(|p| (p.name.clone(), p.value.clone()))
            .collect();
        RelationInstance {
            outbound_id,
            type_name,
            inbound_id,
            description: String::new(),
            properties,
        }
    }
}

impl PropertyInstanceGetter for RelationInstance {
    fn get<S: Into<String>>(&self, property_name: S) -> Option<Value> {
        self.properties.get(&property_name.into()).and_then(|v| Some(v.clone()))
    }

    fn as_bool<S: Into<String>>(&self, property_name: S) -> Option<bool> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_bool())
    }

    fn as_u64<S: Into<String>>(&self, property_name: S) -> Option<u64> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_u64())
    }

    fn as_i64<S: Into<String>>(&self, property_name: S) -> Option<i64> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_i64())
    }

    fn as_f64<S: Into<String>>(&self, property_name: S) -> Option<f64> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_f64())
    }

    fn as_string<S: Into<String>>(&self, property_name: S) -> Option<String> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_str().and_then(|s| Some(s.to_string())))
    }
}

impl MutablePropertyInstanceSetter for RelationInstance {
    fn set<S: Into<String>>(&mut self, property_name: S, value: Value) {
        let property_value = self.properties.get_mut(&property_name.into()).unwrap();
        *property_value = value.clone()
    }
}