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
// package org.allbinary.game.layer.unit
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::game::layer::SteeringVisitor::SteeringVisitor;
use crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer;
use crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer;
use crate::org::allbinary::game::layer::CollidableRTSBehavior::CollidableRTSBehavior;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::game::input::event::GameKeyEventFactory::GameKeyEventFactory;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::game::layer::TiledLayerUtil::TiledLayerUtil;
use crate::org::allbinary::game::layer::geographic::map::LayerPartialCellPositionsUtil::LayerPartialCellPositionsUtil;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::java::bool::BooleanFactory::BooleanFactory;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCompositeInterface::GeographicMapCompositeInterface;
use crate::org::allbinary::media::graphics::geography::map::drop::DropCellPositionHistory::DropCellPositionHistory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::UnitLayer::UnitLayer;
use super::SimpleSteeringVisitor::SimpleSteeringVisitor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollidableUnitBehavior {
    pub base: CollidableRTSBehavior,
    pub layerPartialCellPositionsUtil: Option<Rc<RefCell<LayerPartialCellPositionsUtil>>>,
    pub simpleSteeringVisitor: Option<Rc<RefCell<SimpleSteeringVisitor>>>,
}

impl crate::org::allbinary::layer::NamedInterface::NamedInterface for CollidableUnitBehavior {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl CollidableUnitBehavior {
    

