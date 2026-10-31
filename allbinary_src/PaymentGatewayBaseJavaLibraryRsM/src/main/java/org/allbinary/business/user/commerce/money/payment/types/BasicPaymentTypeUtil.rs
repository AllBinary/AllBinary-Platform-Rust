//
// AllBinary Open License Version 1
// Copyright (c) 2011 AllBinary
// 
// By agreeing to this license you and any business entity you represent are
// legally bound to the AllBinary Open License Version 1 legal agreement.
// 
// You may obtain the AllBinary Open License Version 1 legal agreement from
// AllBinary or the root directory of AllBinary's AllBinary Platform repository.
// 
// Created By: Travis Berthelot
//  
// Generated Code Do Not Modify
// package org.allbinary.business.user.commerce.money.payment.types
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use super::BasicPaymentType::BasicPaymentType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicPaymentTypeUtil {
    pub paymentTypeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static BasicPaymentTypeUtil_instance: Option<Rc<RefCell<BasicPaymentTypeUtil>>> = BasicPaymentTypeUtil::new(); }

impl BasicPaymentTypeUtil {
    

    // Java static initializer: call this once at startup
    pub fn static_init() {
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicPaymentTypeUtil>>> {
        return BasicPaymentTypeUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            paymentTypeVector: None,
        };
        this.paymentTypeVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn add(&mut self, mut paymentType: Option<Rc<RefCell<BasicPaymentType>>>) {
        jcall!(self.paymentTypeVector).add(crate::java_into_object(paymentType.clone()));
    }
    

    // throws Exception
    pub fn get(&mut self, mut paymentTypeString: String) -> Option<Rc<RefCell<BasicPaymentType>>> {
        let mut size: i32 = jcall!(self.paymentTypeVector).size();
        for i in (0..size) {
            let mut paymentType: Option<Rc<RefCell<BasicPaymentType>>> = { let java_object = (jcall!(self.paymentTypeVector).get(i.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<BasicPaymentType>>>>().expect("ClassCastException").clone()) };
            if jcall!(jcall!(paymentType).getName()).compareTo(paymentTypeString.clone()) == 0 {
                return paymentType.clone();
            }
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Unknown PaymentType: "), paymentTypeString))));
    }
    

    pub fn isContain(&mut self, mut paymentType: Option<Rc<RefCell<BasicPaymentType>>>) -> bool {
        return jcall!(self.paymentTypeVector).contains(crate::java_into_object(paymentType.clone()));
    }
    

    pub fn difference(&mut self, mut a_PaymentTypeVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut diff: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(self.paymentTypeVector).size();
        for i in (0..size) {
            let mut paymentType: Option<Rc<RefCell<BasicPaymentType>>> = { let java_object = (jcall!(self.paymentTypeVector).get(i.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<BasicPaymentType>>>>().expect("ClassCastException").clone()) };
            if !jcall!(a_PaymentTypeVector).contains(crate::java_into_object(paymentType.clone())) {
                jcall!(diff).add(crate::java_into_object(paymentType.clone()));
            }
        }
        return diff.clone();
    }
}

