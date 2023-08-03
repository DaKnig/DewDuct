/* channel_header.rs
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
// use glib::g_warning;
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/channel_header.ui")]
    pub struct DewChannelHeader {
        // Template widgets
        // #[template_child]
        // pub(super) title: TemplateChild<gtk::Label>,
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
        fn subscribe_clicked(&self) {}
        #[template_callback]
        fn background_clicked(&self) {}
        #[template_callback]
        fn play_all_clicked(&self) {}
	#[template_callback]
	fn poppup_clicked(&self) {}
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
}

impl Default for DewChannelHeader {
    fn default() -> Self {
        Self::new()
    }
}
