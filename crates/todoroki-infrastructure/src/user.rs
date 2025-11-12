use crate::shared::postgresql::Postgresql;

use sqlx::{
    prelude::{FromRow, Type},
    types::chrono,
};
use todoroki_domain::{
    entities::user::{User, UserEmail, UserId, UserName, UserRole},
    repositories::user::{UserRepository, UserRepositoryError},
    value_objects::datetime::DateTime,
};
use uuid::Uuid;

#[derive(FromRow)]
struct UserRow {
    id: Uuid,
    role: UserRoleColumn,
    name: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Type)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRoleColumn {
    Owner,
    Contributor,
}

impl From<UserRole> for UserRoleColumn {
    fn from(value: UserRole) -> Self {
        match value {
            UserRole::Owner => Self::Owner,
            UserRole::Contributor => Self::Contributor,
        }
    }
}

impl From<UserRoleColumn> for UserRole {
    fn from(value: UserRoleColumn) -> Self {
        match value {
            UserRoleColumn::Owner => Self::Owner,
            UserRoleColumn::Contributor => Self::Contributor,
        }
    }
}

struct UserIdColumn {
    id: Uuid,
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        Self::new(
            UserId::new(value.id),
            UserRole::from(value.role),
            UserName::new(value.name),
            UserEmail::new(value.email),
            DateTime::new(value.created_at),
            DateTime::new(value.updated_at),
        )
    }
}

pub struct PgUserRepository {
    db: Postgresql,
}

impl PgUserRepository {
    pub fn new(db: Postgresql) -> Self {
        Self { db }
    }
}

impl UserRepository for PgUserRepository {
    async fn create(&self, user: User) -> Result<UserId, UserRepositoryError> {
        let res = sqlx::query_as!(
            UserIdColumn,
            r#"
           INSERT INTO users (id, name, email)
           VALUES ($1, $2, $3, $4)
           RETURNING id
            "#,
            user.id().clone().value(),
            UserRoleColumn::from(user.role().clone()) as UserRoleColumn,
            user.name().clone().value(),
            user.email().clone().value(),
        )
        .fetch_one(&*self.db)
        .await;

        match res {
            Ok(id_column) => Ok(UserId::new(id_column.id)),
            Err(e) => match e.as_database_error() {
                Some(e) => Err(UserRepositoryError::InternalError(e.message().to_string())),
                _ => Err(UserRepositoryError::InternalError(e.to_string())),
            },
        }
    }

    async fn get_by_id(&self, id: UserId) -> Result<User, UserRepositoryError> {
        let res = sqlx::query_as!(
            UserRow,
            r#"SELECT
            users.id AS "id",
            users.role AS "role",
            users.name AS "name",
            users.email AS "email",
            users.created_at AS "created_at",
            users.updated_at AS "updated_at",
            users.deleted_at AS "deleted_at?"
            FROM users WHERE id = $1"#,
            id.value()
        )
        .fetch_one(&*self.db)
        .await;

        // TODO: check not found
        res.map(User::from)
            .map_err(|e: sqlx::Error| UserRepositoryError::InternalError(e.to_string()))
    }

    async fn get_by_email(&self, email: UserEmail) -> Result<User, UserRepositoryError> {
        let res = sqlx::query_as!(
            UserRow,
            r#"SELECT
            users.id AS "id",
            users.role AS "role",
            users.name AS "name",
            users.email AS "email",
            users.created_at AS "created_at",
            users.updated_at AS "updated_at",
            users.deleted_at AS "deleted_at?"
            FROM users WHERE email = $1"#,
            email.value()
        )
        .fetch_one(&*self.db)
        .await;

        // TODO: check not found
        res.map(User::from)
            .map_err(|e: sqlx::Error| UserRepositoryError::InternalError(e.to_string()))
    }
}
