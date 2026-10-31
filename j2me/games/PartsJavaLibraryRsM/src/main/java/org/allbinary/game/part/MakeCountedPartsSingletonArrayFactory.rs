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
// package org.allbinary.game.part
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactory;
use crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactoryPool::CountedPickedUpLayerInterfaceFactoryPool;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use super::CountedLayerInterfaceFactoryPart::CountedLayerInterfaceFactoryPart;
use super::PartInterface::PartInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MakeCountedPartsSingletonArrayFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static MakeCountedPartsSingletonArrayFactory_instance: Option<Rc<RefCell<MakeCountedPartsSingletonArrayFactory>>> = MakeCountedPartsSingletonArrayFactory::new(); }

impl MakeCountedPartsSingletonArrayFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            list: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.list = jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<MakeCountedPartsSingletonArrayFactory>>> {
        return MakeCountedPartsSingletonArrayFactory_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getUpdated(&mut self, mut partInterfaceArray: Vec<Option<Rc<RefCell<dyn PartInterface>>>>) -> Vec<Option<Rc<RefCell<dyn PartInterface>>>> {
        self.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        for index in (0..(partInterfaceArray.len() as i32)) {
            jcall!(self.list).add(crate::java_into_object(partInterfaceArray[(index) as usize].clone()));
        }
        let mut countedBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(CountedPickedUpLayerInterfaceFactoryPool::getInstance()).getList();
        let mut layerInterfaceFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactoryDyn>>> = None;
        let mut size: i32 = jcall!(countedBasicArrayList).size();
        for index in (0..size) {
            layerInterfaceFactoryInterface = { let java_object = (jcall!(countedBasicArrayList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactoryDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(self.list).add(crate::java_into_object(CountedLayerInterfaceFactoryPart::new(0, layerInterfaceFactoryInterface.clone())));
        }
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Total Parts: "));
        jcall!(stringBuffer).appendint(jcall!(self.list).size());
        jcall!(stringBuffer).append(String::from(" Counted: "));
        jcall!(stringBuffer).appendint(jcall!(countedBasicArrayList).size());
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut newPartInterfaceArray: Vec<Option<Rc<RefCell<dyn PartInterface>>>> = vec![None; (jcall!(self.list).size()) as usize];
        let mut newSize: i32 = (newPartInterfaceArray.len() as i32).clone();
        for index in (0..newSize) {
            newPartInterfaceArray[(index) as usize] = { let java_object = jcall!(self.list).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn PartInterface>>>>().expect("ClassCastException").clone()) };
        }
        return newPartInterfaceArray.clone();
    }
}

