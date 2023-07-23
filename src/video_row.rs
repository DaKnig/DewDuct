/* video_row.rs
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

use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

// use invidious::video::Video;

use crate::thumbnail::DewThumbnail;
use crate::util;

mod imp {
    use super::*;

    #[derive(/*Properties,*/ Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/video_row.ui")]
    // #[properties(wrapper_type = super::DewVideoRow)]
    pub struct DewVideoRow {
        // Template widgets
        #[template_child]
        pub(super) title: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) channel: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) views: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) published: TemplateChild<gtk::Label>,
        #[template_child]
        pub(crate) thumbnail: TemplateChild<DewThumbnail>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewVideoRow {
        const NAME: &'static str = "DewVideoRow";
        type Type = super::DewVideoRow;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            DewThumbnail::ensure_type();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewVideoRow {}
    impl WidgetImpl for DewVideoRow {}
    impl BoxImpl for DewVideoRow {}
}

glib::wrapper! {
    pub struct DewVideoRow(ObjectSubclass<imp::DewVideoRow>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl DewVideoRow {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub(crate) async fn set_from_params<'a>(
        &'a self,
        author: String,
        id: String,
        length: u32,
        published: u64,
        thumbnails: &Vec<invidious::hidden::VideoThumbnail>,
        title: String,
        views: u64,
    ) -> anyhow::Result<()> {
        let watched_progress: f64 = 0.; // todo!

        self.imp().title.set_text(&title);
        self.imp().channel.set_text(&author);
        self.set_views(views);
        self.set_published(published);
        // self.imp().thumbnail.update_from_vid_data(&vid_data).await
        self.imp()
            .thumbnail
            .update_from_params(id, thumbnails, length, watched_progress)
            .await?;
        Ok(())
    }

    fn set_published(&self, published: u64) {
        let now = SystemTime::now();
        let published: SystemTime =
            UNIX_EPOCH + Duration::from_secs(published);
        let rel_upload_time = if now > published {
            now.duration_since(published)
                .map(|duration| util::format_rel_time(duration) + " ago")
        } else {
            published.duration_since(now).map(|duration| {
                "in ".to_string() + &util::format_rel_time(duration)
            })
        };
        let Ok(rel_upload_time) = rel_upload_time else {
            println!("{}", rel_upload_time.unwrap_err());
            return;
        };

        self.imp().published.set_text(&rel_upload_time);
    }

    fn set_views(&self, views: u64) {
        self.imp().views.set_text(
            &(util::format_semi_engineering(views as f32) + " views"),
        );
    }
}

impl Default for DewVideoRow {
    fn default() -> Self {
        Self::new()
    }
}
