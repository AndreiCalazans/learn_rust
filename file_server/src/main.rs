use actix_files as fs;
use actix_multipart::Multipart;
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use std::path::PathBuf;

async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field
            .content_disposition()
            .expect("Expected Content-Disposition");
        let filename = content_disposition
            .get_filename()
            .expect("Expected filename");
        let mut path = PathBuf::from("./uploads");
        path.push(filename);

        let mut f = web::block(|| std::fs::File::create(path)).await.unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
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
    use actix_web::{test, App};
    use actix_web::http::StatusCode;
    use actix_multipart::Field;
    use bytes::Bytes;
    use futures::stream::once;

    #[actix_rt::test]
    async fn test_index() {
        let mut app = test::init_service(
            App::new().route("/", web::get().to(index))
        ).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_upload() {
        let mut app = test::init_service(
            App::new().route("/upload", web::post().to(upload))
        ).await;

        let payload = r#"-----------------------------325491532399963166993862150
Content-Disposition: form-data; name="textfield"; filename="test.txt"
Content-Type: text/plain

Hello World!
-----------------------------325491532399963166993862150--
"#;

        let req = test::TestRequest::post()
            .uri("/upload")
            .header(
                "content-type",
                "multipart/form-data; boundary=---------------------------325491532399963166993862150"
            )
            .set_payload(payload)
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_file_download() {
        let field = Field::new(
            "testfile",
            once(Ok::<_, std::io::Error>(Bytes::from_static(b"test file content"))),
            None,
            None,
        );
        let content = upload(field).await.unwrap();

        assert_eq!(content.status(), StatusCode::OK);

        let mut app = test::init_service(
            App::new().service(fs::Files::new("/files", "./uploads").show_files_listing())
        ).await;

        let req = test::TestRequest::get().uri("/files/testfile").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