    pub fn new_struct(mut collidable: bool) -> Self {
        let mut this = Self {
            base: CollidableRTSBehavior::new_struct(collidable.clone()),
            layerPartialCellPositionsUtil: None,
            simpleSteeringVisitor: None,
        };
        this.layerPartialCellPositionsUtil = LayerPartialCellPositionsUtil::getInstance();
        this.simpleSteeringVisitor = SimpleSteeringVisitor::new();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut collidable: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(collidable))))
    }
    

    // throws Exception
    pub fn collideNone(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) {
        self.chase(ownerLayer.clone(), collidableInterfaceCompositeInterface.clone());
        let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = (collidableInterfaceCompositeInterface).clone() /* as AdvancedRTSGameLayer */;
        if jcall!(rtsLayer).getType() == UnitLayer::getStaticType() {
            self.collideUnit(ownerLayer.clone(), (rtsLayer).clone() /* as UnitLayer */);
        }
    }
    

    // throws Exception
    pub fn steer(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool {
        let mut ownerUnitLayer: Option<Rc<RefCell<UnitLayer>>> = (ownerLayer).clone() /* as UnitLayer */;
        let mut unitLayer: Option<Rc<RefCell<UnitLayer>>> = (collidableInterfaceCompositeInterface).clone() /* as UnitLayer */;
        let mut angleInfo2: Option<Rc<RefCell<AngleInfo>>> = jcall!(jcall!(unitLayer).getRotationAnimationInterface()).getAngleInfoP();
        let mut angleInfo: Option<Rc<RefCell<AngleInfo>>> = jcall!(jcall!(ownerUnitLayer).getRotationAnimationInterface()).getAngleInfoP();
        let mut angle: i32 = ((jcall!(angleInfo).getAngle()) as i32) - ((jcall!(angleInfo2).getAngle()) as i32);
        if angle < 90 || angle > 270 {
            return true;
        } else if angle >= 90 || angle <= 270 {
            jcall!(jcall!(ownerUnitLayer).getGameKeyEventList()).add(jcall!(GameKeyEventFactory::getInstance()).getInstanceForKey(ownerUnitLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface>>), unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() }.clone()));
            return false;
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Error"))));
    }
    

    // throws Exception
    pub fn chase(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) {
        let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = (collidableInterfaceCompositeInterface).clone() /* as AdvancedRTSGameLayer */;
        if jcall!(rtsLayer).getType() == UnitLayer::getStaticType() {
            if !jcall!(jcall!(self.simpleSteeringVisitor).getList()).contains(collidableInterfaceCompositeInterface.clone()) {
                jcall!(jcall!(self.simpleSteeringVisitor).getList()).add(collidableInterfaceCompositeInterface.clone());
            }
            let mut ownerUnitLayer: Option<Rc<RefCell<UnitLayer>>> = (ownerLayer).clone() /* as UnitLayer */;
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(jcall!(ownerUnitLayer).getUnitWaypointBehavior()).getSteeringVisitorList();
            if !jcall!(list).contains(crate::java_into_object(self.simpleSteeringVisitor.clone())) {
                jcall!(list).add(crate::java_into_object(self.simpleSteeringVisitor.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn collideUnit(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut unitLayer: Option<Rc<RefCell<UnitLayer>>>) {
        let mut ownerUnitLayer: Option<Rc<RefCell<UnitLayer>>> = (ownerLayer).clone() /* as UnitLayer */;
        let mut partialPositionList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = UnitLayer::getPartialpositionlist();
        let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (jcall!(ownerUnitLayer).allBinaryGameLayerManagerP).clone() /* as GeographicMapCompositeInterface */;
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
        let mut basicGeographicMap: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = geographicMapInterface.clone();
        let mut dropCellPositionHistory: Option<Rc<RefCell<DropCellPositionHistory>>> = DropCellPositionHistory::getInstance();
        let mut tiledLayerUtil: Option<Rc<RefCell<TiledLayerUtil>>> = TiledLayerUtil::getInstance();
        let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(basicGeographicMap).getAllBinaryTiledLayer();
        if jcall!(ownerLayer).getXP() < jcall!(unitLayer).getXP() && jcall!(ownerLayer).getX2() > jcall!(unitLayer).getXP() {
            let mut diff: i32 = jcall!(ownerLayer).getWidth() + 1;
            jcall!(self.layerPartialCellPositionsUtil).getAllDXY(basicGeographicMap.clone(), ownerLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>), -diff, 0, partialPositionList.clone());
            if !jcall!(dropCellPositionHistory).anyCellPositionWithDrop(partialPositionList.clone()) {
                let mut x: i32 = jcall!(unitLayer).getXP() - diff;
                let mut y: i32 = jcall!(ownerLayer).getYP();
                x = jcall!(tiledLayerUtil).keepOnMapX(tiledLayer.clone(), x.clone(), jcall!(ownerLayer).getWidth());
                y = jcall!(tiledLayerUtil).keepOnMapY(tiledLayer.clone(), y.clone(), jcall!(ownerLayer).getHeight());
                jcall!(ownerLayer).setPosition(x.clone(), y.clone(), jcall!(ownerLayer).getZP());
            }
            jcall!(self.layerPartialCellPositionsUtil).getAllDXY(basicGeographicMap.clone(), unitLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>), diff.clone(), 0, partialPositionList.clone());
            if !jcall!(dropCellPositionHistory).anyCellPositionWithDrop(partialPositionList.clone()) {
                let mut x: i32 = jcall!(unitLayer).getXP() + diff;
                let mut y: i32 = jcall!(unitLayer).getYP();
                x = jcall!(tiledLayerUtil).keepOnMapX(tiledLayer.clone(), x.clone(), jcall!(unitLayer).getWidth());
                y = jcall!(tiledLayerUtil).keepOnMapY(tiledLayer.clone(), y.clone(), jcall!(unitLayer).getHeight());
                jcall!(unitLayer).setPosition(x.clone(), y.clone(), jcall!(unitLayer).getZP());
            }
        }
        if jcall!(ownerLayer).getYP() < jcall!(unitLayer).getYP() && jcall!(ownerLayer).getY2() > jcall!(unitLayer).getYP() {
            let mut diff: i32 = jcall!(ownerLayer).getHeight() + 1;
            jcall!(self.layerPartialCellPositionsUtil).getAllDXY(basicGeographicMap.clone(), ownerLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>), 0, -diff, partialPositionList.clone());
            if !jcall!(dropCellPositionHistory).anyCellPositionWithDrop(partialPositionList.clone()) {
                let mut x: i32 = jcall!(ownerLayer).getXP();
                let mut y: i32 = jcall!(unitLayer).getYP() - diff;
                x = jcall!(tiledLayerUtil).keepOnMapX(tiledLayer.clone(), x.clone(), jcall!(ownerLayer).getWidth());
                y = jcall!(tiledLayerUtil).keepOnMapY(tiledLayer.clone(), y.clone(), jcall!(ownerLayer).getHeight());
                jcall!(ownerLayer).setPosition(x.clone(), y.clone(), jcall!(ownerLayer).getZP());
            }
            jcall!(self.layerPartialCellPositionsUtil).getAllDXY(basicGeographicMap.clone(), unitLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>), 0, diff.clone(), partialPositionList.clone());
            if !jcall!(dropCellPositionHistory).anyCellPositionWithDrop(partialPositionList.clone()) {
                let mut x: i32 = jcall!(unitLayer).getXP();
                let mut y: i32 = jcall!(unitLayer).getYP() + diff;
                x = jcall!(tiledLayerUtil).keepOnMapX(tiledLayer.clone(), x.clone(), jcall!(unitLayer).getWidth());
                y = jcall!(tiledLayerUtil).keepOnMapY(tiledLayer.clone(), y.clone(), jcall!(unitLayer).getHeight());
                jcall!(unitLayer).setPosition(x.clone(), y.clone(), jcall!(unitLayer).getZP());
            }
        }
    }
}
impl std::ops::Deref for CollidableUnitBehavior {
    type Target = CollidableRTSBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CollidableUnitBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::CollidableRTSBehavior::CollidableRTSBehaviorDyn for CollidableUnitBehavior {
    fn as_CollidableRTSBehavior(&self) -> &crate::org::allbinary::game::layer::CollidableRTSBehavior::CollidableRTSBehavior { &self.base }
    fn as_CollidableRTSBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableRTSBehavior::CollidableRTSBehavior { &mut self.base }
    fn collideNone(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) { Self::collideNone(self, ownerLayer, collidableInterfaceCompositeInterface) }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehaviorDyn for CollidableUnitBehavior {
    fn as_CollidableDestroyableDamageableBehavior(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior { &self.base.base }
    fn as_CollidableDestroyableDamageableBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior { &mut self.base.base }
}
impl crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn for CollidableUnitBehavior {
    fn as_CollidableBaseBehavior(&self) -> &crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &self.base.base.base }
    fn as_CollidableBaseBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &mut self.base.base.base }
    fn collide(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) { crate::org::allbinary::game::layer::CollidableRTSBehavior::CollidableRTSBehavior::collide(crate::org::allbinary::game::layer::CollidableRTSBehavior::CollidableRTSBehaviorDyn::as_CollidableRTSBehavior_mut(self), p1, p2) }
    fn isCollision(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool { crate::org::allbinary::game::layer::CollidableRTSBehavior::CollidableRTSBehavior::isCollision(crate::org::allbinary::game::layer::CollidableRTSBehavior::CollidableRTSBehaviorDyn::as_CollidableRTSBehavior_mut(self), p1, p2) }
    fn isCollisionInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface>>>) -> bool { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior::isCollisionInterface(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehaviorDyn::as_CollidableDestroyableDamageableBehavior_mut(self), p1, p2) }
    fn collideInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior::collideInterface(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehaviorDyn::as_CollidableDestroyableDamageableBehavior_mut(self), p1, p2) }
}


