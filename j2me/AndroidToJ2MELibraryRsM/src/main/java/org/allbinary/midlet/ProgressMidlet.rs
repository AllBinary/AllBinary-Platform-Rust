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
// package org.allbinary.midlet
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::globals::Globals::Globals;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ClientInformationFactory::ClientInformationFactory;
use crate::org::allbinary::thread::PrimaryThreadPool::PrimaryThreadPool;
use crate::org::allbinary::thread::ThreadPool::ThreadPool;
use super::AllBinaryMidlet::AllBinaryMidlet;
use super::ExitRunnable::ExitRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//AndroidToJ2ME
pub struct ProgressMidlet {
    pub base: AllBinaryMidlet,
    pub abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>,
}


impl ProgressMidlet {
    

    pub fn new_struct(mut clientInformationFactory: Option<Rc<RefCell<ClientInformationFactory>>>) -> Self {
        let mut this = Self {
            base: AllBinaryMidlet::new_struct(),
            abeClientInformation: None,
        };
        this.preInit();
        this.abeClientInformation = jcall!(clientInformationFactory).getInstance();
        this
    }

    pub fn new(mut clientInformationFactory: Option<Rc<RefCell<ClientInformationFactory>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(clientInformationFactory))));
            jcall!(Globals::getInstance()).init(jcall!(TsUtil::getInstance()).getClassClassLoader((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone())), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
        this
    }
    

    pub fn preInit(&mut self) {
    }
    

    pub fn exitProgress(&mut self, mut isProgress: bool) {
        let mut processor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>> = jcall!(MidletExitProcessorFactory::MidletExitProcessorFactory::getInstance()).getExitInstance(None /* Java this: a &mut self method has no Rc of itself */);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut primaryThreadPool: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>> = PrimaryThreadPool::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>);
            let mut runnable: Option<Rc<RefCell<dyn Runnable>>> = ExitRunnable::new(None /* Java this: a &mut self method has no Rc of itself */, processor.clone(), isProgress.clone()).map(|reference| reference as Rc<RefCell<dyn Runnable>>);
            jcall!(primaryThreadPool).runTask(runnable.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("exit"), crate::java_into_object(e.clone()));
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                jcall!(processor).process();
            })) {
                Ok(_) => {}
                Err(e2) => {
            let e2: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e2 as Box<dyn std::any::Any>)));

                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("exit"), crate::java_into_object(e.clone()));
                }
            }
            }
        }
    }
}
impl std::ops::Deref for ProgressMidlet {
    type Target = AllBinaryMidlet;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ProgressMidlet {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ProgressMidletDyn: crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn {
    fn as_ProgressMidlet(&self) -> &ProgressMidlet;
    fn as_ProgressMidlet_mut(&mut self) -> &mut ProgressMidlet;
    fn preInit(&mut self) { ProgressMidlet::preInit(self.as_ProgressMidlet_mut()) }
    fn exitProgress(&mut self, mut isProgress: bool) { ProgressMidlet::exitProgress(self.as_ProgressMidlet_mut(), isProgress) }
    fn preInit(&mut self) { ProgressMidlet::preInit(self.as_ProgressMidlet_mut()) }
    fn exitProgress(&mut self, mut isProgress: bool) { ProgressMidlet::exitProgress(self.as_ProgressMidlet_mut(), isProgress) }
    fn preInit(&mut self) { ProgressMidlet::preInit(self.as_ProgressMidlet_mut()) }
    fn exitProgress(&mut self, mut isProgress: bool) { ProgressMidlet::exitProgress(self.as_ProgressMidlet_mut(), isProgress) }
}
impl ProgressMidletDyn for ProgressMidlet {
    fn as_ProgressMidlet(&self) -> &ProgressMidlet { self }
    fn as_ProgressMidlet_mut(&mut self) -> &mut ProgressMidlet { self }
}

impl crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn for ProgressMidlet {
    fn as_AllBinaryMidlet(&self) -> &crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet { &self.base }
    fn as_AllBinaryMidlet_mut(&mut self) -> &mut crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet { &mut self.base }
}


