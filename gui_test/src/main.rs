#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate gtk;
extern crate gio;

use gtk::{Builder, LevelBar, Stack, Statusbar, Scale, prelude::*};
use gio::prelude::*;

use std::env::args;
use std::include_str;
use gtk::{Application, ApplicationWindow, Button};


fn main() {
    gtk::init().expect("GTK fail to initialize");

    let application = Application::new(
        Some("com.tamara.open_ip_audio_codec"),
        gio::ApplicationFlags::FLAGS_NONE
    ).expect("GTK app fail to initialize.");

    // builder
    let main_src = include_str!("../../glade/main_ui.glade");
    let builder: Builder = Builder::from_string(main_src);

    // main window
    let main_window: gtk::Window = builder.get_object("main_ui").unwrap();

    // cpu level
    let lvl_cpu: gtk::LevelBar = builder.get_object("lvl_cpu").unwrap();

    // input
    let lbl_topbar_input: gtk::Label = builder.get_object("lbl_topbar_input").unwrap();
    let lbl_input_level_l: gtk::Label = builder.get_object("lbl_input_level_l").unwrap();
    let lbl_input_level_r: gtk::Label = builder.get_object("lbl_input_level_r").unwrap();
    let lvl_input_l: gtk::LevelBar = builder.get_object("lvl_input_l").unwrap();
    let lvl_input_r: gtk::LevelBar = builder.get_object("lvl_input_r").unwrap();
    let btn_input_level_dec: gtk::Button = builder.get_object("btn_input_level_dec").unwrap();
    let btn_input_level_inc: gtk::Button = builder.get_object("btn_input_level_inc").unwrap();
    let sldr_input_level: gtk::Scale = builder.get_object("sldr_input_level").unwrap();
    let lbl_input_device: gtk::Label = builder.get_object("lbl_input_device").unwrap();
    let cmb_input_device: gtk::ComboBox = builder.get_object("cmb_input_device").unwrap();
    let btn_input_mute: gtk::ToggleButton = builder.get_object("btn_input_mute").unwrap();

    // output
    let lbl_topbar_input: gtk::Label = builder.get_object("lbl_topbar_input").unwrap();
    let lbl_output_level_l: gtk::Label = builder.get_object("lbl_output_level_l").unwrap();
    let lbl_output_level_r: gtk::Label = builder.get_object("lbl_output_level_r").unwrap();
    let lvl_output_l: gtk::LevelBar = builder.get_object("lvl_output_l").unwrap();
    let lvl_output_r: gtk::LevelBar = builder.get_object("lvl_output_r").unwrap();
    let btn_output_level_dec: gtk::Button = builder.get_object("btn_output_level_dec").unwrap();
    let btn_output_level_inc: gtk::Button = builder.get_object("btn_output_level_inc").unwrap();
    let sldr_output_level: gtk::Scale = builder.get_object("sldr_output_level").unwrap();
    let lbl_output_device: gtk::Label = builder.get_object("lbl_output_device").unwrap();
    let cmb_output_device: gtk::ComboBox = builder.get_object("cmb_output_device").unwrap();
    let btn_output_mute: gtk::ToggleButton = builder.get_object("btn_output_mute").unwrap();

    // main function button
    let btn_main_call: gtk::Button = builder.get_object("btn_main_call").unwrap();
    let btn_main_account: gtk::Button = builder.get_object("btn_main_account").unwrap();
    let btn_main_settings: gtk::Button = builder.get_object("btn_main_settings").unwrap();
    let btn_main_codec: gtk::Button = builder.get_object("btn_main_codec").unwrap();
    let btn_main_about: gtk::Button = builder.get_object("btn_main_about").unwrap();

    // main content wrapper
    let btnbox_main: gtk::ButtonBox = builder.get_object("btnbox_main").unwrap();
    let stack_main: gtk::Stack = builder.get_object("stack_main").unwrap();

    // status bar
    let statusbar_main: gtk::Statusbar = builder.get_object("statusbar_main").unwrap();
    let lbl_statusbar_main: gtk::Label = builder.get_object("lbl_statusbar_main").unwrap();

    // default config
    sldr_input_level.set_range(0.0, 100.0);
    sldr_input_level.set_value(100.0);
    sldr_input_level.set_increments(1.0, 5.0);
    sldr_input_level.set_slider_size_fixed(true);
    sldr_input_level.set_round_digits(0);
    sldr_input_level.set_digits(0);

    sldr_input_level.connect_value_changed(| sldr| {
        println!("sldr_input_level value: {}", sldr.get_value());
    });

    // default config
    sldr_output_level.set_range(0.0,100.0);
    sldr_output_level.set_value(100.0);
    sldr_output_level.set_increments(1.0, 5.0);
    sldr_output_level.set_slider_size_fixed(true);
    sldr_output_level.set_round_digits(0);
    sldr_output_level.set_digits(0);

    sldr_output_level.connect_value_changed(| sldr | {
        println!("sldr_output_level value: {}", sldr.get_value());
    });


    // input
    btn_input_level_dec.connect_clicked(move|_| {
        println!("btn_input_level_dec");
    });

    btn_input_level_inc.connect_clicked(move|_|{
        println!("btn_input_level_inc");
    });

    btn_input_mute.connect_clicked(|btn| {
        println!("btn_input_mute active: {}", btn.get_active());
    });

    // output
    btn_output_level_dec.connect_clicked(|_| {
        println!("btn_output_level_dec");
    });

    btn_output_level_inc.connect_clicked(|_| {
        println!("btn_output_level_inc");
    });

    btn_output_mute.connect_clicked(|btn| {
        println!("btn_output_mute active: {}", btn.get_active());
    });



    // init application
    application.connect_activate(move |app| {
        // input
        main_window.set_application(Some(app));
        main_window.show_all();
    });


    // add env later this only for
    // sub testing gui
    application.run(&args().collect::<Vec<_>>());
}
