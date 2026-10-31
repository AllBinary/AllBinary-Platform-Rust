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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use crate::org::allbinary::game::layer::geographic::map::LayerCoveringCellPositionsUtil::LayerCoveringCellPositionsUtil;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::CellPositionsUtil::CellPositionsUtil;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::util::BasicArrayListS::BasicArrayListS;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use super::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapCellPositionArea {
    pub base: GeographicMapCellPositionAreaBase,
    pub cellPositionsUtil: Option<Rc<RefCell<CellPositionsUtil>>>,
    pub layerCoveringCellPositionsUtil: Option<Rc<RefCell<LayerCoveringCellPositionsUtil>>>,
    pub reusableOccupyingGeographicMapCellPositionList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub reusableSurroundingGeographicMapCellPositionList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub surroundingCircularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
    pub layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>,
    pub LIST: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub occupyingGeographicMapCellPositionList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub surroundingGeographicMapCellPositionList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl GeographicMapCellPositionArea {
    

    pub fn new_struct(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Self {
        let mut this = Self {
            base: GeographicMapCellPositionAreaBase::new_struct(),
            cellPositionsUtil: None,
            layerCoveringCellPositionsUtil: None,
            reusableOccupyingGeographicMapCellPositionList: None,
            reusableSurroundingGeographicMapCellPositionList: None,
            surroundingCircularIndexUtil: None,
            layerInterface: None,
            LIST: None,
            occupyingGeographicMapCellPositionList: None,
            surroundingGeographicMapCellPositionList: None,
        };
        this.cellPositionsUtil = CellPositionsUtil::getInstance();
        this.layerCoveringCellPositionsUtil = LayerCoveringCellPositionsUtil::getInstance();
        this.reusableOccupyingGeographicMapCellPositionList = BasicArrayListS::new(4).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.reusableSurroundingGeographicMapCellPositionList = BasicArrayListS::new(12).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.surroundingCircularIndexUtil = CircularIndexUtil::createInstance(0);
        this.LIST = jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
        this.occupyingGeographicMapCellPositionList = this.LIST.clone();
        this.surroundingGeographicMapCellPositionList = this.LIST.clone();
        this.layerInterface = layerInterface.clone();
        this
    }

    pub fn new(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(layerInterface))))
    }
    

    // throws Exception
    pub fn update(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) {
        self.occupyingGeographicMapCellPositionList = { let java_receiver = self.layerCoveringCellPositionsUtil.clone(); let java_arguments = (geographicMapInterface.clone(), self.layerInterface.clone(), jcall!(self.layerInterface).getXP(), jcall!(self.layerInterface).getYP(), self.reusableOccupyingGeographicMapCellPositionList.clone(),); let java_result = jcall!(java_receiver).getAllXY(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4); java_result };
        self.surroundingGeographicMapCellPositionList = jcall!(self.cellPositionsUtil).getAllSurrounding(geographicMapInterface.clone(), self.occupyingGeographicMapCellPositionList.clone(), self.reusableSurroundingGeographicMapCellPositionList.clone());
        { let java_receiver = self.surroundingCircularIndexUtil.clone(); let java_arguments = (jcall!(self.surroundingGeographicMapCellPositionList).size(),); jcall!(java_receiver).setSize(java_arguments.0); };
    }
    

    pub fn getOccupyingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.occupyingGeographicMapCellPositionList.clone();
    }
    

    pub fn getSurroundingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.surroundingGeographicMapCellPositionList.clone();
    }
    

    pub fn getNextSurroundingGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = { let java_object = ({ let java_receiver = self.surroundingGeographicMapCellPositionList.clone(); let java_arguments = (jcall!(self.surroundingCircularIndexUtil).getIndex(),); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
        jcall!(self.surroundingCircularIndexUtil).next();
        return geographicMapCellPosition.clone();
    }
}
impl std::ops::Deref for GeographicMapCellPositionArea {
    type Target = GeographicMapCellPositionAreaBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GeographicMapCellPositionArea {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn for GeographicMapCellPositionArea {
    fn as_GeographicMapCellPositionAreaBase(&self) -> &crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBase { &self.base }
    fn as_GeographicMapCellPositionAreaBase_mut(&mut self) -> &mut crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBase { &mut self.base }
}
impl crate::org::allbinary::game::layer::GeographicMapCellPositionAreaInterface::GeographicMapCellPositionAreaInterface for GeographicMapCellPositionArea {
    fn getNextSurroundingGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { Self::getNextSurroundingGeographicMapCellPosition(self) }
    fn getOccupyingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getOccupyingGeographicMapCellPositionList(self) }
    fn getSurroundingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getSurroundingGeographicMapCellPositionList(self) }
    fn update(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) { Self::update(self, geographicMapInterface) }
}


