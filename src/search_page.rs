/* search_page.rs
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
use gtk::SearchEntry;
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use html_escape::decode_html_entities;
use invidious::hidden::SearchItem;
use invidious::ClientSyncTrait;
use urlencoding::encode;

use crate::video_row::DewVideoRow;
use crate::yt_item_list::DewYtItemList;

#[allow(unused_imports)]
use crate::util::*;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/search_page.ui")]
    pub struct DewSearchPage {
        #[template_child]
        pub(super) search_bar: TemplateChild<gtk::SearchBar>,
        #[template_child]
        pub(super) search_entry: TemplateChild<SearchEntry>,
        #[template_child]
        not_found_page: TemplateChild<adw::StatusPage>,
        #[template_child]
        pub(super) results_page: TemplateChild<DewYtItemList>,
        #[template_child]
        search_stack: TemplateChild<gtk::Stack>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewSearchPage {
        const NAME: &'static str = "DewSearchPage";
        type Type = super::DewSearchPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
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
        }
    }
    impl WidgetImpl for DewSearchPage {}
    impl BoxImpl for DewSearchPage {}

    #[gtk::template_callbacks]
    impl DewSearchPage {
        #[template_callback]
        pub(crate) fn search_started(&self) {
            // glib::g_warning!("Dew", "search_started");
        }
        #[template_callback]
        pub(crate) fn stop_search(&self) {
            // glib::g_warning!("Dew", "stop_search");
        }
        #[template_callback]
        pub(crate) async fn search_activate(&self, entry: &SearchEntry) {
            glib::g_warning!("Dew", "search activated");
            let query = entry.text();
            g_warning!("DewSearchPage", "searching {}...", query);

            // qeury for search results
            let query_transformed = format!("q={}", encode(&query));
            let client = self.obj().invidious_client();
            let search_results = tokio::task::spawn_blocking(move || {
                match client.search(Some(&query_transformed)) {
                    Ok(search) => search.items,
                    Err(err) => {
                        g_warning!(
                            "Dew",
                            "instance {}; no results: {:?}",
                            &client.instance,
                            err
                        );
                        vec![]
                    }
                }
            })
            .await
            .unwrap_or(vec![]);

            // if zero, show the "not found" page
            if search_results.is_empty() {
                self.search_stack.set_visible_child(&*self.not_found_page);
                return;
            } else {
                self.search_stack.set_visible_child(&*self.results_page)
            }

            {
                // actually putting in the items
                let search_results: Vec<_> = search_results
                    .into_iter()
                    .filter(|x| {
                        // we only support these types for now...
                        matches!(x, SearchItem::Channel { .. })
                            || matches!(x, SearchItem::Video { .. })
                    })
                    .map(|x| x.into())
                    .collect();

                self.results_page.set_from_vec(search_results);
            }
        }
        #[template_callback]
        pub(crate) async fn search_changed(&self, entry: &SearchEntry) {
            // glib::g_warning!("Dew", "search_changed");
            let client = self.obj().invidious_client();

            // get the dang results, errors = no results
            let query = entry.text().to_string().to_owned();
            if query.is_empty() {
                return;
            }

            // encode to make utf8 work
            let query_transformed = format!("q={}", encode(&query));
            let search_suggestions =
                tokio::task::spawn_blocking(move || {
                    match client
                        .search_suggestions(Some(&query_transformed))
                    {
                        Ok(search) => {
                            g_warning!(
                                "DewSearch",
                                "search.query = {}",
                                decode_html_entities(&search.query)
                            );

                            search.suggestions
                        }
                        Err(err) => {
                            g_warning!(
                                "DewSearch",
                                "no results: {:?}",
                                err
                            );
                            vec![]
                        }
                    }
                });
            let Ok(search_suggestions): Result<Vec<_>, _> =
                search_suggestions.await
            else {
                return;
            };
            if query != entry.text() {
                g_warning!(
                    "DewSearchPage",
                    "query was {}, returning!",
                    &query
                );
                return;
            }
            let search_suggestions: Vec<_> = search_suggestions
                .into_iter()
                .map(|s| decode_html_entities(&s).into_owned())
                .collect();

            // now display suggestions
            let for_display = search_suggestions
                .into_iter()
                .fold("".into(), |a: String, b: String| {
                    a + ", " + b.as_ref()
                });
            glib::g_warning!("DewSearch", "results: {}", for_display);
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
    pub fn search_entry(&self) -> &SearchEntry {
        &self.imp().search_entry
    }
    pub fn invidious_client(&self) -> invidious::ClientSync {
        let window: crate::window::DewDuctWindow =
            self.root().and_downcast().unwrap();
        window.invidious_client()
    }
}
