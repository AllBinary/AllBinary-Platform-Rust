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
// package org.allbinary.game.layer.weapon
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::khronos::opengles::GL::GL;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior;
use crate::org::allbinary::game::combat::destroy::DestroyedLayerProcessor::DestroyedLayerProcessor;
use crate::org::allbinary::game::combat::weapon::WeaponProperties::WeaponProperties;
use crate::org::allbinary::game::configuration::GameConfigurationUtil::GameConfigurationUtil;
use crate::org::allbinary::game::identification::BasicGroupFactory::BasicGroupFactory;
use crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer;
use crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo;
use crate::org::allbinary::game::score::NoScoreable::NoScoreable;
use crate::org::allbinary::game::score::ScoreableInterface::ScoreableInterface;
use crate::org::allbinary::game::tick::TickableInterface::TickableInterface;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::physics::movement::Movement::Movement;
use crate::org::allbinary::view::ViewPosition::ViewPosition;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use crate::java::lang::Exception::*;
use super::CollidableWeaponBehavior::CollidableWeaponBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WeaponLayer {
    pub base: MultiPlayerGameLayer,
    pub animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub initAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub destroyedAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>,
    pub totalDamage: i32,
    pub initDamage: i32,
    pub destroyed: bool,
    pub scoreableInterface: Option<Rc<RefCell<dyn ScoreableInterface>>>,
    pub movement: Option<Rc<RefCell<dyn crate::org::allbinary::physics::movement::Movement::MovementDyn>>>,
    pub weaponProperties: Option<Rc<RefCell<WeaponProperties>>>,
    pub multiPlayerType: i32,
    pub collidableWeaponBehavior: Option<Rc<RefCell<CollidableWeaponBehavior>>>,
}

