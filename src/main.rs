use hyper::{Client, Request, Uri, body::HttpBody};
use hyper::client::HttpConnector;
use hyper_proxy::{Proxy, ProxyConnector, Intercept};
use headers::Authorization;
use tokio::io::{stdout, AsyncWriteExt as _};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    for url in ["http://jenna.brotsky.net", "https://brotsky.com"] {
        println!("Url: {}", url);
        let uri: Uri = url.parse().unwrap();
        let mut req = Request::get(uri.clone()).body(hyper::Body::empty()).unwrap();

        // make the proxy client
        let proxy = {
            let proxy_uri = "http://192.150.23.9:8080".parse().unwrap();
            // let proxy_uri = "http://127.0.0.1:8888".parse().unwrap();
            let mut proxy = Proxy::new(Intercept::All, proxy_uri);
            if std::env::args().len() > 1 {
                proxy.set_authorization(Authorization::basic("John Doe", "Agent1234"));
            }
            let connector = HttpConnector::new();
            ProxyConnector::from_proxy(connector, proxy).unwrap()
        };
        // add any needed proxy headers the request (typically auth)
        if let Some(headers) = proxy.http_headers(&uri) {
            req.headers_mut().extend(headers.clone().into_iter());
        }
        // build the client against the proxy connector
        let client = Client::builder().build(proxy);
        let mut resp = client.request(req).await?;
        println!("Response: {}", resp.status());
        while let Some(chunk) = resp.body_mut().data().await {
            stdout().write_all(&chunk?).await?;
        }
    }
    Ok(())
}
