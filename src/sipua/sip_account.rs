use pjproject::pjsua::{AccountConfig, AccountInfo, CallSetting, CredentialInfo, CredentialInfoType, MAX_ACC, MessageData};

use crate::pjproject::prelude::*;
use crate::pjproject::pjsua;



// #[derive(Clone)]
pub struct Account {
    acc_id: i32,
    pub config: pjsua::AccountConfig
}

impl From<i32> for Account {
    fn from(acc_id: i32) -> Self {
        let mut ret = Account::new();
        ret.acc_id = acc_id;
        pjsua::acc_get_config(acc_id, &mut ret.config).unwrap();
        ret
    }
}


impl Account {

    pub fn new() -> Self {
        Self {
            acc_id: -1,
            config: AccountConfig::new(),
        }
    }

    // configure for default setting.
    pub fn config_default(&mut self) {

        pjsua::acc_config_default(&mut self.config);

        self.config.set_cred_count(1);
        self.config.set_reg_retry_interval(300);
        self.config.set_reg_first_retry_interval(60);

        for cred_info in self.config.cred_info.iter_mut() {
            cred_info.set_data_type(CredentialInfoType::PlainText);
            cred_info.set_scheme("Digest".to_string());
        }

        let mut cred = [CredentialInfo::new(); 8usize];
        cred[0].set_data_type(CredentialInfoType::PlainText);
        cred[0].set_scheme("Digest".to_string());

        self.config.set_cred_info(cred);
    }

    // set sip id for account
    // pub fn set_id(&mut self, value: String) {
    //     self.config.set_id(value);
    // }

    // set registrar uri
    // pub fn set_reg_uri(&mut self, value: String) {
    //     self.config.set_reg_uri(value);
    // }

    // set realm for account
    pub fn set_realm(&mut self, value: String) {
        self.config.cred_info[0].set_realm(value);
    }

    // set username
    pub fn set_username(&mut self, value: String) {
        self.config.cred_info[0].set_username(value);
    }

    // set password
    pub fn set_password(&mut self, value: String) {
        self.config.cred_info[0].set_data(value);

    }

    // check if this account valid or not
    pub fn is_valid(&self) -> bool {
        pjsua::acc_is_valid(self.acc_id)
    }

    // set this account to be default
    pub fn set_default(&self) {
        pjsua::acc_set_default(self.acc_id)
        .expect("SIPAccount::pjsua_acc_set_default");
    }

    // add account to internal pjsua
    pub fn add(&mut self, is_default: bool) {

        pjsua::acc_add(&mut self.config, is_default, &mut self.acc_id)
        .expect("SIPAccount::pjsua_acc_add");
    }

    // remove account from internal pjsua
    pub fn del(&self) {
        pjsua::acc_del(self.acc_id)
        .expect("SIPAccount::pjsua_acc_del");
    }

    // get inner config
    pub fn get_config(&self) -> Result<AccountConfig, i32> {

        let mut acc_cfg = AccountConfig::new();

        if let Err(e) = pjsua::acc_get_config(self.acc_id, &mut acc_cfg) {
            return Err(e)
        }

        Ok(acc_cfg)
    }

    // modify acoount config
    pub fn modify(&self, acc_config: &mut AccountConfig) {
        pjsua::acc_modify( self.acc_id, acc_config)
        .expect("SIPAccount::pjsua_acc_modify");
    }

    // set online or ofline status
    pub fn set_online_status(&self, is_online: bool) {
        pjsua::acc_set_online_status( self.acc_id, is_online)
        .expect("SIPAccount::pjsua_acc_set_online_status");
    }

    // this online status more like presence state for account
    // pub fn set_online_status2(&self, is_online: bool, pr: &mut pjrpid_element) {
    //     pjsua::acc_set_online_status2( self.acc_id, is_online, pr )
    //     .expect("SIPAccount::pjsua_acc_set_online_status2")
    // }

    // set registration process
    pub fn set_registration(&self, renew: bool) {
        pjsua::acc_set_registration( self.acc_id, renew)
        .expect("SIPAccount::pjsua_acc_set_registration");
    }

    // get inner account info
    pub fn get_info(&self) -> Result<AccountInfo, i32> {

        let mut info = AccountInfo::new();

        if let Err(e) = pjsua::acc_get_info( self.acc_id, &mut info) {
            return Err(e);
        }

        Ok(info)
    }

