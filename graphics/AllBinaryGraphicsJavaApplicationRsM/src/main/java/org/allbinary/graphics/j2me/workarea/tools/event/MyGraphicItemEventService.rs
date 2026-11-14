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
// package org.allbinary.graphics.j2me.workarea.tools.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::lang::Exception::*;
use super::EventListenerList::EventListenerList;
use super::EventListener::EventListener;
use super::MyGraphicItemEventListener::MyGraphicItemEventListener;
use super::MyGraphicItemEvent::MyGraphicItemEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MyGraphicItemEventService {
}

pub static mut MyGraphicItemEventService_listenerList: std::cell::LazyCell<Option<Rc<RefCell<EventListenerList>>>> = std::cell::LazyCell::new(|| EventListenerList::new());
pub static mut MyGraphicItemEventService_DELETE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("delete"));
pub static mut MyGraphicItemEventService_DESELECT: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("deselect"));
pub static mut MyGraphicItemEventService_SELECT: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("select"));
pub static mut MyGraphicItemEventService_DUPLICATE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("duplicate"));
pub static mut MyGraphicItemEventService_ROTATE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("rotate"));

impl MyGraphicItemEventService {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // synchronized
    pub fn addListener(mut listener: Option<Rc<RefCell<dyn MyGraphicItemEventListener>>>) {
        jcall!(unsafe { MyGraphicItemEventService_listenerList.clone() }).add((std::any::type_name_of_val(&listener)).clone() /* as Class */, (listener).clone() /* as EventListener */);
    }
    

    // synchronized
    pub fn removeListener(mut listener: Option<Rc<RefCell<dyn MyGraphicItemEventListener>>>) {
        jcall!(unsafe { MyGraphicItemEventService_listenerList.clone() }).remove((std::any::type_name_of_val(&listener)).clone() /* as Class */, (listener).clone() /* as EventListener */);
    }
    

    // throws Exception
    // synchronized
    pub fn fire(mut evt: Option<Rc<RefCell<MyGraphicItemEvent>>>) {
        let mut listeners: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(unsafe { MyGraphicItemEventService_listenerList.clone() }).getListenerList();
        for i in (0..(listeners.len() as i32)).step_by((2) as usize) {
            if match (&listeners[(i) as usize], &std::any::type_name_of_val(&listeners[(i + 1) as usize])) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if (jcall!(evt).getCommand().cmp(&unsafe { MyGraphicItemEventService_SELECT.clone() }.clone()) as i32) == 0 {
                    jcall!(({ let java_object = (listeners[(i + 1) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn MyGraphicItemEventListener>>>>().expect("ClassCastException").clone()) })).highlight(evt.clone());
                } else if (jcall!(evt).getCommand().cmp(&unsafe { MyGraphicItemEventService_DESELECT.clone() }.clone()) as i32) == 0 {
                    jcall!(({ let java_object = (listeners[(i + 1) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn MyGraphicItemEventListener>>>>().expect("ClassCastException").clone()) })).deselect(evt.clone());
                } else if (jcall!(evt).getCommand().cmp(&unsafe { MyGraphicItemEventService_DELETE.clone() }.clone()) as i32) == 0 {
                    jcall!(({ let java_object = (listeners[(i + 1) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn MyGraphicItemEventListener>>>>().expect("ClassCastException").clone()) })).delete(evt.clone());
                } else if (jcall!(evt).getCommand().cmp(&unsafe { MyGraphicItemEventService_DUPLICATE.clone() }.clone()) as i32) == 0 {
                    jcall!(({ let java_object = (listeners[(i + 1) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn MyGraphicItemEventListener>>>>().expect("ClassCastException").clone()) })).duplicate(evt.clone());
                } else if (jcall!(evt).getCommand().cmp(&unsafe { MyGraphicItemEventService_ROTATE.clone() }.clone()) as i32) == 0 {
                    jcall!(({ let java_object = (listeners[(i + 1) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn MyGraphicItemEventListener>>>>().expect("ClassCastException").clone()) })).rotate(evt.clone());
                }
            }
        }
    }
}

