use diesel::prelude::*;
use diesel::result::Error as DBError;
use domain::models::done_alerts::*;
use infrastructure::DBPool;
use uuid::Uuid;

pub fn get_done_alerts_by_project_id_and_type(conn: &DBPool, project_id_: Uuid, alert_type_: AlertType) -> Result<Vec<DoneAlert>, DBError> {
    use domain::schema::done_alerts::dsl::*;

    done_alerts.filter(project_id.eq(project_id_))
        .filter(type_.eq(alert_type_))
        .get_results(&mut conn.get().unwrap())
}

pub fn create_done_alert(conn: &DBPool, new_done_alert: NewDoneAlert) -> Result<(), DBError> {
    use domain::schema::done_alerts::dsl::*;

    diesel::insert_into(done_alerts)
        .values(&new_done_alert)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::projects::test::test_create_project;
    use infrastructure::init_pool;

    struct TestContext {
        conn: DBPool,
    }

    impl TestContext {
        fn new() -> Self {
            TestContext {
                conn: init_pool(
                    dotenvy::var("DATABASE_URL")
                        .expect("DATABASE_URL must be set")
                        .as_str()
                ),
            }
        }
    }

    fn test_create_done_alert() -> (Uuid, AlertType) {
        let ctx = TestContext::new();
        let (project_id, _) = test_create_project();
        let new_done_alert = NewDoneAlert {
            description: Some("Test done alert".to_string()),
            project_id,
            type_: AlertType::Started,
        };

        create_done_alert(&ctx.conn, new_done_alert).unwrap();

        (project_id, AlertType::Started)
    }

    #[test]
    fn test_get_done_alerts_by_project_id_and_type() {
        let ctx = TestContext::new();
        let (project_id, alert_type) = test_create_done_alert();

        let done_alerts = get_done_alerts_by_project_id_and_type(&ctx.conn, project_id, alert_type).unwrap();

        assert_eq!(done_alerts[0].clone().description.unwrap(), "Test done alert".to_string());
    }
}