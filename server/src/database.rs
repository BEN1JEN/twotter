use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;
use surrealdb::Datastore;
use tokio::fs;

pub struct Database {
	database: Datastore,
	files_path: PathBuf,
}

impl Database {
	pub async fn open<P: AsRef<Path>>(path: P) -> Result<Database> {
		let files_path = path.as_ref().join("files");
		fs::create_dir_all(&files_path).await?;
		let database = Datastore::new(&format!("file://{}", path.as_ref().display())).await?;
		Ok(Database {
			database,
			files_path,
		})
	}
}
