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
use crate::org::allbinary::game::configuration::event::ChangedGameFeatureListener::ChangedGameFeatureListener;
use crate::org::allbinary::game::configuration::event::GameInitializedEvent::GameInitializedEvent;
use crate::org::allbinary::game::configuration::event::GameInitializedEventHandler::GameInitializedEventHandler;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::MainFeatureFactory::MainFeatureFactory;
use crate::org::allbinary::game::resource::ResourceLoadingLevelFactory::ResourceLoadingLevelFactory;
use crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvas;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use super::GameInitializationInterface::GameInitializationInterface;
use super::BasicBuildGameInitializerFactory::BasicBuildGameInitializerFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameInitializationUtil {
    pub EVENT: Option<Rc<RefCell<GameInitializedEvent>>>,
}

thread_local! { pub static GameInitializationUtil_instance: Option<Rc<RefCell<GameInitializationUtil>>> = GameInitializationUtil::new(); }

impl GameInitializationUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            EVENT: None,
        };
        this.EVENT = GameInitializedEvent::new(this);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn initDemo(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut canvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>>, mut gameInitializationInterfaceFactoryInterface: Option<Rc<RefCell<BasicBuildGameInitializerFactory>>>) {
        let mut mainFeatureFactory: Option<Rc<RefCell<MainFeatureFactory>>> = MainFeatureFactory::getInstance();
        if jcall!(ChangedGameFeatureListener::getInstance()).isChangedFeature(jcall!(mainFeatureFactory).STATIC.clone()) {
            let mut gameInitializationInterface: Option<Rc<RefCell<dyn GameInitializationInterface>>> = jcall!(gameInitializationInterfaceFactoryInterface).getInstance();
            let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
            if jcall!(features).isFeature(jcall!(mainFeatureFactory).LOAD_ALL.clone()) {
                self.initGame(abeClientInformation.clone(), canvas.clone(), gameInitializationInterfaceFactoryInterface.clone());
            } else if jcall!(features).isFeature(jcall!(mainFeatureFactory).LOAD_ONDEMAND.clone()) {
                let mut resourceLoadingLevelFactory: Option<Rc<RefCell<ResourceLoadingLevelFactory>>> = ResourceLoadingLevelFactory::getInstance();
                jcall!(gameInitializationInterface).init(abeClientInformation.clone(), jcall!(canvas).getCustomCommandListener(), jcall!(jcall!(resourceLoadingLevelFactory).LOAD_EARLY).getLevel());
                jcall!(self.EVENT).setResourceLoadingLevel(jcall!(resourceLoadingLevelFactory).LOAD_EARLY.clone());
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Loading Feature Available"))));
            }
            jcall!(GameInitializedEventHandler::getInstance()).fireEvent(self.EVENT.clone());
        }
    }
    

    // throws Exception
    pub fn initGame(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut canvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::MyCanvas::MyCanvasDyn>>>, mut gameInitializationInterfaceFactoryInterface: Option<Rc<RefCell<BasicBuildGameInitializerFactory>>>) {
        if jcall!(ChangedGameFeatureListener::getInstance()).isChangedFeature(jcall!(MainFeatureFactory::getInstance()).STATIC.clone()) {
            let mut gameInitializationInterface: Option<Rc<RefCell<dyn GameInitializationInterface>>> = jcall!(gameInitializationInterfaceFactoryInterface).getInstance();
            let mut resourceLoadingLevelFactory: Option<Rc<RefCell<ResourceLoadingLevelFactory>>> = ResourceLoadingLevelFactory::getInstance();
            jcall!(gameInitializationInterface).init(abeClientInformation.clone(), jcall!(canvas).getCustomCommandListener(), jcall!(jcall!(resourceLoadingLevelFactory).LOAD_ALL).getLevel());
            jcall!(self.EVENT).setResourceLoadingLevel(jcall!(resourceLoadingLevelFactory).LOAD_ALL.clone());
            jcall!(GameInitializedEventHandler::getInstance()).fireEvent(self.EVENT.clone());
            jcall!(ChangedGameFeatureListener::getInstance()).remove(jcall!(MainFeatureFactory::getInstance()).STATIC.clone());
        }
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameInitializationUtil>>> {
        return GameInitializationUtil_instance.with(Clone::clone).clone();
    }
}

