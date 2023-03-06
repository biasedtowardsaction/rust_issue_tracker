mod model;
use crate::model::{Issue, IssuesState};
use actix_cors::Cors;
use actix_web::http::header::ContentType;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use chrono::prelude::Utc;
use serde::Serialize;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[get("/health")]
async fn health_check_handler() -> impl Responder {
    const MESSAGE: &str = "a simple issue tracker";
    let response_json = &GenericResponse {
        status: "healthy".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[get("/issues")]
async fn get_issues_handler(data: web::Data<IssuesState>) -> impl Responder {
    let issues = data.issues.lock().unwrap();

    let response = serde_json::to_string(&(*issues)).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}
#[get("/issues/{issue_id}")]
async fn get_issue_by_id_handler(data: web::Data<IssuesState>, issue_id: String) -> impl Responder {
    let issue = data.issues.lock().unwrap();
    if let Some(issue) = issue.iter().find(|&item| item.issue_id == issue_id) {
        // Issue is found
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(issue)
    } else{
        //The issue does not exist
        HttpResponse::NotFound().body("Issue not found!")
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("ISSUE_LOGGER").is_none() {
        std::env::set_var("ISSUE_LOGGER", "issue_api=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    let app_data = web::Data::new(IssuesState {
        issues: Mutex::new(vec![Issue {
            issue_id: Uuid::new_v4().to_string(),
            committer_id: Uuid::new_v4().to_string(),
            resolver_id: Uuid::new_v4().to_string(),
            status: Some(true),
            body: String::from("first issue"),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }]),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000")
            .allowed_origin("http://localhost:8000/")
            .supports_credentials();
        App::new()
            .app_data(app_data.clone())
            .service(health_check_handler)
            .service(get_issues_handler)
            .service(get_issue_by_id_handler)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
