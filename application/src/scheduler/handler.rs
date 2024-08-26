use crate::authentication::tokens::encode_token;
use crate::database::config::get_config_by_user_id;
use crate::database::done_alerts::{create_done_alert, get_done_alerts_by_project_id_and_type};
use crate::database::groups::{get_group_from_student_and_project_id, get_groups_and_students_from_project_id, get_groups_from_project_id, ProjectGroup};
use crate::database::marks::get_students_who_didnt_evaluate_group;
use crate::database::projects::{get_project_by_id, update_project};
use crate::database::promotions::get_promotion_by_id;
use crate::database::students_tokens::{create_student_token, get_student_tokens_from_student_and_project_id};
use crate::database::users::get_user_by_id;
use crate::mail::send::{build_mail, send_mail, MailProps};
use crate::marks::handler::handle_project_rating;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Timelike, Utc};
use domain::models::config::UserConfig;
use domain::models::done_alerts::{AlertType, NewDoneAlert};
use domain::models::groups::Group;
use domain::models::projects::{Project, ProjectState, UpdatedProject};
use domain::models::students::Student;
use domain::models::students_tokens::NewStudentToken;
use infrastructure::DBPool;
use lettre::SmtpTransport;
use shared::app_config::Config;
use shared::app_state_model::{AppState, ProjectTimeouts};
use shared::error_models::{APIError, DBError, InternalError, ServerError};
use shared::token_models::StudentClaims;
use std::env;
use tokio::task::JoinHandle;
use tokio_js_set_interval::set_timeout;
use uuid::Uuid;

pub fn handle_projects_alerts(app_state: &AppState, projects: Vec<Project>) {
    let today = Utc::now().date_naive();

    let mut starting_alerts: Vec<Project> = Vec::new();
    let mut ending_alerts: Vec<Project> = Vec::new();
    let mut pending_alerts: Vec<Project> = Vec::new();

    for project in projects {
        let project_notation_end_date: NaiveDate = project.end_date.date() + Duration::days(project.notation_period_duration as i64);

        println!("End Date : {:?}", project.end_date.date());
        println!("Notation End Date : {:?}", project_notation_end_date);
        println!("Today : {:?}", today);
        println!("Check : {:?}", project.end_date.date() < today && project_notation_end_date > today);

        if project.end_date.date() == today {
            starting_alerts.push(project);
        } else if project_notation_end_date == today {
            ending_alerts.push(project);
        } else if project.end_date.date() < today && project_notation_end_date > today {
            pending_alerts.push(project);
        }
    }

    if !starting_alerts.is_empty() { handle_starting_alerts(app_state, starting_alerts) }
    if !ending_alerts.is_empty() { handle_ending_alerts(app_state, ending_alerts) }
    if !pending_alerts.is_empty() { handle_pending_alerts(app_state, pending_alerts) }
}

fn handle_starting_alerts(app_state: &AppState, projects: Vec<Project>) {
    let conn = app_state.database_pool.clone().as_ref().clone();
    let smtp_transport = app_state.smtp_transport.clone().as_ref().clone();

    for project in projects {
        if !should_send_alert(&conn, project.id, AlertType::Started) {
            continue;
        }

        match get_groups_and_students_from_project_id(&conn, project.id) {
            Ok(groups) => {
                for project_group in groups {
                    process_group_tokens(app_state, &project, &project_group);

                    // Get teacher email
                    let teacher_email = match get_teacher_email(&conn, project.promotion_id) {
                        Ok(email) => email,
                        Err(_) => {
                            log_error("Failed to get teacher email");
                            continue;
                        }
                    };

                    // Build email for teacher
                    let mail = build_mail(MailProps {
                        from: "Bitbox <no-reply@sigma-bot.fr>".to_string(),
                        to: teacher_email.to_string(),
                        subject: "360 Notation has begun".to_string(),
                        body: format!("The evaluation for the project \"{}\" of the promotion (TODO) has now begun.\
                            \nThanks to enter all the groups' marks before the end of the timing.\
                            \nRemaining time : {} days", project.name, project.notation_period_duration.to_string()),
                    });

                    // Send email to teacher
                    if let Err(e) = send_mail(&smtp_transport, mail) {
                        log_error(&format!("Failed to send email: {:?}", e));
                    }
                }
            }
            Err(_) => {
                log_error(&format!("Failed to get groups for project {}", project.id));
                continue;
            }
        }

        mark_alert_as_done(&conn, project.id, AlertType::Started);
    }
}

