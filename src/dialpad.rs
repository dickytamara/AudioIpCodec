
use gio::ListStore;
use gtk::{TreeModelExt, TreeViewColumn};

use super::gtk::prelude::*;
use super::gtk::{Builder, Label, Entry, Button, TreeView, CellRendererText};
use super::glib::clone;

#[derive(Clone)]
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
    lbl_call_address: gtk::Label,
    btn_call_address_clear: gtk::Button,
    ent_call_address: gtk::Entry,
    tv_call_log: gtk::TreeView,
    ls_call_log: gtk::ListStore
}

impl DialpadWidget {

    pub fn new (gtk_builder: &gtk::Builder) -> DialpadWidget {
        // liststore types
        let list_types = [u32::static_type(), String::static_type()];

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
            lbl_call_address: gtk_builder.get_object("lbl_call_address").unwrap(),
            btn_call_address_clear: gtk_builder.get_object("btn_call_address_clear").unwrap(),
            ent_call_address: gtk_builder.get_object("ent_call_address").unwrap(),
            tv_call_log: gtk_builder.get_object("tv_call_log").unwrap(),
            ls_call_log: gtk::ListStore::new(&list_types)
        }
    }

    pub fn init(&mut self) {

        // col number
        {
            let col = TreeViewColumn::new();
            let cell = CellRendererText::new();
            col.pack_start(&cell, false);
            col.set_title("NO");
            col.add_attribute(&cell, "text", 0);
            self.tv_call_log.append_column(&col);
        }

        // col address
        {
            let col = TreeViewColumn::new();
            let cell = CellRendererText::new();
            col.pack_start(&cell, true);
            col.set_title("Address");
            col.add_attribute(&cell, "text", 1);
            self.tv_call_log.append_column(&col);
        }

        self.tv_call_log.set_model(Some(&self.ls_call_log));
        self.tv_call_log.set_headers_visible(true);

        // dialpad 1 event
        self.btn_dial_1.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "1"));
        }));

        // dialpad 2 event
        self.btn_dial_2.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "2"));
        }));

        // dialpad 3 event
        self.btn_dial_3.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "3"));
        }));

        // dialpad 4 event
        self.btn_dial_4.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "4"));
        }));

        // dialpad 5 event
        self.btn_dial_5.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "5"));
        }));

        // dialpad 6 event
        self.btn_dial_6.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "6"));
        }));

        // dialpad 7 event
        self.btn_dial_7.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "7"));
        }));

        // dialpad 8 event
        self.btn_dial_8.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "8"));
        }));

        // dialpad 9 event
        self.btn_dial_9.connect_clicked(
            clone!( @weak self.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "9"));
        }));

        // dialpad 0 event
        self.btn_dial_0.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "0"));
        }));

        // dialpad asterisk event
        self.btn_dial_ast.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "*"));
        }));

        // dialpad hash event
        self.btn_dial_hash.connect_clicked (
            clone!( @weak self.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "#"));
        }));

        self.btn_call_address_clear.connect_clicked(
            clone!( @weak self.ent_call_address as entry => move |_| {
                entry.set_text("");
        }));

        // btn dialpad call log clear event
        self.btn_call_log_clear.connect_clicked ( |_| {

        });

        // main button event for calling
        self.btn_call_address_clear.connect_clicked(
            clone!( @weak self.ent_call_address as enty => move |_| {

        }));
    }

    ///  add item to call log list
    pub fn add_call_log(&mut self, call_address: &str) {

        let call_address: String = String::from(call_address);
        let num = self.ls_call_log.iter_n_children(None) + 1;
        let nrow: [&dyn ToValue; 2] = [
            &num,
            &call_address
        ];

        self.ls_call_log.set(&self.ls_call_log.append(),
            &[0u32, 1u32],
            &nrow
        );
    }

    /// get call adress
    pub fn get_call_address_text(&self) -> String {
        String::from(self.ent_call_address.get_text().as_str())
    }

    /// event on button call clicked
    pub fn on_button_call_clicked<F: Fn(&str) + 'static> (&self, callback: F) {
        let wid = self.clone();

        self.btn_call.connect_clicked( move |_| {
            let sip_address = wid.get_call_address_text().clone();
            callback(sip_address.as_str());
        });
    }

    /// update gui to normal state
    pub fn update_state_normal(&self) {

    }

    /// update gui to ringing state
    pub fn update_state_ringing(&self) {

    }

    /// update gui to calling state
    pub fn update_state_calling(&self) {

    }

}
