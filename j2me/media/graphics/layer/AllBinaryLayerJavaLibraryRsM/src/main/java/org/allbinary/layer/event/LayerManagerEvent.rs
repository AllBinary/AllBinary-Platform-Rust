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
// package org.allbinary.layer.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::LayerManager::LayerManager;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerManagerEvent {
    pub base: AllBinaryEventObject,
    pub layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>,
    pub crud: String,
}


impl LayerManagerEvent {
    

    pub fn new_struct(mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>>, mut crud: String) -> Self {
        let mut this = Self {
            base: AllBinaryEventObject::new_struct(layerManager.clone()),
            layerInterface: None,
            crud: String::new(),
        };
        this.layerInterface = crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this.crud = crud.clone();
        this
    }

    pub fn new(mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>>, mut crud: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(layerManager, crud))))
    }
    

    pub fn getLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        return self.layerInterface.clone();
    }
    

    pub fn setLayerInterface(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        self.layerInterface = layerInterface.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("LayerManagerEvent: \n"));
        jcall!(stringBuffer).append(String::from("LayerInterface: "));
        jcall!(stringBuffer).append(jcall!(self.getLayerInterface()).toString());
        jcall!(stringBuffer).append(String::from("\nCRUD: "));
        jcall!(stringBuffer).append(self.crud.clone());
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for LayerManagerEvent {
    type Target = AllBinaryEventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LayerManagerEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for LayerManagerEvent {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base }
}


