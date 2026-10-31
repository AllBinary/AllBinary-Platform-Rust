// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::form::AssignWaypointsUtil::AssignWaypointsUtil;
use crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer;
use crate::org::allbinary::game::layer::RTSLayer::RTSLayer;
use crate::org::allbinary::game::layer::building::BuildingLayer::BuildingLayer;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::game::layer::unit::UnitLayer::UnitLayer;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::logic::util::visitor::Visitor::Visitor;
use super::SelectedRTSLayersPlayerGameInput::SelectedRTSLayersPlayerGameInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SelectAdvancedRTSLayerVisitor {
    pub base: Visitor,
    pub selectedRTSLayersPlayerGameInput: Option<Rc<RefCell<SelectedRTSLayersPlayerGameInput>>>,
}


impl SelectAdvancedRTSLayerVisitor {
    

    pub fn new_struct(mut selectedRTSLayersPlayerGameInput: Option<Rc<RefCell<SelectedRTSLayersPlayerGameInput>>>) -> Self {
        let mut this = Self {
            base: Visitor::new_struct(),
            selectedRTSLayersPlayerGameInput: None,
        };
        this.selectedRTSLayersPlayerGameInput = selectedRTSLayersPlayerGameInput.clone();
        this
    }

    pub fn new(mut selectedRTSLayersPlayerGameInput: Option<Rc<RefCell<SelectedRTSLayersPlayerGameInput>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(selectedRTSLayersPlayerGameInput))))
    }
    

    pub fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut selectedLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> = (anyType).clone() /* as CollidableDestroyableDamageableLayer */;
        if !match (&selectedLayer, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && jcall!(selectedLayer).getType() == BuildingLayer::getStaticType() {
            let mut rtsGameLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = (selectedLayer).clone() /* as AdvancedRTSGameLayer */;
            let mut assignWaypointsUtil: Option<Rc<RefCell<AssignWaypointsUtil>>> = AssignWaypointsUtil::getInstance();
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.selectedRTSLayersPlayerGameInput).getSelectedBasicArrayList();
            let mut currentRTSLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = None;
            let mut unitLayer: Option<Rc<RefCell<UnitLayer>>> = None;
            for index in (0..=jcall!(list).size() - 1).rev() {
                currentRTSLayer = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>>().expect("ClassCastException").clone()) };
                if jcall!(currentRTSLayer).getType() == UnitLayer::getStaticType() {
                    unitLayer = (currentRTSLayer).clone() /* as UnitLayer */;
                    jcall!(unitLayer).setParentLayer(rtsGameLayer.clone());
                    jcall!(assignWaypointsUtil).set(unitLayer.clone(), rtsGameLayer.clone());
                }
            }
        }
        return jcall!(NullUtil::getInstance()).NULL_OBJECT.clone();
    }
}
impl std::ops::Deref for SelectAdvancedRTSLayerVisitor {
    type Target = Visitor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SelectAdvancedRTSLayerVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn for SelectAdvancedRTSLayerVisitor {
    fn as_Visitor(&self) -> &crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &self.base }
    fn as_Visitor_mut(&mut self) -> &mut crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &mut self.base }
}
impl crate::org::allbinary::logic::util::visitor::VisitorInterface::VisitorInterface for SelectAdvancedRTSLayerVisitor {
    fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::visit(self, anyType) }
}


