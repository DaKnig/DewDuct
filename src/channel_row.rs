/* channel_row.rs
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

use std::ops::Deref;
use std::{cell::RefCell, path::Path};

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use glib::{g_warning, Properties};
use gtk::{gdk, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use anyhow::Context;

use crate::cache::DewCache;
use crate::util;
use crate::yt_item_list::Thumbnail;
use crate::{cache, cache_dir};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/null/daknig/DewDuct/channel_row.ui")]
    #[properties(wrapper_type=super::DewChannelRow)]
    pub struct DewChannelRow {
        #[template_child]
        pub(super) thumbnail: TemplateChild<adw::Avatar>,
        #[template_child]
        pub(super) name: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) subs: TemplateChild<gtk::Label>,

        #[property(get, set)]
        pub(super) id: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewChannelRow {
        const NAME: &'static str = "DewChannelRow";
        type Type = super::DewChannelRow;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewChannelRow {}
    impl WidgetImpl for DewChannelRow {}
    impl BoxImpl for DewChannelRow {}
}

glib::wrapper! {
    pub struct DewChannelRow(ObjectSubclass<imp::DewChannelRow>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl DewChannelRow {
    pub async fn set_from_params(
        &self,
        name: String,
        subs: f32,
        thumbnails: impl Deref<Target = Vec<Thumbnail>>,
        id: String,
    ) -> anyhow::Result<()> {
        self.imp().name.set_text(&name);
        self.set_subs(subs);

        if thumbnails.is_empty() {
            g_warning!(
                "DewChannelRow",
                "No thumbnails for channel row of {}!",
                id
            );
            Err(Err::NoThumbnails { id: id.clone() })?;
        }
        let thumb = thumbnails
            .iter()
            .filter(|thumb| thumb.width >= 160)
            .min_by_key(|thumb| thumb.width)
            .or(thumbnails.iter().max_by_key(|thumb| thumb.width))
            .with_context(|| {
                format!("error fetching channel {} thumbnail", &name)
            })?
            .clone();

        drop(thumbnails);

        let mut thumbnail_fname = cache_dir(Path::new(&id));
        thumbnail_fname.push(&thumb.height.to_string());
        thumbnail_fname.set_extension("jpg");

        let status = DewCache::fetch_remote(
            cache(),
            thumbnail_fname.clone(),
            &thumb.url,
        )
        .await;

        let paintable: gdk::Texture = match status {
            Ok(data) => {
                let content_bytes = glib::Bytes::from_owned(data);
                gdk::Texture::from_bytes(&content_bytes)?
            }
            Err(err) => {
                g_warning!(
                    "DewChannelRow",
                    "could not fetch file {}: {err}",
                    thumbnail_fname.clone().display()
                );
                gdk::Texture::from_resource(
                    "/null/daknig/DewDuct/dummi_thumbnail.svg",
                )
            }
        };
        self.imp().thumbnail.set_custom_image(Some(&paintable));

        Ok(())
    }
    fn set_subs(&self, subs: f32) {
        self.imp().subs.set_text(
            &(util::format_semi_engineering(subs) + " subscribers"),
        );
    }
}

use thiserror::Error;
#[derive(Error, Debug)]
pub enum Err {
    #[error("no thumbnails found for vid ID {id} video")]
    NoThumbnails { id: String },
}
