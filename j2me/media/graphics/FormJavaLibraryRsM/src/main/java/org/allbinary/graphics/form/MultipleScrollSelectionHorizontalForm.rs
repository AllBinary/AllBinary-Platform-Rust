// Generated Code Do Not Modify
// package org.allbinary.graphics.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItem;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::java::lang::Exception::*;
use super::ScrollSelectionForm::ScrollSelectionForm;
use super::FormType::FormType;
use super::ItemIndexPaintable::ItemIndexPaintable;
use super::MultipleScrollSelectionFormRectPaintable::MultipleScrollSelectionFormRectPaintable;
use super::ItemPaintableFactory::ItemPaintableFactory;
use super::FormTypeFactory::FormTypeFactory;
use super::MultipleScrollSelectionFormVerticalPaintable::MultipleScrollSelectionFormVerticalPaintable;
use super::MultipleScrollSelectionFormHorizontalPaintable::MultipleScrollSelectionFormHorizontalPaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultipleScrollSelectionHorizontalForm {
    pub base: ScrollSelectionForm,
    pub rectPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
    pub multipleScrollSelectionHorizontalFormTypeItemIndexPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintableDyn>>>,
    pub backgroundColor: i32,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub logged: bool,
    pub fontHeight: i32,
}

impl UpdateMyFontInterface for MultipleScrollSelectionHorizontalForm {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}

impl MultipleScrollSelectionHorizontalForm {
    

