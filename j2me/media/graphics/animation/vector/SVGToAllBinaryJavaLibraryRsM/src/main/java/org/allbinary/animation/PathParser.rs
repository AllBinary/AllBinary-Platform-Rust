// Generated Code Do Not Modify
// package org.allbinary.animation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use super::FixedPoint::FixedPoint;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PathParser {
}


impl PathParser {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn tokenize(mut d: String) -> Vec<String> {
        let mut tokens: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut sb: String = String::new();
        for i in (0..(d.chars().count() as i32)) {
            let mut c: char = d.chars().nth((i.clone()) as usize).unwrap();
            if c == ' ' || c == ',' || c == '\t' || c == '\n' || c == '\r' {
                if jcall!(sb).length() > 0 {
                    jcall!(tokens).add(crate::java_into_object(jcall!(sb).toString()));
                    jcall!(sb).setLength(0);
                }
            } else if c == 'M' || c == 'm' || c == 'L' || c == 'l' || c == 'V' || c == 'v' || c == 'C' || c == 'c' || c == 'Z' || c == 'z' {
                if jcall!(sb).length() > 0 {
                    jcall!(tokens).add(crate::java_into_object(jcall!(sb).toString()));
                    jcall!(sb).setLength(0);
                }
                jcall!(tokens).add(crate::java_into_object((c.clone()).to_string()));
            } else {
                jcall!(sb).append(c.clone());
            }
        }
        if jcall!(sb).length() > 0 {
            jcall!(tokens).add(crate::java_into_object(jcall!(sb).toString()));
        }
        let mut res: Vec<String> = vec![String::new(); (jcall!(tokens).size()) as usize];
        jcall!(tokens).toArrayType(res.clone());
        return res.clone();
    }
    

