use std::env;

mod app_router;
mod cli;
mod handlers;
mod http_client;
mod link_checker;

fn say_hello(port: i32) {
    let reset = "\x1b[0m";
    let blue = "\x1b[34m";
    let green = "\x1b[32m";
    let yellow = "\x1b[33m";
    let bold = "\x1b[1m";

    let logo = r#"
     _     ___ _   _ _  __                             
    | |   |_ _| \ | | |/ /                             
    | |    | ||  \| | ' /                              
    | |___ | || |\  | . \                              
    |_____|___|_|_\_|_|\_\ _____ ____ _____ ___  ____  
    |_ _| \ | / ___||  _ \| ____/ ___|_   _/ _ \|  _ \ 
     | ||  \| \___ \| |_) |  _|| |     | || | | | |_) |
     | || |\  |___) |  __/| |__| |___  | || |_| |  _ < 
    |___|_| \_|____/|_|   |_____\____| |_| \___/|_| \_\
    
    
    "#;

    let message = format!(
        r#"{green}{logo}{green}

{yellow}Accede a la aplicación web desde:{reset} {bold}{blue}http://localhost:{port}{reset}
{yellow}Access the web application at:{reset} {bold}{blue}http://localhost:{port}{reset}

"#
    );
    println!("{}", message);
}

#[tokio::main]
async fn main() {
    cli::cli_handler::manage_cli().await;

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
                say_hello(port);
                axum::serve(ok, app_router.clone()).await.unwrap();
            }
        }
    }
}