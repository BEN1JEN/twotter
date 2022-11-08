mod database;

use std::sync::Arc;

use actix_files::Files;
use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use color_eyre::eyre::Result;
use log::LevelFilter;
use simplelog::{ColorChoice, TermLogger, TerminalMode};

use crate::database::Database;

async fn get_index(req: HttpRequest) -> impl Responder {
	HttpResponse::PermanentRedirect()
		.insert_header((header::LOCATION, "/"))
		.await
}

#[tokio::main]
async fn main() -> Result<()> {
	color_eyre::install()?;
	TermLogger::init(
		LevelFilter::Trace,
		Default::default(),
		TerminalMode::Stdout,
		ColorChoice::Auto,
	)?;

	let database = Arc::new(Database::open("../data").await?);

	HttpServer::new(move || {
		App::new()
			.app_data(Data::new(database.clone()))
			.service(
				Files::new("/", "../client/assets")
					.prefer_utf8(true)
					.redirect_to_slash_directory()
					.index_file("index.html"),
			)
			.default_service(web::route().to(get_index))
	})
	.bind(("0.0.0.0", 11750))?
	.run()
	.await?;

	Ok(())
}
