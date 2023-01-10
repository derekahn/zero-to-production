use actix_web::{http::header::ContentType, web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};

pub async fn admin_dashboard(
    session: TypedSession,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = if let Some(user_id) = session.get_user_id().map_err(e500)? {
        get_username(user_id, &pool).await.map_err(e500)?
    } else {
        return Ok(see_other("/login"));
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
              <title>Admin Dashboard</title>
              <meta charset="UTF-8">
              <meta http-equiv="content-type" content="text/html; charset=utf-8">
              <meta name="viewport" content="width=device-width, initial-scale=1">
            </head>
            <body>
              <p>Welcome {username}!</p>
              <p>Available actions:</p>
              <ol>
                <li>
                    <a href="/admin/password">Change password</a>
                </li>
                <li>
                    <a href="/admin/newsletters">Publish a newsletter</a>
                </li>
                <li>
                  <form name="logoutForm" action="/admin/logout" method="POST">
                    <input type="submit" value="Logout">
                  </form>
                </li>
              </ol>
            </body>
            </html>
            "#
        )))
}

#[tracing::instrument(name = "Get username", skip(pool))]
pub async fn get_username(user_id: Uuid, pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE user_id = $1
        "#,
        user_id,
    )
    .fetch_one(pool)
    .await
    .context("Failed to perform a query to retrieve a username")?;

    Ok(row.username)
}
