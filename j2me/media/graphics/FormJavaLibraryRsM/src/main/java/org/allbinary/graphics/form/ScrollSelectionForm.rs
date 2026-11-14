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
use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::math::RectangleCollisionUtil::RectangleCollisionUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::PaintableForm::PaintableForm;
use super::ItemPaintableFactory::ItemPaintableFactory;
use super::ItemIndexPaintable::ItemIndexPaintable;
use super::ItemIndexDx::ItemIndexDx;
use super::ItemPaintable::ItemPaintable;
use super::FormTypeFactory::FormTypeFactory;
use super::ScrollSelectionFormTempHorizontalPaintable::ScrollSelectionFormTempHorizontalPaintable;
use super::ScrollSelectionFormTempHorizontalDx::ScrollSelectionFormTempHorizontalDx;
use super::ScrollSelectionFormVerticalPaintable::ScrollSelectionFormVerticalPaintable;
use super::ScrollSelectionFormVericalDx::ScrollSelectionFormVericalDx;
use super::ScrollSelectionFormHorizontalPaintable::ScrollSelectionFormHorizontalPaintable;
use super::ScrollSelectionFormHorizontalDx::ScrollSelectionFormHorizontalDx;
use super::FormType::FormType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScrollSelectionForm {
    pub base: PaintableForm,
    pub rectangleCollisionUtil: Option<Rc<RefCell<RectangleCollisionUtil>>>,
    pub border: i32,
    pub halfBorder: i32,
    pub buttonBasicColor: Option<Rc<RefCell<BasicColor>>>,
    pub scrollSelectionFormFormTypeItemIndexPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintableDyn>>>,
    pub formTypeItemIndexDx: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDxDyn>>>,
    pub paintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintableDyn>>>,
    pub adjustedExtraBorder: i32,
}

pub static mut ScrollSelectionForm_NULL_SCROLL_SELECTION_FORM: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());
pub static mut ScrollSelectionForm_NULL_SCROLL_SELECTION_HORIZONTAL_FORM: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());
thread_local! { pub static ScrollSelectionForm_INSIDE_FORM: String = String::from(" inside form"); }
thread_local! { pub static ScrollSelectionForm_IS_IN_FORM: String = String::from("isInForm"); }
thread_local! { pub static ScrollSelectionForm_GET_SELECTED_INDEX: String = String::from("getSelectedIndexForPoint"); }

impl ScrollSelectionForm {
    