fn handle_pending_alerts(app_state: &AppState, projects: Vec<Project>) -> () {
    let conn = app_state.database_pool.clone().as_ref().clone();
    for project in projects {
        // Load teacher config
        let teacher_config =  match get_teacher_config(&conn, project.promotion_id) {
            Ok(config) => config,
            Err(_) => {
                log_error("Failed to get teacher config");
                continue;
            }
        };
        let alerts_timestamps = calculate_alerts_timestamps(&teacher_config, &project);
        let done_alerts = match get_done_alerts_by_project_id_and_type(&conn, project.id, AlertType::Pending) {
            Ok(alerts) => alerts,
            Err(_) => {
                log_error("Failed to get done alerts");
                continue;
            }
        };

        // Filter alerts array by comparing with timestamps of done alerts
        let alerts_to_send: Vec<DateTime<Utc>> = alerts_timestamps.iter()
            .filter(|timestamp|
            done_alerts.iter().all(|alert|
                alert.published_at.date() != timestamp.date_naive() ||
                alert.published_at.hour() != timestamp.hour()
            )
            )
            .cloned()
            .collect();

        println!("Alerts to send : {:?}", alerts_to_send);

        if alerts_to_send.is_empty() { continue; }

        // Generate set_timout functions for each alert
        let mut timeouts: Vec<JoinHandle<()>> = Vec::new();
        for alert in alerts_to_send {
            timeouts.push(match generate_pending_alert_timeout_function(&conn, app_state.smtp_transport.clone().as_ref(), alert, project.id) {
                Ok(timeout) => timeout,
                Err(_) => {
                    log_error("Failed to generate timeout function");
                    continue;
                }
            })
        }

        println!("Timeouts : {:?}", timeouts);

        // Modify App State
        app_state.set_project_reminders(project.id, ProjectTimeouts {
            timeouts,
        });
    }
}

fn handle_ending_alerts(app_state: &AppState, projects: Vec<Project>) {
    let conn = app_state.database_pool.clone().as_ref().clone();
    let smtp_transport = app_state.smtp_transport.clone().as_ref().clone();

    for project in projects {
        if !should_send_alert(&conn, project.id, AlertType::Finished) {
            continue;
        }

        let students = match get_students_for_project(&conn, project.id) {
            Ok(students) if !students.is_empty() => students,
            _ => {
                log_error("No students found for project");
                continue;
            }
        };

        let teacher_email = match get_teacher_email(&conn, project.promotion_id) {
            Ok(email) => email,
            Err(_) => {
                log_error("Failed to get teacher email");
                continue;
            }
        };

        let mut mails_list: Vec<String> = students.iter().map(|student| student.email.clone()).collect();
        mails_list.push(teacher_email);

        for email in mails_list {
            let mail = build_mail(MailProps {
                from: "Bitbox <no-reply@sigma-bot.fr>".to_string(),
                to: email.to_string(),
                subject: "360 Notation has ended".to_string(),
                body: format!("The evaluation for the project \"{}\" has now ended.\nThanks for your participation.", project.name),
            });

            if let Err(e) = send_mail(&smtp_transport, mail) {
                log_error(&format!("Failed to send email: {:?}", e));
            }
        }

        if let Err(e) = handle_project_rating(&conn, project.id) {
            log_error(&format!("Failed to handle project rating: {:?}", e));
            continue;
        }

        mark_alert_as_done(&conn, project.id, AlertType::Finished);
        update_project_state(&conn, project.id, ProjectState::NotationFinished);
    }
}

fn process_group_tokens(app_state: &AppState, project: &Project, project_group: &ProjectGroup) -> () {
    let conn = app_state.database_pool.clone().as_ref().clone();
    let config = app_state.config.read();
    let smtp_transport = app_state.smtp_transport.clone().as_ref().clone();

    let stop_date_time = project.end_date + Duration::days(project.notation_period_duration as i64);
    let stop_date = DateTime::<Utc>::from_naive_utc_and_offset(stop_date_time, Utc);

    for student_group in &project_group.students {
        let student_info = &student_group.student;
        let token = match generate_student_token(&config, student_info.id, project_group.group.id, stop_date) {
            Ok(token) => token,
            Err(e) => {
                log_error(&format!("Failed to generate token: {:?}", e));
                return;
            }
        };

        let new_student_token = NewStudentToken {
            token: token.to_string(),
            student_id: student_info.id,
            project_id: project.id,
        };
        let token_id = match create_student_token(&conn, new_student_token) {
            Ok(id) => id,
            Err(e) => {
                log_error(&format!("Failed to create token: {:?}", e));
                return;
            }
        };

        let web_url = env::var("WEB_URL").unwrap_or("http://localhost:3000".to_string());
        let url = format!("{}/evaluate/{}", web_url, token_id);
        let mail = build_mail(MailProps {
            from: "Bitbox <no-reply@sigma-bot.fr>".to_string(),
            to: student_info.email.clone(),
            subject: "360 Notation has begun".to_string(),
            body: format!(
                "Now's the time to evaluate your peers from group \"{}\" on project \"{}\".\nHere's the link : {}\
                \nRemaining time : {} days",
                project_group.group.name, project.name, url, project.notation_period_duration.to_string()
            ),
        });

        if let Err(e) = send_mail(&smtp_transport, mail) {
            log_error(&format!("Failed to send email: {:?}", e));
        }
    }
}

