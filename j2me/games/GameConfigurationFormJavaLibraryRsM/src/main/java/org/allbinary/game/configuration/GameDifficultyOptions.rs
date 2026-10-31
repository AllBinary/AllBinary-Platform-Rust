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
// package org.allbinary.game.configuration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Choice::Choice;
// use crate::javax::microedition::lcdui::ChoiceGroup::ChoiceGroup;
// use crate::javax::microedition::lcdui::CommandListener::CommandListener;
// use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::game::configuration::persistance::GameDifficultyFactory::GameDifficultyFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm;
use crate::org::allbinary::logic::MEUtil::MEUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameDifficultyOptions {
    pub base: CommandForm,
    pub meUtil: Option<Rc<RefCell<MEUtil>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for GameDifficultyOptions {
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
}

impl GameDifficultyOptions {
    

    pub fn new_struct(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut title: String, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: CommandForm::new_struct(commandListener.clone(), title.clone(), backgrounBasicColor.clone(), foregroundBasicColor.clone()),
            meUtil: None,
            list: None,
        };
        this.meUtil = MEUtil::getInstance();
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.list = list.clone();
        this.update();
        this.initCommands(commandListener.clone());
        let mut gameDifficultyFactory: Option<Rc<RefCell<GameDifficultyFactory>>> = GameDifficultyFactory::getInstance();
        { let java_arguments = (jcall!(gameDifficultyFactory).getLevel(),); this.setSelectedId(java_arguments.0) };
        this
    }

    pub fn new(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut title: String, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(commandListener, title, list, backgrounBasicColor, foregroundBasicColor))))
    }
    

    pub fn open(&mut self) {
        self.base.open();
    }
    

    // throws Exception
    pub fn close(&mut self) {
        self.base.close();
        self.save();
    }
    

    // throws Exception
    pub fn save(&mut self) {
        let mut gameDifficultyFactory: Option<Rc<RefCell<GameDifficultyFactory>>> = GameDifficultyFactory::getInstance();
        jcall!(gameDifficultyFactory).setLevel(self.getSelectedId());
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
        let mut gameCommandsFactory: Option<Rc<RefCell<GameCommandsFactory>>> = GameCommandsFactory::getInstance();
        self.removeAllCommands();
        self.addCommand(jcall!(gameCommandsFactory).CLOSE_AND_SHOW_GAME_CANVAS.clone());
        self.setCommandListener(cmdListener.clone());
    }
    

    // throws Exception
    pub fn update(&mut self) {
        self.deleteAll();
        self.add(self.list.clone(), String::from("Difficulty"), unsafe { crate::javax::microedition::lcdui::Choice::Choice_EXCLUSIVE.clone() }.clone());
        self.base.update();
    }
    

    pub fn add(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut name: String, mut option: i32) {
        let mut choiceGroup: Option<Rc<RefCell<ChoiceGroup>>> = self.getChoiceGroup(list.clone(), name.clone(), option.clone());
        if jcall!(list).size() > 0 {
            jcall!(choiceGroup).setSelectedIndex(0, true);
        }
        jcall!(self.meUtil).appendItem(None /* Java this: a &mut self method has no Rc of itself */, choiceGroup.clone());
    }
    

    pub fn getChoiceGroup(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut name: String, mut option: i32) -> Option<Rc<RefCell<ChoiceGroup>>> {
        let mut METHOD_NAME: String = String::from("addChoiceGroup");
        let mut NAME: String = String::from("Adding Choice: ");
        let mut choiceGroup: Option<Rc<RefCell<ChoiceGroup>>> = ChoiceGroup::new(name.clone(), option.clone(), jcall!(StringUtil::getInstance()).getArrayInstance(), crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE_ARRAY.with(Clone::clone).clone());
        let mut size: i32 = jcall!(list).size();
        let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = None;
        for index in (0..size) {
            anyType = jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize].clone();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(NAME.clone()).append({ let value = jcall!(anyType); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) }).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(choiceGroup).append({ let value = jcall!(anyType); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) }, crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone());
        }
        return choiceGroup.clone();
    }
    

    pub fn setSelectedId(&mut self, mut id: i32) {
        let mut choiceGroup: Option<Rc<RefCell<ChoiceGroup>>> = (self.get(0)).clone() /* as ChoiceGroup */;
        jcall!(choiceGroup).setSelectedIndex(id.clone(), true);
    }
    

    pub fn getSelectedId(&mut self) -> i32 {
        let mut choiceGroup: Option<Rc<RefCell<ChoiceGroup>>> = (self.get(0)).clone() /* as ChoiceGroup */;
        return jcall!(choiceGroup).getSelectedIndex();
    }
    

    pub fn getSelectedText(&mut self) -> i32 {
        let mut choiceGroup: Option<Rc<RefCell<ChoiceGroup>>> = (self.get(0)).clone() /* as ChoiceGroup */;
        let mut value: i32 = (((jcall!(choiceGroup).getString(jcall!(choiceGroup).getSelectedIndex())).to_string()).unwrap() as i32);
        return value.clone();
    }
}
impl std::ops::Deref for GameDifficultyOptions {
    type Target = CommandForm;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameDifficultyOptions {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn for GameDifficultyOptions {
    fn as_CommandForm(&self) -> &crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm { &self.base }
    fn as_CommandForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm { &mut self.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) { Self::initCommands(self, cmdListener) }
    fn open(&mut self) { Self::open(self) }
    fn close(&mut self) { Self::close(self) }
    fn update(&mut self) { Self::update(self) }
}


