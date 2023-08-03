// use invidious::video::Video;

use std::fs::File;
use std::future::Future;
use std::path::{Path, PathBuf};

use glib::g_warning;
use gtk::glib;

use crate::config;

#[derive(Clone)]
pub struct DewCache {
    /// Root location of the cache files.
    dir: PathBuf,
}

impl DewCache {
    pub(crate) fn dir(&self) -> &PathBuf {
        &self.dir
    }
    /// cache: the cache with the directory where the info should be stored.
    /// fname: file we are looking for, relative to the cache.
    /// fetcher: function for fetching said file, if it is not in cache.
    pub(crate) async fn fetch_file<Fetcher, Err, Fut>(
        cache: &Self,
        fname: PathBuf,
        fetcher: Fetcher,
    ) -> Result<(), Err>
    where
        Fetcher: Fn(&Path) -> Fut,
        Fut: Future<Output = Result<(), Err>>,
    {
        let path = cache.dir().join(&fname);
        match File::open(&path).ok() {
            Some(_) => {
                g_warning!(
                    "DewCache",
                    "opening cached file at {}",
                    &path.display()
                );
                Ok(())
            }
            None => {
                g_warning!(
                    "DewCache",
                    "fetching item to {}",
                    &path.display()
                );

                let mut ret = fetcher(&fname).await;
                for i in 0..3 {
                    if ret.is_ok() {
                        break;
                    }
                    g_warning!(
                        "DewCache",
                        "retrying {} now {i} times...",
                        fname.display()
                    );
                    ret = fetcher(&fname).await;
                }
                ret
            }
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
