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
// package org.allbinary.graphics.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::canvas::Processor::Processor;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use super::ScrollSelectionForm::ScrollSelectionForm;
use super::ItemIndexDx::ItemIndexDx;
use super::ItemPaintableFactory::ItemPaintableFactory;
use super::FormTypeFactory::FormTypeFactory;
use super::VerticalFormProcessor::VerticalFormProcessor;
use super::VerticalItemIndexDx::VerticalItemIndexDx;
use super::HorizontalFormProcessor::HorizontalFormProcessor;
use super::HorizontalItemIndexDx::HorizontalItemIndexDx;
use super::TempHorizontalFormProcessor::TempHorizontalFormProcessor;
use super::TempHorizontalItemIndexDx::TempHorizontalItemIndexDx;
use super::FormType::FormType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScrollCurrentSelectionForm {
    pub base: ScrollSelectionForm,
    pub moveForSmallScreen: bool,
    pub updateMyFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub processor: Option<Rc<RefCell<dyn crate::org::allbinary::canvas::Processor::ProcessorDyn>>>,
    pub preItemIndexDx: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDxDyn>>>,
    pub dx: i32,
    pub dy: i32,
    pub maxWidth: i32,
}

impl UpdateMyFontInterface for ScrollCurrentSelectionForm {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl ScrollCurrentSelectionForm {
    

    pub fn new_struct(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut formPaintableFactory: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintableFactory::ItemPaintableFactoryDyn>>>, mut border: i32, mut adjustedExtraBorder: i32, mut moveForSmallScreen: bool, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ScrollSelectionForm::new_struct(title.clone(), items.clone(), formPaintableFactory.clone(), border.clone(), adjustedExtraBorder.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
            moveForSmallScreen: false,
            updateMyFontProcessor: None,
            myFontProcessor: None,
            processor: None,
            preItemIndexDx: None,
            dx: 0,
            dy: 0,
            maxWidth: 0,
        };
        this.updateMyFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.myFontProcessor = this.updateMyFontProcessor.clone();
        this.processor = Processor::getInstance();
        this.preItemIndexDx = ItemIndexDx::getInstance();
        this.maxWidth = 0;
        // super(...) is in the struct literal above
        this.moveForSmallScreen = moveForSmallScreen.clone();
        this
    }

