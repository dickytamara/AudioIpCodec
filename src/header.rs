
use glib::timeout_add_seconds;

use super::gtk::prelude::*;
use super::gtk::prelude::*;

use super::glib::MainContext;

use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::cell::RefCell;
use super::systemstat::{System, Platform, saturating_sub_bytes};

enum SystemLoadAverage {
    UpdateCpuAverage(f32)
}

pub struct HeaderStorage{
    cpu_lvl: gtk::LevelBar,
}

impl HeaderStorage {
    pub fn new (gtk_builder: &gtk::Builder) -> Self {
        HeaderStorage {
            cpu_lvl: gtk_builder.get_object("lvl_cpu").unwrap(),
        }
    }
}

pub struct HeaderWidget {
    ctx: RefCell<HeaderStorage>
}

impl HeaderWidget {

    pub fn new(gtk_builder: &gtk::Builder) -> Self {
        let result = HeaderWidget {
            ctx: RefCell::new(HeaderStorage::new(gtk_builder))
        };

        // start update on thread
        let (sender, receiver) = MainContext::channel(glib::PRIORITY_DEFAULT);
        thread::spawn(move || {
            loop {
                let sys = System::new();
                let mut cpuload: f32 = 0.0;
                match sys.cpu_load_aggregate() {
                    Ok(cpu) => {
                        thread::sleep(Duration::from_millis(80));
                        let cpu = cpu.done().unwrap();
                        // cpuload = (cpu.user + cpu.system) * 100.0;
                        // or
                        cpuload = 100.0 - (cpu.idle * 100.0);
                        // println!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                        //             cpu.user * 100.0, cpu.nice * 100.0,
                        //             cpu.system * 100.0, cpu.interrupt * 100.0,
                        //             cpu.idle * 100.0);
                    },
                    Err(x) => {
                        println!("get system cpu error: {}", x);
                    }
                }

                // send message on thread
                let _ = sender.send(SystemLoadAverage::UpdateCpuAverage(cpuload));
            }
        });

        let cpu_level = result.ctx.borrow().cpu_lvl.clone();
            receiver.attach(None, move |msg| {
                match msg {
                    SystemLoadAverage::UpdateCpuAverage(x) => {
                        //println!("cpu_level : {}", x);
                        cpu_level.set_value(x as f64);
                    }
                }

                glib::Continue(true)
        });

        result
    }
}
