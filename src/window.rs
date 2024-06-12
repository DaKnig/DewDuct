/* window.rs
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
use std::rc::Rc;

#[allow(unused_imports)]
use adw::{prelude::*, subclass::prelude::*};
use glib::{clone, g_warning, GString, Variant};
use gtk::{gio, glib};
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use crate::{
    channel_page::DewChannelPage, popular_page::DewPopularPage,
    search_page::DewSearchPage, subscriptions_page::DewSubscriptionsPage,
    video_page::DewVideoPage,
};

use invidious::{ClientAsyncTrait, ClientSync};
use tokio::runtime::Runtime;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/window.ui")]
    pub struct DewDuctWindow {
        // Template widgets
        #[template_child]
        video_page: TemplateChild<DewVideoPage>,
        #[template_child]
        channel_page: TemplateChild<DewChannelPage>,
        #[template_child]
        search_page: TemplateChild<DewSearchPage>,
        #[template_child]
        screen_stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        popular_page: TemplateChild<DewPopularPage>,
        #[template_child]
        subscriptions_page: TemplateChild<DewSubscriptionsPage>,
        #[template_child]
        search_bar: TemplateChild<gtk::SearchBar>,
        #[template_child]
        nav_view: TemplateChild<adw::NavigationView>,
        _last_visible_page: Rc<RefCell<Option<GString>>>,
        pub(super) invidious_client: RefCell<ClientSync>,

        pub(super) tokio_rt: RefCell<Option<Runtime>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewDuctWindow {
        const NAME: &'static str = "DewDuctWindow";
        type Type = super::DewDuctWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
            klass.install_action("win.back", None, Self::Type::back);
            klass.install_action_async("win.play", None, Self::Type::play);
            klass.install_action(
                "win.search_started",
                None,
                Self::Type::search_started,
            );
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewDuctWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.invidious_client.borrow_mut().instance =
                "https://invidious.fdn.fr".into();

            self.search_bar.set_key_capture_widget(Some(&*self.obj()));
            self.search_bar
                .connect_entry(self.search_page.search_entry());
            self.search_page
                .search_bar()
                .set_key_capture_widget(Some(&*self.obj()));

            self.search_page
                .search_entry()
                .connect_search_started(clone!(
                    @weak self as win => move |_| {
                    win.search_started()
                }));
            // self.popular_page.imp().search_button.connect_whitespace
            // self.search_bar.connect_search_mode_enabled_notify(
            //     clone!(@weak self as win =>
            //         move |_| win.toggle_search_mode()
            //     ),
            // );
        }
    }
    impl WidgetImpl for DewDuctWindow {}
    impl WindowImpl for DewDuctWindow {}
    impl ApplicationWindowImpl for DewDuctWindow {}
    impl AdwApplicationWindowImpl for DewDuctWindow {}

    #[gtk::template_callbacks]
    impl DewDuctWindow {
        #[template_callback]
        pub(super) fn search_started(&self) {
            // search_bar.set_search_mode(true);
            self.nav_view.push_by_tag("search_page");
        }
        pub(super) fn back(&self) {
            self.screen_stack.set_visible_child_name("updates_page");
            self.nav_view.pop_to_tag("main_view");
            self.search_page.search_entry().emit_stop_search();
        }
        pub(super) async fn show_channel(&self, id: &str) {
            let channel_page = self.channel_page.get();
            channel_page.set_channel_id(id).await;
            channel_page.set_visible(true);
            self.screen_stack.set_visible_child(&channel_page);
            self.nav_view.pop_to_tag("main_view");
            self.search_page.search_entry().emit_stop_search();
        }
        pub(super) async fn play(&self, _: String, param: Option<Variant>) {
            // Get param
            let parameter: Option<String> = param
                .expect("Could not get parameter.")
                .get()
                .expect("not a Option<String>!");

            // Update label with new state
            let Some(id) = parameter else {
                g_warning!("DewWindow", "stop playing...");
                self.video_page.imp().reset_vid();
                return;
            };

            let vid_page = &self.video_page;
            let invidious = self.obj().async_invidious_client();

            let vid = self
                .obj()
                .spawn(async move {
                    invidious.video(&id, None).await.map_err(|err| {
                        g_warning!("DewWindow", "cant load {id}: {err:#?}");
                        g_warning!(
                            "DewWindow",
                            "the instance used was {}",
                            invidious.instance
                        );
                    })
                })
                .await;

            let Ok(Ok(vid)) = vid else { return };

            vid_page.imp().set_vid(vid).await;
            self.screen_stack.set_visible_child_name("video_page");
            self.nav_view.pop_to_tag("main_view");
            self.search_page.search_entry().emit_stop_search();
        }
        pub fn connect_subs_changed(
            &self,
            f: impl Fn(&gio::ListStore) + 'static,
        ) -> glib::signal::SignalHandlerId {
            self.subscriptions_page.imp().connect_subs_changed(f)
        }
        pub async fn subscribe(
            &self,
            channel_id: String,
        ) -> anyhow::Result<()> {
            self.subscriptions_page
                .imp()
                .add_subscription(channel_id)
                .await
        }
        pub fn unsubscribe(&self, channel_id: String) {
            self.subscriptions_page.imp().del_subscription(channel_id)
        }
        pub(super) fn set_tokio_rt(&self, tokio_rt: Option<Runtime>) {
            self.tokio_rt.replace(tokio_rt);
        }
    }
}

glib::wrapper! {
    pub struct DewDuctWindow(ObjectSubclass<imp::DewDuctWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Root;
}

impl DewDuctWindow {
    pub fn new<P: IsA<gtk::Application>>(
        application: &P,
        tokio_rt: Option<Runtime>,
    ) -> Self {
        let obj: Self = glib::Object::builder()
            .property("application", application)
            .build();
        obj.imp().set_tokio_rt(tokio_rt);
        obj
    }
    pub async fn play(self, action_name: String, param: Option<Variant>) {
        self.imp().play(action_name, param).await;
    }
    pub fn back(&self, _: &str, _: Option<&Variant>) {
        self.imp().back()
    }
    pub fn search_started(&self, _: &str, _: Option<&Variant>) {
        self.imp().search_started();
    }
    pub fn invidious_client(&self) -> invidious::ClientSync {
        self.imp().invidious_client.borrow().clone()
    }
    pub fn async_invidious_client(&self) -> invidious::ClientAsync {
        let inv = self.imp().invidious_client.borrow();
        invidious::ClientAsync {
            instance: inv.instance.clone(),
            ..Default::default()
        }
    }
    pub async fn show_channel_yt_item(
        &self,
        channel: &crate::yt_item_list::DewYtItem,
    ) {
        self.imp().show_channel(&channel.id()).await
    }
    pub async fn subscribe(
        &self,
        channel_id: String,
    ) -> anyhow::Result<()> {
        self.imp().subscribe(channel_id).await
    }
    pub fn unsubscribe(&self, channel_id: String) {
        self.imp().unsubscribe(channel_id)
    }
    pub fn connect_subs_changed(
        &self,
        f: impl Fn(&gio::ListStore) + 'static,
    ) -> glib::signal::SignalHandlerId {
        self.imp().connect_subs_changed(f)
    }
    pub(crate) fn spawn_blocking<F, R>(
        &self,
        task: F,
    ) -> tokio::task::JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let rt = self.imp().tokio_rt.borrow();
        rt.as_ref().unwrap().spawn_blocking(task)
    }
    pub(crate) fn spawn<F>(
        &self,
        future: F,
    ) -> tokio::task::JoinHandle<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let rt = self.imp().tokio_rt.borrow();
        rt.as_ref().unwrap().spawn(future)
    }
}
