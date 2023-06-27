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

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use invidious::video::Video;

use crate::cache::DewCache;
use crate::thumbnail::DewThumbnail;

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

    pub fn thumbnail(&self) -> gtk::Picture {
        self.imp().thumbnail.thumbnail()
    }

    pub async fn set_from_video_data(
        &self,
        vid_data: Video,
        cache: DewCache,
    ) -> anyhow::Result<()> {
        self.imp().title.set_text(&vid_data.title);
        self.imp().channel.set_text(&vid_data.author);
        self.set_views(vid_data.views);
        self.set_published(vid_data.published);
        self.imp().thumbnail.set_length(vid_data.length);

        self.imp().thumbnail.update_from_vid_data(cache, vid_data).await
    }

    pub fn set_published(&self, published: u64) {
        self.imp().published.set_text(&format!("{}", published));
    }

    pub fn set_views(&self, views: u64) {
        static SUFFIXES: [char; 5] = [' ', 'k', 'M', 'G', 'T'];
        let suffix = (0..)
            .map(|x| 1000f64.powi(x) as u64)
            .zip(SUFFIXES)
            .filter(|x| views > x.0)
            .last()
            .unwrap();

        self.imp().views.set_text(&format!(
            "{}{} views",
            views / suffix.0,
            suffix.1
        ));
    }
}

impl Default for DewVideoRow {
    fn default() -> Self {
        Self::new()
    }
}