    // Create arbitrary requests using the account.
    // Application should only use this function to create auxiliary requests outside dialog,
    // such as OPTIONS, and use the call or presence API to create dialog related requests.
    // pub fn create_request(&self, method: &mut pjsip_method, target: String) -> Result<pjsip_tx_data, i32> {

    //     let mut rdata = pjsip_tx_data::new();

    //     if let Err(e) = pjsua::acc_create_request( self.acc_id, method, target, &mut rdata) {
    //         return Err(e);
    //     }

    //     Ok(rdata)
    // }

    // Create a suitable Contact header value,
    // based on the specified target URI for the specified account.
    pub fn create_uac_contact(&self, contact: String, uri: String) {
        pjsua::acc_create_uac_contact( contact, self.acc_id, uri)
        .expect("SIPAccount::pjsua_acc_create_uac_contact");
    }

    // Create a suitable Contact header value, based on the information in the incoming request.
    // pub fn create_uas_contact(&self, contact: String, rdata: &mut pjsip_rx_data) {
    //     pjsua::acc_create_uas_contact( contact, self.acc_id, rdata)
    //     .expect("SIPAccount::pjsua_acc_create_uas_contact");
    // }

    // set transport by given id transport id for account
    pub fn set_transport(&self, tp_id: i32) {

        pjsua::acc_set_transport( self.acc_id, tp_id)
        .expect("SIPAccount::pjsua_acc_set_transport");
    }

    // Send NOTIFY to inform account presence status or to terminate server side presence subscription.
    // If application wants to reject the incoming request,
    // it should set the state to PJSIP_EVSUB_STATE_TERMINATED.
    // pub fn pres_notify (&self,
    //     srv_pres: &mut pjsua_srv_pres,
    //     state: pjsip_evsub_state,
    //     state_str: String,
    //     reason: String,
    //     with_body: bool,
    //     msg_data: Option<&mut MessageData>
    // ) {
    //     pjsua::pres_notify(self.acc_id, srv_pres, state, state_str, reason, with_body, msg_data)
    //     .expect("SIPAccount::pjsua_pres_notify");
    // }

    // Send instant messaging outside dialog, using the specified account for route set and authentication
    pub fn im_send(&self,
        to: String,
        mime_type: String,
        content: String,
        msg_data: Option<&mut MessageData>
    ) {
        pjsua::im_send(self.acc_id, to, mime_type, content, msg_data)
        .expect("SIPAccount::pjsua_im_send");
    }

    // Send typing indication outside dialog.
    pub fn im_typing(&self, to: String, is_typing: bool, msg_data: Option<&mut MessageData>) {
        pjsua::im_typing(self.acc_id, to, is_typing, msg_data)
        .expect("SIPAccount::pjsua_im_typing");
    }

    pub fn call(
        &self,
        dst_uri: String,
        opt: Option<&mut CallSetting>,
        msg_data: Option<&mut MessageData>,
        p_call_id: Option<&mut i32>
    ) {
        pjsua::call_make_call(self.acc_id, dst_uri, opt, msg_data, p_call_id)
        .expect("SIPAccount::pjsua_call_make_call");
    }

}

// true interface for managing accounts
pub struct SIPAccounts {  }

// please remember default pjsua account
// was 8 see pjsua sample application at
// pjproject.
impl SIPAccounts {

    // create new accounts
    pub fn new() -> SIPAccounts {
        SIPAccounts { }
    }

    // get default account
    pub fn get_default(&self) -> i32 {
        pjsua::acc_get_default()
    }

    // get number of registered to pjsua library
    pub fn get_count(&self) -> u32 {
        pjsua::acc_get_count()
    }

    // enumerate current registered account id
    pub fn enum_accs_id(&self) -> Vec<i32> {

        let mut ret = Vec::<i32>::new();

        let mut acc_id: [i32; MAX_ACC] = [-1; MAX_ACC];
        let mut count = 8u32;

        if let Ok(_)= pjsua::enum_accs(&mut acc_id, &mut count) {

            for i in 0..count as usize {
                ret.push(acc_id[i]);
            }
        } else {
            println!("ERR cant enumerate accounts id.");
        }

        ret
    }

    // This is an internal function to find the most appropriate account to used to reach to the specified URL.
    pub fn find_for_outgoing(&self, value: String) -> i32 {
        pjsua::acc_find_for_outgoing(value)
    }

    // This is an internal function to find the most appropriate account to be used to handle incoming calls.
    // pub fn on_acc_find_for_incoming (&self, rdata: &mut pjsip_rx_data) -> i32 {
    //     pjsua::acc_find_for_incoming(rdata)
    // }

}