// impl TickableInterface for WeaponLayer with the methods of the trait
impl crate::org::allbinary::game::combat::damage::DamageableInterface::DamageableInterface for WeaponLayer {
    fn damage(&mut self, mut damage: i32, mut damageType: i32) { Self::damage(self, damage, damageType) }
    fn getDamage(&mut self, mut damageType: i32) -> i32 { Self::getDamage(self, damageType) }
}
impl crate::org::allbinary::game::layer::special::SpecialGameInputInterface::SpecialGameInputInterface for WeaponLayer {
    fn up(&mut self) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::up(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn down(&mut self) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::down(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn right(&mut self) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::right(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn left(&mut self) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::left(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn strafeLeft(&mut self) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::strafeLeft(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn strafeRight(&mut self) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::strafeRight(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn fire(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::fire(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
    fn special1(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::special1(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
    fn special2(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::special2(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
    fn special3(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::special3(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
    fn special4(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::special4(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
    fn special5(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<crate::org::allbinary::game::input::event::GameKeyEvent::GameKeyEvent>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::special5(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1, p2) }
}
impl crate::org::allbinary::layer::LayerInterface::LayerInterface for WeaponLayer {
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getHeight(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getWidth(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getHalfHeight(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getHalfHeight(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getHalfWidth(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getHalfWidth(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getXP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getXP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getYP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getYP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getZP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getZP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getX2(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getX2(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getY2(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getY2(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getZ2(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getZ2(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn isVisible(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::isVisible(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn moveDXYZ(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::moveDXYZ(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
    fn setVisible(&mut self, p1: bool) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setVisible(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
    fn implmentsTickableInterface(&mut self) -> bool { Self::implmentsTickableInterface(self) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn implmentsGameInputInterface(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::implmentsGameInputInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn implmentsArtificialIntelligenceCompositeInterface(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::implmentsArtificialIntelligenceCompositeInterface(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getType(&mut self) -> i32 { Self::getType(self) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for WeaponLayer {
    fn getName(&mut self) -> String { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getName(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
}
impl crate::org::allbinary::layer::PositionInterface::PositionInterface for WeaponLayer {
    fn setPosition(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setPosition(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
}
impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for WeaponLayer {
    fn isDestroyed(&mut self) -> bool { Self::isDestroyed(self) }
}
impl crate::org::allbinary::game::identification::GroupInterfaceCompositeInterface::GroupInterfaceCompositeInterface for WeaponLayer {
    fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<crate::org::allbinary::game::identification::Group::Group>>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getGroupInterface(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for WeaponLayer {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paintThreed(self, graphics) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for WeaponLayer {
    fn onEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}
impl crate::org::allbinary::view::event::ViewPositionEventListenerInterface::ViewPositionEventListenerInterface for WeaponLayer {
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}

impl WeaponLayer {
    

    pub fn new_struct(mut name: String, mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut multiPlayerType: i32, mut movement: Option<Rc<RefCell<dyn crate::org::allbinary::physics::movement::Movement::MovementDyn>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut destroyedAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Self {
        let mut this = Self {
            base: MultiPlayerGameLayer::new_struct(remoteInfo.clone(), { let java_field = jcall!(BasicGroupFactory::getInstance()).NONE_ARRAY.clone(); java_field }.clone(), name.clone(), rectangle.clone(), viewPosition.clone()),
            animationInterface: None,
            initAnimationInterface: None,
            destroyedAnimationInterface: None,
            sourceLayerInterface: None,
            totalDamage: 0,
            initDamage: 0,
            destroyed: false,
            scoreableInterface: None,
            movement: None,
            weaponProperties: None,
            multiPlayerType: 0,
            collidableWeaponBehavior: None,
        };
        this.animationInterface = jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0);
        this.sourceLayerInterface = crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone).clone();
        this.destroyed = true;
        this.scoreableInterface = NoScoreable::getInstance().map(|reference| reference as Rc<RefCell<dyn ScoreableInterface>>);
        this.weaponProperties = crate::org::allbinary::game::combat::weapon::WeaponProperties::WeaponProperties_NULL_WEAPON_PROPERTIES.with(Clone::clone).clone();
        this.collidableWeaponBehavior = CollidableWeaponBehavior::getNullInstance();
        // super(...) is in the struct literal above
        this.initAnimationInterface = animationInterface.clone();
        this.setAnimationInterface(animationInterface.clone());
        this.destroyedAnimationInterface = destroyedAnimationInterface.clone();
        this.movement = movement.clone();
        this.multiPlayerType = multiPlayerType.clone();
        this
    }

    pub fn new(mut name: String, mut remoteInfo: Option<Rc<RefCell<RemoteInfo>>>, mut multiPlayerType: i32, mut movement: Option<Rc<RefCell<dyn crate::org::allbinary::physics::movement::Movement::MovementDyn>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut destroyedAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, remoteInfo, multiPlayerType, movement, animationInterface, destroyedAnimationInterface, rectangle, viewPosition))))
    }
    

    pub fn setCollidableInferface(&mut self, mut collidableInferface: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>>) {
        { let java_arguments = (collidableInferface.clone(),); self.base.setCollidableInferface(java_arguments.0) };
        self.collidableWeaponBehavior = (self.getCollidableInferface()).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<CollidableWeaponBehavior>() }, "ClassCastException: not a CollidableWeaponBehavior"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<CollidableWeaponBehavior>) } });
    }
    

    // throws Exception
    pub fn initXYZ(&mut self, mut x: i32, mut y: i32, mut z: i32) {
        self.x = x.clone();
        self.y = y.clone();
        self.z = z.clone();
    }
    

    // throws Exception
    pub fn init(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut angle: i32, mut otherAngle: i32, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>, mut scoreable: Option<Rc<RefCell<dyn ScoreableInterface>>>) {
        self.setWeaponPropertiesP(weaponProperties.clone());
        self.setReadyForExplosion(false);
        { let java_arguments = (self.getInitAnimationInterfaceP(),); self.setAnimationInterface(java_arguments.0) };
        self.setOwnerLayerInterface(sourceLayerInterface.clone());
        jcall!(self.collidableWeaponBehavior).init(sourceLayerInterface.clone());
        self.setGroupInterfaces();
        if scoreable.is_some() {
            self.scoreableInterface = scoreable.clone();
        }
        self.totalDamage = 0;
        self.initDamage = { let java_value = (jcall!(weaponProperties).getDamage()).clone(); java_value };
        self.setDestroyed(false);
        { let java_receiver = self.movement.clone(); let java_arguments = (jcall!(weaponProperties).getSpeed(), angle.clone(), otherAngle.clone(),); jcall!(java_receiver).as_Movement_mut().init(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn setGroupInterfaces(&mut self) {
        let mut basicGroupFactory: Option<Rc<RefCell<BasicGroupFactory>>> = BasicGroupFactory::getInstance();
        if match (&jcall!(self.sourceLayerInterface).getGroupInterface()[(0) as usize], &{ let java_field = jcall!(basicGroupFactory).ENEMY.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            { let java_arguments = ({ let java_field = jcall!(basicGroupFactory).ENEMY_ARRAY.clone(); java_field }.clone(),); self.setGroupInterface(java_arguments.0) };
        } else {
            { let java_arguments = (jcall!(self.sourceLayerInterface).getGroupInterface(),); self.setGroupInterface(java_arguments.0) };
        }
    }
    

    // throws Exception
    pub fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        if self.isExhausted() && !self.isDestroyed() {
            if self.isReadyForExplosion() {
                let mut indexedAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> { panic!("ClassCastException: a Rust dyn Animation cannot be cast to IndexedAnimation") })();
                let mut currentFrame: i32 = jcall!(indexedAnimationInterface).getFrame();
                if currentFrame < jcall!(indexedAnimationInterface).getAnimationSize() - 1 {
                    jcall!(indexedAnimationInterface).nextFrame();
                } else {
                    self.setDestroyed(true);
                }
            } else {
                let mut collidableWeaponBehavior: Option<Rc<RefCell<CollidableWeaponBehavior>>> = (self.getCollidableInferface()).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<CollidableWeaponBehavior>() }, "ClassCastException: not a CollidableWeaponBehavior"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<CollidableWeaponBehavior>) } });
                if jcall!(collidableWeaponBehavior).isCollided() {
                    { let java_arguments = (self.destroyedAnimationInterface.clone(),); self.setAnimationInterface(java_arguments.0) };
                    jcall!(self.getMovement()).as_Movement_mut().stop();
                    self.setReadyForExplosion(true);
                } else {
                    self.setDestroyed(true);
                }
            }
        }
        jcall!(self.movement).as_Movement_mut().process(None /* Java this: a &mut self method has no Rc of itself */);
        self.totalDamage += ((jcall!(self.weaponProperties).getDissipation()) as i32);
    }
    

    pub fn isDestroyed(&mut self) -> bool {
        return self.destroyed.clone();
    }
    

    pub fn damage(&mut self, mut damage: i32, mut damageType: i32) {
        self.totalDamage += damage * damage;
    }
    

    pub fn givePoints(&mut self, mut total: i32) {
        { let java_receiver = self.scoreableInterface.clone(); let java_arguments = (jcall!(GameConfigurationUtil::getInstance()).getCompetitionValue() * total,); jcall!(java_receiver).addPoints(java_arguments.0); };
    }
    

    pub fn isExhausted(&mut self) -> bool {
        if self.totalDamage > self.getInitDamage() || self.totalDamage < 0 {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn getDamage(&mut self, mut damageType: i32) -> i32 {
        if !self.isExhausted() {
            let mut total: i32 = self.getInitDamage() - self.totalDamage;
            self.givePoints(total.clone());
            return total.clone();
        }
        return 0;
    }
    

    pub fn setDestroyed(&mut self, mut destroyed: bool) {
        self.destroyed = destroyed.clone();
        if self.isDestroyed() {
            self.scoreableInterface = { let java_value = (NoScoreable::getInstance()).clone(); java_value }.map(|reference| reference as Rc<RefCell<dyn ScoreableInterface>>);
            jcall!(DestroyedLayerProcessor::getInstance()).add(None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
    

    pub fn getInitDamage(&mut self) -> i32 {
        return self.initDamage.clone();
    }
    

    pub fn setAnimationInterface(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) {
        self.animationInterface = animationInterface.clone();
    }
    

    pub fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.animationInterface.clone();
    }
    

    pub fn getInitAnimationInterfaceP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.initAnimationInterface.clone();
    }
    

    pub fn setInitAnimationInterfaceP(&mut self, mut initAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) {
        self.initAnimationInterface = initAnimationInterface.clone();
    }
    

    pub fn getDestroyedAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.destroyedAnimationInterface.clone();
    }
    

    pub fn getOwnerLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        return self.sourceLayerInterface.clone();
    }
    

    pub fn setOwnerLayerInterface(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        self.sourceLayerInterface = sourceLayerInterface.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> = self.getViewPosition();
        { let java_receiver = self.animationInterface.clone(); let java_arguments = (graphics.clone(), { let java_value = jcall!(viewPosition).getX(); java_value } - self.getHalfWidth(), { let java_value = jcall!(viewPosition).getY(); java_value } - self.getHalfHeight(),); jcall!(java_receiver).paintXY(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> = self.getViewPosition();
        { let java_receiver = self.animationInterface.clone(); let java_arguments = (graphics.clone(), { let java_value = jcall!(viewPosition).getX(); java_value } - self.getHalfWidth(), { let java_value = jcall!(viewPosition).getY(); java_value } - self.getHalfHeight(), { let java_value = jcall!(viewPosition).getZ(); java_value } - self.getHalfDepth(),); jcall!(java_receiver).paintThreedXYZ(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
    }
    

    pub fn getMovement(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::physics::movement::Movement::MovementDyn>>> {
        return self.movement.clone();
    }
    

    pub fn setMovement(&mut self, mut movement: Option<Rc<RefCell<dyn crate::org::allbinary::physics::movement::Movement::MovementDyn>>>) {
        self.movement = movement.clone();
    }
    

    pub fn getType(&mut self) -> i32 {
        return WeaponLayer::getStaticType();
    }
    

    pub fn getStaticType() -> i32 {
        return 0;
    }
    

    pub fn getMultiPlayerType(&mut self) -> i32 {
        return self.multiPlayerType.clone();
    }
    

    pub fn setWeaponPropertiesP(&mut self, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>) {
        self.weaponProperties = weaponProperties.clone();
    }
    

    pub fn getWeaponPropertiesP(&mut self) -> Option<Rc<RefCell<WeaponProperties>>> {
        return self.weaponProperties.clone();
    }
    

    pub fn implmentsTickableInterface(&mut self) -> bool {
        return true;
    }
    

    // throws Exception
    pub fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) {
        jcall!(self.initAnimationInterface).as_Animation_mut().set(gl.clone());
    }
}
impl std::ops::Deref for WeaponLayer {
    type Target = MultiPlayerGameLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for WeaponLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait WeaponLayerDyn: crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn {
    fn as_WeaponLayer(&self) -> &WeaponLayer;
    fn as_WeaponLayer_mut(&mut self) -> &mut WeaponLayer;
    fn initXYZ(&mut self, mut x: i32, mut y: i32, mut z: i32) { WeaponLayer::initXYZ(self.as_WeaponLayer_mut(), x, y, z) }
    fn init(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut angle: i32, mut otherAngle: i32, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>, mut scoreable: Option<Rc<RefCell<dyn ScoreableInterface>>>) { WeaponLayer::init(self.as_WeaponLayer_mut(), sourceLayerInterface, angle, otherAngle, weaponProperties, scoreable) }
    fn setGroupInterfaces(&mut self) { WeaponLayer::setGroupInterfaces(self.as_WeaponLayer_mut()) }
    fn givePoints(&mut self, mut total: i32) { WeaponLayer::givePoints(self.as_WeaponLayer_mut(), total) }
    fn isExhausted(&mut self) -> bool { WeaponLayer::isExhausted(self.as_WeaponLayer_mut()) }
    fn setDestroyed(&mut self, mut destroyed: bool) { WeaponLayer::setDestroyed(self.as_WeaponLayer_mut(), destroyed) }
    fn getInitDamage(&mut self) -> i32 { WeaponLayer::getInitDamage(self.as_WeaponLayer_mut()) }
    fn setAnimationInterface(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) { WeaponLayer::setAnimationInterface(self.as_WeaponLayer_mut(), animationInterface) }
    fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { WeaponLayer::getAnimationInterface(self.as_WeaponLayer_mut()) }
    fn getInitAnimationInterfaceP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { WeaponLayer::getInitAnimationInterfaceP(self.as_WeaponLayer_mut()) }
    fn setInitAnimationInterfaceP(&mut self, mut initAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) { WeaponLayer::setInitAnimationInterfaceP(self.as_WeaponLayer_mut(), initAnimationInterface) }
    fn getDestroyedAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { WeaponLayer::getDestroyedAnimationInterface(self.as_WeaponLayer_mut()) }
    fn getOwnerLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { WeaponLayer::getOwnerLayerInterface(self.as_WeaponLayer_mut()) }
    fn setOwnerLayerInterface(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { WeaponLayer::setOwnerLayerInterface(self.as_WeaponLayer_mut(), sourceLayerInterface) }
    fn getMovement(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::physics::movement::Movement::MovementDyn>>> { WeaponLayer::getMovement(self.as_WeaponLayer_mut()) }
    fn setMovement(&mut self, mut movement: Option<Rc<RefCell<dyn crate::org::allbinary::physics::movement::Movement::MovementDyn>>>) { WeaponLayer::setMovement(self.as_WeaponLayer_mut(), movement) }
    fn setWeaponPropertiesP(&mut self, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>) { WeaponLayer::setWeaponPropertiesP(self.as_WeaponLayer_mut(), weaponProperties) }
    fn getWeaponPropertiesP(&mut self) -> Option<Rc<RefCell<WeaponProperties>>> { WeaponLayer::getWeaponPropertiesP(self.as_WeaponLayer_mut()) }
}
impl WeaponLayerDyn for WeaponLayer {
    fn as_WeaponLayer(&self) -> &WeaponLayer { self }
    fn as_WeaponLayer_mut(&mut self) -> &mut WeaponLayer { self }
}

impl crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn for WeaponLayer {
    fn as_CollidableCompositeLayer(&self) -> &crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &self.base.base.base }
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &mut self.base.base.base }
    fn setCollidableInferface(&mut self, mut collidableInferface: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>>) { Self::setCollidableInferface(self, collidableInferface) }
}
impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for WeaponLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base.base.base.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base.base.base.base }
    fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processTick(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn for WeaponLayer {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &self.base.base }
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &mut self.base.base }
    fn damage(&mut self, mut damage: i32, mut damageType: i32) { Self::damage(self, damage, damageType) }
    fn getDamage(&mut self, mut damageType: i32) -> i32 { Self::getDamage(self, damageType) }
    fn isDestroyed(&mut self) -> bool { Self::isDestroyed(self) }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for WeaponLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base.base.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base.base.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paintThreed(self, graphics) }
    fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<crate::org::allbinary::game::identification::Group::Group>>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getGroupInterface(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn implmentsTickableInterface(&mut self) -> bool { Self::implmentsTickableInterface(self) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn getType(&mut self) -> i32 { Self::getType(self) }
    fn getMultiPlayerType(&mut self) -> i32 { Self::getMultiPlayerType(self) }
    fn toString(&mut self) -> String { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toString(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for WeaponLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base.base.base.base }
    fn setPosition(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setPosition(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
    fn moveDXYZ(&mut self, p1: i32, p2: i32, p3: i32) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::moveDXYZ(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1, p2, p3) }
    fn getXP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getXP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getYP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getYP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getZP(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getZP(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getWidth(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn getHeight(&mut self) -> i32 { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::getHeight(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn setVisible(&mut self, p1: bool) { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::setVisible(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self), p1) }
    fn isVisible(&mut self) -> bool { crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer::isVisible(crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn::as_AllBinaryLayer_mut(self)) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toStringAppend(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn for WeaponLayer {
    fn as_MultiPlayerGameLayer(&self) -> &crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &self.base }
    fn as_MultiPlayerGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &mut self.base }
}
impl crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface for WeaponLayer {
    fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::getCollidableInferface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayerInterface::MultiPlayerGameLayerInterface for WeaponLayer {
    fn getMultiplayerBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::multiplayer::layer::MultiplayerBehavior::MultiplayerBehaviorDyn>>> { crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer::getMultiplayerBehavior(crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn::as_MultiPlayerGameLayer_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for WeaponLayer {
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
}
impl crate::org::allbinary::game::layer::pickup::PickupCompositeInterface::PickupCompositeInterface for WeaponLayer {
    fn getPickupBehavior(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn setPickupBehavior(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::setPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}


