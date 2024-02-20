use std::{io, sync::Arc};
use std::sync::{
    atomic::{AtomicBool, Ordering},
};

use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_lab::{extract::Path, respond::Html};

mod broadcast;
use self::broadcast::Broadcaster;

async fn do_broadcast_task(shutdown_marker: Arc<AtomicBool>, broadcaster: Arc<Broadcaster>) {
    loop {
        if shutdown_marker.load(Ordering::SeqCst) {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(200));
        let now = std::time::Instant::now();
        let msg = format!("{:?}", now);
        broadcaster.broadcast(&msg).await;
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let broadcaster = Broadcaster::create();
    let broadcaster_clone = broadcaster.clone();

    log::info!("starting HTTP server at http://localhost:8080");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(Arc::clone(&broadcaster)))
            .service(index)
            .service(event_stream)
            .service(broadcast_msg)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .disable_signals()
    .workers(2)
    .run();

    let server_handle = server.handle();
    let task_shutdown_marker = Arc::new(AtomicBool::new(false));

    let server_task = actix_web::rt::spawn(server);

    let broadcast_task = actix_web::rt::spawn(do_broadcast_task(Arc::clone(&task_shutdown_marker), broadcaster_clone));

    let shutdown = actix_web::rt::spawn(async move {
        // listen for ctrl-c
        actix_web::rt::signal::ctrl_c().await.unwrap();

        // start shutdown of tasks
        let server_stop = server_handle.stop(true);
        task_shutdown_marker.store(true, Ordering::SeqCst);

        // await shutdown of tasks
        server_stop.await;
    });

    let _ = tokio::try_join!(server_task, broadcast_task, shutdown).expect("Unable to join tasks");

    Ok(())
}

#[get("/")]
async fn index() -> impl Responder {
    Html(include_str!("index.html").to_owned())
}

#[get("/events")]
async fn event_stream(broadcaster: web::Data<Broadcaster>) -> impl Responder {
    broadcaster.new_client().await
}

#[post("/broadcast/{msg}")]
async fn broadcast_msg(
    broadcaster: web::Data<Broadcaster>,
    Path((msg,)): Path<(String,)>,
) -> impl Responder {
    broadcaster.broadcast(&msg).await;
    HttpResponse::Ok().body("msg sent")
}
