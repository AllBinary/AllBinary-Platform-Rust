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
// package org.allbinary.game.init
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
use crate::org::allbinary::game::resource::ResourceInitialization::ResourceInitialization;
use crate::org::allbinary::game::resource::ResourceLoadingLevelFactory::ResourceLoadingLevelFactory;
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::BaseGameInitialization::BaseGameInitialization;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CommonGameInitialization {
    pub base: BaseGameInitialization,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

impl crate::org::allbinary::game::init::GameInitializationInterface::GameInitializationInterface for CommonGameInitialization {
    fn init(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut level: i32) { Self::init(self, abeClientInformation, commandListener, level) }
}

impl CommonGameInitialization {
    

    pub fn new_struct(mut resourceInitializationArray: Vec<Option<Rc<RefCell<ResourceInitialization>>>>, mut portion: i32) -> Self {
        let mut this = Self {
            base: BaseGameInitialization::new_struct(resourceInitializationArray.clone(), portion.clone()),
            commonStrings: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut resourceInitializationArray: Vec<Option<Rc<RefCell<ResourceInitialization>>>>, mut portion: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(resourceInitializationArray, portion))))
    }
    

    // throws Exception
    pub fn init(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut level: i32) {
        self.base.init(abeClientInformation.clone(), commandListener.clone(), level.clone());
        let mut resourceLoadingLevelFactory: Option<Rc<RefCell<ResourceLoadingLevelFactory>>> = ResourceLoadingLevelFactory::getInstance();
        if !self.isGameInitialized() && level == jcall!(jcall!(resourceLoadingLevelFactory).LOAD_ALL).getLevel() {
            self.setGameInitialized(true);
            let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
            jcall!(SmallIntegerSingletonFactory::getInstance()).init();
            jcall!(progressCanvas).addNormalPortion(50, String::from("Integers"));
            jcall!(PointFactory::getInstance()).init();
            jcall!(progressCanvas).addNormalPortion(50, String::from("Points"));
            AngleFactory::getInstance();
            jcall!(progressCanvas).addNormalPortion(50, String::from("Angles"));
            self.initGame();
            jcall!(self.resourceInitializationArray[(self.GAME_RESOURCES) as usize]).init();
        }
        self.base.resourceInitialization(level.clone());
    }
    

    // throws Exception
    pub fn initGame(&mut self) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
}
impl std::ops::Deref for CommonGameInitialization {
    type Target = BaseGameInitialization;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CommonGameInitialization {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::init::BaseGameInitialization::BaseGameInitializationDyn for CommonGameInitialization {
    fn as_BaseGameInitialization(&self) -> &crate::org::allbinary::game::init::BaseGameInitialization::BaseGameInitialization { &self.base }
    fn as_BaseGameInitialization_mut(&mut self) -> &mut crate::org::allbinary::game::init::BaseGameInitialization::BaseGameInitialization { &mut self.base }
    fn init(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut level: i32) { Self::init(self, abeClientInformation, commandListener, level) }
}


