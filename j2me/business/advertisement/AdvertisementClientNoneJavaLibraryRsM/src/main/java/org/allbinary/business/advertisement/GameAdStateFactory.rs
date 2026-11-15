// Generated Code Do Not Modify
// package org.allbinary.business.advertisement
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::GameAdState::GameAdState;
use crate::org::allbinary::logic::system::SoftwareInformation::SoftwareInformation;
use crate::java::lang::Exception::*;
use super::GameAdStateFactoryInterface::GameAdStateFactoryInterface;
use super::AppNameLicensingAdConfiguration::AppNameLicensingAdConfiguration;
use super::GameAdStateBase::GameAdStateBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameAdStateFactory {
    pub gameAdStateArray: Vec<Option<Rc<RefCell<GameAdState>>>>,
    pub gameAdState: Option<Rc<RefCell<GameAdState>>>,
}

pub static GameAdStateFactory_instance: crate::JavaStatic<Option<Rc<RefCell<GameAdStateFactory>>>> = crate::JavaStatic::new(|| GameAdStateFactory::new());
impl GameAdStateFactoryInterface for GameAdStateFactory {
    fn getInstanceForApp(&mut self, mut softwareInformation: Option<Rc<RefCell<SoftwareInformation>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::GameAdStateBase::GameAdStateBaseDyn>>> { Self::getInstanceForApp(self, softwareInformation) }
}

impl GameAdStateFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            gameAdStateArray: Vec::new(),
            gameAdState: None,
        };
        this.gameAdStateArray = vec![GameAdState::new(AppNameLicensingAdConfiguration::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::advertisement::AdConfiguration::AdConfigurationDyn>>))];
        this.gameAdState = this.gameAdStateArray[(0) as usize].clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameAdStateFactory>>> {
        return GameAdStateFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn getCurrentInstance(&mut self) -> Option<Rc<RefCell<GameAdState>>> {
        return self.gameAdState.clone();
    }
    

    // throws Exception
    pub fn getInstanceForApp(&mut self, mut softwareInformation: Option<Rc<RefCell<SoftwareInformation>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::GameAdStateBase::GameAdStateBaseDyn>>> {
        self.gameAdState = { let java_value = (self.gameAdStateArray[(0) as usize]).clone(); java_value }.clone();
        return self.gameAdStateArray[(0) as usize].clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::advertisement::GameAdStateBase::GameAdStateBaseDyn>>);
    }
    

    pub fn isEnabled(&mut self) -> bool {
        return false;
    }
}

