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
// package org.allbinary.game.ag.layer.weapon.mine
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::FeaturedAnimationInterfaceFactoryInterfaceFactory::FeaturedAnimationInterfaceFactoryInterfaceFactory;
use crate::org::allbinary::game::combat::weapon::WeaponProperties::WeaponProperties;
use crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactory;
use crate::org::allbinary::game::layer::pickup::IconLayerFactory::IconLayerFactory;
use crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface;
use crate::org::allbinary::game::layer::pickup::PickedUpLayerTypeFactory::PickedUpLayerTypeFactory;
use crate::org::allbinary::game::layer::weapon::WeaponLayerCircularPool::WeaponLayerCircularPool;
use crate::org::allbinary::game::score::ScoreableInterface::ScoreableInterface;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MineCountedPickedUpLayerInterfaceFactory {
    pub base: CountedPickedUpLayerInterfaceFactory,
    pub weaponLayerCircularStaticPool: Option<Rc<RefCell<WeaponLayerCircularPool>>>,
    pub weaponProperties: Option<Rc<RefCell<WeaponProperties>>>,
    pub smallIntegerSingletonFactory: Option<Rc<RefCell<SmallIntegerSingletonFactory>>>,
}

pub static mut MineCountedPickedUpLayerInterfaceFactory_pickedUpLayerInterfaceFactoryInterface: std::cell::LazyCell<Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>>> = std::cell::LazyCell::new(|| crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactory_NULL_COUNTED_PICKUP_LAYER_FACTORY.with(Clone::clone).clone());
impl crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactoryInterface::CountedPickedUpLayerInterfaceFactoryInterface for MineCountedPickedUpLayerInterfaceFactory {
    fn getId(&mut self) -> i32 { self.base.getId() }
    fn getTotal(&mut self) -> i32 { Self::getTotal(self) }
}
impl crate::org::allbinary::layer::LayerInterfaceFactoryInterface::LayerInterfaceFactoryInterface for MineCountedPickedUpLayerInterfaceFactory {
    fn getNextInstance(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut x: i32, mut y: i32, mut z: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { Self::getNextInstance(self, hashtable, x, y, z) }
}

impl MineCountedPickedUpLayerInterfaceFactory {
    

    // throws Exception
    pub fn init() {
        unsafe { *MineCountedPickedUpLayerInterfaceFactory_pickedUpLayerInterfaceFactoryInterface = MineCountedPickedUpLayerInterfaceFactory::new() };
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CountedPickedUpLayerInterfaceFactory::new_struct(jcall!(PickedUpLayerTypeFactory::getInstance()).MINE.clone(), IconLayerFactory::getInstance(jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).get(jcall!(MineWeaponResources::MineWeaponResources::getInstance()).DROP_ICON_RESOURCE.clone())).getInstance(0), 10, 10), jcall!(jcall!(FeaturedAnimationInterfaceFactoryInterfaceFactory::getInstance()).get(jcall!(MineWeaponResources::MineWeaponResources::getInstance()).DROP_RESOURCE.clone())).getInstance(0)),
            weaponLayerCircularStaticPool: None,
            weaponProperties: None,
            smallIntegerSingletonFactory: None,
        };
        this.weaponLayerCircularStaticPool = MineLayerCircularStaticPool::MineLayerCircularStaticPool::getInstance();
        this.weaponProperties = WeaponProperties::new(-1_i64, -1_i64, 0, 8000, ((10) as i16));
        this.smallIntegerSingletonFactory = SmallIntegerSingletonFactory::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn PickedUpLayerInterfaceFactoryInterface>>> {
        return unsafe { MineCountedPickedUpLayerInterfaceFactory_pickedUpLayerInterfaceFactoryInterface.clone() }.clone();
    }
    

    pub fn getTotal(&mut self) -> i32 {
        return 3;
    }
    

    // throws Exception
    pub fn getNextInstance(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut x: i32, mut y: i32, mut z: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        let mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = { let java_object = (jcall!(hashtable).get(crate::java_into_object((jcall!(self.smallIntegerSingletonFactory).getAt(0)).clone() /* as Any */))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>>().expect("ClassCastException").clone()) };
        let mut angle: Option<i32> = { let java_object = jcall!(hashtable).get(crate::java_into_object((jcall!(self.smallIntegerSingletonFactory).getAt(1)).clone() /* as Any */)); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i32>().copied().or_else(|| object.downcast_ref::<Option<i32>>().copied().flatten()).expect("ClassCastException") }) };
        let mut scoreableInterface: Option<Rc<RefCell<dyn ScoreableInterface>>> = (sourceLayerInterface).clone() /* as ScoreableInterface */;
        return { let java_receiver = self.weaponLayerCircularStaticPool.clone(); let java_arguments = (sourceLayerInterface.clone(), x.clone(), y.clone(), z.clone(), ((angle).unwrap() as i32), 0, self.weaponProperties.clone(), scoreableInterface.clone(),); let java_result = jcall!(java_receiver).getInstanceWeaponLayer(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5, java_arguments.6, java_arguments.7); java_result };
    }
}
impl std::ops::Deref for MineCountedPickedUpLayerInterfaceFactory {
    type Target = CountedPickedUpLayerInterfaceFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MineCountedPickedUpLayerInterfaceFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactoryDyn for MineCountedPickedUpLayerInterfaceFactory {
    fn as_CountedPickedUpLayerInterfaceFactory(&self) -> &crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactory { &self.base }
    fn as_CountedPickedUpLayerInterfaceFactory_mut(&mut self) -> &mut crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactory { &mut self.base }
    fn getNextInstance(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut x: i32, mut y: i32, mut z: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { Self::getNextInstance(self, hashtable, x, y, z) }
    fn getTotal(&mut self) -> i32 { Self::getTotal(self) }
}
impl crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactoryDyn for MineCountedPickedUpLayerInterfaceFactory {
    fn as_PickedUpLayerInterfaceFactory(&self) -> &crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory { &self.base.base }
    fn as_PickedUpLayerInterfaceFactory_mut(&mut self) -> &mut crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory { &mut self.base.base }
}
impl crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBaseDyn for MineCountedPickedUpLayerInterfaceFactory {
    fn as_PickedUpLayerInterfaceFactoryBase(&self) -> &crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBase { &self.base.base.base }
    fn as_PickedUpLayerInterfaceFactoryBase_mut(&mut self) -> &mut crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBase { &mut self.base.base.base }
}
impl crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface for MineCountedPickedUpLayerInterfaceFactory {
    fn getPickedUpLayerType(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickedUpLayerType::PickedUpLayerType>>> { crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory::getPickedUpLayerType(crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactoryDyn::as_PickedUpLayerInterfaceFactory_mut(self)) }
    fn getIconLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory::getIconLayer(crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactoryDyn::as_PickedUpLayerInterfaceFactory_mut(self)) }
    fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactory::getAnimationInterface(crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactory::PickedUpLayerInterfaceFactoryDyn::as_PickedUpLayerInterfaceFactory_mut(self)) }
}


