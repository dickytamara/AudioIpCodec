
use gtk::prelude::*;
use gtk::{Builder, Notebook, Button};
use std::cell::RefCell;



use super::settings_call::SettingsCallWidget;
use super::settings_turn::SettingsTurnWidget;
use super::settings_ice::SettingsIceWidget;


#[derive(Clone, Copy)]
pub enum SettingsCurrentActivePage {
    Call,
    Turn,
    Ice,
    Buffer,
    Media
}


#[derive(Clone)]
pub struct SettingsWidgetStorage {
    // gtk notebook
    ntbk_settings: Notebook,
    btn_settings_reset: Button,
    btn_settings_save: Button,
}

impl SettingsWidgetStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsWidgetStorage {
            ntbk_settings: gtk_builder.get_object("ntbk_settings").unwrap(),
            btn_settings_reset: gtk_builder.get_object("btn_settings_reset").unwrap(),
            btn_settings_save: gtk_builder.get_object("btn_settings_save").unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct SettingsWidget {
    // inner data just borrow not mutate
    ctx: RefCell<SettingsWidgetStorage>,
    pub call: SettingsCallWidget,
    pub turn: SettingsTurnWidget,
    pub ice: SettingsIceWidget,

}


impl SettingsWidget {
    pub fn new(gtk_builder: &gtk::Builder) -> Self {
        SettingsWidget {
            ctx: RefCell::new(SettingsWidgetStorage::new(gtk_builder)),
            call: SettingsCallWidget::new(gtk_builder),
            turn: SettingsTurnWidget::new(gtk_builder),
            ice: SettingsIceWidget::new(gtk_builder),
        }
    }

    // get current active page
    pub fn get_current_active_page(&self) -> Option<SettingsCurrentActivePage> {
        match self.ctx.borrow().ntbk_settings.get_current_page() {
            Some(idx) => {
                match idx {
                    0 => Some(SettingsCurrentActivePage::Call),
                    1 => Some(SettingsCurrentActivePage::Turn),
                    2 => Some(SettingsCurrentActivePage::Ice),
                    3 => Some(SettingsCurrentActivePage::Buffer),
                    4 => Some(SettingsCurrentActivePage::Media),
                    _ => None
                }
            },
            None => None
        }
    }

    pub fn reset_connect_clicked<F> (&self, callback: F)
    where
        F: Fn(Option<SettingsCurrentActivePage>) + 'static
    {
        let wid = self.clone();
        self.ctx.borrow()
        .btn_settings_reset.connect_clicked( move |_| {
            (callback)(wid .get_current_active_page());
        });
    }

    pub fn save_connect_clicked<F> (&self, callback: F)
    where
        F: Fn(Option<SettingsCurrentActivePage>) + 'static
    {
        let wid = self.clone();
        self.ctx.borrow()
        .btn_settings_save.connect_clicked( move |_| {
            (callback)(wid.get_current_active_page());
        });
    }
}