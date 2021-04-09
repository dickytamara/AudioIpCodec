


use gtk::prelude::*;
use gtk::{Builder, Label, Entry, Button, TreeView, TreeSelection, CellRendererText, ListStore, TreeViewColumn};
use glib::clone;
use std::cell::RefCell;


pub enum CallButtonState {
    Call,
    Answer,
    Abort,
    Hangup,
    None
}


#[derive(Clone)]
pub struct DialpadStorage {
    btn_dial_1: Button,
    btn_dial_2: Button,
    btn_dial_3: Button,
    btn_dial_4: Button,
    btn_dial_5: Button,
    btn_dial_6: Button,
    btn_dial_7: Button,
    btn_dial_8: Button,
    btn_dial_9: Button,
    btn_dial_0: Button,
    btn_dial_ast: Button,
    btn_dial_hash: Button,
    btn_call: Button,
    btn_call_log_clear: Button,
    lbl_call_address: Label,
    btn_call_address_clear: Button,
    ent_call_address: Entry,
    tv_call_log: TreeView,
    tv_call_log_selection: TreeSelection,
    ls_call_log: gtk::ListStore
}

impl DialpadStorage {

    pub fn new (gtk_builder: &Builder) -> Self {
        // liststore types
        let list_types = [u32::static_type(), String::static_type()];

        DialpadStorage{
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
            tv_call_log_selection: gtk_builder.get_object("tv_call_log_selection").unwrap(),
            ls_call_log: ListStore::new(&list_types)
        }
    }
}

unsafe impl Send for DialpadWidget {}

#[derive(Clone)]
pub struct DialpadWidget {
    ctx: RefCell<DialpadStorage>,
}

impl DialpadWidget {

    pub fn new(gtk_builder: &Builder) -> Self {
        let result = DialpadWidget {
            // inner data just borrow not mutate
            ctx: RefCell::new(DialpadStorage::new(gtk_builder))
        };

        // fix this
        result.init();

        result
    }

    pub fn init(&self) {

        let widget = self.ctx.borrow();
        // col number
        {
            let col = TreeViewColumn::new();
            let cell = CellRendererText::new();
            col.pack_start(&cell, false);
            col.set_title("NO");
            col.add_attribute(&cell, "text", 0);
            widget.tv_call_log.append_column(&col);
        }

        // col address
        {
            let col = TreeViewColumn::new();
            let cell = CellRendererText::new();
            col.pack_start(&cell, true);
            col.set_title("Address");
            col.add_attribute(&cell, "text", 1);
            widget.tv_call_log.append_column(&col);
        }

        widget.tv_call_log.set_model(Some(&widget.ls_call_log));
        widget.tv_call_log.set_headers_visible(true);

        widget.btn_call_log_clear.set_label(format!("\nClear\n").as_str());

        // update to normal state
        self.update_state_normal();

        // dialpad 1 event
        widget.btn_dial_1.connect_clicked (
            clone!( @weak widget.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "1"));
        }));

        // dialpad 2 event
        widget.btn_dial_2.connect_clicked (
            clone!( @weak widget.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "2"));
        }));

        // dialpad 3 event
        widget.btn_dial_3.connect_clicked (
            clone!( @weak widget.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "3"));
        }));

        // dialpad 4 event
        widget.btn_dial_4.connect_clicked (
            clone!( @weak widget.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "4"));
        }));

        // dialpad 5 event
        widget.btn_dial_5.connect_clicked (
            clone!( @weak widget.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "5"));
        }));

        // dialpad 6 event
        widget.btn_dial_6.connect_clicked (
            clone!( @weak widget.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "6"));
        }));

        // dialpad 7 event
        widget.btn_dial_7.connect_clicked (
            clone!( @weak widget.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "7"));
        }));

        // dialpad 8 event
        widget.btn_dial_8.connect_clicked (
            clone!( @weak widget.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "8"));
        }));

        // dialpad 9 event
        widget.btn_dial_9.connect_clicked(
            clone!( @weak widget.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "9"));
        }));

        // dialpad 0 event
        widget.btn_dial_0.connect_clicked (
            clone!( @weak widget.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "0"));
        }));

        // dialpad asterisk event
        widget.btn_dial_ast.connect_clicked (
            clone!( @weak widget.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "*"));
        }));

        // dialpad hash event
        widget.btn_dial_hash.connect_clicked (
            clone!( @weak widget.ent_call_address as entry => move |_| {
                entry.set_text(&format!("{}{}", entry.get_text(), "#"));
        }));

        widget.btn_call_address_clear.connect_clicked(
            clone!( @weak widget.ent_call_address as entry => move |_| {
                entry.set_text("");
        }));

        // btn dialpad call log clear event
        widget.btn_call_log_clear.connect_clicked (
            clone!( @weak widget.ls_call_log as list => move |_| {
                list.clear();
            }));

        //self.ent_call_address.set_events(EventMask::BUTTON2_MOTION_MASK);
    }

    ///  add item to call log list
    pub fn add_call_log(&self, call_address: &str) {

        let widget = self.ctx.borrow();

        let call_address: String = String::from(call_address);
        let num = widget.ls_call_log.iter_n_children(None) + 1;
        let nrow: [&dyn ToValue; 2] = [
            &num,
            &call_address
        ];

        widget.ls_call_log.set(&widget.ls_call_log.append(),
            &[0u32, 1u32],
            &nrow
        );
    }

    /// get call adress
    pub fn get_call_address_text(&self) -> String {
        String::from(self.ctx.borrow().ent_call_address.get_text().as_str())
    }

    /// set call address
    pub fn set_call_address_text(&self, call_address: String) {
        self.ctx.borrow().ent_call_address.set_text(call_address.as_str());
    }

    /// event on button call clicked
    pub fn on_button_call_clicked<F: Fn(&str, CallButtonState) + 'static> (&self, callback: F) {
        let wid = self.clone();
        self.ctx.borrow().btn_call.connect_clicked( move | b | {

            let sip_address = wid.get_call_address_text().clone();
            let b_str = b.get_label().unwrap().to_string();

            let state = match b_str.as_str() {
                "\nCall\n" => CallButtonState::Call,
                "\nAnswer\n" => CallButtonState::Answer,
                "\nAbort\n" => CallButtonState::Abort,
                "\nHangup\n" => CallButtonState::Hangup,
                _ => CallButtonState::None

            };

            callback(sip_address.as_str(), state);
        });
    }

    /// clear log external code
    pub fn clear_log(&self) {
        self.ctx.borrow().ls_call_log.clear();
    }

    /// update gui to normal state
    pub fn update_state_normal(&self) {
        self.ctx.borrow().btn_call.set_label(format!("\nCall\n").as_str());
    }

    /// update gui to ringing state
    pub fn update_state_incoming(&self) {
        self.ctx.borrow().btn_call.set_label(format!("\nAnswer\n").as_str());
    }

    /// update gui to calling state
    pub fn update_state_outgoing(&self) {
        self.ctx.borrow().btn_call.set_label(format!("\nAbort\n").as_str());
    }

    /// update gui to confirmed call state
    pub fn update_state_oncall(&self) {
        self.ctx.borrow().btn_call.set_label(format!("\nHangup\n").as_str());
    }
}

