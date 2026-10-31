// Generated Code Do Not Modify
// package org.allbinary.game.input.analog
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::AnalogLocationInput::AnalogLocationInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AnalogLocationInputFactory {
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static AnalogLocationInputFactory_instance: Option<Rc<RefCell<AnalogLocationInputFactory>>> = AnalogLocationInputFactory::new(); }

impl AnalogLocationInputFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<AnalogLocationInputFactory>>> {
        return AnalogLocationInputFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            list: None,
        };
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn addPlayer(&mut self, mut playerInputId: i32) {
        let mut analogLocationInput: Option<Rc<RefCell<AnalogLocationInput>>> = AnalogLocationInput_NULL_ANALOG_LOCATION_INPUT.with(Clone::clone).clone();
        if playerInputId < jcall!(self.list).size() {
            analogLocationInput = { let java_object = (jcall!(self.list).get(playerInputId.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<AnalogLocationInput>>>>().expect("ClassCastException").clone()) };
        }
        if match (&analogLocationInput, &AnalogLocationInput_NULL_ANALOG_LOCATION_INPUT.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            analogLocationInput = AnalogLocationInput::new(playerInputId.clone());
            jcall!(self.list).add(crate::java_into_object(analogLocationInput.clone()));
        }
    }
    

    pub fn getInstanceForPlayer(&mut self, mut playerInputId: i32) -> Option<Rc<RefCell<AnalogLocationInput>>> {
        return { let java_object = (jcall!(self.list).get(playerInputId.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<AnalogLocationInput>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.list.clone();
    }
}

