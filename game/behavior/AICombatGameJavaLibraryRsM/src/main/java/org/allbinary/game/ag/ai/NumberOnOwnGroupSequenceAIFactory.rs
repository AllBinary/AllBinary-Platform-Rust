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
// package org.allbinary.game.ag.ai
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface;
use crate::org::allbinary::game::input::GameInput::GameInput;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::NumberOnOwnGroupSequenceAI::NumberOnOwnGroupSequenceAI;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NumberOnOwnGroupSequenceAIFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl NumberOnOwnGroupSequenceAIFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut artificialIntelligenceInterface: Vec<Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>>>, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut gameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::GameInput::GameInputDyn>>>) -> Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>> {
        let mut integerArray: Vec<Option<i32>> = jcall!(NumberInSameGroupSequence::NumberInSameGroupSequence::getInstance()).NUMBER_ON_SAME_TEAM_SEQUENCE.clone();
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        for index in (0..(integerArray.len() as i32)) {
            let mut integerCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(hashtable).get(crate::java_into_object((integerArray[(index) as usize]).clone() /* as Any */));
            if integerCanBeNull.is_none() {
                break;
            }
            jcall!(list).add(crate::java_into_object(integerCanBeNull.clone()));
        }
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Total AI Properties: "));
        jcall!(stringBuffer).appendint(jcall!(list).size());
        jcall!(stringBuffer).append(String::from(" == Total AI: "));
        jcall!(stringBuffer).appendint((artificialIntelligenceInterface.len() as i32).clone());
        jcall!(stringBuffer).append(String::from(" + 1"));
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        if (artificialIntelligenceInterface.len() as i32) - 1 > jcall!(list).size() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not enough AI properties."))));
        }
        if (artificialIntelligenceInterface.len() as i32) - 1 < jcall!(list).size() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Too Many AI properties."))));
        }
        let mut numberOnSameTeam: Vec<Option<i32>> = vec![None; (jcall!(list).size()) as usize];
        for index in (0..(numberOnSameTeam.len() as i32)) {
            numberOnSameTeam[(index) as usize] = { let java_object = jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]; java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
        }
        return NumberOnOwnGroupSequenceAI::new(numberOnSameTeam.clone(), artificialIntelligenceInterface.clone(), ownerLayerInterface.clone(), gameInput.clone()).map(|reference| reference as Rc<RefCell<dyn ArtificialIntelligenceInterface>>);
    }
}

