use axum::{
    Router, body::Body, extract::Request, http::StatusCode, response::Response, routing::any,
};
use reqwest::Client;

// async fn proxy(req: Request) -> Response {
//     let backend = "https://httpbin.org";

//     // 拆分 request 的 parts 和 body
//     let (parts, body) = req.into_parts();

//     let uri = format!("{}{}", backend, parts.uri);

//     let client = Client::new();

//     let body = axum::body::to_bytes(body, usize::MAX).await.unwrap();
//     // 转换 body
//     // let body = match axum::body::to_bytes(body, usize::MAX).await {
//     //     Ok(b) => b,
//     //     Err(_) => return Response::builder()
//     //         .status(StatusCode::BAD_REQUEST)
//     //         .body(Body::empty())
//     //         .unwrap(),
//     // };

//     // 构建 reqwest 请求
//     let mut builder = client.request(parts.method, uri);

//     // 转发 headers
//     for (name, value) in parts.headers.iter() {
//         builder = builder.header(name, value);
//     }

//     // 发送请求
//     let resp = match builder.body(body).send().await {
//         Ok(r) => r,
//         Err(_) => return Response::builder()
//             .status(StatusCode::BAD_GATEWAY)
//             .body(Body::empty())
//             .unwrap(),
//     };

//     // 保存 status 和 headers
//     let status = resp.status();
//     let headers = resp.headers().clone();

//     // 获取响应 body
//     let bytes = match resp.bytes().await {
//         Ok(b) => b,
//         Err(_) => return Response::builder()
//             .status(StatusCode::BAD_GATEWAY)
//             .body(Body::empty())
//             .unwrap(),
//     };

//     // 构建最终 Response
//     let mut response = Response::builder().status(status);
//     for (name, value) in headers.iter() {
//         response = response.header(name, value);
//     }

//     response.body(Body::from(bytes)).unwrap()
// }

async fn proxy(req: Request) -> Result<Response, StatusCode> {
    let backend = "https://httpbin.org";

    let uri = format!("{}{}", backend, req.uri());

    let client = Client::new();

    // 转发
    let builder = client.request(req.method().clone(), uri);

    let body = axum::body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let resp = builder
        .body(body)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let response_builder = Response::builder().status(resp.status());

    let bytes = resp.bytes().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok(response_builder.body(Body::from(bytes)).unwrap())
}

use tower::limit::ConcurrencyLimitLayer;

#[tokio::main]
async fn main() {
    let rate_limit = ConcurrencyLimitLayer::new(5); //
    // let size_limit = RequestBodyLimitLayer::new(1024 * 1024); // 1 MB limit
    let app = Router::new().route("/{*path}", any(proxy).layer(rate_limit));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("proxy listening: http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
