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
// package org.allbinary.input.automation.actions.script.condition
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::Color::Color;
// use crate::java::awt::Point::Point;
// use crate::java::awt::image::BufferedImage::BufferedImage;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::data::tree::dom::DomNodeHelper::DomNodeHelper;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::graphics::color::ColorCacheFactory::ColorCacheFactory;
use crate::org::allbinary::graphics::color::ColorCacheable::ColorCacheable;
use crate::org::allbinary::input::media::image::capture::CapturedBufferedImagesCacheSingleton::CapturedBufferedImagesCacheSingleton;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::cache::AutomaticCacheInterface::AutomaticCacheInterface;
use crate::org::allbinary::media::image::analysis::ColorRange::ColorRange;
use crate::org::allbinary::media::image::analysis::ColorRangeInterface::ColorRangeInterface;
use crate::org::allbinary::media::image::cache::BufferedImageFrameCacheable::BufferedImageFrameCacheable;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::org::w3c::dom::NodeList::NodeList;
use crate::java::lang::Exception::*;
use super::BasicProfileActionScriptCondition::BasicProfileActionScriptCondition;
use super::ColorAtActionScriptConditionInterface::ColorAtActionScriptConditionInterface;
use super::ColorAtActionScriptConditionJPanel::ColorAtActionScriptConditionJPanel;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ColorAtActionScriptCondition {
    pub base: BasicProfileActionScriptCondition,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub colorAtActionScriptConditionJPanel: Option<Rc<RefCell<ColorAtActionScriptConditionJPanel>>>,
    pub point: Option<Rc<RefCell<Point>>>,
    pub colorRangeInterface: Option<Rc<RefCell<dyn ColorRangeInterface>>>,
}

pub static ColorAtActionScriptCondition_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("If Color At"));
impl ColorAtActionScriptConditionInterface for ColorAtActionScriptCondition {
    fn getPoint(&mut self) -> Option<Rc<RefCell<Point>>> { Self::getPoint(self) }
    fn setPoint(&mut self, mut point: Option<Rc<RefCell<Point>>>) { Self::setPoint(self, point) }
    fn getColorRangeInterface(&mut self) -> Option<Rc<RefCell<dyn ColorRangeInterface>>> { Self::getColorRangeInterface(self) }
    fn setColorRangeInterface(&mut self, mut colorRangeInterface: Option<Rc<RefCell<dyn ColorRangeInterface>>>) { Self::setColorRangeInterface(self, colorRangeInterface) }
    fn log(&mut self) { Self::log(self) }
}

impl ColorAtActionScriptCondition {
    

