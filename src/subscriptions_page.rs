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
use std::path::PathBuf;

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use glib::{g_warning, user_data_dir, MainContext};
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use futures::{stream::FuturesUnordered, StreamExt};
use invidious::{channel::Channel, ClientAsyncTrait};
use lazy_static::lazy_static;
use serde::Deserialize;

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
        fn async_invidious_client(&self) -> invidious::ClientAsync {
            self.obj()
                .root()
                .and_downcast::<crate::window::DewDuctWindow>()
                .unwrap()
                .async_invidious_client()
        }
        #[template_callback]
        async fn import_newpipe_subs(&self) {
            lazy_static! {
                static ref SUBS: PathBuf =
                    user_data_dir().join("DewDuct/").join("subs.json");
            }

            dbg!(SUBS.display());

            fn sync_import_subs() -> Vec<String> {
                // - get info from subs file
                let contents = read(SUBS.as_path()).unwrap_or_default();
                let contents: &str =
                    std::str::from_utf8(&contents).unwrap_or_default();
                let subs: Vec<_> = serde_json::from_str(contents)
                    .unwrap_or_else(|_| {
                        g_warning!(
                            "DewSubscriptionsPage",
                            "malformed subscriptions file!"
                        );
                        SubscriptionList {
                            subscriptions: vec![],
                        }
                    })
                    .subscriptions
                    .into_iter()
                    .filter_map(|sub| {
                        dbg!(&sub);
                        if sub.service_id != 0 {
                            return None;
                        }
                        let url = sub.url;
                        let stripped = url.strip_prefix(
                            "https://www.youtube.com/channel/",
                        );
                        stripped.map(|id| id.into())
                    })
                    .collect();
                assert!(subs.len() != 0);
                subs
            }
            let subs: Vec<String> =
                tokio::task::spawn_blocking(sync_import_subs)
                    .await
                    .unwrap_or_else(|err| {
                        g_warning!(
                            "DewSubscriptions",
                            "this should not crash: {}",
                            err
                        );
                        vec![]
                    });
            let invidious =
                std::sync::Arc::new(self.async_invidious_client());
            let channels_or_errors: Vec<_> = futures::stream::iter(subs)
                .map(|id| {
                    let invidious = invidious.clone();
                    async move {
                        println!("fetch {}", &id);
                        invidious.channel(&id, None).await
                    }
                })
                .buffer_unordered(10)
                .collect()
                .await;
            // if error fetching, then skip
            let channels =
                channels_or_errors.into_iter().filter_map(|x| x.ok());
            let dew_yt_items: Vec<DewYtItem> =
                channels.map(|chan| chan.into()).collect();
            // - display it
            self.subs_list.set_from_vec(dew_yt_items);
        }
    }

    #[derive(Deserialize)]
    pub(super) struct SubscriptionList {
        subscriptions: Vec<Subscription>,
    }

    #[derive(Deserialize, Debug)]
    pub(super) struct Subscription {
        url: String,
        name: String,
        service_id: u8,
    }
}

glib::wrapper! {
    pub struct DewSubscriptionsPage(ObjectSubclass<imp::DewSubscriptionsPage>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap;
}
