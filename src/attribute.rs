use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql, WriteTuple};
use diesel::sql_types::{Record, Text, Uuid};
use uuid;

use std::io::Write;

use sql_types;

#[derive(Clone, Debug, Eq, PartialEq, Hash, AsExpression, FromSqlRow)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[sql_type = "sql_types::AbacAttribute"]
pub struct AbacAttribute {
    pub namespace_id: uuid::Uuid,
    pub key: String,
    pub value: String,
}

impl AbacAttribute {
    pub fn new<T>(namespace_id: uuid::Uuid, attr: T) -> Self
    where
        T: Attribute,
    {
        AbacAttribute {
            namespace_id,
            key: attr.key(),
            value: attr.value(),
        }
    }
}

impl ToSql<sql_types::AbacAttribute, Pg> for AbacAttribute {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        WriteTuple::<(Text, Text, Uuid)>::write_tuple(
            &(self.value.as_str(), self.key.as_str(), self.namespace_id),
            out,
        )
    }
}

impl FromSql<sql_types::AbacAttribute, Pg> for AbacAttribute {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let (value, key, namespace_id) =
            FromSql::<Record<(Text, Text, Uuid)>, Pg>::from_sql(bytes)?;
        Ok(AbacAttribute {
            namespace_id,
            key,
            value,
        })
    }
}

pub trait Attribute {
    fn key(&self) -> String;
    fn value(&self) -> String;
}
