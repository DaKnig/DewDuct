// use invidious::video::Video;
use gtk::glib;
use std::fs::File;
use std::future::Future;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct DewCache {
    /// Root location of the cache files.
    dir: PathBuf,
}

impl DewCache {
    pub fn new(dir: &Path) -> Self {
        DewCache { dir: dir.into() }
    }
    pub fn change_dir(&mut self, dir: &Path) {
        self.dir = dir.into();
    }
    pub fn dir(&self) -> PathBuf {
        self.dir.clone()
    }
    /// cache: the cache with the directory where the info should be stored.
    /// fname: file we are looking for, relative to the cache.
    /// fetcher: function for fetching said file, if it is not in cache.
    pub async fn fetch_file<E>(
        cache: &Self,
        fname: &Path,
        fetcher: impl Future<Output = Result<(), E>>,
        // impl FnOnce() -> Fut
        // where
        //    Fut: Future<Output = ()>,
    ) -> Result<(), E> {
        let path = cache.dir().join(fname);
        match File::open(&path).ok() {
            Some(_) => Ok(()),
            None => fetcher.await,
        }
    }
}

impl Default for DewCache {
    fn default() -> Self {
        DewCache {
            dir: glib::tmp_dir(),
        }
    }
}
