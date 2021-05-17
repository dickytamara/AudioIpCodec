
use pj_sys::*;

use crate::utils::{check_boolean, check_status};

pub struct PJThread {
    ctx: Box<*mut pj_thread_t>
}

impl From<Box<*mut pj_thread_t>> for PJThread {
    fn from(thread: Box<*mut pj_thread_t>) -> Self {
        Self { ctx: thread }
    }
}

impl PJThread {

// pj_status_t 	pj_thread_create (pj_pool_t *pool, const char *thread_name, pj_thread_proc *proc, void *arg, pj_size_t stack_size, unsigned flags, pj_thread_t **thread)

    pub fn thread_register(thread_name: Option<String>, desc: &mut pj_thread_desc) -> Result<Self, i32> {
        unsafe {
            let mut thread = Self { ctx: Box::new(std::ptr::null_mut()) };

            let thread_name = match thread_name {
                Some(value) => std::ffi::CString::new(value.as_str())
                    .expect("Error::pj_thread_register").into_raw(),
                None => std::ptr::null_mut()
            };

            let status = pj_thread_register( thread_name, desc.as_mut_ptr(), thread.ctx.as_mut() as *mut _);

            match check_status(status) {
                Ok(()) => {
                    if !(*thread.ctx).is_null() {
                        return Ok(thread);
                    } else {
                        return Err(-1);
                    }
                },
                Err(e) => { return Err(e); }
            }
        }
    }

    pub fn thread_is_registered() -> bool {
        unsafe { check_boolean(pj_thread_is_registered()) }
    }

    pub fn thread_get_prio(&self) -> i32 {
        unsafe { pj_thread_get_prio(*self.ctx) }
    }

    pub fn thread_set_prio(&self, prio: i32) -> Result<(), i32> {
        unsafe {
            check_status(pj_thread_set_prio(*self.ctx, prio))
        }
    }

    pub fn thread_get_prio_min(&self) -> i32 {
        unsafe { pj_thread_get_prio_min(*self.ctx) }
    }

    pub fn thread_get_prio_max(&self) -> i32 {
        unsafe { pj_thread_get_prio_max(*self.ctx) }
    }

    // void * 	pj_thread_get_os_handle (pj_thread_t *thread)
    // const char * 	pj_thread_get_name (pj_thread_t *thread)

    pub fn thread_this() -> Self {
        unsafe {
            Self::from(Box::new(pj_thread_this()))
        }
    }

    pub fn thread_resume(&self) -> Result<(), i32> {
        unsafe { check_status(pj_thread_resume(*self.ctx)) }
    }

    pub fn thread_join(&self) -> Result<(), i32> {
        unsafe { check_status(pj_thread_join(*self.ctx)) }
    }

    pub fn thread_destroy(self) -> Result<(), i32> {
        unsafe { check_status(pj_thread_destroy(*self.ctx)) }
    }

    pub fn thread_sleep(msec: u32) -> Result<(), i32> {
        unsafe { check_status(pj_thread_sleep(msec)) }
    }

    pub fn getpid() -> u32 {
        unsafe { pj_getpid() }
    }

}
