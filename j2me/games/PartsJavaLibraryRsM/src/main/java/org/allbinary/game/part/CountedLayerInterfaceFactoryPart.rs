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
// package org.allbinary.game.part
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactory;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::math::PrimitiveLongUtil::PrimitiveLongUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::java::lang::Exception::*;
use super::PartInterface::PartInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CountedLayerInterfaceFactoryPart {
    pub updateMyFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub total: i32,
    pub totalString: Vec<char>,
    pub xOffset: i32,
    pub countedPickedUpLayerInterfaceFactory: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactoryDyn>>>,
    pub primitiveLongUtil: Option<Rc<RefCell<PrimitiveLongUtil>>>,
}

pub static CountedLayerInterfaceFactoryPart_NULL_COUNTED_LAYER_INTERFACE_FACTORY: crate::JavaStatic<Option<Rc<RefCell<CountedLayerInterfaceFactoryPart>>>> = crate::JavaStatic::new(|| CountedLayerInterfaceFactoryPart::new(0, crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactory_NULL_COUNTED_PICKUP_LAYER_FACTORY.with(Clone::clone).clone()));
impl PartInterface for CountedLayerInterfaceFactoryPart {
}
impl crate::org::allbinary::animation::AnimationInterfaceCompositeInterface::AnimationInterfaceCompositeInterface for CountedLayerInterfaceFactoryPart {
    fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getAnimationInterface(self) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for CountedLayerInterfaceFactoryPart {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paintThreed(self, graphics) }
}
impl UpdateMyFontInterface for CountedLayerInterfaceFactoryPart {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}

impl CountedLayerInterfaceFactoryPart {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut total: i32, mut countedPickedUpLayerInterfaceFactory: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactoryDyn>>>) -> Self {
        let mut this = Self {
            updateMyFontProcessor: None,
            myFontProcessor: None,
            animationInterface: None,
            total: 0,
            totalString: Vec::new(),
            xOffset: 0,
            countedPickedUpLayerInterfaceFactory: None,
            primitiveLongUtil: None,
        };
        this.myFontProcessor = this.updateMyFontProcessor.clone();
        this.animationInterface = jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0);
        this.totalString = { let java_field = jcall!(NullUtil::getInstance()).NULL_CHAR_ARRAY.clone(); java_field }.clone();
        this.countedPickedUpLayerInterfaceFactory = crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactory_NULL_COUNTED_PICKUP_LAYER_FACTORY.with(Clone::clone).clone();
        this.primitiveLongUtil = { let java_value = (PrimitiveLongUtil::createPowerOfTen(1000)).clone(); java_value };
        this.init(total.clone(), countedPickedUpLayerInterfaceFactory.clone());
        this
    }

    pub fn new(mut total: i32, mut countedPickedUpLayerInterfaceFactory: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(total.clone(), countedPickedUpLayerInterfaceFactory.clone()))));
        jcall!(this).updateMyFontProcessor = UpdateMyFontProcessor::new(this.clone().map(|reference| reference as Rc<RefCell<dyn UpdateMyFontInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        { let java_arguments = (jcall!(font).charsWidth(self.totalString.clone(), 0, jcall!(self.primitiveLongUtil).getCurrentTotalDigits()) + (jcall!(font).getSize() >> 1),); self.setXOffset(java_arguments.0) };
        self.myFontProcessor = { let java_value = (MyFontProcessor::getInstance()).clone(); java_value };
    }
    

    pub fn init(&mut self, mut total: i32, mut countedPickedUpLayerInterfaceFactory: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactoryDyn>>>) {
        self.setCountedPickedUpLayerInterfaceFactory(countedPickedUpLayerInterfaceFactory.clone());
        self.total = total.clone();
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut hashtable: Option<Rc<RefCell<ABHashtable<crate::JavaObject, crate::JavaObject>>>>, mut x: i32, mut y: i32, mut z: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        if self.total > 0 {
            { let java_arguments = (self.total - 1,); self.setTotal(java_arguments.0) };
            return jcall!(self.getCountedPickedUpLayerInterfaceFactory()).getNextInstance(hashtable.clone(), x.clone(), y.clone(), z.clone());
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No more left. Could use a listener to automatically remove"))));
        }
    }
    

    pub fn getTotal(&mut self) -> i32 {
        return self.total.clone();
    }
    

    pub fn setTotal(&mut self, mut total: i32) {
        self.total = total.clone();
        self.totalString = { let java_value = (jcall!(self.primitiveLongUtil).getCharArray(total.clone())).clone(); java_value };
        self.myFontProcessor = self.updateMyFontProcessor.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
    }
    

    pub fn getAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.animationInterface.clone();
    }
    

    pub fn setAnimationInterface(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) {
        self.animationInterface = animationInterface.clone();
    }
    

    pub fn getXOffset(&mut self) -> i32 {
        return self.xOffset.clone();
    }
    

    pub fn setXOffset(&mut self, mut totalStringWidth: i32) {
        self.xOffset = totalStringWidth.clone();
    }
    

    pub fn getTotalString(&mut self) -> Vec<char> {
        return self.totalString.clone();
    }
    

    pub fn getCountedPickedUpLayerInterfaceFactory(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactoryDyn>>> {
        return self.countedPickedUpLayerInterfaceFactory.clone();
    }
    

    pub fn setCountedPickedUpLayerInterfaceFactory(&mut self, mut countedPickedUpLayerInterfaceFactory: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::pickup::CountedPickedUpLayerInterfaceFactory::CountedPickedUpLayerInterfaceFactoryDyn>>>) {
        self.countedPickedUpLayerInterfaceFactory = countedPickedUpLayerInterfaceFactory.clone();
    }
}

