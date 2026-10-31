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
// package org.allbinary.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::identification::BasicGroupFactory::BasicGroupFactory;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::RectangleFactory::RectangleFactory;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureUtil::OpenGLFeatureUtil;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::math::PositionStrings::PositionStrings;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent;
use super::Layer::Layer;
use super::LayerInterface::LayerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

fn java_string_hash(text: &str) -> i32 {
    text.chars().fold(0i32, |hash, character| hash.wrapping_mul(31).wrapping_add(character as i32))
}

pub struct AllBinaryLayer {
    pub base: Layer,
    pub viewPositionEvent: Option<Rc<RefCell<ViewPositionEvent>>>,
    pub halfWidth: i32,
    pub halfHeight: i32,
    pub name: String,
    pub viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>,
}

thread_local! { pub static AllBinaryLayer_NULL_ALLBINARY_LAYER: Option<Rc<RefCell<dyn AllBinaryLayerDyn>>> = AllBinaryLayer::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), crate::org::allbinary::graphics::RectangleFactory::RectangleFactory_SINGLETON.with(Clone::clone).clone(), crate::org::allbinary::view::ViewPositionBase::ViewPositionBase_NULL_VIEW_POSITION.with(Clone::clone).clone()).map(|reference| reference as Rc<RefCell<dyn AllBinaryLayerDyn>>); }
thread_local! { pub static AllBinaryLayer_TYPE: String = String::from("Type: "); }
// impl LayerInterface for AllBinaryLayer with the methods of the trait

impl AllBinaryLayer {
    

