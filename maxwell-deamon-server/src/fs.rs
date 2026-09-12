
use crate::case::BareCase;

use std::{
  sync::Arc,
  ffi::OsStr,
  path::{
    Path,
    PathBuf,
  },
};

use tokio::{
  fs,
  task,
  io::{
    self,
    ErrorKind,
  },
};



pub struct FileSystem {
  root: Box<Path>,
}



impl FileSystem {
  pub async fn new(root: impl AsRef<Path>)-> io::Result<Self> {
    let root_=root;
    let root=Box::<Path>::from(root_.as_ref());

    if !fs::metadata(root_).await?.is_dir() {
      return Err(io::Error::new(ErrorKind::NotFound,"filesystem root not found"));
    }

    Ok(Self {
      root,
    })
  }

  pub async fn open_case(self: Arc<Self>,path: impl Into<PathBuf>)-> anyhow::Result<BareCase> {
    let path=path.into();
    let repo=task::spawn_blocking(move || gix::open(path)).await??;

    Ok(BareCase {
      repo,
      fs: Arc::clone(&self),
    })
  }

  pub async fn init_case(self: Arc<Self>,path: impl Into<PathBuf>)-> anyhow::Result<BareCase> {
    let path=path.into();
    let repo=task::spawn_blocking(move || gix::init(path)).await??;

    Ok(BareCase {
      repo,
      fs: Arc::clone(&self),
    })
  }

  #[inline(always)]
  pub async fn write_file(&self,path: impl AsRef<Path>,buf: impl AsRef<[u8]>)-> io::Result<()> {
    fs::write(self.root.join(path.as_ref()),buf).await
  }

  pub async fn read_file(&self,path: impl AsRef<Path>)-> io::Result<Vec<u8>> {
    fs::read(self.root.join(path.as_ref())).await
  }


}





