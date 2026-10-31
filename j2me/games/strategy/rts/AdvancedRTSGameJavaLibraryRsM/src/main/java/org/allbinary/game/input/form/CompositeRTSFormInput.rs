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
// package org.allbinary.game.input.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use super::RTSFormInput::RTSFormInput;
use super::WaypointRTSFormInput::WaypointRTSFormInput;
use super::UnitRTSFormInput::UnitRTSFormInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CompositeRTSFormInput {
    pub base: RTSFormInput,
    pub rtsFormInputArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn>>>>,
    pub itemIndex: Vec<i32>,
    pub isPrimaryWaypointCreator: bool,
}


impl CompositeRTSFormInput {
    

    pub fn new_struct(mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut isPrimaryWaypointCreator: bool, mut itemIndex: Vec<i32>) -> Self {
        let mut this = Self {
            base: RTSFormInput::new_struct(groupInterface.clone()),
            rtsFormInputArray: Vec::new(),
            itemIndex: Vec::new(),
            isPrimaryWaypointCreator: false,
        };
        // super(...) is in the struct literal above
        this.itemIndex = itemIndex.clone();
        this.isPrimaryWaypointCreator = isPrimaryWaypointCreator.clone();
        this.rtsFormInputArray = vec![None; (2) as usize];
        this.rtsFormInputArray[(0) as usize] = WaypointRTSFormInput::new(this.groupInterfaceArray.clone(), isPrimaryWaypointCreator.clone());
        this.rtsFormInputArray[(1) as usize] = UnitRTSFormInput::new(this.groupInterfaceArray.clone());
        this
    }

    pub fn new(mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut isPrimaryWaypointCreator: bool, mut itemIndex: Vec<i32>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(groupInterface, isPrimaryWaypointCreator, itemIndex))))
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        self.base.setAllBinaryGameLayerManager(allBinaryGameLayerManager.clone());
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (allBinaryGameLayerManager).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        let mut size: i32 = (self.rtsFormInputArray.len() as i32).clone();
        for index in (0..size) {
            jcall!(self.rtsFormInputArray[(index) as usize]).setAllBinaryGameLayerManager(allBinaryGameLayerManager.clone());
        }
    }
    

    // throws Exception
    pub fn processAtPoint(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        let mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> = jcall!(rtsPlayerLayerInterface).getCurrentScrollSelectionForm();
        let mut index: i32 = jcall!(scrollSelectionForm).getSelectedIndexForPoint(point.clone());
        if self.isStickyItemSelected() && match (&associatedRtsLayer, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if self.getSelectedStickyItemIndex() <= self.itemIndex[(0) as usize] {
                jcall!(self.rtsFormInputArray[(0) as usize]).processAtPoint(associatedRtsLayer.clone(), rtsPlayerLayerInterface.clone(), layerManager.clone(), point.clone());
            }
        } else if index > self.itemIndex[(0) as usize] {
            jcall!(self.rtsFormInputArray[(1) as usize]).processAtPoint(associatedRtsLayer.clone(), rtsPlayerLayerInterface.clone(), layerManager.clone(), point.clone());
        }
    }
    

    // throws Exception
    pub fn processGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut index: i32) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", format!("{}{}", format!("{}{}", jcall!(CommonLabels::getInstance()).INDEX_LABEL, index), String::from(" > ")), self.itemIndex[(0) as usize]), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).PROCESS.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        if self.isStickyItemSelected() && match (&associatedRtsLayer, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if self.getSelectedStickyItemIndex() <= self.itemIndex[(0) as usize] {
                jcall!(self.rtsFormInputArray[(0) as usize]).processGameSpecific(associatedRtsLayer.clone(), rtsPlayerLayerInterface.clone(), layerManager.clone(), item.clone(), index.clone());
            }
        } else if index > self.itemIndex[(0) as usize] {
            jcall!(self.rtsFormInputArray[(1) as usize]).processGameSpecific(associatedRtsLayer.clone(), rtsPlayerLayerInterface.clone(), layerManager.clone(), item.clone(), index.clone());
        }
    }
    

    // throws Exception
    pub fn processSticky(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> bool {
        return jcall!(self.rtsFormInputArray[(0) as usize]).processSticky(associatedRtsLayer.clone(), rtsPlayerLayerInterface.clone(), layerManager.clone(), point.clone());
    }
    

    // throws Exception
    pub fn processStickyGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut index: i32) {
        jcall!(self.rtsFormInputArray[(0) as usize]).processStickyGameSpecific(associatedRtsLayer.clone(), rtsPlayerLayerInterface.clone(), layerManager.clone(), item.clone(), index.clone());
    }
    

    pub fn getSelectedStickyItemIndex(&mut self) -> i32 {
        return jcall!(self.rtsFormInputArray[(0) as usize]).getSelectedStickyItemIndex();
    }
    

    pub fn isStickyItemSelected(&mut self) -> bool {
        return jcall!(self.rtsFormInputArray[(0) as usize]).isStickyItemSelected();
    }
    

    pub fn setStickyItemSelected(&mut self, mut stickyItemSelected: bool) {
        jcall!(self.rtsFormInputArray[(0) as usize]).setStickyItemSelected(stickyItemSelected.clone());
    }
    

    pub fn getSelectedStickyItem(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> {
        return jcall!(self.rtsFormInputArray[(0) as usize]).getSelectedStickyItem();
    }
    

    pub fn setSelectedStickyItem(&mut self, mut selectedStickyItem: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) {
        jcall!(self.rtsFormInputArray[(0) as usize]).setSelectedStickyItem(selectedStickyItem.clone());
    }
}
impl std::ops::Deref for CompositeRTSFormInput {
    type Target = RTSFormInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CompositeRTSFormInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInputDyn for CompositeRTSFormInput {
    fn as_RTSFormInput(&self) -> &crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput { &self.base }
    fn as_RTSFormInput_mut(&mut self) -> &mut crate::org::allbinary::game::input::form::RTSFormInput::RTSFormInput { &mut self.base }
    fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { Self::setAllBinaryGameLayerManager(self, allBinaryGameLayerManager) }
    fn processAtPoint(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) { Self::processAtPoint(self, associatedRtsLayer, rtsPlayerLayerInterface, layerManager, point) }
    fn processGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut index: i32) { Self::processGameSpecific(self, associatedRtsLayer, rtsPlayerLayerInterface, layerManager, item, index) }
    fn processSticky(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> bool { Self::processSticky(self, associatedRtsLayer, rtsPlayerLayerInterface, layerManager, point) }
    fn processStickyGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut index: i32) { Self::processStickyGameSpecific(self, associatedRtsLayer, rtsPlayerLayerInterface, layerManager, item, index) }
    fn isStickyItemSelected(&mut self) -> bool { Self::isStickyItemSelected(self) }
    fn setStickyItemSelected(&mut self, mut stickyItemSelected: bool) { Self::setStickyItemSelected(self, stickyItemSelected) }
    fn getSelectedStickyItem(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> { Self::getSelectedStickyItem(self) }
    fn setSelectedStickyItem(&mut self, mut selectedStickyItem: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) { Self::setSelectedStickyItem(self, selectedStickyItem) }
    fn getSelectedStickyItemIndex(&mut self) -> i32 { Self::getSelectedStickyItemIndex(self) }
}


