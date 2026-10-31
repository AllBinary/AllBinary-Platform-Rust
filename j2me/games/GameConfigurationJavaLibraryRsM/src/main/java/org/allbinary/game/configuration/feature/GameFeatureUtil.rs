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
use crate::Enumeration;
use super::*;
// use crate::java::util::Enumeration::Enumeration;
// use crate::javax::microedition::lcdui::ChoiceGroup::ChoiceGroup;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::EnumerationUtil::EnumerationUtil;
use crate::org::allbinary::util::HashtableUtil::HashtableUtil;
use super::Features::Features;
use super::Feature::Feature;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameFeatureUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
    pub enumerationUtil: Option<Rc<RefCell<EnumerationUtil>>>,
    pub ON_GAME_FEATURE_CHANGE: String,
    pub GAME_FEATURE_CHANGED: String,
}

thread_local! { pub static GameFeatureUtil_instance: Option<Rc<RefCell<GameFeatureUtil>>> = GameFeatureUtil::new(); }

impl GameFeatureUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            tsUtil: None,
            enumerationUtil: None,
            ON_GAME_FEATURE_CHANGE: String::new(),
            GAME_FEATURE_CHANGED: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.tsUtil = TsUtil::getInstance();
        this.enumerationUtil = EnumerationUtil::getInstance();
        this.ON_GAME_FEATURE_CHANGE = String::from("onGameFeatureChange");
        this.GAME_FEATURE_CHANGED = String::from("Game Feature Changed: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameFeatureUtil>>> {
        return GameFeatureUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn setDefault(&mut self, mut choiceGroup: Option<Rc<RefCell<ChoiceGroup>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut METHOD_NAME: String = String::from("setDefault");
        let mut SELECTED_ARRAY_RETURN: String = String::from("selectedArray_return[");
        let mut SELECTED_SEP: String = String::from("] = ");
        let mut selectedArray_return: Vec<bool> = vec![false; (jcall!(choiceGroup).size()) as usize];
        let mut total: i32 = jcall!(choiceGroup).getSelectedFlags(selectedArray_return.clone());
        jcall!(stringBuffer).append(String::from("Multiple Total Choices: "));
        jcall!(stringBuffer).appendint(total.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        for index in (0..(selectedArray_return.len() as i32)) {
            let mut isSelected: bool = selectedArray_return[(index) as usize].clone();
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(SELECTED_ARRAY_RETURN.clone());
            jcall!(stringBuffer).appendint(index.clone());
            jcall!(stringBuffer).append(SELECTED_SEP.clone());
            jcall!(stringBuffer).appendboolean(isSelected.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            let mut selectedChoiceLabel: String = jcall!(choiceGroup).getString(index.clone());
            let mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>> = Feature::getInstance(selectedChoiceLabel.clone());
            if jcall!(features).isDefault(gameFeature.clone()) {
                jcall!(features).add(gameFeature.clone());
                jcall!(choiceGroup).setSelectedIndex(index.clone(), true);
            } else {
                jcall!(features).remove(gameFeature.clone());
                jcall!(choiceGroup).setSelectedIndex(index.clone(), false);
            }
        }
    }
    

    // throws Exception
    pub fn updateMultiple(&mut self, mut choiceGroup: Option<Rc<RefCell<ChoiceGroup>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut METHOD_NAME: String = String::from("updateMultiple");
        let mut SELECTED_ARRAY_RETURN: String = String::from("selectedArray_return[");
        let mut SELECTED_SEP: String = String::from("] = ");
        let mut selectedArray_return: Vec<bool> = vec![false; (jcall!(choiceGroup).size()) as usize];
        let mut total: i32 = jcall!(choiceGroup).getSelectedFlags(selectedArray_return.clone());
        jcall!(stringBuffer).append(String::from("Multiple Total Choices: "));
        jcall!(stringBuffer).appendint(total.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        for index in (0..(selectedArray_return.len() as i32)) {
            let mut isSelected: bool = selectedArray_return[(index) as usize].clone();
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(SELECTED_ARRAY_RETURN.clone());
            jcall!(stringBuffer).appendint(index.clone());
            jcall!(stringBuffer).append(SELECTED_SEP.clone());
            jcall!(stringBuffer).appendboolean(isSelected.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            let mut selectedChoiceLabel: String = jcall!(choiceGroup).getString(index.clone());
            let mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>> = Feature::getInstance(selectedChoiceLabel.clone());
            if !isSelected && jcall!(features).isFeature(gameFeature.clone()) {
                jcall!(features).remove(gameFeature.clone());
            } else if isSelected && !jcall!(features).isFeature(gameFeature.clone()) {
                jcall!(features).add(gameFeature.clone());
            }
        }
    }
    

    // throws Exception
    pub fn updateExclusiveForChoiceGroup(&mut self, mut choiceGroup: Option<Rc<RefCell<ChoiceGroup>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut METHOD_NAME: String = String::from("updateExclusive");
        let mut SELECTED_ARRAY_RETURN: String = String::from("selectedArray_return[");
        let mut SELECTED_SEP: String = String::from("] = ");
        let mut selectedArray_return: Vec<bool> = vec![false; (jcall!(choiceGroup).size()) as usize];
        let mut total: i32 = jcall!(choiceGroup).getSelectedFlags(selectedArray_return.clone());
        jcall!(stringBuffer).append(String::from("Exclusive Total Choices: 1=="));
        jcall!(stringBuffer).appendint(total.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        for index in (0..(selectedArray_return.len() as i32)) {
            let mut isSelected: bool = selectedArray_return[(index) as usize].clone();
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(SELECTED_ARRAY_RETURN.clone());
            jcall!(stringBuffer).appendint(index.clone());
            jcall!(stringBuffer).append(SELECTED_SEP.clone());
            jcall!(stringBuffer).appendboolean(isSelected.clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), METHOD_NAME.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if isSelected {
                let mut selectedChoiceLabel: String = jcall!(choiceGroup).getString(index.clone());
                self.updateExclusive(selectedChoiceLabel.clone());
            }
        }
    }
    

    pub fn isExclusive(&mut self, mut itemLabel: String) -> bool {
        let mut enumeration: Option<Rc<RefCell<dyn Enumeration>>> = jcall!(jcall!(GameFeatureChoiceGroups::GameFeatureChoiceGroups::getExclusiveInstance()).get()).keys();
        let mut name: String = String::new();
        while jcall!(self.enumerationUtil).hasMoreElements(enumeration.clone()) {
            name = (jcall!(self.enumerationUtil).nextElement(enumeration.clone())).clone() /* as String */;
            if { let left = &(itemLabel); let right = &(name); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
                return true;
            }
        }
        return false;
    }
    

    // throws Exception
    pub fn updateExclusive(&mut self, mut selectedChoiceLabel: String) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).ITEM_LABEL.clone()).append(selectedChoiceLabel.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateExclusive"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>> = Feature::getInstance(selectedChoiceLabel.clone());
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(GameFeatureChoiceGroups::GameFeatureChoiceGroups::getExclusiveInstance()).get();
        let mut objectArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(HashtableUtil::getInstance()).getKeysAsArray(hashtable.clone());
        let mut basicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = None;
        let mut size: i32 = (objectArray.len() as i32).clone();
        for index in (0..size) {
            anyType = (objectArray[(index) as usize]).clone() /* as Any */;
            basicArrayList = { let java_object = (jcall!(hashtable).get(anyType.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
            self.updateExclusiveList(gameFeature.clone(), basicArrayList.clone());
        }
    }
    

    // throws Exception
    pub fn updateExclusiveList(&mut self, mut gameFeature: Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        if jcall!(list).contains(crate::java_into_object(gameFeature.clone())) {
            let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
            let mut addIndex: i32 = jcall!(list).indexOf(crate::java_into_object(gameFeature.clone()));
            let mut size: i32 = jcall!(list).size();
            for index in (0..size) {
                if addIndex != index {
                    jcall!(features).remove({ let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>>().expect("ClassCastException").clone()) });
                } else {
                    jcall!(features).add({ let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>>>().expect("ClassCastException").clone()) });
                }
            }
        }
    }
}

