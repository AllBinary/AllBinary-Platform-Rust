// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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
use crate::org::allbinary::game::identification::GroupCommonFactory::GroupCommonFactory;
use crate::org::allbinary::game::layer::RTSLayer::RTSLayer;
use crate::org::allbinary::game::layer::RTSPlayerLayerInterface::RTSPlayerLayerInterface;
use crate::org::allbinary::game::layer::item::LayerInterfaceFactoryImageItem::LayerInterfaceFactoryImageItem;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::direction::DirectionFactory::DirectionFactory;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::layer::LayerInterfaceFactoryInterface::LayerInterfaceFactoryInterface;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory;
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSFormInput {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub groupCommonFactory: Option<Rc<RefCell<GroupCommonFactory>>>,
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
    pub newUnconstructedRTSLayerInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>>,
    pub groupInterfaceArray: Vec<Option<Rc<RefCell<Group>>>>,
    pub selectedGeographicCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>,
    pub stickyItemSelected: bool,
    pub selectedStickyItemIndex: i32,
    pub selectedStickyItem: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>,
}


impl RTSFormInput {
    

    pub fn new_struct(mut groupInterfaceArray: Vec<Option<Rc<RefCell<Group>>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            groupCommonFactory: None,
            hashtable: None,
            newUnconstructedRTSLayerInterfaceArray: Vec::new(),
            groupInterfaceArray: Vec::new(),
            selectedGeographicCellPosition: None,
            stickyItemSelected: false,
            selectedStickyItemIndex: 0,
            selectedStickyItem: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.groupCommonFactory = GroupCommonFactory::getInstance();
        this.hashtable = jcall!(StdUtil::getInstance()).createHashtable();
        this.newUnconstructedRTSLayerInterfaceArray = vec![None; (7) as usize];
        this.selectedGeographicCellPosition = crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
        this.selectedStickyItem = ABCustomItem::getNullInstance();
        let mut size: i32 = (this.newUnconstructedRTSLayerInterfaceArray.len() as i32).clone();
        for index in (0..size) {
            this.newUnconstructedRTSLayerInterfaceArray[(index) as usize] = CollidableDestroyableDamageableLayer::getNullInstance();
        }
        this.groupInterfaceArray = groupInterfaceArray.clone();
        { let java_receiver = this.hashtable.clone(); let java_arguments = (crate::java_into_object(jcall!(DirectionFactory::getInstance()).NAME.clone()), crate::java_into_object(jcall!(DirectionFactory::getInstance()).DOWN.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1); };
        if groupInterfaceArray.is_some() {
            jcall!(this.hashtable).put(crate::java_into_object(jcall!(this.groupCommonFactory).ID.clone()), crate::java_into_object(groupInterfaceArray.clone()));
        }
        this
    }

    pub fn new(mut groupInterfaceArray: Vec<Option<Rc<RefCell<Group>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(groupInterfaceArray))))
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
    }
    

    // throws Exception
    pub fn processAtPoint(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        let mut index: i32 = self.getIndexAt(rtsPlayerLayerInterface.clone(), point.clone());
        if index < 0 {
            return;
        }
        let mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> = jcall!(rtsPlayerLayerInterface).getCurrentScrollSelectionForm();
        let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> = jcall!(scrollSelectionForm).as_ScrollSelectionForm_mut().get(index.clone());
        if item.is_none() {
            return;
        }
        self.processGameSpecific(associatedRtsLayer.clone(), rtsPlayerLayerInterface.clone(), layerManager.clone(), item.clone(), index.clone());
    }
    

    // throws Exception
    pub fn process(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        self.setAllBinaryGameLayerManager((layerManager).clone() /* as AllBinaryGameLayerManager */);
    }
    

    // throws Exception
    pub fn processGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut index: i32) {
    }
    

    // throws Exception
    pub fn processSticky(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> bool {
        let mut index: i32 = self.getIndexAt(rtsPlayerLayerInterface.clone(), point.clone());
        if index < 0 {
            return false;
        }
        let mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> = jcall!(rtsPlayerLayerInterface).getCurrentScrollSelectionForm();
        let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> = jcall!(scrollSelectionForm).as_ScrollSelectionForm_mut().get(index.clone());
        if item.is_none() {
            return false;
        }
        self.processStickyGameSpecific(associatedRtsLayer.clone(), rtsPlayerLayerInterface.clone(), layerManager.clone(), item.clone(), index.clone());
        return true;
    }
    

    // throws Exception
    pub fn processStickyGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut index: i32) {
    }
    

    // throws Exception
    pub fn getItemAt(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Point: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(point.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getItemAt"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> = jcall!(rtsPlayerLayerInterface).getCurrentScrollSelectionForm();
        return jcall!(scrollSelectionForm).getSelectedItem(point.clone());
    }
    

    // throws Exception
    pub fn getIndexAt(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> i32 {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Point: ")).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(point.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getItemAt"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> = jcall!(rtsPlayerLayerInterface).getCurrentScrollSelectionForm();
        return jcall!(scrollSelectionForm).getSelectedIndexForPoint(point.clone());
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut aItem: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> {
        let mut item: Option<Rc<RefCell<LayerInterfaceFactoryImageItem>>> = (aItem).clone() /* as LayerInterfaceFactoryImageItem */;
        let mut layerInterfaceFactoryInterface: Option<Rc<RefCell<dyn LayerInterfaceFactoryInterface>>> = jcall!(item).as_ABCustomItem_mut().getLayerInterfaceFactoryInterface();
        let mut cellPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(geographicMapCellPosition).getPoint();
        if layerInterfaceFactoryInterface.is_some() {
            jcall!(self.hashtable).put(crate::java_into_object(crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager_ID.with(Clone::clone).clone()), crate::java_into_object(layerManager.clone()));
            return (jcall!(layerInterfaceFactoryInterface).getNextInstance(self.getHashtable(), jcall!(cellPoint).getX(), jcall!(cellPoint).getY(), jcall!(cellPoint).getZ())).clone() /* as RTSLayer */;
        }
        return CollidableDestroyableDamageableLayer::getNullInstance();
    }
    

    pub fn getSelectedGeographicCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        return self.selectedGeographicCellPosition.clone();
    }
    

    pub fn setSelectedGeographicCellPosition(&mut self, mut selectedGeographicCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Selected GeographicMapCellPosition: ")).append(jcall!(selectedGeographicCellPosition).toString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setSelectedGeographicCellPosition"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.selectedGeographicCellPosition = selectedGeographicCellPosition.clone();
    }
    

    pub fn isStickyItemSelected(&mut self) -> bool {
        return self.stickyItemSelected.clone();
    }
    

    pub fn setStickyItemSelected(&mut self, mut stickyItemSelected: bool) {
        self.stickyItemSelected = stickyItemSelected.clone();
    }
    

    pub fn getSelectedStickyItem(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> {
        return self.selectedStickyItem.clone();
    }
    

    pub fn setSelectedStickyItem(&mut self, mut selectedStickyItem: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) {
        self.selectedStickyItem = selectedStickyItem.clone();
    }
    

    pub fn getSelectedStickyItemIndex(&mut self) -> i32 {
        return self.selectedStickyItemIndex.clone();
    }
    

    pub fn setSelectedStickyItemIndex(&mut self, mut selectedStickyItemIndex: i32) {
        self.selectedStickyItemIndex = selectedStickyItemIndex.clone();
    }
    

    pub fn getHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return self.hashtable.clone();
    }
}
pub trait RTSFormInputDyn {
    fn as_RTSFormInput(&self) -> &RTSFormInput;
    fn as_RTSFormInput_mut(&mut self) -> &mut RTSFormInput;
    fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { RTSFormInput::setAllBinaryGameLayerManager(self.as_RTSFormInput_mut(), allBinaryGameLayerManager) }
    fn processAtPoint(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) { RTSFormInput::processAtPoint(self.as_RTSFormInput_mut(), associatedRtsLayer, rtsPlayerLayerInterface, layerManager, point) }
    fn process(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { RTSFormInput::process(self.as_RTSFormInput_mut(), layerManager) }
    fn processGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut index: i32) { RTSFormInput::processGameSpecific(self.as_RTSFormInput_mut(), associatedRtsLayer, rtsPlayerLayerInterface, layerManager, item, index) }
    fn processSticky(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> bool { RTSFormInput::processSticky(self.as_RTSFormInput_mut(), associatedRtsLayer, rtsPlayerLayerInterface, layerManager, point) }
    fn processStickyGameSpecific(&mut self, mut associatedRtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut index: i32) { RTSFormInput::processStickyGameSpecific(self.as_RTSFormInput_mut(), associatedRtsLayer, rtsPlayerLayerInterface, layerManager, item, index) }
    fn getItemAt(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> { RTSFormInput::getItemAt(self.as_RTSFormInput_mut(), rtsPlayerLayerInterface, point) }
    fn getIndexAt(&mut self, mut rtsPlayerLayerInterface: Option<Rc<RefCell<dyn RTSPlayerLayerInterface>>>, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> i32 { RTSFormInput::getIndexAt(self.as_RTSFormInput_mut(), rtsPlayerLayerInterface, point) }
    fn getInstance(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut aItem: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> { RTSFormInput::getInstance(self.as_RTSFormInput_mut(), layerManager, aItem, geographicMapCellPosition) }
    fn getSelectedGeographicCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { RTSFormInput::getSelectedGeographicCellPosition(self.as_RTSFormInput_mut()) }
    fn setSelectedGeographicCellPosition(&mut self, mut selectedGeographicCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { RTSFormInput::setSelectedGeographicCellPosition(self.as_RTSFormInput_mut(), selectedGeographicCellPosition) }
    fn isStickyItemSelected(&mut self) -> bool { RTSFormInput::isStickyItemSelected(self.as_RTSFormInput_mut()) }
    fn setStickyItemSelected(&mut self, mut stickyItemSelected: bool) { RTSFormInput::setStickyItemSelected(self.as_RTSFormInput_mut(), stickyItemSelected) }
    fn getSelectedStickyItem(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> { RTSFormInput::getSelectedStickyItem(self.as_RTSFormInput_mut()) }
    fn setSelectedStickyItem(&mut self, mut selectedStickyItem: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) { RTSFormInput::setSelectedStickyItem(self.as_RTSFormInput_mut(), selectedStickyItem) }
    fn getSelectedStickyItemIndex(&mut self) -> i32 { RTSFormInput::getSelectedStickyItemIndex(self.as_RTSFormInput_mut()) }
    fn setSelectedStickyItemIndex(&mut self, mut selectedStickyItemIndex: i32) { RTSFormInput::setSelectedStickyItemIndex(self.as_RTSFormInput_mut(), selectedStickyItemIndex) }
    fn getHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { RTSFormInput::getHashtable(self.as_RTSFormInput_mut()) }
}
impl RTSFormInputDyn for RTSFormInput {
    fn as_RTSFormInput(&self) -> &RTSFormInput { self }
    fn as_RTSFormInput_mut(&mut self) -> &mut RTSFormInput { self }
}


