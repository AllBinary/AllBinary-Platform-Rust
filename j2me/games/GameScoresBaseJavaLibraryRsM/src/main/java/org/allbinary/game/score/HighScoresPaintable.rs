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
// package org.allbinary.game.score
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::color::ColorChangeEvent::ColorChangeEvent;
use crate::org::allbinary::graphics::color::ColorChangeListener::ColorChangeListener;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::HighScores::HighScores;
use super::HighScore::HighScore;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HighScoresPaintable {
    pub base: Paintable,
    pub displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub basicColor: Option<Rc<RefCell<BasicColor>>>,
    pub highScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>,
    pub anchor: i32,
    pub charHeight: i32,
}

impl ColorChangeListener for HighScoresPaintable {
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for HighScoresPaintable {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}
impl UpdateMyFontInterface for HighScoresPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}

impl HighScoresPaintable {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            displayInfoSingleton: None,
            myFontProcessor: None,
            basicColor: None,
            highScores: None,
            anchor: 0,
            charHeight: 0,
        };
        this.displayInfoSingleton = DisplayInfoSingleton::getInstance();
        this.basicColor = { let java_field = jcall!(BasicColorFactory::getInstance()).WHITE.clone(); java_field }.clone();
        this.highScores = NullHighScoresSingletonFactory::NullHighScoresSingletonFactory::getInstance();
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct())));
        jcall!(this).myFontProcessor = UpdateMyFontProcessor::new(this.clone().map(|reference| reference as Rc<RefCell<dyn UpdateMyFontInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        self.charHeight = { let java_value = (jcall!(font).getHeight()).clone(); java_value };
        self.myFontProcessor = { let java_value = (MyFontProcessor::getInstance()).clone(); java_value };
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        let mut colorChangeEvent: Option<Rc<RefCell<ColorChangeEvent>>> = (eventObject).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<ColorChangeEvent>() }, "ClassCastException: not a ColorChangeEvent"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<ColorChangeEvent>) } });
        self.basicColor = { let java_value = (jcall!(colorChangeEvent).getBasicColorP()).clone(); java_value };
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        let mut width: i32 = jcall!(self.displayInfoSingleton).getLastWidth();
        let mut height: i32 = jcall!(self.displayInfoSingleton).getLastHeight();
        jcall!(graphics).setColor({ let java_argument = jcall!(self.getBasicColorP()).intValue(); java_argument });
        let mut heading: String = jcall!(self.highScores).getHeading();
        let mut topScoresWidth: i32 = (jcall!(jcall!(graphics).getFont()).stringWidth(heading.clone()) >> 1);
        let mut charHeight: i32 = self.charHeight.clone();
        jcall!(graphics).drawString(heading.clone(), (width >> 1) - topScoresWidth, charHeight.clone(), self.anchor.clone());
        jcall!(graphics).drawString(jcall!(self.highScores).getColumnOneHeading(), 10, charHeight * 3, self.anchor.clone());
        let mut columnTwoHeading: String = jcall!(self.highScores).getColumnTwoHeading();
        let mut columnTwoHeadingWidth: i32 = jcall!(jcall!(graphics).getFont()).stringWidth(columnTwoHeading.clone());
        jcall!(graphics).drawString(columnTwoHeading.clone(), width - 10 - columnTwoHeadingWidth, charHeight * 3, self.anchor.clone());
        let mut index: i32 = 4;
        let mut largestSecondColumnWidth: i32 = columnTwoHeadingWidth.clone();
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(self.highScores).as_HighScores_mut().getList();
        let mut size: i32 = jcall!(list).size();
        let mut vectorIndex: i32 = 0;
        let mut highScore: Option<Rc<RefCell<HighScore>>> = None;
        while vectorIndex < size && charHeight * index < height - (charHeight * 2) {
            highScore = { let java_value = ({ let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(vectorIndex) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<HighScore>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            let mut nextScoreWidth: i32 = jcall!(jcall!(graphics).getFont()).stringWidth(jcall!(highScore).getScoreString());
            if nextScoreWidth > largestSecondColumnWidth {
                largestSecondColumnWidth = nextScoreWidth.clone();
            }
            vectorIndex += 1;
        }
        vectorIndex = 0;
        while vectorIndex < size && charHeight * index < height - (charHeight * 2) {
            highScore = { let java_value = ({ let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(vectorIndex) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<HighScore>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            jcall!(graphics).drawString(jcall!(highScore).getName(), 10, charHeight * index, self.anchor.clone());
            jcall!(graphics).drawString(jcall!(highScore).getScoreString(), width - 10 - largestSecondColumnWidth, charHeight * index, self.anchor.clone());
            index += 1;
            vectorIndex += 1;
        }
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.basicColor = basicColor.clone();
    }
    

    pub fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.basicColor.clone();
    }
    

    pub fn setHighScores(&mut self, mut highScores: Option<Rc<RefCell<dyn crate::org::allbinary::game::score::HighScores::HighScoresDyn>>>) {
        self.highScores = highScores.clone();
    }
}
impl std::ops::Deref for HighScoresPaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HighScoresPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for HighScoresPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for HighScoresPaintable {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::paint::Paintable::Paintable::paintThreed(crate::org::allbinary::graphics::paint::Paintable::PaintableDyn::as_Paintable_mut(self), p1) }
}


