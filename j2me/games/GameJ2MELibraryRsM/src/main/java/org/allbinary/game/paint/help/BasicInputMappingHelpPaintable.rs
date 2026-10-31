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
// package org.allbinary.game.paint.help
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::GameKey::GameKey;
use crate::org::allbinary::game::input::GameKeyFactory::GameKeyFactory;
use crate::org::allbinary::game::input::Input::Input;
use crate::org::allbinary::game::input::PlatformInputMappingFactory::PlatformInputMappingFactory;
use crate::org::allbinary::game::input::mapping::GameInputMapping::GameInputMapping;
use crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMapping;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::HelpPaintable::HelpPaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicInputMappingHelpPaintable {
    pub base: HelpPaintable,
    pub gameInputMappingArray: Vec<Option<Rc<RefCell<GameInputMapping>>>>,
    pub NONE: Option<Rc<RefCell<GameKey>>>,
}


impl BasicInputMappingHelpPaintable {
    

    pub fn new_struct(mut gameInputMappingArray: Vec<Option<Rc<RefCell<GameInputMapping>>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: HelpPaintable::new_struct(String::from("Help Screen"), jcall!(BasicColorFactory::getInstance()).BLACK.clone(), basicColor.clone()),
            gameInputMappingArray: Vec::new(),
            NONE: None,
        };
        this.NONE = jcall!(GameKeyFactory::getInstance()).NONE.clone();
        // super(...) is in the struct literal above
        this.gameInputMappingArray = gameInputMappingArray.clone();
        this.update(this.NONE.clone(), this.NONE.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        this
    }

    pub fn new(mut gameInputMappingArray: Vec<Option<Rc<RefCell<GameInputMapping>>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameInputMappingArray, basicColor))))
    }
    

    pub fn update(&mut self, mut selectedGameKey: Option<Rc<RefCell<GameKey>>>, mut selectedInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) {
        let mut gameKeyMapping: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMappingDyn>>> = jcall!(PlatformInputMappingFactory::getInstance()).getPersistentInputMappingInstance();
        let mut EQUAL: String = String::from(" = ");
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut size: i32 = (self.gameInputMappingArray.len() as i32).clone();
        let mut keyInfo: Vec<String> = vec![String::new(); (size) as usize];
        let mut gameInputMapping: Option<Rc<RefCell<GameInputMapping>>> = None;
        let mut gameKey: Option<Rc<RefCell<GameKey>>> = None;
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        for index in (0..size) {
            gameInputMapping = self.gameInputMappingArray[(index) as usize].clone();
            gameKey = jcall!(gameInputMapping).getGameKey();
            list = jcall!(jcall!(gameKeyMapping).getInputMapping()).getMappedInput(gameKey.clone());
            jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
            keyInfo[(index) as usize] = jcall!(stringMaker).append(jcall!(gameInputMapping).getName()).append(EQUAL.clone()).append(self.get(list.clone())).toString();
        }
        self.base.setInputInfoP(keyInfo.clone());
    }
    

    pub fn get(&mut self, mut keyList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> String {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut AND: String = String::from("and ");
        let mut key: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = None;
        let mut size: i32 = jcall!(keyList).size();
        for index in (0..size) {
            key = { let java_object = (jcall!(keyList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(stringBuffer).append(jcall!(key).getName());
            if index + 1 < jcall!(keyList).size() {
                if jcall!(keyList).size() == 2 {
                    jcall!(stringBuffer).append(jcall!(commonSeps).SPACE.clone()).append(AND.clone());
                } else {
                    if index + 2 == jcall!(keyList).size() {
                        jcall!(stringBuffer).append(jcall!(commonSeps).COMMA_SEP.clone());
                        jcall!(stringBuffer).append(AND.clone());
                    } else {
                        jcall!(stringBuffer).append(jcall!(commonSeps).COMMA_SEP.clone());
                    }
                }
            }
        }
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for BasicInputMappingHelpPaintable {
    type Target = HelpPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicInputMappingHelpPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::paint::help::HelpPaintable::HelpPaintableDyn for BasicInputMappingHelpPaintable {
    fn as_HelpPaintable(&self) -> &crate::org::allbinary::game::paint::help::HelpPaintable::HelpPaintable { &self.base }
    fn as_HelpPaintable_mut(&mut self) -> &mut crate::org::allbinary::game::paint::help::HelpPaintable::HelpPaintable { &mut self.base }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for BasicInputMappingHelpPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base }
}


