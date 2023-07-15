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
use invidious::hidden::SearchItem;
use invidious::ClientAsyncTrait;
use urlencoding::encode;

use crate::video_row::DewVideoRow;

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
        results_page: TemplateChild<gtk::ScrolledWindow>,
        #[template_child]
        search_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        search_result_list: TemplateChild<gio::ListStore>,

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
        pub(crate) async fn search_activate(&self, entry: &SearchEntry) {
            glib::g_warning!("Dew", "search activated");
            let query = entry.text();
            eprintln!("searching {}...", query);

            // qeury for search results
            let query_transformed = format!("q={}", encode(&query));
            let client = &self.invidious_client.borrow().clone();
            let search_results: Vec<SearchItem> =
                match client.search(Some(&query_transformed)).await {
                    Ok(search) => search.items,
                    Err(err) => {
                        glib::g_warning!("Dew", "no results: {:?}", err);
                        vec![]
                    }
                };

            // if zero, show the "not found" page
            if search_results.is_empty() {
                self.search_stack.set_visible_child(&*self.not_found_page);
                return;
            } else {
                self.search_stack.set_visible_child(&*self.results_page)
            }

            {
                // actually putting in the items
                self.search_result_list.remove_all();
                let search_results: Vec<_> = search_results
                    .into_iter()
                    .filter(|x| matches!(x, SearchItem::Video { .. }))
                    .map(glib::BoxedAnyObject::new)
                    .collect();

                self.search_result_list.extend_from_slice(&search_results);
            }
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

            // now display suggestions
            let for_display = search_suggestions
                .into_iter()
                .fold("".into(), |a: String, b: String| {
                    a + ", " + b.as_ref()
                });
            glib::g_warning!("Dew", "{}", for_display);
        }

        #[template_callback(function)]
        fn setup_row(list_item: gtk::ListItem) {
            let row = DewVideoRow::new();
            list_item.set_child(Some(&row));
        }

        #[template_callback(function)]
        async fn bind_row(list_item: gtk::ListItem) {
            let boxed: glib::BoxedAnyObject = list_item
                .item()
                .and_downcast()
                .expect("The item has to be an `BoxedAnyObject`");
            // get_type_of_value(&boxed);
            let search_item: std::cell::Ref<SearchItem> =
                boxed.try_borrow().unwrap();
            let SearchItem::Video{title, author, views, published,
				  id, length, thumbnails, ..} =
		&*search_item
	    else {
		todo!()
	    };

            let row: DewVideoRow = list_item
                .child()
                .and_downcast()
                .expect("The item needs to be a DewVideoRow");

            row.set_from_params(
                id.clone(),
                *views,
                author.clone(),
                title.clone(),
                *published,
                *length as u32,
                thumbnails,
            )
            .await
            .unwrap_or_else(|err| {
                glib::g_warning!("Dew", "{}", err);
            });
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
