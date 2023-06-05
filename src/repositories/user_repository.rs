use crate::core::helpers::db::current_timestamp;
use crate::core::helpers::get_db_conn;
use crate::core::helpers::string::password_hash;
use crate::models::user::{RegisterForm, User, UserStatus};
use crate::models::DBPool;
use crate::schema::users;
use crate::schema::users::{email, user_id};
use diesel::result::Error;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, QueryResult, RunQueryDsl};
use std::ops::DerefMut;
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
            status: user_stringy_status(UserStatus::Active).parse().unwrap(),
            password: password_hash(data.password),
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        let user = diesel::insert_into(users::dsl::users)
            .values(model)
            .get_result::<User>(get_db_conn(pool).deref_mut())
            .unwrap();

        Ok(user)
    }

    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> Result<Option<User>, Error> {
        users::table
            .filter(user_id.eq(id))
            .first::<User>(get_db_conn(pool).deref_mut())
            .optional()
    }

    pub fn find_by_email(&mut self, pool: &DBPool, email_addr: String) -> QueryResult<User> {
        users::table
            .filter(email.eq(email_addr))
            .first::<User>(get_db_conn(pool).deref_mut())
    }
}

pub fn user_stringy_status(status: UserStatus) -> &'static str {
    match status {
        UserStatus::Active => "active",
        UserStatus::Inactive => "inactive",
        UserStatus::Pending => "pending",
    }
}

pub fn user_status_is(str: String, status: UserStatus) -> bool {
    return str.eq(user_stringy_status(status));
}
