
use gtk::prelude::*;
use gtk::{Label, LevelBar, Button, Scale, ComboBoxText, ToggleButton, Builder};
use glib::clone;

use std::cell::RefCell;

#[derive(Clone)]
pub struct AudioLineStorage {
    lbl_topbar: Label,
    lbl_level_l: Label,
    lbl_level_r: Label,
    lvl_l: LevelBar,
    lvl_r: LevelBar,
    btn_level_dec: Button,
    btn_level_inc: Button,
    sldr_level: Scale,
    lbl_device: Label,
    cmb_device: ComboBoxText,
    btn_mute: ToggleButton
}

impl AudioLineStorage {
    pub fn new (gtk_builder: &Builder,
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
        btn_mute_id: &str) -> Self
    {
        AudioLineStorage{
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

#[derive(Clone)]
pub struct AudioLineWidget {
    // inner data just borrow not mutate
    ctx: RefCell<AudioLineStorage>
}

impl AudioLineWidget {

    pub fn new(gtk_builder: &Builder,
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
    ) -> Self {
        let result = AudioLineWidget{
            ctx: RefCell::new(AudioLineStorage::new(
                gtk_builder, lbl_topbar_id, lbl_level_l_id, lbl_level_r_id, lvl_l_id, lvl_r_id,
                btn_level_dec_id, btn_level_inc_id, sldr_level_id, lbl_device_id, cmb_device_id, btn_mute_id
            ))
        };

        // adjust level bar
        result.ctx.borrow().lvl_l.set_max_value(128.0);
        result.ctx.borrow().lvl_l.set_min_value(0.0);

        result.ctx.borrow().lvl_r.set_max_value(128.0);
        result.ctx.borrow().lvl_r.set_min_value(0.0);

        // adjust slider
        result.ctx.borrow().sldr_level.set_range(0.0, 100.0);
        result.ctx.borrow().sldr_level.set_value(100.0);
        result.ctx.borrow().sldr_level.set_increments(1.0, 5.0);
        result.ctx.borrow().sldr_level.set_slider_size_fixed(true);
        result.ctx.borrow().sldr_level.set_round_digits(0);
        result.ctx.borrow().sldr_level.set_digits(0);

        let sldr_level = result.ctx.borrow().sldr_level.clone();
        result.ctx.borrow().btn_level_dec.connect_clicked(
        clone!( @weak sldr_level as sldr => move |_| {
            sldr.set_value(sldr.get_value() - 1.0);
        }));

        let sldr_level = result.ctx.borrow().sldr_level.clone();
        result.ctx.borrow().btn_level_inc.connect_clicked(
        clone!( @weak sldr_level as sldr => move |_| {
            sldr.set_value(sldr.get_value() + 1.0);
        }));


        result
    }

    // set audio level status
    pub fn set_level_bar(&self, left: u32, right: u32) {
        self.ctx.borrow().lvl_l.set_value(left as f64);
        self.ctx.borrow().lvl_r.set_value(right as f64);
    }

    // add device to combobox
    pub fn add_device_text(&self, name: &str){
        self.ctx.borrow().cmb_device.append_text(name);
    }

    // clear combobox device
    pub fn clear_device_text(&self) {
        self.ctx.borrow().cmb_device.remove_all();
    }

    /// on slider level change value
    pub fn on_scale_changed_value<F> (&self, call: F)
    where
        F: Fn(i32) + 'static
    {
        self.ctx.borrow().sldr_level.connect_value_changed(move |s| {
            call(s.get_value() as i32);
        });
    }

    /// on button mute clicked
    pub fn on_button_mute_clicked<F> (&self, callback: F)
    where
        F: Fn(bool) + 'static
    {
        let widget = self.ctx.borrow();
        widget.btn_mute.connect_toggled(
            clone!(@weak widget.btn_mute as mute => move | btn | {
                let state = btn.get_active();
                callback(state);
        }));
    }

}

/// create transmit widget
pub fn create_transmit_widget(builder: &Builder) -> AudioLineWidget {
    AudioLineWidget::new(builder,
       "lbl_topbar_tx",
       "lbl_tx_l",
       "lbl_tx_r",
       "lvl_tx_l",
       "lvl_tx_r",
       "btn_tx_level_dec",
       "btn_tx_level_inc",
       "sldr_tx_level",
       "lbl_tx_device",
       "cmb_tx_device",
       "btn_tx_mute"
       )
}

/// create receive widget
pub fn create_receive_widget(builder: &Builder) -> AudioLineWidget {
    AudioLineWidget::new(builder,
        "lbl_topbar_rx",
        "lbl_rx_l",
        "lbl_rx_r",
        "lvl_rx_l",
        "lvl_rx_r",
        "btn_rx_level_dec",
        "btn_rx_level_inc",
        "sldr_rx_level",
        "lbl_rx_device",
        "cmb_rx_device",
        "btn_rx_mute"
     )
}






