use chrono::NaiveDateTime;

pub fn current_timestamp() -> NaiveDateTime {
    chrono::Local::now().naive_local()
}
