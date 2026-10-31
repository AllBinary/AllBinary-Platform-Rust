// Generated Code Do Not Modify
// package org.allbinary.game.layer.drop
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::layer::AllBinaryLayerFactoryInterface::AllBinaryLayerFactoryInterface;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlacedLayerFactory {
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub basicArrayListUtil: Option<Rc<RefCell<BasicArrayListUtil>>>,
}

thread_local! { pub static PlacedLayerFactory_instance: Option<Rc<RefCell<PlacedLayerFactory>>> = PlacedLayerFactory::new(); }

impl PlacedLayerFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            list: None,
            basicArrayListUtil: None,
        };
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.basicArrayListUtil = BasicArrayListUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn clear(&mut self) {
        jcall!(self.list).clear();
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PlacedLayerFactory>>> {
        return PlacedLayerFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn getSize(&mut self) -> i32 {
        return jcall!(self.list).size();
    }
    

    pub fn getRandomInstance(&mut self) -> Option<Rc<RefCell<dyn AllBinaryLayerFactoryInterface>>> {
        return { let java_object = jcall!(self.basicArrayListUtil).getRandom(self.list.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn AllBinaryLayerFactoryInterface>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn add(&mut self, mut layerInterfaceFactoryInterface: Option<Rc<RefCell<dyn AllBinaryLayerFactoryInterface>>>) {
        jcall!(self.list).add(crate::java_into_object(layerInterfaceFactoryInterface.clone()));
    }
}

