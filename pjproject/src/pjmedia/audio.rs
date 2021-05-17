
use std::ffi::{CStr, IntoStringError};
use super::*;

pub trait MediaAudDevInfoExt {
    fn get_name(&self) -> Result<String, IntoStringError>;
    fn get_input_count(&self) -> u32;
    fn get_output_count(&self) -> u32;
    fn get_default_samples_per_sec(&self) -> u32;
    fn get_driver(&self) -> Result<String, IntoStringError>;
    fn get_caps(&self) -> u32;
    fn get_routes(&self) -> u32;
    fn get_ext_fmt_cnt(&self) -> u32;
    fn get_ext_fmt(&self) -> Vec<pjmedia_format>;

    // [pjmedia_format; 8usize];
}


impl MediaAudDevInfoExt for MediaAudDevInfo {

    fn get_name(&self) -> Result<String, IntoStringError> {
        unsafe {
            CStr::from_ptr(self.name.as_ptr())
            .to_owned().into_string()
        }
    }

    fn get_input_count(&self) -> u32 {
        self.input_count
    }

    fn get_output_count(&self) -> u32 {
        self.output_count
    }

    fn get_default_samples_per_sec(&self) -> u32 {
        self.default_samples_per_sec
    }

    fn get_driver(&self) -> Result<String, IntoStringError> {
        unsafe {
            CStr::from_ptr(self.driver.as_ptr())
            .to_owned().into_string()
        }
    }

    fn get_caps(&self) -> u32 {
        self.caps
    }

    fn get_routes(&self) -> u32 {
        self.routes
    }

    fn get_ext_fmt_cnt(&self) -> u32 {
        self.ext_fmt_cnt
    }

    fn get_ext_fmt(&self) -> Vec<pjmedia_format> {
        todo!()
    }
}


pub trait MediaSndDevInfoExt {
    fn get_name(&self) -> Result<String, IntoStringError>;
    fn get_input_count(&self) -> u32;
    fn get_output_count(&self) -> u32;
    fn get_default_samples_per_sec(&self) -> u32;
}


impl MediaSndDevInfoExt for MediaSndDevInfo {

    fn get_name(&self) -> Result<String, IntoStringError> {
        unsafe {
            CStr::from_ptr(self.name.as_ptr()).to_owned().into_string()
        }
    }

    fn get_input_count(&self) -> u32 {
        self.input_count
    }

    fn get_output_count(&self) -> u32 {
        self.output_count
    }

    fn get_default_samples_per_sec(&self) -> u32 {
        self.default_samples_per_sec
    }
}