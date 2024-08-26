use crate::database::projects::{get_current_projects, update_project};
use crate::scheduler::handler::handle_projects_alerts;
use domain::models::projects::{Project, ProjectState, UpdatedProject};
use shared::app_state_model::AppState;
use shared::error_models::DBError;
use std::sync::Arc;
use tokio_js_set_interval;
use tokio_js_set_interval::set_interval;

pub fn init_projects_check(app_state: &AppState) -> u64 {
    let database_pool = Arc::clone(&app_state.database_pool);
    let smtp_transport = Arc::clone(&app_state.smtp_transport);
    let config = Arc::clone(&app_state.config);
    let runtime_values = Arc::clone(&app_state.runtime_values);
    let interval = 60*60*24*1000;

    let projects = check_projects(&app_state);
    match projects {
        Ok(projects) => handle_projects_alerts(&app_state, projects),
        Err(e) => eprintln!("Error while checking projects: {:?}", e),
    }

    // Create a new interval that runs every 24 hours
    set_interval!(move || {
        let database_pool = Arc::clone(&database_pool);
        let smtp_transport = Arc::clone(&smtp_transport);
        let config = Arc::clone(&config);
        let runtime_values = Arc::clone(&runtime_values);

        tokio::spawn(async move {
            let app_state = AppState {
                database_pool,
                smtp_transport,
                config,
                runtime_values,
            };

            let projects = check_projects(&app_state);
            match projects {
                Ok(projects) => handle_projects_alerts(&app_state, projects),
                Err(e) => eprintln!("Error while checking projects: {:?}", e),
            }
        });
    }, interval)
}

fn check_projects(app_state: &AppState) -> Result<Vec<Project>, DBError> {
    let conn = app_state.database_pool.clone().as_ref().clone();
    let current_projects_lists = app_state.get_all_project_ids();
    let projects = get_current_projects(&conn)?;
    let mut sorted_projects: Vec<Project> = Vec::new();

    // Update project state if needed
    for project in projects.iter().clone() {
        let now = chrono::Utc::now().date_naive();
        let mut state: ProjectState = project.state.clone();

        // Check dates
        if project.start_date.date() <= now && project.end_date.date() > now {
            state = ProjectState::InProgress;
        } else if project.end_date.date() == now {
            state = ProjectState::Finished;
        }

        // Update project in DB
        let updated_project = UpdatedProject {
            name: None,
            description: None,
            start_date: None,
            end_date: None,
            notation_period_duration: None,
            state: Some(state),
        };
        let _ = update_project(&conn, project.id, updated_project);
        // TODO : Add logging

        // Add modified value
        if state == ProjectState::NotStarted || state == ProjectState::InProgress { continue; }
        sorted_projects.push(Project {
            id: project.id,
            name: project.name.clone(),
            description: project.description.clone(),
            start_date: project.start_date.clone(),
            end_date: project.end_date.clone(),
            notation_period_duration: project.notation_period_duration.clone(),
            promotion_id: project.promotion_id,
            state,
        });
    }

    // Return all projects that are not in the current_projects_list
    let next_projects = projects.into_iter().filter(|project| {
        !current_projects_lists.contains(&project.id)
    })
        .collect();

    Ok(next_projects)
}