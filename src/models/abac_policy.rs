use uuid::Uuid;

use attribute::AbacAttribute;
use schema::abac_policy;

#[derive(Insertable, Identifiable, Queryable, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[primary_key(subject, object, action, namespace_id)]
#[table_name = "abac_policy"]
pub struct AbacPolicy {
    pub subject: Vec<AbacAttribute>,
    pub object: Vec<AbacAttribute>,
    pub action: Vec<AbacAttribute>,
    pub namespace_id: Uuid,
}
