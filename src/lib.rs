#[macro_use]
extern crate diesel;
extern crate uuid;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod models;
pub mod schema;

pub mod functions {
    use diesel::sql_types::{Array, Bool, Integer, Uuid};
    use types::AbacAttributeSqlType;

    sql_function!(fn abac_authorize(subject: Array<AbacAttributeSqlType>, object: Array<AbacAttributeSqlType>, action: Array<AbacAttributeSqlType>, namespace_id: Array<Uuid>) -> Bool);
    sql_function!(fn abac_subject_target(x: Array<AbacAttributeSqlType>) -> AbacAttributeSqlType);
    sql_function!(fn abac_object_target(x: Array<AbacAttributeSqlType>) -> AbacAttributeSqlType);
    sql_function!(fn abac_action_target(x: Array<AbacAttributeSqlType>) -> AbacAttributeSqlType);
    sql_function!(fn abac_object_list_1(x: AbacAttributeSqlType, offset: Integer, limit: Integer) -> AbacAttributeSqlType);
    sql_function!(fn abac_object_list_2(x: AbacAttributeSqlType, y: AbacAttributeSqlType, offset: Integer, limit: Integer) -> AbacAttributeSqlType);
    sql_function!(fn abac_object_list_3(x: AbacAttributeSqlType, y: AbacAttributeSqlType, z: AbacAttributeSqlType, offset: Integer, limit: Integer) -> AbacAttributeSqlType);
    sql_function!(fn abac_object_list(x: Array<AbacAttributeSqlType>, offset: Integer, limit: Integer) -> AbacAttributeSqlType);
}

pub mod types {
    use diesel::deserialize::{self, FromSql};
    use diesel::pg::Pg;
    use diesel::serialize::{self, Output, ToSql, WriteTuple};
    use diesel::sql_types::{Record, Text, Uuid};
    use uuid;

    use std::io::Write;

    #[derive(SqlType, QueryId)]
    #[postgres(type_name = "abac_attribute")]
    pub struct AbacAttributeSqlType;

    #[derive(Clone, Debug, Eq, PartialEq, Hash, AsExpression, FromSqlRow)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[sql_type = "AbacAttributeSqlType"]
    pub struct AbacAttribute {
        pub namespace_id: uuid::Uuid,
        pub key: String,
        pub value: String,
    }

    impl ToSql<AbacAttributeSqlType, Pg> for AbacAttribute {
        fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
            WriteTuple::<(Text, Text, Uuid)>::write_tuple(
                &(self.value.as_str(), self.key.as_str(), self.namespace_id),
                out,
            )
        }
    }

    impl FromSql<AbacAttributeSqlType, Pg> for AbacAttribute {
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
}

pub mod dsl {
    use types::AbacAttributeSqlType;
    use diesel::expression::{grouped::Grouped, Expression};

    mod predicates {
        use diesel::pg::Pg;
        use diesel::sql_types::{Uuid};

        diesel_postfix_operator!(NamespaceId, ".namespace_id", Uuid, backend: Pg);
    }

    use self::predicates::*;

    pub trait AbacAttributeExpressionMethods: Expression<SqlType = AbacAttributeSqlType> {
        fn namespace_id(self) -> NamespaceId<Grouped<Self>>
            where
                Self: Sized,
        {
            NamespaceId::new(Grouped(self))
        }
    }

    impl<T: Expression<SqlType = AbacAttributeSqlType>> AbacAttributeExpressionMethods for T {}
}
