/* main.rs
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

mod application;
mod cache;
mod channel_header;
mod channel_page;
mod channel_row;
mod config;
mod search_page;
mod thumbnail;
mod popular_page;
mod subscriptions_page;
mod util;
mod video_page;
mod video_row;
mod window;
mod yt_item_list;
mod yt_item_row;

use self::application::DewDuctApplication;
use self::window::DewDuctWindow;

// use config::{GETTEXT_PACKAGE, LOCALEDIR};
#[allow(unused_imports)]
use gtk::prelude::*;
use gtk::{gio, glib};

pub use util::*;

#[tokio::main]
async fn main() -> glib::ExitCode {
    // Load resources
    gio::resources_register_include!("dewduct.gresource")
        .expect("Failed to register resources.");
    glib::BoxedAnyObject::static_type();
    yt_item_list::DewYtItem::static_type();
    // let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/dewduct.gresource")
    //     .expect("Could not load resources");
    // gio::resources_register(&resources);

    // Create a new GtkApplication. The application manages our main loop,
    // application windows, integration with the window manager/compositor, and
    // desktop features such as file opening and single-instance applications.
    let app = DewDuctApplication::new(
        "null.daknig.DewDuct",
        &gio::ApplicationFlags::empty(),
    );

    // Run the application. This function will block until the application
    // exits. Upon return, we have our exit code to return to the shell. (This
    // is the code you see when you do `echo $?` after running a command in a
    // terminal.
    app.run()
}
