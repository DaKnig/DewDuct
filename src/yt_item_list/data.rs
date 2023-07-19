/* yt_item_list.rs
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

#![allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use glib::Properties;
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use anyhow::Context;
use isahc::AsyncReadResponseExt;

use std::cell::{Cell, Ref, RefCell};
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::path::Path;

use crate::cache::DewCache;
use crate::util::{cache, cache_dir};
use crate::video_row::DewVideoRow;

mod imp_data {
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::DewYtItem)]
    pub struct DewYtItem {
        #[property(get, set)]
        pub title: RefCell<String>,
        #[property(get, set)]
        pub id: RefCell<String>,
        #[property(get, set)]
        pub author: RefCell<String>,
        // #[property(get, set)]
        pub author_thumbnails:
            RefCell<Vec<invidious::hidden::AuthorThumbnail>>,
        #[property(get, set)]
        pub length: Cell<u32>,
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
        pub likes: Cell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewYtItem {
        const NAME: &'static str = "DewYtItem";
        type Type = super::DewYtItem;
        type ParentType = glib::Object;
    }
    impl ObjectImpl for DewYtItem {}
}

glib::wrapper! {
    pub struct DewYtItem(ObjectSubclass<imp_data::DewYtItem>);
}

impl DewYtItem {
    pub fn new() -> Self {
        glib::Object::new()
    }

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

impl From<invidious::video::Video> for DewYtItem {
    fn from(vid: invidious::video::Video) -> Self {
        use invidious::video::Video;
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
            ..
        } = vid.clone();

        let ret = Self::new();

        ret.set_author(author);
        ret.set_author_thumbnails(author_thumbnails);
        ret.set_id(id);
        ret.set_length(length);
        ret.set_likes(likes);
        ret.set_live(live);
        ret.set_published(published);
        ret.set_sub_count_text(sub_count_text);
        ret.set_thumbnails(thumbnails);
        ret.set_title(title);
        ret.set_views(views);

        return ret;
    }
}
