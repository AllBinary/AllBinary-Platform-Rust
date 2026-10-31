// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
// package org.allbinary.game.layer.building
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHud;
use crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHudFactory::NumberStringHudFactory;
use crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::game::graphics::hud::BasicHudFactory::BasicHudFactory;
use crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer;
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::logic::math::MathUtil::MathUtil;
use super::ProductivityNumberStringHud::ProductivityNumberStringHud;
use super::EfficiencyNumberStringHud::EfficiencyNumberStringHud;
use super::HealthNumberStringHud::HealthNumberStringHud;
use super::MaxHealthNumberStringHud::MaxHealthNumberStringHud;
use super::BuildingLayer::BuildingLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BuildingInfoHudPaintable {
    pub base: SelectionHudPaintable,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub HEALTH: String,
    pub productivityHud: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn>>>,
    pub efficiencyHud: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn>>>,
    pub healthHud: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn>>>,
    pub maxHealthHud: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn>>>,
    pub rtsLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>,
    pub currentHealth: i32,
}

thread_local! { pub static BuildingInfoHudPaintable_instance: Option<Rc<RefCell<BuildingInfoHudPaintable>>> = BuildingInfoHudPaintable::new(); }
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for BuildingInfoHudPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::graphics::paint::InitUpdatePaintableInterface::InitUpdatePaintableInterface for BuildingInfoHudPaintable {
}
impl crate::org::allbinary::init::InitInterface::InitInterface for BuildingInfoHudPaintable {
    fn init(&mut self) { self.base.init() }
}
impl crate::org::allbinary::init::UpdateInterface::UpdateInterface for BuildingInfoHudPaintable {
    fn update(&mut self) { self.base.update() }
}

impl BuildingInfoHudPaintable {
    

