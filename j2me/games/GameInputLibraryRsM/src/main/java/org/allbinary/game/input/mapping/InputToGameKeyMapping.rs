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
use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::game::input::GameKey::GameKey;
use crate::org::allbinary::game::input::GameKeyFactory::GameKeyFactory;
use crate::org::allbinary::game::input::Input::Input;
use crate::org::allbinary::game::input::InputFactory::InputFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::java::lang::Exception::*;
use super::InputMapping::InputMapping;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputToGameKeyMapping {
    pub base: InputMapping,
    pub gameKeyFactory: Option<Rc<RefCell<GameKeyFactory>>>,
    pub mappedGameKeys: Vec<Option<Rc<RefCell<GameKey>>>>,
    pub platformToGameKeyMapping: Vec<Option<Rc<RefCell<GameKey>>>>,
    pub negativePlatformToGameKeyMapping: Vec<Option<Rc<RefCell<GameKey>>>>,
}

pub static mut InputToGameKeyMapping_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());

impl InputToGameKeyMapping {
    

    pub fn getNullInstance() -> Option<Rc<RefCell<InputToGameKeyMapping>>> {
        if match (&unsafe { InputToGameKeyMapping_instance.clone() }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *InputToGameKeyMapping_instance = crate::java_into_object(InputToGameKeyMapping::new()) };
        }
        return { let java_object = (unsafe { InputToGameKeyMapping_instance.clone() }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<InputToGameKeyMapping>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: InputMapping::new_struct(),
            gameKeyFactory: None,
            mappedGameKeys: Vec::new(),
            platformToGameKeyMapping: Vec::new(),
            negativePlatformToGameKeyMapping: Vec::new(),
        };
        this.gameKeyFactory = GameKeyFactory::getInstance();
        this.mappedGameKeys = vec![{ let java_field = jcall!(this.gameKeyFactory).UP.clone(); java_field }.clone(), { let java_field = jcall!(this.gameKeyFactory).DOWN.clone(); java_field }.clone(), { let java_field = jcall!(this.gameKeyFactory).LEFT.clone(); java_field }.clone(), { let java_field = jcall!(this.gameKeyFactory).RIGHT.clone(); java_field }.clone()];
        this.platformToGameKeyMapping = vec![None; ({ let java_field = jcall!(InputFactory::getInstance()).MAX.clone(); java_field }) as usize];
        this.negativePlatformToGameKeyMapping = vec![None; (0) as usize];
        jcall!(this.logUtil).putF({ let java_field = jcall!(this.commonStrings).START.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(this.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
        this.clear();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn clear(&mut self) {
        for index in (0..=(self.platformToGameKeyMapping.len() as i32) - 1).rev() {
            { let java_index = (index) as usize; self.platformToGameKeyMapping[java_index] = { let java_field = jcall!(self.gameKeyFactory).NONE.clone(); java_field }.clone(); };
        }
    }
    

    pub fn removeAll(&mut self) {
        self.base.removeAll();
        self.clear();
    }
    

    pub fn addMapping(&mut self, mut inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>) {
        for index in (0..={ let java_bound = jcall!(inputToGameKeyMapping).getMaxMappable() - 1; java_bound }).rev() {
            let mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = jcall!(inputToGameKeyMapping).getInstance(index.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>);
            if !match (&input, &{ let java_field = jcall!(self.gameKeyFactory).NONE.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                { let java_arguments = (input.clone(), jcall!(inputToGameKeyMapping).getMappedInput(input.clone()),); self.addAll(java_arguments.0, java_arguments.1) };
            }
        }
    }
    

    pub fn addAll(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        for index in (0..={ let java_bound = jcall!(list).size() - 1; java_bound }).rev() {
            let mut mappedToInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>>().expect("ClassCastException").clone()) };
            self.add(input.clone(), mappedToInput.clone());
        }
    }
    

    pub fn add(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>, mut mappedToInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) {
        { let java_arguments = (input.clone(), mappedToInput.clone(),); self.base.add(java_arguments.0, java_arguments.1) };
        self.set(input.clone(), mappedToInput.clone());
    }
    

    pub fn remove(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>, mut mappedToInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append({ let java_field = jcall!(CommonLabels::getInstance()).START_LABEL.clone(); java_field }.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(input.clone()))).append(String::from(" == ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(mappedToInput.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("InputToGameKeyMapping::remove"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        { let java_arguments = (input.clone(), mappedToInput.clone(),); self.base.remove(java_arguments.0, java_arguments.1) };
        { let java_arguments = (input.clone(), { let java_field = jcall!(self.gameKeyFactory).NONE.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>),); self.set(java_arguments.0, java_arguments.1) };
    }
    

    pub fn set(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>, mut mappedToInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) {
        let mut id: i32 = jcall!(mappedToInput).getId();
        if id >= 0 && id < (self.platformToGameKeyMapping.len() as i32) {
            { let java_index = (id) as usize; self.platformToGameKeyMapping[java_index] = (input).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<GameKey>() }, "ClassCastException: not a GameKey"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<GameKey>) } }); };
        }
    }
    

    pub fn getMaxMappable(&mut self) -> i32 {
        return (self.platformToGameKeyMapping.len() as i32).clone();
    }
    

    pub fn getReverseInstance(&mut self, mut id: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut inputFactory: Option<Rc<RefCell<InputFactory>>> = InputFactory::getInstance();
        for index in (0..(self.platformToGameKeyMapping.len() as i32)) {
            if jcall!(self.platformToGameKeyMapping[(index) as usize]).getId() == id {
                let mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = jcall!(inputFactory).getInstanceById(index.clone());
                if input.is_some() {
                    jcall!(list).add(crate::java_into_object(input.clone()));
                }
            }
        }
        return list.clone();
    }
    

    // throws Exception
    pub fn initGameKey(&mut self, mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>, mut gameKey: Option<Rc<RefCell<GameKey>>>) {
        let mut key: i32 = jcall!(canvas).getKeyCode(jcall!(gameKey).getId());
        if key < 0 && -key < (self.negativePlatformToGameKeyMapping.len() as i32) {
            { let java_index = (-key) as usize; self.negativePlatformToGameKeyMapping[java_index] = gameKey.clone(); };
        } else if key > 0 {
            if match (&self.platformToGameKeyMapping[(key) as usize], &{ let java_field = jcall!(self.gameKeyFactory).NONE.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                { let java_index = (key) as usize; self.platformToGameKeyMapping[java_index] = gameKey.clone(); };
            }
        } else {
            PreLogUtil::putOE(jcall!(StringMaker::new()).append(jcall!(gameKey).toString()).append({ let java_field = jcall!(CommonSeps::getInstance()).COLON_SEP.clone(); java_field }.clone()).appendint(key.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone(), crate::java_into_object(format!("{}: {:?}", "Exception", ())));
        }
    }
    

    // throws Exception
    pub fn getSmallestCanvasGameKeyCode(&mut self, mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>) -> i32 {
        let mut nextKey: i32 = 0;
        let mut smallestKey: i32 = 0;
        {
            let mut index: i32 = (self.mappedGameKeys.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            nextKey = { let java_value = (jcall!(canvas).getKeyCode(jcall!(self.mappedGameKeys[(index) as usize]).getId())).clone(); java_value };
            if nextKey < smallestKey {
                smallestKey = nextKey.clone();
            }
            }
        }
        return smallestKey.clone();
    }
    

    pub fn init(&mut self, mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut negativePlatformToGameKeyMapping: Vec<Option<Rc<RefCell<GameKey>>>> = vec![None; (0) as usize];
            let mut smallestKey: i32 = self.getSmallestCanvasGameKeyCode(canvas.clone());
            if smallestKey < 0 {
                smallestKey -= 1;
                if smallestKey < -256 {
                    smallestKey = -256;
                }
                negativePlatformToGameKeyMapping = vec![None; (-smallestKey) as usize];
                {
                    let mut index: i32 = (negativePlatformToGameKeyMapping.len() as i32).clone();
                    while { index -= 1; index } >= 0 {
                    negativePlatformToGameKeyMapping[(index) as usize] = { let java_field = jcall!(self.gameKeyFactory).NONE.clone(); java_field }.clone();
                    }
                }
                self.negativePlatformToGameKeyMapping = negativePlatformToGameKeyMapping.clone();
            }
            {
                let mut index: i32 = (self.mappedGameKeys.len() as i32).clone();
                while { index -= 1; index } >= 0 {
                { let java_arguments = (canvas.clone(), self.mappedGameKeys[(index) as usize].clone(),); self.initGameKey(java_arguments.0, java_arguments.1) };
                }
            }
        })) {
            Ok(_) => {}
            Err(t) => {
            let t: String = crate::java::lang::Exception::java_panic_message(&t);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).INIT.clone(); java_field }.clone(), crate::java_into_object(t.clone()));
            }
        }
    }
    

    pub fn getInstanceForCanvas(&mut self, mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>>, mut id: i32) -> Option<Rc<RefCell<GameKey>>> {
        if id >= 0 && id < (self.platformToGameKeyMapping.len() as i32) {
            return self.platformToGameKeyMapping[(id) as usize].clone();
        } else if id < 0 && -id < (self.negativePlatformToGameKeyMapping.len() as i32) {
            return self.negativePlatformToGameKeyMapping[(-id) as usize].clone();
        } else {
            return { let java_field = jcall!(self.gameKeyFactory).NONE.clone(); java_field }.clone();
        }
    }
    

    pub fn getInstance(&mut self, mut id: i32) -> Option<Rc<RefCell<GameKey>>> {
        if id >= 0 && id < (self.platformToGameKeyMapping.len() as i32) {
            return self.platformToGameKeyMapping[(id) as usize].clone();
        } else if id < 0 && -id < (self.negativePlatformToGameKeyMapping.len() as i32) {
            return self.negativePlatformToGameKeyMapping[(-id) as usize].clone();
        } else {
            return { let java_field = jcall!(self.gameKeyFactory).NONE.clone(); java_field }.clone();
        }
    }
}
impl std::ops::Deref for InputToGameKeyMapping {
    type Target = InputMapping;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InputToGameKeyMapping {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::mapping::InputMapping::InputMappingDyn for InputToGameKeyMapping {
    fn as_InputMapping(&self) -> &crate::org::allbinary::game::input::mapping::InputMapping::InputMapping { &self.base }
    fn as_InputMapping_mut(&mut self) -> &mut crate::org::allbinary::game::input::mapping::InputMapping::InputMapping { &mut self.base }
    fn removeAll(&mut self) { Self::removeAll(self) }
    fn remove(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>, mut mappedToInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) { Self::remove(self, input, mappedToInput) }
    fn addMapping(&mut self, mut inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>) { Self::addMapping(self, inputToGameKeyMapping) }
    fn add(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>, mut mappedToInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) { Self::add(self, input, mappedToInput) }
}


