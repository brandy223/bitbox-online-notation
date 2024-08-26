use actix_web::rt::task::JoinHandle;
use infrastructure::DBPool;
use lettre::SmtpTransport;
use parking_lot::{Mutex, RwLock};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::app_config::Config;

// type TimeoutFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

pub struct ProjectTimeouts {
    // timeouts: Vec<(TimeoutFuture, Instant)>,
    pub timeouts: Vec<JoinHandle<()>>,
}

pub struct RuntimeValues {
    pub projects_checker: Arc<Mutex<Option<u64>>>,
    pub project_reminders: Arc<Mutex<HashMap<Uuid, ProjectTimeouts>>>,
}

impl RuntimeValues {
    pub fn init() -> RuntimeValues {
        let projects_checker = Arc::new(Mutex::new(None));
        let project_reminders = Arc::new(Mutex::new(HashMap::new()));

        RuntimeValues {
            projects_checker,
            project_reminders,
        }
    }
}

pub struct AppState {
    pub database_pool: Arc<DBPool>,
    pub smtp_transport: Arc<SmtpTransport>,
    pub config: Arc<RwLock<Config>>,
    pub runtime_values: Arc<RwLock<RuntimeValues>>,
}

impl AppState {
    pub fn init(database_pool: DBPool, smtp_transport: SmtpTransport, config: Config) -> AppState {
        let database_pool = Arc::new(database_pool);
        let smtp_transport = Arc::new(smtp_transport);
        let config = Arc::new(RwLock::new(config));
        let runtime_values = Arc::new(RwLock::new(RuntimeValues::init()));

        AppState {
            database_pool,
            smtp_transport,
            config,
            runtime_values,
        }
    }

    pub fn get_all_project_ids(&self) -> Vec<Uuid> {
        let runtime_values = self.runtime_values.read();
        let project_reminders = runtime_values.project_reminders.lock();
        project_reminders.keys().cloned().collect()
    }

    pub fn get_project_reminders(&self, project_id: Uuid) -> Option<ProjectTimeouts> {
        let mut runtime_values = self.runtime_values.write();
        let mut project_reminders = runtime_values.project_reminders.lock();
        project_reminders.remove(&project_id)
    }

    pub fn set_project_reminders(&self, project_id: Uuid, project_timeouts: ProjectTimeouts) {
        let mut runtime_values = self.runtime_values.write();
        let mut project_reminders = runtime_values.project_reminders.lock();
        project_reminders.insert(project_id, project_timeouts);
    }
}