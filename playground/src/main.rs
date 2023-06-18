use tokio::time::{timeout, Duration, sleep};
use chrono::Local;

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
    }).await {
        println!("Value: {:?}", value);
        println!("Done at {}", Local::now());
    } else {
        println!("Timed out!");
    }
}

#[tokio::main]
async fn main() {
    /*
     * Example of using if let with async/await.
     * */
    run_example_if_let().await;

    /*
     * Example with fake async function
     * */
    run_example_with_fake_async().await;
}

