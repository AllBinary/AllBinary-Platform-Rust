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
// package org.allbinary.media.graphics.geography.map
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::GeographicMapCellPositionBaseFactory::GeographicMapCellPositionBaseFactory;
use super::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use super::BasicGeographicMap::BasicGeographicMap;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapCellPositionFactory {
    pub base: GeographicMapCellPositionBaseFactory,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static GeographicMapCellPositionFactory_hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(StdUtil::getInstance()).createHashtable(); }

impl GeographicMapCellPositionFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: GeographicMapCellPositionBaseFactory::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut allBinaryTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
        let mut geographicMapCellPositionFactoryCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(GeographicMapCellPositionFactory_hashtable.with(Clone::clone)).get(crate::java_into_object(jcall!(allBinaryTiledLayer).getDataId()));
        if geographicMapCellPositionFactoryCanBeNull.is_none() {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Creating GeographicMapCellPositionFactory for TileLayer: ")).appendint(((jcall!(allBinaryTiledLayer).getDataId()).unwrap() as i32)).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            geographicMapCellPositionFactoryCanBeNull = BasicGeographicMapCellPositionFactory::new(geographicMapInterface.clone());
            jcall!(GeographicMapCellPositionFactory_hashtable.with(Clone::clone)).put(crate::java_into_object(jcall!(allBinaryTiledLayer).getDataId()), geographicMapCellPositionFactoryCanBeNull.clone());
            return (geographicMapCellPositionFactoryCanBeNull).clone() /* as BasicGeographicMapCellPositionFactory */;
        } else {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Reusing GeographicMapCellPositionFactory for TileLayer: ")).appendint(((jcall!(allBinaryTiledLayer).getDataId()).unwrap() as i32)).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            return (geographicMapCellPositionFactoryCanBeNull).clone() /* as BasicGeographicMapCellPositionFactory */;
        }
    }
    

    pub fn getHashtable() -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return GeographicMapCellPositionFactory_hashtable.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for GeographicMapCellPositionFactory {
    type Target = GeographicMapCellPositionBaseFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GeographicMapCellPositionFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionBaseFactory::GeographicMapCellPositionBaseFactoryDyn for GeographicMapCellPositionFactory {
    fn as_GeographicMapCellPositionBaseFactory(&self) -> &crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionBaseFactory::GeographicMapCellPositionBaseFactory { &self.base }
    fn as_GeographicMapCellPositionBaseFactory_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionBaseFactory::GeographicMapCellPositionBaseFactory { &mut self.base }
    fn getInstance(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> { Self::getInstance(self, geographicMapInterface) }
}


