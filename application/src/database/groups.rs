use crate::database::marks::get_marks_given_to_student_id_and_group_id;
use crate::database::projects::get_promotion_from_project_id;
use crate::database::students::{get_student_by_id, get_students_from_promotion_id};
use diesel::prelude::*;
use diesel::result::Error as DBError;
use domain::models::groups::*;
use domain::models::students::Student;
use infrastructure::DBPool;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct ProjectGroup {
    pub group: Group,
    pub students: Vec<StudentGroup>,
}

#[derive(Serialize, ToSchema)]
pub struct StudentGroup {
    pub student: Student,
    pub mark: Option<f64>,
}

#[derive(Serialize, ToSchema)]
pub struct StudentGroupMarkDetails {
    pub student: Student,
    pub marks: Vec<StudentGroupMark>
}

#[derive(Serialize, ToSchema)]
pub struct StudentGroupMark {
    pub grader: Student,
    pub mark: Option<f64>,
    pub max_mark: i32,
    pub comment: Option<String>
}

#[derive(Serialize, ToSchema)]
pub struct MinimalGroupStudents {
    pub group_id: Uuid,
    pub students: Vec<MinimalStudent>,
}

#[derive(Serialize, ToSchema)]
pub struct MinimalStudent {
    pub student_id: Uuid,
    pub name: String,
    pub surname: String,
}

pub fn get_group_id_of_student(conn: &DBPool, student_id_: Uuid, project_id_: Uuid) -> Result<Option<Uuid>, DBError> {
    use domain::schema::groups_students::dsl::*;

    groups_students
        .filter(student_id.eq(student_id_))
        .inner_join(domain::schema::groups::dsl::groups)
        .filter(domain::schema::groups::dsl::project_id.eq(project_id_))
        .select(group_id)
        .first::<Uuid>(&mut conn.get().unwrap())
        .optional()
}

pub fn get_group_by_id(conn: &DBPool, id_: Uuid) -> Result<Group, DBError> {
    use domain::schema::groups::dsl::*;

    groups.filter(id.eq(id_))
        .first(&mut conn.get().unwrap())
}

pub fn get_groups_from_project_id(conn: &DBPool, project_id_: Uuid) -> Result<Vec<Group>, DBError> {
    use domain::schema::groups::dsl::*;

    groups.filter(project_id.eq(project_id_))
        .load::<Group>(&mut conn.get().unwrap())
}

pub fn get_groups_and_students_from_project_id(conn: &DBPool, project_id_: Uuid) -> Result<Vec<ProjectGroup>, DBError> {
    let groups = get_groups_from_project_id(conn, project_id_).unwrap();
    let groups_ids: Vec<Uuid> = groups.iter().map(|group| group.id).collect();

    let mut result: Vec<ProjectGroup> = Vec::new();
    for group_id in groups_ids {
        let students = get_students_and_marks_from_group(conn, group_id)?;
        result.push(ProjectGroup {
            group: get_group_by_id(conn, group_id).unwrap(),
            students
        });
    }

    Ok(result)
}

pub fn get_group_from_student_and_project_id(conn: &DBPool, student_id_: Uuid, project_id_: Uuid) -> Result<Option<Group>, DBError> {
    use domain::schema::groups::dsl::*;
    use domain::schema::groups_students::dsl::*;

    let group_id_ = groups_students
        .filter(student_id.eq(student_id_))
        .inner_join(groups)
        .filter(project_id.eq(project_id_))
        .select(group_id)
        .first::<Uuid>(&mut conn.get().unwrap())
        .optional();

    match group_id_ {
        Ok(Some(group_id_)) => Ok(Some(get_group_by_id(conn, group_id_).unwrap())),
        _ => Ok(None)
    }
}

pub fn get_group_student(conn: &DBPool, group_id_: Uuid, student_id_: Uuid) -> Result<GroupStudent, DBError> {
    use domain::schema::groups_students::dsl::*;

    groups_students
        .filter(group_id.eq(group_id_))
        .filter(student_id.eq(student_id_))
        .first(&mut conn.get().unwrap())
}

pub fn get_students_from_groups(conn: &DBPool, group_ids_: Vec<Uuid>) -> Result<Vec<Student>, DBError> {
    use domain::schema::groups_students::dsl::*;
    use domain::schema::students::dsl::*;

    let students_ids = groups_students
        .filter(group_id.eq_any(group_ids_))
        .select(student_id)
        .load::<Uuid>(&mut conn.get().unwrap())?;

    students.filter(id.eq_any(students_ids))
        .load::<Student>(&mut conn.get().unwrap())
}

