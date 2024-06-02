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
use std::rc::Rc;

use invidious::channel::Channel;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "MyEnum")]
pub enum DewYtItemKind {
    #[default]
    Video,
    Channel,
    // header because of the limitation of ListView, you cant have it as a
    // separate widget on top of the list... sad.
    Header,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Thumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

fn normalize_thumbnail_url(s: String) -> String {
    // sometimes links start with plain `//`. strip that.
    if s.starts_with("https:") {
        s
    } else {
        "https:".to_owned() + &s
    }
}

impl From<invidious::CommonThumbnail> for Thumbnail {
    fn from(thumb: invidious::CommonThumbnail) -> Self {
        Self {
            url: normalize_thumbnail_url(thumb.url),
            width: thumb.width,
            height: thumb.height,
        }
    }
}

impl From<invidious::CommonImage> for Thumbnail {
    fn from(thumb: invidious::CommonImage) -> Self {
        Self {
            url: normalize_thumbnail_url(thumb.url),
            width: thumb.width,
            height: thumb.height,
        }
    }
}

mod imp_data {
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::DewYtItem)]
    pub struct DewYtItem {
        pub(super) kind: Cell<DewYtItemKind>,

        #[property(get, set)]
        pub title: RefCell<String>,
        #[property(get, set)]
        pub id: RefCell<String>,
        #[property(get, set)]
        pub author: RefCell<String>,
        // #[property(get, set)]
        pub author_thumbnails: RefCell<Vec<Thumbnail>>,
        #[property(get, set)]
        pub length: Cell<u64>,
        // #[property(get, set)]
        pub thumbnails: RefCell<Rc<Vec<Thumbnail>>>,
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
        #[property(get, set)]
        pub subscribers: Cell<f32>,
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
    pub fn thumbnails(&self) -> Rc<Vec<Thumbnail>> {
        self.imp().thumbnails.borrow().clone()
    }
    pub fn set_thumbnails(&self, thumbs: Vec<Thumbnail>) {
        self.imp().thumbnails.replace(Rc::new(thumbs));
    }

    pub fn author_thumbnails(&self) -> Ref<Vec<Thumbnail>> {
        self.imp().author_thumbnails.borrow()
    }
    pub fn set_author_thumbnails(&self, author_thumbs: Vec<Thumbnail>) {
        self.imp().author_thumbnails.replace(author_thumbs);
    }

    pub fn kind(&self) -> DewYtItemKind {
        self.imp().kind.get()
    }
    fn set_kind(&self, new_val: DewYtItemKind) {
        self.imp().kind.set(new_val);
    }

    pub fn header(channel: &invidious::channel::Channel) -> Self {
        let ret: Self = glib::Object::builder()
            .property("id", &channel.id)
            .property("author", &channel.name)
            .property("title", &channel.name)
            .property("subscribers", channel.subscribers as f32)
            .build();

        ret.set_thumbnails(
            channel
                .thumbnails
                .iter()
                .map(|thumb| thumb.clone().into())
                .collect::<Vec<_>>(),
        );
        ret.set_kind(DewYtItemKind::Header);
        ret
    }
}

use invidious::hidden::SearchItem;
impl From<SearchItem> for DewYtItem {
    fn from(vid: SearchItem) -> Self {
        match vid {
            SearchItem::Video(ref vid) => vid.into(),
            SearchItem::Channel(chan) => chan.into(),
            _ => todo!(),
        }
    }
}

impl From<CommonChannel> for DewYtItem {
    fn from(chan: CommonChannel) -> Self {
        let CommonChannel {
            name,
            id,
            description,
            subscribers,
            thumbnails,
            ..
        } = chan;
        let ret: Self = glib::Object::builder()
            .property("author", &name)
            .property("id", id)
            .property("title", name)
            .property("description", description)
            .property("subscribers", subscribers as f32)
            .build();
        let thumbnails: Vec<_> =
            thumbnails.into_iter().map(|x| x.into()).collect();
        ret.set_thumbnails(thumbnails);
        ret.set_kind(DewYtItemKind::Channel);

        ret
    }
}

impl From<Channel> for DewYtItem {
    fn from(chan: Channel) -> Self {
        let cc: CommonChannel = chan.into();
        cc.into()
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
        let thumbnails: Vec<_> =
            thumbnails.into_iter().map(|x| x.into()).collect();
        ret.set_thumbnails(thumbnails);
        ret.set_kind(DewYtItemKind::Video);

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

        let author_thumbnails: Vec<_> =
            author_thumbnails.into_iter().map(|x| x.into()).collect();
        ret.set_author_thumbnails(author_thumbnails);
        let thumbnails: Vec<_> =
            thumbnails.into_iter().map(|x| x.into()).collect();
        ret.set_thumbnails(thumbnails);
        ret.set_kind(DewYtItemKind::Video);

        ret
    }
}

use invidious::{CommonChannel, CommonVideo};
impl From<&CommonVideo> for DewYtItem {
    fn from(vid: &CommonVideo) -> Self {
        let CommonVideo {
            author,
            description,
            id,
            length,
            live,
            published,
            thumbnails,
            title,
            views,
            ..
        } = vid;

        let ret: Self = glib::Object::builder()
            .property("author", author)
            .property("description", Some(description))
            .property("id", id)
            .property("length", *length as u64)
            .property("likes", 0)
            .property("live", live)
            .property("published", published)
            .property("sub-count-text", "")
            .property("title", title)
            .property("views", views)
            .build();

        ret.set_author_thumbnails(vec![]);
        let thumbnails: Vec<_> =
            thumbnails.iter().map(|x| x.clone().into()).collect();
        ret.set_thumbnails(thumbnails);
        ret.set_kind(DewYtItemKind::Video);

        ret
    }
}
