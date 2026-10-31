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
// use crate::javax::microedition::lcdui::Item::Item;
// use crate::javax::microedition::lcdui::NullImage::NullImage;
// use crate::javax::microedition::lcdui::StringItem::StringItem;
use crate::org::allbinary::game::commands::GameCommandsFactory::GameCommandsFactory;
use crate::org::allbinary::game::configuration::persistance::GamePersistanceSingleton::GamePersistanceSingleton;
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

pub struct LoadGameForm {
    pub base: CommandForm,
    pub meUtil: Option<Rc<RefCell<MEUtil>>>,
    pub areChoices: bool,
}

impl crate::org::allbinary::game::displayable::canvas::MenuListener::MenuListener for LoadGameForm {
    fn open(&mut self) { self.base.open() }
    fn close(&mut self) { self.base.close() }
}

impl LoadGameForm {
    

    pub fn new_struct(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut title: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: CommandForm::new_struct(commandListener.clone(), title.clone(), backgrounBasicColor.clone(), foregroundBasicColor.clone()),
            meUtil: None,
            areChoices: false,
        };
        this.meUtil = MEUtil::getInstance();
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.update();
        this.initCommands(commandListener.clone());
        this
    }

    pub fn new(mut commandListener: Option<Rc<RefCell<CommandListener>>>, mut title: String, mut backgrounBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(commandListener, title, backgrounBasicColor, foregroundBasicColor))))
    }
    

    pub fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) {
        let mut gameCommandsFactory: Option<Rc<RefCell<GameCommandsFactory>>> = GameCommandsFactory::getInstance();
        self.removeAllCommands();
        self.addCommand(jcall!(gameCommandsFactory).LOAD_FILE.clone());
        self.addCommand(jcall!(gameCommandsFactory).DELETE_FILE.clone());
        self.addCommand(jcall!(gameCommandsFactory).CLOSE_AND_SHOW_GAME_CANVAS.clone());
        self.setCommandListener(cmdListener.clone());
    }
    

    // throws Exception
    pub fn update(&mut self) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(GamePersistanceSingleton::getInstance()).getIds();
        self.deleteAll();
        if jcall!(list).size() > 0 {
            self.add(list.clone(), String::from("Select A Saved Game"), unsafe { crate::javax::microedition::lcdui::Choice::Choice_EXCLUSIVE.clone() }.clone());
            self.areChoices = true;
        } else {
            { let java_receiver = self.meUtil.clone(); let java_arguments = (None /* Java this: a &mut self method has no Rc of itself */, StringItem::new(String::from("No Saved Games"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Item::Item_PLAIN.clone() }.clone()),); jcall!(java_receiver).appendItem(java_arguments.0, java_arguments.1); };
            self.areChoices = false;
        }
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
        let mut METHOD_NAME: String = String::from("getChoiceGroup");
        let mut ADDING_CHOICE: String = String::from("Adding Choice: ");
        let mut choiceGroup: Option<Rc<RefCell<ChoiceGroup>>> = ChoiceGroup::new(name.clone(), option.clone(), jcall!(StringUtil::getInstance()).getArrayInstance(), crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE_ARRAY.with(Clone::clone).clone());
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize].clone();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(ADDING_CHOICE.clone()).append({ let value = jcall!(anyType); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) }).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(choiceGroup).append({ let value = jcall!(anyType); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) }, crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone());
        }
        return choiceGroup.clone();
    }
    

    pub fn getSelectedId(&mut self) -> i32 {
        if self.areChoices {
            let mut choiceGroup: Option<Rc<RefCell<ChoiceGroup>>> = (self.get(0)).clone() /* as ChoiceGroup */;
            return jcall!(choiceGroup).getSelectedIndex();
        } else {
            return -1;
        }
    }
    

    pub fn getSelectedText(&mut self) -> i32 {
        if self.areChoices {
            let mut choiceGroup: Option<Rc<RefCell<ChoiceGroup>>> = (self.get(0)).clone() /* as ChoiceGroup */;
            let mut value: i32 = (((jcall!(choiceGroup).getString(jcall!(choiceGroup).getSelectedIndex())).to_string()).unwrap() as i32);
            return value.clone();
        } else {
            return -1;
        }
    }
}
impl std::ops::Deref for LoadGameForm {
    type Target = CommandForm;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LoadGameForm {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn for LoadGameForm {
    fn as_CommandForm(&self) -> &crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm { &self.base }
    fn as_CommandForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm { &mut self.base }
    fn initCommands(&mut self, mut cmdListener: Option<Rc<RefCell<CommandListener>>>) { Self::initCommands(self, cmdListener) }
    fn update(&mut self) { Self::update(self) }
}


