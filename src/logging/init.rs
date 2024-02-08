use actix_web::Error;
use elasticsearch::{http::transport::Transport, Elasticsearch};
use slog::Drain;

pub struct ElasticDrain {
    pub client: Elasticsearch,
}

impl ElasticDrain {
    pub fn new(url: &str) -> Result<Self, Error> {
        let transport = Transport::single_node(url).unwrap();
        let client = Elasticsearch::new(transport);

        Ok(ElasticDrain { client })
    }
}

impl Drain for ElasticDrain {
    type Ok = ();
    type Err = slog::Error;

    fn log(
        &self,
        record: &slog::Record,
        values: &slog::OwnedKVList,
    ) -> std::result::Result<Self::Ok, Self::Err> {
        todo!()
    }
}
