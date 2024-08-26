use crate::authentication::tokens::encode_token;
use crate::database::tokens::create_token;
use crate::database::users::update_user;
use crate::mail::send::{build_mail, send_mail, MailProps};
use diesel::internal::derives::multiconnection::chrono::{Duration, Utc};
use domain::models::tokens::{NewToken, TokenType};
use domain::models::users::{UpdatedUser, User};
use shared::app_state_model::AppState;
use shared::token_models::SpecificClaims;

pub fn request_password_reset(app_state: &AppState, user: User) {
    let smtp_transport = app_state.smtp_transport.clone().as_ref().clone();

    let config = {
        let config_guard = app_state.config.read();
        config_guard.clone()
    };

    // Increment the token version to invalidate all existing tokens
    let updated_user = UpdatedUser {
        username: None,
        email: None,
        has_validated_email: None,
        role: None,
        token_version: Some(user.clone().token_version + 1),
    };
    update_user(&app_state.database_pool, user.id, updated_user).unwrap();

    // Invalid possible previous tokens ?

    // Generate new token for password reset
    let now = Utc::now();
    let expiration = now + Duration::minutes(config.jwt_config.password_reset_expires_in.parse::<i64>().unwrap());
    let reset_claim = SpecificClaims{
        sub: user.id,
        type_: TokenType::PassReset,
        iat: now.timestamp() as usize,
        exp: expiration.timestamp() as usize,
    };
    let token = encode_token::<SpecificClaims>(&reset_claim, &config).unwrap();

    // Add token to database
    let token_id = create_token(&app_state.database_pool, NewToken{
        token,
        type_: TokenType::PassReset,
    }).unwrap();

    // Send email with token
    let url = format!("http://127.0.0.1:8080/api/auth/reset-token?id={}", token_id);
    let email = build_mail(MailProps{
        from: "Brandy <no-reply@sigma-bot.fr>".to_string(),
        to: format!("Brandy test <{}>", user.email),
        subject: "Password reset request".to_string(),
        body: format!("Here's the link you have to click in order to reset your password :\n{}", url),
    });
    println!("{:?}", send_mail(&smtp_transport, email).unwrap());
}