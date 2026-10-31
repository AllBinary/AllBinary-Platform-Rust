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
// package org.allbinary.business.context.modules.storefronts
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::init::InitSql::InitSql;
use crate::org::allbinary::business::init::db::UserDbInitInfo::UserDbInitInfo;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::BasicStoreFront::BasicStoreFront;
use super::BasicStoreFrontInterface::BasicStoreFrontInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicStoreFrontsEntity {
    pub base: InitSql,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tableName: String,
}


impl BasicStoreFrontsEntity {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: InitSql::new_struct(UserDbInitInfo::new()),
            logUtil: None,
            tableName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.tableName = String::from("storefronts");
        // super(...) is in the struct literal above
        this.setTable(this.tableName.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getStoreFrontInterface(&mut self, mut name: String) -> Option<Rc<RefCell<dyn BasicStoreFrontInterface>>> {
        let mut keysAndValues: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(keysAndValues).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), name.clone());
        let mut storeHashMap: Option<Rc<RefCell<HashMap>>> = self.base.getRow(keysAndValues.clone());
        if storeHashMap.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("No Such Store: "), name))));
        }
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).FACTORYERROR.clone()) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("StoreFront: "), jcall!(storeHashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getStore()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
        return (BasicStoreFront::new1(storeHashMap.clone())).clone() /* as BasicStoreFrontInterface */;
    }
}
impl std::ops::Deref for BasicStoreFrontsEntity {
    type Target = InitSql;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicStoreFrontsEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::init::InitSql::InitSqlDyn for BasicStoreFrontsEntity {
    fn as_InitSql(&self) -> &crate::org::allbinary::business::init::InitSql::InitSql { &self.base }
    fn as_InitSql_mut(&mut self) -> &mut crate::org::allbinary::business::init::InitSql::InitSql { &mut self.base }
}


