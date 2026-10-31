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
// package org.allbinary.game.part.weapon
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::game::combat::weapon::WeaponProperties::WeaponProperties;
use crate::org::allbinary::game::part::PartInterface::PartInterface;
use crate::org::allbinary::game::score::NoScoreable::NoScoreable;
use crate::org::allbinary::game::score::ScoreableInterface::ScoreableInterface;
use crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationship;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use super::SalvoInterface::SalvoInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicWeaponPart {
    pub animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>,
    pub weaponProperties: Option<Rc<RefCell<WeaponProperties>>>,
    pub scoreableInterface: Option<Rc<RefCell<dyn ScoreableInterface>>>,
    pub relativeRelationship: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationshipDyn>>>,
}

thread_local! { pub static BasicWeaponPart_NULL_BASIC_WEAPON_PART: Option<Rc<RefCell<dyn BasicWeaponPartDyn>>> = BasicWeaponPart::createBasicWeaponPart(jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0)); }
impl PartInterface for BasicWeaponPart {
}
impl crate::org::allbinary::animation::AnimationInterfaceCompositeInterface::AnimationInterfaceCompositeInterface for BasicWeaponPart {
    fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getAnimationInterface(self) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for BasicWeaponPart {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}
impl SalvoInterface for BasicWeaponPart {
    fn process(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut angle: i16, mut otherAngle: i16) { Self::process(self, allbinaryLayerManager, angle, otherAngle) }
    fn processScore(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut angle: i16, mut otherAngle: i16, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>, mut scoreableInterface: Option<Rc<RefCell<dyn ScoreableInterface>>>) { Self::processScore(self, allbinaryLayerManager, angle, otherAngle, weaponProperties, scoreableInterface) }
}

impl BasicWeaponPart {
    

    pub fn createBasicWeaponPart(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn BasicWeaponPartDyn>>> {
        return BasicWeaponPart::new(animationInterface.clone(), crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone).clone(), crate::org::allbinary::game::combat::weapon::WeaponProperties::WeaponProperties_NULL_WEAPON_PROPERTIES.with(Clone::clone).clone(), NoScoreable::getInstance(), crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationship_NULL_RELATIVE_RELATIONSHIP.with(Clone::clone).clone()).map(|reference| reference as Rc<RefCell<dyn BasicWeaponPartDyn>>);
    }
    

    pub fn new_struct(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>, mut scoreableInterface: Option<Rc<RefCell<dyn ScoreableInterface>>>, mut relativeRelationship: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationshipDyn>>>) -> Self {
        let mut this = Self {
            animationInterface: None,
            ownerLayerInterface: None,
            weaponProperties: None,
            scoreableInterface: None,
            relativeRelationship: None,
        };
        this.animationInterface = jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0);
        this.ownerLayerInterface = crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone).clone();
        this.weaponProperties = crate::org::allbinary::game::combat::weapon::WeaponProperties::WeaponProperties_NULL_WEAPON_PROPERTIES.with(Clone::clone).clone();
        this.scoreableInterface = NoScoreable::getInstance();
        this.relativeRelationship = crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationship_NULL_RELATIVE_RELATIONSHIP.with(Clone::clone).clone();
        this.init(sourceLayerInterface.clone(), weaponProperties.clone(), scoreableInterface.clone(), relativeRelationship.clone());
        this.setAnimationInterface(animationInterface.clone());
        this
    }

    pub fn new(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>, mut scoreableInterface: Option<Rc<RefCell<dyn ScoreableInterface>>>, mut relativeRelationship: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationshipDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(animationInterface, sourceLayerInterface, weaponProperties, scoreableInterface, relativeRelationship))))
    }
    

    pub fn init(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>, mut scoreableInterface: Option<Rc<RefCell<dyn ScoreableInterface>>>, mut relativeRelationship: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationshipDyn>>>) {
        self.setOwnerLayerInterface(sourceLayerInterface.clone());
        self.setWeaponProperties(weaponProperties.clone());
        self.scoreableInterface = scoreableInterface.clone();
        self.relativeRelationship = relativeRelationship.clone();
    }
    

    // throws Exception
    pub fn process(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut angle: i16, mut otherAngle: i16) {
        { let java_arguments = (allbinaryLayerManager.clone(), angle.clone(), otherAngle.clone(), self.getWeaponProperties(), self.scoreableInterface.clone(),); self.processScore(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4) };
    }
    

    // throws Exception
    pub fn processScore(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut angle: i16, mut otherAngle: i16, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>, mut scoreableInterface: Option<Rc<RefCell<dyn ScoreableInterface>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(CommonStrings::getInstance()).NOT_IMPLEMENTED.clone())));
    }
    

    pub fn getOwnerLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        return self.ownerLayerInterface.clone();
    }
    

    pub fn setOwnerLayerInterface(&mut self, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        self.ownerLayerInterface = ownerLayerInterface.clone();
    }
    

    pub fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.animationInterface.clone();
    }
    

    pub fn setAnimationInterface(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) {
        self.animationInterface = animationInterface.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> = jcall!(self.getOwnerLayerInterface()).getViewPosition();
        let mut viewX: i32 = jcall!(viewPosition).getX();
        let mut viewY: i32 = jcall!(viewPosition).getY();
        jcall!(self.animationInterface).paintXY(graphics.clone(), viewX.clone(), viewY.clone());
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn getWeaponProperties(&mut self) -> Option<Rc<RefCell<WeaponProperties>>> {
        return self.weaponProperties.clone();
    }
    

    pub fn setWeaponProperties(&mut self, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>) {
        self.weaponProperties = weaponProperties.clone();
    }
}
pub trait BasicWeaponPartDyn: crate::org::allbinary::game::part::weapon::SalvoInterface::SalvoInterface {
    fn as_BasicWeaponPart(&self) -> &BasicWeaponPart;
    fn as_BasicWeaponPart_mut(&mut self) -> &mut BasicWeaponPart;
    fn init(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>, mut scoreableInterface: Option<Rc<RefCell<dyn ScoreableInterface>>>, mut relativeRelationship: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationshipDyn>>>) { BasicWeaponPart::init(self.as_BasicWeaponPart_mut(), sourceLayerInterface, weaponProperties, scoreableInterface, relativeRelationship) }
    fn getOwnerLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { BasicWeaponPart::getOwnerLayerInterface(self.as_BasicWeaponPart_mut()) }
    fn setOwnerLayerInterface(&mut self, mut ownerLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { BasicWeaponPart::setOwnerLayerInterface(self.as_BasicWeaponPart_mut(), ownerLayerInterface) }
    fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { BasicWeaponPart::getAnimationInterface(self.as_BasicWeaponPart_mut()) }
    fn setAnimationInterface(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) { BasicWeaponPart::setAnimationInterface(self.as_BasicWeaponPart_mut(), animationInterface) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { BasicWeaponPart::paint(self.as_BasicWeaponPart_mut(), graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { BasicWeaponPart::paintThreed(self.as_BasicWeaponPart_mut(), graphics) }
    fn getWeaponProperties(&mut self) -> Option<Rc<RefCell<WeaponProperties>>> { BasicWeaponPart::getWeaponProperties(self.as_BasicWeaponPart_mut()) }
    fn setWeaponProperties(&mut self, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>) { BasicWeaponPart::setWeaponProperties(self.as_BasicWeaponPart_mut(), weaponProperties) }
}
impl BasicWeaponPartDyn for BasicWeaponPart {
    fn as_BasicWeaponPart(&self) -> &BasicWeaponPart { self }
    fn as_BasicWeaponPart_mut(&mut self) -> &mut BasicWeaponPart { self }
}


