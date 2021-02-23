
use super::gtk::prelude::*;

pub struct StatusbarWidget {
    statusbar: gtk::Statusbar,
    lbl_status: gtk::Label
}

impl StatusbarWidget {

    pub fn new(gtk_builder: &gtk::Builder) -> StatusbarWidget {
        StatusbarWidget {
            statusbar: gtk_builder.get_object("statusbar_main").unwrap(),
            lbl_status: gtk_builder.get_object("lbl_statusbar_main").unwrap()
        }
    }

    pub fn init(&self) {

    }
}
