
use gtk::prelude::*;

use gtk::{Switch, Label, Builder};
use glib::clone;

use std::cell::RefCell;


pub struct SettingsCallWidgetStorage {
    swt_autoanswer: Switch,
    lbl_autoanswer: Label,
}

impl SettingsCallWidgetStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsCallWidgetStorage {
            swt_autoanswer: gtk_builder.get_object("swt_autoanswer").unwrap(),
            lbl_autoanswer: gtk_builder.get_object("lbl_autoanswer").unwrap(),
        }
    }
}


pub struct SettingsCallWidget {
    // inner data just borrow not mutate
    ctx: RefCell<SettingsCallWidgetStorage>
}

impl SettingsCallWidget {

    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsCallWidget{
            ctx: RefCell::new(SettingsCallWidgetStorage::new(gtk_builder))
        }
    }

    /// sub procedure reset state gui
    pub fn default(&self) {
        self.ctx.borrow().swt_autoanswer.set_state(false);
    }

    // set autoanswer state
    pub fn set_autoanswer(&self, value: bool) {
        self.ctx.borrow().swt_autoanswer.set_state(value);
    }

    // get autoanswer state
    pub fn get_autoanswer(&self) -> bool {
        self.ctx.borrow().swt_autoanswer.get_state()
    }
}