/* popular_page.rs
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
use glib::{g_warning, MainContext, PRIORITY_LOW};
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use invidious::ClientSyncTrait;

use crate::yt_item_list::*;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/popular_page.ui")]
    pub struct DewPopularPage {
        // Template widgets
        #[template_child]
        update_button: TemplateChild<gtk::Button>,
        #[template_child]
        vid_list: TemplateChild<DewYtItemList>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewPopularPage {
        const NAME: &'static str = "DewPopularPage";
        type Type = super::DewPopularPage;
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

    impl ObjectImpl for DewPopularPage {
        fn constructed(&self) {
            self.parent_constructed();

            let page = self.obj().clone();
            MainContext::default()
                .spawn_local_with_priority(PRIORITY_LOW, async move {
                    page.imp().update_vids().await
                });
        }
    }
    impl WidgetImpl for DewPopularPage {}
    impl BoxImpl for DewPopularPage {}

    #[gtk::template_callbacks]
    impl DewPopularPage {
        fn invidious_client(&self) -> invidious::ClientSync {
            self.obj()
                .root()
                .and_downcast::<crate::window::DewDuctWindow>()
                .unwrap()
                .invidious_client()
        }
        #[template_callback]
        async fn update_vids(&self) {
            let invidious = self.invidious_client();

            self.update_button.set_sensitive(false);

            let Ok(Some(popular)) =
                tokio::task::spawn_blocking(move || {
                    match invidious.popular(None) {
                        Err(err) => {
                            g_warning!("DewPopularPage",
                                       "cant update page: {:#?}", err);
                            None
                        },
                        Ok(ok) => Some(ok),
                    }
                })
                .await
            else {
                self.update_button.add_css_class("error");
		self.update_button.set_sensitive(true);
                return
            };
            self.update_button.remove_css_class("error");
            self.update_button.set_sensitive(true);

            // let mut store = self.new_vids_store.clone();
            let vids =
                popular.items.into_iter().map(|x| x.into()).collect();

            self.vid_list.set_from_vec(vids);
            // let n_items = store.n_items();
            // store.splice(0, n_items, &[]); // empty
            // store.extend(vids);
        }
    }
}

glib::wrapper! {
    pub struct DewPopularPage(ObjectSubclass<imp::DewPopularPage>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap;
}
