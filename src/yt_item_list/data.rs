/* yt_item_list/data.rs
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
use glib::{ParamSpec, Properties, Value};
use gtk::glib;
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use std::cell::{Cell, Ref, RefCell};

mod imp_data {
    use super::*;

    #[derive(Default, Debug, Copy, Clone, PartialEq, Eq, glib::Enum)]
    #[enum_type(name = "MyEnum")]
    pub enum DewYtItemKind {
        #[default]
        Video,
        Channel,
    }

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::DewYtItem)]
    pub struct DewYtItem {
        pub kind: RefCell<DewYtItemKind>,

        #[property(get, set)]
        pub title: RefCell<String>,
        #[property(get, set)]
        pub id: RefCell<String>,
        #[property(construct, get, set)]
        pub author: RefCell<String>,
        // #[property(get, set)]
        pub author_thumbnails:
            RefCell<Vec<invidious::hidden::AuthorThumbnail>>,
        #[property(get, set)]
        pub length: Cell<u64>,
        // #[property(get, set)]
        pub thumbnails: RefCell<Vec<invidious::hidden::VideoThumbnail>>,
        #[property(get, set)]
        pub views: Cell<u64>,
        #[property(get, set)]
        pub published: Cell<u64>,
        #[property(get, set)]
        pub sub_count_text: RefCell<String>,
        #[property(get, set)]
        pub live: Cell<bool>,
        #[property(get, set)]
        pub likes: Cell<i32>,
        #[property(get, set)]
        pub description: RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewYtItem {
        const NAME: &'static str = "DewYtItem";
        type Type = super::DewYtItem;
        type ParentType = glib::Object;
    }
    impl ObjectImpl for DewYtItem {
        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(
            &self,
            id: usize,
            value: &Value,
            pspec: &ParamSpec,
        ) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
            self.derived_property(id, pspec)
        }
    }
}

glib::wrapper! {
    pub struct DewYtItem(ObjectSubclass<imp_data::DewYtItem>);
}

impl DewYtItem {
    pub fn thumbnails(
        &self,
    ) -> Ref<Vec<invidious::hidden::VideoThumbnail>> {
        self.imp().thumbnails.borrow()
    }
    pub fn set_thumbnails(
        &self,
        thumbs: Vec<invidious::hidden::VideoThumbnail>,
    ) {
        self.imp().thumbnails.replace(thumbs);
    }

    pub fn author_thumbnails(
        &self,
    ) -> Ref<Vec<invidious::hidden::AuthorThumbnail>> {
        self.imp().author_thumbnails.borrow()
    }
    pub fn set_author_thumbnails(
        &self,
        author_thumbs: Vec<invidious::hidden::AuthorThumbnail>,
    ) {
        self.imp().author_thumbnails.replace(author_thumbs);
    }
}

use invidious::hidden::SearchItem;
impl From<SearchItem> for DewYtItem {
    fn from(vid: SearchItem) -> Self {
        if let SearchItem::Video {
            author,
            id,
            length,
            live,
            published,
            thumbnails,
            title,
            views,
            description,
            ..
        } = vid
        {
            let ret: Self = glib::Object::builder()
                .property("author", author)
                .property("id", id)
                .property("length", length as u64)
                .property("likes", 0)
                .property("live", live)
                .property("published", published)
                .property("sub-count-text", "")
                .property("title", title)
                .property("views", views)
                .property("description", Some(description))
                .build();

            ret.set_author_thumbnails(vec![]);
            ret.set_thumbnails(thumbnails);
            ret.imp().kind.replace(imp_data::DewYtItemKind::Video);

            ret
        } else {
            todo!()
        }
    }
}

use invidious::hidden::PopularItem;
impl From<PopularItem> for DewYtItem {
    fn from(item: PopularItem) -> Self {
        let PopularItem {
            author,
            id,
            length,
            published,
            thumbnails,
            title,
            views,
            ..
        } = item;

        let ret: Self = glib::Object::builder()
            .property("author", author)
            .property("id", id)
            .property("length", length as u64)
            .property("likes", 0)
            .property("live", false)
            .property("published", published)
            .property("sub-count-text", "".to_string())
            .property("title", title)
            .property("views", views)
            .property("description", None::<String>)
            .build();

        ret.set_author_thumbnails(vec![]);
        ret.set_thumbnails(thumbnails);
        ret.imp().kind.replace(imp_data::DewYtItemKind::Video);

        ret
    }
}

use invidious::video::Video;
impl From<Video> for DewYtItem {
    fn from(vid: Video) -> Self {
        let Video {
            author,
            author_thumbnails,
            id,
            length,
            likes,
            live,
            published,
            sub_count_text,
            thumbnails,
            title,
            views,
            description,
            ..
        } = vid;

        let ret: Self = glib::Object::builder()
            .property("author", author)
            .property("id", id)
            .property("length", length)
            .property("likes", likes)
            .property("live", live)
            .property("published", published)
            .property("sub-count-text", sub_count_text)
            .property("title", title)
            .property("views", views)
            .property("description", Some(description))
            .build();

        ret.set_author_thumbnails(author_thumbnails);
        ret.set_thumbnails(thumbnails);
        ret.imp().kind.replace(imp_data::DewYtItemKind::Video);

        ret
    }
}
