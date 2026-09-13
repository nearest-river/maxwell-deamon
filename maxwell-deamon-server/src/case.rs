

use tokio::io;
use gix::Repository;
use std::path::Path;
use crate::fs::FileSystem;



pub struct BareCase<'a> {
  pub(crate) repo: Repository,
  pub(crate) fs: &'a FileSystem,
}


impl BareCase<'_> {
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





