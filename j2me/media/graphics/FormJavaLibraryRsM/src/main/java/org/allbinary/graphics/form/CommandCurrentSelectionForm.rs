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
// use crate::javax::microedition::lcdui::Command::Command;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::animation::vector::RectangleAdjustedAnimation::RectangleAdjustedAnimation;
use crate::org::allbinary::animation::vector::RectangleFilledAdjustedAnimation::RectangleFilledAdjustedAnimation;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::form::item::CommandTextItem::CommandTextItem;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use super::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CommandCurrentSelectionForm {
    pub base: ScrollCurrentSelectionForm,
    pub selectedAnimationArray: Vec<Option<Rc<RefCell<Animation>>>>,
    pub unSelectedAnimationArray: Vec<Option<Rc<RefCell<Animation>>>>,
}

impl UpdateMyFontInterface for CommandCurrentSelectionForm {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl CommandCurrentSelectionForm {
    

    pub fn new_struct(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut border: i32, mut moveForSmallScreen: bool, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ScrollCurrentSelectionForm::new_struct(title.clone(), items.clone(), ItemPaintableFactory::ItemPaintableFactory::getInstance(), border.clone(), -3, moveForSmallScreen.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
            selectedAnimationArray: Vec::new(),
            unSelectedAnimationArray: Vec::new(),
        };
        this.selectedAnimationArray = vec![None; (16) as usize];
        this.unSelectedAnimationArray = vec![None; (16) as usize];
        // super(...) is in the struct literal above
        this.initAnimations();
        this.addAll(items.clone());
        this
    }

    pub fn new(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut border: i32, mut moveForSmallScreen: bool, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(title, items, border, moveForSmallScreen, backgroundBasicColor, foregroundBasicColor))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        { let java_arguments = (graphics.clone(), self.getAllitems(),); self.updateAll(java_arguments.0, java_arguments.1) };
        self.base.updateMeasurement(graphics.clone());
    }
    

