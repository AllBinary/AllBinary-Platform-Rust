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
// package org.allbinary.logic.system.hardware.linux
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypeFactory::LogConfigTypeFactory;
use crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes;
use crate::org::allbinary::logic::system::hardware::components::ComponentFactory::ComponentFactory;
use crate::org::allbinary::logic::system::hardware::components::interfaces::HardwareComponentInterface::HardwareComponentInterface;
use crate::org::allbinary::logic::system::hardware::components::linux::Bridge::Bridge;
use crate::org::allbinary::logic::system::hardware::components::linux::Ethernet::Ethernet;
use crate::org::allbinary::logic::system::hardware::components::linux::HardDriveController::HardDriveController;
use crate::org::allbinary::logic::system::hardware::components::linux::Media::Media;
use crate::org::allbinary::logic::system::hardware::components::linux::PCI::PCI;
use crate::org::allbinary::logic::system::hardware::components::linux::Unknown::Unknown;
use crate::org::allbinary::logic::system::hardware::components::linux::Usb::Usb;
use crate::org::allbinary::logic::system::hardware::components::linux::Video::Video;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PCComponentFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub componentFactory: Option<Rc<RefCell<ComponentFactory>>>,
}

thread_local! { pub static PCComponentFactory_instance: Option<Rc<RefCell<PCComponentFactory>>> = PCComponentFactory::new(); }

impl PCComponentFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PCComponentFactory>>> {
        return PCComponentFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            componentFactory: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.componentFactory = ComponentFactory::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn getInstance2(&mut self, mut component: String, mut componentData: String) -> Option<Rc<RefCell<dyn HardwareComponentInterface>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if true {
                if (component.cmp(&jcall!({ let java_field = jcall!(self.componentFactory).ETHERNET.clone(); java_field }).toString()) as i32) == 0 {
                    return Some((Ethernet::new(componentData.clone())).clone() /* as HardwareComponentInterface */);
                } else if (component.cmp(&jcall!({ let java_field = jcall!(self.componentFactory).BRIDGE.clone(); java_field }).toString()) as i32) == 0 {
                    return Some((Bridge::new(componentData.clone())).clone() /* as HardwareComponentInterface */);
                } else if (component.cmp(&jcall!({ let java_field = jcall!(self.componentFactory).IDE.clone(); java_field }).toString()) as i32) == 0 {
                    return Some((HardDriveController::new(componentData.clone())).clone() /* as HardwareComponentInterface */);
                } else if (component.cmp(&jcall!({ let java_field = jcall!(self.componentFactory).MULTIMEDIA.clone(); java_field }).toString()) as i32) == 0 {
                    return Some((Media::new(componentData.clone())).clone() /* as HardwareComponentInterface */);
                } else if (component.cmp(&jcall!({ let java_field = jcall!(self.componentFactory).SCSI.clone(); java_field }).toString()) as i32) == 0 {
                    return Some((HardDriveController::new(componentData.clone())).clone() /* as HardwareComponentInterface */);
                } else if (component.cmp(&jcall!({ let java_field = jcall!(self.componentFactory).USB.clone(); java_field }).toString()) as i32) == 0 {
                    return Some((Usb::new(componentData.clone())).clone() /* as HardwareComponentInterface */);
                } else if (component.cmp(&jcall!({ let java_field = jcall!(self.componentFactory).VGA.clone(); java_field }).toString()) as i32) == 0 {
                    return Some((Video::new(componentData.clone())).clone() /* as HardwareComponentInterface */);
                } else if (component.cmp(&jcall!({ let java_field = jcall!(self.componentFactory).PCI.clone(); java_field }).toString()) as i32) == 0 {
                    return Some((PCI::new(componentData.clone())).clone() /* as HardwareComponentInterface */);
                }
            }
            return Some((Unknown::new(componentData.clone())).clone() /* as HardwareComponentInterface */);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            if jcall!(unsafe { crate::org::allbinary::logic::communication::log::config::r#type::LogConfigTypes::LogConfigTypes_LOGGING.clone() }).contains(crate::java_into_object({ let java_field = jcall!(LogConfigTypeFactory::getInstance()).FACTORYERROR.clone(); java_field }.clone())) {
                jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).GET_INSTANCE.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            return Unknown::new({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone()).map(|reference| reference as Rc<RefCell<dyn HardwareComponentInterface>>).map(|reference| reference as Rc<RefCell<dyn HardwareComponentInterface>>);
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn getComponentType(&mut self, mut component: String) -> String {
        if component.find(&jcall!({ let java_field = jcall!(self.componentFactory).ETHERNET.clone(); java_field }).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
            return jcall!({ let java_field = jcall!(self.componentFactory).ETHERNET.clone(); java_field }).toString();
        } else if component.find(&jcall!({ let java_field = jcall!(self.componentFactory).BRIDGE.clone(); java_field }).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
            return jcall!({ let java_field = jcall!(self.componentFactory).BRIDGE.clone(); java_field }).toString();
        } else if component.find(&jcall!({ let java_field = jcall!(self.componentFactory).IDE.clone(); java_field }).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
            return jcall!({ let java_field = jcall!(self.componentFactory).IDE.clone(); java_field }).toString();
        } else if component.find(&jcall!({ let java_field = jcall!(self.componentFactory).MULTIMEDIA.clone(); java_field }).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
            return jcall!({ let java_field = jcall!(self.componentFactory).MULTIMEDIA.clone(); java_field }).toString();
        } else if component.find(&jcall!({ let java_field = jcall!(self.componentFactory).SCSI.clone(); java_field }).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
            return jcall!({ let java_field = jcall!(self.componentFactory).SCSI.clone(); java_field }).toString();
        } else if component.find(&jcall!({ let java_field = jcall!(self.componentFactory).USB.clone(); java_field }).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
            return jcall!({ let java_field = jcall!(self.componentFactory).USB.clone(); java_field }).toString();
        } else if component.find(&jcall!({ let java_field = jcall!(self.componentFactory).VGA.clone(); java_field }).toString()).map(|index| index as i32).unwrap_or(-1) >= 0 {
            return jcall!({ let java_field = jcall!(self.componentFactory).VGA.clone(); java_field }).toString();
        } else if (component.cmp(&jcall!({ let java_field = jcall!(self.componentFactory).PCI.clone(); java_field }).toString()) as i32) == 0 {
            return jcall!({ let java_field = jcall!(self.componentFactory).PCI.clone(); java_field }).toString();
        }
        return jcall!({ let java_field = jcall!(self.componentFactory).UNKNOWN.clone(); java_field }).toString();
    }
}

