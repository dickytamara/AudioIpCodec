#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate pjsua_sys;
extern crate pjmedia_sys;
extern crate pjsip_sys;
extern crate pjsip_simple_sys;
extern crate pj_sys;

extern crate gtk;
extern crate gio;
extern crate glib;

use gtk::{HeaderBar, TreeView, prelude::*};
use gio::prelude::*;

use std::env;
use std::fmt::format;
use std::include_str;
use gtk::{Application, ApplicationWindow, Statusbar, Button, Builder, Scale};
use gtk::ComboBoxText;
use glib::clone;


// gui module
mod dialpad;
mod audio_line;
mod maintab;
mod header;

// sipua module
mod pjdefault;
mod pjlib;
mod pjsip;
mod pjmedia;
mod pjsua;
mod sipua;

use dialpad::DialpadWidget;
use audio_line::AudioLineWidget;
use maintab::MaintabWidget;
use header::HeaderWidget;

use sipua::*;


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


fn main() {
    gtk::init()
    .expect("Cant initalize gtk");

    let application = Application::new(
        Some("com.tamara.open_ip_audio_codec"),
        gio::ApplicationFlags::FLAGS_NONE
    ).expect("GTK app fail to initialize.");

    let sipua = SIPUserAgent::new();
    sipua.start();

    // builder
    let main_src = include_str!("../glade/main_ui.glade");
    let builder: Builder = Builder::from_string(main_src);

    let main_window: gtk::ApplicationWindow = builder.get_object("main_ui").unwrap();

    // create input widget
    let mut input_widget: AudioLineWidget = AudioLineWidget::new(&builder,
         "lbl_topbar_input",
         "lbl_input_level_l",
         "lbl_input_level_r",
         "lvl_input_l",
         "lvl_input_r",
         "btn_input_level_dec",
         "btn_input_level_inc",
         "sldr_input_level",
         "lbl_input_device",
         "cmb_input_device",
         "btn_input_mute"
      );
    input_widget.init();

    // create output widget
    let mut output_widget: AudioLineWidget = AudioLineWidget::new(&builder,
         "lbl_topbar_output",
         "lbl_output_level_l",
         "lbl_output_level_r",
         "lvl_output_l",
         "lvl_output_r",
         "btn_output_level_dec",
         "btn_output_level_inc",
         "sldr_output_level",
         "lbl_output_device",
         "cmb_output_device",
         "btn_output_mute"
    );
    output_widget.init();

    let mut maintab_widget: MaintabWidget = MaintabWidget::new(&builder);
    maintab_widget.init();

    let statusbar_widget: StatusbarWidget = StatusbarWidget::new(&builder);

    let headerbar_widget: HeaderWidget = HeaderWidget::new(&builder);

    let mut dialpad_widget: DialpadWidget = DialpadWidget::new(&builder);
    dialpad_widget.init();

    for dev_name in sipua.get_input_device_list().iter_mut() {
        input_widget.add_device_text(dev_name);
    }

    for dev_name in sipua.get_output_device_list().iter_mut() {
        output_widget.add_device_text(dev_name);
    }

    // init application
    application.connect_activate(move |app| {
        // input
        main_window.set_application(Some(app));
        main_window.show_all();
    });

    // sub testing gui
    application.run(&env::args().collect::<Vec<_>>());

}
