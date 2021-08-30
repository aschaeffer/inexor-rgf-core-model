use std::collections::HashMap;
use std::str::FromStr;

use indradb::{NamedProperty, Type, Vertex, VertexProperties};
use serde_json::json;
use uuid::Uuid;

use crate::{EntityInstance, Flow};
use crate::{MutablePropertyInstanceSetter, PropertyInstanceGetter};
use crate::tests::utils::r_string;

#[test]
fn flow_test() {
    let flow_id = Uuid::new_v4();
    let flow_type_name = r_string();
    let flow_name = r_string();
    let flow_description = r_string();

    let flow = Flow {
        id: flow_id,
        type_name: flow_type_name.clone(),
        name: flow_name.clone(),
        description: flow_description.to_string(),
        entity_instances: Vec::new(),
        relation_instances: Vec::new()
    };

    assert_eq!(flow_type_name.clone(), flow.type_name.clone());
    assert_eq!(flow_id.clone(), flow.id.clone());
    assert_eq!(flow_name.clone(), flow.name.clone());
    assert_eq!(flow_description.clone(), flow.description.clone());
    assert_eq!(0, flow.entity_instances.len());
    assert_eq!(0, flow.relation_instances.len());
}