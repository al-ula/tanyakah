use crate::auth::SimpleAuth;
use salvo::prelude::*;
use small_uid::SmallUid;
use tracing::{debug, info};

#[handler]
pub async fn verify_hx_request(req: &mut Request, res: &mut Response) {
    if req.headers().get("hx-request").map(|h| h.to_str().unwrap()) != Some("true") {
        info!("Not htmx request");
        res.status_code(StatusCode::BAD_REQUEST);
        return;
    }
}

#[handler]
pub async fn verify_auth(req: &mut Request) {
    let auth_token = req
        .cookies()
        .get("auth_token_token")
        .map(|c| c.value().to_string());
    if auth_token.is_none() {
        debug!("Auth token not found");
        return;
    }
    let auth_token = auth_token.unwrap();
    let claims = match SimpleAuth::from_token(&auth_token) {
        Ok(c) => c,
        Err(e) => {
            info!("Failed to parse auth token: {}", e);
            return;
        }
    };
    let claims_user = u64::from(match SmallUid::try_from(claims.user.clone()) {
        Ok(u) => u,
        Err(e) => {
            info!("Failed to parse user id: {}", e);
            return;
        }
    });

    match crate::db::check_user(claims_user) {
        Ok(_) => {}
        Err(e) => {
            info!("User not found: {}", e);
            return;
        }
    };
    let creds = claims;
    req.extensions_mut().insert(creds);
}
