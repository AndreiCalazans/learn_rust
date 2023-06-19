use actix_files as fs;
use actix_multipart::Multipart;
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

use std::path::PathBuf;

async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let filename = field.content_disposition().get_filename();

        let mut path = PathBuf::from("./uploads");

        match filename {
            Some(_filename) => {
                path.push(_filename);
                let mut f = web::block(|| std::fs::File::create(path)).await.unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || {
                        f.and_then(|mut file| file.write_all(&data).map(|_| file))
                    })
                    .await
                    .map_err(|e| {
                        // Handle the error case, such as logging or returning an error response
                        eprintln!("Error writing to file: {}", e);
                        actix_web::error::ErrorInternalServerError("Failed to write to file")
                    })?;
                }
            }
            None => {
                // Handle the case when the Option is None
                println!("No filename provided");
            }
        }
    }
    Ok(HttpResponse::Ok().into())
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("File Server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("./uploads")?;

    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/files", "./uploads").show_files_listing())
            .route("/", web::get().to(index))
            .route("/upload", web::post().to(upload))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_http::header;
    use actix_http::header::HeaderValue;
    use actix_web::http::StatusCode;
    use actix_web::{test, App};
    // use actix_multipart::Field;
    // use bytes::Bytes;

    #[actix_rt::test]
    async fn test_index() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_upload() {
        let mut app = test::init_service(App::new().route("/upload", web::post().to(upload))).await;

        let payload = r#"-----------------------------325491532399963166993862150
Content-Disposition: form-data; name="textfield"; filename="test.txt"
Content-Type: text/plain

Hello World!
-----------------------------325491532399963166993862150--
"#;

        let mut req = test::TestRequest::post()
            .uri("/upload")
            .set_payload(payload)
            .to_request();

        req.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static(
            "multipart/form-data; boundary=---------------------------325491532399963166993862150",
            ),
        );

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    // #[actix_rt::test]
    // async fn test_file_download() {
    //     let field = Field::default()
    //         .part("testfile")
    //         .file_name("testfile.txt")
    //         .stream(Bytes::from_static(b"test file content"))
    //         .unwrap();

    //     let content = upload(field).await.unwrap();

    //     assert_eq!(content.status(), StatusCode::OK);

    //     let mut app = test::init_service(
    //         App::new().service(fs::Files::new("/files", "./uploads").show_files_listing()),
    //     )
    //     .await;

    //     let req = test::TestRequest::get().uri("/files/testfile").to_request();
    //     let resp = test::call_service(&mut app, req).await;

    //     assert_eq!(resp.status(), StatusCode::OK);
    // }
}
