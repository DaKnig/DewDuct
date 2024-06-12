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

use std::fs::{read, File};
use std::path::PathBuf;

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use gio::ListStore;
use glib::{g_debug, g_warning, user_data_dir};
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use anyhow::Context;
use futures::StreamExt;
use invidious::ClientAsyncTrait;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::window::DewDuctWindow;
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
            glib::spawn_future_local(glib::clone!(@weak self as page =>
                 async move {page.load_state().await}));
        }
    }
    impl WidgetImpl for DewSubscriptionsPage {}
    impl BoxImpl for DewSubscriptionsPage {}

    #[gtk::template_callbacks]
    impl DewSubscriptionsPage {
        pub fn connect_subs_changed(
            &self,
            f: impl Fn(&ListStore) + 'static,
        ) -> glib::signal::SignalHandlerId {
            self.subs_list.connect_items_changed(f)
        }
        fn async_invidious_client(&self) -> invidious::ClientAsync {
            self.obj()
                .root()
                .and_downcast_ref::<crate::window::DewDuctWindow>()
                .unwrap()
                .async_invidious_client()
        }
        fn window(&self) -> DewDuctWindow {
            self.obj().root().and_downcast().unwrap()
        }
        fn store_state(&self) {
            let path = self.subs_file_path();
            let file: File = File::create(&path)
                .or_else(|e| {
                    if let Some(parent) = path.parent() {
                        std::fs::create_dir_all(parent)?;
                        File::create(&path)
                    } else {
                        Err(e)
                    }
                })
                .with_context(|| {
                    format!(
                        "unable to create file {} to store state",
                        &path.display()
                    )
                })
                .unwrap();
            let subs_vec = self.subs_list.get_vec();
            let subscription_list_serialization = SubscriptionList {
                subscriptions: subs_vec
                    .into_iter()
                    .map(|x| x.into())
                    .collect(),
            };
            serde_json::to_writer(file, &subscription_list_serialization)
                .with_context(|| {
                    format!(
                        "unable to write to file {} to store state",
                        &path.display()
                    )
                })
                .unwrap();
        }
        pub fn del_subscription(&self, id: String) {
            self.subs_list.del_item_with_id(id);
            self.store_state();
        }
        pub async fn add_subscription(
            &self,
            id: String,
        ) -> anyhow::Result<()> {
            let v = self.subs_list.get_vec();
            // if is subscribed already, do nothing
            if v.into_iter()
                .any(|vid| vid.imp().id.borrow().as_str() == id)
            {
                return Ok(());
            }

            let item =
                self.async_invidious_client().channel(&id, None).await;

            match item {
                Ok(item) => {
                    self.subs_list.insert_sorted(&item.into(), |a, b| {
                        a.title().cmp(&b.title())
                    });
                    self.store_state();
                    Ok(())
                }
                Err(e) => {
                    let e_str =
                        format!("Unable to subscribe to {}: {}", &id, e);
                    g_warning!("DewSubscriptionsPage", "{}", e_str);
                    anyhow::bail!(e_str);
                }
            }
        }
        #[template_callback]
        async fn import_newpipe_subs(&self) {
            let json_filter = gtk::FileFilter::new();
            json_filter.add_suffix("json");
            let filters = gio::ListStore::from_iter([json_filter; 1]);
            let dialog = gtk::FileDialog::builder()
                .filters(&filters)
                .title("Import NewPipe data")
                .build();
            let dialog_res = dialog.open_future(None::<&gtk::Window>).await;
            match dialog_res {
                Ok(x) if x.path().is_some() => {
                    let path = x.path().unwrap();
                    self.load_newpipe_subs_from_file(path).await;
                }
                Err(e) if e.matches(gtk::DialogError::Dismissed) => {
                    g_debug!("DewSubscriptionsPage", "{}", e.message())
                }
                Err(e) => {
                    g_warning!("DewSubscriptionsPage", "{}", e.message())
                }
                Ok(_) => g_warning!(
                    "DewSubscriptionsPage",
                    "invalid path selected"
                ),
            }
        }
        fn subs_file_path(&self) -> PathBuf {
            lazy_static! {
                static ref SUBS: PathBuf =
                    user_data_dir().join("DewDuct/").join("subs.json");
            }
            SUBS.to_path_buf()
        }
        #[template_callback]
        async fn load_state(&self) {
            g_warning!("DewSubscriptionsPage", "loading state");
            let path = self.subs_file_path();
            dbg!(path.display());
            self.load_newpipe_subs_from_file(path).await;
        }
        async fn load_newpipe_subs_from_file(&self, file: PathBuf) {
            fn sync_import_subs(file: PathBuf) -> Vec<String> {
                // - get info from subs file
                let contents = read(file).unwrap_or_default();
                let subs: Vec<_> = serde_json::from_slice(&contents)
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
                        if sub.service_id != 0 {
                            return None;
                        }
                        let url = sub.url;
                        let stripped = url.strip_prefix(
                            "https://www.youtube.com/channel/",
                        );
                        if url.starts_with("https://www.youtube.com/user/")
                        {
                            g_warning!(
                                "DewSubscriptionPage",
                                "problem with importing channel {}: \
                                 can't use /user/ api!",
                                url
                            );
                        }
                        stripped.map(|id| id.into())
                    })
                    .collect();
                subs
            }
            let fetch_file = move || sync_import_subs(file);
            let subs: Vec<String> = self
                .window()
                .spawn_blocking(fetch_file)
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
                    async move { invidious.channel(&id, None).await }
                })
                .buffer_unordered(10)
                .collect()
                .await;
            // if error fetching, then skip
            let channels =
                channels_or_errors.into_iter().filter_map(|x| x.ok());
            let mut dew_yt_items: Vec<DewYtItem> =
                channels.map(|chan| chan.into()).collect();
            // - display it
            // for dedup purposes
            let subs = self.subs_list.get_vec();
            dew_yt_items.extend(subs);
            dew_yt_items.sort_unstable_by_key(|item| item.title());
            self.subs_list.set_from_vec(dew_yt_items);
            self.store_state();
        }
    }

    #[derive(Deserialize, Serialize)]
    pub(super) struct SubscriptionList {
        subscriptions: Vec<Subscription>,
    }

    #[derive(Deserialize, Serialize)]
    pub(super) struct Subscription {
        url: String,
        name: String,
        service_id: u8,
    }
    impl From<DewYtItem> for Subscription {
        fn from(item: DewYtItem) -> Self {
            let mut url = item.id();
            url.insert_str(0, "https://www.youtube.com/channel/");

            Self {
                url,
                name: item.title(),
                service_id: 0,
            }
        }
    }
}

glib::wrapper! {
    pub struct DewSubscriptionsPage(ObjectSubclass<imp::DewSubscriptionsPage>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap;
}
