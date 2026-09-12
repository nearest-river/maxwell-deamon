

use tokio::io;
use gix::Repository;
use crate::fs::FileSystem;

use std::{
  sync::Arc,
  path::Path,
};


pub struct BareCase {
  pub(crate) repo: Repository,
  pub(crate) fs: Arc<FileSystem>,
}


impl BareCase {
  pub async fn read_file(&self,filepath: impl AsRef<Path>)-> io::Result<Vec<u8>> {
    let fullpath=self.repo.path()
    .join(filepath.as_ref());

    self.fs.read_file(fullpath).await
  }

  pub async fn write_file(&self,filepath: impl AsRef<Path>,contents: Vec<u8>)-> io::Result<()> {
    let fullpath=self.repo.path()
    .join(filepath.as_ref());

    self.fs.write_file(fullpath,contents).await
  }
}