    pub fn new(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut formPaintableFactory: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintableFactory::ItemPaintableFactoryDyn>>>, mut border: i32, mut adjustedExtraBorder: i32, mut moveForSmallScreen: bool, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(title, items, formPaintableFactory, border, adjustedExtraBorder, moveForSmallScreen, backgroundBasicColor, foregroundBasicColor))))
    }
    

    // throws Exception
    pub fn init(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut formType: Option<Rc<RefCell<FormType>>>) {
        self.base.init(rectangle.clone(), formType.clone());
        let mut formTypeFactory: Option<Rc<RefCell<FormTypeFactory>>> = FormTypeFactory::getInstance();
        if match (&formType, &jcall!(formTypeFactory).TEMP_HORIZONTAL_FORM) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.processor = TempHorizontalFormProcessor::new(None /* Java this: a &mut self method has no Rc of itself */);
            self.preItemIndexDx = TempHorizontalItemIndexDx::new(None /* Java this: a &mut self method has no Rc of itself */);
        } else if match (&formType, &jcall!(formTypeFactory).HORIZONTAL_FORM) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.processor = HorizontalFormProcessor::new(None /* Java this: a &mut self method has no Rc of itself */);
            self.preItemIndexDx = HorizontalItemIndexDx::new(None /* Java this: a &mut self method has no Rc of itself */);
        } else if match (&formType, &jcall!(formTypeFactory).VERTICAL_CENTER_FORM) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.processor = VerticalFormProcessor::new(None /* Java this: a &mut self method has no Rc of itself */);
            self.preItemIndexDx = VerticalItemIndexDx::new(None /* Java this: a &mut self method has no Rc of itself */);
        } else if match (&formType, &jcall!(formTypeFactory).NULL_FORM_TYPE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
        } else {
            jcall!(self.logUtil).putF(jcall!(formTypeFactory).UNK.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).INIT.clone());
        }
        self.myFontProcessor = self.updateMyFontProcessor.clone();
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn processTempHorizontalForm(&mut self) {
        self.dx = self.x - 30 + (jcall!(self.rectangle).getWidth() >> 1);
        self.dy = self.y.clone();
    }
    

    pub fn processHorizontalForm(&mut self) {
        let mut size: i32 = self.size();
        let mut totalWidth: i32 = 0;
        let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> = None;
        for index in (0..size) {
            item = self.get(index.clone());
            totalWidth += jcall!(item).getMinimumWidth() + self.border;
        }
        self.dx = self.x + (jcall!(self.rectangle).getWidth() >> 1) - (totalWidth >> 1);
        self.dy = self.y + (jcall!(self.rectangle).getHeight() >> 1);
        if self.moveForSmallScreen {
            let mut maxTitleHeight: i32 = 175;
            if self.dy < maxTitleHeight {
                self.dy = maxTitleHeight.clone();
            }
        }
    }
    

    pub fn processVerticalForm(&mut self) {
        let mut totalHeight: i32 = 0;
        let mut size: i32 = self.size();
        let mut item2: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> = None;
        for index in (0..size) {
            item2 = self.get(index.clone());
            if self.maxWidth < jcall!(item2).getMinimumWidth() {
                self.maxWidth = jcall!(item2).getMinimumWidth();
            }
            totalHeight += jcall!(item2).getMinimumHeight() + self.border;
        }
        self.dx = ((jcall!(self.rectangle).getWidth() - self.maxWidth) / 2);
        if self.size() > 0 {
            self.dy = self.y + ((jcall!(self.rectangle).getHeight() - totalHeight) / 2);
            if self.moveForSmallScreen {
                let mut maxTitleHeight: i32 = 175;
                if self.dy < maxTitleHeight {
                    self.dy = maxTitleHeight.clone();
                }
            }
        } else {
            self.dy = self.y.clone();
        }
    }
    

    pub fn getXTempHorizontalForm(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
        return self.getDiffX(item.clone());
    }
    

    pub fn getXHorizontalForm(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
        return 0;
    }
    

    pub fn getXVerticalForm(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
        return self.getDiffX(item.clone()) + self.halfBorder;
    }
    

    pub fn getDiffX(&mut self, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 {
        return ((self.maxWidth - jcall!(item).getMinimumWidth()) >> 1);
    }
    

    pub fn getDx(&mut self) -> i32 {
        return self.dx.clone();
    }
    

    pub fn getDy(&mut self) -> i32 {
        return self.dy.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.processor).process();
            let mut delta: i32 = 0;
            let mut deltaX: i32 = self.getDx();
            let mut deltaY: i32 = self.getDy();
            let mut size: i32 = self.size();
            let mut formTypeFactory: Option<Rc<RefCell<FormTypeFactory>>> = FormTypeFactory::getInstance();
            let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> = None;
            for index in (0..size) {
                item = self.get(index.clone());
                let mut diffX: i32 = jcall!(self.preItemIndexDx).getDx(index.clone(), item.clone(), self.dx.clone(), self.dy.clone());
                if index == self.getSelectedIndex() {
                    delta = self.paintItem(graphics.clone(), index.clone(), item.clone(), deltaX + diffX, deltaY.clone());
                    jcall!(self.paintable).paint(graphics.clone(), index.clone(), deltaX + diffX, deltaY.clone());
                } else {
                    delta = self.paintUnselectedItem(graphics.clone(), index.clone(), item.clone(), deltaX + diffX, deltaY.clone());
                    jcall!(self.paintable).paint(graphics.clone(), index.clone(), deltaX + diffX, deltaY.clone());
                }
                if match (&self.formType, &jcall!(formTypeFactory).TEMP_HORIZONTAL_FORM) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                } else if match (&self.formType, &jcall!(formTypeFactory).HORIZONTAL_FORM) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    deltaX = delta.clone();
                } else if match (&self.formType, &jcall!(formTypeFactory).VERTICAL_CENTER_FORM) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    deltaY = delta.clone();
                } else {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(formTypeFactory).UNK.clone())));
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.canvasStrings).PAINT.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for ScrollCurrentSelectionForm {
    type Target = ScrollSelectionForm;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ScrollCurrentSelectionForm {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ScrollCurrentSelectionFormDyn: crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn {
    fn as_ScrollCurrentSelectionForm(&self) -> &ScrollCurrentSelectionForm;
    fn as_ScrollCurrentSelectionForm_mut(&mut self) -> &mut ScrollCurrentSelectionForm;
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { ScrollCurrentSelectionForm::updateMeasurement(self.as_ScrollCurrentSelectionForm_mut(), graphics) }
    fn processTempHorizontalForm(&mut self) { ScrollCurrentSelectionForm::processTempHorizontalForm(self.as_ScrollCurrentSelectionForm_mut()) }
    fn processHorizontalForm(&mut self) { ScrollCurrentSelectionForm::processHorizontalForm(self.as_ScrollCurrentSelectionForm_mut()) }
    fn processVerticalForm(&mut self) { ScrollCurrentSelectionForm::processVerticalForm(self.as_ScrollCurrentSelectionForm_mut()) }
    fn getXTempHorizontalForm(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { ScrollCurrentSelectionForm::getXTempHorizontalForm(self.as_ScrollCurrentSelectionForm_mut(), index, item, dx, dy) }
    fn getXHorizontalForm(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { ScrollCurrentSelectionForm::getXHorizontalForm(self.as_ScrollCurrentSelectionForm_mut(), index, item, dx, dy) }
    fn getXVerticalForm(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { ScrollCurrentSelectionForm::getXVerticalForm(self.as_ScrollCurrentSelectionForm_mut(), index, item, dx, dy) }
}
impl ScrollCurrentSelectionFormDyn for ScrollCurrentSelectionForm {
    fn as_ScrollCurrentSelectionForm(&self) -> &ScrollCurrentSelectionForm { self }
    fn as_ScrollCurrentSelectionForm_mut(&mut self) -> &mut ScrollCurrentSelectionForm { self }
}

impl crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn for ScrollCurrentSelectionForm {
    fn as_PaintableForm(&self) -> &crate::org::allbinary::graphics::form::PaintableForm::PaintableForm { &self.base.base }
    fn as_PaintableForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::PaintableForm::PaintableForm { &mut self.base.base }
    fn init(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut formType: Option<Rc<RefCell<FormType>>>) { Self::init(self, rectangle, formType) }
}
impl crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn for ScrollCurrentSelectionForm {
    fn as_ScrollSelectionForm(&self) -> &crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm { &self.base }
    fn as_ScrollSelectionForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm { &mut self.base }
    fn getDiffX(&mut self, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 { Self::getDiffX(self, item) }
    fn getDx(&mut self) -> i32 { Self::getDx(self) }
    fn getDy(&mut self) -> i32 { Self::getDy(self) }
}
impl crate::org::allbinary::graphics::form::ABCustomForm::ABCustomFormDyn for ScrollCurrentSelectionForm {
    fn as_ABCustomForm(&self) -> &crate::org::allbinary::graphics::form::ABCustomForm::ABCustomForm { &self.base.base.base }
    fn as_ABCustomForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ABCustomForm::ABCustomForm { &mut self.base.base.base }
    fn append(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 { crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm::append(crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn::as_ScrollSelectionForm_mut(self), p1) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::form::PaintableForm::PaintableForm::getWidth(crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn::as_PaintableForm_mut(self)) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}
impl crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayableDyn for ScrollCurrentSelectionForm {
    fn as_CustomDisplayable(&self) -> &crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &self.base.base.base.base.base }
    fn as_CustomDisplayable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::graphics::form::CustomScreen::CustomScreenDyn for ScrollCurrentSelectionForm {
    fn as_CustomScreen(&self) -> &crate::org::allbinary::graphics::form::CustomScreen::CustomScreen { &self.base.base.base.base }
    fn as_CustomScreen_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::CustomScreen::CustomScreen { &mut self.base.base.base.base }
}


pub struct TempHorizontalFormProcessor {
    pub base: Processor,
    pub scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>,
}


impl TempHorizontalFormProcessor {
        

        pub fn new_struct(mut scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>) -> Self {
            let mut this = Self {
            base: Processor::new_struct(),
            scrollCurrentSelectionForm: None,
            };
            this.scrollCurrentSelectionForm = scrollCurrentSelectionForm.clone();
            this
        }

        pub fn new(mut scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(scrollCurrentSelectionForm))))
        }
        

        // throws Exception
        pub fn process(&mut self) {
            jcall!(self.scrollCurrentSelectionForm).processTempHorizontalForm();
            jcall!(self.scrollCurrentSelectionForm).as_ScrollCurrentSelectionForm_mut().processor = Processor::getInstance();
        }
}
impl std::ops::Deref for TempHorizontalFormProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TempHorizontalFormProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for TempHorizontalFormProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self }
    fn process(&mut self) { Self::process(self) }
}


pub struct HorizontalFormProcessor {
    pub base: Processor,
    pub scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>,
}


impl HorizontalFormProcessor {
        

        pub fn new_struct(mut scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>) -> Self {
            let mut this = Self {
            base: Processor::new_struct(),
            scrollCurrentSelectionForm: None,
            };
            this.scrollCurrentSelectionForm = scrollCurrentSelectionForm.clone();
            this
        }

        pub fn new(mut scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(scrollCurrentSelectionForm))))
        }
        

        // throws Exception
        pub fn process(&mut self) {
            jcall!(self.scrollCurrentSelectionForm).processHorizontalForm();
            jcall!(self.scrollCurrentSelectionForm).as_ScrollCurrentSelectionForm_mut().processor = Processor::getInstance();
        }
}
impl std::ops::Deref for HorizontalFormProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HorizontalFormProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for HorizontalFormProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self }
    fn process(&mut self) { Self::process(self) }
}


