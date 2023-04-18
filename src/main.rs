/// Import the implementation for the data
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    #[serde(rename = "userId")] // rename the field to match the json
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let get_todos = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send() // send the request
        .await? // await the response
        .json::<Vec<Todo>>() // deserealize the JSON response as a type Vec<Todo>
        .await?;
    println!("{:#?}", get_todos);

    // You can do lots of things with the data
    let text = get_todos
        .clone()
        .into_iter()
        .map(|t| t.title)
        .collect::<String>();
    println!("Text -> {}", text);

    let sum = get_todos
        .clone()
        .into_iter()
        .fold(0, |acc, t| acc + t.id.unwrap_or(0));
    println!("Sum -> {}", sum);

    // It's possible to send data as JSON
    let new_todo = Todo {
        user_id: 1,
        id: None,
        title: "New Todo".to_string(),
        completed: false,
    };
    let post_todo: Todo = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos") // create a POST request
        .json(&new_todo) // serialize the (Todo) data as JSON
        .send() // send the request
        .await? // await the response
        .json() // deserialize the JSON response as a type Todo (in the type annotation)
        .await?;
    println!("Posted TODO: {:#?}", post_todo);

    // Even from literals can JSONs be created
    let post_todo: Todo = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&serde_json::json!({
            "userId": 1,
            "title": "New Todo",
            "completed": false
        }))
        .send() // send the request
        .await? // await the response
        .json() // deserialize the JSON response as a type Todo
        .await?;
    println!("Hardcoded posted TODO: {:#?}", post_todo);

    Ok(())
}
