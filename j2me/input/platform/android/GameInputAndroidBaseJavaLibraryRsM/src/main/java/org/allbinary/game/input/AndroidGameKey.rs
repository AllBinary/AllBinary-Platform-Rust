// Generated Code Do Not Modify
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::Input::Input;
use super::InputFactory::InputFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidGameKey {
    pub base: Input,
}


impl AndroidGameKey {
    

    pub fn new_struct(mut keyCode: i32, mut name: String) -> Self {
        let mut this = Self {
            base: Input::new_struct2(keyCode.clone(), name.clone()),
        };
        // super(...) is in the struct literal above
        let mut inputFactory: Option<Rc<RefCell<InputFactory>>> = InputFactory::getInstance();
        jcall!(inputFactory).inputIntegerArray[(this.getId()) as usize] = this;
        this
    }

    pub fn new(mut keyCode: i32, mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(keyCode, name))))
    }
    

    pub fn toString(&mut self) -> String {
        return format!("{}{}", String::from("AndroidGameKey: "), self.base.toString());
    }
    

    pub fn getString(mut keyCode: i32) -> String {
        let mut input: Option<Rc<RefCell<Input>>> = jcall!(InputFactory::getInstance()).getInstanceById(keyCode.clone());
        if input.is_some() {
            return jcall!(input).getName();
        } else {
            return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        }
    }
}
impl std::ops::Deref for AndroidGameKey {
    type Target = Input;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AndroidGameKey {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



