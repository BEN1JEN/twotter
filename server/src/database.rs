use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;
use surrealdb::{Datastore, Session};
use tokio::fs;

pub struct Database {
	database: Datastore,
	files_path: PathBuf,
}

const INIT_QUERIES: &str = include_str!(setup.surreal);

impl Database {
	pub async fn open<P: AsRef<Path>>(path: P) -> Result<Database> {
		let files_path = path.as_ref().join("files");
		fs::create_dir_all(&files_path).await?;
		let database = Datastore::new(&format!("file://{}", path.as_ref().display())).await?;
		let res = database.execute("SELECT id FROM info:info WHERE valid == true AND version == 1", &Session::for_kv(), None, false).await?;
		if !matches!(&res, &[]) {}
		Ok(Database {
			database,
			files_path,
		})
	}
}
