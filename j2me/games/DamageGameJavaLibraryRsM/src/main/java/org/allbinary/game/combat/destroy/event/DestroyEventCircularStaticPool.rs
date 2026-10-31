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
// package org.allbinary.game.combat.destroy.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvas;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::util::event::AllBinaryEventCircularPool::AllBinaryEventCircularPool;
use super::DestroyEventFactory::DestroyEventFactory;
use super::DestroyedEvent::DestroyedEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DestroyEventCircularStaticPool {
    pub EVENT_POOL: Option<Rc<RefCell<AllBinaryEventCircularPool>>>,
}

thread_local! { pub static DestroyEventCircularStaticPool_instance: Option<Rc<RefCell<DestroyEventCircularStaticPool>>> = DestroyEventCircularStaticPool::new(); }

impl DestroyEventCircularStaticPool {
    

    pub fn getInstance() -> Option<Rc<RefCell<DestroyEventCircularStaticPool>>> {
        return DestroyEventCircularStaticPool_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            EVENT_POOL: None,
        };
        this.EVENT_POOL = AllBinaryEventCircularPool::new(20);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self, mut combatGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        jcall!(self.EVENT_POOL).initAllBinaryEventCircularPool(DestroyEventFactory::new(combatGameCanvas.clone()));
    }
    

    // throws Exception
    pub fn getInstanceForLayer(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<DestroyedEvent>>> {
        let mut destroyedEvent: Option<Rc<RefCell<DestroyedEvent>>> = { let java_object = (jcall!(self.EVENT_POOL).getNextInstance()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<DestroyedEvent>>>>().expect("ClassCastException").clone()) };
        jcall!(destroyedEvent).setLayerInterfaceForCircularStaticPool(layerInterface.clone());
        return destroyedEvent.clone();
    }
}

