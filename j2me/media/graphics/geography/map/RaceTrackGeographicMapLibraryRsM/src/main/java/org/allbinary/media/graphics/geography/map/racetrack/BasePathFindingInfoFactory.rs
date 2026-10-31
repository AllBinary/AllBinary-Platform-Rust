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
// package org.allbinary.media.graphics.geography.map.racetrack
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingInfo::PathFindingInfo;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNode;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionFactoryInitVisitorInterface::GeographicMapCellPositionFactoryInitVisitorInterface;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::RaceTrackGeographicMapCellPositionFactoryInitVisitor::RaceTrackGeographicMapCellPositionFactoryInitVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasePathFindingInfoFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}


impl BasePathFindingInfoFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut mapArray: Vec<Vec<i32>>) {
        /* Java local class: move it out of the function
pub struct RaceTrackGeographicMapCellPositionFactoryInitVisitor {
    pub startLineId: i32,
    pub finishLineId: i32,
}

impl GeographicMapCellPositionFactoryInitVisitorInterface for RaceTrackGeographicMapCellPositionFactoryInitVisitor {
    fn visit(&mut self, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { Self::visit(self, tiledLayer, cellPosition) }
}

impl RaceTrackGeographicMapCellPositionFactoryInitVisitor {
            

            pub fn new_struct() -> Self {
                let mut this = Self {
            startLineId: 0,
            finishLineId: 0,
                };
                let mut raceTrackGeographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>> = jcall!(this.base.geographicMapInterface).getGeographicMapCellTypeFactory();
                this.startLineId = jcall!(raceTrackGeographicMapCellTypeFactory).getStartType();
                this.finishLineId = jcall!(raceTrackGeographicMapCellTypeFactory).getEndType();
                this
            }

            pub fn new() -> Option<Rc<RefCell<Self>>> {
                Some(Rc::new(RefCell::new(Self::new_struct())))
            }
            

            // throws Exception
            pub fn visit(&mut self, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
                let mut row: i32 = jcall!(cellPosition).getRow();
                let mut column: i32 = jcall!(cellPosition).getColumn();
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut cellTypeId: i32 = self.base.mapArray[(row) as usize][(column) as usize].clone();
                    let mut geographicCellType: i32 = jcall!(self.base.geographicMapInterface).getCellTypeFromMapCellTypeInt(cellTypeId.clone());
                    if geographicCellType == self.startLineId {
                        self.addStartPathFindingNode(self.base.pathFindingInfo.clone(), cellPosition.clone());
                    }
                    if geographicCellType == self.finishLineId {
                        if geographicCellType == self.startLineId {
                            self.addEndPathFindingNode(self.base.pathFindingInfo.clone(), jcall!(jcall!(self.base.geographicMapInterface).getGeographicMapCellPositionFactoryInterface()).getInstance(self.base.geographicMapInterface.clone(), jcall!(cellPosition).getColumn(), jcall!(cellPosition).getRow(), jcall!(tiledLayer).getColumns(), jcall!(tiledLayer).getRows(), jcall!(tiledLayer).getCellWidth(), jcall!(tiledLayer).getCellHeight()));
                        } else {
                            self.addEndPathFindingNode(self.base.pathFindingInfo.clone(), cellPosition.clone());
                        }
                    }
                    let mut raceTrackGeographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>> = jcall!(self.base.geographicMapInterface).getGeographicMapCellTypeFactory();
                    let mut geographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>> = GeographicMapCellTypeFactory::getInstance();
                    if jcall!(raceTrackGeographicMapCellTypeFactory).isPath(jcall!(geographicMapCellTypeFactory).get(geographicCellType.clone())) {
                        jcall!(RaceTrackRoadsGeographicMapCellHistoryFactory::RaceTrackRoadsGeographicMapCellHistoryFactory::getInstance()).track(cellPosition.clone());
                    } else {
                    }
                })) {
                    Ok(_) => {}
                    Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                    let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
                    jcall!(self.logUtil).put(jcall!(StringMaker::new()).append(jcall!(commonSeps).BRACKET_OPEN.clone()).appendint(row.clone()).append(jcall!(commonSeps).BRACKET_CLOSE.clone()).append(jcall!(commonSeps).BRACKET_OPEN.clone()).appendint(column.clone()).append(String::from("] in [")).appendint((self.base.mapArray.len() as i32).clone()).append(jcall!(commonSeps).BRACKET_CLOSE.clone()).append(jcall!(commonSeps).BRACKET_OPEN.clone()).appendint((self.base.mapArray[(0) as usize].len() as i32).clone()).append(jcall!(commonSeps).BRACKET_CLOSE.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("visit"), crate::java_into_object(e.clone()));
                    panic!("{:?}", e);
                    }
                }
            }
}
*/
        jcall!(jcall!(geographicMapInterface).getGeographicMapCellPositionFactory()).visit(RaceTrackGeographicMapCellPositionFactoryInitVisitor::new());
    }
    

    // throws Exception
    pub fn addStartPathFindingNode(&mut self, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut startGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        jcall!(pathFindingInfo).addStartPathFindingNode(PathFindingNode::new(jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(), startGeographicMapCellPosition.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>));
    }
    

    // throws Exception
    pub fn addEndPathFindingNode(&mut self, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut endGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        jcall!(pathFindingInfo).addEndPathFindingNode(PathFindingNode::new(jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(), endGeographicMapCellPosition.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNode::PathFindingNodeDyn>>));
    }
    

    // throws Exception
    pub fn getInstancePathFindingInfo(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut graphArray: Vec<Vec<i32>>) -> Option<Rc<RefCell<PathFindingInfo>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait BasePathFindingInfoFactoryDyn {
    fn as_BasePathFindingInfoFactory(&self) -> &BasePathFindingInfoFactory;
    fn as_BasePathFindingInfoFactory_mut(&mut self) -> &mut BasePathFindingInfoFactory;
    fn init(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut mapArray: Vec<Vec<i32>>) { BasePathFindingInfoFactory::init(self.as_BasePathFindingInfoFactory_mut(), geographicMapInterface, pathFindingInfo, mapArray) }
    fn addStartPathFindingNode(&mut self, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut startGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { BasePathFindingInfoFactory::addStartPathFindingNode(self.as_BasePathFindingInfoFactory_mut(), pathFindingInfo, startGeographicMapCellPosition) }
    fn addEndPathFindingNode(&mut self, mut pathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>>, mut endGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { BasePathFindingInfoFactory::addEndPathFindingNode(self.as_BasePathFindingInfoFactory_mut(), pathFindingInfo, endGeographicMapCellPosition) }
    fn getInstancePathFindingInfo(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut graphArray: Vec<Vec<i32>>) -> Option<Rc<RefCell<PathFindingInfo>>> { BasePathFindingInfoFactory::getInstancePathFindingInfo(self.as_BasePathFindingInfoFactory_mut(), geographicMapInterface, graphArray) }
}
impl BasePathFindingInfoFactoryDyn for BasePathFindingInfoFactory {
    fn as_BasePathFindingInfoFactory(&self) -> &BasePathFindingInfoFactory { self }
    fn as_BasePathFindingInfoFactory_mut(&mut self) -> &mut BasePathFindingInfoFactory { self }
}


