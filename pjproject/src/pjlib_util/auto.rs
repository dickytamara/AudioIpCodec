
use pjlib_util_sys::*;
use crate::prelude::AutoCreate;

impl AutoCreate<pj_getopt_option> for pj_getopt_option  {}
impl AutoCreate<pj_crc32_context> for pj_crc32_context  {}
impl AutoCreate<pj_md5_context> for pj_md5_context  {}
impl AutoCreate<pj_hmac_md5_context> for pj_hmac_md5_context  {}
impl AutoCreate<pj_sha1_context> for pj_sha1_context  {}
impl AutoCreate<pj_hmac_sha1_context> for pj_hmac_sha1_context  {}
impl AutoCreate<pj_dns_hdr> for pj_dns_hdr  {}
impl AutoCreate<pj_dns_parsed_query> for pj_dns_parsed_query  {}
impl AutoCreate<pj_dns_parsed_rr> for pj_dns_parsed_rr  {}
impl AutoCreate<pj_dns_parsed_rr_rdata> for pj_dns_parsed_rr_rdata  {}
impl AutoCreate<pj_dns_parsed_rr_rdata_srv> for pj_dns_parsed_rr_rdata_srv  {}
impl AutoCreate<pj_dns_parsed_rr_rdata_cname> for pj_dns_parsed_rr_rdata_cname  {}
impl AutoCreate<pj_dns_parsed_rr_rdata_ns> for pj_dns_parsed_rr_rdata_ns  {}
impl AutoCreate<pj_dns_parsed_rr_rdata_ptr> for pj_dns_parsed_rr_rdata_ptr  {}
impl AutoCreate<pj_dns_parsed_rr_rdata_a> for pj_dns_parsed_rr_rdata_a  {}
impl AutoCreate<pj_dns_parsed_rr_rdata_aaaa> for pj_dns_parsed_rr_rdata_aaaa  {}
impl AutoCreate<pj_dns_parsed_packet> for pj_dns_parsed_packet  {}
impl AutoCreate<pj_dns_resolver> for pj_dns_resolver  {}
impl AutoCreate<pj_dns_async_query> for pj_dns_async_query  {}
impl AutoCreate<pj_dns_settings> for pj_dns_settings  {}
impl AutoCreate<pj_dns_a_record> for pj_dns_a_record  {}
impl AutoCreate<pj_dns_addr_record> for pj_dns_addr_record  {}
impl AutoCreate<pj_dns_addr_record__bindgen_ty_1> for pj_dns_addr_record__bindgen_ty_1  {}
impl AutoCreate<pj_dns_addr_record__bindgen_ty_1__bindgen_ty_1> for pj_dns_addr_record__bindgen_ty_1__bindgen_ty_1  {}
impl AutoCreate<pj_dns_srv_record> for pj_dns_srv_record  {}
impl AutoCreate<pj_dns_srv_record__bindgen_ty_1> for pj_dns_srv_record__bindgen_ty_1  {}
impl AutoCreate<pj_dns_srv_async_query> for pj_dns_srv_async_query  {}
impl AutoCreate<pj_dns_server> for pj_dns_server  {}
impl AutoCreate<pj_cis_buf_t> for pj_cis_buf_t  {}
impl AutoCreate<pj_cis_t> for pj_cis_t  {}
impl AutoCreate<pj_scanner> for pj_scanner  {}
impl AutoCreate<pj_scan_state> for pj_scan_state  {}
impl AutoCreate<pj_xml_attr> for pj_xml_attr  {}
impl AutoCreate<pj_xml_node_head> for pj_xml_node_head  {}
impl AutoCreate<pj_xml_node> for pj_xml_node  {}
impl AutoCreate<pj_json_list> for pj_json_list  {}
impl AutoCreate<pj_json_elem> for pj_json_elem  {}
impl AutoCreate<pj_json_elem__bindgen_ty_1> for pj_json_elem__bindgen_ty_1  {}
impl AutoCreate<pj_json_err_info> for pj_json_err_info  {}
impl AutoCreate<pjstun_msg_hdr> for pjstun_msg_hdr  {}
impl AutoCreate<pjstun_attr_hdr> for pjstun_attr_hdr  {}
impl AutoCreate<pjstun_mapped_addr_attr> for pjstun_mapped_addr_attr  {}
impl AutoCreate<pjstun_change_request_attr> for pjstun_change_request_attr  {}
impl AutoCreate<pjstun_username_attr> for pjstun_username_attr  {}
impl AutoCreate<pjstun_error_code_attr> for pjstun_error_code_attr  {}
impl AutoCreate<pjstun_msg> for pjstun_msg  {}
impl AutoCreate<pjstun_setting> for pjstun_setting  {}
impl AutoCreate<pj_pcap_udp_hdr> for pj_pcap_udp_hdr  {}
impl AutoCreate<pj_pcap_filter> for pj_pcap_filter  {}
impl AutoCreate<pj_pcap_file> for pj_pcap_file  {}
impl AutoCreate<pj_http_req> for pj_http_req  {}
impl AutoCreate<pj_http_header_elmt> for pj_http_header_elmt  {}
impl AutoCreate<pj_http_headers> for pj_http_headers  {}
impl AutoCreate<pj_http_auth_cred> for pj_http_auth_cred  {}
impl AutoCreate<pj_http_req_param> for pj_http_req_param  {}
impl AutoCreate<pj_http_req_param_pj_http_reqdata> for pj_http_req_param_pj_http_reqdata  {}
impl AutoCreate<pj_http_auth_chal> for pj_http_auth_chal  {}
impl AutoCreate<pj_http_resp> for pj_http_resp  {}
impl AutoCreate<pj_http_url> for pj_http_url  {}
impl AutoCreate<pj_http_req_callback> for pj_http_req_callback  {}
impl AutoCreate<pj_cli_t> for pj_cli_t  {}
impl AutoCreate<pj_cli_cfg> for pj_cli_cfg  {}
impl AutoCreate<pj_cli_cmd_spec> for pj_cli_cmd_spec  {}
impl AutoCreate<pj_cli_arg_spec> for pj_cli_arg_spec  {}
impl AutoCreate<pj_cli_cmd_val> for pj_cli_cmd_val  {}
impl AutoCreate<pj_cli_hint_info> for pj_cli_hint_info  {}
impl AutoCreate<pj_cli_exec_info> for pj_cli_exec_info  {}
impl AutoCreate<pj_cli_arg_choice_val> for pj_cli_arg_choice_val  {}
impl AutoCreate<pj_cli_dyn_choice_param> for pj_cli_dyn_choice_param  {}
impl AutoCreate<pj_cli_front_end_op> for pj_cli_front_end_op  {}
impl AutoCreate<pj_cli_front_end> for pj_cli_front_end  {}
impl AutoCreate<pj_cli_sess_op> for pj_cli_sess_op  {}
impl AutoCreate<pj_cli_sess> for pj_cli_sess  {}
impl AutoCreate<pj_cli_console_cfg> for pj_cli_console_cfg  {}
impl AutoCreate<pj_cli_telnet_info> for pj_cli_telnet_info  {}
impl AutoCreate<pj_cli_telnet_cfg> for pj_cli_telnet_cfg  {}