    pub fn getInstance() -> Option<Rc<RefCell<BuildingInfoHudPaintable>>> {
        return BuildingInfoHudPaintable_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: SelectionHudPaintable::new_struct(),
            logUtil: None,
            HEALTH: String::new(),
            productivityHud: None,
            efficiencyHud: None,
            healthHud: None,
            maxHealthHud: None,
            rtsLayer: None,
            currentHealth: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.HEALTH = String::from("Health:");
        this.rtsLayer = crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer_NULL_PATH_FINDING_LAYER.with(Clone::clone).clone();
        this.currentHealth = 0;
        let mut numberStringHudFactory: Option<Rc<RefCell<NumberStringHudFactory>>> = NumberStringHudFactory::getInstance();
        let mut productivityHud: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn>>> = jcall!(numberStringHudFactory).NULL_NUMBER_STRING_HUD.clone();
        let mut efficiencyHud: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn>>> = jcall!(numberStringHudFactory).NULL_NUMBER_STRING_HUD.clone();
        let mut healthHud: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn>>> = jcall!(numberStringHudFactory).NULL_NUMBER_STRING_HUD.clone();
        let mut maxHealthHud: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn>>> = jcall!(numberStringHudFactory).NULL_NUMBER_STRING_HUD.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut index: i32 = 0;
            let mut basicHudFactory: Option<Rc<RefCell<BasicHudFactory>>> = BasicHudFactory::getInstance();
            let mut basicColor: Option<Rc<RefCell<BasicColor>>> = this.getBasicColorP();
            let mut textX: i32 = this.textX.clone();
            let mut y: i32 = this.y.clone();
            let mut firstIndex: i32 = index.clone();
            /* Java local class: move it out of the function
pub struct ProductivityNumberStringHud {
    pub base: NumberStringHud,
}

impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for ProductivityNumberStringHud {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl ProductivityNumberStringHud {
                

                pub fn new_struct() -> Self {
                    let mut this = Self {
            base: NumberStringHud::new_struct(String::from("Productivity:"), 999, jcall!(this.base.basicHudFactory).ABSOLUTE.clone(), jcall!(this.base.basicHudFactory).HORIZONTAL.clone(), 0, this.base.basicColor.clone()),
                    };
                    // super(...) is in the struct literal above
                    this
                }

                pub fn new() -> Option<Rc<RefCell<Self>>> {
                    Some(Rc::new(RefCell::new(Self::new_struct())))
                }
                

                pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
                    let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
                    self.updateMaxWidth = self.base.textX.clone();
                    self.updateMaxHeight = self.base.y + ((self.base.firstIndex + 1) * jcall!(font).getHeight());
                    self.base.updateMeasurement(graphics.clone());
                }
}
impl std::ops::Deref for ProductivityNumberStringHud {
    type Target = NumberStringHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ProductivityNumberStringHud {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn for ProductivityNumberStringHud {
    fn as_NumberStringHud(&self) -> &crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHud { &self }
    fn as_NumberStringHud_mut(&mut self) -> &mut crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHud { &mut self }
}
impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for ProductivityNumberStringHud {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

*/
            productivityHud = ProductivityNumberStringHud::new();
            index += 1;
            let mut secondIndex: i32 = index.clone();
            /* Java local class: move it out of the function
pub struct EfficiencyNumberStringHud {
    pub base: NumberStringHud,
}

impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for EfficiencyNumberStringHud {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl EfficiencyNumberStringHud {
                

                pub fn new_struct() -> Self {
                    let mut this = Self {
            base: NumberStringHud::new_struct(String::from("Efficiency:"), 999, jcall!(this.base.basicHudFactory).ABSOLUTE.clone(), jcall!(this.base.basicHudFactory).HORIZONTAL.clone(), 0, this.base.basicColor.clone()),
                    };
                    // super(...) is in the struct literal above
                    this
                }

                pub fn new() -> Option<Rc<RefCell<Self>>> {
                    Some(Rc::new(RefCell::new(Self::new_struct())))
                }
                

                pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
                    self.base.updateMeasurement(graphics.clone());
                    let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
                    self.updateMaxWidth = self.base.textX.clone();
                    self.updateMaxHeight = self.base.y + ((self.base.secondIndex + 1) * jcall!(font).getHeight());
                }
}
impl std::ops::Deref for EfficiencyNumberStringHud {
    type Target = NumberStringHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for EfficiencyNumberStringHud {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn for EfficiencyNumberStringHud {
    fn as_NumberStringHud(&self) -> &crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHud { &self }
    fn as_NumberStringHud_mut(&mut self) -> &mut crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHud { &mut self }
}
impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for EfficiencyNumberStringHud {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

*/
            efficiencyHud = EfficiencyNumberStringHud::new();
            index += 1;
            let mut HEALTH: String = this.HEALTH.clone();
            let mut thirdIndex: i32 = index.clone();
            /* Java local class: move it out of the function
pub struct HealthNumberStringHud {
    pub base: NumberStringHud,
}

impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for HealthNumberStringHud {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl HealthNumberStringHud {
                

                pub fn new_struct() -> Self {
                    let mut this = Self {
            base: NumberStringHud::new_struct(this.HEALTH.clone(), 99999, jcall!(this.base.basicHudFactory).ABSOLUTE.clone(), jcall!(this.base.basicHudFactory).HORIZONTAL.clone(), 0, this.base.basicColor.clone()),
                    };
                    // super(...) is in the struct literal above
                    this
                }

                pub fn new() -> Option<Rc<RefCell<Self>>> {
                    Some(Rc::new(RefCell::new(Self::new_struct())))
                }
                

                pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
                    let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
                    self.updateMaxWidth = self.base.textX.clone();
                    self.updateMaxHeight = self.base.y + ((self.base.thirdIndex + 1) * jcall!(font).getHeight());
                    self.base.updateMeasurement(graphics.clone());
                }
}
impl std::ops::Deref for HealthNumberStringHud {
    type Target = NumberStringHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HealthNumberStringHud {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn for HealthNumberStringHud {
    fn as_NumberStringHud(&self) -> &crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHud { &self }
    fn as_NumberStringHud_mut(&mut self) -> &mut crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHud { &mut self }
}
impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for HealthNumberStringHud {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

*/
            let mut totalLength: i32 = (HEALTH.chars().count() as i32) + 1;
            healthHud = HealthNumberStringHud::new();
            let mut fourthIndex: i32 = index.clone();
            /* Java local class: move it out of the function
pub struct MaxHealthNumberStringHud {
    pub base: NumberStringHud,
}

impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for MaxHealthNumberStringHud {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl MaxHealthNumberStringHud {
                

                pub fn new_struct() -> Self {
                    let mut this = Self {
            base: NumberStringHud::new_struct(String::from("/ "), 99999, jcall!(this.base.basicHudFactory).ABSOLUTE.clone(), jcall!(this.base.basicHudFactory).HORIZONTAL.clone(), 0, this.base.basicColor.clone()),
                    };
                    // super(...) is in the struct literal above
                    this
                }

                pub fn new() -> Option<Rc<RefCell<Self>>> {
                    Some(Rc::new(RefCell::new(Self::new_struct())))
                }
                

                pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
                    self.base.updateMeasurement(graphics.clone());
                    let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
                    self.updateMaxWidth = self.base.textX + (self.base.totalLength * jcall!(font).getHeight());
                    self.updateMaxHeight = self.base.y + ((self.base.fourthIndex + 1) * jcall!(font).getHeight());
                }
}
impl std::ops::Deref for MaxHealthNumberStringHud {
    type Target = NumberStringHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MaxHealthNumberStringHud {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn for MaxHealthNumberStringHud {
    fn as_NumberStringHud(&self) -> &crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHud { &self }
    fn as_NumberStringHud_mut(&mut self) -> &mut crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHud { &mut self }
}
impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for MaxHealthNumberStringHud {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

*/
            maxHealthHud = MaxHealthNumberStringHud::new();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
        }
        this.productivityHud = productivityHud.clone();
        this.efficiencyHud = efficiencyHud.clone();
        this.healthHud = healthHud.clone();
        this.maxHealthHud = maxHealthHud.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        let mut totalLength: i32 = (self.HEALTH.chars().count() as i32) + jcall!(MathUtil::getInstance()).getTotalDigits(self.currentHealth.clone());
        { let java_receiver = self.maxHealthHud.clone(); let java_arguments = (self.textX + MyFontProcessor::defaultStringWidth(font.clone(), totalLength.clone()),); jcall!(java_receiver).setX(java_arguments.0); };
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.base.setBasicColorP(basicColor.clone());
        jcall!(self.productivityHud).setBasicColorP(basicColor.clone());
        jcall!(self.efficiencyHud).setBasicColorP(basicColor.clone());
        jcall!(self.healthHud).setBasicColorP(basicColor.clone());
        jcall!(self.maxHealthHud).setBasicColorP(basicColor.clone());
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.paint(graphics.clone());
        jcall!(self.myFontProcessor).process(graphics.clone());
        jcall!(self.productivityHud).paint(graphics.clone());
        jcall!(self.efficiencyHud).paint(graphics.clone());
        jcall!(self.healthHud).paint(graphics.clone());
        jcall!(self.maxHealthHud).paint(graphics.clone());
        jcall!(self.getAnimationInterface()).paintXY(graphics.clone(), self.imageX.clone(), self.y.clone());
    }
    

    pub fn updateSelectionInfo(&mut self) {
        let mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>> = (self.rtsLayer).clone() /* as BuildingLayer */;
        { let java_arguments = (jcall!(buildingLayer).getName(),); self.setName(java_arguments.0) };
        { let java_arguments = (jcall!(buildingLayer).getVerticleBuildAnimationInterface(),); self.setAnimationInterface(java_arguments.0) };
        { let java_receiver = self.productivityHud.clone(); let java_arguments = (jcall!(buildingLayer).getProductivity(),); jcall!(java_receiver).set(java_arguments.0); };
        { let java_receiver = self.efficiencyHud.clone(); let java_arguments = (jcall!(buildingLayer).getEfficiency() / 100,); jcall!(java_receiver).set(java_arguments.0); };
        self.currentHealth = jcall!(jcall!(buildingLayer).getHealthInterface()).getHealth();
        jcall!(self.healthHud).set(self.currentHealth.clone());
        self.myFontProcessor = self.updateMyFontProcessor.clone();
        { let java_receiver = self.maxHealthHud.clone(); let java_arguments = (jcall!(jcall!(buildingLayer).getHealthInterface()).getMaxHealth(),); jcall!(java_receiver).set(java_arguments.0); };
    }
    

    pub fn setRtsLayer(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        self.rtsLayer = rtsLayer.clone();
    }
}
impl std::ops::Deref for BuildingInfoHudPaintable {
    type Target = SelectionHudPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BuildingInfoHudPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn for BuildingInfoHudPaintable {
    fn as_SelectionHudPaintable(&self) -> &crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable { &self.base }
    fn as_SelectionHudPaintable_mut(&mut self) -> &mut crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable { &mut self.base }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
    fn updateSelectionInfo(&mut self) { Self::updateSelectionInfo(self) }
}
impl crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn for BuildingInfoHudPaintable {
    fn as_InitUpdatePaintable(&self) -> &crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &self.base.base }
    fn as_InitUpdatePaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &mut self.base.base }
    fn update(&mut self) { crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable::update(crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn::as_SelectionHudPaintable_mut(self)) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for BuildingInfoHudPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base.base }
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


