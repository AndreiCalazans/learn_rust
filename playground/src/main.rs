use chrono::Local;
use reqwest;
use tokio::time::{sleep, timeout, Duration};

mod logger;
mod utils;

async fn process_field(field: i32) {
    println!("Processing field: {:?}", field);
}

async fn run_example_if_let() {
    println!("Running example of if let with async/await.");
    let payload: Vec<Result<Option<i32>, &str>> = vec![
        Ok(Some(1)),
        Ok(Some(2)),
        Ok(None),
        Err("Error"),
        Ok(Some(4)),
    ];

    for item in &payload {
        if let Ok(Some(field)) = item {
            process_field(*field).await;
        } else if let Ok(None) = item {
            println!("Caught a None value.");
        } else {
            println!("Caught error.");
        }
    }

    /*
     * Ok, this is interesting, by default, Rust won't compile a Vec that gets used in two spots? 
     * Specially if you pass it to a function or use it in a for loop like we do with payload
     * above.
     *
     * In this example I could: clone payload or use a reference to it.
     * */

    let result = utils::removeErrorsFromVector(payload);
    println!("remove errors from vec result: {:?}", result); 
    println!("DONE!");
}

async fn fake_http_request() {
    let client = reqwest::Client::new();
    let timeout_duration = Duration::from_secs(5);

    let response = client
        .get("https://catfact.ninja/fact")
        .timeout(timeout_duration)
        .send()
        .await;

    match response {
        Ok(response) => {
            let data = response.json::<utils::types::CatFact>().await;
            match data {
                Ok(data) => {
                    println!("Full Cat Fact: {:?}", data);
                    println!("Fact: {:?}", data.fact);
                }
                Err(error) => {
                    println!("Failed to deserialize JSON: {:?}", error);
                }
            }
        }
        Err(error) => {
            println!("Cat Fact Error: {:?}", error);
        }
    }
}

async fn run_example_with_fake_async() {
    println!("Fake async start: {}", Local::now());
    /*
     * Attention that the timeout function is here is not like in JavaScript's setTimeout. It
     * actually refers to the limit of time that this async block can run.
     *
     * For a setTimeout like behavior you want to use sleep.
     *
     * */
    let timeout_duration = Duration::from_secs(2);
    if let Ok(Some(value)) = timeout(timeout_duration, async {
        sleep(Duration::from_secs(1)).await;
        Some(2)
    })
    .await
    {
        println!("Value: {:?}", value);
        println!("Done at {}", Local::now());
    } else {
        println!("Timed out!");
    }
}

#[tokio::main]
async fn main() {
    logger::log_message("Stating examples");
    /*
     * Example of using if let with async/await.
     * */
    run_example_if_let().await;

    /*
     * Example with fake async function
     * */
    run_example_with_fake_async().await;

    /*
     * HTTP request example
     * */
    fake_http_request().await;

    logger::warn_message("Warning are exiting...");
    logger::error_message("Exit");
}
