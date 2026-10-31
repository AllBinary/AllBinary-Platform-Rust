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
// package org.allbinary.graphics.j2me.workarea.canvas.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::EventListenerList::EventListenerList;
use super::event::event;
use super::swing::swing;
use super::javax::javax;
use super::EventListener::EventListener;
use super::util::util;
use super::java::java;
use super::MyCanvasEventListener::MyCanvasEventListener;
use super::MyCanvasEvent::MyCanvasEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MyCanvasEventService {
}

pub static mut MyCanvasEventService_listenerList: std::cell::LazyCell<Option<Rc<RefCell<EventListenerList>>>> = std::cell::LazyCell::new(|| EventListenerList::new());
pub static mut MyCanvasEventService_DELETE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("delete"));
pub static mut MyCanvasEventService_DUPLICATE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("duplicate"));
pub static mut MyCanvasEventService_SELECT: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("select"));
pub static mut MyCanvasEventService_ROTATE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("rotate"));
pub static mut MyCanvasEventService_EXPLODE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("explode"));
pub static mut MyCanvasEventService_AUTOEXPLODE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("autoExplode"));
pub static mut MyCanvasEventService_CENTER: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("center"));

impl MyCanvasEventService {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // synchronized
    pub fn addListener(mut listener: Option<Rc<RefCell<dyn MyCanvasEventListener>>>) {
        jcall!(unsafe { MyCanvasEventService_listenerList.clone() }).add((std::any::type_name_of_val(&listener)).clone() /* as Class */, (listener).clone() /* as EventListener */);
    }
    

    // synchronized
    pub fn removeListener(mut listener: Option<Rc<RefCell<dyn MyCanvasEventListener>>>) {
        jcall!(unsafe { MyCanvasEventService_listenerList.clone() }).remove((std::any::type_name_of_val(&listener)).clone() /* as Class */, (listener).clone() /* as EventListener */);
    }
    

    // throws Exception
    // synchronized
    pub fn fire(mut evt: Option<Rc<RefCell<MyCanvasEvent>>>) {
        let mut listeners: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(unsafe { MyCanvasEventService_listenerList.clone() }).getListenerList();
        for i in (0..(listeners.len() as i32)).step_by((2) as usize) {
            if match (&listeners[(i) as usize], &std::any::type_name_of_val(&listeners[(i + 1) as usize])) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if jcall!(jcall!(evt).getCommand()).compareTo(unsafe { MyCanvasEventService_DELETE.clone() }.clone()) == 0 {
                    jcall!(((listeners[(i + 1) as usize]).clone() /* as MyCanvasEventListener */)).delete(evt.clone());
                } else if jcall!(jcall!(evt).getCommand()).compareTo(unsafe { MyCanvasEventService_DUPLICATE.clone() }.clone()) == 0 {
                    jcall!(((listeners[(i + 1) as usize]).clone() /* as MyCanvasEventListener */)).duplicate(evt.clone());
                } else if jcall!(jcall!(evt).getCommand()).compareTo(unsafe { MyCanvasEventService_SELECT.clone() }.clone()) == 0 {
                    jcall!(((listeners[(i + 1) as usize]).clone() /* as MyCanvasEventListener */)).select(evt.clone());
                } else if jcall!(jcall!(evt).getCommand()).compareTo(unsafe { MyCanvasEventService_ROTATE.clone() }.clone()) == 0 {
                    jcall!(((listeners[(i + 1) as usize]).clone() /* as MyCanvasEventListener */)).rotate(evt.clone());
                } else if jcall!(jcall!(evt).getCommand()).compareTo(unsafe { MyCanvasEventService_EXPLODE.clone() }.clone()) == 0 {
                    jcall!(((listeners[(i + 1) as usize]).clone() /* as MyCanvasEventListener */)).explode(evt.clone());
                } else if jcall!(jcall!(evt).getCommand()).compareTo(unsafe { MyCanvasEventService_AUTOEXPLODE.clone() }.clone()) == 0 {
                    jcall!(((listeners[(i + 1) as usize]).clone() /* as MyCanvasEventListener */)).autoExplode(evt.clone());
                } else if jcall!(jcall!(evt).getCommand()).compareTo(unsafe { MyCanvasEventService_CENTER.clone() }.clone()) == 0 {
                    jcall!(((listeners[(i + 1) as usize]).clone() /* as MyCanvasEventListener */)).center(evt.clone());
                }
            }
        }
    }
}

