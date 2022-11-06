use warp::Filter;

type QueryParams = std::collections::HashMap<String, String>;

#[tokio::main]
async fn main() {
    let qf = warp::get().and(warp::query::<QueryParams>()).map(|q| {
        eprintln!("Parsed: {:?}", q);
        format!("Parsed: {:?}", q)
    });
    let rqf = warp::post().and(warp::filters::query::raw()).map(|q| {
        eprintln!("Raw: {}", q);
        format!("Raw: {}", q)
    });
    let handle = tokio::spawn(warp::serve(qf.or(rqf)).run(([127, 0, 0, 1], 3030)));
    let client = reqwest::Client::builder().build().unwrap();
    let request = client
        .get("http://localhost:3030/query?this=is+a+test")
        .build()
        .unwrap();
    let response = client.execute(request).await.unwrap();
    eprintln!("Response: {:?}", response);
    handle.abort();
}