    // throws Exception
    pub fn createForm(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut formType: Option<Rc<RefCell<FormType>>>, mut border: i32, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> {
        let mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> = MultipleScrollSelectionHorizontalForm::new(title.clone(), items.clone(), ItemPaintableFactory::getInstance(), border.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>);
        jcall!(scrollSelectionForm).init(rectangle.clone(), formType.clone());
        return scrollSelectionForm.clone();
    }
    

    pub fn new_struct(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut formPaintableFactory: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintableFactory::ItemPaintableFactoryDyn>>>, mut border: i32, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: ScrollSelectionForm::new_struct(title.clone(), items.clone(), formPaintableFactory.clone(), border.clone(), 3, backgroundBasicColor.clone(), foregroundBasicColor.clone()),
            rectPaintable: None,
            multipleScrollSelectionHorizontalFormTypeItemIndexPaintable: None,
            backgroundColor: 0,
            myFontProcessor: None,
            logged: false,
            fontHeight: 0,
        };
        this.rectPaintable = NullPaintable::getInstance();
        this.multipleScrollSelectionHorizontalFormTypeItemIndexPaintable = ItemIndexPaintable::getInstance();
        this.backgroundColor = jcall!({ let java_field = jcall!(BasicColorFactory::getInstance()).TRANSPARENT_GREY.clone(); java_field }).intValue();
        this.logged = false;
        this.fontHeight = 0;
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut formPaintableFactory: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintableFactory::ItemPaintableFactoryDyn>>>, mut border: i32, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(title.clone(), items.clone(), formPaintableFactory.clone(), border.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()))));
        jcall!(this).myFontProcessor = UpdateMyFontProcessor::new(this.clone().map(|reference| reference as Rc<RefCell<dyn UpdateMyFontInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
            if J2MEUtil::isJ2ME() || J2MEUtil::isHTML() {
        } else {
            jcall!(this).rectPaintable = MultipleScrollSelectionFormRectPaintable::new(this.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        }
        this
    }
    

    pub fn fillRect(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(graphics).setColor(self.backgroundColor.clone());
        jcall!(graphics).fillRect(self.x.clone(), self.y.clone(), jcall!(self.rectangle).getWidth(), jcall!(self.rectangle).getHeight());
    }
    

    // throws Exception
    pub fn init(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut formType: Option<Rc<RefCell<FormType>>>) {
        { let java_arguments = (rectangle.clone(), formType.clone(),); self.base.init(java_arguments.0, java_arguments.1) };
        let mut formTypeFactory: Option<Rc<RefCell<FormTypeFactory>>> = FormTypeFactory::getInstance();
        if match (&formType, &{ let java_field = jcall!(formTypeFactory).HORIZONTAL_FORM.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.multipleScrollSelectionHorizontalFormTypeItemIndexPaintable = MultipleScrollSelectionFormHorizontalPaintable::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintableDyn>>);
        } else if match (&formType, &{ let java_field = jcall!(formTypeFactory).VERTICAL_CENTER_FORM.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.multipleScrollSelectionHorizontalFormTypeItemIndexPaintable = MultipleScrollSelectionFormVerticalPaintable::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintableDyn>>);
        } else if match (&formType, &{ let java_field = jcall!(formTypeFactory).NULL_FORM_TYPE.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", ({ let java_field = jcall!(formTypeFactory).UNK.clone(); java_field }.clone())));
        }
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        self.fontHeight = { let java_value = (jcall!(font).getHeight()).clone(); java_value };
        self.myFontProcessor = { let java_value = (MyFontProcessor::getInstance()).clone(); java_value };
    }
    

    // throws Exception
    pub fn paintHorizontal(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
        let mut dx2: i32 = self.paintItem(graphics.clone(), index.clone(), item.clone(), dx.clone(), dy.clone()) + self.border;
        jcall!(self.paintable).paint(graphics.clone(), index.clone(), dx.clone(), dy.clone());
        dx = dx2.clone();
        return dx.clone();
    }
    

    // throws Exception
    pub fn paintVertical(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
        self.paintItem(graphics.clone(), index.clone(), item.clone(), dx.clone(), dy.clone());
        jcall!(self.paintable).paint(graphics.clone(), index.clone(), dx.clone(), dy.clone());
        return dx.clone();
    }
    

    pub fn getStartIndex(&mut self) -> i32 {
        return self.getSelectedIndex();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.myFontProcessor).process(graphics.clone());
            let mut start: i32 = self.getStartIndex();
            let mut size: i32 = self.size();
            let mut dx: i32 = self.x.clone();
            let mut dy: i32 = self.y.clone();
            jcall!(self.rectPaintable).as_Paintable_mut().paint(graphics.clone());
            jcall!(graphics).drawString(self.getTitle(), self.x.clone(), self.y - self.fontHeight, 0);
            let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> = None;
            for index in (start..size) {
                item = self.get(index.clone());
                if dx >= jcall!(self.rectangle).getMaxX() {
                    if self.logged {
                    } else {
                        self.logged = true;
                        PreLogUtil::put(jcall!(StringMaker::new()).append(String::from("painting beyond maxx: ")).appendint(jcall!(self.rectangle).getMaxX()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.canvasStrings).PAINT.clone(); java_field }.clone());
                    }
                }
                dx = { let java_value = (jcall!(self.multipleScrollSelectionHorizontalFormTypeItemIndexPaintable).paint(graphics.clone(), index.clone(), item.clone(), dx.clone(), dy.clone())).clone(); java_value };
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            PreLogUtil::put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.canvasStrings).PAINT.clone(); java_field }.clone());
            }
        }
    }
}
impl std::ops::Deref for MultipleScrollSelectionHorizontalForm {
    type Target = ScrollSelectionForm;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MultipleScrollSelectionHorizontalForm {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn for MultipleScrollSelectionHorizontalForm {
    fn as_PaintableForm(&self) -> &crate::org::allbinary::graphics::form::PaintableForm::PaintableForm { &self.base.base }
    fn as_PaintableForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::PaintableForm::PaintableForm { &mut self.base.base }
    fn init(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut formType: Option<Rc<RefCell<FormType>>>) { Self::init(self, rectangle, formType) }
}
impl crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn for MultipleScrollSelectionHorizontalForm {
    fn as_ScrollSelectionForm(&self) -> &crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm { &self.base }
    fn as_ScrollSelectionForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm { &mut self.base }
    fn getStartIndex(&mut self) -> i32 { Self::getStartIndex(self) }
}
impl crate::org::allbinary::graphics::form::ABCustomForm::ABCustomFormDyn for MultipleScrollSelectionHorizontalForm {
    fn as_ABCustomForm(&self) -> &crate::org::allbinary::graphics::form::ABCustomForm::ABCustomForm { &self.base.base.base }
    fn as_ABCustomForm_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ABCustomForm::ABCustomForm { &mut self.base.base.base }
    fn append(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>) -> i32 { crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm::append(crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn::as_ScrollSelectionForm_mut(self), p1) }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::graphics::form::PaintableForm::PaintableForm::getWidth(crate::org::allbinary::graphics::form::PaintableForm::PaintableFormDyn::as_PaintableForm_mut(self)) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
}
impl crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayableDyn for MultipleScrollSelectionHorizontalForm {
    fn as_CustomDisplayable(&self) -> &crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &self.base.base.base.base.base }
    fn as_CustomDisplayable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::graphics::form::CustomScreen::CustomScreenDyn for MultipleScrollSelectionHorizontalForm {
    fn as_CustomScreen(&self) -> &crate::org::allbinary::graphics::form::CustomScreen::CustomScreen { &self.base.base.base.base }
    fn as_CustomScreen_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::CustomScreen::CustomScreen { &mut self.base.base.base.base }
}


