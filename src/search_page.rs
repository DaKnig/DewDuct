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

use std::{cell::RefCell, rc::Rc};

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use gtk::SearchEntry;
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use html_escape::decode_html_entities;
use invidious::ClientAsyncTrait;
use urlencoding::encode;

use crate::video_row::DewVideoRow;

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
        results_page: TemplateChild<gtk::ScrolledWindow>,
        #[template_child]
        search_stack: TemplateChild<gtk::Stack>,

        invidious_client: Rc<RefCell<invidious::ClientAsync>>,
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
        pub(crate) async fn search_activate(&self, _entry: &SearchEntry) {
            glib::g_warning!("Dew", "search activated");
        }
        #[template_callback]
        pub(crate) async fn search_changed(&self, entry: &SearchEntry) {
            // glib::g_warning!("Dew", "search_changed");
            let client = self.invidious_client.borrow().clone();

            // get the dang results, errors = no results
            let query = &*entry.text();
            if query.is_empty() {
                return;
            }

            // encode to make utf8 work
            let query_transformed = format!("q={}", encode(query));
            let search_suggestions: Vec<_> = match client
                .search_suggestions(Some(&query_transformed))
                .await
            {
                Ok(search) => {
                    println!(
                        "search.query = {}",
                        decode_html_entities(&search.query)
                    );
                    if query != entry.text() {
                        println!("query was {}, returning!", &query);
                        return;
                    }
                    search.suggestions
                }
                Err(err) => {
                    glib::g_warning!("Dew", "no results: {:?}", err);
                    vec![]
                }
            }
            .into_iter()
            .map(|s| decode_html_entities(&s).into_owned())
            .collect();

            // if zero, show the "not found" page
            let page_to_show: &gtk::Widget = match search_suggestions.len()
            {
                0 => self.not_found_page.upcast_ref(),
                _ => self.results_page.upcast_ref(),
            };
            self.search_stack.set_visible_child(page_to_show);

            // now display suggestions
            let for_display = search_suggestions
                .into_iter()
                .fold("".into(), |a: String, b: String| {
                    a + ", " + b.as_ref()
                });
            glib::g_warning!("Dew", "{}", for_display);
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
}
