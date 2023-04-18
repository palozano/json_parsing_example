# Parsing API info in Rust

Using simple HTTP requests (`reqwest` crate), you can parse the JSON response data as a custom type.

If the type is as follows:

```rust
struct Todo {
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}
```

The parsing could be:

```rust
reqwest::Client::new()
    .get("<URL>")
    .send() // send the request
    .await? // await the response
    .json::<Vec<Todo>>() // deserealize the JSON response as a type Vec<Todo>
    .await?;
```
