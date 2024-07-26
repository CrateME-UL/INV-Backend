use axum::extract::Extension;
use domain::User;
use sqlx::PgPool;

pub async fn get_user_db(
    email: String,
    Extension(pool): Extension<PgPool>,
) -> Result<User, Box<dyn std::error::Error>> {
    let record = sqlx::query!(
        "SELECT user_id, user_firstname, user_lastname, user_email, user_password FROM Users WHERE user_email = $1",
        email
    )
    .fetch_one(&pool)
    .await?;

    let user = User {
        user_id: record.user_id,
        user_firstname: record.user_firstname,
        user_lastname: record.user_lastname,
        user_email: record.user_email,
        user_password: record.user_password,
    };

    Ok(user)
}
