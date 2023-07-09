/* search_page.rs
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

// use std::{cell::RefCell, rc::Rc};

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use crate::video_row::DewVideoRow;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/search_page.ui")]
    pub struct DewSearchPage {
        #[template_child]
        pub(super) search_bar: TemplateChild<gtk::SearchBar>,
        #[template_child]
        pub(super) search_entry: TemplateChild<gtk::SearchEntry>,
        // vid: Rc<RefCell<Option<Video>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewSearchPage {
        const NAME: &'static str = "DewSearchPage";
        type Type = super::DewSearchPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            DewVideoRow::ensure_type();
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewSearchPage {
        fn constructed(&self) {
            self.parent_constructed();
            self.search_bar.connect_entry(&*self.search_entry);
            // self.search_bar.set_key_capture_widget(Some(&self.root()));
        }
    }
    impl WidgetImpl for DewSearchPage {}
    impl BoxImpl for DewSearchPage {}

    #[gtk::template_callbacks]
    impl DewSearchPage {
        #[template_callback]
        pub(crate) fn search_started(&self) {
            glib::g_warning!("Dew", "search_started");
        }
    }
}

glib::wrapper! {
    pub struct DewSearchPage(ObjectSubclass<imp::DewSearchPage>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl DewSearchPage {
    pub fn search_bar(&self) -> &gtk::SearchBar {
        &self.imp().search_bar
    }
    pub fn search_entry(&self) -> &gtk::SearchEntry {
        &self.imp().search_entry
    }
}
