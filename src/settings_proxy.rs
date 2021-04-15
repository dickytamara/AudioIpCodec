

use gtk::prelude::*;
use gtk::{Label, Entry, Switch, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;



#[derive(Clone)]
pub struct SettingsProxyStorage {
    lbl_outbound_proxy1: Label,
    lbl_outbound_proxy2: Label,
    lbl_outbound_proxy3: Label,
    lbl_outbound_proxy4: Label,
    ent_outbound_proxy1: Entry,
    ent_outbound_proxy2: Entry,
    ent_outbound_proxy3: Entry,
    ent_outbound_proxy4: Entry,
    ent_outbound_proxy_username1: Entry,
    ent_outbound_proxy_username2: Entry,
    ent_outbound_proxy_username3: Entry,
    ent_outbound_proxy_username4: Entry,
    ent_outbound_proxy_password1: Entry,
    ent_outbound_proxy_password2: Entry,
    ent_outbound_proxy_password3: Entry,
    ent_outbound_proxy_password4: Entry,
    swt_outbound_proxy1: Switch,
    swt_outbound_proxy2: Switch,
    swt_outbound_proxy3: Switch,
    swt_outbound_proxy4: Switch,

}

impl SettingsProxyStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsProxyStorage {
            lbl_outbound_proxy1: gtk_builder.get_object("lbl_outbound_proxy1").unwrap(),
            lbl_outbound_proxy2: gtk_builder.get_object("lbl_outbound_proxy2").unwrap(),
            lbl_outbound_proxy3: gtk_builder.get_object("lbl_outbound_proxy3").unwrap(),
            lbl_outbound_proxy4: gtk_builder.get_object("lbl_outbound_proxy4").unwrap(),
            ent_outbound_proxy1: gtk_builder.get_object("ent_outbound_proxy1").unwrap(),
            ent_outbound_proxy2: gtk_builder.get_object("ent_outbound_proxy2").unwrap(),
            ent_outbound_proxy3: gtk_builder.get_object("ent_outbound_proxy3").unwrap(),
            ent_outbound_proxy4: gtk_builder.get_object("ent_outbound_proxy4").unwrap(),
            ent_outbound_proxy_username1: gtk_builder.get_object("ent_outbound_proxy_username1").unwrap(),
            ent_outbound_proxy_username2: gtk_builder.get_object("ent_outbound_proxy_username2").unwrap(),
            ent_outbound_proxy_username3: gtk_builder.get_object("ent_outbound_proxy_username3").unwrap(),
            ent_outbound_proxy_username4: gtk_builder.get_object("ent_outbound_proxy_username4").unwrap(),
            ent_outbound_proxy_password1: gtk_builder.get_object("ent_outbound_proxy_password1").unwrap(),
            ent_outbound_proxy_password2: gtk_builder.get_object("ent_outbound_proxy_password2").unwrap(),
            ent_outbound_proxy_password3: gtk_builder.get_object("ent_outbound_proxy_password3").unwrap(),
            ent_outbound_proxy_password4: gtk_builder.get_object("ent_outbound_proxy_password4").unwrap(),
            swt_outbound_proxy1: gtk_builder.get_object("swt_outbound_proxy1").unwrap(),
            swt_outbound_proxy2: gtk_builder.get_object("swt_outbound_proxy2").unwrap(),
            swt_outbound_proxy3: gtk_builder.get_object("swt_outbound_proxy3").unwrap(),
            swt_outbound_proxy4: gtk_builder.get_object("swt_outbound_proxy4").unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct SettingsProxyWidget {
    ctx: RefCell<SettingsProxyStorage>
}

impl SettingsProxyWidget {
    pub fn new(gtk_builder: &Builder) -> Self {
        let result = SettingsProxyWidget {
            ctx: RefCell::new(SettingsProxyStorage::new(gtk_builder))
        };

        result.ctx.borrow().lbl_outbound_proxy1.set_sensitive(false);
        result.ctx.borrow().lbl_outbound_proxy2.set_sensitive(false);
        result.ctx.borrow().lbl_outbound_proxy3.set_sensitive(false);
        result.ctx.borrow().lbl_outbound_proxy4.set_sensitive(false);
        result.ctx.borrow().ent_outbound_proxy1.set_sensitive(false);
        result.ctx.borrow().ent_outbound_proxy2.set_sensitive(false);
        result.ctx.borrow().ent_outbound_proxy3.set_sensitive(false);
        result.ctx.borrow().ent_outbound_proxy4.set_sensitive(false);
        result.ctx.borrow().ent_outbound_proxy_username1.set_sensitive(false);
        result.ctx.borrow().ent_outbound_proxy_username2.set_sensitive(false);
        result.ctx.borrow().ent_outbound_proxy_username3.set_sensitive(false);
        result.ctx.borrow().ent_outbound_proxy_username4.set_sensitive(false);
        result.ctx.borrow().ent_outbound_proxy_password1.set_sensitive(false);
        result.ctx.borrow().ent_outbound_proxy_password2.set_sensitive(false);
        result.ctx.borrow().ent_outbound_proxy_password3.set_sensitive(false);
        result.ctx.borrow().ent_outbound_proxy_password4.set_sensitive(false);

        let result_clone = result.clone();
        result.ctx.borrow().swt_outbound_proxy1.connect_property_active_notify(move | swt | {
            result_clone.ctx.borrow().lbl_outbound_proxy1.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_outbound_proxy1.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_outbound_proxy_username1.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_outbound_proxy_password1.set_sensitive(swt.get_state());
        });

        let result_clone = result.clone();
        result.ctx.borrow().swt_outbound_proxy2.connect_property_active_notify(move | swt | {
            result_clone.ctx.borrow().lbl_outbound_proxy2.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_outbound_proxy2.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_outbound_proxy_username2.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_outbound_proxy_password2.set_sensitive(swt.get_state());

        });

        let result_clone = result.clone();
        result.ctx.borrow().swt_outbound_proxy3.connect_property_active_notify(move | swt | {
            result_clone.ctx.borrow().lbl_outbound_proxy3.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_outbound_proxy3.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_outbound_proxy_username3.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_outbound_proxy_password3.set_sensitive(swt.get_state());

        });

        let result_clone = result.clone();
        result.ctx.borrow().swt_outbound_proxy4.connect_property_active_notify(move | swt | {
            result_clone.ctx.borrow().lbl_outbound_proxy4.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_outbound_proxy4.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_outbound_proxy_username4.set_sensitive(swt.get_state());
            result_clone.ctx.borrow().ent_outbound_proxy_password4.set_sensitive(swt.get_state());

        });

        result
    }

    pub fn reset(&self) {
        todo!();
    }
}

impl HelperFileSettings for SettingsProxyWidget {
    fn load(&self, path: PathBuf) {
        todo!()
    }

    fn save(&self, path: PathBuf) {
        todo!()
    }
}