fn generate_pending_alert_timeout_function(conn: &DBPool, smtp_transport: &SmtpTransport, alert_datetime: DateTime<Utc>, project_id: Uuid) -> Result<JoinHandle<()>, APIError> {
    // Calculate ms between now and alert_datetime
    let ms = alert_datetime.timestamp_millis() - Utc::now().timestamp_millis();
    let interval = ms as u64;

    // Get project info
    let project = match get_project_by_id(&conn, project_id) {
        Ok(project) => project,
        Err(_) => {
            log_error("Failed to get project");
            return Err(APIError::ServerError(ServerError::InternalError(InternalError)));
        }
    };
    let end_date = project.end_date + Duration::days(project.notation_period_duration as i64);

    // Clone the necessary data
    let conn = conn.clone();
    let smtp_transport = smtp_transport.clone();

    let timeout = set_timeout!(move || {
        // Get all students from project who hasn't evaluated their group
        let students = match get_students_who_didnt_evaluate_group(&conn, project_id) {
            Ok(students) => students,
            Err(_) => {
                log_error("Failed to get students who didn't evaluate group");
                return;
            }
        };

        // Send email to each student
        for student in students {
            // Get token
            let token_id = match get_student_tokens_from_student_and_project_id(&conn, student.id, project_id)
                .map(|token| token.id) {
                Ok(id) => id,
                Err(_) => {
                    log_error("Failed to get token");
                    return;
                }
            };

            // Get group
            let group = match get_group_from_student_and_project_id(&conn, student.id, project_id) {
                Ok(group) => match group {
                    Some(group) => Some(group),
                    None => {
                        log_error("Student is not in a group");
                        return;
                    },
                },
                Err(_) => {
                    log_error("Failed to get group");
                    return;
                }
            };

            // Send reminder email to student
            match send_reminder_to_student(&smtp_transport, &student, &group.unwrap(), &project, end_date, token_id) {
                Ok(_) => (),
                Err(_) => {
                    log_error("Failed to send reminder to student");
                    return;
                }
            };
        }

        // Check if teacher has given a note to each group
        let check = match are_all_groups_from_project_evaluated(&conn, project_id) {
            Ok(check) => check,
            Err(_) => {
                log_error("Failed to check if all groups are evaluated");
                return;
            }
        };

        // Send email to teacher if not
        if !check {
            // Get teacher email
            let teacher_email = match get_teacher_email(&conn, project.promotion_id) {
                Ok(email) => email,
                Err(_) => {
                    log_error("Failed to get teacher email");
                    return;
                }
            };

            // Send email
            match send_reminder_to_teacher(&smtp_transport, &teacher_email, &project, &end_date) {
                Ok(_) => (),
                Err(_) => {
                    log_error("Failed to send reminder to teacher");
                    return;
                }
            };
        }

        // Mark alert as done
        mark_alert_as_done(&conn, project_id, AlertType::Pending);
    }, interval);

    Ok(timeout)
}

fn are_all_groups_from_project_evaluated(conn: &DBPool, project_id: Uuid) -> Result<bool, DBError> {
    let groups = get_groups_from_project_id(conn, project_id)?;
    let mut flag: bool = false;
    for group in groups {
        if group.mark.is_none() {
            flag = true;
            break;
        }
    }

    Ok(!flag)
}

fn get_students_for_project(conn: &DBPool, project_id: Uuid) -> Result<Vec<Student>, DBError> {
    get_groups_and_students_from_project_id(conn, project_id)
        .map(|groups| groups.into_iter()
            .flat_map(|group| group.students)
            .map(|student_group| student_group.student)
            .collect())
}

