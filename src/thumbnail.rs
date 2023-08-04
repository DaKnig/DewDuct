/* thumbnail.rs
 *
 * Copyright 2023 DaKnig
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use glib::g_warning;
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use anyhow::Context;
use isahc::AsyncReadResponseExt;

use std::fs::File;
use std::future::Future;
use std::io::Write;
use std::path::Path;

use crate::cache::DewCache;
use crate::util::{cache, cache_dir};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/thumbnail.ui")]
    pub struct DewThumbnail {
        // Template widgets
        #[template_child]
        pub(super) thumbnail: TemplateChild<gtk::Picture>,
        #[template_child]
        pub(super) length: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) watched_progress: TemplateChild<gtk::ProgressBar>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewThumbnail {
        const NAME: &'static str = "DewThumbnail";
        type Type = super::DewThumbnail;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
        // g_get_tmp_dir ###@@
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewThumbnail {
        fn constructed(&self) {
            self.thumbnail.set_resource(Some(
                "/null/daknig/DewDuct/dummi_thumbnail.svg",
            ));
        }
    }
    impl WidgetImpl for DewThumbnail {}
    impl BoxImpl for DewThumbnail {}
}

glib::wrapper! {
    pub struct DewThumbnail(ObjectSubclass<imp::DewThumbnail>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl DewThumbnail {
    fn set_length(&self, length: u64) {
        let (hrs, mins, secs) =
            (length / 3600, (length / 60) % 60, length % 60);

        let hrs_str = match hrs {
            0 => "".into(),
            hrs => format!("{hrs}:"),
        };

        self.imp()
            .length
            .set_text(&format!("{}{:02}:{:02}", hrs_str, mins, secs));
    }

    fn set_progress(&self, watched_progress: f64) {
        self.imp()
            .watched_progress
            .get()
            .set_fraction(watched_progress);
    }

    pub(crate) async fn update_from_params<'a, T>(
        &'a self,
        id: String,
        thumbnails: impl Iterator<Item = &'a T>,
        length: u64,
        watched_progress: f64,
    ) -> anyhow::Result<()>
    where
        T: Clone + 'a,
        crate::yt_item_list::Thumbnail: From<T>,
    {
        let thumbnails: std::iter::Map<_, _> = thumbnails.map(|thumb| {
            let thumb: crate::yt_item_list::Thumbnail =
                thumb.clone().into();
            thumb
        });

        self.set_length(length);
        self.set_progress(watched_progress);

        let thumb = thumbnails
            .filter(|thumb| thumb.width >= 320)
            .min_by_key(|thumb| thumb.width)
            .ok_or(Err::NoThumbnails { id: id.clone() })?;

        // thumbnail_fname.push();
        let mut thumbnail_fname = cache_dir(Path::new(&id));
        thumbnail_fname.push(&thumb.height.to_string());
        thumbnail_fname.set_extension("jpg");

        fn fetcher(
            fname: &Path,
            url: String,
        ) -> impl Future<Output = anyhow::Result<()>> {
            let fname = fname.to_owned();
            async move {
                let mut dest: File = {
                    // can safely unwrap since I crafted the directory
                    let parent = fname.parent().unwrap();
                    std::fs::create_dir_all(parent)?;
                    File::create(&fname)
                        .with_context(|| format!("{}", fname.display()))
                        .unwrap()
                    //?
                };

                let target = url;
                let mut response = isahc::get_async(target).await?;

                let content: &[u8] = &response.bytes().await?;
                if content.is_empty() {
                    Err(Err::NoThumbnails {
                        id: fname
                            .file_name()
                            .unwrap()
                            .to_owned()
                            .into_string()
                            .unwrap(),
                    })?;
                } else {
                    g_warning!(
                        "DewThumbnail",
                        "writing {} bytes to {}",
                        content.len(),
                        fname.display()
                    );
                }
                dest.write(content).with_context(|| {
                    format!("error writing to {}", fname.display())
                })?;

                // now it is time to load that jpg into the thumbnail

                anyhow::Ok(())
            }
        }

        DewCache::fetch_file(cache(), thumbnail_fname.clone(), |fname| {
            fetcher(fname, thumb.url.clone())
        })
        .await?;
        self.imp()
            .thumbnail
            .set_filename(Some(thumbnail_fname.as_path()));
        Ok(())
    }
}

use thiserror::Error;
#[derive(Error, Debug)]
pub enum Err {
    #[error("no thumbnails found for vid ID {id} video")]
    NoThumbnails { id: String },
}
