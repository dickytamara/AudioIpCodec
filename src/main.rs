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

extern crate systemstat;

use gtk::{HeaderBar, TreeView, prelude::*};
use gio::prelude::*;

use std::{borrow::BorrowMut, env};
use std::fmt::format;
use std::include_str;
use std::thread;
use std::time::Duration;
use std::ptr;

use gtk::{Application, ApplicationWindow, Statusbar, Button, Builder, Scale};
use gtk::ComboBoxText;
use glib::clone;


// gui module
mod dialpad;
mod audio_line;
mod maintab;
mod header;
mod status;

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
use status::StatusbarWidget;

use pj_sys::*;
use sipua::*;


enum SignalLevel { Level( (u32, u32, u32, u32)) }

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
    let mut tx_widget: AudioLineWidget = AudioLineWidget::new(&builder,
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

    // create output widget
    let mut rx_widget: AudioLineWidget = AudioLineWidget::new(&builder,
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

    let mut maintab_widget: MaintabWidget = MaintabWidget::new(&builder);

    let statusbar_widget: StatusbarWidget = StatusbarWidget::new(&builder);

    let headerbar_widget: HeaderWidget = HeaderWidget::new(&builder);

    let mut dialpad_widget: DialpadWidget = DialpadWidget::new(&builder);

    for dev_name in sipua.get_output_device_list().iter_mut() {
        rx_widget.add_device_text(dev_name);
    }

    for dev_name in sipua.get_input_device_list().iter_mut() {
        tx_widget.add_device_text(dev_name);
    }

    // initialize
    rx_widget.init();
    tx_widget.init();
    maintab_widget.init();
    headerbar_widget.init();
    dialpad_widget.init();

    // slider change
    let sipua_clone = sipua.clone();
    rx_widget.on_scale_changed_value ( move |v| {
        sipua_clone.set_input_level(v);
    });

    let sipua_clone = sipua.clone();
    tx_widget.on_scale_changed_value ( move |v| {
        sipua_clone.set_output_level(v);
    });

    // init application
    application.connect_activate(move |app| {
        // input
        main_window.set_application(Some(app));
        main_window.show_all();
    });

    // test thread
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let sipua_clone = sipua.clone();
    thread::spawn(move || {
        unsafe {
            let mut a_thread_desc: pj_thread_desc = [0;64usize];
            let mut a_thread = ptr::null_mut() as *mut pj_thread_t;

            if pj_thread_is_registered() == PJ_FALSE as pj_bool_t {
                pj_thread_register(ptr::null_mut(),
                    a_thread_desc.as_mut_ptr() as *mut _,
                    &mut a_thread as *mut _);
            }

        loop {
            thread::sleep(Duration::from_millis(40));

            let _ = sender.send(SignalLevel::Level(sipua_clone.get_signal_level()));
        }
    }});

    let rx_widget_clone = rx_widget.clone();
    let tx_widget_clone = tx_widget.clone();

    receiver.attach(None, move |level| {
        match level {
            SignalLevel::Level((tx_l,tx_r, rx_l, rx_r)) => {
                rx_widget.set_level_bar(rx_l, rx_r);
                tx_widget.set_level_bar(tx_l, tx_r);
            }
        }

        glib::Continue(true)
    });


    sipua.call("sip://@27.50.19.174:5060");
    // sub testing gui
    application.run(&env::args().collect::<Vec<_>>());

}
