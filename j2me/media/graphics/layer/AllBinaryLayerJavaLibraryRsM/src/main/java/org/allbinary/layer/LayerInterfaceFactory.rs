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
// package org.allbinary.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use super::LayerInterfaceFactoryInterface::LayerInterfaceFactoryInterface;
use super::AllBinaryLayer::AllBinaryLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerInterfaceFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub HASHTABLE_LABEL: String,
}

thread_local! { pub static LayerInterfaceFactory_SINGLETON: Option<Rc<RefCell<LayerInterfaceFactory>>> = LayerInterfaceFactory::new(); }

impl LayerInterfaceFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            list: None,
            HASHTABLE_LABEL: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.list = jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
        this.HASHTABLE_LABEL = String::from("Hashtable: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<LayerInterfaceFactory>>> {
        return LayerInterfaceFactory_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn init(&mut self) {
        self.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
    }
    

    // throws Exception
    pub fn getNexInstance(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut x: i32, mut y: i32, mut z: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(self.HASHTABLE_LABEL.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(hashtable.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut typeInteger: Option<i32> = { let java_object = jcall!(hashtable).get(crate::java_into_object((Layer_ID.with(Clone::clone)).clone() /* as Any */)); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
        let mut layerInterfaceFactoryInterface: Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>> = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(((typeInteger).unwrap() as i32) - 1) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>>>().expect("ClassCastException").clone()) };
        return jcall!(layerInterfaceFactoryInterface).getNextInstance(hashtable.clone(), x.clone(), y.clone(), z.clone());
    }
    

    pub fn add(&mut self, mut layerInterfaceFactoryInterface: Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>>) {
        jcall!(self.list).add(crate::java_into_object(layerInterfaceFactoryInterface.clone()));
    }
}

