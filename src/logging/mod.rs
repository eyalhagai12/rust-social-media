use std::{env, fmt::Debug};
use elasticsearch::{http::transport::Transport, Elasticsearch, IndexParts};
use lazy_static::lazy_static;
use log::{debug, error, info, warn};
use serde_json::json;

pub struct Logger {
    name: String,
}

#[derive(Debug)]
pub enum LogLevel {
    INFO,
    ERROR,
    WARNING,
    DEBUG,
}

lazy_static! {
    static ref ELASTIC_CLIENT: Elasticsearch = {
        let elastic_url = env::var("ELASTIC_SEARCH_URL").expect("ELASTIC_SEARCH_URL must be set");
        let transport = Transport::single_node(&elastic_url).expect("failed to connect to elasticsearch");
        Elasticsearch::new(transport)
    };
}

impl Logger {
    pub fn new(name: &str) -> Self {
        Logger {
            name: name.to_string(),
        }
    }

    async fn log(&self, level: LogLevel, message: &str) -> Result<(), elasticsearch::Error> {
        let level_str: &str = match level {
            LogLevel::INFO => "INFO",
            LogLevel::WARNING => "WARNING",
            LogLevel::ERROR => "ERROR",
            LogLevel::DEBUG => "DEBUG",
        };

        let record = json!({
            "module": self.name,
            "Level": level_str,
            "message": message,
        });

        ELASTIC_CLIENT
            .index(IndexParts::Index("logs"))
            .body(record)
            .send()
            .await?;

        Ok(())
    }

    pub async fn info(&self, message: &str) -> Result<(), elasticsearch::Error> {
        info!("{}", message);
        self.log(LogLevel::INFO, message).await
    }

    pub async fn error(&self, message: &str) -> Result<(), elasticsearch::Error> {
        error!("{}", message);
        self.log(LogLevel::ERROR, message).await
    }

    pub async fn warning(&self, message: &str) -> Result<(), elasticsearch::Error> {
        warn!("{}", message);
        self.log(LogLevel::WARNING, message).await
    }

    pub async fn debug(&self, message: &str) -> Result<(), elasticsearch::Error> {
        debug!("{}", message);
        self.log(LogLevel::DEBUG, message).await
    }
}
