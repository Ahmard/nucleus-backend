use actix_web::dev::Extensions;
use std::cell::Ref;
use uuid::Uuid;

pub fn get_uuid(ext: Ref<Extensions>) -> Uuid {
    *ext.get::<Uuid>().unwrap()
}
