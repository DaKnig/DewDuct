/* application.rs
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

use adw::subclass::prelude::*;
use glib::g_warning;
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::DewDuctWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct DewDuctApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for DewDuctApplication {
        const NAME: &'static str = "DewDuctApplication";
        type Type = super::DewDuctApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for DewDuctApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for DewDuctApplication {
        // We connect to the activate callback to create a window when the
        // application has been launched. Additionally, this callback
        // notifies us when the user tries to launch a "second instance" of
        // the application. When they try to do that, we'll just present any
        // existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = DewDuctWindow::new(&*application);
                window.upcast()
            };

            g_warning!(
                "DewApplication",
                "instance used: {}",
                window
                    .clone()
                    .downcast::<DewDuctWindow>()
                    .unwrap()
                    .invidious_client()
                    .instance
            );

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for DewDuctApplication {}
    impl AdwApplicationImpl for DewDuctApplication {}
}

glib::wrapper! {
    pub struct DewDuctApplication(ObjectSubclass<imp::DewDuctApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl DewDuctApplication {
    pub fn new(
        application_id: &str,
        flags: &gio::ApplicationFlags,
    ) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
        self.set_accels_for_action("win.back", &["Escape"]);
        self.set_accels_for_action("win.search_started", &["<Ctrl>f"]);
    }

    fn show_about(&self) {
        let about = adw::AboutWindow::from_appdata(
            "/null/daknig/DewDuct/null.daknig.dewduct.metainfo.xml",
            None,
        );
        about.set_copyright("© 2023-2024 DaKnig");
        about.set_developers(&["DaKnig"]);

        about.present();
    }
}
