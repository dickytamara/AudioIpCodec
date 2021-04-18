use gtk::prelude::*;
use gtk::{ScrolledWindow, Button, TreeView, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;


#[derive(Clone)]
pub struct CodecStorage {
    scroll_window: ScrolledWindow,
    tv_codec: TreeView,
    btn_apply: Button,
    btn_save: Button,
    btn_reset: Button,
}


impl CodecStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        CodecStorage{
            scroll_window: gtk_builder.get_object("scwind_codec").unwrap(),
            tv_codec: gtk_builder.get_object("tv_codec").unwrap(),
            btn_apply: gtk_builder.get_object("btn_codec_apply").unwrap(),
            btn_save: gtk_builder.get_object("btn_codec_save").unwrap(),
            btn_reset: gtk_builder.get_object("btn_codec_reset").unwrap(),
        }
    }
}


#[derive(Clone)]
pub struct CodecWidget {
    ctx: RefCell<CodecStorage>
}


impl CodecWidget {
    pub fn new(gtk_builder: &Builder) -> Self {
        let result = CodecWidget {
            ctx: RefCell::new(CodecStorage::new(gtk_builder)),
        };


        let result_clone = result.clone();
        result.ctx.borrow().btn_reset.connect_clicked(move |_| {
            result_clone.reset();
        });

        result
    }

    pub fn reset(&self) {
        todo!();
    }


    pub fn connect_apply<F> (&self, callback: F)
    where
        F : Fn() + 'static
    {
        self.ctx.borrow().btn_apply.connect_clicked(move |_| {
            (callback)();
        });
    }

    pub fn connect_save<F> (&self, callback: F)
    where
        F : Fn() + 'static
    {
        self.ctx.borrow().btn_save.connect_clicked(move |_| {
            (callback)()
        });
    }

}


impl HelperFileSettings for CodecWidget {
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