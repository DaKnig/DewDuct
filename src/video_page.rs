/* video_page.rs
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
use glib::{clone, GString, MainContext};
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use invidious::video::Video;

use crate::cache::DewCache;
use crate::thumbnail::DewThumbnail;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/video_page.ui")]
    pub struct DewVideoPage {
        // Template widgets
        #[template_child]
        vid_thumbnail: TemplateChild<DewThumbnail>,
        // #[template_child]
        // author_thumb: TemplateChild<gtk::Image>,
        // #[template_child]
        // author_name: TemplateChild<gtk::Label>,
        // #[template_child]
        // sub_count: TemplateChild<gtk::Label>,
        // #[template_child]
        // views: TemplateChild<gtk::Label>,
        // #[template_child]
        // likes: TemplateChild<gtk::Label>,
        // #[template_child]
        // bottom_stack: TemplateChild<adw::ViewStack>,
        // #[template_child]
        // description: TemplateChild<gtk::Label>,
        // #[template_child]
        // bottom_switcher: TemplateChild<adw::ViewSwitcherBar>,
        vid: Rc<RefCell<Option<Video>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewVideoPage {
        const NAME: &'static str = "DewVideoPage";
        type Type = super::DewVideoPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            DewThumbnail::ensure_type();
            klass.bind_template();
            // klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewVideoPage {
        fn constructed(&self) {
            self.vid.take();
        }
    }
    impl WidgetImpl for DewVideoPage {}
    impl BoxImpl for DewVideoPage {}

    impl DewVideoPage {
        pub fn set_id(&self, cache: DewCache, new_vid: Video) {
            *self.vid.borrow_mut() = Some(new_vid);
            MainContext::default().spawn_local(
                clone!(@weak self as page => async move {
                    let Some(ref vid) = *page.vid.borrow() else {return};
                    page.vid_thumbnail
                        .update_from_vid_data(cache, vid)
                        .await
                        .unwrap_or_else(|err| {
                            println!(
                                "can't open video in the VideoPage: {}",
                                err
                            )
                        })
                }),
            );

            todo!() // fetch info!
        }

        pub fn reset_id(&self) {
            *self.vid.borrow_mut() = None;
            todo!() // reset the video and all stuffs
        }
    }
}

glib::wrapper! {
    pub struct DewVideoPage(ObjectSubclass<imp::DewVideoPage>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap;
}
