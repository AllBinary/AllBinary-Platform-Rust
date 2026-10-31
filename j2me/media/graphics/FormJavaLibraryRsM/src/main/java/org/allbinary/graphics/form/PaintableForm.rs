// Generated Code Do Not Modify
// package org.allbinary.graphics.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::RectangleFactory::RectangleFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::ABCustomForm::ABCustomForm;
use super::FormType::FormType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaintableForm {
    pub base: ABCustomForm,
    pub canvasStrings: Option<Rc<RefCell<CanvasStrings>>>,
    pub rectangle: Option<Rc<RefCell<Rectangle>>>,
    pub x: i32,
    pub y: i32,
    pub formType: Option<Rc<RefCell<FormType>>>,
}

pub static mut PaintableForm_NULL_PAINTABLE_FORM: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl PaintableForm {
    

    pub fn getNullPaintableForm() -> Option<Rc<RefCell<dyn PaintableFormDyn>>> {
        if match (&unsafe { PaintableForm_NULL_PAINTABLE_FORM.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *PaintableForm_NULL_PAINTABLE_FORM = PaintableForm::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), vec![None; (0) as usize], jcall!(BasicColorFactory::getInstance()).BLACK.clone(), jcall!(BasicColorFactory::getInstance()).WHITE.clone()).map(|reference| reference as Rc<RefCell<dyn PaintableFormDyn>>) };
        }
        return (unsafe { PaintableForm_NULL_PAINTABLE_FORM.clone() }).clone() /* as PaintableForm */;
    }
    

    pub fn new_struct(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ABCustomForm::new_struct(title.clone(), items.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()),
            canvasStrings: None,
            rectangle: None,
            x: 0,
            y: 0,
            formType: None,
        };
        this.canvasStrings = CanvasStrings::getInstance();
        this.rectangle = crate::org::allbinary::graphics::RectangleFactory::RectangleFactory_SINGLETON.with(Clone::clone).clone();
        this.formType = jcall!(FormTypeFactory::FormTypeFactory::getInstance()).NULL_FORM_TYPE.clone();
        // super(...) is in the struct literal above
        this.setSelectedIndex(0);
        this
    }

    pub fn new(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(title, items, backgroundBasicColor, foregroundBasicColor))))
    }
    

    // throws Exception
    pub fn init(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut formType: Option<Rc<RefCell<FormType>>>) {
        self.rectangle = rectangle.clone();
        let mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> = jcall!(self.rectangle).getPoint();
        self.x = jcall!(point).getX();
        self.y = jcall!(point).getY();
        self.formType = formType.clone();
    }
    

    pub fn getWidth(&mut self) -> i32 {
        return jcall!(self.rectangle).getWidth();
    }
}
impl std::ops::Deref for PaintableForm {
    type Target = ABCustomForm;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PaintableForm {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait PaintableFormDyn: crate::org::allbinary::graphics::form::ABCustomForm::ABCustomFormDyn {
    fn as_PaintableForm(&self) -> &PaintableForm;
    fn as_PaintableForm_mut(&mut self) -> &mut PaintableForm;
    fn init(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut formType: Option<Rc<RefCell<FormType>>>) { PaintableForm::init(self.as_PaintableForm_mut(), rectangle, formType) }
}
impl PaintableFormDyn for PaintableForm {
    fn as_PaintableForm(&self) -> &PaintableForm { self }
    fn as_PaintableForm_mut(&mut self) -> &mut PaintableForm { self }
}

impl crate::org::allbinary::graphics::form::ABCustomForm::ABCustomFormDyn for PaintableForm {
    fn as_ABCustomForm(&self) -> &crate::org::allbinary::graphics::form::ABCustomForm::ABCustomForm { &self.base }
    fn as_ABCustomForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ABCustomForm::ABCustomForm { &mut self.base }
    fn getWidth(&mut self) -> i32 { Self::getWidth(self) }
}
impl crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayableDyn for PaintableForm {
    fn as_CustomDisplayable(&self) -> &crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &self.base.base.base }
    fn as_CustomDisplayable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &mut self.base.base.base }
}
impl crate::org::allbinary::graphics::form::CustomScreen::CustomScreenDyn for PaintableForm {
    fn as_CustomScreen(&self) -> &crate::org::allbinary::graphics::form::CustomScreen::CustomScreen { &self.base.base }
    fn as_CustomScreen_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::CustomScreen::CustomScreen { &mut self.base.base }
}


