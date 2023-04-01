use reqwest;

pub async fn get() {
    match get_tasks().await {
                Ok(res) => print_tasks(res).await,
                Err(err) => println!("{}", err)
            }
}


async fn get_tasks() -> Result<reqwest::Response, reqwest::Error> {
    let token = &crate::token();
    let client = reqwest::Client::new();
    client
        .get("https://api.todoist.com/rest/v2/tasks")
        .bearer_auth(token)
        .send()
        .await
}

async fn print_tasks(res: reqwest::Response) {
    let tasks = match res.text().await {
            Ok(result) => result,
            Err(err) => {
                println!("{}", err);
                std::process::exit(1)
            }
    };
    println!("{}", tasks);
}
