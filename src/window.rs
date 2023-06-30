/* window.rs
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

use crate::update_page::DewUpdatePage;
use crate::video_page::DewVideoPage;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/window.ui")]
    pub struct DewDuctWindow {
        // Template widgets
        #[template_child]
        pub flap: TemplateChild<adw::Flap>,
        #[template_child(id = "view-stack")]
        pub view_stack: TemplateChild<adw::ViewStack>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewDuctWindow {
        const NAME: &'static str = "DewDuctWindow";
        type Type = super::DewDuctWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            DewUpdatePage::ensure_type();
            DewVideoPage::ensure_type();
            klass.bind_template();
            // klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewDuctWindow {}
    impl WidgetImpl for DewDuctWindow {}
    impl WindowImpl for DewDuctWindow {}
    impl ApplicationWindowImpl for DewDuctWindow {}
    impl AdwApplicationWindowImpl for DewDuctWindow {}
}

glib::wrapper! {
    pub struct DewDuctWindow(ObjectSubclass<imp::DewDuctWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl DewDuctWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
