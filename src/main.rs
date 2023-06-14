use std::{env, time::Duration};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tokio::process::Command;

#[post("/poweroff")]
async fn poweroff() -> impl Responder {
    let system = actix_web::rt::System::current();
    let arbiter = system.arbiter();

    arbiter.spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;

        system_shutdown::shutdown().unwrap();
    });

    HttpResponse::Ok().finish()
}

#[get("/current")]
async fn current() -> impl Responder {
    if cfg!(windows) {
        "Windows"
    } else {
        "Linux"
    }
}

#[post("/boot_os/{os}")]
async fn boot_os(path: web::Path<(String,)>) -> impl Responder {
    let (os,) = path.into_inner();

    if cfg!(windows) {
        match os.as_str() {
            "linux" => {
                let system = actix_web::rt::System::current();
                let arbiter = system.arbiter();

                arbiter.spawn(async {
                    tokio::time::sleep(Duration::from_secs(1)).await;

                    system_shutdown::reboot().unwrap();
                });
            }
            _ => (),
        };
    } else {
        match os.as_str() {
            "windows" => {
                let system = actix_web::rt::System::current();
                let arbiter = system.arbiter();

                arbiter.spawn(async {
                    tokio::time::sleep(Duration::from_secs(1)).await;

                    let windows_bootnum =
                        env::var("WINDOWS_BOOTNUM").unwrap_or_else(|_| String::from("0006"));

                    Command::new("efibootmgr")
                        .arg("--bootnext")
                        .arg(windows_bootnum)
                        .output()
                        .await
                        .ok();

                    Command::new("reboot").output().await.ok();
                });
            }
            _ => (),
        }
    }

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| String::from("2115"));

    let port_number: u16 = port.parse().expect("Invalid port number");

    HttpServer::new(|| {
        App::new()
            .service(poweroff)
            .service(current)
            .service(boot_os)
    })
    .bind(("0.0.0.0", port_number))?
    .run()
    .await
}
