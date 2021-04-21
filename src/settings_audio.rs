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
    swt_ec_algo: ComboBoxText
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
            swt_ec_algo: gtk_builder.get_object("cmb_audio_ec_algo").unwrap(),
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
}


impl HelperFileSettings for SettingsAudioWidget {
    fn load(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        todo!();
    }

    fn save(&self, path: PathBuf) {
        let mut config = Ini::new();
        config.load(path.to_str().unwrap()).unwrap();

        todo!();

        // config.write(path.to_str().unwrap()).unwrap();
    }
}

