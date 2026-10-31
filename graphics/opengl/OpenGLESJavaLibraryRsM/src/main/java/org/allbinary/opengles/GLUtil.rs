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
// package org.allbinary.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::nio::Buffer::Buffer;
// use crate::java::nio::ByteBuffer::ByteBuffer;
// use crate::java::nio::ByteOrder::ByteOrder;
// use crate::java::nio::FloatBuffer::FloatBuffer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GLUtil {
    pub BYTES_PER_FLOAT: i32,
}

thread_local! { pub static GLUtil_instance: Option<Rc<RefCell<GLUtil>>> = GLUtil::new(); }

impl GLUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            BYTES_PER_FLOAT: 0,
        };
        this.BYTES_PER_FLOAT = 4;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GLUtil>>> {
        return GLUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn makeFloatBuffer3(&mut self, mut p_a: f32, mut p_b: f32, mut p_c: f32) -> Option<Rc<RefCell<FloatBuffer>>> {
        let mut b: Option<Rc<RefCell<ByteBuffer>>> = ByteBuffer::allocateDirect(3 * self.BYTES_PER_FLOAT);
        jcall!(b).order(ByteOrder::nativeOrder());
        let mut buffer: Option<Rc<RefCell<FloatBuffer>>> = jcall!(b).asFloatBuffer();
        jcall!(buffer).put(p_a.clone());
        jcall!(buffer).put(p_b.clone());
        jcall!(buffer).put(p_c.clone());
        self.position(buffer.clone(), 0);
        return buffer.clone();
    }
    

    pub fn makeFloatBuffer4(&mut self, mut p_a: f32, mut p_b: f32, mut p_c: f32, mut p_d: f32) -> Option<Rc<RefCell<FloatBuffer>>> {
        let mut b: Option<Rc<RefCell<ByteBuffer>>> = ByteBuffer::allocateDirect(4 * self.BYTES_PER_FLOAT);
        jcall!(b).order(ByteOrder::nativeOrder());
        let mut buffer: Option<Rc<RefCell<FloatBuffer>>> = jcall!(b).asFloatBuffer();
        jcall!(buffer).put(p_a.clone());
        jcall!(buffer).put(p_b.clone());
        jcall!(buffer).put(p_c.clone());
        jcall!(buffer).put(p_d.clone());
        self.position(buffer.clone(), 0);
        return buffer.clone();
    }
    

    pub fn makeFloatBuffer(&mut self, mut byteBuffer: Option<Rc<RefCell<ByteBuffer>>>) -> Option<Rc<RefCell<FloatBuffer>>> {
        let mut b: Option<Rc<RefCell<ByteBuffer>>> = ByteBuffer::allocateDirect(jcall!(byteBuffer).limit() * self.BYTES_PER_FLOAT);
        jcall!(b).order(ByteOrder::nativeOrder());
        let mut buffer: Option<Rc<RefCell<FloatBuffer>>> = jcall!(b).asFloatBuffer();
        let mut unsigned: i32 = 0;
        let mut v: f32 = 0.0_f32;
        while jcall!(byteBuffer).hasRemaining() {
            unsigned = jcall!(byteBuffer).get() & 0xFF_u32 as i32;
            v = (((unsigned) as f32)) / 255.0_f32;
            jcall!(buffer).put(v.clone());
        }
        self.position(buffer.clone(), 0);
        return buffer.clone();
    }
    

    pub fn position(&mut self, mut buffer: Option<Rc<RefCell<Buffer>>>, mut newPosition: i32) -> Option<Rc<RefCell<Buffer>>> {
        return jcall!(buffer).position(newPosition.clone());
    }
    

    pub fn rotateUVs(&mut self, mut uvs: Vec<f32>, mut angleDegrees: f32, mut pivotU: f32, mut pivotV: f32) {
        let mut angleRad: f64 = ((angleDegrees.clone()) as f64).to_radians();
        let mut cosA: f64 = ((angleRad.clone()) as f64).cos();
        let mut sinA: f64 = ((angleRad.clone()) as f64).sin();
        for i in (0..(uvs.len() as i32)).step_by((2) as usize) {
            let mut u: f32 = uvs[(i) as usize].clone();
            let mut v: f32 = uvs[(i + 1) as usize].clone();
            let mut du: f32 = u - pivotU;
            let mut dv: f32 = v - pivotV;
            let mut ru: f32 = (((((du) as f64) * cosA - ((dv) as f64) * sinA)) as f32) + pivotU;
            let mut rv: f32 = (((((du) as f64) * sinA + ((dv) as f64) * cosA)) as f32) + pivotV;
            uvs[(i) as usize] = ru.clone();
            uvs[(i + 1) as usize] = rv.clone();
        }
    }
}

