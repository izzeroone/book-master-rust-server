use crate::config::DATE_FORMAT;
use crate::models::user::User;
use chrono::{DateTime, Utc, Datelike};
use serde::Serialize;
use std::convert::From;
#[derive(Queryable)]
pub struct BookMaster {
    pub book_id: String,
    pub book_name: String,
    pub author: String,
    pub publisher: String,
    pub publish_date: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookMasterJson {
    pub book_id: String,
    pub book_name: String,
    pub author: String,
    pub publisher: String,
    pub publish_year: String,
    pub publish_month: String,
    pub publish_date: String
}

impl From<BookMasterJson> for BookMaster {
    fn from(&form: BookMasterJson) -> Option<BookMaster> {
        let date: DateTime<Utc> = Utc::now();
        let data : BookMaster = BookMaster{
            book_id: form.book_id,
            book_name: form.book_name,
            author: form.author,
            publisher: form.publisher,
            publish_date: date
        };
        Some(data)
    }
}

impl From<BookMaster> for BookMasterJson {
    fn from(book: BookMaster) -> Self {
        BookMasterJson{
            book_id: book.book_id,
            book_name: book.book_name,
            author: book.author,
            publisher: book.publisher,
            publish_year: book.publish_date.year().to_string(),
            publish_month: book.publish_date.month().to_string(),
            publish_date: book.publish_date.date().to_string()
        }
    }
}