pub struct SimpleSteeringVisitor {
    pub base: SteeringVisitor,
}


impl SimpleSteeringVisitor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: SteeringVisitor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
        

        pub fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>> = (anyType).clone() /* as CollidableCompositeLayer */;
                if jcall!(self.getList()).size() > 0 {
                    let mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>> = (jcall!(self.getList()).get(0)).clone() /* as CollidableCompositeLayer */;
                    let mut clear: bool = self.steer(ownerLayer.clone(), allbinaryLayer.clone());
                    if clear {
                        jcall!(self.getList()).clear();
                        return jcall!(NullUtil::getInstance()).NULL_OBJECT.clone();
                    }
                    return jcall!(BooleanFactory::getInstance()).TRUE.clone();
                }
                return jcall!(NullUtil::getInstance()).NULL_OBJECT.clone();
            })) {
                Ok(java_return) => return java_return,
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("visit"), crate::java_into_object(e.clone()));
                return jcall!(NullUtil::getInstance()).NULL_OBJECT.clone();
                }
            }
        }
}
impl std::ops::Deref for SimpleSteeringVisitor {
    type Target = SteeringVisitor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SimpleSteeringVisitor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::SteeringVisitor::SteeringVisitorDyn for SimpleSteeringVisitor {
    fn as_SteeringVisitor(&self) -> &crate::org::allbinary::game::layer::SteeringVisitor::SteeringVisitor { &self }
    fn as_SteeringVisitor_mut(&mut self) -> &mut crate::org::allbinary::game::layer::SteeringVisitor::SteeringVisitor { &mut self }
}
impl crate::org::allbinary::logic::util::visitor::Visitor::VisitorDyn for SimpleSteeringVisitor {
    fn as_Visitor(&self) -> &crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &self }
    fn as_Visitor_mut(&mut self) -> &mut crate::org::allbinary::logic::util::visitor::Visitor::Visitor { &mut self }
}
impl crate::org::allbinary::logic::util::visitor::VisitorInterface::VisitorInterface for SimpleSteeringVisitor {
    fn visit(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::visit(self, anyType) }
}


