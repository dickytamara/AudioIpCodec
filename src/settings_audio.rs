use gtk::prelude::*;
use gtk::{Label, SpinButton, ComboBoxText, Switch, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;



#[derive(Clone)]
pub struct SettingsAudioStorage {
    lbl_max_jitter: Label,
    lbl_ptime: Label,
    lbl_quality: Label,
    lbl_no_vad: Label,
    lbl_ec_tail: Label,
    lbl_ec_algo: Label,
    spn_max_jitter: SpinButton,
    spn_ptime: SpinButton,
    spn_quality: SpinButton,
    swt_no_vad: Switch,
    spn_ec_tail: SpinButton,
    cmb_ec_algo: ComboBoxText
}

impl SettingsAudioStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsAudioStorage {
            lbl_max_jitter: gtk_builder.get_object("lbl_audio_max_jitter").unwrap(),
            lbl_ptime: gtk_builder.get_object("lbl_audio_ptime").unwrap(),
            lbl_quality: gtk_builder.get_object("lbl_audio_quality").unwrap(),
            lbl_no_vad: gtk_builder.get_object("lbl_audio_no_vad").unwrap(),
            lbl_ec_tail: gtk_builder.get_object("lbl_audio_ec_tail").unwrap(),
            lbl_ec_algo: gtk_builder.get_object("lbl_audio_ec_algo").unwrap(),
            spn_max_jitter: gtk_builder.get_object("spn_audio_max_jitter").unwrap(),
            spn_ptime: gtk_builder.get_object("spn_audio_ptime").unwrap(),
            spn_quality: gtk_builder.get_object("spn_audio_quality").unwrap(),
            swt_no_vad: gtk_builder.get_object("swt_audio_no_vad").unwrap(),
            spn_ec_tail: gtk_builder.get_object("spn_audio_ec_tail").unwrap(),
            cmb_ec_algo: gtk_builder.get_object("cmb_audio_ec_algo").unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct SettingsAudioWidget {
    // inner data just borrow not mutate
    ctx: RefCell<SettingsAudioStorage>
}

impl SettingsAudioWidget {
    pub fn new(gtk_builder: &Builder) -> Self {
        let result = SettingsAudioWidget{
            ctx: RefCell::new(SettingsAudioStorage::new(gtk_builder)),
        };

        result
    }

    pub fn reset(&self) {
        todo!();
    }

    pub fn set_jb_max(&self, value: f64) {
        self.ctx.borrow().spn_max_jitter.set_value(value);
    }

    pub fn get_jb_max(&self) -> f64 {
        self.ctx.borrow().spn_max_jitter.get_value()
    }

    pub fn set_ptime(&self, value: f64) {
        self.ctx.borrow().spn_ptime.set_value(value);
    }

    pub fn get_ptime(&self) -> f64 {
        self.ctx.borrow().spn_ptime.get_value()
    }

    pub fn set_quality(&self, value: f64) {
        self.ctx.borrow().spn_quality.set_value(value);
    }

    pub fn get_quality(&self) -> f64 {
        self.ctx.borrow().spn_quality.get_value()
    }

    pub fn set_no_vad(&self, value: bool) {
        self.ctx.borrow().swt_no_vad.set_state(value);
    }

    pub fn get_no_vad(&self) -> bool {
        self.ctx.borrow().swt_no_vad.get_state()
    }

    pub fn set_ec_tail_len(&self, value: f64) {
        self.ctx.borrow().spn_ec_tail.set_value(value);
    }

    pub fn get_ec_tail_len(&self) -> f64 {
        self.ctx.borrow().spn_ec_tail.get_value()
    }

    pub fn set_ec_options(&self, value: u32) {
        self.ctx.borrow().cmb_ec_algo.set_active(Some(value -1));
    }

    pub fn get_ec_options(&self) -> u32 {
        match self.ctx.borrow().cmb_ec_algo.get_active() {
            Some(value) => value +1,
            None => 0
        }
    }

}


impl HelperFileSettings for SettingsAudioWidget {
    fn load(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        self.set_jb_max(config.get("audio", "jb_max").unwrap().parse().unwrap());
        self.set_ptime(config.get("audio", "ptime").unwrap().parse().unwrap());
        self.set_quality(config.get("audio", "quality").unwrap().parse().unwrap());
        self.set_no_vad(config.get("audio", "no_vad").unwrap().parse().unwrap());
        self.set_ec_tail_len(config.get("audio", "ec_tail_len").unwrap().parse().unwrap());
        self.set_ec_options(config.get("audio", "ec_options").unwrap().parse().unwrap());
    }

    fn save(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        config.set("audio", "jb_max", Some(self.get_jb_max().to_string()));
        config.set("audio", "ptime", Some(self.get_ptime().to_string()));
        config.set("audio", "quality", Some(self.get_quality().to_string()));
        config.set("audio", "no_vad", Some(self.get_no_vad().to_string()));
        config.set("audio", "ec_tail_len", Some(self.get_ec_tail_len().to_string()));
        config.set("audio", "ec_options", Some(self.get_ec_options().to_string()));

        config.write(path.to_str().unwrap()).unwrap();
    }
}

