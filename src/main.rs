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
extern crate gdk;

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
mod account;
mod settings;

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
use account::AccountWidget;
use settings::SettingsWidget;

use pj_sys::*;
use sipua::*;

enum SignalLevel { Level( (u32, u32, u32, u32)) }

/// update receive transmit level bar
fn thread_update_level_bar(sipua_clone: SIPUserAgent, mut rx_widget_clone: AudioLineWidget, mut tx_widget_clone: AudioLineWidget) {
    // sender, receiver more clear to read
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    // let sip = sipua.clone();
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

    receiver.attach(None, move |level| {
        match level {
            SignalLevel::Level((tx_l,tx_r, rx_l, rx_r)) => {
                rx_widget_clone.set_level_bar(rx_l, rx_r);
                tx_widget_clone.set_level_bar(tx_l, tx_r);
            }
        }

        glib::Continue(true)
    });
}

// calback audio line transmit receive
fn callback_audio_line_widget(sipua: &mut SIPUserAgent, rx_widget: &mut AudioLineWidget, tx_widget: &mut AudioLineWidget) {

    // update device list
    for dev_name in sipua.get_output_device_list().iter_mut() {
        rx_widget.add_device_text(dev_name);
    }

    for dev_name in sipua.get_input_device_list().iter_mut() {
        tx_widget.add_device_text(dev_name);
    }

    // slider level change
    let sip = sipua.clone();
    rx_widget.on_scale_changed_value ( move |v| {
        sip.set_input_level(v);
    });

    let sip = sipua.clone();
    tx_widget.on_scale_changed_value ( move |v| {
        sip.set_output_level(v);
    });

    tx_widget.on_button_mute_clicked(|| {
        // todo callback tx mute

    });

    rx_widget.on_button_mute_clicked(|| {
        // todo callback rx mute

    });

}

// callback dialpad widget
fn callback_dialpad_widget(sipua: &mut SIPUserAgent, dialpad: &mut DialpadWidget) {
    // button call clicked
    let sip = sipua.clone();
    dialpad.on_button_call_clicked(move |sip_address| {
        println!("sip_call_addres : {}", sip_address);
        sip.call(sip_address);
    });

}


fn main() {
    gtk::init()
    .expect("Cant initalize gtk");

    let application = Application::new(
        Some("com.tamara.open_ip_audio_codec"),
        gio::ApplicationFlags::FLAGS_NONE
    ).expect("GTK app fail to initialize.");

    let mut sipua = SIPUserAgent::new();
    sipua.start();

    // builder
    let main_src = include_str!("../glade/main_ui.glade");
    let builder: Builder = Builder::from_string(main_src);

    let main_window: gtk::ApplicationWindow = builder.get_object("main_ui").unwrap();

    let mut rx_widget = audio_line::create_transmit_widget(&builder);
    let mut tx_widget = audio_line::create_receive_widget(&builder);

    let mut maintab_widget: MaintabWidget = MaintabWidget::new(&builder);

    let statusbar_widget: StatusbarWidget = StatusbarWidget::new(&builder);

    let headerbar_widget: HeaderWidget = HeaderWidget::new(&builder);

    let mut dialpad_widget: DialpadWidget = DialpadWidget::new(&builder);

    let account_widget: AccountWidget = AccountWidget::new(&builder);
    let settings_widget: SettingsWidget = SettingsWidget::new(&builder);

    // initialize
    rx_widget.init();
    tx_widget.init();
    maintab_widget.init();
    headerbar_widget.init();
    dialpad_widget.init();
    account_widget.init();
    settings_widget.init();

    callback_audio_line_widget(&mut sipua, &mut rx_widget, &mut tx_widget);
    callback_dialpad_widget(&mut sipua, &mut dialpad_widget);

    // test call data
    dialpad_widget.add_call_log("sip://@27.50.19.174");
    dialpad_widget.add_call_log("*888#");
    dialpad_widget.add_call_log("*363#");

    // init application
    application.connect_activate(move |app| {
        // input
        main_window.set_application(Some(app));
        main_window.show_all();
    });

    // thread procedure to update level bar
    // Transmit and Receive
    thread_update_level_bar(sipua.clone(), rx_widget.clone(), tx_widget.clone());

    // sub testing gui
    application.run(&env::args().collect::<Vec<_>>());

}
