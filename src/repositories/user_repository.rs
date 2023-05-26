use std::ops::DerefMut;
use crate::helpers::db::current_timestamp;
use crate::helpers::error_messages::db_failed_to_execute;
use crate::helpers::{get_db_conn};
use crate::helpers::string::password_hash;
use crate::models::user::{RegisterForm, User, UserStatus};
use crate::models::DBPool;
use crate::schema::users;
use crate::schema::users::{email, user_id};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub fn create(&mut self, pool: &DBPool, data: RegisterForm) -> Result<User, String> {
        let existing = self.find_by_email(pool, data.email.clone());
        if existing.is_ok() {
            return Err(String::from("User with such email address already exists"));
        }

        let model = User {
            user_id: Uuid::new_v4(),
            first_name: data.first_name,
            last_name: data.last_name,
            email: data.email,
            status: user_stringy_status(UserStatus::ACTIVE).parse().unwrap(),
            password: password_hash(data.password),
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        let user = diesel::insert_into(users::dsl::users)
            .values(model.clone())
            .get_result::<User>(get_db_conn(pool).deref_mut())
            .expect(db_failed_to_execute());

        Ok(user)
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> QueryResult<User> {
        users::table
            .filter(user_id.eq(id))
            .first::<User>(get_db_conn(pool).deref_mut())
    }

    pub fn find_by_email(&mut self, pool: &DBPool, email_addr: String) -> QueryResult<User> {
        users::table
            .filter(email.eq(email_addr))
            .first::<User>(get_db_conn(pool).deref_mut())
    }
}

pub fn user_stringy_status(status: UserStatus) -> &'static str {
    match status {
        UserStatus::ACTIVE => "active",
        UserStatus::INACTIVE => "inactive",
        UserStatus::PENDING => "pending",
    }
}

pub fn user_status_is(str: String, status: UserStatus) -> bool {
    return str.eq(user_stringy_status(status));
}
