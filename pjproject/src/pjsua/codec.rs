use super::*;

pub trait UACodecInfoExt {

    /// Codec unique identification.
    fn get_codec_id (&self) -> String;


    /// Codec priority (integer 0-255).
    fn get_priority (&self) -> u8;

    ///Codec description.
    fn get_desc (&self) -> String;
    // fn get_buf_ (&self) -> [::std::os::raw::c_char; 64usize],
}



impl UACodecInfoExt for UACodecInfo  {
    fn get_codec_id (&self) -> String {
        self.codec_id.to_string()
    }
    fn get_priority (&self) -> u8 {
        self.priority
    }
    fn get_desc (&self) -> String {
        self.desc.to_string()
    }
}

