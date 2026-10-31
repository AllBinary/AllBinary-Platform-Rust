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
// package org.allbinary.logic.visual.dhtml.style.color
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ColorsView {
    pub commonPhoneStrings: Option<Rc<RefCell<CommonPhoneStrings>>>,
    pub HEXRADIX: i32,
    pub MAX: i32,
    pub delta: i32,
}

impl DomNodeInterface for ColorsView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl ColorsView {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonPhoneStrings: None,
            HEXRADIX: 0,
            MAX: 0,
            delta: 0,
        };
        this.commonPhoneStrings = CommonPhoneStrings::getInstance();
        this.HEXRADIX = 16;
        this.MAX = 255;
        this.delta = 20;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getColorNode(&mut self, mut document: Option<Rc<RefCell<Document>>>, mut redIndex: i32, mut greenIndex: i32, mut blueIndex: i32) -> Option<Rc<RefCell<Node>>> {
        let mut hexColorStringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        if redIndex < self.HEXRADIX {
            jcall!(hexColorStringBuffer).append(jcall!(self.commonPhoneStrings).ZERO.clone());
        }
        jcall!(hexColorStringBuffer).append(Integer::Integer::toString(redIndex.clone(), self.HEXRADIX.clone()));
        if greenIndex < self.HEXRADIX {
            jcall!(hexColorStringBuffer).append(jcall!(self.commonPhoneStrings).ZERO.clone());
        }
        jcall!(hexColorStringBuffer).append(Integer::Integer::toString(greenIndex.clone(), self.HEXRADIX.clone()));
        if blueIndex < self.HEXRADIX {
            jcall!(hexColorStringBuffer).append(jcall!(self.commonPhoneStrings).ZERO.clone());
        }
        jcall!(hexColorStringBuffer).append(Integer::Integer::toString(blueIndex.clone(), self.HEXRADIX.clone()));
        let mut colorNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), jcall!(ColorData::ColorData::getInstance()).NAME.clone(), jcall!(hexColorStringBuffer).toString());
        return colorNode.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), jcall!(ColorsData::ColorsData::getInstance()).NAME.clone(), String::from("RGB Colors In Hex"));
        let mut blackNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), jcall!(ColorData::ColorData::getInstance()).NAME.clone(), String::from("000000"));
        jcall!(node).appendChild(blackNode.clone());
        for index in (0..self.MAX).step_by((self.delta) as usize) {
            let mut hexColorStringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            if index < self.HEXRADIX {
                jcall!(hexColorStringBuffer).append(jcall!(self.commonPhoneStrings).ZERO.clone());
            }
            jcall!(hexColorStringBuffer).append(Integer::Integer::toString(index.clone(), self.HEXRADIX.clone()));
            if index < self.HEXRADIX {
                jcall!(hexColorStringBuffer).append(jcall!(self.commonPhoneStrings).ZERO.clone());
            }
            jcall!(hexColorStringBuffer).append(Integer::Integer::toString(index.clone(), self.HEXRADIX.clone()));
            if index < self.HEXRADIX {
                jcall!(hexColorStringBuffer).append(jcall!(self.commonPhoneStrings).ZERO.clone());
            }
            jcall!(hexColorStringBuffer).append(Integer::Integer::toString(index.clone(), self.HEXRADIX.clone()));
            let mut colorNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), jcall!(ColorData::ColorData::getInstance()).NAME.clone(), jcall!(hexColorStringBuffer).toString());
            jcall!(node).appendChild(colorNode.clone());
        }
        let mut whiteNode: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes(document.clone(), jcall!(ColorData::ColorData::getInstance()).NAME.clone(), String::from("FFFFFF"));
        jcall!(node).appendChild(whiteNode.clone());
        for blueIndex in (self.MAX / 2..self.MAX).step_by((self.delta) as usize) {
            for greenIndex in (0..self.MAX / 2).step_by((self.delta) as usize) {
                for redIndex in (0..self.MAX / 2).step_by((self.delta) as usize) {
                    jcall!(node).appendChild(self.getColorNode(document.clone(), redIndex.clone(), greenIndex.clone(), blueIndex.clone()));
                }
            }
        }
        for redIndex in (self.MAX / 2..self.MAX).step_by((self.delta) as usize) {
            for greenIndex in (0..self.MAX / 2).step_by((self.delta) as usize) {
                for blueIndex in (0..self.MAX / 2).step_by((self.delta) as usize) {
                    jcall!(node).appendChild(self.getColorNode(document.clone(), redIndex.clone(), greenIndex.clone(), blueIndex.clone()));
                }
            }
        }
        for greenIndex in (self.MAX / 2..self.MAX).step_by((self.delta) as usize) {
            for redIndex in (0..self.MAX / 2).step_by((self.delta) as usize) {
                for blueIndex in (0..self.MAX / 2).step_by((self.delta) as usize) {
                    jcall!(node).appendChild(self.getColorNode(document.clone(), redIndex.clone(), greenIndex.clone(), blueIndex.clone()));
                }
            }
        }
        for greenIndex in (0..self.MAX).step_by((2 * self.delta) as usize) {
            for redIndex in (0..self.MAX).step_by((2 * self.delta) as usize) {
                for blueIndex in (0..self.MAX).step_by((2 * self.delta) as usize) {
                    jcall!(node).appendChild(self.getColorNode(document.clone(), redIndex.clone(), greenIndex.clone(), blueIndex.clone()));
                }
            }
        }
        return node.clone();
    }
}

