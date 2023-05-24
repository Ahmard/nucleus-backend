#![macro_use]

#[macro_export]
macro_rules! view {
    ($file:expr, $($json:tt)+) => {
        $crate::helpers::responder::Responder::HtmlTemplate(
            $file.to_string(),
            ::tera::Context::from_value(::serde_json::json!($($json)+)).unwrap(),
            ::actix_web::http::StatusCode::OK
        )
    };
    ($file:expr) => {
        $crate::helpers::responder::Responder::HtmlTemplate(
            $file.to_string(),
            ::tera::Context::new(),
            ::actix_web::http::StatusCode::OK
        )
    };
}

macro_rules! redirect {
    ($uri:expr) => {
        $crate::helpers::responder::Responder::Redirect(
            $uri.to_string(),
            ::actix_web::http::StatusCode::FOUND,
        )
    };
}

pub(crate) use redirect;
pub(crate) use view;
