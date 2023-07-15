/* update_page.rs
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
use gtk::{gio, glib, StringList};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use invidious::ClientAsyncTrait;

use crate::video_row::DewVideoRow;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/update_page.ui")]
    pub struct DewUpdatePage {
        // Template widgets
        #[template_child]
        update_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub(crate) search_button: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        vid_factory: TemplateChild<gtk::SignalListItemFactory>,
        #[template_child]
        new_vids: TemplateChild<gtk::ListView>,

        new_vids_store: StringList,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewUpdatePage {
        const NAME: &'static str = "DewUpdatePage";
        type Type = super::DewUpdatePage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewUpdatePage {
        fn constructed(&self) {
            self.new_vids.set_model(Some(&gtk::NoSelection::new(Some(
                self.new_vids_store.clone(),
            ))));
            let new_vids_store = self.new_vids_store.clone();
            self.new_vids
                .connect_activate(move |list_view, index: u32| {
                    let Some(id) = new_vids_store.string(index)
                                   else {return};
                    let id: String = id.to_string();
                    list_view
                        .activate_action(
                            "win.play",
                            Some(&Some(id).to_variant()),
                        )
                        .expect("the action win.play does not exist");
                });
        }
    }
    impl WidgetImpl for DewUpdatePage {}
    impl BoxImpl for DewUpdatePage {}

    #[gtk::template_callbacks]
    impl DewUpdatePage {
        fn invidious_client(&self) -> invidious::ClientAsync {
            self.obj()
                .root()
                .and_downcast::<crate::window::DewDuctWindow>()
                .unwrap()
                .invidious_client()
        }
        #[template_callback]
        async fn update_vids(&self) {
            let invidious = self.invidious_client();

            let popular_items = invidious.popular(None).await;
            match popular_items {
                Ok(popular) => {
                    let mut store = self.new_vids_store.clone();
                    let vids = popular.items.into_iter().map(|x| x.id);

                    let n_items = store.n_items();
                    store.splice(0, n_items, &[]); // empty
                    store.extend(vids);
                    self.update_button.remove_css_class("error");
                }
                Err(err) => {
                    eprintln!("{}", err);
                    self.update_button.add_css_class("error");
                }
            }
        }

        #[template_callback]
        fn setup_vid_widget(&self, list_item: gtk::ListItem) {
            let row = DewVideoRow::new();
            list_item.set_child(Some(&row));
        }

        #[template_callback]
        async fn bind_vid_widget(&self, list_item: gtk::ListItem) {
            let vid_id: glib::GString = list_item
                .item()
                .and_downcast::<gtk::StringObject>()
                .expect("The item has to be an `StringObject`.")
                .string();
            let row: DewVideoRow = list_item
                .child()
                .and_downcast()
                .expect("The item needs to be a DewVideoRow");

            let invidious = self.invidious_client();

            let vid_data = invidious.video(&vid_id, None).await;

            if let Err(err) = vid_data {
                eprintln!("can't fetch the data of vid ID {vid_id}: {err}");
                return;
            }

            let invidious::video::Video {
                id,
                views,
                author,
                title,
                published,
                length,
                thumbnails,
                ..
            } = vid_data.unwrap();

            row.set_from_params(
                id.clone(),
                views,
                author.clone(),
                title.clone(),
                published,
                length,
                &thumbnails,
            )
            .await
            .unwrap_or_else(|err| {
                println!("error loading video info: {}", err);
            })
        }
    }
}

glib::wrapper! {
    pub struct DewUpdatePage(ObjectSubclass<imp::DewUpdatePage>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap;
}
