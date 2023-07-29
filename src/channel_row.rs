/* channel_row.rs
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
use gtk::glib;
#[allow(unused_imports)]
use gtk::{prelude::*, subclass::prelude::*};

use crate::util;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/null/daknig/DewDuct/channel_row.ui")]
    pub struct DewChannelRow {
        #[template_child]
        pub(super) icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub(super) name: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) subs: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DewChannelRow {
        const NAME: &'static str = "DewChannelRow";
        type Type = super::DewChannelRow;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DewChannelRow {}
    impl WidgetImpl for DewChannelRow {}
    impl BoxImpl for DewChannelRow {}
}

glib::wrapper! {
    pub struct DewChannelRow(ObjectSubclass<imp::DewChannelRow>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl DewChannelRow {
    pub fn set_from_params(&self, icon: &str, name: &str, subs: f32) {
        self.imp().icon.set_from_file(Some(icon));
        self.imp().name.set_text(name);
        self.set_subs(subs);
        todo!()
    }
    fn set_subs(&self, subs: f32) {
        self.imp().subs.set_text(
            &(util::format_semi_engineering(subs as f32) + " subscribers"),
        );
    }
}