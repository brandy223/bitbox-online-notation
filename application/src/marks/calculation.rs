use crate::database::groups::{get_group_student_mark_details, get_groups_from_project_id, get_students_from_groups};
use indexmap::IndexMap;
use infrastructure::DBPool;
use shared::error_models::DBError;
use uuid::Uuid;

pub fn calculate_students_marks_from_project(conn: &DBPool, project_id_: Uuid) -> Result<IndexMap<(Uuid, Uuid), Option<f64>>, DBError> {
    let mut students_marks: IndexMap<(Uuid, Uuid), Option<f64>> = IndexMap::new();

    // Get all students from project that are in a group
    let groups = get_groups_from_project_id(conn, project_id_)?;
    for group in groups {
        let group_avg = calculate_group_average(conn, group.id);
        let students = get_students_from_groups(conn, vec![group.id])?;
        for student in students {
            let details = get_group_student_mark_details(conn, group.id, student.id)?;
            let marks: Vec<f64> = details.marks.iter().filter_map(|detail| detail.mark).collect();
            let average = calculate_average(&marks);
            let delta = group_avg - average;
            let mark = match group.mark {
                Some(mark) => if delta > 0.0 { Some(mark - delta) } else { Some(mark) },
                None => None,
            };
            students_marks.insert((group.id, student.id), mark);
        }
    }

    Ok(students_marks)
}

fn calculate_group_average(conn: &DBPool, group_id: Uuid) -> f64 {
    let students = get_students_from_groups(conn, vec![group_id]).unwrap();
    let mut students_avg: Vec<f64> = Vec::new();
    for student in students {
        let details = get_group_student_mark_details(conn, group_id, student.id).unwrap();
        let marks: Vec<f64> = details.marks.iter().filter_map(|detail| detail.mark).collect();
        students_avg.push(calculate_average(&marks));
    }

    calculate_average(&students_avg)
}

fn calculate_average(numbers: &[f64]) -> f64 {
    if numbers.is_empty() { return 0.0; }

    let sum: f64 = numbers.iter().sum();
    let count = numbers.len() as f64;

    sum / count
}