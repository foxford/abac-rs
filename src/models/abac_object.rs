use schema::abac_object;
use types::AbacAttribute;

#[derive(Insertable, Identifiable, Queryable, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[primary_key(inbound, outbound)]
#[table_name = "abac_object"]
pub struct AbacObject {
    pub inbound: AbacAttribute,
    pub outbound: AbacAttribute,
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::{self, prelude::*};
    use uuid::Uuid;

    #[test]
    fn db_round_trip() {
        let conn = establish_connection();
        conn.begin_test_transaction().unwrap();

        let inbound = AbacAttribute {
            namespace_id: Uuid::new_v4(),
            key: "type".to_owned(),
            value: "apple".to_owned(),
        };
        let outbound = AbacAttribute {
            namespace_id: Uuid::new_v4(),
            key: "type".to_owned(),
            value: "fruit".to_owned(),
        };

        let object = AbacObject {
            inbound: inbound.clone(),
            outbound: outbound.clone(),
        };

        let res = diesel::insert_into(abac_object::table)
            .values(object)
            .get_result::<AbacObject>(&conn)
            .unwrap();

        assert_eq!(res.inbound, inbound);
    }

    fn establish_connection() -> PgConnection {
        let database_url = ::std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }
}
