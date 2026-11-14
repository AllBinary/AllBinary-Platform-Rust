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
// package org.allbinary.game.paint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::ColorChangeEventHandler::ColorChangeEventHandler;
use crate::org::allbinary::graphics::color::ColorChangeListener::ColorChangeListener;
use crate::org::allbinary::graphics::form::FormPaintable::FormPaintable;
use crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm;
use crate::org::allbinary::graphics::form::item::ABStringComponent::ABStringComponent;
use crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItem;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use super::BasicGameDemoPaintable::BasicGameDemoPaintable;
use super::MainGameDemoStatePaintable::MainGameDemoStatePaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameScrollMenuPaintable {
    pub base: BasicGameDemoPaintable,
}

impl ColorChangeListener for GameScrollMenuPaintable {
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for GameScrollMenuPaintable {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl GameScrollMenuPaintable {
    

    pub fn new_struct(mut mainDemoStatePaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>, mut ownershipPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>, mut helpPaintableInterface: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: BasicGameDemoPaintable::new_struct(mainDemoStatePaintable.clone(), ownershipPaintable.clone(), helpPaintableInterface.clone()),
        };
        // super(...) is in the struct literal above
        this.setColor(basicColor.clone());
        this
    }

    pub fn new(mut mainDemoStatePaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>, mut ownershipPaintable: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>, mut helpPaintableInterface: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(mainDemoStatePaintable.clone(), ownershipPaintable.clone(), helpPaintableInterface.clone(), basicColor.clone()))));
            jcall!(ColorChangeEventHandler::getInstance()).addListenerInterface(this.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>));
        this
    }
    

    pub fn setState(&mut self, mut state: i32) {
        { let java_arguments = (self.getMainDemoStatePaintable(),); self.setCurrentStatePaintable(java_arguments.0) };
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
    }
    

    pub fn setColor(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        let mut mainGameDemoStatePaintable: Option<Rc<RefCell<MainGameDemoStatePaintable>>> = (self.getMainDemoStatePaintable()).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<MainGameDemoStatePaintable>() }, "ClassCastException: not a MainGameDemoStatePaintable"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<MainGameDemoStatePaintable>) } });
        let mut formPaintable: Option<Rc<RefCell<FormPaintable>>> = (jcall!(mainGameDemoStatePaintable).getMenuPaintableInterface()).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<FormPaintable>() }, "ClassCastException: not a FormPaintable"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<FormPaintable>) } });
        let mut scrollSelectionForm: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> { panic!("ClassCastException: a Rust dyn PaintableForm cannot be cast to ScrollSelectionForm") })();
        jcall!(jcall!(mainGameDemoStatePaintable).getOwnershipPaintableInterface()).setBasicColorP(basicColor.clone());
        jcall!(scrollSelectionForm).setButtonBasicColor(basicColor.clone());
        let mut item: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn>>> = None;
        let mut stringComponent: Option<Rc<RefCell<ABStringComponent>>> = None;
        for index in (0..={ let java_bound = jcall!(scrollSelectionForm).size() - 1; java_bound }).rev() {
            item = { let java_value = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::item::ABTextItem::ABTextItemDyn>>> { panic!("ClassCastException: a Rust dyn ABCustomItem cannot be cast to ABTextItem") })()).clone(); java_value };
            stringComponent = { let java_value = (jcall!(item).getLabelStringComponent()).clone(); java_value };
            if stringComponent.is_some() {
                jcall!(stringComponent).setForegroundBasicColor(basicColor.clone());
            }
        }
    }
}
impl std::ops::Deref for GameScrollMenuPaintable {
    type Target = BasicGameDemoPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GameScrollMenuPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::paint::BasicGameDemoPaintable::BasicGameDemoPaintableDyn for GameScrollMenuPaintable {
    fn as_BasicGameDemoPaintable(&self) -> &crate::org::allbinary::game::paint::BasicGameDemoPaintable::BasicGameDemoPaintable { &self.base }
    fn as_BasicGameDemoPaintable_mut(&mut self) -> &mut crate::org::allbinary::game::paint::BasicGameDemoPaintable::BasicGameDemoPaintable { &mut self.base }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for GameScrollMenuPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base.base }
}
impl crate::org::allbinary::graphics::paint::StatePaintable::StatePaintableDyn for GameScrollMenuPaintable {
    fn as_StatePaintable(&self) -> &crate::org::allbinary::graphics::paint::StatePaintable::StatePaintable { &self.base.base }
    fn as_StatePaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::StatePaintable::StatePaintable { &mut self.base.base }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for GameScrollMenuPaintable {
    fn paint(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::game::paint::BasicGameDemoPaintable::BasicGameDemoPaintable::paint(crate::org::allbinary::game::paint::BasicGameDemoPaintable::BasicGameDemoPaintableDyn::as_BasicGameDemoPaintable_mut(self), p1) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::paint::Paintable::Paintable::paintThreed(crate::org::allbinary::graphics::paint::Paintable::PaintableDyn::as_Paintable_mut(self), p1) }
}
impl crate::org::allbinary::graphics::displayable::StateControlledInterface::StateControlledInterface for GameScrollMenuPaintable {
    fn setState(&mut self, mut state: i32) { Self::setState(self, state) }
}


