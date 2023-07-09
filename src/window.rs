/* window.rs
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

use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use glib::{clone, GString, Variant};
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use crate::search_page::DewSearchPage;
use crate::update_page::DewUpdatePage;
use crate::video_page::DewVideoPage;

use invidious::{ClientAsync, ClientAsyncTrait};

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/window.ui")]
    pub struct DewDuctWindow {
        // Template widgets
        #[template_child]
        video_page: TemplateChild<DewVideoPage>,
        #[template_child]
        search_page: TemplateChild<DewSearchPage>,
        #[template_child]
        screen_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        update_page: TemplateChild<DewUpdatePage>,
        #[template_child]
        search_entry: TemplateChild<gtk::SearchEntry>,
        #[template_child]
        search_bar: TemplateChild<gtk::SearchBar>,

        last_visible_page: Rc<RefCell<Option<GString>>>,
        invidious: Rc<RefCell<ClientAsync>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewDuctWindow {
        const NAME: &'static str = "DewDuctWindow";
        type Type = super::DewDuctWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            DewUpdatePage::ensure_type();
            DewVideoPage::ensure_type();
            DewSearchPage::ensure_type();
            klass.bind_template();
            klass.bind_template_callbacks();
            klass.install_action("win.back", None, Self::Type::back);
            klass.install_action_async("win.play", None, Self::Type::play);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewDuctWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.search_bar.connect_entry(&*self.search_entry);
            self.search_bar.connect_search_mode_enabled_notify(
                clone!(@weak self as win =>
                    move |_| win.toggle_search_mode()
                ),
            );
        }
    }
    impl WidgetImpl for DewDuctWindow {}
    impl WindowImpl for DewDuctWindow {}
    impl ApplicationWindowImpl for DewDuctWindow {}
    impl AdwApplicationWindowImpl for DewDuctWindow {}

    #[gtk::template_callbacks]
    impl DewDuctWindow {
        #[template_callback]
        pub(super) fn toggle_search_mode(&self) {
            let search_bar = &self.search_bar;
            let screen_stack = &self.screen_stack;
            let last_visible_page = &self.last_visible_page;
            let transition_to = if search_bar.is_search_mode() {
                last_visible_page
                    .replace(screen_stack.visible_child_name());
                "search_page".into()
            } else {
                last_visible_page
                    .take()
                    .unwrap_or("normal_view_page".into())
            };
            self.screen_stack.set_visible_child_name(&transition_to);
        }
        pub(super) fn back(&self) {
            self.screen_stack.set_visible_child_full(
                "normal_view_page",
                gtk::StackTransitionType::SlideDown,
            );
            self.search_bar.set_search_mode(false);
            self.video_page.set_visible(false);
        }

        pub(super) async fn play(&self, _: String, param: Option<Variant>) {
            // Get param
            let parameter: Option<String> = param
                .expect("Could not get parameter.")
                .get()
                .expect("not a Option<String>!");

            // Update label with new state
            let Some(id) = parameter else {
                println!("stop playing...");
                self.video_page.imp().reset_vid();
                return
            };

            let vid_page = self.video_page.get();
            let invidious = self.invidious.borrow().clone();

            match invidious.video(&id, None).await {
                Ok(vid) => {
                    vid_page.imp().set_vid(vid).await;
                    self.screen_stack.set_visible_child_full(
                        "video_page",
                        gtk::StackTransitionType::SlideUp,
                    );
                }
                Err(err) => {
                    println!("cant load {id}: {err}");
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct DewDuctWindow(ObjectSubclass<imp::DewDuctWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl DewDuctWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
    pub async fn play(self, action_name: String, param: Option<Variant>) {
        self.imp().play(action_name, param).await;
    }
    pub fn back(&self, _: &str, _: Option<&Variant>) {
        self.imp().back()
    }
}