    pub fn parsePath(mut d: String, mut outX: Vec<i32>, mut outY: Vec<i32>, mut maxPoints: i32) -> i32 {
        let mut tokens: Vec<String> = Self::tokenize(d.clone());
        let mut count: Vec<i32> = vec![0];
        let mut fixedPoint: Option<Rc<RefCell<FixedPoint>>> = FixedPoint::getInstance();
        let mut curX: i32 = 0;
        let mut curY: i32 = 0;
        let mut startX: i32 = 0;
        let mut startY: i32 = 0;
        let mut idx: i32 = 0;
        while idx < (tokens.len() as i32) {
            let mut tok: String = tokens[({ let value = idx; idx += 1; value }) as usize].clone();
            let mut cmd: char = tok.chars().nth((0) as usize).unwrap();
            if cmd == 'M' || cmd == 'm' {
                let mut x: i32 = jcall!(fixedPoint).toFixed(tokens[({ let value = idx; idx += 1; value }) as usize].clone().parse::<f64>().unwrap());
                let mut y: i32 = jcall!(fixedPoint).toFixed(tokens[({ let value = idx; idx += 1; value }) as usize].clone().parse::<f64>().unwrap());
                if cmd == 'm' {
                    x += curX.clone();
                    y += curY.clone();
                }
                curX = x.clone();
                curY = y.clone();
                startX = x.clone();
                startY = y.clone();
                outX[(count[(0) as usize]) as usize] = curX.clone();
                outY[(count[(0) as usize]) as usize] = curY.clone();
                count[(0) as usize] += 1;
            } else if cmd == 'L' || cmd == 'l' {
                let mut x: i32 = jcall!(fixedPoint).toFixed(tokens[({ let value = idx; idx += 1; value }) as usize].clone().parse::<f64>().unwrap());
                let mut y: i32 = jcall!(fixedPoint).toFixed(tokens[({ let value = idx; idx += 1; value }) as usize].clone().parse::<f64>().unwrap());
                if cmd == 'l' {
                    x += curX.clone();
                    y += curY.clone();
                }
                curX = x.clone();
                curY = y.clone();
                outX[(count[(0) as usize]) as usize] = curX.clone();
                outY[(count[(0) as usize]) as usize] = curY.clone();
                count[(0) as usize] += 1;
            } else if cmd == 'V' || cmd == 'v' {
                let mut y: i32 = jcall!(fixedPoint).toFixed(tokens[({ let value = idx; idx += 1; value }) as usize].clone().parse::<f64>().unwrap());
                if cmd == 'v' {
                    y += curY.clone();
                }
                curY = y.clone();
                outX[(count[(0) as usize]) as usize] = curX.clone();
                outY[(count[(0) as usize]) as usize] = curY.clone();
                count[(0) as usize] += 1;
            } else if cmd == 'C' || cmd == 'c' {
                let mut x1: i32 = jcall!(fixedPoint).toFixed(tokens[({ let value = idx; idx += 1; value }) as usize].clone().parse::<f64>().unwrap());
                let mut y1: i32 = jcall!(fixedPoint).toFixed(tokens[({ let value = idx; idx += 1; value }) as usize].clone().parse::<f64>().unwrap());
                let mut x2: i32 = jcall!(fixedPoint).toFixed(tokens[({ let value = idx; idx += 1; value }) as usize].clone().parse::<f64>().unwrap());
                let mut y2: i32 = jcall!(fixedPoint).toFixed(tokens[({ let value = idx; idx += 1; value }) as usize].clone().parse::<f64>().unwrap());
                let mut x3: i32 = jcall!(fixedPoint).toFixed(tokens[({ let value = idx; idx += 1; value }) as usize].clone().parse::<f64>().unwrap());
                let mut y3: i32 = jcall!(fixedPoint).toFixed(tokens[({ let value = idx; idx += 1; value }) as usize].clone().parse::<f64>().unwrap());
                if cmd == 'c' {
                    x1 += curX.clone();
                    y1 += curY.clone();
                    x2 += curX.clone();
                    y2 += curY.clone();
                    x3 += curX.clone();
                    y3 += curY.clone();
                }
                jcall!(PGUtil::PGUtil::getInstance()).flattenCubicBezier(curX.clone(), curY.clone(), x1.clone(), y1.clone(), x2.clone(), y2.clone(), x3.clone(), y3.clone(), outX.clone(), outY.clone(), count.clone(), maxPoints.clone());
                curX = x3.clone();
                curY = y3.clone();
            } else if cmd == 'Z' || cmd == 'z' {
                curX = startX.clone();
                curY = startY.clone();
                outX[(count[(0) as usize]) as usize] = curX.clone();
                outY[(count[(0) as usize]) as usize] = curY.clone();
                count[(0) as usize] += 1;
            }
            while idx < (tokens.len() as i32) && !Self::isCommand(tokens[(idx) as usize].clone()) {
                if cmd == 'M' || cmd == 'm' || cmd == 'L' || cmd == 'l' {
                    let mut x: i32 = jcall!(fixedPoint).toFixed(tokens[({ let value = idx; idx += 1; value }) as usize].clone().parse::<f64>().unwrap());
                    let mut y: i32 = jcall!(fixedPoint).toFixed(tokens[({ let value = idx; idx += 1; value }) as usize].clone().parse::<f64>().unwrap());
                    if cmd == 'm' || cmd == 'l' {
                        x += curX.clone();
                        y += curY.clone();
                    }
                    curX = x.clone();
                    curY = y.clone();
                    outX[(count[(0) as usize]) as usize] = curX.clone();
                    outY[(count[(0) as usize]) as usize] = curY.clone();
                    count[(0) as usize] += 1;
                } else if cmd == 'V' || cmd == 'v' {
                    let mut y: i32 = jcall!(fixedPoint).toFixed(tokens[({ let value = idx; idx += 1; value }) as usize].clone().parse::<f64>().unwrap());
                    if cmd == 'v' {
                        y += curY.clone();
                    }
                    curY = y.clone();
                    outX[(count[(0) as usize]) as usize] = curX.clone();
                    outY[(count[(0) as usize]) as usize] = curY.clone();
                    count[(0) as usize] += 1;
                }
            }
        }
        return count[(0) as usize].clone();
    }
    

    pub fn isCommand(mut tok: String) -> bool {
        let mut c: char = tok.chars().nth((0) as usize).unwrap();
        return String::from("MmLlVvCcZz").find(c.clone()).map(|index| index as i32).unwrap_or(-1) >= 0;
    }
}

