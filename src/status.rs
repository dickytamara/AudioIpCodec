
use gtk::prelude::*;
use gtk::{Statusbar, Label, Builder};

pub struct StatusbarWidget {
    statusbar: Statusbar,
    lbl_status: Label
}

impl StatusbarWidget {

    pub fn new(gtk_builder: &Builder) -> StatusbarWidget {
        StatusbarWidget {
            statusbar: gtk_builder.get_object("statusbar_main").unwrap(),
            lbl_status: gtk_builder.get_object("lbl_statusbar_main").unwrap()
        }
    }

}
