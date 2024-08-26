use diesel::internal::derives::multiconnection::chrono;
use diesel::prelude::*;
use diesel::result::Error as DBError;
use domain::models::projects::*;
use domain::models::promotions::Promotion;
use infrastructure::DBPool;
use uuid::Uuid;

pub fn get_project_by_id(conn: &DBPool, id_: Uuid) -> Result<Project, DBError> {
    use domain::schema::projects::dsl::*;

    projects.filter(id.eq(id_))
        .first(&mut conn.get().unwrap())
}

pub fn get_current_projects(conn: &DBPool) -> Result<Vec<Project>, DBError> {
    use domain::schema::projects::dsl::*;

    let now = chrono::Utc::now().naive_utc();

    projects.filter(start_date.le(now))
        .filter(state.ne(ProjectState::NotationFinished))
        .get_results(&mut conn.get().unwrap())
}

pub fn get_projects_from_promotion_id(conn: &DBPool, promotion_id_: Uuid) -> Result<Vec<Project>, DBError> {
    use domain::schema::projects::dsl::*;

    projects.filter(promotion_id.eq(promotion_id_))
        .get_results(&mut conn.get().unwrap())
}

pub fn get_promotion_from_project_id(conn: &DBPool, project_id_: Uuid) -> Result<Promotion, DBError> {
    use domain::schema::projects::dsl::*;

    projects.select(promotion_id)
        .filter(id.eq(project_id_))
        .inner_join(domain::schema::promotions::table)
        .select(domain::schema::promotions::all_columns)
        .first(&mut conn.get().unwrap())
}

pub fn create_project(conn: &DBPool, new_project: NewProject) -> Result<Uuid, DBError> {
    use domain::schema::projects::dsl::*;

    let result: Result<Uuid, DBError> = diesel::insert_into(projects)
        .values(&new_project)
        .returning(id)
        .get_result(&mut conn.get().unwrap());

    result
}

pub fn update_project(conn: &DBPool, id_: Uuid, updated_project: UpdatedProject) -> Result<(), DBError> {
    use domain::schema::projects::dsl::*;

    // Check if the student exists
    projects.filter(id.eq(id_.clone()))
        .first::<Project>(&mut conn.get().unwrap())?;

    diesel::update(projects.filter(id.eq(id_)))
        .set(&updated_project)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn delete_project(conn: &DBPool, id_: Uuid) -> Result<(), DBError> {
    use domain::schema::projects::dsl::*;

    // Check if the student exists
    projects.filter(id.eq(id_.clone()))
        .first::<Project>(&mut conn.get().unwrap())?;

    diesel::delete(projects.filter(id.eq(id_)))
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::database::promotions::tests::test_create_promotion;
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

    pub fn test_create_project() -> (Uuid, Uuid) {
        let context = TestContext::new();

        let (promotion_id, _) = test_create_promotion();
        let random = Uuid::new_v4();
        let new_project = NewProject{
            name: format!("test-{}", random),
            description: None,
            start_date: None,
            end_date: Default::default(),
            notation_period_duration: None,
            promotion_id,
            state: None,
        };

        (create_project(&context.conn, new_project).unwrap(), promotion_id)
    }

    #[test]
    fn test_get_project_by_id() {
        let context = TestContext::new();

        let (project_id, _) = test_create_project();

        let project = get_project_by_id(&context.conn, project_id).unwrap();
        assert_eq!(project_id, project.id)
    }

    #[test]
    fn test_get_projects_from_promotion_id() {
        let context = TestContext::new();

        let (project_id, promotion_id) = test_create_project();

        let projects = get_projects_from_promotion_id(&context.conn, promotion_id).unwrap();
        assert_eq!(projects[0].id, project_id);
    }

    #[test]
    fn test_get_promotion_from_project_id() {
        let context = TestContext::new();

        let (project_id, promotion_id) = test_create_project();

        let promotion = get_promotion_from_project_id(&context.conn, project_id).unwrap();

        assert_eq!(promotion.id, promotion_id);
    }

    #[test]
    fn test_update_project() {
        let context = TestContext::new();

        let (project_id, _) = test_create_project();

        let random = Uuid::new_v4();
        let updated_project = UpdatedProject {
            name: Some(format!("updated-{}", random)),
            description: None,
            start_date: None,
            end_date: None,
            notation_period_duration: None,
            state: None,
        };
        update_project(&context.conn, project_id, updated_project).unwrap();

        let project = get_project_by_id(&context.conn, project_id).unwrap();
        assert_eq!(project.name, format!("updated-{}", random));
    }

    #[test]
    fn test_delete_project() {
        let context = TestContext::new();

        let (project_id, _) = test_create_project();

        delete_project(&context.conn, project_id).unwrap();
    }
}