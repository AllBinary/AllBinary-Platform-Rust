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
// package org.allbinary.game.layer.special
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::collision::CollidableNeverCollideBehaviorFactory::CollidableNeverCollideBehaviorFactory;
use crate::org::allbinary::game::combat::damage::DamageableInterface::DamageableInterface;
use crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface;
use crate::org::allbinary::game::combat::destroy::event::DestroyedEvent::DestroyedEvent;
use crate::org::allbinary::game::identification::BasicGroupFactory::BasicGroupFactory;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent;
use crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManager;
use crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer;
use crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface;
use crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior;
use crate::org::allbinary::game::layer::pickup::PickupCompositeInterface::PickupCompositeInterface;
use crate::org::allbinary::game::part::PartInterface::PartInterface;
use crate::org::allbinary::game::part::PartInterfaceUtil::PartInterfaceUtil;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::RectangleFactory::RectangleFactory;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::math::PositionStrings::PositionStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use super::SpecialGameInputInterface::SpecialGameInputInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollidableDestroyableDamageableLayer {
    pub base: CollidableCompositeLayer,
    pub groupInterface: Vec<Option<Rc<RefCell<Group>>>>,
    pub readyForExplosion: bool,
    pub initWidth: i32,
    pub initHeight: i32,
    pub initX: i32,
    pub initY: i32,
    pub initZ: i32,
    pub partInterfaceArrayP: Vec<Option<Rc<RefCell<dyn PartInterface>>>>,
    pub pickupBehavior: Option<Rc<RefCell<PickupBehavior>>>,
    pub allBinaryGameLayerManagerP: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>,
    pub isDraggable: bool,
    pub isDragged: bool,
    pub isDropped: bool,
}

pub static mut CollidableDestroyableDamageableLayer_NULL_COLLIDABLE_DESTROYABLE_DAMAGE_LAYER: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
thread_local! { pub static CollidableDestroyableDamageableLayer_READYFOREXPLOSION: String = String::from("ReadyForExplosion: "); }
// impl DestroyableInterface for CollidableDestroyableDamageableLayer with the methods of the trait
// impl DamageableInterface for CollidableDestroyableDamageableLayer with the methods of the trait
impl PickupCompositeInterface for CollidableDestroyableDamageableLayer {
    fn getPickupBehavior(&mut self) -> Option<Rc<RefCell<PickupBehavior>>> { Self::getPickupBehavior(self) }
    fn setPickupBehavior(&mut self, mut pickupBehavior: Option<Rc<RefCell<PickupBehavior>>>) { Self::setPickupBehavior(self, pickupBehavior) }
}
// impl SpecialGameInputInterface for CollidableDestroyableDamageableLayer with the methods of the trait
impl crate::org::allbinary::game::identification::GroupInterfaceCompositeInterface::GroupInterfaceCompositeInterface for CollidableDestroyableDamageableLayer {
    fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<Group>>>> { Self::getGroupInterface(self) }
}

impl CollidableDestroyableDamageableLayer {
    

