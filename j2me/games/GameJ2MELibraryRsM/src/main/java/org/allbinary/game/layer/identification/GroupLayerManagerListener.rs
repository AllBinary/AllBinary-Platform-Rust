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
// package org.allbinary.game.layer.identification
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::identification::GroupInterfaceCompositeInterface::GroupInterfaceCompositeInterface;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::event::LayerManagerEvent::LayerManagerEvent;
use crate::org::allbinary::layer::event::LayerManagerEventHandler::LayerManagerEventHandler;
use crate::org::allbinary::layer::event::LayerManagerEventListener::LayerManagerEventListener;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GroupLayerManagerListener {
    pub base: LayerManagerEventListener,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

pub static mut GroupLayerManagerListener_SINGLETON: std::cell::LazyCell<Option<Rc<RefCell<GroupLayerManagerListener>>>> = std::cell::LazyCell::new(|| GroupLayerManagerListener::new());
impl crate::org::allbinary::layer::event::LayerManagerEventListenerInterface::LayerManagerEventListenerInterface for GroupLayerManagerListener {
    fn onCreateLayerManagerEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<LayerManagerEvent>>>) { Self::onCreateLayerManagerEvent(self, layerManagerEvent) }
    fn onDeleteLayerManagerEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<LayerManagerEvent>>>) { Self::onDeleteLayerManagerEvent(self, layerManagerEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for GroupLayerManagerListener {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl GroupLayerManagerListener {
    

    pub fn getInstance() -> Option<Rc<RefCell<GroupLayerManagerListener>>> {
        return unsafe { GroupLayerManagerListener_SINGLETON.clone() }.clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: LayerManagerEventListener::new_struct(),
            logUtil: None,
            list: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct())));
            jcall!(LayerManagerEventHandler::getInstance()).addListener((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn clear(&mut self) {
        let mut groupList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        let mut size: i32 = jcall!(self.list).size();
        for index in (0..=size - 1).rev() {
            groupList = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(groupList).clear();
        }
    }
    

    pub fn getGroupSizeFromInterface(&mut self, mut groupInterfaceCompositeInterface: Option<Rc<RefCell<dyn GroupInterfaceCompositeInterface>>>) -> i32 {
        let mut groupInterfaceArray: Vec<Option<Rc<RefCell<Group>>>> = jcall!(groupInterfaceCompositeInterface).getGroupInterface();
        return self.getGroupSize(groupInterfaceArray[(0) as usize].clone());
    }
    

    pub fn getGroupSize(&mut self, mut groupInterface: Option<Rc<RefCell<Group>>>) -> i32 {
        let mut id: i32 = ((jcall!(groupInterface).getGroupId()) as i32);
        return self.getGroupSizeById(id.clone());
    }
    

    pub fn getList(&mut self, mut groupInterface: Option<Rc<RefCell<Group>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut id: i32 = ((jcall!(groupInterface).getGroupId()) as i32);
        return self.getListById(id.clone());
    }
    

    pub fn getListById(&mut self, mut groupId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut groupList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(groupId) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
        return groupList.clone();
    }
    

    pub fn getGroupSizeById(&mut self, mut groupId: i32) -> i32 {
        let mut groupList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(groupId) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
        let mut size: i32 = jcall!(groupList).size();
        return size.clone();
    }
    

    pub fn areAllOtherGroupsEmpty(&mut self, mut groupInterface: Option<Rc<RefCell<Group>>>) -> bool {
        let mut id: i32 = ((jcall!(groupInterface).getGroupId()) as i32);
        let mut size: i32 = jcall!(self.list).size();
        for index in (0..=size - 1).rev() {
            if id != index {
                let mut groupSize: i32 = self.getGroupSizeById(index.clone());
                if groupSize != 0 {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Group Size: ")).appendint(groupSize.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("areAllOtherGroupsEmpty"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    return false;
                }
            }
        }
        return true;
    }
    

    pub fn isIdInList(&mut self, mut id: i32, mut excludeGroupList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> bool {
        let mut size: i32 = jcall!(excludeGroupList).size();
        let mut groupInterfaceArray: Vec<Option<Rc<RefCell<Group>>>> = Vec::new();
        let mut groupInterface: Option<Rc<RefCell<Group>>> = None;
        let mut groupId: i32 = 0;
        for index in (0..=size - 1).rev() {
            groupInterfaceArray = (jcall!(excludeGroupList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone() /* as  */;
            let mut size2: i32 = (groupInterfaceArray.len() as i32).clone();
            for index2 in (0..size2) {
                groupInterface = groupInterfaceArray[(index2) as usize].clone();
                groupId = ((jcall!(groupInterface).getGroupId()) as i32);
                if groupId == id {
                    return true;
                }
            }
        }
        return false;
    }
    

    pub fn areAllOtherGroupsLessThan(&mut self, mut excludeGroupList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut maxSize: i32) -> bool {
        let mut size: i32 = jcall!(self.list).size();
        for index in (0..=size - 1).rev() {
            if !self.isIdInList(index.clone(), excludeGroupList.clone()) {
                let mut groupSize: i32 = self.getGroupSizeById(index.clone());
                if groupSize >= maxSize {
                    return false;
                }
            }
        }
        return true;
    }
    

    pub fn init(&mut self, mut total: i32) {
        while jcall!(self.list).size() <= total + 1 {
            jcall!(self.list).add(crate::java_into_object(BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>)));
        }
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    // throws Exception
    pub fn onCreateLayerManagerEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<LayerManagerEvent>>>) {
        let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = jcall!(layerManagerEvent).getLayerInterface();
        let mut groupInterfaceArray: Vec<Option<Rc<RefCell<Group>>>> = jcall!(layerInterface).getGroupInterface();
        let mut size: i32 = (groupInterfaceArray.len() as i32).clone();
        let mut id: i32 = 0;
        let mut groupList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        for index in (0..size) {
            id = ((jcall!(groupInterfaceArray[(index) as usize]).getGroupId()) as i32);
            groupList = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(id) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
            if groupList.is_none() {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("id: ")).appendint(id.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onCreateLayerManagerEvent"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            if !jcall!(groupList).contains(crate::java_into_object(layerInterface.clone())) {
                jcall!(groupList).add(crate::java_into_object(layerInterface.clone()));
            } else {
            }
        }
    }
    

    // throws Exception
    pub fn onDeleteLayerManagerEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<LayerManagerEvent>>>) {
        let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = jcall!(layerManagerEvent).getLayerInterface();
        let mut groupInterfaceArray: Vec<Option<Rc<RefCell<Group>>>> = jcall!(layerInterface).getGroupInterface();
        let mut size: i32 = (groupInterfaceArray.len() as i32).clone();
        let mut id: i32 = 0;
        let mut groupList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        for index in (0..size) {
            id = ((jcall!(groupInterfaceArray[(index) as usize]).getGroupId()) as i32);
            groupList = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(id) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(groupList).remove(crate::java_into_object(layerInterface.clone()));
        }
    }
    

    pub fn log(&mut self) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut GROUP: String = String::from("Group: ");
        let mut SPACE: String = jcall!(CommonSeps::getInstance()).SPACE.clone();
        let mut TOTAL_LABEL: String = jcall!(CommonLabels::getInstance()).TOTAL_LABEL.clone();
        let mut size: i32 = jcall!(self.list).size();
        for index in (0..=size - 1).rev() {
            let mut groupList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(stringBuffer).append(GROUP.clone());
            jcall!(stringBuffer).appendint(index.clone());
            jcall!(stringBuffer).append(SPACE.clone());
            jcall!(stringBuffer).append(TOTAL_LABEL.clone());
            jcall!(stringBuffer).appendint(jcall!(groupList).size());
            jcall!(stringBuffer).append(SPACE.clone());
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("log"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
}
impl std::ops::Deref for GroupLayerManagerListener {
    type Target = LayerManagerEventListener;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GroupLayerManagerListener {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::layer::event::LayerManagerEventListener::LayerManagerEventListenerDyn for GroupLayerManagerListener {
    fn as_LayerManagerEventListener(&self) -> &crate::org::allbinary::layer::event::LayerManagerEventListener::LayerManagerEventListener { &self.base }
    fn as_LayerManagerEventListener_mut(&mut self) -> &mut crate::org::allbinary::layer::event::LayerManagerEventListener::LayerManagerEventListener { &mut self.base }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
    fn onCreateLayerManagerEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<LayerManagerEvent>>>) { Self::onCreateLayerManagerEvent(self, layerManagerEvent) }
    fn onDeleteLayerManagerEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<LayerManagerEvent>>>) { Self::onDeleteLayerManagerEvent(self, layerManagerEvent) }
}


