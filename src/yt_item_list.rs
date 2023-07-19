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

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use crate::video_row::DewVideoRow;

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
            DewVideoRow::ensure_type();
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
        fn activate(index: u32, list_view: gtk::ListView) {
            let Some(item) = list_view.model().unwrap().item(index as u32)
                                                 else {return};
            let id: String = item.downcast::<DewYtItem>().unwrap().id();
            list_view
                .activate_action("win.play", Some(&Some(id).to_variant()))
                .expect("the action win.play does not exist");
        }

        #[template_callback(function)]
        fn setup_row(list_item: gtk::ListItem) {
            let row = DewVideoRow::new();
            list_item.set_child(Some(&row));
        }

        #[template_callback(function)]
        async fn bind_row(list_item: gtk::ListItem) {
            let item: DewYtItem = list_item
                .item()
                .and_downcast()
                .expect("The item has to be an `DewYtItem`");
            // get_type_of_value(&boxed);

            // let item = item.imp();

            let row: DewVideoRow = list_item
                .child()
                .and_downcast()
                .expect("The item needs to be a DewVideoRow");

            row.set_from_params(
                item.author(),
                item.id(),
                item.length() as u32,
                item.published(),
                &item.thumbnails(),
                item.title(),
                item.views(),
            )
            .await
            .unwrap_or_else(|err| {
                glib::g_warning!("Dew", "{}", err);
            });
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
}
