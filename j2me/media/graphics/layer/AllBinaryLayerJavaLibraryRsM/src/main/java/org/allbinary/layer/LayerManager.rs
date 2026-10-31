// Generated Code Do Not Modify
// package org.allbinary.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::SynchObject::SynchObject;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::ABSystemWrapper::ABSystemWrapper;
use super::LayerManagerLoggingBase::LayerManagerLoggingBase;
use super::AllBinaryLayer::AllBinaryLayer;
use super::Layer::Layer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerManager {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub systemWrapper: Option<Rc<RefCell<ABSystemWrapper>>>,
    pub tsUtil: Option<Rc<RefCell<TsUtil>>>,
    pub layerManagerLogging: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManagerLoggingBase::LayerManagerLoggingBaseDyn>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub anyType: Option<Rc<RefCell<SynchObject>>>,
}


impl LayerManager {
    

    pub fn new_struct(mut layerManagerLogging: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManagerLoggingBase::LayerManagerLoggingBaseDyn>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            systemWrapper: None,
            tsUtil: None,
            layerManagerLogging: None,
            list: None,
            anyType: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.systemWrapper = ABSystemWrapper::getInstance();
        this.tsUtil = TsUtil::getInstance();
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.anyType = SynchObject::new();
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.layerManagerLogging = layerManagerLogging.clone();
        this
    }

    pub fn new(mut layerManagerLogging: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManagerLoggingBase::LayerManagerLoggingBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(layerManagerLogging))))
    }
    

    pub fn contains(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool {
        return jcall!(self.list).contains(crate::java_into_object(layerInterface.clone()));
    }
    

    // throws Exception
    pub fn update(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        let mut had: bool = jcall!(self.list).remove(crate::java_into_object(layerInterface.clone()));
        if had {
            let mut nextLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = None;
            let mut size: i32 = jcall!(self.list).size();
            for index in (0..size) {
                nextLayerInterface = { let java_object = (jcall!(self.list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>>().expect("ClassCastException").clone()) };
                if jcall!(layerInterface).getZP() > jcall!(nextLayerInterface).getZP() {
                    jcall!(self.list).addAt(index.clone(), crate::java_into_object(layerInterface.clone()));
                    return;
                }
            }
            jcall!(self.list).add(crate::java_into_object(layerInterface.clone()));
        }
    }
    

    // throws Exception
    pub fn insert(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        let mut nextLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = None;
        let mut size: i32 = jcall!(self.list).size();
        for index in (0..size) {
            nextLayerInterface = { let java_object = (jcall!(self.list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>>().expect("ClassCastException").clone()) };
            if jcall!(layerInterface).getZP() > jcall!(nextLayerInterface).getZP() {
                self.appendAt(layerInterface.clone(), index.clone());
                return;
            }
        }
        self.append(layerInterface.clone());
    }
    

    // throws Exception
    pub fn append(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        jcall!(self.layerManagerLogging).append(layerInterface.clone());
        jcall!(self.list).add(crate::java_into_object(layerInterface.clone()));
    }
    

    // throws Exception
    pub fn appendAt(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut index: i32) {
        jcall!(self.layerManagerLogging).appendAt(layerInterface.clone(), index.clone());
        jcall!(self.list).addAt(index.clone(), crate::java_into_object(layerInterface.clone()));
    }
    

    // throws Exception
    pub fn remove(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        { // synchronized (self.anyType)
            jcall!(self.layerManagerLogging).remove(layerInterface.clone());
            let mut result: bool = jcall!(self.list).remove(crate::java_into_object(layerInterface.clone()));
            jcall!(self.layerManagerLogging).removeResult(None /* Java this: a &mut self method has no Rc of itself */, layerInterface.clone(), result.clone());
        }
    }
    

    pub fn getLayerAt(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>> {
        return { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getSize(&mut self) -> i32 {
        return jcall!(self.list).size();
    }
    

    // throws Exception
    pub fn cleanup(&mut self) {
        { // synchronized (self.anyType)
            jcall!(self.list).clear();
            jcall!(self.layerManagerLogging).clear();
            jcall!(self.tsUtil).gc();
            jcall!(self.tsUtil).gc();
        }
    }
    

    pub fn paint(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        { // synchronized (self.anyType)
            let mut comp: Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>> = None;
            {
                let mut index: i32 = jcall!(self.list).size();
                while { index -= 1; index } >= 0 {
                comp = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>>().expect("ClassCastException").clone()) };
                if comp.is_some() && jcall!(comp).isVisible() {
                    jcall!(comp).paint(g.clone());
                }
                }
            }
        }
    }
}
pub trait LayerManagerDyn {
    fn as_LayerManager(&self) -> &LayerManager;
    fn as_LayerManager_mut(&mut self) -> &mut LayerManager;
    fn contains(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool { LayerManager::contains(self.as_LayerManager_mut(), layerInterface) }
    fn update(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { LayerManager::update(self.as_LayerManager_mut(), layerInterface) }
    fn insert(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { LayerManager::insert(self.as_LayerManager_mut(), layerInterface) }
    fn append(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { LayerManager::append(self.as_LayerManager_mut(), layerInterface) }
    fn appendAt(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut index: i32) { LayerManager::appendAt(self.as_LayerManager_mut(), layerInterface, index) }
    fn remove(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { LayerManager::remove(self.as_LayerManager_mut(), layerInterface) }
    fn getLayerAt(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>> { LayerManager::getLayerAt(self.as_LayerManager_mut(), index) }
    fn getSize(&mut self) -> i32 { LayerManager::getSize(self.as_LayerManager_mut()) }
    fn cleanup(&mut self) { LayerManager::cleanup(self.as_LayerManager_mut()) }
    fn paint(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { LayerManager::paint(self.as_LayerManager_mut(), g, x, y) }
}
impl LayerManagerDyn for LayerManager {
    fn as_LayerManager(&self) -> &LayerManager { self }
    fn as_LayerManager_mut(&mut self) -> &mut LayerManager { self }
}


