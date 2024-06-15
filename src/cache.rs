use std::fs::{self, metadata};
use std::future::Future;
use std::path::{Path, PathBuf};

use glib::{g_debug, g_warning};
use gtk::glib;

use anyhow::Context;

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
    pub(crate) async fn fetch_remote(
        cache: &Self,
        fname: PathBuf,
        url: &str,
    ) -> anyhow::Result<Vec<u8>> {
        g_warning!("DewCache", "trying to fetch url `{url}`");

        DewCache::fetch_file(cache, fname, move |fname| {
            Self::fetcher(fname, url)
        })
        .await
        .with_context(|| format!("failed to fetch url `{url}`"))
    }
    fn fetcher(
        fname: &Path,
        url: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<u8>>> {
        use anyhow::Context;
        use isahc::AsyncReadResponseExt;

        let fname = fname.to_owned();
        let url = url.to_owned();
        async move {
            let target = url;
            let mut response = isahc::get_async(target).await?;

            let contents = response.bytes().await?;
            if contents.is_empty() {
                Err(Err::NoThumbnails {
                    id: fname
                        .file_name()
                        .unwrap()
                        .to_owned()
                        .into_string()
                        .unwrap(),
                })?;
            }
            g_warning!(
                "DewThumbnail",
                "writing {} bytes to {}",
                contents.len(),
                fname.display()
            );

            // if possible, write to cache
            if let Some(parent) = fname.parent() {
                // try your best, if can't, then no worries
                let _ = fs::create_dir_all(parent);
            }

            fs::write(&fname, &contents)
                .with_context(|| {
                    format!("error writing to {}", fname.display())
                })
                .unwrap_or_else(|e| {
                    g_warning!("DewThumbnail", "{}", e);
                });
            // now it is time to load that jpg into the thumbnail
            anyhow::Ok(contents)
        }
    }
    /// cache: the cache with the directory where the info should be stored.
    /// fname: file we are looking for, relative to the cache.
    /// fetcher: function for fetching said file, if it is not in cache.
    pub(crate) async fn fetch_file<Fetcher, Err, Fut>(
        cache: &Self,
        fname: PathBuf,
        fetcher: Fetcher,
    ) -> Result<Vec<u8>, Err>
    where
        Fetcher: Fn(&Path) -> Fut,
        Fut: Future<Output = Result<Vec<u8>, Err>>,
    {
        let path = cache.dir().join(&fname);
        if metadata(&path).is_ok_and(|m| m.len() != 0) {
            g_debug!(
                "DewCache",
                "opening cached file at {}",
                &path.display()
            );
            if let Ok(contents) = fs::read(&path) {
                return Ok(contents);
            }
            g_debug!(
                "DewCache",
                "unable to read cached file {}",
                &path.display()
            );
        }

        g_warning!("DewCache", "fetching item to {}", &path.display());

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

impl Default for DewCache {
    fn default() -> Self {
        let mut dir = glib::tmp_dir();
        dir.push(config::PKGNAME);

        DewCache { dir }
    }
}

use thiserror::Error;
#[derive(Error, Debug)]
pub enum Err {
    #[error("no thumbnails found for vid ID {id} video")]
    NoThumbnails { id: String },
}
