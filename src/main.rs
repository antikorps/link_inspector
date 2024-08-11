use std::env;

mod app_router;
mod handler_html;
mod handler_office;
mod handlers;
mod http_client;
mod link_checker;

#[tokio::main]
async fn main() {
    let app_router = app_router::router::make_router().await;

    for mut port in 3000..8000 {
        // Make a helper for start the app from the env_var LINK_INSPECTOR_PORT
        let mut use_env_var_port = false;
        match env::var("LINK_INSPECTOR_PORT") {
            Err(_) => (),
            Ok(link_inspector_env) => match link_inspector_env.parse::<i32>() {
                Err(error) => {
                    eprintln!("LINK_INSPECTOR_PORT format is incorrect, not a number: {error}");
                }
                Ok(ok) => {
                    port = ok;
                    use_env_var_port = true;
                }
            },
        }
        let address = format!("0.0.0.0:{port}");
        match tokio::net::TcpListener::bind(address).await {
            Err(error) => {
                if use_env_var_port {
                    panic!("could not start the app in the port for LINK_INSPECTOR_PORT {error}");
                }
            }

            Ok(ok) => {
                println!("Aplicación web iniciada en http://localhost:{port}\nWeb application started on http://localhost:{port}");
                axum::serve(ok, app_router.clone()).await.unwrap();
            }
        }
    }
}
