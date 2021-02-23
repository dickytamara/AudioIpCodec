
use super::gtk::prelude::*;
use super::glib::clone;

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