pub fn get_students_from_group_for_evaluation(conn: &DBPool, group_id_: Uuid) -> Result<MinimalGroupStudents, DBError> {
    use domain::schema::groups_students::dsl::*;
    use domain::schema::students::dsl::*;

    let student_records = students
        .inner_join(groups_students)
        .filter(group_id.eq(group_id_))
        .select((id, name, surname))
        .load::<(Uuid, String, String)>(&mut conn.get().unwrap())?;

    Ok(MinimalGroupStudents {
        group_id: group_id_,
        students: student_records.into_iter().map(|(sid, sname, ssurname)| {
            MinimalStudent {
                student_id: sid,
                name: sname,
                surname: ssurname
            }
        }).collect()
    })
}

pub fn get_students_and_marks_from_group(conn: &DBPool, group_id_: Uuid) -> Result<Vec<StudentGroup>, DBError> {
    use domain::schema::groups_students::dsl::*;
    use domain::schema::students::dsl::*;

    let result = students
        .inner_join(groups_students)
        .filter(group_id.eq(group_id_))
        .load::<(Student, GroupStudent)>(&mut conn.get().unwrap())?;

    Ok(result.into_iter().map(|(student, group_student)| {
        StudentGroup {
            student,
            mark: group_student.student_mark
        }
    }).collect())
}

pub fn get_students_without_group(conn: &DBPool, project_id_: Uuid) -> Result<Vec<Student>, DBError> {
    let promotion = get_promotion_from_project_id(conn, project_id_)?;
    let students = get_students_from_promotion_id(conn, promotion.id)?;

    // Get all students from all groups of the project
    let groups = get_groups_from_project_id(conn, project_id_)?;
    let groups_ids = groups.iter().map(|group| group.id).collect();
    let groups_students = get_students_from_groups(conn, groups_ids)?;

    // Return array with students from promotion that are not in any group
    Ok(students.into_iter().filter(|student| {
        !groups_students.iter().any(|group_student| group_student.id == student.id)
    }).collect())
}

pub fn get_students_with_group(conn: &DBPool, project_id_: Uuid) -> Result<Vec<Student>, DBError> {
    let promotion = get_promotion_from_project_id(conn, project_id_)?;
    let students = get_students_from_promotion_id(conn, promotion.id)?;

    // Get all students from all groups of the project
    let groups = get_groups_from_project_id(conn, project_id_)?;
    let groups_ids = groups.iter().map(|group| group.id).collect();
    let groups_students = get_students_from_groups(conn, groups_ids)?;

    // Return array with students from promotion that are in a group
    Ok(students.into_iter().filter(|student| {
        groups_students.iter().any(|group_student| group_student.id == student.id)
    }).collect())
}

pub fn get_group_student_mark_details(conn: &DBPool, group_id_: Uuid, student_id_: Uuid) -> Result<StudentGroupMarkDetails, DBError> {
    let graded_student = get_student_by_id(conn, student_id_)?;

    let group_students = get_students_from_groups(conn, vec![group_id_])?;
    let mut filtered_group_students: Vec<Student> = group_students.into_iter().filter(|student| student.id != student_id_).collect();

    let marks = get_marks_given_to_student_id_and_group_id(conn, student_id_, group_id_)?;
    let mut student_group_marks: Vec<StudentGroupMark> = Vec::new();
    for value in marks {
        let grader = get_student_by_id(conn, value.grader_student_id)?;
        filtered_group_students = filtered_group_students.into_iter().filter(|student| student.id != grader.id).collect();
        student_group_marks.push(StudentGroupMark {
            grader,
            mark: Some(value.mark),
            max_mark: value.max_mark,
            comment: value.comment
        });
    }

    if filtered_group_students.len() > 0 {
        for student in filtered_group_students {
            student_group_marks.push(StudentGroupMark {
                grader: student,
                mark: None,
                max_mark: 20,
                comment: None
            });
        }
    }

    Ok(StudentGroupMarkDetails {
        student: graded_student,
        marks: student_group_marks,
    })
}

pub fn create_group(conn: &DBPool, new_group: NewGroup) -> Result<Uuid, DBError> {
    use domain::schema::groups::dsl::*;

    let result: Result<Uuid, DBError> = diesel::insert_into(groups)
        .values(&new_group)
        .returning(id)
        .get_result(&mut conn.get().unwrap());

    result
}

