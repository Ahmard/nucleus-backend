use actix_web::dev::Extensions;
use std::cell::Ref;
use uuid::Uuid;
use crate::models::user::User;

pub fn get_auth_id(ext: Ref<Extensions>) -> Uuid {
    return *ext.get::<Uuid>().unwrap();
}

pub fn get_auth_user(ext: Ref<Extensions>) -> User {
    return ext.get::<User>().unwrap().clone()
}
