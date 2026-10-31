// Generated Code Do Not Modify
// package org.allbinary.graphics.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
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
use super::ScrollSelectionForm::ScrollSelectionForm;
use super::FormType::FormType;
use super::ItemIndexPaintable::ItemIndexPaintable;
use super::MPaintable::MPaintable;
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
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl MultipleScrollSelectionHorizontalForm {
    

    // throws Exception
    pub fn createForm(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut formType: Option<Rc<RefCell<FormType>>>, mut border: i32, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> {
        let mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> = MultipleScrollSelectionHorizontalForm::new(title.clone(), items.clone(), ItemPaintableFactory::getInstance(), border.clone(), backgroundBasicColor.clone(), foregroundBasicColor.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>);
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
        this.backgroundColor = jcall!(jcall!(BasicColorFactory::getInstance()).TRANSPARENT_GREY).intValue();
        this.myFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.logged = false;
        this.fontHeight = 0;
        // super(...) is in the struct literal above
        if J2MEUtil::isJ2ME() || J2MEUtil::isHTML() {
        } else {
            /* Java local class: move it out of the function
pub struct MPaintable {
    pub base: Paintable,
}


impl MPaintable {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
                

                pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
                    self.fillRect(graphics.clone());
                }
}
impl std::ops::Deref for MPaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for MPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}

*/
            this.rectPaintable = MPaintable::new();
        }
        this
    }

    pub fn new(mut title: String, mut items: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>>, mut formPaintableFactory: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ItemPaintableFactory::ItemPaintableFactoryDyn>>>, mut border: i32, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(title, items, formPaintableFactory, border, backgroundBasicColor, foregroundBasicColor))))
    }
    

    pub fn fillRect(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(graphics).setColor(self.backgroundColor.clone());
        jcall!(graphics).fillRect(self.x.clone(), self.y.clone(), jcall!(self.rectangle).getWidth(), jcall!(self.rectangle).getHeight());
    }
    

    // throws Exception
    pub fn init(&mut self, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut formType: Option<Rc<RefCell<FormType>>>) {
        self.base.init(rectangle.clone(), formType.clone());
        let mut formTypeFactory: Option<Rc<RefCell<FormTypeFactory>>> = FormTypeFactory::getInstance();
        if match (&formType, &jcall!(formTypeFactory).HORIZONTAL_FORM) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.multipleScrollSelectionHorizontalFormTypeItemIndexPaintable = MultipleScrollSelectionFormHorizontalPaintable::new(None /* Java this: a &mut self method has no Rc of itself */);
        } else if match (&formType, &jcall!(formTypeFactory).VERTICAL_CENTER_FORM) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.multipleScrollSelectionHorizontalFormTypeItemIndexPaintable = MultipleScrollSelectionFormVerticalPaintable::new(None /* Java this: a &mut self method has no Rc of itself */);
        } else if match (&formType, &jcall!(formTypeFactory).NULL_FORM_TYPE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(formTypeFactory).UNK.clone())));
        }
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.fontHeight = jcall!(font).getHeight();
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    // throws Exception
    pub fn paintHorizontal(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
        let mut dx2: i32 = self.paintItem(graphics.clone(), index.clone(), item.clone(), dx.clone(), dy.clone()) + self.border;
        jcall!(self.paintable).paint(graphics.clone(), index.clone(), dx.clone(), dy.clone());
        dx = dx2.clone();
        return dx.clone();
    }
    

    // throws Exception
    pub fn paintVertical(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
        self.paintItem(graphics.clone(), index.clone(), item.clone(), dx.clone(), dy.clone());
        jcall!(self.paintable).paint(graphics.clone(), index.clone(), dx.clone(), dy.clone());
        return dx.clone();
    }
    

    pub fn getStartIndex(&mut self) -> i32 {
        return self.getSelectedIndex();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.myFontProcessor).process(graphics.clone());
            let mut start: i32 = self.getStartIndex();
            let mut size: i32 = self.size();
            let mut dx: i32 = self.x.clone();
            let mut dy: i32 = self.y.clone();
            jcall!(self.rectPaintable).paint(graphics.clone());
            jcall!(graphics).drawString(self.getTitle(), self.x.clone(), self.y - self.fontHeight, 0);
            let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>> = None;
            for index in (start..size) {
                item = self.get(index.clone());
                if dx >= jcall!(self.rectangle).getMaxX() {
                    if self.logged {
                    } else {
                        self.logged = true;
                        PreLogUtil::put(jcall!(StringMaker::new()).append(String::from("painting beyond maxx: ")).appendint(jcall!(self.rectangle).getMaxX()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.canvasStrings).PAINT.clone());
                    }
                }
                dx = jcall!(self.multipleScrollSelectionHorizontalFormTypeItemIndexPaintable).paint(graphics.clone(), index.clone(), item.clone(), dx.clone(), dy.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.canvasStrings).PAINT.clone());
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
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}
impl crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayableDyn for MultipleScrollSelectionHorizontalForm {
    fn as_CustomDisplayable(&self) -> &crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &self.base.base.base.base.base }
    fn as_CustomDisplayable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::CustomDisplayable::CustomDisplayable { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::graphics::form::CustomScreen::CustomScreenDyn for MultipleScrollSelectionHorizontalForm {
    fn as_CustomScreen(&self) -> &crate::org::allbinary::graphics::form::CustomScreen::CustomScreen { &self.base.base.base.base }
    fn as_CustomScreen_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::CustomScreen::CustomScreen { &mut self.base.base.base.base }
}


pub struct MultipleScrollSelectionFormHorizontalPaintable {
    pub base: ItemIndexPaintable,
    pub multipleScrollSelectionForm: Option<Rc<RefCell<MultipleScrollSelectionHorizontalForm>>>,
}


impl MultipleScrollSelectionFormHorizontalPaintable {
        

        pub fn new_struct(mut multipleScrollSelectionForm: Option<Rc<RefCell<MultipleScrollSelectionHorizontalForm>>>) -> Self {
            let mut this = Self {
            base: ItemIndexPaintable::new_struct(),
            multipleScrollSelectionForm: None,
            };
            this.multipleScrollSelectionForm = multipleScrollSelectionForm.clone();
            this
        }

        pub fn new(mut multipleScrollSelectionForm: Option<Rc<RefCell<MultipleScrollSelectionHorizontalForm>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(multipleScrollSelectionForm))))
        }
        

        // throws Exception
        pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
            return jcall!(self.multipleScrollSelectionForm).paintHorizontal(graphics.clone(), index.clone(), item.clone(), dx.clone(), dy.clone());
        }
}
impl std::ops::Deref for MultipleScrollSelectionFormHorizontalPaintable {
    type Target = ItemIndexPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MultipleScrollSelectionFormHorizontalPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintableDyn for MultipleScrollSelectionFormHorizontalPaintable {
    fn as_ItemIndexPaintable(&self) -> &crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintable { &self }
    fn as_ItemIndexPaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintable { &mut self }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { Self::paint(self, graphics, index, item, dx, dy) }
}


pub struct MultipleScrollSelectionFormVerticalPaintable {
    pub base: ItemIndexPaintable,
    pub multipleScrollSelectionForm: Option<Rc<RefCell<MultipleScrollSelectionHorizontalForm>>>,
}


impl MultipleScrollSelectionFormVerticalPaintable {
        

