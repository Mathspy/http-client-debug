use warp::Filter;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let all = warp::filters::method::method()
        .and(warp::filters::path::full())
        .and(warp::filters::header::headers_cloned())
        .and(warp::filters::body::bytes())
        .map(
            |method, path: warp::path::FullPath, headers, bytes: warp::hyper::body::Bytes| {
                println!("Method: {method}");
                println!("Path: {}", path.as_str());
                println!("Headers: {headers:?}");
                if bytes.is_empty() {
                    println!("No Body");
                } else {
                    println!("Body: {}", String::from_utf8_lossy(bytes.as_ref()));
                }
                println!("----------");

                format!("logged")
            },
        );

    warp::serve(all).run(([127, 0, 0, 1], 9277)).await;
}
