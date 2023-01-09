use std::fmt::Write;

use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;

use crate::session_state::TypedSession;
use crate::utils::{e500, see_other};

pub async fn change_password_form(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        return Ok(see_other("/login"));
    };

    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <html lang="en">
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Change Password</title>
            </head>
            <body>
                {msg_html}
                <form action="/admin/password" method="post">
                    <label for="current_password">Current password</label>
                    <input
                        name="current_password"
                        type="password"
                        placeholder="Enter current password"
                    >
                    <br>
                    <label for="new_password">New password</label>
                    <input
                        name="new_password"
                        type="password"
                        placeholder="Enter new password"
                    >
                    <br>
                    <label for="new_password_check">Confirm new password</label>
                    <input
                        name="new_password_check"
                        type="password"
                        placeholder="Type the new password again"
                    >
                    <br>
                    <button type="submit">Change password</button>
                </form>
                <p><a href="/admin/dashboard">&lt;- Back</a></p>
            </body>
            </html>
            "#,
        )))
}
