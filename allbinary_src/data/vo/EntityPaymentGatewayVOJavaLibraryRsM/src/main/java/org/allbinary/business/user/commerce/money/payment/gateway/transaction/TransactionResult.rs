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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::tokens::Tokenizer::Tokenizer;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
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
    pub keyValuePairs: Option<Rc<RefCell<HashMap>>>,
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
            let mut tokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new(jcall!(CommonSeps::getInstance()).AMPERSAND.clone());
            let mut tokens: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(tokenizer).getTokensFromString(result.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
            let mut tokenizer2: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new(jcall!(CommonSeps::getInstance()).EQUALS.clone());
            let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(tokenizer2).getTokens(tokens.clone());
            jcall!(this.keyValuePairs).putAll(hashtable.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut exceptionMessage: String = String::from("TransactionResult Constructor Failed");
            jcall!(this.logUtil).putF(exceptionMessage.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
            }
        }
        this
    }

    pub fn new1(mut result: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(result))))
    }
    

    pub fn new_struct1(mut keyValuePairs: Option<Rc<RefCell<HashMap>>>) -> Self {
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

    pub fn new1(mut keyValuePairs: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(keyValuePairs))))
    }
    

    pub fn toString(&mut self) -> String {
        return self.result.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return self.keyValuePairs.clone();
    }
    

    pub fn getAllPossibleKeys(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut paymentTransactionKeysFactory: Option<Rc<RefCell<PaymentTransactionKeysFactory>>> = PaymentTransactionKeysFactory::getInstance();
        let mut verisignResponseKeys: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).RESULT).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).PNREF).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).RESPMSG).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).AUTHCODE).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).AVSADDR).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).AVSZIP).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).ORIGRESULT).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).STATUS).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).FRAUDCODE).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).FRAUDMSG).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).ERRCODE).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).SCORE).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).REASON1).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).REASON2).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).REASON3).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).EXCEPTION1).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).EXCEPTION2).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).EXCEPTION3).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).EXCEPTION4).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).EXCEPTION5).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).EXCEPTION6).toString()));
        jcall!(verisignResponseKeys).add(crate::java_into_object(jcall!(jcall!(paymentTransactionKeysFactory).EXCEPTION7).toString()));
        return verisignResponseKeys.clone();
    }
    

    pub fn getValues(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut keyVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getAllPossibleKeys();
        let mut valueVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut size: i32 = jcall!(keyVector).size();
        for i in (0..size) {
            let mut key: String = (jcall!(keyVector).get(i.clone())).clone() /* as String */;
            if jcall!(self.keyValuePairs).containsKey(key.clone()) {
                jcall!(valueVector).add(crate::java_into_object(jcall!(self.keyValuePairs).get(key.clone())));
            } else {
                let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
                jcall!(valueVector).add(crate::java_into_object(jcall!(stringUtil).EMPTY_STRING.clone()));
            }
        }
        return valueVector.clone();
    }
    

    pub fn getPnRef(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).PNREF).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getRespMsg(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).RESPMSG).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getResult(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).RESULT).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getScore(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).SCORE).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getStatus(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).STATUS).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getFraudCode(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).FRAUDCODE).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getFraudMsg(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).FRAUDMSG).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getOrigResult(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).ORIGRESULT).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getAvsAddr(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).AVSADDR).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getAvsZip(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).AVSZIP).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getErrCode(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).ERRCODE).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getException1(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).EXCEPTION1).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getException2(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).EXCEPTION2).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getException3(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).EXCEPTION3).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getException4(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).EXCEPTION4).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getException5(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).EXCEPTION5).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getException6(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).EXCEPTION6).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getException7(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).EXCEPTION7).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getReason1(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).REASON1).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getReason2(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).REASON2).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
    

    pub fn getReason3(&mut self) -> String {
        return ({ let java_receiver = self.keyValuePairs.clone(); let java_arguments = (jcall!(jcall!(PaymentTransactionKeysFactory::getInstance()).REASON3).toString(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone() /* as String */;
    }
}

