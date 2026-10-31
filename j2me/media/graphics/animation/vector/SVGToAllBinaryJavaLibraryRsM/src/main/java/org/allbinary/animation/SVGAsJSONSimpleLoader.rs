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
// package org.allbinary.animation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Enumeration;
use super::*;
// use crate::java::util::Enumeration::Enumeration;
use crate::org::allbinary::graphics::GraphicsStrings::GraphicsStrings;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::math::PositionStrings::PositionStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::json::me::JSONArray::JSONArray;
use crate::org::json::me::JSONException::JSONException;
use crate::org::json::me::JSONObject::JSONObject;
use super::ShapeTypeFactory::ShapeTypeFactory;
use super::ShapeType::ShapeType;
use super::Number::Number;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SVGAsJSONSimpleLoader {
    pub graphicsStrings: Option<Rc<RefCell<GraphicsStrings>>>,
    pub shapeTypeFactory: Option<Rc<RefCell<ShapeTypeFactory>>>,
    pub KEY_SVG: String,
    pub KEY_X1: String,
    pub KEY_Y1: String,
    pub KEY_X2: String,
    pub KEY_Y2: String,
    pub KEY_CX: String,
    pub KEY_CY: String,
    pub KEY_R: String,
    pub KEY_X: String,
    pub KEY_Y: String,
    pub ERROR_PARSE_SVG_JSON: String,
    pub ERROR_MISSING_KEY_PREFIX: String,
    pub ERROR_INVALID_NUMBER_PREFIX: String,
    pub ERROR_EXPECTED_OBJECT_PREFIX: String,
    pub EMPTY_POINTS: Vec<Vec<i32>>,
    pub SHAPE_SEPARATOR: Vec<i32>,
    pub CIRCLE_SEGMENTS: i32,
}

thread_local! { pub static SVGAsJSONSimpleLoader_instance: Option<Rc<RefCell<SVGAsJSONSimpleLoader>>> = SVGAsJSONSimpleLoader::new(); }

impl SVGAsJSONSimpleLoader {

    pub fn new_struct() -> Self {
        let mut this = Self {
            graphicsStrings: None,
            shapeTypeFactory: None,
            KEY_SVG: String::new(),
            KEY_X1: String::new(),
            KEY_Y1: String::new(),
            KEY_X2: String::new(),
            KEY_Y2: String::new(),
            KEY_CX: String::new(),
            KEY_CY: String::new(),
            KEY_R: String::new(),
            KEY_X: String::new(),
            KEY_Y: String::new(),
            ERROR_PARSE_SVG_JSON: String::new(),
            ERROR_MISSING_KEY_PREFIX: String::new(),
            ERROR_INVALID_NUMBER_PREFIX: String::new(),
            ERROR_EXPECTED_OBJECT_PREFIX: String::new(),
            EMPTY_POINTS: Vec::new(),
            SHAPE_SEPARATOR: Vec::new(),
            CIRCLE_SEGMENTS: 0,
        };
        this.graphicsStrings = GraphicsStrings::getInstance();
        this.shapeTypeFactory = ShapeTypeFactory::getInstance();
        this.KEY_SVG = String::from("svg");
        this.KEY_X1 = String::from("x1");
        this.KEY_Y1 = String::from("y1");
        this.KEY_X2 = String::from("x2");
        this.KEY_Y2 = String::from("y2");
        this.KEY_CX = String::from("cx");
        this.KEY_CY = String::from("cy");
        this.KEY_R = String::from("r");
        this.KEY_X = jcall!(PositionStrings::getInstance()).X.clone();
        this.KEY_Y = jcall!(PositionStrings::getInstance()).Y.clone();
        this.ERROR_PARSE_SVG_JSON = String::from("Unable to parse SVG JSON");
        this.ERROR_MISSING_KEY_PREFIX = String::from("Missing key: ");
        this.ERROR_INVALID_NUMBER_PREFIX = String::from("Invalid number for key: ");
        this.ERROR_EXPECTED_OBJECT_PREFIX = String::from("Expected object for: ");
        this.EMPTY_POINTS = PrimitiveIntUtil::getTwoDimensionalArrayInstance();
        this.SHAPE_SEPARATOR = vec![1000, 1000];
        this.CIRCLE_SEGMENTS = 16;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<SVGAsJSONSimpleLoader>>> {
        return SVGAsJSONSimpleLoader_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn load(&mut self, mut svgAsJson: String) -> Vec<Vec<i32>> {
        if false {
            return self.EMPTY_POINTS.clone();
        }
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut root: Option<Rc<RefCell<JSONObject>>> = JSONObject::new(svgAsJson.clone());
            let mut svgValue: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(root).opt(self.KEY_SVG.clone());
            let mut svg: Option<Rc<RefCell<JSONObject>>> = (if svgValue.is_none() { root } else { self.asObject(svgValue.clone(), self.KEY_SVG.clone()) });
            let mut pointVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut keys: Option<Rc<RefCell<dyn Enumeration>>> = jcall!(svg).keys();
            while jcall!(keys).hasMoreElements() {
                let mut key: String = (jcall!(keys).nextElement()).clone() /* as String */;
                let mut value: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(svg).opt(key.clone());
                if (jcall!(self.shapeTypeFactory).KEY_LINE == key.clone()) {
                    self.addShapes(pointVector.clone(), value.clone(), jcall!(self.shapeTypeFactory).LINE.clone());
                } else if jcall!(self.shapeTypeFactory).equals(key.clone()) {
                    self.addShapes(pointVector.clone(), value.clone(), jcall!(self.shapeTypeFactory).CIRCLE.clone());
                } else if (jcall!(self.shapeTypeFactory).KEY_RECT == key.clone()) || (jcall!(self.shapeTypeFactory).KEY_RECTANGLE == key.clone()) {
                    self.addShapes(pointVector.clone(), value.clone(), jcall!(self.shapeTypeFactory).RECTANGLE.clone());
                }
            }
            return self.toPoints(pointVector.clone());
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", format!("{}: {:?}", "IllegalArgumentException", (self.ERROR_PARSE_SVG_JSON.clone(), e.clone())));
            }
        }
    }
    

