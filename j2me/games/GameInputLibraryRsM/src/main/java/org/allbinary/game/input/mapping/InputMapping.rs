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
// package org.allbinary.game.input.mapping
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::Input::Input;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::InputToGameKeyMapping::InputToGameKeyMapping;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputMapping {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub mappedList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl InputMapping {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            hashtable: None,
            mappedList: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.hashtable = jcall!(StdUtil::getInstance()).createHashtable();
        this.mappedList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getTotalMapped(&mut self) -> i32 {
        return jcall!(self.hashtable).size();
    }
    

    pub fn isDefaultNew(&mut self) -> bool {
        return false;
    }
    

    pub fn getDefault(&mut self) -> Option<Rc<RefCell<InputToGameKeyMapping>>> {
        return InputToGameKeyMapping::getNullInstance();
    }
    

    pub fn removeAll(&mut self) {
        jcall!(self.hashtable).clear();
        jcall!(self.mappedList).clear();
    }
    

    pub fn remove(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>, mut mappedToInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).START_LABEL.clone());
        jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(input.clone())));
        jcall!(stringBuffer).append(String::from(" == "));
        jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(mappedToInput.clone())));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("InputMapping::remove"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getMappedInput(input.clone());
        if jcall!(list).contains(crate::java_into_object(mappedToInput.clone())) {
            jcall!(list).remove(crate::java_into_object(mappedToInput.clone()));
            jcall!(self.mappedList).remove(crate::java_into_object(mappedToInput.clone()));
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(String::from("Removed: "));
            jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(input.clone())));
            jcall!(stringBuffer).append(String::from(" == "));
            jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(mappedToInput.clone())));
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("InputMapping::remove"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        } else {
            jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
            jcall!(stringBuffer).append(String::from("Not Deleted: "));
            jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(input.clone())));
            jcall!(stringBuffer).append(String::from(" == "));
            jcall!(stringBuffer).append(jcall!(stringUtil).toString(crate::java_into_object(mappedToInput.clone())));
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("InputMapping::remove"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
    

    // throws Exception
    pub fn addMapping(&mut self, mut inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    pub fn add(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>, mut mappedToInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getMappedInput(input.clone());
        if !jcall!(list).contains(crate::java_into_object(mappedToInput.clone())) {
            jcall!(list).add(crate::java_into_object(mappedToInput.clone()));
            jcall!(self.mappedList).add(crate::java_into_object(mappedToInput.clone()));
        }
    }
    

    pub fn isMapped(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        return jcall!(self.mappedList).contains(crate::java_into_object(input.clone()));
    }
    

    pub fn getMappedInput(&mut self, mut id: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut mappingInputCanBeNullList: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.hashtable).get(crate::java_into_object(id.clone()));
        if mappingInputCanBeNullList.is_none() {
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(self.hashtable).put(crate::java_into_object(id.clone()), crate::java_into_object(list.clone()));
            return list.clone();
        }
        return (mappingInputCanBeNullList).clone() /* as BasicArrayList */;
    }
    

    pub fn getHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return self.hashtable.clone();
    }
}
pub trait InputMappingDyn {
    fn as_InputMapping(&self) -> &InputMapping;
    fn as_InputMapping_mut(&mut self) -> &mut InputMapping;
    fn getTotalMapped(&mut self) -> i32 { InputMapping::getTotalMapped(self.as_InputMapping_mut()) }
    fn isDefaultNew(&mut self) -> bool { InputMapping::isDefaultNew(self.as_InputMapping_mut()) }
    fn getDefault(&mut self) -> Option<Rc<RefCell<InputToGameKeyMapping>>> { InputMapping::getDefault(self.as_InputMapping_mut()) }
    fn removeAll(&mut self) { InputMapping::removeAll(self.as_InputMapping_mut()) }
    fn remove(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>, mut mappedToInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) { InputMapping::remove(self.as_InputMapping_mut(), input, mappedToInput) }
    fn addMapping(&mut self, mut inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>) { InputMapping::addMapping(self.as_InputMapping_mut(), inputToGameKeyMapping) }
    fn add(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>, mut mappedToInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) { InputMapping::add(self.as_InputMapping_mut(), input, mappedToInput) }
    fn isMapped(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool { InputMapping::isMapped(self.as_InputMapping_mut(), input) }
    fn getMappedInput(&mut self, mut id: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { InputMapping::getMappedInput(self.as_InputMapping_mut(), id) }
    fn getHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { InputMapping::getHashtable(self.as_InputMapping_mut()) }
}
impl InputMappingDyn for InputMapping {
    fn as_InputMapping(&self) -> &InputMapping { self }
    fn as_InputMapping_mut(&mut self) -> &mut InputMapping { self }
}