    pub fn new_struct(mut name: String, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Self {
        let mut this = Self {
            base: Layer::new_struct(jcall!(rectangle).getWidth(), jcall!(rectangle).getHeight()),
            viewPositionEvent: None,
            halfWidth: 0,
            halfHeight: 0,
            name: String::new(),
            viewPosition: None,
        };
        this.viewPositionEvent = ViewPositionEvent::new(this);
        this.viewPosition = crate::org::allbinary::view::ViewPositionBase::ViewPositionBase_NULL_VIEW_POSITION.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        let mut localName: String = String::new();
        this.name = localName.clone();
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(rectangle).getPoint();
        { let java_arguments = (jcall!(point).getX(), jcall!(point).getY(), jcall!(point).getZ(),); this.setPosition(java_arguments.0, java_arguments.1, java_arguments.2) };
        { let java_arguments = ((this.getWidth() >> 1).clone(),); this.setHalfWidth(java_arguments.0) };
        { let java_arguments = ((this.getHeight() >> 1).clone(),); this.setHalfHeight(java_arguments.0) };
        this.viewPosition = viewPosition.clone();
        this
    }

    pub fn new(mut name: String, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(name, rectangle, viewPosition))));
            if false {
            localName = std::any::type_name_of_val(&this).to_string();
        } else {
            localName = jcall!(StringMaker::new()).append(name.clone()).append(jcall!(CommonSeps::getInstance()).COLON.clone()).append(format!("{:x}", jcall!(TsUtil::getInstance()).hashCode((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone())))).toString();
        }
            jcall!(this.viewPosition).setAllbinaryLayer((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    // throws Exception
    pub fn onChangeEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<ViewPositionEvent>>>) {
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn getX2(&mut self) -> i32 {
        return self.getXP() + self.getWidth();
    }
    

    pub fn getY2(&mut self) -> i32 {
        return self.getYP() + self.getHeight();
    }
    

    pub fn getZ2(&mut self) -> i32 {
        return (((self.getZP() + self.getDepth())) as i32);
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn setHalfWidth(&mut self, mut halfWidth: i32) {
        self.halfWidth = halfWidth.clone();
    }
    

    pub fn getHalfWidth(&mut self) -> i32 {
        return self.halfWidth.clone();
    }
    

    pub fn setHalfHeight(&mut self, mut halfHeight: i32) {
        self.halfHeight = halfHeight.clone();
    }
    

    pub fn getHalfHeight(&mut self) -> i32 {
        return self.halfHeight.clone();
    }
    

    pub fn getHalfDepth(&mut self) -> i32 {
        return ((self.getDepth()) as i32) / 2;
    }
    

    pub fn getViewPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> {
        return self.viewPosition.clone();
    }
    

    pub fn setViewPosition(&mut self, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) {
        self.viewPosition = viewPosition.clone();
    }
    

    pub fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<Group>>>> {
        return jcall!(BasicGroupFactory::getInstance()).NONE_ARRAY.clone();
    }
    

    pub fn implmentsTickableInterface(&mut self) -> bool {
        return false;
    }
    

    pub fn implmentsGameInputInterface(&mut self) -> bool {
        return false;
    }
    

    pub fn implmentsArtificialIntelligenceCompositeInterface(&mut self) -> bool {
        return false;
    }
    

    pub fn implmentsCollidableInterface(&mut self) -> bool {
        return false;
    }
    

    pub fn implementsThreedInterface(&mut self) -> bool {
        return jcall!(OpenGLFeatureUtil::getInstance()).isAnyThreed();
    }
    

    pub fn getType(&mut self) -> i32 {
        return -1;
    }
    

    pub fn getMultiPlayerType(&mut self) -> i32 {
        return -1;
    }
    

    pub fn toStringAppend(&mut self, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>) {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut positionStrings: Option<Rc<RefCell<PositionStrings>>> = PositionStrings::getInstance();
        let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
        jcall!(stringBuffer).append(self.getName());
        jcall!(stringBuffer).append(jcall!(commonSeps).COLON.clone());
        jcall!(stringBuffer).append(AllBinaryLayer_TYPE.with(Clone::clone).clone());
        jcall!(stringBuffer).appendint(self.getType());
        jcall!(stringBuffer).append(jcall!(commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(positionStrings).X_LABEL.clone());
        jcall!(stringBuffer).appendint(((self.getXP()) as i32));
        jcall!(stringBuffer).append(jcall!(commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(positionStrings).Y_LABEL.clone());
        jcall!(stringBuffer).appendint(((self.getYP()) as i32));
        jcall!(stringBuffer).append(jcall!(commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(positionStrings).Z_LABEL.clone());
        jcall!(stringBuffer).appendint(((self.getZP()) as i32));
        jcall!(stringBuffer).append(jcall!(commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(commonLabels).WIDTH_LABEL.clone());
        jcall!(stringBuffer).appendint(((self.getWidth()) as i32));
        jcall!(stringBuffer).append(jcall!(commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(commonLabels).HEIGHT_LABEL.clone());
        jcall!(stringBuffer).appendint(((self.getHeight()) as i32));
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        self.toStringAppend(stringBuffer.clone());
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for AllBinaryLayer {
    type Target = Layer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinaryLayerDyn: crate::org::allbinary::layer::Layer::LayerDyn {
    fn as_AllBinaryLayer(&self) -> &AllBinaryLayer;
    fn as_AllBinaryLayer_mut(&mut self) -> &mut AllBinaryLayer;
    fn onChangeEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<ViewPositionEvent>>>) { AllBinaryLayer::onChangeEvent(self.as_AllBinaryLayer_mut(), layerManagerEvent) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { AllBinaryLayer::paintThreed(self.as_AllBinaryLayer_mut(), graphics) }
    fn getX2(&mut self) -> i32 { AllBinaryLayer::getX2(self.as_AllBinaryLayer_mut()) }
    fn getY2(&mut self) -> i32 { AllBinaryLayer::getY2(self.as_AllBinaryLayer_mut()) }
    fn getZ2(&mut self) -> i32 { AllBinaryLayer::getZ2(self.as_AllBinaryLayer_mut()) }
    fn getName(&mut self) -> String { AllBinaryLayer::getName(self.as_AllBinaryLayer_mut()) }
    fn setHalfWidth(&mut self, mut halfWidth: i32) { AllBinaryLayer::setHalfWidth(self.as_AllBinaryLayer_mut(), halfWidth) }
    fn getHalfWidth(&mut self) -> i32 { AllBinaryLayer::getHalfWidth(self.as_AllBinaryLayer_mut()) }
    fn setHalfHeight(&mut self, mut halfHeight: i32) { AllBinaryLayer::setHalfHeight(self.as_AllBinaryLayer_mut(), halfHeight) }
    fn getHalfHeight(&mut self) -> i32 { AllBinaryLayer::getHalfHeight(self.as_AllBinaryLayer_mut()) }
    fn getHalfDepth(&mut self) -> i32 { AllBinaryLayer::getHalfDepth(self.as_AllBinaryLayer_mut()) }
    fn getViewPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>> { AllBinaryLayer::getViewPosition(self.as_AllBinaryLayer_mut()) }
    fn setViewPosition(&mut self, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>) { AllBinaryLayer::setViewPosition(self.as_AllBinaryLayer_mut(), viewPosition) }
    fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<Group>>>> { AllBinaryLayer::getGroupInterface(self.as_AllBinaryLayer_mut()) }
    fn implmentsTickableInterface(&mut self) -> bool { AllBinaryLayer::implmentsTickableInterface(self.as_AllBinaryLayer_mut()) }
    fn implmentsGameInputInterface(&mut self) -> bool { AllBinaryLayer::implmentsGameInputInterface(self.as_AllBinaryLayer_mut()) }
    fn implmentsArtificialIntelligenceCompositeInterface(&mut self) -> bool { AllBinaryLayer::implmentsArtificialIntelligenceCompositeInterface(self.as_AllBinaryLayer_mut()) }
    fn implmentsCollidableInterface(&mut self) -> bool { AllBinaryLayer::implmentsCollidableInterface(self.as_AllBinaryLayer_mut()) }
    fn implementsThreedInterface(&mut self) -> bool { AllBinaryLayer::implementsThreedInterface(self.as_AllBinaryLayer_mut()) }
    fn getType(&mut self) -> i32 { AllBinaryLayer::getType(self.as_AllBinaryLayer_mut()) }
    fn getMultiPlayerType(&mut self) -> i32 { AllBinaryLayer::getMultiPlayerType(self.as_AllBinaryLayer_mut()) }
    fn toString(&mut self) -> String { AllBinaryLayer::toString(self.as_AllBinaryLayer_mut()) }
}
impl AllBinaryLayerDyn for AllBinaryLayer {
    fn as_AllBinaryLayer(&self) -> &AllBinaryLayer { self }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut AllBinaryLayer { self }
}

impl crate::org::allbinary::layer::Layer::LayerDyn for AllBinaryLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base }
    fn toStringAppend(&mut self, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>) { Self::toStringAppend(self, stringBuffer) }
}


