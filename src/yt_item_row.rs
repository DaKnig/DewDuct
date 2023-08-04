/* yt_item_row.rs
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

use crate::channel_row::DewChannelRow;
use crate::video_row::DewVideoRow;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/yt_item_row.ui")]
    // #[properties(wrapper_type = super::YtItemRow)]
    pub struct DewYtItemRow {
        // Template widgets
        #[template_child]
        pub(super) stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub(super) video_row: TemplateChild<DewVideoRow>,
        #[template_child]
        pub(super) channel_row: TemplateChild<DewChannelRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewYtItemRow {
        const NAME: &'static str = "DewYtItemRow";
        type Type = super::DewYtItemRow;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            DewChannelRow::ensure_type();
            DewVideoRow::ensure_type();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewYtItemRow {}
    impl WidgetImpl for DewYtItemRow {}
    impl BinImpl for DewYtItemRow {}
}

glib::wrapper! {
    pub struct DewYtItemRow(ObjectSubclass<imp::DewYtItemRow>)
        @extends gtk::Widget, adw::Bin,
        @implements gio::ActionGroup, gio::ActionMap;
}

use crate::yt_item_list::{DewYtItem, DewYtItemKind};

impl DewYtItemRow {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn become_video(&self) -> DewVideoRow {
        let vid = self.imp().video_row.get();
        self.imp().stack.set_visible_child(&vid);
        vid
    }
    pub fn become_channel(&self) -> DewChannelRow {
        let chan = self.imp().channel_row.get();
        self.imp().stack.set_visible_child(&chan);
        chan
    }

    pub async fn set_from_yt_item(
        &self,
        item: &DewYtItem,
    ) -> anyhow::Result<()> {
        use DewYtItemKind::*;
        match item.kind() {
            Video => {
                self.become_video()
                    .set_from_params(
                        item.author(),
                        item.id(),
                        item.length(),
                        item.published(),
                        item.thumbnails().iter(),
                        item.title(),
                        item.views(),
                    )
                    .await?;
                // todo!()
            }
            Channel => {
                self.become_channel().set_from_params(
                    item.title(),
                    item.subscribers(),
                    &item.thumbnails(),
                );
                // todo!()
            }
            Header => unreachable!(),
        }
        Ok(())
    }
}

impl Default for DewYtItemRow {
    fn default() -> Self {
        Self::new()
    }
}
