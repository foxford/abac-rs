use schema::abac_subject;
use types::AbacAttribute;

#[derive(Insertable, Identifiable, Queryable, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[primary_key(inbound, outbound)]
#[table_name = "abac_subject"]
pub struct AbacSubject {
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
            key: "uri".to_owned(),
            value: "account/1".to_owned(),
        };
        let outbound = AbacAttribute {
            namespace_id: Uuid::new_v4(),
            key: "role".to_owned(),
            value: "user".to_owned(),
        };

        let subject = AbacSubject {
            inbound: inbound.clone(),
            outbound: outbound.clone(),
        };

        let res = diesel::insert_into(abac_subject::table)
            .values(subject)
            .get_result::<AbacSubject>(&conn)
            .unwrap();

        assert_eq!(res.inbound, inbound);
    }

    fn establish_connection() -> PgConnection {
        let database_url = ::std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }
}
