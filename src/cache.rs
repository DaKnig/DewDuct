// use invidious::video::Video;

use std::fs::File;
use std::future::Future;
use std::path::{Path, PathBuf};

use gtk::glib;

use crate::config;

#[derive(Clone)]
pub struct DewCache {
    /// Root location of the cache files.
    dir: PathBuf,
}

impl DewCache {
    pub fn new(dir: &Path) -> Self {
        DewCache { dir: dir.into() }
    }
    pub(crate) fn dir(&self) -> &PathBuf {
        &self.dir
    }
    /// cache: the cache with the directory where the info should be stored.
    /// fname: file we are looking for, relative to the cache.
    /// fetcher: function for fetching said file, if it is not in cache.
    pub(crate) async fn fetch_file<E>(
        cache: &Self,
        fname: &Path,
        fetcher: impl Future<Output = Result<(), E>>,
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
        let mut dir = glib::tmp_dir();
        dir.push(config::PKGNAME);

        DewCache { dir }
    }
}
