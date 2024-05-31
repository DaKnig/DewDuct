/* channel_header.rs
 *
 * Copyright 2023-2024 DaKnig
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

use std::{cell::RefCell, path::Path};

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use glib::g_warning;
use gtk::{gdk, gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use anyhow::Context;

use crate::cache::DewCache;
use crate::util::{cache, cache_dir};
use crate::yt_item_list::DewYtItem;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/channel_header.ui")]
    pub struct DewChannelHeader {
        // Template widgets
        #[template_child]
        pub(super) channel: TemplateChild<adw::WindowTitle>,
        #[template_child]
        pub(super) thumbnail: TemplateChild<adw::Avatar>,
        #[template_child]
        pub(super) subscribe: TemplateChild<gtk::Button>,

        pub(super) id: RefCell<String>,
        pub(super) is_subscribed: RefCell<bool>,
        subscribed_handle: RefCell<Option<glib::signal::SignalHandlerId>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewChannelHeader {
        const NAME: &'static str = "DewChannelHeader";
        type Type = super::DewChannelHeader;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewChannelHeader {}
    impl WidgetImpl for DewChannelHeader {}
    impl BoxImpl for DewChannelHeader {}

    #[gtk::template_callbacks]
    impl DewChannelHeader {
        #[template_callback]
        async fn subscribe_clicked(&self, button: &gtk::Button) {
            let win = self.win();
            let id = self.id.borrow().clone();
            let res: Result<(), _> = if !*self.is_subscribed.borrow() {
                win.subscribe(id).await
            } else {
                win.unsubscribe(id);
                Ok(())
            };
            if res.is_err() {
                button.add_css_class("error");
            } else {
                button.remove_css_class("error");
            }
        }
        #[template_callback]
        fn background_clicked(&self) {
            g_warning!(
                "DewChannelHeader",
                "background {} clicked!",
                self.id.borrow()
            );
        }
        #[template_callback]
        fn play_all_clicked(&self) {
            g_warning!(
                "DewChannelHeader",
                "play_all {} clicked!",
                self.id.borrow()
            );
        }
        #[template_callback]
        fn poppup_clicked(&self) {
            g_warning!(
                "DewChannelHeader",
                "poppup {} clicked!",
                self.id.borrow()
            );
        }
    }

    impl DewChannelHeader {
        fn win(&self) -> crate::window::DewDuctWindow {
            self.obj().root().and_downcast().unwrap()
        }
        fn is_subbed(&self, list_store: &gio::ListStore) -> bool {
            list_store.into_iter().flatten().any(|item| {
                item.downcast_ref::<DewYtItem>().is_some_and(|item| {
                    item.id() == self.id.borrow().as_str()
                })
            })
        }
        fn set_is_subscribed(&self, is_subscribed: bool) {
            self.is_subscribed.replace(is_subscribed);
            self.subscribe.get().set_label(if is_subscribed {
                "SUBSCRIBED"
            } else {
                "SUBSCRIBE"
            });
        }
        fn set_id(&self, new: String) {
            self.id.replace(new);
            // if not yet connected to update with the list of subscriptions,
            if self.subscribed_handle.borrow().is_none() {
                let header = self.downgrade();
                let check_is_subbed = move |list_store: &gio::ListStore| {
                    let Some(header) = header.upgrade() else {
                        return;
                    };
                    let is_subbed = header.is_subbed(list_store);
                    header.set_is_subscribed(is_subbed);
                };

                let handle =
                    self.win().connect_subs_changed(check_is_subbed);
                self.subscribed_handle.replace(Some(handle));
            }
        }
        pub async fn set_from_yt_item(
            &self,
            item: &DewYtItem,
        ) -> anyhow::Result<()> {
            self.channel.set_title(&item.title());
            self.channel.set_subtitle(&format!(
                "{} subscribers",
                crate::format_semi_engineering(item.subscribers())
            ));

            let thumbnails = item.thumbnails();

            if thumbnails.is_empty() {
                g_warning!(
                    "DewChannelHeader",
                    "No thumbnails for channel header of {}!",
                    item.id()
                );
                Err(Err::NoThumbnails { id: item.id() })?;
            }

            let thumb = thumbnails
                .iter()
                .filter(|thumb| thumb.width >= 160)
                .min_by_key(|thumb| thumb.width)
                .or(thumbnails.iter().max_by_key(|thumb| thumb.width))
                .with_context(|| {
                    format!(
                        "error fetching channel {} thumbnail",
                        item.id()
                    )
                })?;
            self.set_id(item.id());

            // thumbnail_fname.push();
            let mut thumbnail_fname = cache_dir(Path::new(&item.id()));
            thumbnail_fname.push(&thumb.height.to_string());
            thumbnail_fname.set_extension("jpg");

            DewCache::fetch_remote(
                cache(),
                thumbnail_fname.clone(),
                &thumb.url,
            )
            .await
            .map_err(|err| {
                g_warning!(
                    "DewChannelHeader",
                    "could not fetch file {}: {err}",
                    thumbnail_fname.clone().display()
                )
            })
            .unwrap();

            let paintable = gdk::Texture::from_filename(thumbnail_fname)?;
            self.thumbnail.set_custom_image(Some(&paintable));

            Ok(())
        }
    }
}

glib::wrapper! {
    pub struct DewChannelHeader(ObjectSubclass<imp::DewChannelHeader>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl DewChannelHeader {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub async fn set_from_yt_item(
        &self,
        item: &DewYtItem,
    ) -> anyhow::Result<()> {
        self.imp().set_from_yt_item(item).await
    }
}

impl Default for DewChannelHeader {
    fn default() -> Self {
        Self::new()
    }
}

use thiserror::Error;
#[derive(Error, Debug)]
pub enum Err {
    #[error("no thumbnails found for vid ID {id} video")]
    NoThumbnails { id: String },
}
