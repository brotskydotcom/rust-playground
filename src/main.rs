use warp::Filter;

#[tokio::main]
async fn main() {
    let route1 = warp::any()
        .and(warp::header::<String>("X-Frotz"))
        .map(|ct| format!("Accepted Request with X-Frotz: {}\r\n\r\n", &ct));
    warp::serve(route1).run(([127, 0, 0, 1], 3030)).await;
}
