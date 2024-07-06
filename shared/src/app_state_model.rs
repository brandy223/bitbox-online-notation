use std::sync::Arc;

use lettre::SmtpTransport;
use parking_lot::RwLock;

use infrastructure::DBPool;

use crate::app_config::Config;

pub struct AppState {
    pub database_pool: Arc<DBPool>,
    pub smtp_transport: Arc<SmtpTransport>,
    pub config: Arc<RwLock<Config>>,
}

impl AppState {
    pub fn init(database_pool: DBPool, smtp_transport: SmtpTransport, config: Config) -> AppState {
        let database_pool = Arc::new(database_pool);
        let smtp_transport = Arc::new(smtp_transport);
        let config = Arc::new(RwLock::new(config));

        AppState {
            database_pool,
            smtp_transport,
            config,
        }
    }
}