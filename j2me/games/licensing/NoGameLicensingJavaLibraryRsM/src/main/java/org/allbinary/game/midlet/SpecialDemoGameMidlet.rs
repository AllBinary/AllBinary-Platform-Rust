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
// package org.allbinary.game.midlet
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::system::security::licensing::ClientInformationFactory::ClientInformationFactory;
use crate::org::allbinary::logic::system::security::licensing::LicenseServerInitFileUtil::LicenseServerInitFileUtil;
use super::DemoGameMidlet::DemoGameMidlet;
use super::LicenseLoadingType::LicenseLoadingType;
use super::LicensedDemoSetupFactory::LicensedDemoSetupFactory;
use super::LicenseCheckRunnableFactory::LicenseCheckRunnableFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SpecialDemoGameMidlet {
    pub base: DemoGameMidlet,
}


impl SpecialDemoGameMidlet {
    

    pub fn new_struct(mut clientInformationFactory: Option<Rc<RefCell<ClientInformationFactory>>>, mut licenseLoadingType: Option<Rc<RefCell<LicenseLoadingType>>>, mut demoSetupFactory: Option<Rc<RefCell<LicensedDemoSetupFactory>>>, mut licenseCheckRunnableFactory: Option<Rc<RefCell<LicenseCheckRunnableFactory>>>) -> Self {
        let mut this = Self {
            base: DemoGameMidlet::new_struct(clientInformationFactory.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut clientInformationFactory: Option<Rc<RefCell<ClientInformationFactory>>>, mut licenseLoadingType: Option<Rc<RefCell<LicenseLoadingType>>>, mut demoSetupFactory: Option<Rc<RefCell<LicensedDemoSetupFactory>>>, mut licenseCheckRunnableFactory: Option<Rc<RefCell<LicenseCheckRunnableFactory>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(clientInformationFactory, licenseLoadingType, demoSetupFactory, licenseCheckRunnableFactory))))
    }
    

    pub fn initView(&mut self) {
    }
    

    pub fn preInit(&mut self) {
        jcall!(LicenseServerInitFileUtil::new()).init();
    }
}
impl std::ops::Deref for SpecialDemoGameMidlet {
    type Target = DemoGameMidlet;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SpecialDemoGameMidlet {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait SpecialDemoGameMidletDyn: crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidletDyn {
    fn as_SpecialDemoGameMidlet(&self) -> &SpecialDemoGameMidlet;
    fn as_SpecialDemoGameMidlet_mut(&mut self) -> &mut SpecialDemoGameMidlet;
    fn initView(&mut self) { SpecialDemoGameMidlet::initView(self.as_SpecialDemoGameMidlet_mut()) }
}
impl SpecialDemoGameMidletDyn for SpecialDemoGameMidlet {
    fn as_SpecialDemoGameMidlet(&self) -> &SpecialDemoGameMidlet { self }
    fn as_SpecialDemoGameMidlet_mut(&mut self) -> &mut SpecialDemoGameMidlet { self }
}

impl crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidletDyn for SpecialDemoGameMidlet {
    fn as_DemoGameMidlet(&self) -> &crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidlet { &self.base }
    fn as_DemoGameMidlet_mut(&mut self) -> &mut crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidlet { &mut self.base }
}
impl crate::org::allbinary::game::midlet::GameMidlet::GameMidletDyn for SpecialDemoGameMidlet {
    fn as_GameMidlet(&self) -> &crate::org::allbinary::game::midlet::GameMidlet::GameMidlet { &self.base.base }
    fn as_GameMidlet_mut(&mut self) -> &mut crate::org::allbinary::game::midlet::GameMidlet::GameMidlet { &mut self.base.base }
    fn setDemo(&mut self) { crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidlet::setDemo(crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidletDyn::as_DemoGameMidlet_mut(self)) }
    fn createGame(&mut self) { crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidlet::createGame(crate::org::allbinary::game::midlet::DemoGameMidlet::DemoGameMidletDyn::as_DemoGameMidlet_mut(self)) }
}
impl crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidletDyn for SpecialDemoGameMidlet {
    fn as_AllBinaryMidlet(&self) -> &crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet { &self.base.base.base.base }
    fn as_AllBinaryMidlet_mut(&mut self) -> &mut crate::org::allbinary::midlet::AllBinaryMidlet::AllBinaryMidlet { &mut self.base.base.base.base }
    fn startApp(&mut self) { crate::org::allbinary::game::midlet::GameMidlet::GameMidlet::startApp(crate::org::allbinary::game::midlet::GameMidlet::GameMidletDyn::as_GameMidlet_mut(self)) }
    fn pauseApp(&mut self) { crate::org::allbinary::game::midlet::GameMidlet::GameMidlet::pauseApp(crate::org::allbinary::game::midlet::GameMidlet::GameMidletDyn::as_GameMidlet_mut(self)) }
    fn destroyAppInRunnable(&mut self, p1: bool, p2: bool) { crate::org::allbinary::game::midlet::GameMidlet::GameMidlet::destroyAppInRunnable(crate::org::allbinary::game::midlet::GameMidlet::GameMidletDyn::as_GameMidlet_mut(self), p1, p2) }
    fn destroyApp(&mut self, p1: bool) { crate::org::allbinary::game::midlet::GameMidlet::GameMidlet::destroyApp(crate::org::allbinary::game::midlet::GameMidlet::GameMidletDyn::as_GameMidlet_mut(self), p1) }
}
impl crate::org::allbinary::midlet::ProgressMidlet::ProgressMidletDyn for SpecialDemoGameMidlet {
    fn as_ProgressMidlet(&self) -> &crate::org::allbinary::midlet::ProgressMidlet::ProgressMidlet { &self.base.base.base }
    fn as_ProgressMidlet_mut(&mut self) -> &mut crate::org::allbinary::midlet::ProgressMidlet::ProgressMidlet { &mut self.base.base.base }
    fn preInit(&mut self) { Self::preInit(self) }
}


