#![cfg(feature = "web-auth")]
//! Synthetic test-only RSA keys. Never use these fixtures in a deployment.
use actix_web::{cookie::Cookie, http::StatusCode, test, web, App, HttpResponse};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde_json::{json, Value};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use uno_api::auth::{
    session::{Permission, SessionVerifier},
    web::ProtectAdmin,
};

fn verifier() -> SessionVerifier {
    SessionVerifier::new(
        [(
            "test".to_string(),
            include_str!("fixtures/session-test-public.pem").to_string(),
        )]
        .into(),
        "test-issuer",
        "uno",
        "https://uno.test",
    )
    .unwrap()
}

fn claims() -> Value {
    let now = jsonwebtoken::get_current_timestamp();
    json!({"sub":"operator-1", "role":"operator", "iss":"test-issuer", "aud":"uno", "iat":now, "exp":now+300, "auth_time":now, "amr":["mfa"]})
}

fn token(claims: &Value) -> String {
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some("test".into());
    encode(
        &header,
        claims,
        &EncodingKey::from_rsa_pem(include_bytes!("fixtures/session-test-private.pem")).unwrap(),
    )
    .unwrap()
}

#[actix_web::test]
async fn rejects_invalid_session_claims_and_signature() {
    let verifier = verifier();
    assert_eq!(
        verifier.verify(&token(&claims())).unwrap().sub,
        "operator-1"
    );
    for field in ["exp", "iss", "aud", "sub", "iat"] {
        let mut value = claims();
        value.as_object_mut().unwrap().remove(field);
        assert!(verifier.verify(&token(&value)).is_err(), "missing {field}");
    }
    for (field, value) in [
        ("aud", json!("other")),
        ("iss", json!("other")),
        ("sub", json!("")),
        ("exp", json!(1)),
        ("iat", json!(u64::MAX)),
        ("nbf", json!(u64::MAX)),
    ] {
        let mut invalid = claims();
        invalid[field] = value;
        assert!(
            verifier.verify(&token(&invalid)).is_err(),
            "invalid {field}"
        );
    }
    let valid = token(&claims());
    let mut pieces: Vec<_> = valid.split('.').map(str::to_string).collect();
    pieces[2] = "invalid-signature".into();
    assert!(verifier.verify(&pieces.join(".")).is_err());
    let mut header = Header::new(Algorithm::HS256);
    header.kid = Some("test".into());
    let wrong_algorithm = encode(
        &header,
        &claims(),
        &EncodingKey::from_secret(b"not-a-trusted-key"),
    )
    .unwrap();
    assert!(verifier.verify(&wrong_algorithm).is_err());
}

#[actix_web::test]
async fn role_mfa_and_recent_authentication_are_required() {
    let verifier = verifier();
    let mut author = claims();
    author["role"] = json!("content_author");
    let author = verifier.verify(&token(&author)).unwrap();
    assert!(author.require(Permission::ContentWrite).is_ok());
    assert!(author.require(Permission::ContentPublish).is_err());
    assert!(author.require(Permission::Operator).is_err());
    let mut no_mfa = claims();
    no_mfa["amr"] = json!(["pwd"]);
    assert!(verifier
        .verify(&token(&no_mfa))
        .unwrap()
        .require(Permission::Operator)
        .is_err());
    let mut old_auth = claims();
    old_auth["auth_time"] = json!(1);
    assert!(verifier
        .verify(&token(&old_auth))
        .unwrap()
        .require(Permission::ContentOverride)
        .is_err());
}

#[actix_web::test]
async fn unauthorized_requests_never_reach_mutations() {
    let writes = Arc::new(AtomicUsize::new(0));
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(verifier()))
            .app_data(web::Data::new(writes.clone()))
            .wrap(ProtectAdmin)
            .route(
                "/api/mutate",
                web::post().to(|writes: web::Data<Arc<AtomicUsize>>| async move {
                    writes.fetch_add(1, Ordering::SeqCst);
                    HttpResponse::Ok().finish()
                }),
            ),
    )
    .await;
    for credential in [
        None,
        Some("dev-admin-key".to_string()),
        Some("Bearer forged.payload.signature".to_string()),
    ] {
        let mut request = test::TestRequest::post().uri("/api/mutate");
        if let Some(credential) = credential {
            request = request.insert_header(("Authorization", credential));
        }
        assert_eq!(
            test::call_service(&app, request.to_request())
                .await
                .status(),
            StatusCode::UNAUTHORIZED
        );
    }
    let mut author = claims();
    author["role"] = json!("content_author");
    let request = test::TestRequest::post()
        .uri("/api/mutate")
        .insert_header(("Authorization", format!("Bearer {}", token(&author))))
        .to_request();
    assert_eq!(
        test::call_service(&app, request).await.status(),
        StatusCode::FORBIDDEN
    );
    assert_eq!(writes.load(Ordering::SeqCst), 0);
    let request = test::TestRequest::post()
        .uri("/api/mutate")
        .insert_header(("Authorization", format!("Bearer {}", token(&claims()))))
        .to_request();
    assert_eq!(
        test::call_service(&app, request).await.status(),
        StatusCode::OK
    );
    assert_eq!(writes.load(Ordering::SeqCst), 1);
}

#[actix_web::test]
async fn cookie_writes_require_the_configured_origin() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(verifier()))
            .wrap(ProtectAdmin)
            .route(
                "/api/mutate",
                web::post().to(|| async { HttpResponse::Ok().finish() }),
            ),
    )
    .await;
    for (origin, expected) in [
        (None, StatusCode::FORBIDDEN),
        (Some("https://attacker.test"), StatusCode::FORBIDDEN),
        (Some("https://uno.test"), StatusCode::OK),
    ] {
        let mut request = test::TestRequest::post()
            .uri("/api/mutate")
            .cookie(Cookie::new("uno_session", token(&claims())));
        if let Some(origin) = origin {
            request = request.insert_header(("Origin", origin));
        }
        assert_eq!(
            test::call_service(&app, request.to_request())
                .await
                .status(),
            expected
        );
    }
}
