
use super::gtk::prelude::*;
use super::gtk::{Builder, Label, LevelBar, Button, Scale, ComboBoxText};
use super::glib::clone;

pub struct AudioLineWidget {
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

impl AudioLineWidget {

    pub fn new(gtk_builder: &gtk::Builder,
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
      ) -> AudioLineWidget {
        AudioLineWidget{
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

    // add device to combobox
    pub fn add_device_text(&mut self, name: &str){
        self.cmb_device.append_text(name);
    }

    // clear combobox device
    pub fn clear_device_text(&mut self) {
        self.cmb_device.remove_all();
    }

    pub fn on_scale_changed_value<F: Fn(i32) + 'static>(&self, call: F) {
        self.sldr_level.connect_value_changed(move |s| {
            call(s.get_value() as i32);
        });
    }

}
