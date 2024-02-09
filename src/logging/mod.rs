use std::{env, fmt::Debug};

use elasticsearch::{http::transport::Transport, Elasticsearch, IndexParts};
use serde_json::json;

pub struct Logger {
    name: String,
    elastic_client: elasticsearch::Elasticsearch,
}

#[derive(Debug)]
pub enum LogLevel {
    INFO,
    ERROR,
    WARNING,
    DEBUG,
}

impl Logger {
    pub fn new(name: &str) -> Self { // Here i need to see how i put the url inside
        let elastic_url = env::var("ELASTIC_SEARCH_URL").expect("ELASTIC_SEARCH_URL must be set");
        let transport = Transport::single_node(&elastic_url).expect("failed to connect to elasticsearch");
        let client = Elasticsearch::new(transport);      

        Logger {
            name: name.to_string(),
            elastic_client: client
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

        self.elastic_client
            .index(IndexParts::Index("logs"))
            .body(record)
            .send()
            .await?;

        Ok(())
    }

    pub async fn info(&self, message: &str) -> Result<(), elasticsearch::Error> {
        self.log(LogLevel::INFO, message).await
    }

    pub async fn error(&self, message: &str) -> Result<(), elasticsearch::Error> {
        self.log(LogLevel::ERROR, message).await
    }

    pub async fn warning(&self, message: &str) -> Result<(), elasticsearch::Error> {
        self.log(LogLevel::WARNING, message).await
    }

    pub async fn debug(&self, message: &str) -> Result<(), elasticsearch::Error> {
        self.log(LogLevel::DEBUG, message).await
    }
}
