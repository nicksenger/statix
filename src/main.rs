use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "statix")]
struct Opt {
    #[structopt(short = "p", long = "port", default_value = "8000")]
    port: String,
    #[structopt(short = "d", long = "dir", default_value = "./static")]
    dir: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let opt = Opt::from_args();
    let url = format!("127.0.0.1:{}", opt.port);

    println!("Statix running at: http://{}", url);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(Files::new("/", Opt::from_args().dir).index_file("index.html"))
    })
    .bind(&url)
    .expect("Failed to start statix")
    .run()
    .await
}
