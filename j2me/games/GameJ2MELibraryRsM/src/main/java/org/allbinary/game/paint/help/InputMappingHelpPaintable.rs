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
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::input::GameKey::GameKey;
use crate::org::allbinary::game::input::GameKeyFactory::GameKeyFactory;
use crate::org::allbinary::game::input::Input::Input;
use crate::org::allbinary::game::input::PlatformInputMappingFactory::PlatformInputMappingFactory;
use crate::org::allbinary::game::input::mapping::GameInputMapping::GameInputMapping;
use crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMapping;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::HelpPaintable::HelpPaintable;
use super::UpdateMyFontProperties::UpdateMyFontProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputMappingHelpPaintable {
    pub base: HelpPaintable,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
    pub basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>>,
    pub NONE: Option<Rc<RefCell<GameKey>>>,
    pub EMPTY_STRING: String,
    pub gameInputMappingArray: Vec<Option<Rc<RefCell<GameInputMapping>>>>,
    pub keyMappingArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>,
    pub actionBasicColor: Vec<Option<Rc<RefCell<BasicColor>>>>,
    pub inputBasicColorArray: Vec<Vec<Option<Rc<RefCell<BasicColor>>>>>,
    pub selectedBasicColor: Option<Rc<RefCell<BasicColor>>>,
    pub updateMyFontProperties: Option<Rc<RefCell<UpdateMyFontProperties>>>,
}

