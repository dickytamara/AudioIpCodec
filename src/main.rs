

extern crate pjsua_sys;
extern crate pj_sys;

mod pjdefault;
mod pjlib;
mod pjsip;
mod pjmedia;
mod pjsua;
mod sipua;

use sipua::*;
//use pjdefault::AutoCreate;





fn main() {
    let mut sip = SIPUserAgent::new();
    let mut ln = String::new();
    sip.start();
    std::io::stdin().read_line(&mut ln).unwrap();
}
