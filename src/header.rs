use super::gtk::prelude::*;

pub struct HeaderWidget{
    cpu_lvl: gtk::LevelBar
}

impl HeaderWidget {

    pub fn new(gtk_builder: &gtk::Builder) -> HeaderWidget {
        HeaderWidget {
            cpu_lvl: gtk_builder.get_object("lvl_cpu").unwrap()
        }
    }

    pub fn init(&self) {

    }

    // start cpu monitoring tools
    pub fn start_cpu_monitor(&self) {

    }

    // stop cpu minitoring tools
    pub fn stop_cpu_monitor(&self) {


    }
}