    pub fn new_struct1(mut node: Option<Rc<RefCell<Node>>>) -> Self {
        let mut this = Self {
            base: BasicProfileActionScriptCondition::new_struct2(ColorAtActionScriptCondition_NAME.with(Clone::clone).clone(), node.clone()),
            logUtil: None,
            colorAtActionScriptConditionJPanel: None,
            point: None,
            colorRangeInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.colorRangeInterface = (ColorRange::new()).clone() /* as ColorRangeInterface */;
        // super(...) is in the struct literal above
        this.point = Point::new(0, 0);
        let mut actionNode: Option<Rc<RefCell<Node>>> = DomSearchHelper::getNode(unsafe { ColorAtActionScriptConditionData_NAME.clone() }.clone(), jcall!(node).getChildNodes());
        if actionNode.is_some() {
            let mut nodeList: Option<Rc<RefCell<NodeList>>> = jcall!(actionNode).getChildNodes();
            for index in (0..{ let java_bound = jcall!(nodeList).getLength(); java_bound }) {
                let mut childNode: Option<Rc<RefCell<Node>>> = jcall!(nodeList).item(index.clone());
                if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { ColorAtActionScriptConditionData_LOCATION_X.clone() }.clone()) == 0 {
                    let mut locationX: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    jcall!(this.point).x = Some((locationX.clone()).trim().parse::<i32>().expect("NumberFormatException"));
                } else if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { ColorAtActionScriptConditionData_LOCATION_Y.clone() }.clone()) == 0 {
                    let mut locationY: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    jcall!(this.point).y = Some((locationY.clone()).trim().parse::<i32>().expect("NumberFormatException"));
                } else if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { ColorAtActionScriptConditionData_MIN_RED.clone() }.clone()) == 0 {
                    let mut string: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    jcall!(this.getColorRangeInterface()).setMinRed(Some((string.clone()).trim().parse::<i32>().expect("NumberFormatException")));
                } else if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { ColorAtActionScriptConditionData_MAX_RED.clone() }.clone()) == 0 {
                    let mut string: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    jcall!(this.getColorRangeInterface()).setMaxRed(Some((string.clone()).trim().parse::<i32>().expect("NumberFormatException")));
                } else if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { ColorAtActionScriptConditionData_MIN_GREEN.clone() }.clone()) == 0 {
                    let mut string: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    jcall!(this.getColorRangeInterface()).setMinGreen(Some((string.clone()).trim().parse::<i32>().expect("NumberFormatException")));
                } else if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { ColorAtActionScriptConditionData_MAX_GREEN.clone() }.clone()) == 0 {
                    let mut string: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    jcall!(this.getColorRangeInterface()).setMaxGreen(Some((string.clone()).trim().parse::<i32>().expect("NumberFormatException")));
                } else if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { ColorAtActionScriptConditionData_MIN_BLUE.clone() }.clone()) == 0 {
                    let mut string: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    jcall!(this.getColorRangeInterface()).setMinBlue(Some((string.clone()).trim().parse::<i32>().expect("NumberFormatException")));
                } else if jcall!(jcall!(childNode).getNodeName()).compareTo(unsafe { ColorAtActionScriptConditionData_MAX_BLUE.clone() }.clone()) == 0 {
                    let mut string: String = DomNodeHelper::getTextNodeValue(childNode.clone());
                    jcall!(this.getColorRangeInterface()).setMaxBlue(Some((string.clone()).trim().parse::<i32>().expect("NumberFormatException")));
                }
            }
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Color At Action Script Condition Node Null"))));
        }
        this.init();
        this
    }

    pub fn new1(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(node))))
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicProfileActionScriptCondition::new_struct1(ColorAtActionScriptCondition_NAME.with(Clone::clone).clone()),
            logUtil: None,
            colorAtActionScriptConditionJPanel: None,
            point: None,
            colorRangeInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.colorRangeInterface = (ColorRange::new()).clone() /* as ColorRangeInterface */;
        // super(...) is in the struct literal above
        this.point = Point::new(0, 0);
        this.init();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
        self.colorAtActionScriptConditionJPanel = ColorAtActionScriptConditionJPanel::new(None /* Java this: a &mut self method has no Rc of itself */);
    }
    

    pub fn getPoint(&mut self) -> Option<Rc<RefCell<Point>>> {
        return self.point.clone();
    }
    

    pub fn setPoint(&mut self, mut point: Option<Rc<RefCell<Point>>>) {
        self.point = point.clone();
    }
    

    pub fn showDialog(&mut self) {
        jcall!(jcall!(self.colorAtActionScriptConditionJPanel).getColorAtActionJDialog()).setVisible(true);
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(crate::java_into_object(unsafe { ColorAtActionScriptConditionData_LOCATION_X.clone() }.clone()), { let java_argument = crate::java_into_object(({ let java_field = jcall!(self.getPoint()).x.clone(); java_field }.clone()).to_string()); java_argument });
        jcall!(hashMap).put(crate::java_into_object(unsafe { ColorAtActionScriptConditionData_LOCATION_Y.clone() }.clone()), { let java_argument = crate::java_into_object(({ let java_field = jcall!(self.getPoint()).y.clone(); java_field }.clone()).to_string()); java_argument });
        jcall!(hashMap).put(crate::java_into_object(unsafe { ColorAtActionScriptConditionData_MIN_RED.clone() }.clone()), { let java_argument = crate::java_into_object(({ let java_argument = jcall!(self.getColorRangeInterface()).getMinRed(); java_argument }).to_string()); java_argument });
        jcall!(hashMap).put(crate::java_into_object(unsafe { ColorAtActionScriptConditionData_MAX_RED.clone() }.clone()), { let java_argument = crate::java_into_object(({ let java_argument = jcall!(self.getColorRangeInterface()).getMaxRed(); java_argument }).to_string()); java_argument });
        jcall!(hashMap).put(crate::java_into_object(unsafe { ColorAtActionScriptConditionData_MIN_GREEN.clone() }.clone()), { let java_argument = crate::java_into_object(({ let java_argument = jcall!(self.getColorRangeInterface()).getMinGreen(); java_argument }).to_string()); java_argument });
        jcall!(hashMap).put(crate::java_into_object(unsafe { ColorAtActionScriptConditionData_MAX_GREEN.clone() }.clone()), { let java_argument = crate::java_into_object(({ let java_argument = jcall!(self.getColorRangeInterface()).getMaxGreen(); java_argument }).to_string()); java_argument });
        jcall!(hashMap).put(crate::java_into_object(unsafe { ColorAtActionScriptConditionData_MIN_BLUE.clone() }.clone()), { let java_argument = crate::java_into_object(({ let java_argument = jcall!(self.getColorRangeInterface()).getMinBlue(); java_argument }).to_string()); java_argument });
        jcall!(hashMap).put(crate::java_into_object(unsafe { ColorAtActionScriptConditionData_MAX_BLUE.clone() }.clone()), { let java_argument = crate::java_into_object(({ let java_argument = jcall!(self.getColorRangeInterface()).getMaxBlue(); java_argument }).to_string()); java_argument });
        { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("HashMap: "), jcall!(hashMap).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toHashMap()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = { let java_arguments = (document.clone(),); self.base.toXmlNode(java_arguments.0) };
        jcall!(node).appendChild({ let java_argument = ModDomHelper::createNodeWithValueNodes_Document_Node_HashMap(document.clone(), unsafe { ColorAtActionScriptConditionData_NAME.clone() }.clone(), self.toHashMap()); java_argument });
        return node.clone();
    }
    

    // throws Exception
    pub fn shouldProcess(&mut self, mut frame: i64) -> bool {
        let mut capturedBufferedImageCacheable: Option<Rc<RefCell<BufferedImageFrameCacheable>>> = (jcall!(((|| -> Option<Rc<RefCell<dyn AutomaticCacheInterface>>> { panic!("ClassCastException: a Rust dyn CacheInterface cannot be cast to dyn AutomaticCacheInterface") })())).get(crate::java_into_object(Some((frame.clone()) as i64)))).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<BufferedImageFrameCacheable>() }, "ClassCastException: not a BufferedImageFrameCacheable"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<BufferedImageFrameCacheable>) } });
        let mut bufferedImage: Option<Rc<RefCell<BufferedImage>>> = jcall!(capturedBufferedImageCacheable).getBufferedImage();
        let mut automaticCacheInterface: Option<Rc<RefCell<dyn AutomaticCacheInterface>>> = ColorCacheFactory::getInstance();
        let mut colorInteger: Option<i32> = Some(({ let java_argument = jcall!(bufferedImage).getRGB({ let java_field = jcall!(self.getPoint()).x.clone(); java_field }.clone(), { let java_field = jcall!(self.getPoint()).y.clone(); java_field }.clone()); java_argument }) as i32);
        let mut colorCacheable: Option<Rc<RefCell<ColorCacheable>>> = (jcall!(automaticCacheInterface).get(crate::java_into_object((colorInteger.clone()).expect("NullPointerException")))).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<ColorCacheable>() }, "ClassCastException: not a ColorCacheable"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<ColorCacheable>) } });
        let mut color: Option<Rc<RefCell<Color>>> = jcall!(colorCacheable).getColor();
        if jcall!(self.colorRangeInterface).isInRange(color.clone()) {
            return { let java_arguments = (frame.clone(),); self.base.shouldProcess(java_arguments.0) };
        }
        return false;
    }
    

    pub fn log(&mut self) {
        let mut message: String = jcall!(StringMaker::new()).append(String::from("getPoint(): ")).append({ let java_argument = jcall!(self.getPoint()).toString(); java_argument }).append({ let java_field = jcall!(CommonSeps::getInstance()).NEW_LINE.clone(); java_field }.clone()).append({ let java_argument = jcall!(self.getColorRangeInterface()).toString(); java_argument }).toString();
        jcall!(self.logUtil).putF(message.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("log"));
    }
    

    pub fn getColorRangeInterface(&mut self) -> Option<Rc<RefCell<dyn ColorRangeInterface>>> {
        return self.colorRangeInterface.clone();
    }
    

    pub fn setColorRangeInterface(&mut self, mut colorRangeInterface: Option<Rc<RefCell<dyn ColorRangeInterface>>>) {
        self.colorRangeInterface = colorRangeInterface.clone();
    }
}
impl std::ops::Deref for ColorAtActionScriptCondition {
    type Target = BasicProfileActionScriptCondition;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ColorAtActionScriptCondition {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::input::automation::actions::script::condition::BasicProfileActionScriptCondition::BasicProfileActionScriptConditionDyn for ColorAtActionScriptCondition {
    fn as_BasicProfileActionScriptCondition(&self) -> &crate::org::allbinary::input::automation::actions::script::condition::BasicProfileActionScriptCondition::BasicProfileActionScriptCondition { &self.base }
    fn as_BasicProfileActionScriptCondition_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::condition::BasicProfileActionScriptCondition::BasicProfileActionScriptCondition { &mut self.base }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItemDyn for ColorAtActionScriptCondition {
    fn as_ProfileActionScriptItem(&self) -> &crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &self.base.base }
    fn as_ProfileActionScriptItem_mut(&mut self) -> &mut crate::org::allbinary::input::automation::actions::script::ProfileActionScriptItem::ProfileActionScriptItem { &mut self.base.base }
    fn init(&mut self) { Self::init(self) }
    fn showDialog(&mut self) { Self::showDialog(self) }
}


