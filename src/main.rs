use lambda_web::actix_web::{self, get, post, web, App, HttpResponse, HttpServer, Responder};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};
use webdocker::run;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World, This a feature extractor!")
}

#[post("/predict")]
async fn predict(image: web::Bytes) -> impl Responder {
    let img = image::load_from_memory(&image)
        .expect("Failed to load image from bytes");

    match run(&img) {
        Ok(output) => {
            let output_json = serde_json::to_string(&output).expect("Failed to serialize output as JSON");
            HttpResponse::Ok().body(output_json.into_bytes())
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
#[actix_web::main]
async fn main() -> Result<(),LambdaError> {
    println!("Running the service");
    let factory = move || {
        App::new()
            .service(hello)
            .service(predict)
    };

    if is_running_on_lambda() {
        // Run on AWS Lambda
        run_actix_on_lambda(factory).await?;
    } else {
        // Local server
        HttpServer::new(factory)
            .bind("0.0.0.0:8080")?
            .run()
            .await?;
    }
    Ok(())
}