pub fn create_group_students(conn: &DBPool, new_group_students: Vec<NewGroupStudent>) -> Result<(), DBError> {
    use domain::schema::groups_students::dsl::*;

    diesel::insert_into(groups_students)
        .values(&new_group_students)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn update_group(conn: &DBPool, id_: Uuid, updated_group: UpdatedGroup) -> Result<(), DBError> {
    use domain::schema::groups::dsl::*;

    // Check if the student exists
    groups.filter(id.eq(id_.clone()))
        .first::<Group>(&mut conn.get().unwrap())?;

    diesel::update(groups.filter(id.eq(id_)))
        .set(&updated_group)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn update_group_student(conn: &DBPool, group_id_: Uuid, student_id_: Uuid, updated_group_student: UpdatedGroupStudent) -> Result<(), DBError> {
    use domain::schema::groups_students::dsl::*;

    // Check if the student exists
    groups_students
        .filter(group_id.eq(group_id_))
        .filter(student_id.eq(student_id_))
        .first::<GroupStudent>(&mut conn.get().unwrap())?;

    diesel::update(groups_students
        .filter(group_id.eq(group_id_))
        .filter(student_id.eq(student_id_))
    ).set(&updated_group_student)
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn delete_group(conn: &DBPool, id_: Uuid) -> Result<(), DBError> {
    use domain::schema::groups::dsl::*;

    // Check if the student exists
    groups.filter(id.eq(id_.clone()))
        .first::<Group>(&mut conn.get().unwrap())?;

    diesel::delete(groups.filter(id.eq(id_)))
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn delete_group_student(conn: &DBPool, group_id_: Uuid, student_id_: Uuid) -> Result<(), DBError> {
    use domain::schema::groups_students::dsl::*;

    diesel::delete(groups_students
        .filter(group_id.eq(group_id_))
        .filter(student_id.eq(student_id_))
    ).execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn remove_students_from_groups(conn: &DBPool, student_id_: Uuid) -> Result<(), DBError> {
    use domain::schema::groups_students::dsl::*;

    diesel::delete(groups_students.filter(student_id.eq(student_id_)))
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

pub fn remove_all_students_from_a_group(conn: &DBPool, group_id_: Uuid) -> Result<(), DBError> {
    use domain::schema::groups_students::dsl::*;

    diesel::delete(groups_students.filter(group_id.eq(group_id_)))
        .execute(&mut conn.get().unwrap())?;

    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::database::projects::test::test_create_project;
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

    pub fn test_create_group() -> (Uuid, Uuid) {
        let context = TestContext::new();

        let (project_id, _) = test_create_project();
        let random = Uuid::new_v4();
        let new_group = NewGroup{
            name: format!("test-{}", random),
            project_id,
            max_mark: None,
        };

        (create_group(&context.conn, new_group).unwrap(), project_id)
    }

    fn test_create_group_student() -> (Uuid, Uuid) {
        let context = TestContext::new();

        let (group_id, _) = test_create_group();
        let student_id = test_create_student();

        let new_group_student = vec![NewGroupStudent {
            group_id,
            student_id
        }];

        create_group_students(&context.conn, new_group_student).unwrap();

        (group_id, student_id)
    }

    #[test]
    fn test_get_group_by_id() {
        let context = TestContext::new();

        let (group_id, _) = test_create_group();

        let group = get_group_by_id(&context.conn, group_id).unwrap();
        assert_eq!(group_id, group.id)
    }

    #[test]
    fn test_get_group_by_project_id() {
        let context = TestContext::new();

        let (group_id, project_id) = test_create_group();

        let groups = get_groups_from_project_id(&context.conn, project_id).unwrap();
        assert_eq!(groups[0].id, group_id);
    }

    #[test]
    fn test_get_students_without_group() {
        let context = TestContext::new();

        let (project_id, _) = test_create_project();

        let students = get_students_without_group(&context.conn, project_id).unwrap();
    }

    #[test]
    fn test_update_group() {
        let context = TestContext::new();

        let (group_id, _) = test_create_group();

        let random = Uuid::new_v4();
        let updated_group = UpdatedGroup {
            name: Some(format!("updated-{}", random)),
            mark: None,
            max_mark: None,
        };
        update_group(&context.conn, group_id, updated_group).unwrap();

        let group = get_group_by_id(&context.conn, group_id).unwrap();
        assert_eq!(group.name, format!("updated-{}", random));
    }

    #[test]
    fn test_update_group_student() {
        let context = TestContext::new();

        let (group_id, student_id) = test_create_group_student();

        let updated_group_student = UpdatedGroupStudent {
            student_mark: Some(10.0),
        };
        update_group_student(&context.conn, group_id, student_id, updated_group_student).unwrap();

        let group_student = get_group_student(&context.conn, group_id, student_id).unwrap();
        assert_eq!(group_student.student_mark, Some(10.0));
    }

    #[test]
    fn test_delete_group() {
        let context = TestContext::new();

        let (group_id, _) = test_create_group();

        delete_group(&context.conn, group_id).unwrap();
    }

    #[test]
    fn test_delete_group_student() {
        let context = TestContext::new();

        let (group_id, student_id) = test_create_group_student();

        delete_group_student(&context.conn, group_id, student_id).unwrap();
    }

    fn test_remove_student_from_groups() {
        let context = TestContext::new();

        let (group_id, student_id) = test_create_group_student();

        remove_students_from_groups(&context.conn, student_id).unwrap();
    }
}