    // throws Exception
    pub fn addShapes(&mut self, mut pointVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut value: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut shapeType: Option<Rc<RefCell<ShapeType>>>) {
        if (value).is_some() /* instanceof JSONArray */ {
            let mut array: Option<Rc<RefCell<JSONArray>>> = (value).clone() /* as JSONArray */;
            let mut size: i32 = jcall!(array).length();
            for index in (0..size) {
                { let java_arguments = (pointVector.clone(), jcall!(array).opt(index.clone()), shapeType.clone(),); self.addShape(java_arguments.0, java_arguments.1, java_arguments.2) };
            }
        } else {
            self.addShape(pointVector.clone(), value.clone(), shapeType.clone());
        }
    }
    

    // throws Exception
    pub fn addShape(&mut self, mut pointVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut value: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut shapeType: Option<Rc<RefCell<ShapeType>>>) {
        let mut shape: Option<Rc<RefCell<JSONObject>>> = { let java_arguments = (value.clone(), jcall!(shapeType).getName(),); self.asObject(java_arguments.0, java_arguments.1) };
        if match (&shapeType, &jcall!(self.shapeTypeFactory).LINE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.addLine(pointVector.clone(), shape.clone());
        } else if match (&shapeType, &jcall!(self.shapeTypeFactory).CIRCLE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.addCircle(pointVector.clone(), shape.clone());
        } else {
            self.addRectangle(pointVector.clone(), shape.clone());
        }
    }
    

    // throws Exception
    pub fn addLine(&mut self, mut pointVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut line: Option<Rc<RefCell<JSONObject>>>) {
        { let java_arguments = (pointVector.clone(), self.readInt(line.clone(), self.KEY_X1.clone()), self.readInt(line.clone(), self.KEY_Y1.clone()),); self.addPoint(java_arguments.0, java_arguments.1, java_arguments.2) };
        { let java_arguments = (pointVector.clone(), self.readInt(line.clone(), self.KEY_X2.clone()), self.readInt(line.clone(), self.KEY_Y2.clone()),); self.addPoint(java_arguments.0, java_arguments.1, java_arguments.2) };
        self.addSeparator(pointVector.clone());
    }
    

    // throws Exception
    pub fn addCircle(&mut self, mut pointVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut circle: Option<Rc<RefCell<JSONObject>>>) {
        let mut cx: i32 = self.readInt(circle.clone(), self.KEY_CX.clone());
        let mut cy: i32 = self.readInt(circle.clone(), self.KEY_CY.clone());
        let mut radius: i32 = self.readInt(circle.clone(), self.KEY_R.clone());
        for index in (0..=self.CIRCLE_SEGMENTS) {
            let mut angle: f64 = (unsafe { Math_PI.clone() } * 2.0 * index) / self.CIRCLE_SEGMENTS;
            let mut x: i32 = ((((cx + (((angle.clone()) as f64).cos() * radius)) as f64).round() as i64) as i32);
            let mut y: i32 = ((((cy + (((angle.clone()) as f64).sin() * radius)) as f64).round() as i64) as i32);
            self.addPoint(pointVector.clone(), x.clone(), y.clone());
        }
        self.addSeparator(pointVector.clone());
    }
    