thread_local! { pub static InputMappingHelpPaintable_AND: String = String::from(" and "); }
thread_local! { pub static InputMappingHelpPaintable_SEP: String = String::from(", "); }
thread_local! { pub static InputMappingHelpPaintable_MORE_THAN_TWO_IN_LIST_AND: String = String::from(", and "); }
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for InputMappingHelpPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl InputMappingHelpPaintable {
    

    pub fn new_struct(mut gameInputMappingArray: Vec<Option<Rc<RefCell<GameInputMapping>>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: HelpPaintable::new_struct(String::from("Input Mapping"), backgroundBasicColor.clone(), basicColor.clone()),
            logUtil: None,
            commonStrings: None,
            commonSeps: None,
            stringUtil: None,
            basicColorFactory: None,
            NONE: None,
            EMPTY_STRING: String::new(),
            gameInputMappingArray: Vec::new(),
            keyMappingArray: Vec::new(),
            actionBasicColor: Vec::new(),
            inputBasicColorArray: Vec::new(),
            selectedBasicColor: None,
            updateMyFontProperties: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.stringUtil = StringUtil::getInstance();
        this.basicColorFactory = BasicColorFactory::getInstance();
        this.NONE = jcall!(GameKeyFactory::getInstance()).NONE.clone();
        this.EMPTY_STRING = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.keyMappingArray = vec![None; (0) as usize];
        this.actionBasicColor = vec![None; (0) as usize];
        this.inputBasicColorArray = vec![vec![None; (0) as usize]; (0) as usize];
        this.updateMyFontProperties = UpdateMyFontProperties::new();
        // super(...) is in the struct literal above
        this.gameInputMappingArray = gameInputMappingArray.clone();
        this.update(this.NONE.clone(), this.NONE.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
        if match (&backgroundBasicColor, &jcall!(this.basicColorFactory).WHITE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&basicColor, &jcall!(this.basicColorFactory).WHITE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            this.selectedBasicColor = jcall!(this.basicColorFactory).RED.clone();
        } else {
            this.selectedBasicColor = jcall!(this.basicColorFactory).WHITE.clone();
        }
        this
    }

    pub fn new(mut gameInputMappingArray: Vec<Option<Rc<RefCell<GameInputMapping>>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(gameInputMappingArray, backgroundBasicColor, basicColor))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        let mut updateMyFontProperties: Option<Rc<RefCell<UpdateMyFontProperties>>> = UpdateMyFontProperties::new();
        jcall!(updateMyFontProperties).charHeight = jcall!(font).getHeight();
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut inputInfo: Vec<String> = self.inputInfo.clone();
        let mut size: i32 = (inputInfo.len() as i32).clone();
        jcall!(updateMyFontProperties).actionStringArray = vec![String::new(); (size) as usize];
        jcall!(updateMyFontProperties).keymappingBeginWidthArray = vec![0; (size) as usize];
        jcall!(updateMyFontProperties).actionStringDeltaXArray = vec![0; (size) as usize];
        jcall!(updateMyFontProperties).inputDeltaXArray = vec![vec![0; (0) as usize]; (size) as usize];
        jcall!(updateMyFontProperties).sepArray = vec![vec![String::new(); (0) as usize]; (size) as usize];
        jcall!(updateMyFontProperties).sepWidthArray = vec![vec![0; (0) as usize]; (size) as usize];
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        let mut keyMappings: String = String::new();
        let mut actionString: String = String::new();
        let mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = None;
        let mut size2: i32 = 0;
        for index in (0..size) {
            list = self.keyMappingArray[(index) as usize].clone();
            keyMappings = self.get(list.clone());
            jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
            actionString = jcall!(stringMaker).append(inputInfo[(index) as usize].clone()).append(jcall!(self.commonSeps).COLON.clone()).append(jcall!(self.commonSeps).SPACE.clone()).append(jcall!(self.commonSeps).SPACE.clone()).toString();
            jcall!(updateMyFontProperties).actionStringArray[(index) as usize] = actionString.clone();
            jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
            jcall!(updateMyFontProperties).keymappingBeginWidthArray[(index) as usize] = (jcall!(font).stringWidth(jcall!(stringMaker).append(jcall!(updateMyFontProperties).actionStringArray[(index) as usize].clone()).append(keyMappings.clone()).toString()) >> 1);
            jcall!(updateMyFontProperties).actionStringDeltaXArray[(index) as usize] = jcall!(font).stringWidth(actionString.clone());
            size2 = jcall!(list).size();
            jcall!(updateMyFontProperties).inputDeltaXArray[(index) as usize] = vec![0; (size2) as usize];
            jcall!(updateMyFontProperties).sepArray[(index) as usize] = vec![String::new(); (size2) as usize];
            jcall!(updateMyFontProperties).sepWidthArray[(index) as usize] = vec![0; (size2) as usize];
            for index2 in (0..size2) {
                input = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index2) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>>().expect("ClassCastException").clone()) };
                jcall!(updateMyFontProperties).inputDeltaXArray[(index) as usize][(index2) as usize] = jcall!(font).stringWidth(jcall!(input).getName());
                jcall!(updateMyFontProperties).sepArray[(index) as usize][(index2) as usize] = self.EMPTY_STRING.clone();
                if index2 + 1 < jcall!(list).size() {
                    if jcall!(list).size() == 2 {
                        jcall!(updateMyFontProperties).sepArray[(index) as usize][(index2) as usize] = InputMappingHelpPaintable_AND.with(Clone::clone).clone();
                    } else {
                        if index2 + 2 == jcall!(list).size() {
                            jcall!(updateMyFontProperties).sepArray[(index) as usize][(index2) as usize] = InputMappingHelpPaintable_MORE_THAN_TWO_IN_LIST_AND.with(Clone::clone).clone();
                        } else {
                            jcall!(updateMyFontProperties).sepArray[(index) as usize][(index2) as usize] = InputMappingHelpPaintable_SEP.with(Clone::clone).clone();
                        }
                    }
                }
                if jcall!(updateMyFontProperties).sepArray[(index) as usize][(index2) as usize] != self.EMPTY_STRING {
                    jcall!(updateMyFontProperties).sepWidthArray[(index) as usize][(index2) as usize] = jcall!(font).stringWidth(jcall!(updateMyFontProperties).sepArray[(index) as usize][(index2) as usize].clone());
                }
            }
        }
        self.updateMyFontProperties = updateMyFontProperties.clone();
        self.base.updateMeasurement(graphics.clone());
    }
    

    pub fn update(&mut self, mut selectedGameKey: Option<Rc<RefCell<GameKey>>>, mut selectedInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) {
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(jcall!(CommonLabels::getInstance()).START_LABEL.clone()).append(String::from("selected GameKey: ")).append(jcall!(self.stringUtil).toString(crate::java_into_object(selectedGameKey.clone()))).append(String::from(" Input: ")).append(jcall!(self.stringUtil).toString(crate::java_into_object(selectedInput.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut gameKeyMapping: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMappingDyn>>> = jcall!(PlatformInputMappingFactory::getInstance()).getPersistentInputMappingInstance();
        let mut size: i32 = (self.gameInputMappingArray.len() as i32).clone();
        let mut keyInfo: Vec<String> = vec![String::new(); (size) as usize];
        let mut keyMappingArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>> = vec![None; (size) as usize];
        let mut actionBasicColor: Vec<Option<Rc<RefCell<BasicColor>>>> = vec![None; (size) as usize];
        let mut inputBasicColorArray: Vec<Vec<Option<Rc<RefCell<BasicColor>>>>> = vec![Vec::new(); (size) as usize];
        let mut gameInputMapping: Option<Rc<RefCell<GameInputMapping>>> = None;
        let mut gameKey: Option<Rc<RefCell<GameKey>>> = None;
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        for index in (0..size) {
            gameInputMapping = self.gameInputMappingArray[(index) as usize].clone();
            gameKey = jcall!(gameInputMapping).getGameKey();
            list = jcall!(jcall!(gameKeyMapping).getInputMapping()).getMappedInput(gameKey.clone());
            let mut size2: i32 = jcall!(list).size();
            inputBasicColorArray[(index) as usize] = vec![None; (size2) as usize];
            for index2 in (0..size2) {
                inputBasicColorArray[(index) as usize][(index2) as usize] = self.basicColor.clone();
            }
            if match (&gameKey, &selectedGameKey) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(String::from("Found: selected GameKey: ")).append(jcall!(self.stringUtil).toString(crate::java_into_object(selectedGameKey.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                actionBasicColor[(index) as usize] = self.selectedBasicColor.clone();
                let mut indexOfSelectedInput: i32 = jcall!(list).indexOf(crate::java_into_object(selectedInput.clone()));
                if indexOfSelectedInput >= 0 {
                    jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(String::from("Found: selected Input: ")).append(jcall!(self.stringUtil).toString(crate::java_into_object(selectedInput.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    inputBasicColorArray[(index) as usize][(indexOfSelectedInput) as usize] = self.selectedBasicColor.clone();
                }
            } else {
                actionBasicColor[(index) as usize] = self.basicColor.clone();
            }
            keyInfo[(index) as usize] = jcall!(gameInputMapping).getName();
            keyMappingArray[(index) as usize] = list.clone();
        }
        self.keyMappingArray = keyMappingArray.clone();
        self.actionBasicColor = actionBasicColor.clone();
        self.inputBasicColorArray = inputBasicColorArray.clone();
        self.base.setInputInfoP(keyInfo.clone());
        self.myFontProcessor = self.updateMyFontProcessor.clone();
    }
    

    pub fn get(&mut self, mut keyList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut key: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = None;
        let mut size: i32 = jcall!(keyList).size();
        for index in (0..size) {
            key = { let java_object = (jcall!(keyList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(stringBuffer).append(jcall!(key).getName());
            if index + 1 < jcall!(keyList).size() {
                if jcall!(keyList).size() == 2 {
                    jcall!(stringBuffer).append(InputMappingHelpPaintable_AND.with(Clone::clone).clone());
                } else {
                    if index + 2 == jcall!(keyList).size() {
                        jcall!(stringBuffer).append(InputMappingHelpPaintable_MORE_THAN_TWO_IN_LIST_AND.with(Clone::clone).clone());
                    } else {
                        jcall!(stringBuffer).append(InputMappingHelpPaintable_SEP.with(Clone::clone).clone());
                    }
                }
            }
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        let mut halfWidth: i32 = jcall!(self.displayInfo).getLastHalfWidth();
        jcall!(graphics).setColor(jcall!(self.basicColor).intValue());
        jcall!(graphics).drawString(self.title.clone(), halfWidth - self.titleBeginWidth, jcall!(self.updateMyFontProperties).charHeight.clone(), self.anchor.clone());
        let mut inputInfo: Vec<String> = self.inputInfo.clone();
        let mut size: i32 = (inputInfo.len() as i32).clone();
        let mut y: i32 = 0;
        let mut deltaX: i32 = 0;
        let mut size2: i32 = 0;
        let mut actionString: String = String::new();
        let mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = None;
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        let mut sep: String = String::new();
        let mut beginWidth: i32 = 0;
        for index in (0..size) {
            y = (index + 3) * jcall!(self.updateMyFontProperties).charHeight;
            deltaX = 0;
            list = self.keyMappingArray[(index) as usize].clone();
            size2 = jcall!(list).size();
            beginWidth = jcall!(self.updateMyFontProperties).keymappingBeginWidthArray[(index) as usize].clone();
            jcall!(graphics).setColor(jcall!(self.actionBasicColor[(index) as usize]).intValue());
            actionString = jcall!(self.updateMyFontProperties).actionStringArray[(index) as usize].clone();
            jcall!(graphics).drawString(actionString.clone(), halfWidth - beginWidth + deltaX, y.clone(), self.anchor.clone());
            deltaX += jcall!(self.updateMyFontProperties).actionStringDeltaXArray[(index) as usize].clone();
            for index2 in (0..size2) {
                input = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index2) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>>().expect("ClassCastException").clone()) };
                jcall!(graphics).setColor(((self.inputBasicColorArray[(index) as usize][(index2) as usize]).unwrap() as i32));
                jcall!(graphics).drawString(jcall!(input).getName(), halfWidth - beginWidth + deltaX, y.clone(), self.anchor.clone());
                deltaX += jcall!(self.updateMyFontProperties).inputDeltaXArray[(index) as usize][(index2) as usize].clone();
                sep = jcall!(self.updateMyFontProperties).sepArray[(index) as usize][(index2) as usize].clone();
                jcall!(graphics).setColor(jcall!(self.basicColor).intValue());
                jcall!(graphics).drawString(sep.clone(), halfWidth - beginWidth + deltaX, y.clone(), self.anchor.clone());
                deltaX += jcall!(self.updateMyFontProperties).sepWidthArray[(index) as usize][(index2) as usize].clone();
            }
        }
    }
}
impl std::ops::Deref for InputMappingHelpPaintable {
    type Target = HelpPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InputMappingHelpPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::paint::help::HelpPaintable::HelpPaintableDyn for InputMappingHelpPaintable {
    fn as_HelpPaintable(&self) -> &crate::org::allbinary::game::paint::help::HelpPaintable::HelpPaintable { &self.base }
    fn as_HelpPaintable_mut(&mut self) -> &mut crate::org::allbinary::game::paint::help::HelpPaintable::HelpPaintable { &mut self.base }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for InputMappingHelpPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


pub struct UpdateMyFontProperties {
    pub actionStringArray: Vec<String>,
    pub keymappingBeginWidthArray: Vec<i32>,
    pub actionStringDeltaXArray: Vec<i32>,
    pub inputDeltaXArray: Vec<Vec<i32>>,
    pub sepArray: Vec<Vec<String>>,
    pub sepWidthArray: Vec<Vec<i32>>,
    pub charHeight: i32,
}


impl UpdateMyFontProperties {

    pub fn new_struct() -> Self {
        let mut this = Self {
            actionStringArray: Vec::new(),
            keymappingBeginWidthArray: Vec::new(),
            actionStringDeltaXArray: Vec::new(),
            inputDeltaXArray: Vec::new(),
            sepArray: Vec::new(),
            sepWidthArray: Vec::new(),
            charHeight: 0,
        };
        this.actionStringArray = jcall!(StringUtil::getInstance()).getArrayInstance();
        this.keymappingBeginWidthArray = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY.clone();
        this.actionStringDeltaXArray = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY.clone();
        this.inputDeltaXArray = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY_ARRAY.clone();
        this.sepArray = vec![vec![String::new(); (0) as usize]; (0) as usize];
        this.sepWidthArray = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY_ARRAY.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