pub struct VerticalFormProcessor {
    pub base: Processor,
    pub scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>,
}


impl VerticalFormProcessor {
        

        pub fn new_struct(mut scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>) -> Self {
            let mut this = Self {
            base: Processor::new_struct(),
            scrollCurrentSelectionForm: None,
            };
            this.scrollCurrentSelectionForm = scrollCurrentSelectionForm.clone();
            this
        }

        pub fn new(mut scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(scrollCurrentSelectionForm))))
        }
        

        // throws Exception
        pub fn process(&mut self) {
            jcall!(self.scrollCurrentSelectionForm).processVerticalForm();
            jcall!(self.scrollCurrentSelectionForm).as_ScrollCurrentSelectionForm_mut().processor = Processor::getInstance();
        }
}
impl std::ops::Deref for VerticalFormProcessor {
    type Target = Processor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for VerticalFormProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::canvas::Processor::ProcessorDyn for VerticalFormProcessor {
    fn as_Processor(&self) -> &crate::org::allbinary::canvas::Processor::Processor { &self }
    fn as_Processor_mut(&mut self) -> &mut crate::org::allbinary::canvas::Processor::Processor { &mut self }
    fn process(&mut self) { Self::process(self) }
}


pub struct TempHorizontalItemIndexDx {
    pub base: ItemIndexDx,
    pub scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>,
}


impl TempHorizontalItemIndexDx {
        

