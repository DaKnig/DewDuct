/* subscriptions_page.rs
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

use std::fs::read;
use std::io::BufReader;
use std::path::PathBuf;

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use glib::{g_warning, user_cache_dir, MainContext, PRIORITY_LOW};
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use invidious::ClientSyncTrait;

use lazy_static::lazy_static;

use crate::yt_item_list::*;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/subscriptions_page.ui")]
    pub struct DewSubscriptionsPage {
        // Template widgets
        #[template_child]
        subs_list: TemplateChild<DewYtItemList>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewSubscriptionsPage {
        const NAME: &'static str = "DewSubscriptionsPage";
        type Type = super::DewSubscriptionsPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            DewYtItemList::ensure_type();
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewSubscriptionsPage {
        fn constructed(&self) {
            self.parent_constructed();

            let page = self.obj().clone();
            MainContext::default()
                .spawn_local(async move { page.imp().read_subs().await });
        }
    }
    impl WidgetImpl for DewSubscriptionsPage {}
    impl BoxImpl for DewSubscriptionsPage {}

    #[gtk::template_callbacks]
    impl DewSubscriptionsPage {
        fn invidious_client(&self) -> invidious::ClientSync {
            self.obj()
                .root()
                .and_downcast::<crate::window::DewDuctWindow>()
                .unwrap()
                .invidious_client()
        }
        async fn read_subs(&self) {
            let invidious = self.invidious_client();

            lazy_static! {
                static ref CACHE: PathBuf =
                    user_cache_dir().join("dewduct/").join("subs.toml");
            }

            dbg!(CACHE.display());

            tokio::task::spawn_blocking(move || {
                // PLAN:
                // - get info from cache dir
                let contents =
                    std::fs::read(CACHE.as_path()).unwrap_or_default();
		
                // - - if none exists, create and store
                // - display it
            })
            .await
            .unwrap_or_else(|err| {
                g_warning!(
                    "DewSubscriptions",
                    "unable to open `{}`: {}",
                    // cache.display(),
                    "",
                    err
                );
            });
        }
    }
}

glib::wrapper! {
    pub struct DewSubscriptionsPage(ObjectSubclass<imp::DewSubscriptionsPage>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap;
}
