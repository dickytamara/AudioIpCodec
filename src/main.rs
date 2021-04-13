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
extern crate configparser;

// gui module
mod helper;
mod about;
mod audio_line;
mod codec;
mod dialpad;
mod maintab;
mod header;
mod status;
mod account;
mod settings;
mod settings_buffer;
mod settings_call;
mod settings_ice;
mod settings_media;
mod settings_turn;

// sipua module
pub mod pjproject;
pub mod sipua;


use gtk::prelude::*;
use gio::prelude::*;

use std::env;
use std::include_str;
use std::thread;
use std::time::Duration;
use std::ptr;

use gtk::{Application, Builder};

use dialpad::{DialpadWidget, CallButtonState};
use audio_line::AudioLineWidget;
use maintab::MaintabWidget;
use header::HeaderWidget;
use status::StatusbarWidget;
use account::AccountWidget;
use settings::{SettingsCurrentActivePage, SettingsWidget};
use helper::{HelperFileSettings, application_config_path};


use sipua::sip_account::SIPAccount;
use sipua::SIPInviteState;
use pjproject::utils::AutoCreate;

use pj_sys::*;
use sipua::*;

use sip_account::SIPAccountExt;

enum SignalLevel { Level( (u32, u32, u32, u32)) }


/// update receive transmit level bar
fn thread_update_level_bar(sipua_clone: SIPUserAgent, rx_widget_clone: AudioLineWidget, tx_widget_clone: AudioLineWidget) {

    // sender, receiver more clear to read
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

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

    // todo fix the late destroy of
    // thread. this segmentation fault
    // trigered when receiver destroyed
    // before sender.
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
fn callback_audio_line_widget(sipua: &mut SIPUserAgent, rx_widget: &AudioLineWidget, tx_widget: &AudioLineWidget) {

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

    let sip = sipua.clone();
    rx_widget.on_button_mute_clicked(move |state| {
        sip.input_mute(state);
    });

    let sip = sipua.clone();
    tx_widget.on_button_mute_clicked(move |state| {
        sip.output_mute(state);
    });
}

// callback dialpad widget
fn callback_dialpad_widget(sipua: &mut SIPUserAgent, dialpad: &DialpadWidget) {

    // button call clicked
    let sip = sipua.clone();
    dialpad.on_button_call_clicked(move | sip_address, state | {
        println!("sip_call_addres : {}", sip_address);

        match state {
            CallButtonState::Call => sip.call(sip_address),
            CallButtonState::Hangup => sip.call_hangup(),
            CallButtonState::Abort => sip.call_hangup(),
            CallButtonState::Answer => sip.call_answer(),
            _ => ()
        }

    });

    // callback inv state
    let dialpad = dialpad.clone();
    sipua.connect_invite ( move | state | {
        let dialpad = dialpad.clone();
        glib::source::idle_add( move || {
            match state {
                SIPInviteState::Null => dialpad.update_state_normal(),
                SIPInviteState::Calling => dialpad.update_state_outgoing(),
                SIPInviteState::Incoming => dialpad.update_state_incoming(),
                SIPInviteState::Early => dialpad.update_state_normal(),
                SIPInviteState::Connecting => dialpad.update_state_normal(),
                SIPInviteState::Confirmed => dialpad.update_state_oncall(),
                SIPInviteState::Disconnected => dialpad.update_state_normal(),
                SIPInviteState::Unknown => dialpad.update_state_normal()
            }

            glib::Continue(false)
        });

    });
}

// callback account widget
fn callback_account_widget(sipua: &mut SIPUserAgent, account: &AccountWidget) {

    let account_clone = account.clone();
    account.save_connect_clicked(move ||{
        account_clone.save(application_config_path());
    });

    let account_clone = account.clone();
    account.on_btn_connect_clicked(move || {

        let mut sipacc = SIPAccount::new();

        sipacc.config_default();
        let mut transport = pjsua_sys::pjsua_transport_config::new();
        transport.port = 4000;
        sipacc.set_rtp_cfg(transport);

        sipacc.set_id(account_clone.get_sip_url());
        sipacc.set_reg_uri(account_clone.get_registrar_url());
        sipacc.set_realm(account_clone.get_realm());
        sipacc.set_username(account_clone.get_username());
        sipacc.set_password(account_clone.get_password());

        sipacc.add(true);
    });
}

fn callback_settings_widget(sipua: &mut SIPUserAgent, settings: &SettingsWidget) {

    // save setting for each tab
    let settings_clone = settings.clone();
    settings.save_connect_clicked(move |page| {
        let config_path = application_config_path();
        match page.unwrap() {
            SettingsCurrentActivePage::Call => settings_clone.call.save(config_path),
            SettingsCurrentActivePage::Turn => settings_clone.turn.save(config_path),
            SettingsCurrentActivePage::Ice => settings_clone.ice.save(config_path),
            SettingsCurrentActivePage::Buffer => todo!(),
            SettingsCurrentActivePage::Media => todo!(),
        };
    });

    // reset configuration for each tab
    let settings_clone = settings.clone();
    settings.reset_connect_clicked(move |page| {
        match page.unwrap() {
            SettingsCurrentActivePage::Call => settings_clone.call.reset(),
            SettingsCurrentActivePage::Turn => settings_clone.turn.reset(),
            SettingsCurrentActivePage::Ice => settings_clone.ice.reset(),
            SettingsCurrentActivePage::Buffer => todo!(),
            SettingsCurrentActivePage::Media => todo!(),
        };
    });

    // apply configuration for each tab
    let settings_clone = settings.clone();
    let ua = sipua.clone();
    settings.apply_connect_clicked(move |page| {
        match page.unwrap() {
            SettingsCurrentActivePage::Call => {
                ua.set_autoanswer(settings_clone.call.get_autoanswer());
            },
            SettingsCurrentActivePage::Turn => { todo!(); },
            SettingsCurrentActivePage::Ice => { todo!(); },
            SettingsCurrentActivePage::Buffer => { todo!(); },
            SettingsCurrentActivePage::Media => { todo!(); },
        }
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
    let builder= Builder::from_string("../glade/main_ui.glade");
    let main_window: gtk::ApplicationWindow = builder.get_object("main_ui").unwrap();

    let rx_widget = audio_line::create_transmit_widget(&builder);
    let tx_widget = audio_line::create_receive_widget(&builder);
    let maintab_widget= MaintabWidget::new(&builder);
    let statusbar_widget = StatusbarWidget::new(&builder);
    let headerbar_widget = HeaderWidget::new(&builder);
    let dialpad_widget = DialpadWidget::new(&builder);
    let account_widget = AccountWidget::new(&builder);
    let settings_widget= SettingsWidget::new(&builder);

    // set callback
    callback_audio_line_widget(&mut sipua, &rx_widget, &tx_widget);
    callback_dialpad_widget(&mut sipua, &dialpad_widget);
    callback_account_widget(&mut sipua, &account_widget);
    callback_settings_widget(&mut sipua, &settings_widget);

    // test call data
    dialpad_widget.set_call_address_text(String::from("sip://@27.50.19.174"));
    dialpad_widget.add_call_log("sip://@27.50.19.174");
    dialpad_widget.add_call_log("*888#");
    dialpad_widget.add_call_log("*363#");

    // account test
    account_widget.set_sip_url("sip:ipcodec01@27.50.19.174");
    account_widget.set_registrar_url("sip:27.50.19.174");
    account_widget.set_realm("asterisk");
    account_widget.set_username("ipcodec01");
    account_widget.set_password("12345678");

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
