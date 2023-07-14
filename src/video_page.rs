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
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use invidious::video::Video;

use crate::thumbnail::DewThumbnail;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/video_page.ui")]
    pub struct DewVideoPage {
        // Template widgets
        #[template_child]
        vid_thumbnail: TemplateChild<DewThumbnail>,
        #[template_child]
        title: TemplateChild<gtk::Label>,
        // #[template_child]
        // author_thumb: TemplateChild<gtk::Image>,
        #[template_child]
        author_name: TemplateChild<gtk::Label>,
        #[template_child]
        sub_count: TemplateChild<gtk::Label>,
        // #[template_child]
        // views: TemplateChild<gtk::Label>,
        // #[template_child]
        // likes: TemplateChild<gtk::Label>,
        // #[template_child]
        // bottom_stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        description: TemplateChild<gtk::Label>,
        // #[template_child]
        // bottom_switcher: TemplateChild<adw::ViewSwitcherBar>,
        id: Rc<RefCell<Option<String>>>,
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
            self.id.take();
        }
    }
    impl WidgetImpl for DewVideoPage {}
    impl BoxImpl for DewVideoPage {}

    impl DewVideoPage {
        pub(crate) async fn set_vid(&self, new_vid: Video) {
            if !self
                .id
                .borrow()
                .as_ref()
                .is_some_and(|id| id == &new_vid.id)
            {
                println!(
                    "was {:?} became {:?}",
                    self.id.borrow().as_ref(),
                    Some(&new_vid.id)
                );

                let Video {
                    id,
                    thumbnails,
                    length,
                    author,
                    title,
                    description,
                    ..
                } = new_vid;

                self.author_name.set_text(&author);
                self.title.set_text(&title);
                self.sub_count.set_text(&format!(
                    "{} subscribers",
                    new_vid.sub_count_text
                ));
                // self.description.set_markup(&new_vid.description_html);
                self.description.set_text(&description);
                *self.id.borrow_mut() = Some(id.clone());
                self.vid_thumbnail
                    .update_from_params(
                        id.clone(),
                        &thumbnails,
                        length,
                        0.0f64,
                    )
                    .await
                    .unwrap_or_else(|err| {
                        println!(
                            "can't open video {} in the VideoPage: {}",
                            id, err
                        )
                    });
            } else {
                println!("clicked on the same vid...")
            }
            self.obj().set_visible(true);
        }

        pub(crate) fn reset_vid(&self) {
            println!("was Some({:?}) became None", self.id.take());
            *self.id.borrow_mut() = None;
            self.obj().set_visible(false);
            todo!() // reset the video and all stuffs
        }
    }
}

glib::wrapper! {
    pub struct DewVideoPage(ObjectSubclass<imp::DewVideoPage>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap;
}
