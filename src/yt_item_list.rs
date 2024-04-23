/* yt_item_list.rs
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

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use glib::g_warning;
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use crate::channel_header::DewChannelHeader;
use crate::yt_item_row::DewYtItemRow;

mod data;
pub use data::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/yt_item_list.ui")]
    pub struct DewYtItemList {
        #[template_child]
        pub(super) list_store: TemplateChild<gio::ListStore>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewYtItemList {
        const NAME: &'static str = "DewYtItemList";
        type Type = super::DewYtItemList;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            DewYtItemRow::ensure_type();
            klass.bind_template();
            klass.bind_template_callbacks();
        }
        // g_get_tmp_dir ###@@
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewYtItemList {}
    impl WidgetImpl for DewYtItemList {}
    impl BinImpl for DewYtItemList {}

    #[gtk::template_callbacks]
    impl DewYtItemList {
        #[template_callback(function)]
        async fn activate(index: u32, list_view: gtk::ListView) {
            use data::DewYtItemKind::*;

            let Some(item) = list_view.model().unwrap().item(index) else {
                return;
            };
            let item: DewYtItem = item.downcast().unwrap();
            let id: String = item.id();
            match item.kind() {
                Video => list_view
                    .activate_action(
                        "win.play",
                        Some(&Some(id).to_variant()),
                    )
                    .expect("the action win.play does not exist"),
                Channel => {
                    let window: crate::window::DewDuctWindow =
                        list_view.root().and_downcast().unwrap();

                    window.show_channel_yt_item(&item).await;
                }
                // clicking on the header outside buttons- does nothing.
                Header => {}
            }
        }

        #[template_callback(function)]
        fn setup_row(list_item: gtk::ListItem) {
            let row = DewYtItemRow::new();
            list_item.set_child(Some(&row));
        }

        #[template_callback(function)]
        async fn bind_row(list_item: gtk::ListItem) {
            let item: DewYtItem = list_item
                .item()
                .and_downcast()
                .expect("The item has to be an `DewYtItem`");

            if item.kind() == DewYtItemKind::Header {
                list_item.set_activatable(false);
                let header = DewChannelHeader::new();
                list_item.set_child(Some(&header));
                if let Err(err) = header.set_from_yt_item(&item).await {
                    g_warning!(
                        "DewYtItemList",
                        "can't bind header row: {}",
                        err
                    );
                }
            } else {
                list_item.set_activatable(true);
                let row: DewYtItemRow =
                    list_item.child().and_downcast().unwrap_or_default();

                row.set_from_yt_item(&item).await.unwrap_or_else(|err| {
                    glib::g_warning!(
                        "DewYtItemList",
                        "error binding row: {}",
                        err
                    );
                });

                // in case it was used as a header a moment ago...
                if !list_item
                    .child()
                    .is_some_and(|x| x.is::<DewYtItemRow>())
                {
                    list_item.set_child(Some(&row));
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct DewYtItemList(ObjectSubclass<imp::DewYtItemList>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl DewYtItemList {
    pub fn remove_all(&self) {
        self.imp().list_store.remove_all()
    }

    pub fn set_from_vec(&self, vec: Vec<DewYtItem>) {
        self.remove_all();
        self.imp().list_store.extend_from_slice(&vec);
    }

    pub fn get_vec(&self) -> Vec<DewYtItem> {
        let list_store = &self.imp().list_store;
        list_store
            .into_iter()
            .filter_map(|x| x.ok())
            .filter_map(|x| x.downcast().ok())
            .collect()
    }
}
