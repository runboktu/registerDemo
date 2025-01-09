use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::models::{CreateUser, LoginCredentials, User, AuthResponse};
use crate::utils::{hash_password, verify_password, create_token};

#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    user_data: web::Json<CreateUser>,
) -> impl Responder {
    // Hash the password
    let password_hash = match hash_password(&user_data.password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().json("Password hashing failed"),
    };

    // Create new user
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, username, email, password_hash, created_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, username, email, password_hash, created_at"
    )
    .bind(Uuid::new_v4())
    .bind(&user_data.username)
    .bind(&user_data.email)
    .bind(&password_hash)
    .bind(Utc::now())
    .fetch_one(pool.get_ref())
    .await;

    match user {
        Ok(user) => {
            let token = create_token(user.id).unwrap_or_default();
            HttpResponse::Ok().json(AuthResponse { token, user })
        }
        Err(_) => HttpResponse::InternalServerError().json("Could not create user"),
    }
}

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    credentials: web::Json<LoginCredentials>,
) -> impl Responder {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&credentials.email)
    .fetch_optional(pool.get_ref())
    .await;

    match user {
        Ok(Some(user)) => {
            match verify_password(&credentials.password, &user.password_hash) {
                Ok(true) => {
                    let token = create_token(user.id).unwrap_or_default();
                    HttpResponse::Ok().json(AuthResponse { token, user })
                }
                _ => HttpResponse::Unauthorized().json("Invalid credentials"),
            }
        }
        _ => HttpResponse::Unauthorized().json("Invalid credentials"),
    }
} 