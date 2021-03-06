use std::collections::HashMap;
use std::sync::Arc;

use dashmap::DashMap;
use dashmap::DashSet;
use indradb::{EdgeKey, EdgeProperties, Identifier};
use serde_json::Map;
use serde_json::Value;
use uuid::Uuid;

use crate::PropertyInstanceGetter;
use crate::PropertyInstanceSetter;
use crate::ReactiveEntityInstance;
use crate::ReactivePropertyInstance;
use crate::RelationInstance;

/// Reactive instance of a relation in the directed property graph.
///
/// Property Graph: The relation instance can store properties.
///
/// Directed Graph: The direction of the relation point from the outbound
/// entity instance to the inbound entity instance.
///
/// Reactive Instance: The properties are streams with a local copies of
/// the last result. The streams can be connected, combined, folded or zipped.
///
/// One example for a directed reactive relation instance is a connector which
/// propagates changes on a property of the outbound entity to a property of
/// the inbound entity.
///
/// Another example would be the velocity transformation which are also using
/// the streams of the properties of the outbound entity, the inbound entity
/// and/or the relation itself.
///
/// Last but not least relation instances can be used for semantic
/// representations like the current camera of a player:
/// Player--(CurrentCamera)-->Camera
///
pub struct ReactiveRelationInstance {
    /// The outbound entity instance.
    pub outbound: Arc<ReactiveEntityInstance>,

    /// The name of the relation type.
    pub type_name: String,

    /// The outbound entity instance.
    pub inbound: Arc<ReactiveEntityInstance>,

    /// An optional description of the relation.
    pub description: String,

    /// The reactive properties.
    pub properties: DashMap<String, ReactivePropertyInstance>,

    /// The names of the components which are applied on this relation instance.
    pub components: DashSet<String>,

    /// The names of the behaviours which are applied on this relation instance.
    pub behaviours: DashSet<String>,
}

impl ReactiveRelationInstance {
    // TODO: rename to "from_properties"
    pub fn from(outbound: Arc<ReactiveEntityInstance>, inbound: Arc<ReactiveEntityInstance>, properties: EdgeProperties) -> ReactiveRelationInstance {
        let type_name = properties.edge.key.t.to_string();
        let properties = properties
            .props
            .iter()
            .map(|named_property| {
                (
                    named_property.name.to_string(),
                    ReactivePropertyInstance::new(
                        Uuid::new_v4(), // or generate a combined uuid from "outbound_id + type + inbound_id"
                        named_property.name.to_string(),
                        named_property.value.clone(),
                    ),
                )
            })
            .collect();
        ReactiveRelationInstance {
            outbound,
            type_name,
            inbound,
            description: String::new(),
            properties,
            components: DashSet::new(),
            behaviours: DashSet::new(),
        }
    }

    pub fn from_instance(outbound: Arc<ReactiveEntityInstance>, inbound: Arc<ReactiveEntityInstance>, instance: RelationInstance) -> ReactiveRelationInstance {
        let properties = instance
            .properties
            .iter()
            .map(|(name, value)| (name.clone(), ReactivePropertyInstance::new(Uuid::new_v4(), name.clone(), value.clone())))
            .collect();
        ReactiveRelationInstance {
            outbound,
            type_name: instance.type_name.clone(),
            inbound,
            description: instance.description,
            properties,
            components: DashSet::new(),
            behaviours: DashSet::new(),
        }
    }

    // TODO: unit test
    // TODO: rename to "new_with_properties"
    pub fn create_with_properties<S: Into<String>>(
        outbound: Arc<ReactiveEntityInstance>,
        type_name: S,
        inbound: Arc<ReactiveEntityInstance>,
        properties: HashMap<String, Value>,
    ) -> ReactiveRelationInstance {
        let properties = properties
            .iter()
            .map(|(name, value)| {
                (
                    name.clone(),
                    ReactivePropertyInstance::new(
                        Uuid::new_v4(), // or generate a combined uuid from "outbound_id + type + inbound_id"
                        name.clone(),
                        value.clone(),
                    ),
                )
            })
            .collect();
        ReactiveRelationInstance {
            outbound,
            type_name: type_name.into(),
            inbound,
            description: String::new(),
            properties,
            components: DashSet::new(),
            behaviours: DashSet::new(),
        }
    }

    pub fn get_key(&self) -> Option<EdgeKey> {
        Identifier::new(self.type_name.as_str())
            .map(|t| EdgeKey::new(self.outbound.id, t, self.inbound.id))
            .ok()
    }

    pub fn tick(&self) {
        for property_instance in &self.properties {
            property_instance.tick();
        }
    }

    pub fn add_property<S: Into<String>>(&self, name: S, value: Value) {
        let name = name.into();
        if !self.properties.contains_key(name.as_str()) {
            let property_instance = ReactivePropertyInstance::new(Uuid::new_v4(), name.clone(), value);
            self.properties.insert(name, property_instance);
        }
    }

    pub fn add_component<S: Into<String>>(&self, component: S) {
        self.components.insert(component.into());
    }

    pub fn remove_component<S: Into<String>>(&self, component: S) {
        self.components.remove(component.into().as_str());
    }

    /// Returns true, if the relation instance is composed with the given component.
    pub fn is_a<S: Into<String>>(&self, component: S) -> bool {
        self.components.contains(component.into().as_str())
    }

    pub fn add_behaviour<S: Into<String>>(&self, behaviour: S) {
        self.behaviours.insert(behaviour.into());
    }

    pub fn remove_behaviour<S: Into<String>>(&self, behaviour: S) {
        self.behaviours.remove(behaviour.into().as_str());
    }

    /// Returns true, if the relation instance behaves as the given behaviour.
    pub fn behaves_as<S: Into<String>>(&self, behaviour: S) -> bool {
        self.behaviours.contains(behaviour.into().as_str())
    }
}

impl From<Arc<ReactiveRelationInstance>> for RelationInstance {
    fn from(instance: Arc<ReactiveRelationInstance>) -> Self {
        let properties = instance
            .properties
            .iter()
            .map(|property_instance| (property_instance.key().clone(), property_instance.get()))
            .collect();
        RelationInstance {
            outbound_id: instance.outbound.id,
            type_name: instance.type_name.clone(),
            inbound_id: instance.inbound.id,
            description: instance.description.clone(),
            properties,
        }
    }
}

impl PropertyInstanceGetter for ReactiveRelationInstance {
    fn get<S: Into<String>>(&self, property_name: S) -> Option<Value> {
        self.properties.get(&property_name.into()).map(|p| p.get())
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
        self.properties.get(&property_name.into()).and_then(|p| p.as_string())
    }

    fn as_array<S: Into<String>>(&self, property_name: S) -> Option<Vec<Value>> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_array())
    }

    fn as_object<S: Into<String>>(&self, property_name: S) -> Option<Map<String, Value>> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_object())
    }
}

impl PropertyInstanceSetter for ReactiveRelationInstance {
    fn set<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.properties.get(&property_name.into()) {
            instance.set(value);
        }
    }

    fn set_no_propagate<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.properties.get(&property_name.into()) {
            instance.set_no_propagate(value);
        }
    }
}
