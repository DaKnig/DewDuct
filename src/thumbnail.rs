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
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use anyhow::Context;
use invidious::video::Video;
use isahc::AsyncReadResponseExt;

use std::fs::File;
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
            self.thumbnail.get().set_resource(Some(
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
    fn set_length(&self, length: u32) {
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

    pub(crate) async fn update_from_vid_data(
        &self,
        vid_data: impl std::ops::Deref<Target = Video>,
    ) -> anyhow::Result<()> {
        self.set_length(vid_data.length);

        let thumb = vid_data
            .thumbnails
            .iter()
            .filter(|thumb| thumb.width >= 320)
            .min_by_key(|thumb| thumb.width)
            .ok_or(Err::NoThumbnails {
                id: vid_data.id.clone(),
            })?;

        // thumbnail_fname.push();
        let mut thumbnail_fname = cache_dir(Path::new(&vid_data.id));
        thumbnail_fname.push(&thumb.quality);
        thumbnail_fname.set_extension("jpg");

        let fetcher = async {
            let mut dest: File = {
                // can safely unwrap since I crafted the directory right
                let parent = thumbnail_fname.parent().unwrap();
                std::fs::create_dir_all(parent)?;
                File::create(&thumbnail_fname)
                    .with_context(|| {
                        format!("{}", thumbnail_fname.display())
                    })
                    .unwrap()
                //?
            };

            let target = &thumb.url;
            let mut response = isahc::get_async(target).await?;

            let content: &[u8] = &response.bytes().await?;
            dest.write(content).with_context(|| {
                format!("error writing to {}", thumbnail_fname.display())
            })?;

            // now it is time to load that jpg into the thumbnail

            anyhow::Ok(())
        };

        DewCache::fetch_file(cache(), &thumbnail_fname, fetcher).await?;
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
