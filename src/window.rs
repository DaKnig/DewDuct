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
use gio::SimpleAction;
use glib::{clone, MainContext};
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use crate::cache::DewCache;
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
        #[template_child(id = "screen-stack")]
        screen_stack: TemplateChild<gtk::Stack>,
        // #[template_child(id = "view-stack")]
        // pub view_stack: TemplateChild<adw::ViewStack>,
        cache: Rc<RefCell<DewCache>>,
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
            klass.bind_template();
            // klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewDuctWindow {
        fn constructed(&self) {
            self.parent_constructed();

            // Add action "player"
            let action_play = SimpleAction::new_stateful(
                "play",
                Some(&Option::<String>::static_variant_type()),
                None::<String>.to_variant(),
            );

            action_play.connect_activate(
                clone!(@weak self as win => move |action, param| {
                    // Get state
                    let state: Option<String> = action
                        .state()
                        .expect("Could not get state.")
                        .get()
                        .expect("not a Option<String>!");

                    // Get param
                    let parameter: Option<String> = param
                        .expect("Could not get parameter.")
                        .get()
                        .expect("not a Option<String>!");

                    if parameter == state {
                        println!("clicked on the same vid...");
                        return
                    } else {
                        println!("was {state:?} became {parameter:?}");
                    }
                    // Increase state by parameter and save state
                    // state += parameter;
                    action.set_state(parameter.to_variant());

                    // Update label with new state

                    let Some(id) = parameter else {
                        println!("stop playing...");
			win.video_page.imp().reset_vid();
                        return
                    };

                    // let inv = invidious.borrow();

                    // vid_page.imp().set_vid(cache, vid);
                    MainContext::default().spawn_local(async move {
                        let cache = win.cache.borrow();
                        let vid_page = win.video_page.get();
                        let invidious = win.invidious.borrow().clone();

                        match invidious.video(&id, None).await {
                            Ok(vid) => {
                                vid_page.imp().set_vid(&cache, vid).await;
				win.screen_stack.set_visible_child_full(
				    "video_page",
				    gtk::StackTransitionType::SlideUp
				);
                            },
                            Err(err) => {
                                println!("cant load {id}: {err}");
                            }
                        }
                    });

                }),
            );
            self.obj().add_action(&action_play);
        }
    }
    impl WidgetImpl for DewDuctWindow {}
    impl WindowImpl for DewDuctWindow {}
    impl ApplicationWindowImpl for DewDuctWindow {}
    impl AdwApplicationWindowImpl for DewDuctWindow {}
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
}
