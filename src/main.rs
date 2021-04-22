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
mod settings_audio;
mod settings_ua;
mod settings_ice;
mod settings_media;
mod settings_stun;
mod settings_turn;
mod settings_proxy;
mod settings_dns;

// sipua module
pub mod pjproject;
pub mod sipua;
mod settings_tls;


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
use codec::CodecWidget;
use settings::{SettingsCurrentActivePage, SettingsWidget};
use helper::{HelperFileSettings, application_config_path};

use sipua::prelude::*;
use sipua::sip_account::SIPAccount;
use sipua::SIPInviteState;
use pjproject::utils::AutoCreate;

use pj_sys::*;
use pjmedia_sys::*;
use sipua::*;

use sip_account::SIPAccountExt;

enum SignalLevel { Level( (u32, u32, u32, u32)) }


// TODO: Check BBC reference sip application
// with comparable like apple to apple

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
            SettingsCurrentActivePage::Ua => settings_clone.call.save(config_path),
            SettingsCurrentActivePage::Stun => settings_clone.stun.save(config_path),
            SettingsCurrentActivePage::Turn => settings_clone.turn.save(config_path),
            SettingsCurrentActivePage::Ice => settings_clone.ice.save(config_path),
            SettingsCurrentActivePage::Audio => settings_clone.audio.save(config_path),
            SettingsCurrentActivePage::Media => settings_clone.media.save(config_path),
            SettingsCurrentActivePage::Proxy => settings_clone.proxy.save(config_path),
            SettingsCurrentActivePage::Dns => settings_clone.dns.save(config_path),
        };
    });

    // reset configuration for each tab
    let settings_clone = settings.clone();
    settings.reset_connect_clicked(move |page| {
        match page.unwrap() {
            SettingsCurrentActivePage::Ua => settings_clone.call.reset(),
            SettingsCurrentActivePage::Stun => settings_clone.stun.reset(),
            SettingsCurrentActivePage::Turn => settings_clone.turn.reset(),
            SettingsCurrentActivePage::Ice => settings_clone.ice.reset(),
            SettingsCurrentActivePage::Audio => settings_clone.audio.reset(),
            SettingsCurrentActivePage::Media => settings_clone.media.reset(),
            SettingsCurrentActivePage::Proxy => settings_clone.proxy.reset(),
            SettingsCurrentActivePage::Dns => settings_clone.dns.reset(),
        };
    });

    // apply configuration for each tab
    let settings_clone = settings.clone();
    let ua = sipua.clone();
    settings.apply_connect_clicked(move |page| {
        match page.unwrap() {
            SettingsCurrentActivePage::Ua => {
                ua.set_autoanswer(settings_clone.call.get_autoanswer());
                ua.set_no_refersub(settings_clone.call.get_no_refersub());
                ua.set_compact_form(settings_clone.call.get_compact_form());
                ua.set_no_forcelr(settings_clone.call.get_no_forcelr());
            },
            SettingsCurrentActivePage::Stun => {

                let mut stun_data: Vec<SIPStunServerData> = Vec::new();

                if settings_clone.stun.get_state_server1() {
                    stun_data.push(
                        SIPStunServerData::new(
                            settings_clone.stun.get_server1(),
                            settings_clone.stun.get_username1(),
                            settings_clone.stun.get_password1()
                        )
                    );
                }

                if settings_clone.stun.get_state_server2() {
                    stun_data.push(
                        SIPStunServerData::new(
                            settings_clone.stun.get_server2(),
                            settings_clone.stun.get_username2(),
                            settings_clone.stun.get_password2()
                        )
                    );
                }

                if settings_clone.stun.get_state_server3() {
                    stun_data.push(
                        SIPStunServerData::new(
                            settings_clone.stun.get_server3(),
                            settings_clone.stun.get_username3(),
                            settings_clone.stun.get_password3()
                        )
                    );
                }

                if settings_clone.stun.get_state_server4() {
                    stun_data.push(
                        SIPStunServerData::new(
                            settings_clone.stun.get_server4(),
                            settings_clone.stun.get_username4(),
                            settings_clone.stun.get_password4()
                        )
                    );
                }
                // update default stun server
                ua.set_stun_server(stun_data);
            },
            SettingsCurrentActivePage::Turn => {
                ua.set_use_turn(settings_clone.turn.get_use_turn());

                // set transport type
                match settings_clone.turn.get_transport() {
                    1 => ua.set_turn_conn_type(PJ_TURN_TP_UDP),
                    2 => ua.set_turn_conn_type(PJ_TURN_TP_TCP),
                    3 => ua.set_turn_conn_type(PJ_TURN_TP_TLS),
                    _ => ()
                }

                ua.set_turn_server(SIPTurnServerData::new(
                    settings_clone.turn.get_server(),
                    settings_clone.turn.get_username(),
                    settings_clone.turn.get_password()
                ));
            },
            SettingsCurrentActivePage::Ice => {
                ua.set_use_ice(settings_clone.ice.get_use_ice());
                ua.set_no_rtcp(settings_clone.ice.get_no_rtcp());
                // deprecated
                // unable to find any doc in broadcasting tools
                // ua.set_aggressive_nomination(settings_clone.ice.get_aggressive());
                // match settings_clone.ice.get_trickle_method() {
                //     1 => ua.set_trickle_method(PJ_ICE_SESS_TRICKLE_DISABLED),
                //     2 => ua.set_trickle_method(PJ_ICE_SESS_TRICKLE_HALF),
                //     3 => ua.set_trickle_method(PJ_ICE_SESS_TRICKLE_FULL),
                //     _ => ()
                // }
                ua.set_ice_max_host_cands(settings_clone.ice.get_max_hosts() as i32);
            },
            SettingsCurrentActivePage::Audio => {
                ua.set_jb_max(settings_clone.audio.get_jb_max() as i32);
                ua.set_ptime(settings_clone.audio.get_ptime() as u32);
                ua.set_quality(settings_clone.audio.get_quality() as u32);
                ua.set_no_vad(settings_clone.audio.get_no_vad());
                ua.set_ec_tail_len(settings_clone.audio.get_ec_tail_len() as u32);
                match settings_clone.audio.get_ec_options() {
                    1 => ua.set_ec_options(PJMEDIA_ECHO_DEFAULT),
                    2 => ua.set_ec_options(PJMEDIA_ECHO_SPEEX),
                    3 => ua.set_ec_options(PJMEDIA_ECHO_SIMPLE),
                    4 => ua.set_ec_options(PJMEDIA_ECHO_WEBRTC),
                    _ => ()
                }
            },
            SettingsCurrentActivePage::Media => {
                todo!();
            },
            SettingsCurrentActivePage::Proxy => {

                let mut proxy: Vec<SIPOutboundProxyServerData> = Vec::new();
                // update proxy server 1
                if settings_clone.proxy.get_state_proxy1() {
                    proxy.push(
                        SIPOutboundProxyServerData::new(
                            settings_clone.proxy.get_proxy1(),
                            settings_clone.proxy.get_username1(),
                            settings_clone.proxy.get_password1()
                        )
                    );
                }

                // update proxy server 2
                if settings_clone.proxy.get_state_proxy2() {
                    proxy.push(
                        SIPOutboundProxyServerData::new(
                            settings_clone.proxy.get_proxy2(),
                            settings_clone.proxy.get_username2(),
                            settings_clone.proxy.get_password2()
                        )
                    );
                }

                // update proxy server 3
                if settings_clone.proxy.get_state_proxy3() {
                    proxy.push(
                        SIPOutboundProxyServerData::new(
                            settings_clone.proxy.get_proxy3(),
                            settings_clone.proxy.get_username3(),
                            settings_clone.proxy.get_password3()
                        )
                    );
                }

                // update proxy server 4
                if settings_clone.proxy.get_state_proxy4() {
                    proxy.push(
                        SIPOutboundProxyServerData::new(
                            settings_clone.proxy.get_proxy4(),
                            settings_clone.proxy.get_username4(),
                            settings_clone.proxy.get_password4()
                        )
                    );
                }

                // update outbound proxy server
                ua.set_outbound_proxy(proxy);
            },
            SettingsCurrentActivePage::Dns => {

                let mut server: Vec<String> = Vec::new();

                // update nameserver 1
                if settings_clone.dns.get_state_nameserver1() {
                    server.push(settings_clone.dns.get_nameserver1());
                }

                // update namserver 2
                if settings_clone.dns.get_state_nameserver2() {
                    server.push(settings_clone.dns.get_nameserver2());
                }

                // update nameserver 3
                if settings_clone.dns.get_state_nameserver3() {
                    server.push(settings_clone.dns.get_nameserver3());
                }

                // update nameserver 4
                if settings_clone.dns.get_state_nameserver4() {
                    server.push(settings_clone.dns.get_nameserver4());
                }

                // update dns
                ua.set_nameserver(server);
            }
        }
    });
}

