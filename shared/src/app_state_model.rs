use std::sync::Arc;

use infrastructure::DBPool;
use crate::app_config::Config;

pub struct AppState {
    pub database_pool: Arc<DBPool>,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn init(config: Config) -> AppState {
        let database_url = config.database_url.clone();
        let database_pool = Arc::new(infrastructure::init_pool(&database_url));
        let config = Arc::new(config);

        AppState {
            database_pool,
            config,
        }
    }
}