        pub fn new_struct(mut scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>) -> Self {
            let mut this = Self {
            base: ItemIndexDx::new_struct(),
            scrollCurrentSelectionForm: None,
            };
            this.scrollCurrentSelectionForm = scrollCurrentSelectionForm.clone();
            this
        }

        pub fn new(mut scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(scrollCurrentSelectionForm))))
        }
        

        // throws Exception
        pub fn getDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
            return jcall!(self.scrollCurrentSelectionForm).getXTempHorizontalForm(index.clone(), item.clone(), dx.clone(), dy.clone());
        }
}
impl std::ops::Deref for TempHorizontalItemIndexDx {
    type Target = ItemIndexDx;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TempHorizontalItemIndexDx {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDxDyn for TempHorizontalItemIndexDx {
    fn as_ItemIndexDx(&self) -> &crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDx { &self }
    fn as_ItemIndexDx_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDx { &mut self }
    fn getDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { Self::getDx(self, index, item, dx, dy) }
}


pub struct HorizontalItemIndexDx {
    pub base: ItemIndexDx,
    pub scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>,
}


impl HorizontalItemIndexDx {
        

        pub fn new_struct(mut scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>) -> Self {
            let mut this = Self {
            base: ItemIndexDx::new_struct(),
            scrollCurrentSelectionForm: None,
            };
            this.scrollCurrentSelectionForm = scrollCurrentSelectionForm.clone();
            this
        }

        pub fn new(mut scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(scrollCurrentSelectionForm))))
        }
        

        // throws Exception
        pub fn getDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
            return jcall!(self.scrollCurrentSelectionForm).getXHorizontalForm(index.clone(), item.clone(), dx.clone(), dy.clone());
        }
}
impl std::ops::Deref for HorizontalItemIndexDx {
    type Target = ItemIndexDx;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HorizontalItemIndexDx {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDxDyn for HorizontalItemIndexDx {
    fn as_ItemIndexDx(&self) -> &crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDx { &self }
    fn as_ItemIndexDx_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDx { &mut self }
    fn getDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { Self::getDx(self, index, item, dx, dy) }
}


pub struct VerticalItemIndexDx {
    pub base: ItemIndexDx,
    pub scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>,
}


impl VerticalItemIndexDx {
        

        pub fn new_struct(mut scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>) -> Self {
            let mut this = Self {
            base: ItemIndexDx::new_struct(),
            scrollCurrentSelectionForm: None,
            };
            this.scrollCurrentSelectionForm = scrollCurrentSelectionForm.clone();
            this
        }

        pub fn new(mut scrollCurrentSelectionForm: Option<Rc<RefCell<dyn ScrollCurrentSelectionFormDyn>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(scrollCurrentSelectionForm))))
        }
        

        // throws Exception
        pub fn getDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
            return jcall!(self.scrollCurrentSelectionForm).getXVerticalForm(index.clone(), item.clone(), dx.clone(), dy.clone());
        }
}
impl std::ops::Deref for VerticalItemIndexDx {
    type Target = ItemIndexDx;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for VerticalItemIndexDx {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDxDyn for VerticalItemIndexDx {
    fn as_ItemIndexDx(&self) -> &crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDx { &self }
    fn as_ItemIndexDx_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDx { &mut self }
    fn getDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { Self::getDx(self, index, item, dx, dy) }
}