    pub fn createForm(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut formPaintableFactory: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintableFactory::ItemPaintableFactoryDyn>>>, mut border: i32, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn>>> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if match (&formPaintableFactory, &ItemPaintableFactory::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return Some(ScrollSelectionForm::new(title.clone(), items.clone(), formPaintableFactory.clone(), border.clone(), -3, backgroundBasicColor.clone(), foregroundBasicColor.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn>>));
            } else {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                jcall!(logUtil).putF({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(logUtil.clone()), { let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone());
                return Some(PaintableForm::getNullPaintableForm());
            }None
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            jcall!(logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(logUtil.clone()), { let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            return PaintableForm::getNullPaintableForm();
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn getNullScrollSelectionForm() -> Option<Rc<RefCell<dyn ScrollSelectionFormDyn>>> {
        if match (&unsafe { ScrollSelectionForm_NULL_SCROLL_SELECTION_FORM.clone() }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *ScrollSelectionForm_NULL_SCROLL_SELECTION_FORM = crate::java_into_object(ScrollSelectionForm::createForm({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), vec![None; (0) as usize], ItemPaintableFactory::getInstance(), 0, { let java_field = jcall!(BasicColorFactory::getInstance()).BLACK.clone(); java_field }.clone(), { let java_field = jcall!(BasicColorFactory::getInstance()).WHITE.clone(); java_field }.clone())) };
        }
        return { let java_object = (unsafe { ScrollSelectionForm_NULL_SCROLL_SELECTION_FORM.clone() }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ScrollSelectionFormDyn>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getNullScrollSelectionFormHorizontal() -> Option<Rc<RefCell<dyn ScrollSelectionFormDyn>>> {
        if match (&unsafe { ScrollSelectionForm_NULL_SCROLL_SELECTION_HORIZONTAL_FORM.clone() }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *ScrollSelectionForm_NULL_SCROLL_SELECTION_HORIZONTAL_FORM = crate::java_into_object(ScrollSelectionForm::createForm({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), vec![None; (0) as usize], ItemPaintableFactory::getInstance(), 0, { let java_field = jcall!(BasicColorFactory::getInstance()).BLACK.clone(); java_field }.clone(), { let java_field = jcall!(BasicColorFactory::getInstance()).WHITE.clone(); java_field }.clone())) };
        }
        return { let java_object = (unsafe { ScrollSelectionForm_NULL_SCROLL_SELECTION_HORIZONTAL_FORM.clone() }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ScrollSelectionFormDyn>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn new_struct(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut formPaintableFactory: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintableFactory::ItemPaintableFactoryDyn>>>, mut border: i32, mut adjustedExtraBorder: i32, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: PaintableForm::new_struct(title.clone(), items.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
            rectangleCollisionUtil: None,
            border: 0,
            halfBorder: 0,
            buttonBasicColor: None,
            scrollSelectionFormFormTypeItemIndexPaintable: None,
            formTypeItemIndexDx: None,
            paintable: None,
            adjustedExtraBorder: 0,
        };
        this.rectangleCollisionUtil = RectangleCollisionUtil::getInstance();
        this.scrollSelectionFormFormTypeItemIndexPaintable = ItemIndexPaintable::getInstance();
        this.formTypeItemIndexDx = ItemIndexDx::getInstance();
        this.paintable = ItemPaintableFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintable::ItemPaintableDyn>>);
        // super(...) is in the struct literal above
        this.adjustedExtraBorder = adjustedExtraBorder.clone();
        this.buttonBasicColor = foregroundBasicColor.clone();
        this.border = border.clone();
        this.halfBorder = (border >> 1);
        this
    }

    pub fn new(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut formPaintableFactory: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintableFactory::ItemPaintableFactoryDyn>>>, mut border: i32, mut adjustedExtraBorder: i32, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(title.clone(), items.clone(), formPaintableFactory.clone(), border.clone(), adjustedExtraBorder.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()))));
            jcall!(this).paintable = { let java_value = (jcall!(formPaintableFactory).getInstanceItemPaintable(this.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn>>))).clone(); java_value };
        this
    }
    

    // throws Exception
    pub fn init(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut formType: Option<Rc<RefCell<FormType>>>) {
        { let java_arguments = (rectangle.clone(), formType.clone(),); self.base.init(java_arguments.0, java_arguments.1) };
        let mut formTypeFactory: Option<Rc<RefCell<FormTypeFactory>>> = FormTypeFactory::getInstance();
        if match (&formType, &{ let java_field = jcall!(formTypeFactory).HORIZONTAL_FORM.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.scrollSelectionFormFormTypeItemIndexPaintable = ScrollSelectionFormHorizontalPaintable::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintableDyn>>);
            self.formTypeItemIndexDx = ScrollSelectionFormHorizontalDx::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDxDyn>>);
        } else if match (&formType, &{ let java_field = jcall!(formTypeFactory).VERTICAL_CENTER_FORM.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.scrollSelectionFormFormTypeItemIndexPaintable = ScrollSelectionFormVerticalPaintable::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintableDyn>>);
            self.formTypeItemIndexDx = ScrollSelectionFormVericalDx::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDxDyn>>);
        } else if match (&formType, &{ let java_field = jcall!(formTypeFactory).TEMP_HORIZONTAL_FORM.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.scrollSelectionFormFormTypeItemIndexPaintable = ScrollSelectionFormTempHorizontalPaintable::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintableDyn>>);
            self.formTypeItemIndexDx = ScrollSelectionFormTempHorizontalDx::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemIndexDx::ItemIndexDxDyn>>);
        } else if match (&formType, &{ let java_field = jcall!(formTypeFactory).NULL_FORM_TYPE.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", ({ let java_field = jcall!(formTypeFactory).UNK.clone(); java_field }.clone())));
        }
    }
    

    pub fn paintItemHorizontal(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut x: i32, mut y: i32) -> i32 {
        let mut width: i32 = jcall!(item).getMinimumWidth();
        return x + width + self.border;
    }
    

    pub fn paintItemVertical(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut x: i32, mut y: i32) -> i32 {
        let mut height: i32 = jcall!(item).getMinimumHeight();
        return y + height + self.border;
    }
    

    pub fn paintItemTempHorizontal(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut x: i32, mut y: i32) -> i32 {
        return 0;
    }
    

    pub fn getSelectedIndexForPointHorizontalDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
        return dx - self.halfBorder;
    }
    

    pub fn getSelectedIndexForPointVerticalDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
        return dx + self.getDiffX(item.clone());
    }
    

