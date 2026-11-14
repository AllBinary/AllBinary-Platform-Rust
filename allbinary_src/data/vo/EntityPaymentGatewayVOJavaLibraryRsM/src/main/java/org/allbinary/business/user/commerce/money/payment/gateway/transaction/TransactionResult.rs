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
// package org.allbinary.business.user.commerce.money.payment.gateway.transaction
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::tokens::Tokenizer::Tokenizer;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::java::lang::Exception::*;
use super::TransactionResultInterface::TransactionResultInterface;
use super::PaymentTransactionKeysFactory::PaymentTransactionKeysFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransactionResult {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub keyValuePairs: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>,
    pub result: String,
}

impl TransactionResultInterface for TransactionResult {
    fn toString(&mut self) -> String { Self::toString(self) }
    fn getValues(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getValues(self) }
    fn getResult(&mut self) -> String { Self::getResult(self) }
    fn getPnRef(&mut self) -> String { Self::getPnRef(self) }
    fn getRespMsg(&mut self) -> String { Self::getRespMsg(self) }
    fn getAvsAddr(&mut self) -> String { Self::getAvsAddr(self) }
    fn getAvsZip(&mut self) -> String { Self::getAvsZip(self) }
    fn getOrigResult(&mut self) -> String { Self::getOrigResult(self) }
    fn getStatus(&mut self) -> String { Self::getStatus(self) }
    fn getFraudCode(&mut self) -> String { Self::getFraudCode(self) }
    fn getFraudMsg(&mut self) -> String { Self::getFraudMsg(self) }
    fn getErrCode(&mut self) -> String { Self::getErrCode(self) }
    fn getScore(&mut self) -> String { Self::getScore(self) }
    fn getReason1(&mut self) -> String { Self::getReason1(self) }
    fn getReason2(&mut self) -> String { Self::getReason2(self) }
    fn getReason3(&mut self) -> String { Self::getReason3(self) }
    fn getException1(&mut self) -> String { Self::getException1(self) }
    fn getException2(&mut self) -> String { Self::getException2(self) }
    fn getException3(&mut self) -> String { Self::getException3(self) }
    fn getException4(&mut self) -> String { Self::getException4(self) }
    fn getException5(&mut self) -> String { Self::getException5(self) }
    fn getException6(&mut self) -> String { Self::getException6(self) }
    fn getException7(&mut self) -> String { Self::getException7(self) }
}

impl TransactionResult {
    

    pub fn new_struct1(mut result: String) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            keyValuePairs: None,
            result: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            this.result = result.clone();
            let mut tokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new({ let java_field = jcall!(CommonSeps::getInstance()).AMPERSAND.clone(); java_field }.clone());
            let mut tokens: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(tokenizer).getTokensFromString(result.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
            let mut tokenizer2: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new({ let java_field = jcall!(CommonSeps::getInstance()).EQUALS.clone(); java_field }.clone());
            let mut hashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>> = jcall!(tokenizer2).getTokens(tokens.clone());
            jcall!(this.keyValuePairs).putAll(hashtable.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut exceptionMessage: String = String::from("TransactionResult Constructor Failed");
            jcall!(this.logUtil).putF(exceptionMessage.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
            }
        }
        this
    }

    pub fn new1(mut result: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(result))))
    }
    

    pub fn new_struct1_HashMap(mut keyValuePairs: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            keyValuePairs: None,
            result: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        keyValuePairs = keyValuePairs.clone();
        this.result = String::from("Needs to have the hashmap converted to string for other constructor");
        this
    }

    pub fn new1_HashMap(mut keyValuePairs: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_HashMap(keyValuePairs))))
    }
    

    pub fn toString(&mut self) -> String {
        return self.result.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        return self.keyValuePairs.clone();
    }
    

    pub fn getAllPossibleKeys(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut paymentTransactionKeysFactory: Option<Rc<RefCell<PaymentTransactionKeysFactory>>> = PaymentTransactionKeysFactory::getInstance();
        let mut verisignResponseKeys: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).RESULT.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).PNREF.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).RESPMSG.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).AUTHCODE.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).AVSADDR.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).AVSZIP.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).ORIGRESULT.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).STATUS.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).FRAUDCODE.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).FRAUDMSG.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).ERRCODE.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).SCORE.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).REASON1.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).REASON2.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).REASON3.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).EXCEPTION1.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).EXCEPTION2.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).EXCEPTION3.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).EXCEPTION4.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).EXCEPTION5.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).EXCEPTION6.clone(); java_field }).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!({ let java_field = jcall!(paymentTransactionKeysFactory).EXCEPTION7.clone(); java_field }).toString()));
        return verisignResponseKeys.clone();
    }
    

    pub fn getValues(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut keyVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getAllPossibleKeys();
        let mut valueVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(keyVector).size();
        for i in (0..size) {
            let mut key: String = { let java_object = (jcall!(keyVector).get(i.clone())).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            if jcall!(self.keyValuePairs).containsKey(crate::java_into_object(key.clone())) {
                jcall!(valueVector).add(crate::java_into_object(jcall!(self.keyValuePairs).get(crate::java_into_object(key.clone()))));
            } else {
                let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
                jcall!(valueVector).add(crate::java_into_object({ let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }.clone()));
            }
        }
        return valueVector.clone();
    }
    

    pub fn getPnRef(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).PNREF.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getRespMsg(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).RESPMSG.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getResult(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).RESULT.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getScore(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).SCORE.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getStatus(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).STATUS.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getFraudCode(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).FRAUDCODE.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getFraudMsg(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).FRAUDMSG.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getOrigResult(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).ORIGRESULT.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getAvsAddr(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).AVSADDR.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getAvsZip(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).AVSZIP.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getErrCode(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).ERRCODE.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getException1(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).EXCEPTION1.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getException2(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).EXCEPTION2.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getException3(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).EXCEPTION3.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getException4(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).EXCEPTION4.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getException5(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).EXCEPTION5.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getException6(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).EXCEPTION6.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getException7(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).EXCEPTION7.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getReason1(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).REASON1.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getReason2(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).REASON2.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
    

    pub fn getReason3(&mut self) -> String {
        return { let java_object = ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (crate::java_into_object(jcall!({ let java_field = jcall!(PaymentTransactionKeysFactory::getInstance()).REASON3.clone(); java_field }).toString()),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
    }
}

