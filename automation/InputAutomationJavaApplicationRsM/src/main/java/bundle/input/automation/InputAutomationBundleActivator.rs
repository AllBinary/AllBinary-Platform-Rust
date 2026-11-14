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
// package bundle.input.automation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::osgi::framework::BundleActivator::BundleActivator;
use crate::org::osgi::framework::BundleContext::BundleContext;
use crate::bundle::input::automation::module::configuration::InputAutomationConfigurationModuleChangeListener::InputAutomationConfigurationModuleChangeListener;
use crate::bundle::input::automation::robot::InputAutomationRobotChangeListener::InputAutomationRobotChangeListener;
use crate::org::allbinary::input::automation::InputAutomationJFrame::InputAutomationJFrame;
use crate::org::allbinary::input::automation::module::osgi::InputAutomationModuleServiceConsumer::InputAutomationModuleServiceConsumer;
use crate::org::allbinary::input::automation::module::osgi::InputAutomationNewBundleRunnable::InputAutomationNewBundleRunnable;
use crate::org::allbinary::input::automation::osgi::InputAutomationServiceFactory::InputAutomationServiceFactory;
use crate::org::allbinary::input::automation::robot::osgi::InputAutomationRobotServiceConsumer::InputAutomationRobotServiceConsumer;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::osgi::OSGIActivatorUtil::OSGIActivatorUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Thread::*;
use crate::java::lang::Exception::*;
use super::InputAutomationBundleActivatorListenerInterface::InputAutomationBundleActivatorListenerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationBundleActivator {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

pub static mut InputAutomationBundleActivator_moduleManagementThread: std::cell::LazyCell<Option<Rc<RefCell<Thread>>>> = std::cell::LazyCell::new(|| None);
pub static mut InputAutomationBundleActivator_bundleContext: std::cell::LazyCell<Option<Rc<RefCell<dyn BundleContext>>>> = std::cell::LazyCell::new(|| None);
impl BundleActivator for InputAutomationBundleActivator {
}
impl InputAutomationBundleActivatorListenerInterface for InputAutomationBundleActivator {
    fn registerAsService(&mut self) { Self::registerAsService(self) }
    fn useServices(&mut self) { Self::useServices(self) }
}

impl InputAutomationBundleActivator {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getBundleContext() -> Option<Rc<RefCell<dyn BundleContext>>> {
        return unsafe { InputAutomationBundleActivator_bundleContext.clone() }.clone();
    }
    

    // throws Exception
    pub fn start(&mut self, mut bundleContext: Option<Rc<RefCell<dyn BundleContext>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone());
            unsafe { *InputAutomationBundleActivator_bundleContext = bundleContext.clone() };
            unsafe { *InputAutomationBundleActivator_moduleManagementThread = Thread::new1(InputAutomationNewBundleRunnable::new(None /* Java this: a &mut self method has no Rc of itself */)) };
            jcall!(unsafe { InputAutomationBundleActivator_moduleManagementThread.clone() }).start();
            InputAutomationJFrame::create(None /* Java this: a &mut self method has no Rc of itself */);
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn registerAsService(&mut self) {
        OSGIActivatorUtil::registerAsService(self.getBundleContext(), { let java_argument = crate::java_into_object(InputAutomationServiceFactory::getInstance()); java_argument }, { let java_argument = jcall!(std::any::type_name::<InputAutomationConfigurationModuleChangeListener>()).getName(); java_argument });
        OSGIActivatorUtil::registerAsService(self.getBundleContext(), { let java_argument = crate::java_into_object(InputAutomationServiceFactory::getInstance()); java_argument }, { let java_argument = jcall!(std::any::type_name::<InputAutomationRobotChangeListener>()).getName(); java_argument });
    }
    

    // throws Exception
    pub fn useServices(&mut self) {
        jcall!(InputAutomationRobotServiceConsumer::new(self.getBundleContext())).process();
        jcall!(InputAutomationModuleServiceConsumer::new(self.getBundleContext())).process();
    }
    

    // throws Exception
    pub fn stop(&mut self, mut context: Option<Rc<RefCell<dyn BundleContext>>>) {
        jcall!(self.logUtil).putF({ let java_field = jcall!(self.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("stop"));
        if InputAutomationJFrame::getInstance().is_some() {
            InputAutomationJFrame::destroy();
        } else {
            jcall!(self.logUtil).putF(String::from("Nothing to stop"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("stop"));
        }
    }
}

