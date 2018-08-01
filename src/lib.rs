#[macro_use]
extern crate diesel;
extern crate uuid;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub use attribute::{AbacAttribute, Attribute};

mod attribute;
pub mod models;
pub mod schema;

pub mod functions {
    use diesel::sql_types::{Array, Bool, Integer, Uuid};
    use sql_types::AbacAttribute;

    sql_function!(fn abac_authorize(subject: Array<AbacAttribute>, object: Array<AbacAttribute>, action: Array<AbacAttribute>, namespace_id: Array<Uuid>) -> Bool);
    sql_function!(fn abac_subject_target(x: Array<AbacAttribute>) -> AbacAttribute);
    sql_function!(fn abac_object_target(x: Array<AbacAttribute>) -> AbacAttribute);
    sql_function!(fn abac_action_target(x: Array<AbacAttribute>) -> AbacAttribute);
    sql_function!(fn abac_object_list_1(x: AbacAttribute, offset: Integer, limit: Integer) -> AbacAttribute);
    sql_function!(fn abac_object_list_2(x: AbacAttribute, y: AbacAttribute, offset: Integer, limit: Integer) -> AbacAttribute);
    sql_function!(fn abac_object_list_3(x: AbacAttribute, y: AbacAttribute, z: AbacAttribute, offset: Integer, limit: Integer) -> AbacAttribute);
    sql_function!(fn abac_object_list(x: Array<AbacAttribute>, offset: Integer, limit: Integer) -> AbacAttribute);
}

pub mod sql_types {
    #[derive(SqlType, QueryId)]
    #[postgres(type_name = "abac_attribute")]
    pub struct AbacAttribute;
}

pub mod dsl {
    use diesel::expression::{grouped::Grouped, Expression};
    use sql_types::AbacAttribute;

    mod predicates {
        use diesel::pg::Pg;
        use diesel::sql_types::{Text, Uuid};

        diesel_postfix_operator!(Key, ".key", Text, backend: Pg);
        diesel_postfix_operator!(NamespaceId, ".namespace_id", Uuid, backend: Pg);
    }

    use self::predicates::*;

    pub trait AbacAttributeExpressionMethods: Expression<SqlType = AbacAttribute> + Sized {
        fn key(self) -> Key<Grouped<Self>> {
            Key::new(Grouped(self))
        }

        fn namespace_id(self) -> NamespaceId<Grouped<Self>> {
            NamespaceId::new(Grouped(self))
        }
    }

    impl<T: Expression<SqlType = AbacAttribute>> AbacAttributeExpressionMethods for T {}
}
