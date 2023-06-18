
async fn process_field(field: i32) {
    println!("Processing field: {:?}", field);
}

async fn run_example_if_let() {
    println!("Running example of if let with async/await.");
    let payload: Vec<Result<Option<i32>, &str>> = vec![Ok(Some(1)), Ok(Some(2)), Ok(None), Err("Error"), Ok(Some(4))];

    for item in payload {
        if let Ok(Some(field)) = item {
            process_field(field).await;
        } else if let Ok(None) = item {
            println!("Caught a None value.");
        } else {
            println!("Caught error.");
        }
    }
    println!("DONE!");
}

#[tokio::main]
async fn main() {
    /*
     * Example of using if let with async/await.
     * */
    run_example_if_let().await;
}

