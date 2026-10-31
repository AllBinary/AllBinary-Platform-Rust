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
// package org.allbinary.game.layer.pickup
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use super::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBase;
use super::PickedUpLayerType::PickedUpLayerType;
use super::IconLayer::IconLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PickedUpLayerInterfaceFactory {
    pub base: PickedUpLayerInterfaceFactoryBase,
    pub pickedUpLayerType: Option<Rc<RefCell<PickedUpLayerType>>>,
    pub iconLayer: Option<Rc<RefCell<IconLayer>>>,
    pub animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
}


impl PickedUpLayerInterfaceFactory {
    

    pub fn new_struct(mut pickeUpLayerType: Option<Rc<RefCell<PickedUpLayerType>>>, mut iconLayer: Option<Rc<RefCell<IconLayer>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Self {
        let mut this = Self {
            base: PickedUpLayerInterfaceFactoryBase::new_struct(),
            pickedUpLayerType: None,
            iconLayer: None,
            animationInterface: None,
        };
        this.pickedUpLayerType = pickeUpLayerType.clone();
        this.animationInterface = animationInterface.clone();
        this.iconLayer = iconLayer.clone();
        this
    }

    pub fn new(mut pickeUpLayerType: Option<Rc<RefCell<PickedUpLayerType>>>, mut iconLayer: Option<Rc<RefCell<IconLayer>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(pickeUpLayerType, iconLayer, animationInterface))))
    }
    

    pub fn getIconLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        return self.iconLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>);
    }
    

    pub fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.animationInterface.clone();
    }
    

    pub fn getPickedUpLayerType(&mut self) -> Option<Rc<RefCell<PickedUpLayerType>>> {
        return self.pickedUpLayerType.clone();
    }
}
impl std::ops::Deref for PickedUpLayerInterfaceFactory {
    type Target = PickedUpLayerInterfaceFactoryBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PickedUpLayerInterfaceFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait PickedUpLayerInterfaceFactoryDyn: crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBaseDyn {
    fn as_PickedUpLayerInterfaceFactory(&self) -> &PickedUpLayerInterfaceFactory;
    fn as_PickedUpLayerInterfaceFactory_mut(&mut self) -> &mut PickedUpLayerInterfaceFactory;
}
impl PickedUpLayerInterfaceFactoryDyn for PickedUpLayerInterfaceFactory {
    fn as_PickedUpLayerInterfaceFactory(&self) -> &PickedUpLayerInterfaceFactory { self }
    fn as_PickedUpLayerInterfaceFactory_mut(&mut self) -> &mut PickedUpLayerInterfaceFactory { self }
}

impl crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBaseDyn for PickedUpLayerInterfaceFactory {
    fn as_PickedUpLayerInterfaceFactoryBase(&self) -> &crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBase { &self.base }
    fn as_PickedUpLayerInterfaceFactoryBase_mut(&mut self) -> &mut crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryBase::PickedUpLayerInterfaceFactoryBase { &mut self.base }
}
impl crate::org::allbinary::game::layer::pickup::PickedUpLayerInterfaceFactoryInterface::PickedUpLayerInterfaceFactoryInterface for PickedUpLayerInterfaceFactory {
    fn getPickedUpLayerType(&mut self) -> Option<Rc<RefCell<PickedUpLayerType>>> { Self::getPickedUpLayerType(self) }
    fn getIconLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { Self::getIconLayer(self) }
    fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getAnimationInterface(self) }
}


