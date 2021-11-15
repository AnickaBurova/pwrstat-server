use actix_web::{get, App, HttpServer, Responder};
use std::process::Command;
use structopt::StructOpt;

mod status;
use status::*;


#[get("/ups_status")]
async fn ups_status() -> impl Responder {
    let text = match Command::new("pwrstat").arg("-status").output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(text) => text,
            Err(err) => return format!("Failed to convert pwrstat output to a string: {}", err),
        },
        Err(err) => return format!("Failed to execute pwrstat: {}", err),
    };
    let lines = text.lines().collect::<Vec<_>>();
    match Ups::from_pwrstat(&lines[..])
        .and_then(|ups| serde_json::to_string(&ups).map_err(anyhow::Error::from))
    {
        Ok(result) => result,
        Err(err) => format!("Failed to parse ups data: {}", err),
    }
}

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short, long, default_value = "9999")]
    port: u16,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::from_args();
    HttpServer::new(|| App::new().service(ups_status))
        .bind(format!("0.0.0.0:{}", args.port))?
        .run()
        .await?;

    Ok(())
}