        pub fn new_struct(mut multipleScrollSelectionForm: Option<Rc<RefCell<MultipleScrollSelectionHorizontalForm>>>) -> Self {
            let mut this = Self {
            base: ItemIndexPaintable::new_struct(),
            multipleScrollSelectionForm: None,
            };
            this.multipleScrollSelectionForm = multipleScrollSelectionForm.clone();
            this
        }

        pub fn new(mut multipleScrollSelectionForm: Option<Rc<RefCell<MultipleScrollSelectionHorizontalForm>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(multipleScrollSelectionForm))))
        }
        

        // throws Exception
        pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 {
            return jcall!(self.multipleScrollSelectionForm).paintVertical(graphics.clone(), index.clone(), item.clone(), dx.clone(), dy.clone());
        }
}
impl std::ops::Deref for MultipleScrollSelectionFormVerticalPaintable {
    type Target = ItemIndexPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MultipleScrollSelectionFormVerticalPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintableDyn for MultipleScrollSelectionFormVerticalPaintable {
    fn as_ItemIndexPaintable(&self) -> &crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintable { &self }
    fn as_ItemIndexPaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::form::ItemIndexPaintable::ItemIndexPaintable { &mut self }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut index: i32, mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABCustomItem::ABCustomItemDyn>>>, mut dx: i32, mut dy: i32) -> i32 { Self::paint(self, graphics, index, item, dx, dy) }
}


