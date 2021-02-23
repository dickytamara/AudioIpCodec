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
use std::include_str;
use gtk::{Application, ApplicationWindow, Statusbar, Button, Builder, Scale};
use gtk::ComboBoxText;
use glib::clone;


mod pjdefault;
mod pjlib;
mod pjsip;
mod pjmedia;
mod pjsua;
mod sipua;

use sipua::*;


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
    cmb_device: gtk::ComboBoxText,
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

    pub fn init(&mut self) {
        // adjust slider
        self.sldr_level.set_range(0.0, 100.0);
        self.sldr_level.set_value(100.0);
        self.sldr_level.set_increments(1.0, 5.0);
        self.sldr_level.set_slider_size_fixed(true);
        self.sldr_level.set_round_digits(0);
        self.sldr_level.set_digits(0);

        self.btn_level_dec.connect_clicked(
          clone!( @weak self.sldr_level as sldr => move |_| {
              sldr.set_value(sldr.get_value() - 1.0);
          }));

        self.btn_level_inc.connect_clicked(
          clone!( @weak self.sldr_level as sldr => move |_| {
              sldr.set_value(sldr.get_value() + 1.0);
        }));

        self.cmb_device.remove_all();

    }

    // add device
    pub fn add_device_text(&mut self, name: &str){
        self.cmb_device.append_text(name);
    }

    pub fn clear_device_text(&mut self) {
        self.cmb_device.remove_all();
    }
}

pub struct MaintabWidget {
   btnbox: gtk::ButtonBox,
   stack: gtk::Stack, btn_sip: gtk::Button,
   btn_account: gtk::Button,
   btn_settings: gtk::Button,
   btn_codec: gtk::Button,
   btn_about: gtk::Button
}

impl MaintabWidget {

    pub fn new(gtk_builder: &gtk::Builder) -> MaintabWidget {

        MaintabWidget {
            btnbox: gtk_builder.get_object("btnbox_main").unwrap(),
            stack: gtk_builder.get_object("stack_main").unwrap(),
            btn_sip: gtk_builder.get_object("btn_main_sip").unwrap(),
            btn_account: gtk_builder.get_object("btn_main_account").unwrap(),
            btn_settings: gtk_builder.get_object("btn_main_settings").unwrap(),
            btn_codec: gtk_builder.get_object("btn_main_codec").unwrap(),
            btn_about: gtk_builder.get_object("btn_main_about").unwrap()
        }
    }


    pub fn init(&mut self) {

        self.btn_sip.connect_clicked(
          clone!( @weak self.stack as stk => move |_| {
              stk.set_visible_child_name("page0");
        }));

        self.btn_account.connect_clicked(
          clone!( @weak self.stack as stk => move |_| {
              stk.set_visible_child_name("page1");
        }));

        self.btn_settings.connect_clicked(
          clone!( @weak self.stack as stk => move |_| {
              stk.set_visible_child_name("page2");
        }));

        self.btn_codec.connect_clicked(
          clone!( @weak self.stack as stk => move |_| {
              stk.set_visible_child_name("page3");
        }));

        self.btn_about.connect_clicked(
          clone!( @weak self.stack as stk => move |_| {
              stk.set_visible_child_name("page4");
        }));
    }

}

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

pub struct HeaderbarWidget{
    cpu_lvl: gtk::LevelBar
}

impl HeaderbarWidget {

    pub fn new(gtk_builder: &gtk::Builder) -> HeaderbarWidget {
        HeaderbarWidget {
            cpu_lvl: gtk_builder.get_object("lvl_cpu").unwrap()
        }
    }

    pub fn init(&self) {

    }
}

pub struct DialpadWidget {
    btn_dial_1: gtk::Button,
    btn_dial_2: gtk::Button,
    btn_dial_3: gtk::Button,
    btn_dial_4: gtk::Button,
    btn_dial_5: gtk::Button,
    btn_dial_6: gtk::Button,
    btn_dial_7: gtk::Button,
    btn_dial_8: gtk::Button,
    btn_dial_9: gtk::Button,
    btn_dial_0: gtk::Button,
    btn_dial_ast: gtk::Button,
    btn_dial_hash: gtk::Button,
    btn_call: gtk::Button,
    btn_call_log_clear: gtk::Button,
    ent_call_address: gtk::Entry,
    tv_call_log: gtk::TreeView
}

impl DialpadWidget {
    pub fn new (gtk_builder: &gtk::Builder) -> DialpadWidget {
        DialpadWidget{
            btn_dial_1: gtk_builder.get_object("btn_dial_1").unwrap(),
            btn_dial_2: gtk_builder.get_object("btn_dial_2").unwrap(),
            btn_dial_3: gtk_builder.get_object("btn_dial_3").unwrap(),
            btn_dial_4: gtk_builder.get_object("btn_dial_4").unwrap(),
            btn_dial_5: gtk_builder.get_object("btn_dial_5").unwrap(),
            btn_dial_6: gtk_builder.get_object("btn_dial_6").unwrap(),
            btn_dial_7: gtk_builder.get_object("btn_dial_7").unwrap(),
            btn_dial_8: gtk_builder.get_object("btn_dial_8").unwrap(),
            btn_dial_9: gtk_builder.get_object("btn_dial_9").unwrap(),
            btn_dial_0: gtk_builder.get_object("btn_dial_0").unwrap(),
            btn_dial_ast: gtk_builder.get_object("btn_dial_ast").unwrap(),
            btn_dial_hash: gtk_builder.get_object("btn_dial_hash").unwrap(),
            btn_call: gtk_builder.get_object("btn_call").unwrap(),
            btn_call_log_clear: gtk_builder.get_object("btn_call_log_clear").unwrap(),
            ent_call_address: gtk_builder.get_object("ent_call_address").unwrap(),
            tv_call_log: gtk_builder.get_object("tv_call_log").unwrap()
        }
    }

    pub fn init(&mut self) {

        // dialpad 1 event
        self.btn_dial_1.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {

        }));

        // dialpad 2 event
        self.btn_dial_2.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {

        }));

        // dialpad 3 event
        self.btn_dial_3.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {

        }));

        // dialpad 4 event
        self.btn_dial_4.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {

        }));

        // dialpad 5 event
        self.btn_dial_5.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {

        }));

        // dialpad 6 event
        self.btn_dial_6.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {

        }));

        // dialpad 7 event
        self.btn_dial_7.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {

        }));

        // dialpad 8 event
        self.btn_dial_8.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {

        }));

        // dialpad 9 event
        self.btn_dial_9.connect_clicked(
            clone!( @weak self.ent_call_address as entry => move |_| {

        }));

        // dialpad 0 event
        self.btn_dial_0.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {

        }));

        // dialpad asterisk event
        self.btn_dial_ast.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {

        }));

        // dialpad hash event
        self.btn_dial_hash.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {

        }));

        // btn dialpad call event
        self.btn_call.connect_clicked ( |_| {

        });

        // btn dialpad call log clear event
        self.btn_call_log_clear.connect_clicked ( |_| {

        });

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
    let mut input_widget: TopbarWidget = TopbarWidget::new(&builder,
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
    let mut output_widget: TopbarWidget = TopbarWidget::new(&builder,
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

    let headerbar_widget: HeaderbarWidget = HeaderbarWidget::new(&builder);

    let dialpad_widget: DialpadWidget = DialpadWidget::new(&builder);

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
