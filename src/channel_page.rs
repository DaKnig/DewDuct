/* channel_page.rs
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

use std::cell::RefCell;

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use glib::g_warning;
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use invidious::{channel::Channel, ClientAsyncTrait};

use crate::{
    window::DewDuctWindow,
    yt_item_list::{DewYtItem, DewYtItemList},
};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/channel_page.ui")]
    pub struct DewChannelPage {
        // Template widgets
        #[template_child]
        pub(super) vid_list: TemplateChild<DewYtItemList>,

        pub(super) channel: RefCell<Option<Channel>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewChannelPage {
        const NAME: &'static str = "DewChannelPage";
        type Type = super::DewChannelPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewChannelPage {}
    impl WidgetImpl for DewChannelPage {}
    impl BoxImpl for DewChannelPage {}

    // #[gtk::template_callbacks]
    impl DewChannelPage {
        pub fn set_channel(&self, channel: Channel) {
            let header = DewYtItem::header(&channel);

            self.vid_list.set_from_vec(
                Some(header)
                    .into_iter()
                    .chain(
                        channel
                            .lastest_videos
                            .iter()
                            .map(|x: &invidious::CommonVideo| x.into()),
                    )
                    .collect::<Vec<_>>(),
            );
            g_warning!("DewChannelPage", "changed to id {}", &channel.id);
            self.channel.replace(Some(channel));
        }
    }
}

glib::wrapper! {
    pub struct DewChannelPage(ObjectSubclass<imp::DewChannelPage>)
        @extends gtk::Widget, gtk::Box,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl DewChannelPage {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_channel(&self, channel: Channel) {
        self.imp().set_channel(channel)
    }

    fn window(&self) -> DewDuctWindow {
        self.root().and_downcast().unwrap()
    }

    pub async fn set_channel_id(&self, id: &str) {
        let id = id.to_owned();
        let invidious = self.async_invidious_client();
        let Ok(channel) = self
            .window()
            .spawn(async move {
                invidious.channel(&id, None).await.map_err(|err| {
                    g_warning!(
                        "DewChannelPage",
                        "cant load {id}: {err:#?}"
                    );
                    g_warning!(
                        "DewChannelPage",
                        "the instance used was {}",
                        invidious.instance
                    );
                })
            })
            .await
            .unwrap()
        else {
            // if we get fetch error
            return;
        };

        self.set_channel(channel);
    }

    pub fn async_invidious_client(&self) -> invidious::ClientAsync {
        let window: crate::window::DewDuctWindow =
            self.root().and_downcast().unwrap();
        window.async_invidious_client()
    }
}

impl Default for DewChannelPage {
    fn default() -> Self {
        Self::new()
    }
}