    pub fn getSelectedIndexForPointTempHorizontalDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
        return dx + self.getDiffX(item.clone());
    }
    

    pub fn append(&mut self, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 {
        return { let java_arguments = (item.clone(),); self.base.append(java_arguments.0) };
    }
    

    // throws Exception
    pub fn getSelectedItem(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> {
        let mut index: i32 = self.getSelectedIndexForPoint(point.clone());
        if index != -1 {
            return self.get(index.clone());
        } else {
            return ABCustomItem::getNullInstance();
        }
    }
    

    // throws Exception
    pub fn getItemIndex(&mut self, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 {
        let mut nextItem: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> = None;
        let mut size: i32 = self.size();
        for index in (0..size) {
            nextItem = (self.get(index.clone())).clone();
            if match (&nextItem, &item) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return index.clone();
            }
        }
        return -1;
    }
    

    pub fn getStartIndex(&mut self) -> i32 {
        return 0;
    }
    

    // throws Exception
    pub fn getSelectedIndexForPoint(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> i32 {
        let mut start: i32 = self.getStartIndex();
        let mut size: i32 = self.size();
        let mut dx: i32 = self.getDx();
        let mut dy: i32 = self.getDy();
        let mut formTypeFactory: Option<Rc<RefCell<FormTypeFactory>>> = FormTypeFactory::getInstance();
        let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append({ let java_field = jcall!(commonLabels).START_LABEL.clone(); java_field }.clone());
        jcall!(stringBuffer).appendint(start.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(CommonSeps::getInstance()).SPACE.clone(); java_field }.clone());
        jcall!(stringBuffer).append({ let java_field = jcall!(commonLabels).TOTAL_LABEL.clone(); java_field }.clone());
        jcall!(stringBuffer).appendint(size.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), ScrollSelectionForm_GET_SELECTED_INDEX.with(Clone::clone).clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> = None;
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut diffX: i32 = 0;
        for index in (start..size) {
            item = self.get(index.clone());
            width = { let java_value = (jcall!(item).getMinimumWidth()).clone(); java_value };
            height = { let java_value = (jcall!(item).getMinimumHeight()).clone(); java_value };
            diffX = { let java_value = (jcall!(self.formTypeItemIndexDx).getDx(index.clone(), item.clone(), dx.clone(), dy.clone())).clone(); java_value };
            if { let java_receiver = self.rectangleCollisionUtil.clone(); let java_arguments = (diffX.clone(), dy - self.halfBorder, diffX + width + self.border, dy + height + self.halfBorder + 1, jcall!(point).getX(), jcall!(point).getY(),); let java_result = jcall!(java_receiver).isInside(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); java_result } {
                jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
                jcall!(stringBuffer).append({ let java_field = jcall!(commonLabels).ITEM_LABEL.clone(); java_field }.clone());
                jcall!(stringBuffer).append(jcall!(item).getLabel());
                jcall!(stringBuffer).append({ let java_field = jcall!(CommonSeps::getInstance()).SPACE.clone(); java_field }.clone());
                jcall!(stringBuffer).append({ let java_field = jcall!(commonLabels).INDEX_LABEL.clone(); java_field }.clone());
                jcall!(stringBuffer).appendint(index.clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), ScrollSelectionForm_GET_SELECTED_INDEX.with(Clone::clone).clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                return index.clone();
            }
            if match (&self.formType, &{ let java_field = jcall!(formTypeFactory).HORIZONTAL_FORM.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                dx = dx + width + self.border;
                if dx > jcall!(self.rectangle).getMaxX() {
                    break;
                }
            } else if match (&self.formType, &{ let java_field = jcall!(formTypeFactory).VERTICAL_CENTER_FORM.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                dy = dy + height + self.border;
                if dy > jcall!(self.rectangle).getMaxY() {
                    break;
                }
            } else if match (&self.formType, &{ let java_field = jcall!(formTypeFactory).NULL_FORM_TYPE.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            } else {
                panic!("{:?}", format!("{}: {:?}", "Exception", ({ let java_field = jcall!(formTypeFactory).UNK.clone(); java_field }.clone())));
            }
        }
        return -1;
    }
    

    // throws Exception
    pub fn processInputKey(&mut self, mut gameKeyCode: i32) -> i32 {
        let mut formTypeFactory: Option<Rc<RefCell<FormTypeFactory>>> = FormTypeFactory::getInstance();
        if self.size() == 0 {
            return -1;
        }
        let mut index: i32 = self.getSelectedIndex();
        if match (&self.formType, &{ let java_field = jcall!(formTypeFactory).HORIZONTAL_FORM.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&self.formType, &{ let java_field = jcall!(formTypeFactory).TEMP_HORIZONTAL_FORM.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if gameKeyCode == crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.with(Clone::clone) {
                index += 1;
            } else if gameKeyCode == crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.with(Clone::clone) {
                index -= 1;
            }
        } else if match (&self.formType, &{ let java_field = jcall!(formTypeFactory).VERTICAL_CENTER_FORM.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if gameKeyCode == crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.with(Clone::clone) {
                index += 1;
            } else if gameKeyCode == crate::javax::microedition::lcdui::Canvas::Canvas_UP.with(Clone::clone) {
                index -= 1;
            }
        } else if match (&self.formType, &{ let java_field = jcall!(formTypeFactory).NULL_FORM_TYPE.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", ({ let java_field = jcall!(formTypeFactory).UNK.clone(); java_field }.clone())));
        }
        let mut max: i32 = self.size() - 1;
        if index < 0 {
            self.setSelectedIndex(0);
        } else if index > max {
            self.setSelectedIndex(max.clone());
        } else {
            self.setSelectedIndex(index.clone());
        }
        return -1;
    }
    

    pub fn isInForm(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> bool {
        if { let java_receiver = self.rectangleCollisionUtil.clone(); let java_arguments = (self.x.clone(), self.y - self.halfBorder, jcall!(self.rectangle).getMaxX() + self.border, jcall!(self.rectangle).getMaxY() + self.border, jcall!(point).getX(), jcall!(point).getY(),); let java_result = jcall!(java_receiver).isInside(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); java_result } {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(point.clone()))).append(ScrollSelectionForm_INSIDE_FORM.with(Clone::clone).clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), ScrollSelectionForm_IS_IN_FORM.with(Clone::clone).clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            return true;
        }
        return false;
    }
    

    // throws Exception
    pub fn paintItem(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut x: i32, mut y: i32) -> i32 {
        jcall!(item).paintXY(graphics.clone(), x.clone(), y.clone());
        let mut width: i32 = jcall!(item).getMinimumWidth();
        let mut height: i32 = jcall!(item).getMinimumHeight();
        jcall!(graphics).setColor({ let java_argument = jcall!(self.getButtonBasicColor()).intValue(); java_argument });
        let mut adjustedBorder: i32 = 3;
        jcall!(graphics).drawRect(x - self.halfBorder - adjustedBorder, y - self.halfBorder - adjustedBorder, width + self.border + self.adjustedExtraBorder, height + self.border + self.adjustedExtraBorder);
        return jcall!(self.scrollSelectionFormFormTypeItemIndexPaintable).paint(graphics.clone(), index.clone(), item.clone(), x.clone(), y.clone());
    }
    

    // throws Exception
    pub fn paintUnselectedItem(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut x: i32, mut y: i32) -> i32 {
        jcall!(graphics).setColor({ let java_argument = jcall!(self.getButtonBasicColor()).intValue(); java_argument });
        jcall!(item).paintUnselected(graphics.clone(), x.clone(), y.clone());
        return jcall!(self.scrollSelectionFormFormTypeItemIndexPaintable).paint(graphics.clone(), index.clone(), item.clone(), x.clone(), y.clone());
    }
    

    pub fn getDiffX(&mut self, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 {
        return 0;
    }
    

    pub fn getDx(&mut self) -> i32 {
        return self.x.clone();
    }
    

    pub fn getDy(&mut self) -> i32 {
        return self.y.clone();
    }
    

    pub fn setButtonBasicColor(&mut self, mut buttonBasicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.buttonBasicColor = buttonBasicColor.clone();
    }
    

    pub fn getButtonBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.buttonBasicColor.clone();
    }
}
impl std::ops::Deref for ScrollSelectionForm {
    type Target = PaintableForm;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ScrollSelectionForm {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ScrollSelectionFormDyn: crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn {
    fn as_ScrollSelectionForm(&self) -> &ScrollSelectionForm;
    fn as_ScrollSelectionForm_mut(&mut self) -> &mut ScrollSelectionForm;
    fn paintItemHorizontal(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut x: i32, mut y: i32) -> i32 { ScrollSelectionForm::paintItemHorizontal(self.as_ScrollSelectionForm_mut(), graphics, index, item, x, y) }
    fn paintItemVertical(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut x: i32, mut y: i32) -> i32 { ScrollSelectionForm::paintItemVertical(self.as_ScrollSelectionForm_mut(), graphics, index, item, x, y) }
    fn paintItemTempHorizontal(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut x: i32, mut y: i32) -> i32 { ScrollSelectionForm::paintItemTempHorizontal(self.as_ScrollSelectionForm_mut(), graphics, index, item, x, y) }
    fn getSelectedIndexForPointHorizontalDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { ScrollSelectionForm::getSelectedIndexForPointHorizontalDx(self.as_ScrollSelectionForm_mut(), index, item, dx, dy) }
    fn getSelectedIndexForPointVerticalDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { ScrollSelectionForm::getSelectedIndexForPointVerticalDx(self.as_ScrollSelectionForm_mut(), index, item, dx, dy) }
    fn getSelectedIndexForPointTempHorizontalDx(&mut self, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { ScrollSelectionForm::getSelectedIndexForPointTempHorizontalDx(self.as_ScrollSelectionForm_mut(), index, item, dx, dy) }
    fn getSelectedItem(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> { ScrollSelectionForm::getSelectedItem(self.as_ScrollSelectionForm_mut(), point) }
    fn getItemIndex(&mut self, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 { ScrollSelectionForm::getItemIndex(self.as_ScrollSelectionForm_mut(), item) }
    fn getStartIndex(&mut self) -> i32 { ScrollSelectionForm::getStartIndex(self.as_ScrollSelectionForm_mut()) }
    fn getSelectedIndexForPoint(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> i32 { ScrollSelectionForm::getSelectedIndexForPoint(self.as_ScrollSelectionForm_mut(), point) }
    fn processInputKey(&mut self, mut gameKeyCode: i32) -> i32 { ScrollSelectionForm::processInputKey(self.as_ScrollSelectionForm_mut(), gameKeyCode) }
    fn isInForm(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> bool { ScrollSelectionForm::isInForm(self.as_ScrollSelectionForm_mut(), point) }
    fn paintItem(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut x: i32, mut y: i32) -> i32 { ScrollSelectionForm::paintItem(self.as_ScrollSelectionForm_mut(), graphics, index, item, x, y) }
    fn paintUnselectedItem(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut x: i32, mut y: i32) -> i32 { ScrollSelectionForm::paintUnselectedItem(self.as_ScrollSelectionForm_mut(), graphics, index, item, x, y) }
    fn getDiffX(&mut self, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 { ScrollSelectionForm::getDiffX(self.as_ScrollSelectionForm_mut(), item) }
    fn getDx(&mut self) -> i32 { ScrollSelectionForm::getDx(self.as_ScrollSelectionForm_mut()) }
    fn getDy(&mut self) -> i32 { ScrollSelectionForm::getDy(self.as_ScrollSelectionForm_mut()) }
    fn setButtonBasicColor(&mut self, mut buttonBasicColor: Option<Rc<RefCell<BasicColor>>>) { ScrollSelectionForm::setButtonBasicColor(self.as_ScrollSelectionForm_mut(), buttonBasicColor) }
    fn getButtonBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> { ScrollSelectionForm::getButtonBasicColor(self.as_ScrollSelectionForm_mut()) }
}
impl ScrollSelectionFormDyn for ScrollSelectionForm {
    fn as_ScrollSelectionForm(&self) -> &ScrollSelectionForm { self }
    fn as_ScrollSelectionForm_mut(&mut self) -> &mut ScrollSelectionForm { self }
}

impl crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn for ScrollSelectionForm {
    fn as_PaintableForm(&self) -> &crate::org::allbinary::graphics::form::PaintableForm::PaintableForm { &self.base }
    fn as_PaintableForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::PaintableForm::PaintableForm { &mut self.base }
    fn init(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut formType: Option<Rc<RefCell<FormType>>>) { Self::init(self, rectangle, formType) }
}
impl crate::org::allbinary::graphics::form::ABCustomForm::ABCustomFormDyn for ScrollSelectionForm {
    fn as_ABCustomForm(&self) -> &crate::org::allbinary::graphics::form::ABCustomForm::ABCustomForm { &self.base.base }
    fn as_ABCustomForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ABCustomForm::ABCustomForm { &mut self.base.base }
    fn append(&mut self, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 { Self::append(self, item) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::form::PaintableForm::PaintableForm::getWidth(crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn::as_PaintableForm_mut(self)) }
}
impl crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayableDyn for ScrollSelectionForm {
    fn as_CustomDisplayable(&self) -> &crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &self.base.base.base.base }
    fn as_CustomDisplayable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &mut self.base.base.base.base }
}
impl crate::org::allbinary::graphics::form::CustomScreen::CustomScreenDyn for ScrollSelectionForm {
    fn as_CustomScreen(&self) -> &crate::org::allbinary::graphics::form::CustomScreen::CustomScreen { &self.base.base.base }
    fn as_CustomScreen_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::CustomScreen::CustomScreen { &mut self.base.base.base }
}


