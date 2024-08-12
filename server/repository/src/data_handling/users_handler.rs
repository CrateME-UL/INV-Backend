use crate::get_db_pool;
use domain::{LoginRequest, User};

pub async fn get_user_db(payload: &LoginRequest) -> Result<User, Box<dyn std::error::Error>> {
    let default = "";
    println!(
        "{}:email",
        format!(
            "{:?}:email",
            payload.user_email.as_deref().unwrap_or(default)
        )
    );
    let user_result = sqlx::query!(
        "SELECT user_id, user_firstname, user_lastname, user_email, user_password FROM Users WHERE user_email = $1",
        payload.user_email.as_deref().unwrap_or(default),
    )
    .fetch_one(get_db_pool())
    .await;
    println!("{}", format!("{:?}", user_result));

    let user = match user_result {
        Ok(record) => User {
            user_id: record.user_id,
            user_firstname: record.user_firstname,
            user_lastname: record.user_lastname,
            user_email: record.user_email,
            user_password: record.user_password,
        },
        Err(err) => return Err(Box::new(err)),
    };

    Ok(user)
}
