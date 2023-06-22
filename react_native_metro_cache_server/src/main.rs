use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use flate2::{
    read::GzDecoder, // change this import
    write::GzEncoder,
    Compression,
};
use std::fs;
use std::io::{Cursor, Read, Write};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_data)
            .service(put_data)
    })
    .bind("127.0.0.1:8989")?
    .run()
    .await
}

#[get("/metro")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/metro/{key}")]
async fn get_data(info: web::Path<String>) -> impl Responder {
    let key = info.into_inner();
    println!("GET: {}", key);
    match fs::read(format!("./cache/{}", key)) {
        Ok(contents) => {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&contents).unwrap();
            HttpResponse::Ok().body(encoder.finish().unwrap())
        }
        Err(_) => HttpResponse::NotFound().body("not found"),
    }
}

#[post("/metro/{key}")]
async fn put_data(info: web::Path<String>, bytes: web::Bytes) -> impl Responder {
    let key = info.into_inner();
    println!("POST: {}", key);
    let mut d = GzDecoder::new(Cursor::new(&bytes[..])); // change this line
    let mut s = Vec::new();
    match d.read_to_end(&mut s) {
        Ok(_) => {
            fs::write(format!("./cache/{}", key), s).unwrap();
            HttpResponse::Ok().body(r#"{"status":"ok"}"#)
        }
        Err(_) => HttpResponse::InternalServerError().body("error"),
    }
}
