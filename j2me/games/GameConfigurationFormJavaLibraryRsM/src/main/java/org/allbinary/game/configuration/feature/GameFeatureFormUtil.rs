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
// package org.allbinary.game.configuration.feature
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::ChoiceGroup::ChoiceGroup;
// use crate::javax::microedition::lcdui::Item::Item;
// use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::org::allbinary::game::configuration::GameConfigurationGauge::GameConfigurationGauge;
use crate::org::allbinary::game::configuration::GameConfigurationUtil::GameConfigurationUtil;
use crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandForm;
use crate::org::allbinary::logic::MEUtil::MEUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::HashtableUtil::HashtableUtil;
use super::Features::Features;
use super::Feature::Feature;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameFeatureFormUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub meUtil: Option<Rc<RefCell<MEUtil>>>,
}

thread_local! { pub static GameFeatureFormUtil_instance: Option<Rc<RefCell<GameFeatureFormUtil>>> = GameFeatureFormUtil::new(); }

impl GameFeatureFormUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            meUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.meUtil = MEUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameFeatureFormUtil>>> {
        return GameFeatureFormUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getChoiceGroup(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut name: String, mut option: i32) -> Option<Rc<RefCell<ChoiceGroup>>> {
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut choiceGroup: Option<Rc<RefCell<ChoiceGroup>>> = ChoiceGroup::new(name.clone(), option.clone(), jcall!(StringUtil::getInstance()).getArrayInstance(), crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE_ARRAY.with(Clone::clone).clone());
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_object = (jcall!(hashtable).get(crate::java_into_object((name).clone() /* as Any */))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        let mut ADD_CHOICE: String = String::from(": Adding Choice: ");
        let mut GET_CHOICE_GROUP: String = String::from("getChoiceGroup");
        let mut size: i32 = jcall!(list).size();
        for index in (0..size) {
            let mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>> = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(name.clone()).append(ADD_CHOICE.clone()).append(jcall!(gameFeature).toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), GET_CHOICE_GROUP.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            jcall!(choiceGroup).append(jcall!(gameFeature).toString(), crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone());
            if jcall!(features).isFeature(gameFeature.clone()) {
                jcall!(choiceGroup).setSelectedIndex(index.clone(), true);
            }
        }
        return choiceGroup.clone();
    }
    

    pub fn addChoiceGroup(&mut self, mut form: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>>, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut option: i32) {
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut ADDING_CHOICE_GROUP: String = String::from("Adding Choice Group: ");
        let mut ADD_CHOICE_GROUP: String = String::from("addChoiceGroup");
        let mut size: i32 = jcall!(hashtable).size();
        let mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(HashtableUtil::getInstance()).getKeysAsArray(hashtable.clone());
        let mut name: String = String::new();
        for index in (0..size) {
            name = (objectArray[(index) as usize]).clone() /* as String */;
            jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringMaker).append(ADDING_CHOICE_GROUP.clone()).append(name.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), ADD_CHOICE_GROUP.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            { let java_receiver = self.meUtil.clone(); let java_arguments = (form.clone(), self.getChoiceGroup(hashtable.clone(), name.clone(), option.clone()),); jcall!(java_receiver).appendItem(java_arguments.0, java_arguments.1); };
        }
    }
    

    // throws Exception
    pub fn setDefault(&mut self, mut form: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::displayable::screen::CommandForm::CommandFormDyn>>>) {
        let mut size: i32 = jcall!(form).as_CommandForm_mut().size();
        for index in (0..size) {
            let mut item: Option<Rc<RefCell<Item>>> = jcall!(form).as_CommandForm_mut().get(index.clone());
            if (item).is_some() /* instanceof GameConfigurationGauge */ {
                jcall!(GameConfigurationUtil::getInstance()).setDefault((item).clone() /* as GameConfigurationGauge */);
            } else if (item).is_some() /* instanceof ChoiceGroup */ {
                jcall!(GameFeatureUtil::GameFeatureUtil::getInstance()).setDefault((item).clone() /* as ChoiceGroup */);
            }
        }
    }
}