    pub fn initAnimations(&mut self) {
        let mut nullAnimation: Option<Rc<RefCell<Animation>>> = jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0);
        {
            let mut index: i32 = (self.selectedAnimationArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            self.selectedAnimationArray[(index) as usize] = nullAnimation.clone();
            }
        }
        {
            let mut index: i32 = (self.unSelectedAnimationArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            self.unSelectedAnimationArray[(index) as usize] = nullAnimation.clone();
            }
        }
    }
    

    pub fn addAll(&mut self, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>) {
        {
            let mut index: i32 = (items.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            self.addAt(index.clone(), items[(index) as usize].clone());
            }
        }
    }
    

    pub fn addAt(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) {
        let mut basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>> = BasicColorFactory::getInstance();
        let mut buttonColor: Option<Rc<RefCell<BasicColor>>> = jcall!(basicColorFactory).TRANSPARENT_GREY.clone();
        let mut selectedButtonColor: Option<Rc<RefCell<BasicColor>>> = jcall!(basicColorFactory).TRANSPARENT_RED.clone();
        let mut width: i32 = jcall!(item).getMinimumWidth();
        let mut height: i32 = jcall!(item).getMinimumHeight();
        let mut adjustedBorder: i32 = 3;
        let mut offset: i32 = -(self.halfBorder + adjustedBorder);
        let mut rectWidth: i32 = width + self.border - adjustedBorder;
        let mut rectHeight: i32 = height + self.border - adjustedBorder;
        if J2MEUtil::isJ2ME() || J2MEUtil::isHTML() {
            self.selectedAnimationArray[(index) as usize] = RectangleAdjustedAnimation::new(rectWidth.clone(), rectHeight.clone(), offset.clone(), offset.clone(), selectedButtonColor.clone());
        } else {
            self.selectedAnimationArray[(index) as usize] = RectangleFilledAdjustedAnimation::new(rectWidth.clone(), rectHeight.clone(), offset.clone(), offset.clone(), selectedButtonColor.clone());
        }
        adjustedBorder = 4;
        if J2MEUtil::isJ2ME() || J2MEUtil::isHTML() {
            self.unSelectedAnimationArray[(index) as usize] = RectangleAdjustedAnimation::new(rectWidth.clone(), rectHeight.clone(), offset.clone(), offset.clone(), buttonColor.clone());
        } else {
            self.unSelectedAnimationArray[(index) as usize] = RectangleFilledAdjustedAnimation::new(rectWidth.clone(), rectHeight.clone(), offset.clone(), offset.clone(), buttonColor.clone());
        }
    }
    

    pub fn updateAll(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>) {
        {
            let mut index: i32 = (items.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(items[(index) as usize]).preMeasurement(graphics.clone());
            self.updateAt(index.clone(), items[(index) as usize].clone());
            }
        }
    }
    

    pub fn updateAt(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) {
        let mut basicColorFactory: Option<Rc<RefCell<BasicColorFactory>>> = BasicColorFactory::getInstance();
        let mut buttonColor: Option<Rc<RefCell<BasicColor>>> = jcall!(basicColorFactory).TRANSPARENT_GREY.clone();
        let mut selectedButtonColor: Option<Rc<RefCell<BasicColor>>> = jcall!(basicColorFactory).TRANSPARENT_RED.clone();
        let mut width: i32 = jcall!(item).getMinimumWidth();
        let mut height: i32 = jcall!(item).getMinimumHeight();
        let mut adjustedBorder: i32 = 3;
        let mut offset: i32 = -(self.halfBorder + adjustedBorder);
        let mut rectWidth: i32 = width + self.border - adjustedBorder;
        let mut rectHeight: i32 = height + self.border - adjustedBorder;
        if J2MEUtil::isJ2ME() || J2MEUtil::isHTML() {
            let mut rectangleAdjustedAnimation: Option<Rc<RefCell<RectangleAdjustedAnimation>>> = (self.selectedAnimationArray[(index) as usize]).clone() /* as RectangleAdjustedAnimation */;
            jcall!(rectangleAdjustedAnimation).setWidth(rectWidth.clone());
            jcall!(rectangleAdjustedAnimation).setHeight(rectHeight.clone());
            jcall!(rectangleAdjustedAnimation).setOffsetX(offset.clone());
            jcall!(rectangleAdjustedAnimation).setOffsetY(offset.clone());
            jcall!(rectangleAdjustedAnimation).setBasicColorP(selectedButtonColor.clone());
        } else {
            let mut rectangleAdjustedAnimation: Option<Rc<RefCell<RectangleFilledAdjustedAnimation>>> = (self.selectedAnimationArray[(index) as usize]).clone() /* as RectangleFilledAdjustedAnimation */;
            jcall!(rectangleAdjustedAnimation).setWidth(rectWidth.clone());
            jcall!(rectangleAdjustedAnimation).setHeight(rectHeight.clone());
            jcall!(rectangleAdjustedAnimation).setOffsetX(offset.clone());
            jcall!(rectangleAdjustedAnimation).setOffsetY(offset.clone());
            jcall!(rectangleAdjustedAnimation).setBasicColorP(selectedButtonColor.clone());
        }
        adjustedBorder = 4;
        if J2MEUtil::isJ2ME() || J2MEUtil::isHTML() {
            let mut rectangleAdjustedAnimation: Option<Rc<RefCell<RectangleAdjustedAnimation>>> = (self.unSelectedAnimationArray[(index) as usize]).clone() /* as RectangleAdjustedAnimation */;
            jcall!(rectangleAdjustedAnimation).setWidth(rectWidth.clone());
            jcall!(rectangleAdjustedAnimation).setHeight(rectHeight.clone());
            jcall!(rectangleAdjustedAnimation).setOffsetX(offset.clone());
            jcall!(rectangleAdjustedAnimation).setOffsetY(offset.clone());
            jcall!(rectangleAdjustedAnimation).setBasicColorP(buttonColor.clone());
        } else {
            let mut rectangleAdjustedAnimation: Option<Rc<RefCell<RectangleFilledAdjustedAnimation>>> = (self.unSelectedAnimationArray[(index) as usize]).clone() /* as RectangleFilledAdjustedAnimation */;
            jcall!(rectangleAdjustedAnimation).setWidth(rectWidth.clone());
            jcall!(rectangleAdjustedAnimation).setHeight(rectHeight.clone());
            jcall!(rectangleAdjustedAnimation).setOffsetX(offset.clone());
            jcall!(rectangleAdjustedAnimation).setOffsetY(offset.clone());
            jcall!(rectangleAdjustedAnimation).setBasicColorP(buttonColor.clone());
        }
    }
    

    pub fn getSelectedCommand(&mut self) -> Option<Rc<RefCell<Command>>> {
        let mut index: i32 = self.base.getSelectedIndex();
        let mut commandTextItem: Option<Rc<RefCell<CommandTextItem>>> = (self.get(index.clone())).clone() /* as CommandTextItem */;
        return jcall!(commandTextItem).getCommand();
    }
    

    pub fn append(&mut self, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 {
        let mut result: i32 = self.base.append(item.clone());
        self.addAt(result.clone(), item.clone());
        return result.clone();
    }
    

    pub fn delete(&mut self, mut itemNum: i32) {
        self.base.delete(itemNum.clone());
    }
    

    pub fn deleteAll(&mut self) {
        self.initAnimations();
        self.base.deleteAll();
    }
    

    pub fn insert(&mut self, mut itemNum: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) {
        self.base.insert(itemNum.clone(), item.clone());
    }
    

    pub fn set(&mut self, mut itemNum: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) {
        self.base.set(itemNum.clone(), item.clone());
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        self.base.paint(graphics.clone());
    }
    

    // throws Exception
    pub fn paintItem(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut x: i32, mut y: i32) -> i32 {
        jcall!(self.selectedAnimationArray[(index) as usize]).paintXY(graphics.clone(), x.clone(), y.clone());
        return self.base.paintItem(graphics.clone(), index.clone(), item.clone(), x.clone(), y.clone());
    }
    

    // throws Exception
    pub fn paintUnselectedItem(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut x: i32, mut y: i32) -> i32 {
        jcall!(self.unSelectedAnimationArray[(index) as usize]).paintXY(graphics.clone(), x.clone(), y.clone());
        return self.base.paintUnselectedItem(graphics.clone(), index.clone(), item.clone(), x.clone(), y.clone());
    }
}
impl std::ops::Deref for CommandCurrentSelectionForm {
    type Target = ScrollCurrentSelectionForm;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CommandCurrentSelectionForm {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn for CommandCurrentSelectionForm {
    fn as_PaintableForm(&self) -> &crate::org::allbinary::graphics::form::PaintableForm::PaintableForm { &self.base.base.base }
    fn as_PaintableForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::PaintableForm::PaintableForm { &mut self.base.base.base }
    fn init(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::Rectangle::Rectangle>>>, p2: Option<Rc<RefCell<crate::org::allbinary::graphics::form::FormType::FormType>>>) { crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm::init(crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn::as_ScrollCurrentSelectionForm_mut(self), p1, p2) }
}
impl crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn for CommandCurrentSelectionForm {
    fn as_ScrollCurrentSelectionForm(&self) -> &crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm { &self.base }
    fn as_ScrollCurrentSelectionForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm { &mut self.base }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn for CommandCurrentSelectionForm {
    fn as_ScrollSelectionForm(&self) -> &crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm { &self.base.base }
    fn as_ScrollSelectionForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm { &mut self.base.base }
    fn paintItem(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut x: i32, mut y: i32) -> i32 { Self::paintItem(self, graphics, index, item, x, y) }
    fn paintUnselectedItem(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut x: i32, mut y: i32) -> i32 { Self::paintUnselectedItem(self, graphics, index, item, x, y) }
    fn getDiffX(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 { crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm::getDiffX(crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn::as_ScrollCurrentSelectionForm_mut(self), p1) }
    fn getDx(&mut self) -> i32 { crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm::getDx(crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn::as_ScrollCurrentSelectionForm_mut(self)) }
    fn getDy(&mut self) -> i32 { crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionForm::getDy(crate::org::allbinary::graphics::form::ScrollCurrentSelectionForm::ScrollCurrentSelectionFormDyn::as_ScrollCurrentSelectionForm_mut(self)) }
}
impl crate::org::allbinary::graphics::form::ABCustomForm::ABCustomFormDyn for CommandCurrentSelectionForm {
    fn as_ABCustomForm(&self) -> &crate::org::allbinary::graphics::form::ABCustomForm::ABCustomForm { &self.base.base.base.base }
    fn as_ABCustomForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ABCustomForm::ABCustomForm { &mut self.base.base.base.base }
    fn deleteAll(&mut self) { Self::deleteAll(self) }
    fn delete(&mut self, mut itemNum: i32) { Self::delete(self, itemNum) }
    fn append(&mut self, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 { Self::append(self, item) }
    fn insert(&mut self, mut itemNum: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) { Self::insert(self, itemNum, item) }
    fn set(&mut self, mut itemNum: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) { Self::set(self, itemNum, item) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::form::PaintableForm::PaintableForm::getWidth(crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn::as_PaintableForm_mut(self)) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}
impl crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayableDyn for CommandCurrentSelectionForm {
    fn as_CustomDisplayable(&self) -> &crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &self.base.base.base.base.base.base }
    fn as_CustomDisplayable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &mut self.base.base.base.base.base.base }
}
impl crate::org::allbinary::graphics::form::CustomScreen::CustomScreenDyn for CommandCurrentSelectionForm {
    fn as_CustomScreen(&self) -> &crate::org::allbinary::graphics::form::CustomScreen::CustomScreen { &self.base.base.base.base.base }
    fn as_CustomScreen_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::CustomScreen::CustomScreen { &mut self.base.base.base.base.base }
}


