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
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ClientInformationFactory::ClientInformationFactory;
use crate::org::allbinary::thread::PrimaryThreadPool::PrimaryThreadPool;
use crate::org::allbinary::thread::ThreadPool::ThreadPool;
use crate::java::lang::Exception::*;
use super::AllBinaryMidlet::AllBinaryMidlet;
use super::ExitRunnable::ExitRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

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
        this.abeClientInformation = { let java_value = (jcall!(clientInformationFactory).getInstance()).clone(); java_value }.map(|_java_reference| -> Rc<RefCell<dyn AbeClientInformationInterface>> { panic!("ClassCastException: the Rust trait of ClientInformation does not have AbeClientInformationInterface as a base trait") });
        this
    }

    pub fn new(mut clientInformationFactory: Option<Rc<RefCell<ClientInformationFactory>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(clientInformationFactory))))
    }
    

    pub fn preInit(&mut self) {
    }
    

    pub fn exitProgress(&mut self, mut isProgress: bool) {
        let mut processor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>> = jcall!(MidletExitProcessorFactory::MidletExitProcessorFactory::getInstance()).getExitInstance(None /* Java this: a &mut self method has no Rc of itself */);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut primaryThreadPool: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>> = PrimaryThreadPool::getInstance();
            let mut runnable: Option<Rc<RefCell<dyn Runnable>>> = ExitRunnable::new(None /* Java this: a &mut self method has no Rc of itself */, processor.clone(), isProgress.clone()).map(|reference| reference as Rc<RefCell<dyn Runnable>>).map(|reference| reference as Rc<RefCell<dyn Runnable>>);
            jcall!(primaryThreadPool).runTask(runnable.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("exit"), crate::java_into_object(e.clone()));
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                jcall!(processor).process();
            })) {
                Ok(_) => {}
                Err(e2) => {
            let e2: String = crate::java::lang::Exception::java_panic_message(&e2);

                jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("exit"), crate::java_into_object(e.clone()));
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
}
impl ProgressMidletDyn for ProgressMidlet {
    fn as_ProgressMidlet(&self) -> &ProgressMidlet { self }
    fn as_ProgressMidlet_mut(&mut self) -> &mut ProgressMidlet { self }
}

impl crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn for ProgressMidlet {
    fn as_AllBinaryMidlet(&self) -> &crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet { &self.base }
    fn as_AllBinaryMidlet_mut(&mut self) -> &mut crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet { &mut self.base }
}
impl crate::javax::microedition::midlet::MIDlet::MIDletDyn for ProgressMidlet {
    fn as_MIDlet(&self) -> &crate::javax::microedition::midlet::MIDlet::MIDlet { &self.base.base }
    fn as_MIDlet_mut(&mut self) -> &mut crate::javax::microedition::midlet::MIDlet::MIDlet { &mut self.base.base }
    fn startApp(&mut self) { crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet::startApp(crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn::as_AllBinaryMidlet_mut(self)) }
    fn pauseApp(&mut self) { crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet::pauseApp(crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn::as_AllBinaryMidlet_mut(self)) }
    fn destroyApp(&mut self, p1: bool) { crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet::destroyApp(crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn::as_AllBinaryMidlet_mut(self), p1) }
}
impl crate::javax::microedition::lcdui::CommandListener::CommandListener for ProgressMidlet {
    fn commandAction(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>, p2: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Displayable::DisplayableDyn>>>) { crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet::commandAction(crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn::as_AllBinaryMidlet_mut(self), p1, p2) }
}


