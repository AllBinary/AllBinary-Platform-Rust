// Generated Code Do Not Modify
// package org.allbinary.android
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use crate::android::content::Context::Context;
use crate::android::view::View::View;
use crate::android::widget::Toast::Toast;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::MakeToast::MakeToast;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ToastUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static ToastUtil_instance: Option<Rc<RefCell<ToastUtil>>> = ToastUtil::new(); }

impl ToastUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ToastUtil>>> {
        return ToastUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn makeToast(&mut self, mut view: Option<Rc<RefCell<View>>>, mut context: Option<Rc<RefCell<Context>>>, mut message: String, mut time: i32) {
        /* Java local class: move it out of the function
pub struct MakeToast {
    pub string: String,
}

impl Runnable for MakeToast {
    fn run(&mut self) { Self::run(self) }
}

impl MakeToast {
            

            pub fn new_struct(mut string: String) -> Self {
                let mut this = Self {
            string: String::new(),
                };
                this.string = string.clone();
                this
            }

            pub fn new(mut string: String) -> Option<Rc<RefCell<Self>>> {
                Some(Rc::new(RefCell::new(Self::new_struct(string))))
            }
            

            pub fn run(&mut self) {
                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    jcall!(Toast::makeText(self.base.context.clone(), self.string.clone(), self.base.time.clone())).show();
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                    jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
                    }
                }
            }
}
*/
        jcall!(view).post(MakeToast::new(message.clone()));
    }
}

