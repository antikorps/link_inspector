mod app_router;
mod handler_html;
mod handler_office;
mod http_client;
mod link_checker;

#[tokio::main]
async fn main() {
    let app_router = app_router::router::make_router().await;

    for port in 3000..8000 {
        let address = format!("0.0.0.0:{port}");
        match tokio::net::TcpListener::bind(address).await {
            Err(_) => continue,
            Ok(ok) => {
                println!("Aplicación web iniciada en http://localhost:{port}\nWeb application started on http://localhost:{port}");
                axum::serve(ok, app_router.clone()).await.unwrap();
            }
        }
    }
}