    pub fn getNullInstance() -> Option<Rc<RefCell<dyn CollidableDestroyableDamageableLayerDyn>>> {
        if match (&unsafe { CollidableDestroyableDamageableLayer_NULL_COLLIDABLE_DESTROYABLE_DAMAGE_LAYER.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *CollidableDestroyableDamageableLayer_NULL_COLLIDABLE_DESTROYABLE_DAMAGE_LAYER = CollidableDestroyableDamageableLayer::new(jcall!(BasicGroupFactory::getInstance()).NONE_ARRAY.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), crate::org::allbinary::graphics::RectangleFactory::RectangleFactory_SINGLETON.with(Clone::clone).clone(), crate::org::allbinary::view::ViewPositionBase::ViewPositionBase_NULL_VIEW_POSITION.with(Clone::clone).clone()).map(|reference| reference as Rc<RefCell<dyn CollidableDestroyableDamageableLayerDyn>>) };
        }
        return (unsafe { CollidableDestroyableDamageableLayer_NULL_COLLIDABLE_DESTROYABLE_DAMAGE_LAYER.clone() }).clone() /* as CollidableDestroyableDamageableLayer */;
    }
    

    pub fn new_struct(mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut name: String, mut layerInfo: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Self {
        let mut this = Self {
            base: CollidableCompositeLayer::new_struct(name.clone(), layerInfo.clone(), viewPosition.clone(), CollidableNeverCollideBehaviorFactory::getInstance()),
            groupInterface: Vec::new(),
            readyForExplosion: false,
            initWidth: 0,
            initHeight: 0,
            initX: 0,
            initY: 0,
            initZ: 0,
            partInterfaceArrayP: Vec::new(),
            pickupBehavior: None,
            allBinaryGameLayerManagerP: None,
            isDraggable: false,
            isDragged: false,
            isDropped: false,
        };
        this.partInterfaceArrayP = PartInterfaceUtil::getZeroArray();
        this.allBinaryGameLayerManagerP = AllBinaryGameLayerManager::getNullInstance();
        this.isDraggable = false;
        this.isDragged = false;
        this.isDropped = false;
        // super(...) is in the struct literal above
        this.initWidth = jcall!(layerInfo).getWidth();
        this.initHeight = jcall!(layerInfo).getHeight();
        this.groupInterface = groupInterface.clone();
        this.pickupBehavior = PickupBehavior::getNullInstance();
        this
    }

    pub fn new(mut groupInterface: Vec<Option<Rc<RefCell<Group>>>>, mut name: String, mut layerInfo: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(groupInterface, name, layerInfo, viewPosition))))
    }
    

    // throws Exception
    pub fn initPosition(&mut self) {
        self.setPosition(self.initX.clone(), self.initY.clone(), self.initZ.clone());
    }
    

    // throws Exception
    pub fn initPositionXYZ(&mut self, mut x: i32, mut y: i32, mut z: i32) {
        self.initX = x.clone();
        self.initY = y.clone();
        self.initZ = z.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.paint(graphics.clone());
    }
    

    // throws Exception
    pub fn damage(&mut self, mut damage: i32, mut damageType: i32) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    // throws Exception
    pub fn getDamage(&mut self, mut damageType: i32) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    // throws Exception
    pub fn isDestroyed(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    pub fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<Group>>>> {
        return self.groupInterface.clone();
    }
    

    pub fn setGroupInterface(&mut self, mut teamInterface: Vec<Option<Rc<RefCell<Group>>>>) {
        self.groupInterface = teamInterface.clone();
    }
    

    // throws Exception
    pub fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) {
        self.allBinaryGameLayerManagerP = allBinaryGameLayerManager.clone();
        if self.allBinaryGameLayerManagerP.is_none() {
            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
        }
    }
    

    pub fn isReadyForExplosion(&mut self) -> bool {
        return self.readyForExplosion.clone();
    }
    

    pub fn setReadyForExplosion(&mut self, mut isReadyForExplosion: bool) {
        self.readyForExplosion = isReadyForExplosion.clone();
    }
    

    pub fn getInitWidth(&mut self) -> i32 {
        return self.initWidth.clone();
    }
    

    pub fn getInitHeight(&mut self) -> i32 {
        return self.initHeight.clone();
    }
    

    pub fn getInitX(&mut self) -> i32 {
        return self.initX.clone();
    }
    

    pub fn getInitY(&mut self) -> i32 {
        return self.initY.clone();
    }
    

    // throws Exception
    pub fn up(&mut self) {
    }
    

    // throws Exception
    pub fn down(&mut self) {
    }
    

    // throws Exception
    pub fn right(&mut self) {
    }
    

    // throws Exception
    pub fn left(&mut self) {
    }
    

    // throws Exception
    pub fn strafeLeft(&mut self) {
    }
    

    // throws Exception
    pub fn strafeRight(&mut self) {
    }
    

    // throws Exception
    pub fn fire(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    // throws Exception
    pub fn special1(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    // throws Exception
    pub fn special2(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    // throws Exception
    pub fn special3(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    // throws Exception
    pub fn special4(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    // throws Exception
    pub fn special5(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) {
    }
    

    // throws Exception
    pub fn upAnalog(&mut self, mut yAnalogValue: i32) {
    }
    

    // throws Exception
    pub fn downAnalog(&mut self, mut yAnalogValue: i32) {
    }
    

    // throws Exception
    pub fn rightAnalog(&mut self, mut xAnalogValue: i32) {
    }
    

    // throws Exception
    pub fn leftAnalog(&mut self, mut xAnalogValue: i32) {
    }
    

    // throws Exception
    pub fn rightTrigger(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>, mut xAnalogValue: i32) {
    }
    

    // throws Exception
    pub fn leftTrigger(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>, mut xAnalogValue: i32) {
    }
    

    pub fn onDestroyed(&mut self, mut destroyedEvent: Option<Rc<RefCell<DestroyedEvent>>>) {
    }
    

    pub fn getPickupBehavior(&mut self) -> Option<Rc<RefCell<PickupBehavior>>> {
        return self.pickupBehavior.clone();
    }
    

    pub fn setPickupBehavior(&mut self, mut pickupBehavior: Option<Rc<RefCell<PickupBehavior>>>) {
        self.pickupBehavior = pickupBehavior.clone();
    }
    

    // throws Exception
    pub fn addPart(&mut self, mut pickedUpLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>>) {
    }
    

    pub fn setPartInterfaceArray(&mut self, mut partInterfaceArray: Vec<Option<Rc<RefCell<dyn PartInterface>>>>) {
        self.partInterfaceArrayP = partInterfaceArray.clone();
    }
    

    pub fn getPartInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn PartInterface>>>> {
        return self.partInterfaceArrayP.clone();
    }
    

    pub fn toStringAppend(&mut self, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>) {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut positionStrings: Option<Rc<RefCell<PositionStrings>>> = PositionStrings::getInstance();
        self.base.toStringAppend(stringBuffer.clone());
        jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
        let mut groupInterfaceArray: Vec<Option<Rc<RefCell<Group>>>> = self.getGroupInterface();
        let mut size: i32 = (groupInterfaceArray.len() as i32).clone();
        for index in (0..size) {
            jcall!(stringBuffer).append(jcall!(groupInterfaceArray[(index) as usize]).toString());
            jcall!(stringBuffer).append(jcall!(commonSeps).COMMA.clone());
        }
        let mut pickupBehavior: Option<Rc<RefCell<PickupBehavior>>> = self.getPickupBehavior();
        if pickupBehavior.is_some() {
            jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
            jcall!(stringBuffer).append(jcall!(pickupBehavior).toString());
        }
        jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
        jcall!(stringBuffer).append(CollidableDestroyableDamageableLayer_READYFOREXPLOSION.with(Clone::clone).clone());
        jcall!(stringBuffer).appendboolean(self.readyForExplosion.clone());
        jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
        jcall!(stringBuffer).append(jcall!(self.commonStrings).INIT.clone());
        jcall!(stringBuffer).append(jcall!(commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(positionStrings).X_LABEL.clone());
        jcall!(stringBuffer).appendint(self.initX.clone());
        jcall!(stringBuffer).append(jcall!(positionStrings).Y_LABEL.clone());
        jcall!(stringBuffer).appendint(self.initX.clone());
        jcall!(stringBuffer).append(jcall!(positionStrings).Z_LABEL.clone());
        jcall!(stringBuffer).appendint(self.initZ.clone());
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        self.toStringAppend(stringBuffer.clone());
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for CollidableDestroyableDamageableLayer {
    type Target = CollidableCompositeLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CollidableDestroyableDamageableLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CollidableDestroyableDamageableLayerDyn: crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn + crate::org::allbinary::game::layer::pickup::PickupCompositeInterface::PickupCompositeInterface {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &CollidableDestroyableDamageableLayer;
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut CollidableDestroyableDamageableLayer;
    fn initPosition(&mut self) { CollidableDestroyableDamageableLayer::initPosition(self.as_CollidableDestroyableDamageableLayer_mut()) }
    fn initPositionXYZ(&mut self, mut x: i32, mut y: i32, mut z: i32) { CollidableDestroyableDamageableLayer::initPositionXYZ(self.as_CollidableDestroyableDamageableLayer_mut(), x, y, z) }
    fn damage(&mut self, mut damage: i32, mut damageType: i32) { CollidableDestroyableDamageableLayer::damage(self.as_CollidableDestroyableDamageableLayer_mut(), damage, damageType) }
    fn getDamage(&mut self, mut damageType: i32) -> i32 { CollidableDestroyableDamageableLayer::getDamage(self.as_CollidableDestroyableDamageableLayer_mut(), damageType) }
    fn isDestroyed(&mut self) -> bool { CollidableDestroyableDamageableLayer::isDestroyed(self.as_CollidableDestroyableDamageableLayer_mut()) }
    fn setGroupInterface(&mut self, mut teamInterface: Vec<Option<Rc<RefCell<Group>>>>) { CollidableDestroyableDamageableLayer::setGroupInterface(self.as_CollidableDestroyableDamageableLayer_mut(), teamInterface) }
    fn setAllBinaryGameLayerManager(&mut self, mut allBinaryGameLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayerManager::AllBinaryGameLayerManagerDyn>>>) { CollidableDestroyableDamageableLayer::setAllBinaryGameLayerManager(self.as_CollidableDestroyableDamageableLayer_mut(), allBinaryGameLayerManager) }
    fn isReadyForExplosion(&mut self) -> bool { CollidableDestroyableDamageableLayer::isReadyForExplosion(self.as_CollidableDestroyableDamageableLayer_mut()) }
    fn setReadyForExplosion(&mut self, mut isReadyForExplosion: bool) { CollidableDestroyableDamageableLayer::setReadyForExplosion(self.as_CollidableDestroyableDamageableLayer_mut(), isReadyForExplosion) }
    fn getInitWidth(&mut self) -> i32 { CollidableDestroyableDamageableLayer::getInitWidth(self.as_CollidableDestroyableDamageableLayer_mut()) }
    fn getInitHeight(&mut self) -> i32 { CollidableDestroyableDamageableLayer::getInitHeight(self.as_CollidableDestroyableDamageableLayer_mut()) }
    fn getInitX(&mut self) -> i32 { CollidableDestroyableDamageableLayer::getInitX(self.as_CollidableDestroyableDamageableLayer_mut()) }
    fn getInitY(&mut self) -> i32 { CollidableDestroyableDamageableLayer::getInitY(self.as_CollidableDestroyableDamageableLayer_mut()) }
    fn up(&mut self) { CollidableDestroyableDamageableLayer::up(self.as_CollidableDestroyableDamageableLayer_mut()) }
    fn down(&mut self) { CollidableDestroyableDamageableLayer::down(self.as_CollidableDestroyableDamageableLayer_mut()) }
    fn right(&mut self) { CollidableDestroyableDamageableLayer::right(self.as_CollidableDestroyableDamageableLayer_mut()) }
    fn left(&mut self) { CollidableDestroyableDamageableLayer::left(self.as_CollidableDestroyableDamageableLayer_mut()) }
    fn strafeLeft(&mut self) { CollidableDestroyableDamageableLayer::strafeLeft(self.as_CollidableDestroyableDamageableLayer_mut()) }
    fn strafeRight(&mut self) { CollidableDestroyableDamageableLayer::strafeRight(self.as_CollidableDestroyableDamageableLayer_mut()) }
    fn fire(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { CollidableDestroyableDamageableLayer::fire(self.as_CollidableDestroyableDamageableLayer_mut(), layerManager, gameKeyEvent) }
    fn special1(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { CollidableDestroyableDamageableLayer::special1(self.as_CollidableDestroyableDamageableLayer_mut(), layerManager, gameKeyEvent) }
    fn special2(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { CollidableDestroyableDamageableLayer::special2(self.as_CollidableDestroyableDamageableLayer_mut(), layerManager, gameKeyEvent) }
    fn special3(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { CollidableDestroyableDamageableLayer::special3(self.as_CollidableDestroyableDamageableLayer_mut(), layerManager, gameKeyEvent) }
    fn special4(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { CollidableDestroyableDamageableLayer::special4(self.as_CollidableDestroyableDamageableLayer_mut(), layerManager, gameKeyEvent) }
    fn special5(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>) { CollidableDestroyableDamageableLayer::special5(self.as_CollidableDestroyableDamageableLayer_mut(), layerManager, gameKeyEvent) }
    fn upAnalog(&mut self, mut yAnalogValue: i32) { CollidableDestroyableDamageableLayer::upAnalog(self.as_CollidableDestroyableDamageableLayer_mut(), yAnalogValue) }
    fn downAnalog(&mut self, mut yAnalogValue: i32) { CollidableDestroyableDamageableLayer::downAnalog(self.as_CollidableDestroyableDamageableLayer_mut(), yAnalogValue) }
    fn rightAnalog(&mut self, mut xAnalogValue: i32) { CollidableDestroyableDamageableLayer::rightAnalog(self.as_CollidableDestroyableDamageableLayer_mut(), xAnalogValue) }
    fn leftAnalog(&mut self, mut xAnalogValue: i32) { CollidableDestroyableDamageableLayer::leftAnalog(self.as_CollidableDestroyableDamageableLayer_mut(), xAnalogValue) }
    fn rightTrigger(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>, mut xAnalogValue: i32) { CollidableDestroyableDamageableLayer::rightTrigger(self.as_CollidableDestroyableDamageableLayer_mut(), layerManager, gameKeyEvent, xAnalogValue) }
    fn leftTrigger(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>>, mut xAnalogValue: i32) { CollidableDestroyableDamageableLayer::leftTrigger(self.as_CollidableDestroyableDamageableLayer_mut(), layerManager, gameKeyEvent, xAnalogValue) }
    fn onDestroyed(&mut self, mut destroyedEvent: Option<Rc<RefCell<DestroyedEvent>>>) { CollidableDestroyableDamageableLayer::onDestroyed(self.as_CollidableDestroyableDamageableLayer_mut(), destroyedEvent) }
    fn addPart(&mut self, mut pickedUpLayerInterfaceFactoryInterface: Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>>) { CollidableDestroyableDamageableLayer::addPart(self.as_CollidableDestroyableDamageableLayer_mut(), pickedUpLayerInterfaceFactoryInterface) }
    fn setPartInterfaceArray(&mut self, mut partInterfaceArray: Vec<Option<Rc<RefCell<dyn PartInterface>>>>) { CollidableDestroyableDamageableLayer::setPartInterfaceArray(self.as_CollidableDestroyableDamageableLayer_mut(), partInterfaceArray) }
    fn getPartInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn PartInterface>>>> { CollidableDestroyableDamageableLayer::getPartInterfaceArray(self.as_CollidableDestroyableDamageableLayer_mut()) }
}
impl CollidableDestroyableDamageableLayerDyn for CollidableDestroyableDamageableLayer {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &CollidableDestroyableDamageableLayer { self }
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut CollidableDestroyableDamageableLayer { self }
}

impl crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn for CollidableDestroyableDamageableLayer {
    fn as_CollidableCompositeLayer(&self) -> &crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &self.base }
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &mut self.base }
}
impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for CollidableDestroyableDamageableLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base.base }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for CollidableDestroyableDamageableLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<Group>>>> { Self::getGroupInterface(self) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn toString(&mut self) -> String { Self::toString(self) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for CollidableDestroyableDamageableLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn toStringAppend(&mut self, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>) { Self::toStringAppend(self, stringBuffer) }
}
impl crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface for CollidableDestroyableDamageableLayer {
    fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::getCollidableInferface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for CollidableDestroyableDamageableLayer {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::set(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}


