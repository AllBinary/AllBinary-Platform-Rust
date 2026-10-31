// Generated Code Do Not Modify
// package org.allbinary.business.advertisement
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::GameAdState::GameAdState;
use crate::org::allbinary::logic::system::SoftwareInformation::SoftwareInformation;
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

thread_local! { pub static GameAdStateFactory_instance: Option<Rc<RefCell<GameAdStateFactory>>> = GameAdStateFactory::new(); }
impl GameAdStateFactoryInterface for GameAdStateFactory {
    fn getInstanceForApp(&mut self, mut softwareInformation: Option<Rc<RefCell<SoftwareInformation>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::GameAdStateBase::GameAdStateBaseDyn>>> { Self::getInstanceForApp(self, softwareInformation) }
}

impl GameAdStateFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            gameAdStateArray: Vec::new(),
            gameAdState: None,
        };
        this.gameAdStateArray = vec![GameAdState::new(AppNameLicensingAdConfiguration::new())];
        this.gameAdState = this.gameAdStateArray[(0) as usize];
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
        self.gameAdState = self.gameAdStateArray[(0) as usize].clone();
        return self.gameAdStateArray[(0) as usize].clone();
    }
    

    pub fn isEnabled(&mut self) -> bool {
        return false;
    }
}

