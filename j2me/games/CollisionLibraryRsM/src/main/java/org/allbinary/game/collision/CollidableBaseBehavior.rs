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
// package org.allbinary.game.collision
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::CollidableInterface::CollidableInterface;
use super::LayerCollisionUtil::LayerCollisionUtil;
use super::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface;
use super::CollisionType::CollisionType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollidableBaseBehavior {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub layerCollisionUtil: Option<Rc<RefCell<LayerCollisionUtil>>>,
    pub collidable: bool,
}

impl CollidableInterface for CollidableBaseBehavior {
    fn isCollidable(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool { Self::isCollidable(self, ownerLayer) }
    fn isCollision(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collisionLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool { Self::isCollision(self, ownerLayer, collisionLayer) }
    fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut allbinaryCollidableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) { Self::collide(self, ownerLayer, allbinaryCollidableLayer) }
    fn getCollisionTypeWith(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<CollisionType>>> { Self::getCollisionTypeWith(self, layerInterface) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for CollidableBaseBehavior {
    fn getName(&mut self) -> String { Self::getName(self) }
}

impl CollidableBaseBehavior {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut collidable: bool) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            layerCollisionUtil: None,
            collidable: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.layerCollisionUtil = LayerCollisionUtil::getInstance();
        this.collidable = true;
        this.collidable = collidable.clone();
        this
    }

    pub fn new(mut collidable: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(collidable))))
    }
    

    pub fn update(&mut self) {
    }
    

    pub fn getName(&mut self) -> String {
        return std::any::type_name_of_val(&self).to_string();
    }
    

    pub fn setCollidable(&mut self, mut collidable: bool) {
        self.collidable = collidable.clone();
    }
    

    pub fn isCollidable(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool {
        return self.collidable.clone();
    }
    

    // throws Exception
    pub fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut allbinaryCollidableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", ({ let java_argument = jcall!(StringMaker::new()).append({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone()).append({ let java_field = jcall!(CommonLabels::getInstance()).COLON_SEP.clone(); java_field }.clone()).append({ let java_argument = std::any::type_name_of_val(&self).to_string(); java_argument }).toString(); java_argument })));
    }
    

    pub fn isCollision(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collisionLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool {
        return jcall!(self.layerCollisionUtil).isCollision(ownerLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>), collisionLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>));
    }
    

    pub fn isCollisionInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) -> bool {
        return jcall!(self.layerCollisionUtil).isCollision(ownerLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>), (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { panic!("ClassCastException: a Rust dyn CollidableInterfaceCompositeInterface cannot be cast to AllBinaryLayer") })());
    }
    

    // throws Exception
    pub fn collideInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) {
        panic!("{:?}", format!("{}: {:?}", "Exception", ({ let java_field = jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(); java_field }.clone())));
    }
    

    pub fn paint(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
    }
    

    pub fn getCollisionTypeWith(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<CollisionType>>> {
        return { let java_field = jcall!(CollisionTypeFactory::CollisionTypeFactory::getInstance()).NONE.clone(); java_field }.clone();
    }
}
pub trait CollidableBaseBehaviorDyn: std::any::Any {
    fn as_CollidableBaseBehavior(&self) -> &CollidableBaseBehavior;
    fn as_CollidableBaseBehavior_mut(&mut self) -> &mut CollidableBaseBehavior;
    fn update(&mut self) { CollidableBaseBehavior::update(self.as_CollidableBaseBehavior_mut()) }
    fn getName(&mut self) -> String { CollidableBaseBehavior::getName(self.as_CollidableBaseBehavior_mut()) }
    fn setCollidable(&mut self, mut collidable: bool) { CollidableBaseBehavior::setCollidable(self.as_CollidableBaseBehavior_mut(), collidable) }
    fn isCollidable(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool { CollidableBaseBehavior::isCollidable(self.as_CollidableBaseBehavior_mut(), ownerLayer) }
    fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut allbinaryCollidableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) { CollidableBaseBehavior::collide(self.as_CollidableBaseBehavior_mut(), ownerLayer, allbinaryCollidableLayer) }
    fn isCollision(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collisionLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool { CollidableBaseBehavior::isCollision(self.as_CollidableBaseBehavior_mut(), ownerLayer, collisionLayer) }
    fn isCollisionInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) -> bool { CollidableBaseBehavior::isCollisionInterface(self.as_CollidableBaseBehavior_mut(), ownerLayer, collidableInterfaceCompositeInterface) }
    fn collideInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) { CollidableBaseBehavior::collideInterface(self.as_CollidableBaseBehavior_mut(), ownerLayer, collidableInterfaceCompositeInterface) }
    fn paint(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { CollidableBaseBehavior::paint(self.as_CollidableBaseBehavior_mut(), ownerLayer, graphics) }
    fn getCollisionTypeWith(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<CollisionType>>> { CollidableBaseBehavior::getCollisionTypeWith(self.as_CollidableBaseBehavior_mut(), layerInterface) }
}
impl CollidableBaseBehaviorDyn for CollidableBaseBehavior {
    fn as_CollidableBaseBehavior(&self) -> &CollidableBaseBehavior { self }
    fn as_CollidableBaseBehavior_mut(&mut self) -> &mut CollidableBaseBehavior { self }
}


