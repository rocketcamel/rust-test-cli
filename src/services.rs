use actix_web::{get, HttpRequest, HttpResponse};
use serde_json::json;

trait UserAgent {
    fn get_agent(&self) -> Option<&str>;
}

impl UserAgent for HttpRequest {
    fn get_agent(&self) -> Option<&str> {
        self.headers()
            .get("User-Agent")
            .and_then(|value| value.to_str().ok())
    }
}

#[get("/")]
pub async fn hello(req: HttpRequest) -> HttpResponse {
    let agent = req.get_agent().unwrap_or("unknown");
    HttpResponse::Ok().json(json!({
      "message:": "Hello World!",
      "user-agent": agent
    }))
}
