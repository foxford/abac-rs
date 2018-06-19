use uuid::Uuid;

use schema::abac_policy;
use types::AbacAttribute;

#[derive(Insertable, Identifiable, Queryable, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[primary_key(subject, object, action, namespace_id)]
#[table_name = "abac_policy"]
pub struct AbacPolicy {
    pub subject: Vec<AbacAttribute>,
    pub object: Vec<AbacAttribute>,
    pub action: Vec<AbacAttribute>,
    pub namespace_id: Uuid,
}