    // throws Exception
    pub fn addRectangle(&mut self, mut pointVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut rectangle: Option<Rc<RefCell<JSONObject>>>) {
        let mut x: i32 = self.readInt3(rectangle.clone(), self.KEY_X.clone(), 0);
        let mut y: i32 = self.readInt3(rectangle.clone(), self.KEY_Y.clone(), 0);
        let mut width: i32 = self.readInt(rectangle.clone(), jcall!(self.graphicsStrings).WIDTH.clone());
        let mut height: i32 = self.readInt(rectangle.clone(), jcall!(self.graphicsStrings).HEIGHT.clone());
        self.addPoint(pointVector.clone(), x.clone(), y.clone());
        self.addPoint(pointVector.clone(), x + width, y.clone());
        self.addPoint(pointVector.clone(), x + width, y + height);
        self.addPoint(pointVector.clone(), x.clone(), y + height);
        self.addPoint(pointVector.clone(), x.clone(), y.clone());
        self.addSeparator(pointVector.clone());
    }
    

    pub fn addPoint(&mut self, mut pointVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut x: i32, mut y: i32) {
        jcall!(pointVector).add(crate::java_into_object(vec![x.clone(), y.clone()]));
    }
    

    pub fn addSeparator(&mut self, mut pointVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        jcall!(pointVector).add(crate::java_into_object(vec![self.SHAPE_SEPARATOR[(0) as usize].clone(), self.SHAPE_SEPARATOR[(1) as usize].clone()]));
    }
    

    // throws Exception
    pub fn readInt(&mut self, mut anyType: Option<Rc<RefCell<JSONObject>>>, mut key: String) -> i32 {
        let mut value: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(anyType).get(key.clone());
        if value.is_none() {
            panic!("{:?}", format!("{}: {:?}", "IllegalArgumentException", (format!("{}{}", self.ERROR_MISSING_KEY_PREFIX, key))));
        }
        return self.toInt(value.clone(), key.clone());
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn readInt3(&mut self, mut anyType: Option<Rc<RefCell<JSONObject>>>, mut key: String, mut defaultValue: i32) -> i32 {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(anyType).has(key.clone()) {
                let mut value: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(anyType).get(key.clone());
                return self.toInt(value.clone(), key.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            eprintln!("{:?}", e);
            }
        }
        return defaultValue.clone();
    }
    

    pub fn toInt(&mut self, mut value: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut key: String) -> i32 {
        if (value).is_some() /* instanceof Number */ {
            return ((((((((value).clone() /* as Number */)).unwrap() as f64)) as f64).round() as i64) as i32);
        }
        if (value).is_some() /* instanceof String */ {
            return (((((value).clone() /* as String */.parse::<f64>().unwrap()) as f64).round() as i64) as i32);
        }
        panic!("{:?}", format!("{}: {:?}", "IllegalArgumentException", (format!("{}{}", self.ERROR_INVALID_NUMBER_PREFIX, key))));
    }
    

    pub fn asObject(&mut self, mut value: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut name: String) -> Option<Rc<RefCell<JSONObject>>> {
        if (value).is_some() /* instanceof JSONObject */ {
            return (value).clone() /* as JSONObject */;
        }
        panic!("{:?}", format!("{}: {:?}", "IllegalArgumentException", (format!("{}{}", self.ERROR_EXPECTED_OBJECT_PREFIX, name))));
    }
    

    pub fn toPoints(&mut self, mut pointVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Vec<Vec<i32>> {
        if jcall!(pointVector).size() == 0 {
            return self.EMPTY_POINTS.clone();
        }
        let mut points: Vec<Vec<i32>> = vec![vec![0; (2) as usize]; (jcall!(pointVector).size()) as usize];
        for index in (0..jcall!(pointVector).size()) {
            let mut point: Vec<i32> = (jcall!(pointVector).get(index.clone())).clone() /* as  */;
            points[(index) as usize] = point.clone();
        }
        return points.clone();
    }
    

    // throws Exception
    pub fn main(mut args: Vec<String>) {
        let mut svgAsJson: String = format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", format!("{}{}", String::from("{\"svg\": {\n"), String::from("    \"rect\": {\n")), String::from("        \"width\": 20,\n")), String::from("        \"fill\": \"#008c00\",\n")), String::from("        \"height\": 20\n")), String::from("    },\n")), String::from("    \"xmlns\": \"http://www.w3.org/2000/svg\",\n")), String::from("    \"width\": 20,\n")), String::from("    \"height\": 20\n")), String::from("}}\n"));
        let mut points: Vec<Vec<i32>> = jcall!(SVGAsJSONSimpleLoader::getInstance()).load(svgAsJson.clone());
        let mut stringBuilder: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut size: i32 = (points.len() as i32).clone();
        for index in (0..size) {
            let mut size2: i32 = (points[(index) as usize].len() as i32).clone();
            for index2 in (0..size2) {
                jcall!(stringBuilder).appendint(((points[(index) as usize][(index2) as usize]) as i32).clone());
                jcall!(stringBuilder).append(jcall!(CommonSeps::getInstance()).COMMA.clone());
            }
        }
        println!("{}", jcall!(stringBuilder).toString());
    }
}

