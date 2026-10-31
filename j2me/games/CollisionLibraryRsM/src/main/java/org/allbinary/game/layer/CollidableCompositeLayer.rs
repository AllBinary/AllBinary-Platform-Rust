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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligence;
use crate::org::allbinary::ai::ArtificialIntelligenceInterface::ArtificialIntelligenceInterface;
use crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior;
use crate::org::allbinary::game::collision::CollidableBaseBehaviorFactory::CollidableBaseBehaviorFactory;
use crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface;
use crate::org::allbinary::game::collision::CollidableNeverCollideBehaviorFactory::CollidableNeverCollideBehaviorFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::RectangleFactory::RectangleFactory;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::AllBinaryGameLayer::AllBinaryGameLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollidableCompositeLayer {
    pub base: AllBinaryGameLayer,
    pub collidableInferface: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>>,
}

pub static mut CollidableCompositeLayer_NULL_COLLIDABLE_COMPOSITE_LAYER: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
impl CollidableInterfaceCompositeInterface for CollidableCompositeLayer {
    fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { Self::getCollidableInferface(self) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for CollidableCompositeLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl CollidableCompositeLayer {
    

    pub fn getNullInstance() -> Option<Rc<RefCell<dyn CollidableCompositeLayerDyn>>> {
        if match (&unsafe { CollidableCompositeLayer_NULL_COLLIDABLE_COMPOSITE_LAYER.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *CollidableCompositeLayer_NULL_COLLIDABLE_COMPOSITE_LAYER = CollidableCompositeLayer::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), crate::org::allbinary::graphics::RectangleFactory::RectangleFactory_SINGLETON.with(Clone::clone).clone(), crate::org::allbinary::view::ViewPositionBase::ViewPositionBase_NULL_VIEW_POSITION.with(Clone::clone).clone(), CollidableNeverCollideBehaviorFactory::getInstance()).map(|reference| reference as Rc<RefCell<dyn CollidableCompositeLayerDyn>>) };
        }
        return (unsafe { CollidableCompositeLayer_NULL_COLLIDABLE_COMPOSITE_LAYER.clone() }).clone() /* as CollidableCompositeLayer */;
    }
    

    pub fn new_struct(mut name: String, mut layerInfo: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>, mut collidableBaseBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehaviorFactory::CollidableBaseBehaviorFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: AllBinaryGameLayer::new_struct(name.clone(), layerInfo.clone(), viewPosition.clone()),
            collidableInferface: None,
        };
        this.collidableInferface = jcall!(CollidableNeverCollideBehaviorFactory::getInstance()).createBehavior();
        // super(...) is in the struct literal above
        if !match (&collidableBaseBehaviorFactory, &CollidableNeverCollideBehaviorFactory::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            { let java_arguments = (jcall!(collidableBaseBehaviorFactory).createBehavior(),); this.setCollidableInferface(java_arguments.0) };
        }
        this
    }

    pub fn new(mut name: String, mut layerInfo: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>, mut collidableBaseBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehaviorFactory::CollidableBaseBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, layerInfo, viewPosition, collidableBaseBehaviorFactory))))
    }
    

    pub fn getArtificialIntelligenceInterface(&mut self) -> Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>> {
        ForcedLogUtil::log(jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
        return ArtificialIntelligence::getInstance().map(|reference| reference as Rc<RefCell<dyn ArtificialIntelligenceInterface>>);
    }
    

    pub fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> {
        return self.collidableInferface.clone();
    }
    

    pub fn setCollidableInferface(&mut self, mut collidableInferface: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>>) {
        self.collidableInferface = collidableInferface.clone();
    }
    

    pub fn implmentsCollidableInterface(&mut self) -> bool {
        return true;
    }
    

    pub fn toStringAppend(&mut self, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>) {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        self.base.toStringAppend(stringBuffer.clone());
        jcall!(stringBuffer).append(jcall!(commonSeps).NEW_LINE.clone());
        jcall!(stringBuffer).append(jcall!(self.getCollidableInferface()).toString());
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        self.toStringAppend(stringBuffer.clone());
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for CollidableCompositeLayer {
    type Target = AllBinaryGameLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CollidableCompositeLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CollidableCompositeLayerDyn: crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn + crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface {
    fn as_CollidableCompositeLayer(&self) -> &CollidableCompositeLayer;
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut CollidableCompositeLayer;
    fn getArtificialIntelligenceInterface(&mut self) -> Option<Rc<RefCell<dyn ArtificialIntelligenceInterface>>> { CollidableCompositeLayer::getArtificialIntelligenceInterface(self.as_CollidableCompositeLayer_mut()) }
    fn setCollidableInferface(&mut self, mut collidableInferface: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>>) { CollidableCompositeLayer::setCollidableInferface(self.as_CollidableCompositeLayer_mut(), collidableInferface) }
}
impl CollidableCompositeLayerDyn for CollidableCompositeLayer {
    fn as_CollidableCompositeLayer(&self) -> &CollidableCompositeLayer { self }
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut CollidableCompositeLayer { self }
}

impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for CollidableCompositeLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for CollidableCompositeLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn implmentsCollidableInterface(&mut self) -> bool { Self::implmentsCollidableInterface(self) }
    fn toString(&mut self) -> String { Self::toString(self) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for CollidableCompositeLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base }
    fn toStringAppend(&mut self, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>) { Self::toStringAppend(self, stringBuffer) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for CollidableCompositeLayer {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::set(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
}


