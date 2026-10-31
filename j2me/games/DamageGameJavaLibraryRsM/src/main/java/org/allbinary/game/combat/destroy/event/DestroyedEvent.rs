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
use crate::org::allbinary::game::displayable::canvas::NullGameCanvas::NullGameCanvas;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DestroyedEvent {
    pub base: AllBinaryEventObject,
    pub allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>,
    pub allBinaryLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>,
}


impl DestroyedEvent {
    

    pub fn new_struct(mut combatGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Self {
        let mut this = Self {
            base: AllBinaryEventObject::new_struct(DestroyedEventHandler::DestroyedEventHandler::getInstance()),
            allBinaryGameCanvas: None,
            allBinaryLayerInterface: None,
        };
        this.allBinaryGameCanvas = NullGameCanvas::getInstance();
        this.allBinaryLayerInterface = crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this.setAllBinaryGameCanvas(combatGameCanvas.clone());
        this
    }

    pub fn new(mut combatGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(combatGameCanvas))))
    }
    

    pub fn getAllBinaryGameCanvas(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>> {
        return self.allBinaryGameCanvas.clone();
    }
    

    pub fn getLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        return self.allBinaryLayerInterface.clone();
    }
    

    pub fn setAllBinaryGameCanvas(&mut self, mut allBinaryGameCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvas::AllBinaryGameCanvasDyn>>>) {
        self.allBinaryGameCanvas = allBinaryGameCanvas.clone();
    }
    

    pub fn setLayerInterfaceForCircularStaticPool(&mut self, mut allBinaryLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        self.allBinaryLayerInterface = allBinaryLayerInterface.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("DestroyedEvent: \n"));
        jcall!(stringBuffer).append(String::from("AllBinaryLayerInterface: "));
        jcall!(stringBuffer).append(jcall!(self.allBinaryLayerInterface).toString());
        jcall!(stringBuffer).append(String::from("\nAllBinaryGameCanvas: "));
        jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(self.getAllBinaryGameCanvas())));
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for DestroyedEvent {
    type Target = AllBinaryEventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DestroyedEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for DestroyedEvent {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base }
}


