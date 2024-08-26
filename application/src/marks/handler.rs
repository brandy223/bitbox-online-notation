use crate::database::groups::update_group_student;
use crate::marks::calculation::calculate_students_marks_from_project;
use domain::models::groups::UpdatedGroupStudent;
use infrastructure::DBPool;
use shared::error_models::DBError;
use uuid::Uuid;

pub fn handle_project_rating(conn: &DBPool, project_id_: Uuid) -> Result<(), DBError> {
    let students_marks = calculate_students_marks_from_project(conn, project_id_)?;

    for ((group_id, student_id), mark) in students_marks {
        let updated_group_student = UpdatedGroupStudent {
            student_mark: mark,
        };

        update_group_student(conn, group_id, student_id, updated_group_student)?;
    }

    Ok(())
}