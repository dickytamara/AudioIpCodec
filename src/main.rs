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

use gtk::prelude::*;
use gio::prelude::*;

use std::env;
use std::include_str;
use gtk::{Application, ApplicationWindow, Button, Builder, Scale};


mod pjdefault;
mod pjlib;
mod pjsip;
mod pjmedia;
mod pjsua;
mod sipua;

// use sipua::*;


// this struct for grouping gtk_object
pub struct TopbarWidget {
    lbl_topbar: gtk::Label,
    lbl_level_l: gtk::Label, 
    lbl_level_r: gtk::Label,
    lvl_l: gtk::LevelBar,
    lvl_r: gtk::LevelBar,
    btn_level_dec: gtk::Button,
    btn_level_inc: gtk::Button,
    sldr_level: gtk::Scale,
    lbl_device: gtk::Label,
    cmb_device: gtk::ComboBox,
    btn_mute: gtk::ToggleButton
}

impl TopbarWidget {
    
    fn new(gtk_builder: &gtk::Builder,
        lbl_topbar_id: &str,
        lbl_level_l_id: &str,
        lbl_level_r_id: &str,
        lvl_l_id: &str,
        lvl_r_id: &str,
        btn_level_dec_id: &str,
        btn_level_inc_id: &str,
        sldr_level_id: &str,
        lbl_device_id: &str,
        cmb_device_id: &str,
        btn_mute_id: &str
      ) -> TopbarWidget {
        TopbarWidget{
            lbl_topbar: gtk_builder.get_object(lbl_topbar_id).unwrap(),
            lbl_level_l: gtk_builder.get_object(lbl_level_l_id).unwrap(), 
            lbl_level_r: gtk_builder.get_object(lbl_level_r_id).unwrap(),
            lvl_l: gtk_builder.get_object(lvl_l_id).unwrap(),
            lvl_r: gtk_builder.get_object(lvl_r_id).unwrap(),
            btn_level_dec: gtk_builder.get_object(btn_level_dec_id).unwrap(),
            btn_level_inc: gtk_builder.get_object(btn_level_inc_id).unwrap(),
            sldr_level: gtk_builder.get_object(sldr_level_id).unwrap(),
            lbl_device: gtk_builder.get_object(lbl_device_id).unwrap(),
            cmb_device: gtk_builder.get_object(cmb_device_id).unwrap(),
            btn_mute: gtk_builder.get_object(btn_mute_id).unwrap() 
        }
    }
}



fn main() {
    gtk::init()
    .expect("Cant initalize gtk");

    let application = Application::new(
        Some("com.tamara.open_ip_audio_codec"),
        gio::ApplicationFlags::FLAGS_NONE
    ).expect("GTK app fail to initialize.");

    // builder
    let main_src = include_str!("../glade/main_ui.glade");
    let builder: Builder = Builder::from_string(main_src);

    let main_window: gtk::ApplicationWindow = builder.get_object("main_ui").unwrap();
    
    // create input widget
    let input_widget: TopbarWidget = TopbarWidget::new(&builder,
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
    let output_widget: TopbarWidget = TopbarWidget::new(&builder,
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


    // init application
    application.connect_activate(move |app| {
        // input
        main_window.set_application(Some(app));
        main_window.show_all();
    });

    // sub testing gui
    application.run(&env::args().collect::<Vec<_>>());


    // let sip = SIPUserAgent::new();
    // let mut ln = String::new();
    // sip.start();
    // println!("todo: main application here.");
    // std::io::stdin().read_line(&mut ln).unwrap();
}