fn get_teacher_email(conn: &DBPool, promotion_id: Uuid) -> Result<String, DBError> {
    let promotion = get_promotion_by_id(conn, promotion_id)?;
    let teacher = get_user_by_id(conn, promotion.teacher_id)?;
    Ok(teacher.email)
}

fn get_teacher_config(conn: &DBPool, promotion_id: Uuid) -> Result<UserConfig, DBError> {
    let promotion = get_promotion_by_id(conn, promotion_id)?;
    let teacher = get_user_by_id(conn, promotion.teacher_id)?;
    get_config_by_user_id(conn, teacher.id)
}

fn send_reminder_to_student(
    smtp_transport: &SmtpTransport,
    student: &Student,
    group: &Group,
    project: &Project,
    end_date: NaiveDateTime,
    token_id: Uuid
) -> Result<(), APIError> {
    // Build email
    let web_url = env::var("WEB_URL").unwrap_or("http://localhost:3000".to_string());
    let url = format!("{}/evaluate/{}", web_url, token_id);
    let mail = build_mail(MailProps {
        from: "Bitbox <no-reply@sigma-bot.fr>".to_string(),
        to: student.email.clone(),
        subject: "360 Notation Reminder".to_string(),
        body: format!(
            "Please evaluate your colleagues from group \"{}\" on project \"{}\".\
                    \nHere's the link : {}\
                    \nYou have till the {} to complete the evaluation.",
            group.name, project.name, url, end_date.date().to_string()
        ),
    });

    // Send mail
    send_mail(smtp_transport, mail)
}

fn send_reminder_to_teacher(smtp_transport: &SmtpTransport, teacher_email: &String, project: &Project, end_date: &NaiveDateTime) -> Result<(), APIError> {
    // Build email
    let mail = build_mail(MailProps {
        from: "Bitbox <no-reply@sigma-bot.fr>".to_string(),
        to: teacher_email.to_string(),
        subject: "Marks Reminder".to_string(),
        body: format!(
            "Please enter all the notes on the platform for the groups of project \"{}\".\
                    \nYou have till the {} to complete the evaluation.",
            project.name, end_date.date().to_string()
        ),
    });

    // Send mail
    send_mail(smtp_transport, mail)
}

fn calculate_alerts_timestamps(teacher_config: &UserConfig, project: &Project) -> Vec<DateTime<Utc>> {
    let mut timestamps: Vec<DateTime<Utc>> = Vec::new();

    for alert in &teacher_config.alerts {
        if let Some(alert) = alert {
            let alert_time = Duration::hours(alert.hours as i64);
            let project_evaluation_end_date = project.end_date + Duration::days(project.notation_period_duration as i64);
            let alert_date = if alert.before_event {
                project_evaluation_end_date - alert_time
            } else {
                project.end_date + alert_time
            };

            let alert_timestamp = DateTime::from_naive_utc_and_offset(alert_date, Utc);
            timestamps.push(alert_timestamp);
        }
    }

    timestamps
}

fn should_send_alert(conn: &DBPool, project_id: Uuid, alert_type: AlertType) -> bool {
    match get_done_alerts_by_project_id_and_type(conn, project_id, alert_type) {
        Ok(alerts) => alerts.is_empty(),
        Err(_) => {
            log_error(&format!("Failed to check existing alerts for project {}", project_id));
            false
        }
    }
}

fn generate_student_token(config: &Config, student_id: Uuid, group_id: Uuid, stop_date: DateTime<Utc>) -> Result<String, APIError> {
    let claims = StudentClaims {
        sub: student_id,
        group_id,
        exp: stop_date.timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
    };
    encode_token::<StudentClaims>(&claims, config)
}

fn mark_alert_as_done(conn: &DBPool, project_id: Uuid, alert_type: AlertType) {
    let new_done_alert = NewDoneAlert {
        description: None,
        project_id,
        type_: alert_type,
    };

    if let Err(e) = create_done_alert(conn, new_done_alert) {
        log_error(&format!("Failed to create done alert for project {}: {:?}", project_id, e));
    }
}

fn update_project_state(conn: &DBPool, project_id: Uuid, state: ProjectState) {
    let updated_project = UpdatedProject {
        name: None,
        description: None,
        start_date: None,
        end_date: None,
        notation_period_duration: None,
        state: Some(state),
    };
    if let Err(e) = update_project(&conn, project_id, updated_project) {
        log_error(&format!("Failed to update project status: {:?}", e));
    }
}

fn log_error(message: &str) {
    eprintln!("Error: {}", message);
    // TODO: Implement proper logging
}