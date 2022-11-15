use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;
use surrealdb::{Datastore, Response, Session};
use tokio::fs;

pub struct Database {
	database: Datastore,
	files_path: PathBuf,
}

const INIT_QUERIES: &str = include_str!("setup.surreal");

impl Database {
	pub async fn open<P: AsRef<Path>>(path: P) -> Result<Database> {
		let files_path = path.as_ref().join("files");
		fs::create_dir_all(&files_path).await?;
		let database = Datastore::new(&format!("file://{}", path.as_ref().display())).await?;
		database
			.execute("DEFINE NS main", &Session::for_kv(), None, false)
			.await?;
		database
			.execute(
				"USE NS main; DEFINE DB main",
				&Session::for_kv(),
				None,
				false,
			)
			.await?;
		let res = database
			.execute(
				"USE NS main; USE DB main; SELECT id FROM info:info WHERE valid == true AND version == 1",
				&Session::for_kv(),
				None,
				false,
			)
			.await?;
		if let &[_, _, Response {
			result: Ok(result), ..
		}] = &res.as_slice()
		{
			println!("Res: {:?}", result);
		} else {
			database
				.execute(INIT_QUERIES, &Session::for_kv(), None, false)
				.await?;
		}
		Ok(Database {
			database,
			files_path,
		})
	}
}
