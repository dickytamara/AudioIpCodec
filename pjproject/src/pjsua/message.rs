

use pjsip_sys::{pjsip_hdr, pjsip_media_type, pjsip_multipart_part};
use super::*;


pub trait MessageDataExt {

    /// Optional remote target URI (i.e. Target header). If NULL, the target will be set to the
    /// remote URI (To header). This field is used by pjsua_call_make_call(), pjsua_im_send(),
    /// pjsua_call_reinvite(), pjsua_call_set_hold(), and pjsua_call_update().
    fn set_target_uri (&mut self, value: String);
    fn get_target_uri (&self) -> String;

    /// Additional message headers as linked list. Application can add headers to the list by
    /// creating the header, either from the heap/pool or from temporary local variable, and add
    /// the header using linked list operation. See pjsua_app.c for some sample codes.
    fn set_hdr_list (&mut self, value: pjsip_hdr);
    fn get_hdr_list (&self) -> &pjsip_hdr;

    /// MIME type of optional message body.
    fn set_content_type (&mut self, value: String);
    fn get_content_type (&self) -> String;

    /// Optional message body to be added to the message, only when the message doesn't have a
    /// body.
    fn set_msg_body (&mut self, value: String);
    fn get_msg_body (&self) -> String;

    /// Content type of the multipart body. If application wants to send multipart message bodies,
    /// it puts the parts in parts and set the content type in multipart_ctype. If the message
    /// already contains a body, the body will be added to the multipart bodies.
    fn set_multipart_ctype (&mut self, value: pjsip_media_type);
    fn get_multipart_ctype (&self) -> pjsip_media_type;

    /// List of multipart parts. If application wants to send multipart message bodies, it puts the
    /// parts in parts and set the content type in multipart_ctype. If the message already contains
    /// a body, the body will be added to the multipart bodies.
    fn get_multipart_parts (&mut self) -> pjsip_multipart_part;
    fn set_multipart_parts (&self, value: pjsip_multipart_part);

}

impl MessageDataExt for MessageData {

    fn set_target_uri (&mut self, value: String) {
        self.target_uri = pj_str_t::from_string(value);
    }

    fn get_target_uri (&self) -> String {
        self.target_uri.to_string()
    }

    fn set_hdr_list (&mut self, value: pjsip_hdr) {
        self.hdr_list = value;
    }

    fn get_hdr_list (&self) -> &pjsip_hdr {
        &self.hdr_list
    }

    fn set_content_type (&mut self, value: String) {
        self.content_type = pj_str_t::from_string(value);
    }

    fn get_content_type (&self) -> String {
        self.content_type.to_string()
    }

    fn set_msg_body (&mut self, value: String) {
        self.content_type = pj_str_t::from_string(value);
    }

    fn get_msg_body (&self) -> String {
        self.content_type.to_string()
    }

    fn set_multipart_ctype (&mut self, value: pjsip_media_type) {
        todo!()
    }

    fn get_multipart_ctype (&self) -> pjsip_media_type {
        todo!()
    }

    fn get_multipart_parts (&mut self) -> pjsip_multipart_part {
        todo!()
    }

    fn set_multipart_parts (&self, value: pjsip_multipart_part) {
        todo!()
    }
}