use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    //let hello = warp::path!("hello" / String)
    //    .map(|name| format!("Hello, {}!", name));

    println!("Launched Server...");

    let test = warp::path("index")
        .and(warp::fs::file("res/index.html"));

    warp::serve(warp::fs::dir("./generated").or(test))
        .run(([192, 168, 0, 163], 3030))
        .await;
}
