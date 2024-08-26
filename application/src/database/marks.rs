use crate::database::groups::get_students_with_group;
use diesel::prelude::*;
use diesel::result::Error as DBError;
use domain::models::marks::*;
use domain::models::students::Student;
use infrastructure::DBPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn get_all_marks_of_project(conn: &DBPool, project_id_: Uuid) -> Result<Vec<Mark>, DBError> {
    use domain::schema::marks::dsl::*;

    marks.filter(project_id.eq(project_id_))
        .get_results(&mut conn.get().unwrap())
}

pub fn get_marks_from_group_id(conn: &DBPool, group_id_: Uuid) -> Result<Vec<Mark>, DBError> {
    use domain::schema::marks::dsl::*;

    marks.filter(group_id.eq(group_id_))
        .get_results(&mut conn.get().unwrap())
}

pub fn get_marks_given_to_student_id(conn: &DBPool, student_id_: Uuid) -> Result<Vec<Mark>, DBError> {
    use domain::schema::marks::dsl::*;

    marks.filter(noted_student_id.eq(student_id_))
        .get_results(&mut conn.get().unwrap())
}

pub fn get_marks_given_to_student_id_and_group_id(conn: &DBPool, student_id_: Uuid, group_id_: Uuid) -> Result<Vec<Mark>, DBError> {
    use domain::schema::marks::dsl::*;

    marks.filter(noted_student_id.eq(student_id_))
        .filter(group_id.eq(group_id_))
        .get_results(&mut conn.get().unwrap())
}

pub fn get_students_who_didnt_evaluate_group(conn: &DBPool, project_id_: Uuid) -> Result<Vec<Student>, DBError> {
    let project_students = get_students_with_group(conn, project_id_)?;
    let project_marks = get_all_marks_of_project(conn, project_id_)?;

    let mut students = Vec::new();
    for student in project_students {
        let mut found = false;
        for mark in project_marks.iter() {
            if mark.grader_student_id == student.id {
                found = true;
                break;
            }
        }

        if !found {
            students.push(student);
        }
    }

    Ok(students)
}

pub fn create_mark(conn: &DBPool, new_mark: NewMark) -> Result<(), DBError> {
    use domain::schema::marks::dsl::*;

    diesel::insert_into(marks)
        .values(&new_mark)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn delete_all_marks_from_student(conn: &DBPool, student_id_: Uuid) -> Result<(), DBError> {
    use domain::schema::marks::dsl::*;

    diesel::delete(marks.filter(noted_student_id.eq(student_id_))
        .filter(grader_student_id.eq(student_id_)))
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::database::groups::test::test_create_group;
    use crate::database::students::test::test_create_student;
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

    fn test_create_mark() -> (Uuid, Uuid, Uuid) {
        let context = TestContext::new();

        let (group_id, project_id) = test_create_group();
        let student_id = test_create_student();
        let new_mark = NewMark {
            project_id,
            group_id,
            noted_student_id: student_id,
            grader_student_id: student_id,
            mark: 10.0,
            max_mark: None,
            comment: Some("test".to_string()),
        };

        create_mark(&context.conn, new_mark).unwrap();

        (group_id, student_id, student_id)
    }

    #[test]
    fn test_get_marks_from_group_id() {
        let context = TestContext::new();

        let (group_id, _, _) = test_create_mark();

        let marks = get_marks_from_group_id(&context.conn, group_id).unwrap();

        assert_eq!(marks.len(), 1);
    }

    #[test]
    fn test_get_marks_given_to_student_id() {
        let context = TestContext::new();

        let (_, student_id, _) = test_create_mark();

        let marks = get_marks_given_to_student_id(&context.conn, student_id).unwrap();

        assert_eq!(marks.len(), 1);
    }

    #[test]
    fn test_get_marks_given_to_student_id_and_group_id() {
        let context = TestContext::new();

        let (group_id, student_id, _) = test_create_mark();

        let marks = get_marks_given_to_student_id_and_group_id(&context.conn, student_id, group_id).unwrap();

        assert_eq!(marks.len(), 1);
    }

    #[test]
    fn test_delete_all_marks_from_student() {
        let context = TestContext::new();

        let (_, student_id, _) = test_create_mark();

        delete_all_marks_from_student(&context.conn, student_id).unwrap();

        let marks = get_marks_given_to_student_id(&context.conn, student_id).unwrap();

        assert_eq!(marks.len(), 0);
    }
}