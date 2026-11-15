// 
// To change this license header, choose License Headers in Project Properties. 
// To change this template file, choose Tools | Templates 
// and open the template in the editor.  
// Generated Code Do Not Modify
// package org.allbinary.game.multiplayer.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RemoteInfo {
}

pub static RemoteInfo_NEW_PLAYER_ID: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 0);
pub static RemoteInfo_REMOTE_INFO: crate::JavaStatic<Option<Rc<RefCell<RemoteInfo>>>> = crate::JavaStatic::new(|| RemoteInfo::new({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), -1, -1, RemoteInfo_NEW_PLAYER_ID.with(Clone::clone).clone()));

impl RemoteInfo {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut username: String, mut actorSessionId: i32, mut playerInputId: i32, mut id: i32) -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new(mut username: String, mut actorSessionId: i32, mut playerInputId: i32, mut id: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(username, actorSessionId, playerInputId, id))))
    }
}

