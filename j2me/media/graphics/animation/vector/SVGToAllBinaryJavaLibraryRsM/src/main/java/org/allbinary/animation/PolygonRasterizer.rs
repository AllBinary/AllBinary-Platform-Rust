// Generated Code Do Not Modify
// package org.allbinary.animation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::FixedPoint::FixedPoint;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PolygonRasterizer {
}

thread_local! { pub static PolygonRasterizer_instance: Option<Rc<RefCell<PolygonRasterizer>>> = PolygonRasterizer::new(); }

impl PolygonRasterizer {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PolygonRasterizer>>> {
        return PolygonRasterizer_instance.with(Clone::clone).clone();
    }
    

    pub fn getPixelBuffer(&mut self, mut svgPath: String, mut pathX: Vec<i32>, mut pathY: Vec<i32>, mut pixelBuffer: Vec<i32>, mut width: i32, mut height: i32) -> Vec<i32> {
        let mut pointCount: i32 = PathParser::PathParser::parsePath(svgPath.clone(), pathX.clone(), pathY.clone(), (pathX.len() as i32).clone());
        for i in (0..(pixelBuffer.len() as i32)) {
            pixelBuffer[(i) as usize] = 0xFFFFFFFF_u32 as i32;
        }
        jcall!(PolygonRasterizer::getInstance()).fillPolygon(pathX.clone(), pathY.clone(), pointCount.clone(), pixelBuffer.clone(), width.clone(), height.clone(), 0xFFFF00FF_u32 as i32);
        return pixelBuffer.clone();
    }
    

    pub fn fillPolygon(&mut self, mut polyX: Vec<i32>, mut polyY: Vec<i32>, mut numPoints: i32, mut buffer: Vec<i32>, mut width: i32, mut height: i32, mut colorRGB: i32) {
        if numPoints < 3 {
            return;
        }
        let mut fixedPoint: Option<Rc<RefCell<FixedPoint>>> = FixedPoint::getInstance();
        let mut x: Vec<i32> = vec![0; (numPoints) as usize];
        let mut y: Vec<i32> = vec![0; (numPoints) as usize];
        let mut minY: i32 = height.clone();
        let mut maxY: i32 = 0;
        for i in (0..numPoints) {
            x[(i) as usize] = jcall!(fixedPoint).toInt(polyX[(i) as usize].clone());
            y[(i) as usize] = jcall!(fixedPoint).toInt(polyY[(i) as usize].clone());
            if y[(i) as usize] < minY {
                minY = y[(i) as usize].clone();
            }
            if y[(i) as usize] > maxY {
                maxY = y[(i) as usize].clone();
            }
        }
        if minY < 0 {
            minY = 0;
        }
        if maxY >= height {
            maxY = height - 1;
        }
        let mut scanlineX: Vec<i32> = vec![0; (numPoints) as usize];
        for scanY in (minY..=maxY) {
            let mut intersectCount: i32 = 0;
            for i in (0..numPoints) {
                let mut next: i32 = (i + 1) % numPoints;
                let mut x1: i32 = x[(i) as usize].clone();
                let mut y1: i32 = y[(i) as usize].clone();
                let mut x2: i32 = x[(next) as usize].clone();
                let mut y2: i32 = y[(next) as usize].clone();
                if (y1 < scanY && y2 >= scanY) || (y2 < scanY && y1 >= scanY) {
                    if y2 != y1 {
                        let mut interX: i32 = x1 + (scanY - y1) * (x2 - x1) / (y2 - y1);
                        scanlineX[({ let value = intersectCount; intersectCount += 1; value }) as usize] = interX.clone();
                    }
                }
            }
            for i in (0..intersectCount - 1) {
                for j in (i + 1..intersectCount) {
                    if scanlineX[(i) as usize] > scanlineX[(j) as usize] {
                        let mut temp: i32 = scanlineX[(i) as usize].clone();
                        scanlineX[(i) as usize] = scanlineX[(j) as usize].clone();
                        scanlineX[(j) as usize] = temp.clone();
                    }
                }
            }
            for i in (0..intersectCount).step_by((2) as usize) {
                if i + 1 >= intersectCount {
                    break;
                }
                let mut startX: i32 = scanlineX[(i) as usize].clone();
                let mut endX: i32 = scanlineX[(i + 1) as usize].clone();
                if startX < 0 {
                    startX = 0;
                }
                if endX >= width {
                    endX = width - 1;
                }
                let mut rowOffset: i32 = scanY * width;
                for pixelX in (startX..=endX) {
                    buffer[(rowOffset + pixelX) as usize] = colorRGB.clone();
                }
            }
        }
    }
    

    pub fn main(&mut self, mut args: Vec<String>) {
        let mut width: i32 = 100;
        let mut height: i32 = 100;
        let mut pathX: Vec<i32> = vec![0; (200) as usize];
        let mut pathY: Vec<i32> = vec![0; (200) as usize];
        let mut pixelBuffer: Vec<i32> = vec![0; (width * height) as usize];
        let mut svgPath: String = String::from("M 32.139812,81.554575 14.450128,74.294372 V 50.000034 25.705697 l 17.73231,-7.268329 C 41.935209,14.439788 49.953112,11.168898 50,11.168723 c 0.04689,-1.74e-4 8.064791,3.270238 17.817562,7.267583 l 17.73231,7.267899 V 50 74.295795 l -17.73231,7.2679 c -9.752771,3.997344 -17.789855,7.264116 -17.860188,7.259492 -0.07033,-0.0046 -8.088235,-3.275499 -17.817562,-7.268612 z");
        jcall!(PolygonRasterizer::getInstance()).getPixelBuffer(svgPath.clone(), pathX.clone(), pathY.clone(), pixelBuffer.clone(), width.clone(), height.clone());
    }
}

