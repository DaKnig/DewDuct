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

use std::{
    cell::RefCell,
    process::{Child, Command},
    rc::Rc,
};

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use glib::g_warning;
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use invidious::video::Video;

use crate::format_semi_engineering;
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
        #[template_child]
        views: TemplateChild<gtk::Label>,
        #[template_child]
        likes: TemplateChild<gtk::Label>,
        // #[template_child]
        // bottom_stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        description: TemplateChild<gtk::Label>,
        // #[template_child]
        // bottom_switcher: TemplateChild<adw::ViewSwitcherBar>,
        vid: Rc<RefCell<Option<Video>>>,
        id: Rc<RefCell<Option<String>>>,
        mpv_child: Rc<RefCell<Option<Child>>>,
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
            {
                // let's spawn mpv when thumbnail is clicked!
                let click = gtk::GestureClick::new();

                let page = self.obj().clone();
                click.connect_pressed(move |_, _n, _x, _y| {
                    page.imp().play_mpv()
                });

                self.vid_thumbnail.add_controller(click);
            }
        }
    }
    impl WidgetImpl for DewVideoPage {}
    impl BoxImpl for DewVideoPage {}

    impl DewVideoPage {
        pub(crate) fn play_mpv(&self) {
            let id = self.id.clone();
            let mpv_child = self.mpv_child.clone();

            let tmp = id.borrow();
            let Some(id) = tmp.as_ref() else {return};

            let url = format!("https://youtube.com/watch?v={}", id);
            let mut mpv = Command::new("mpv");
            mpv.arg(url).arg("--ytdl-format=best[height<=480]");
            g_warning!(
                "Dew",
                "running... {:?} {:?}",
                mpv.get_program(),
                mpv.get_args().collect::<Vec<_>>()
            );

            // spawn child process
            let mpv_process = mpv.spawn().expect("mpv not found");
            let prev_mpv = mpv_child.replace(Some(mpv_process));

            // if there was already a mpv instance running...
            if let Some(mut prev_mpv) = prev_mpv {
                prev_mpv.kill().expect("error killing it");
            }
        }

        pub(crate) async fn set_vid(&self, new_vid: Video) {
            if !self
                .id
                .borrow()
                .as_ref()
                .is_some_and(|id| id == &new_vid.id)
            {
                g_warning!(
                    "DewVideoPage",
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
                    likes,
                    views,
                    sub_count_text,
                    ..
                } = &new_vid;

                self.author_name.set_text(author);
                self.title.set_text(title);
                self.likes.set_text(
                    &(format_semi_engineering(*likes as f32) + " likes"),
                );
                self.views.set_text(
                    &(format_semi_engineering(*views as f32) + " views"),
                );
                self.sub_count
                    .set_text(&format!("{} subscribers", sub_count_text));
                // self.description.set_markup(&new_vid.description_html);
                self.description.set_text(description);
                *self.id.borrow_mut() = Some(id.clone());

                self.vid_thumbnail
                    .update_from_params(
                        id.clone(),
                        thumbnails.iter().map(|x| x.clone().into()),
                        *length,
                        0.0f64,
                    )
                    .await
                    .unwrap_or_else(|err| {
                        g_warning!(
                            "DewVideoPage",
                            "can't open video {} in the VideoPage: {}",
                            id,
                            err
                        )
                    });
                self.vid.replace(Some(new_vid));
            } else {
                g_warning!("DewVideoPage", "clicked on the same vid...")
            }
            self.obj().set_visible(true);
        }

        pub(crate) fn reset_vid(&self) {
            g_warning!(
                "DewVideoPage",
                "was Some({:?}) became None",
                self.id.take()
            );
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