fn callback_codec_widget(sipua: &mut SIPUserAgent, settings: &CodecWidget) {

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
    let builder= Builder::from_string(include_str!("../glade/main_ui.glade"));
    let main_window: gtk::ApplicationWindow = builder.get_object("main_ui").unwrap();

    let rx_widget = audio_line::create_transmit_widget(&builder);
    let tx_widget = audio_line::create_receive_widget(&builder);
    let maintab_widget= MaintabWidget::new(&builder);
    let statusbar_widget = StatusbarWidget::new(&builder);
    let headerbar_widget = HeaderWidget::new(&builder);
    let dialpad_widget = DialpadWidget::new(&builder);
    let account_widget = AccountWidget::new(&builder);
    let settings_widget= SettingsWidget::new(&builder);
    let codec_widget = CodecWidget::new(&builder);

    // set callback
    callback_audio_line_widget(&mut sipua, &rx_widget, &tx_widget);
    callback_dialpad_widget(&mut sipua, &dialpad_widget);
    callback_account_widget(&mut sipua, &account_widget);
    callback_settings_widget(&mut sipua, &settings_widget);
    callback_codec_widget(&mut sipua, &codec_widget);

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

    // test google stun server
    settings_widget.stun.set_server1("stun1.l.google.com:19302".to_string());
    settings_widget.stun.set_server2("stun2.l.google.com:19302".to_string());
    settings_widget.stun.set_server3("stun3.l.google.com:19302".to_string());
    settings_widget.stun.set_server4("stun4.l.google.com:19302".to_string());


    // test public dns nameserver
    settings_widget.dns.set_nameserver1("8.8.8.8".to_string()); // google main dns
    settings_widget.dns.set_nameserver2("8.8.4.4".to_string()); // google backup dns
    settings_widget.dns.set_nameserver3("1.1.1.1".to_string()); // cloudflare main dns
    settings_widget.dns.set_nameserver4("1.0.0.1".to_string()); // cloudflare backup dns

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
