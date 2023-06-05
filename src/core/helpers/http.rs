use serde::Deserialize;
use std::str::FromStr;
use uuid::{Error, Uuid};

#[derive(Deserialize)]
pub struct QueryParams {
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Deserialize)]
pub struct IdPathParam {
    pub id: String,
}

impl IdPathParam {
    pub fn get_uuid(&mut self) -> Result<Uuid, Error> {
        Uuid::from_str(self.id.clone().as_str())
    }
}

impl QueryParams {
    pub fn get_search_query(&mut self) -> String {
        match self.search.clone() {
            None => String::from(""),
            Some(q) => q,
        }
    }

    #[allow(dead_code)]
    pub fn get_limit(&mut self) -> i64 {
        self.limit.unwrap_or(10)
    }

    pub fn get_page(&mut self) -> i64 {
        self.page.unwrap_or(1)
    }

    pub fn get_per_page(&mut self) -> i64 {
        self.per_page.unwrap_or(10)
    }
}
