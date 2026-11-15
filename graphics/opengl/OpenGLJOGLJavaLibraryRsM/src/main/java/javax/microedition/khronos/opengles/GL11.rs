// Generated Code Do Not Modify
// package javax.microedition.khronos.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::GL10::GL10;
use super::Buffer::Buffer;
use super::FloatBuffer::FloatBuffer;
use super::IntBuffer::IntBuffer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait GL11: GL10 {
    

    fn glBindBuffer(&mut self, target: i32, buffer: i32);
    

    fn glBufferData(&mut self, target: i32, size: i32, data: Option<Rc<RefCell<Buffer>>>, usage: i32);
    

    fn glBufferSubData(&mut self, target: i32, offset: i32, size: i32, data: Option<Rc<RefCell<Buffer>>>);
    

    fn glClipPlanef3(&mut self, plane: i32, equation: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glClipPlanef(&mut self, plane: i32, equation: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glColor4ub(&mut self, red: i8, green: i8, blue: i8, alpha: i8);
    

    fn glColorPointer(&mut self, size: i32, r#type: i32, stride: i32, offset: i32);
    

    fn glDeleteBuffers3(&mut self, n: i32, buffers: Vec<i32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glDeleteBuffers(&mut self, n: i32, buffers: Option<Rc<RefCell<IntBuffer>>>);
    

    fn glDrawElements(&mut self, mode: i32, count: i32, r#type: i32, offset: i32);
    

    fn glGenBuffers3(&mut self, n: i32, buffers: Vec<i32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glGenBuffers(&mut self, n: i32, buffers: Option<Rc<RefCell<IntBuffer>>>);
    

    fn glGetBufferParameteriv4(&mut self, target: i32, pname: i32, params: Vec<i32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glGetBufferParameteriv(&mut self, target: i32, pname: i32, params: Option<Rc<RefCell<IntBuffer>>>);
    

    fn glGetClipPlanef3(&mut self, pname: i32, eqn: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glGetClipPlanef(&mut self, pname: i32, eqn: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glGetFloatv3(&mut self, pname: i32, params: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glGetFloatv(&mut self, pname: i32, params: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glGetLightfv4(&mut self, light: i32, pname: i32, params: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glGetLightfv(&mut self, light: i32, pname: i32, params: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glGetMaterialfv4(&mut self, face: i32, pname: i32, params: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glGetMaterialfv(&mut self, face: i32, pname: i32, params: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glGetTexEnviv4(&mut self, env: i32, pname: i32, params: Vec<i32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glGetTexEnviv(&mut self, env: i32, pname: i32, params: Option<Rc<RefCell<IntBuffer>>>);
    

    fn glGetTexParameterfv4(&mut self, target: i32, pname: i32, params: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glGetTexParameterfv(&mut self, target: i32, pname: i32, params: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glGetTexParameteriv4(&mut self, target: i32, pname: i32, params: Vec<i32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glGetTexParameteriv(&mut self, target: i32, pname: i32, params: Option<Rc<RefCell<IntBuffer>>>);
    

    fn glIsBuffer(&mut self, buffer: i32) -> bool;
    

    fn glIsEnabled(&mut self, cap: i32) -> bool;
    

    fn glIsTexture(&mut self, texture: i32) -> bool;
    

    fn glNormalPointer(&mut self, r#type: i32, stride: i32, offset: i32);
    

    fn glPointParameterf(&mut self, pname: i32, param: f32);
    

    fn glPointParameterfv3(&mut self, pname: i32, params: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glPointParameterfv(&mut self, pname: i32, params: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glTexCoordPointer(&mut self, size: i32, r#type: i32, stride: i32, offset: i32);
    

    fn glTexEnvi(&mut self, target: i32, pname: i32, param: i32);
    

    fn glTexEnviv4(&mut self, target: i32, pname: i32, params: Vec<i32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glTexEnviv(&mut self, target: i32, pname: i32, params: Option<Rc<RefCell<IntBuffer>>>);
    

    fn glTexParameterfv4(&mut self, target: i32, pname: i32, params: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glTexParameterfv(&mut self, target: i32, pname: i32, params: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glTexParameteri(&mut self, target: i32, pname: i32, param: i32);
    

    fn glTexParameteriv4(&mut self, target: i32, pname: i32, params: Vec<i32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glTexParameteriv(&mut self, target: i32, pname: i32, params: Option<Rc<RefCell<IntBuffer>>>);
    

    fn glVertexPointer(&mut self, size: i32, r#type: i32, stride: i32, offset: i32);
}

pub static GL11_GL_ACTIVE_TEXTURE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34016);
pub static GL11_GL_ADD_SIGNED: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34164);
pub static GL11_GL_ALPHA_SCALE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3356);
pub static GL11_GL_ALPHA_TEST_FUNC: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3009);
pub static GL11_GL_ALPHA_TEST_REF: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3010);
pub static GL11_GL_ARRAY_BUFFER: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34962);
pub static GL11_GL_ARRAY_BUFFER_BINDING: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34964);
pub static GL11_GL_BLEND_DST: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3040);
pub static GL11_GL_BLEND_SRC: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3041);
pub static GL11_GL_BUFFER_ACCESS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35003);
pub static GL11_GL_BUFFER_SIZE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34660);
pub static GL11_GL_BUFFER_USAGE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34661);
pub static GL11_GL_CLIENT_ACTIVE_TEXTURE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34017);
pub static GL11_GL_CLIP_PLANE0: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 12288);
pub static GL11_GL_CLIP_PLANE1: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 12289);
pub static GL11_GL_CLIP_PLANE2: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 12290);
pub static GL11_GL_CLIP_PLANE3: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 12291);
pub static GL11_GL_CLIP_PLANE4: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 12292);
pub static GL11_GL_CLIP_PLANE5: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 12293);
pub static GL11_GL_COLOR_ARRAY_BUFFER_BINDING: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34968);
pub static GL11_GL_COLOR_ARRAY_POINTER: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32912);
pub static GL11_GL_COLOR_ARRAY_SIZE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32897);
pub static GL11_GL_COLOR_ARRAY_STRIDE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32899);
pub static GL11_GL_COLOR_ARRAY_TYPE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32898);
pub static GL11_GL_COLOR_CLEAR_VALUE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3106);
pub static GL11_GL_COLOR_WRITEMASK: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3107);
pub static GL11_GL_COMBINE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34160);
pub static GL11_GL_COMBINE_ALPHA: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34162);
pub static GL11_GL_COMBINE_RGB: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34161);
pub static GL11_GL_CONSTANT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34166);
pub static GL11_GL_COORD_REPLACE_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34914);
pub static GL11_GL_CULL_FACE_MODE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2885);
pub static GL11_GL_CURRENT_COLOR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2816);
pub static GL11_GL_CURRENT_NORMAL: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2818);
pub static GL11_GL_CURRENT_TEXTURE_COORDS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2819);
pub static GL11_GL_DEPTH_CLEAR_VALUE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2931);
pub static GL11_GL_DEPTH_FUNC: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2932);
pub static GL11_GL_DEPTH_RANGE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2928);
pub static GL11_GL_DEPTH_WRITEMASK: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2930);
pub static GL11_GL_DOT3_RGB: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34478);
pub static GL11_GL_DOT3_RGBA: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34479);
pub static GL11_GL_DYNAMIC_DRAW: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35048);
pub static GL11_GL_ELEMENT_ARRAY_BUFFER: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34963);
pub static GL11_GL_ELEMENT_ARRAY_BUFFER_BINDING: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34965);
pub static GL11_GL_FRONT_FACE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2886);
pub static GL11_GL_GENERATE_MIPMAP: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33169);
pub static GL11_GL_GENERATE_MIPMAP_HINT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33170);
pub static GL11_GL_INTERPOLATE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34165);
pub static GL11_GL_LINE_WIDTH: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2849);
pub static GL11_GL_LOGIC_OP_MODE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3056);
pub static GL11_GL_MATRIX_MODE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2976);
pub static GL11_GL_MAX_CLIP_PLANES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3378);
pub static GL11_GL_MODELVIEW_MATRIX: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2982);
pub static GL11_GL_MODELVIEW_MATRIX_FLOAT_AS_INT_BITS_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35213);
pub static GL11_GL_MODELVIEW_STACK_DEPTH: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2979);
pub static GL11_GL_NORMAL_ARRAY_BUFFER_BINDING: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34967);
pub static GL11_GL_NORMAL_ARRAY_POINTER: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32911);
pub static GL11_GL_NORMAL_ARRAY_STRIDE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32895);
pub static GL11_GL_NORMAL_ARRAY_TYPE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32894);
pub static GL11_GL_OPERAND0_ALPHA: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34200);
pub static GL11_GL_OPERAND0_RGB: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34192);
pub static GL11_GL_OPERAND1_ALPHA: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34201);
pub static GL11_GL_OPERAND1_RGB: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34193);
pub static GL11_GL_OPERAND2_ALPHA: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34202);
pub static GL11_GL_OPERAND2_RGB: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34194);
pub static GL11_GL_POINT_DISTANCE_ATTENUATION: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33065);
pub static GL11_GL_POINT_FADE_THRESHOLD_SIZE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33064);
pub static GL11_GL_POINT_SIZE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2833);
pub static GL11_GL_POINT_SIZE_ARRAY_BUFFER_BINDING_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35743);
pub static GL11_GL_POINT_SIZE_ARRAY_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35740);
pub static GL11_GL_POINT_SIZE_ARRAY_POINTER_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35212);
pub static GL11_GL_POINT_SIZE_ARRAY_STRIDE_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35211);
pub static GL11_GL_POINT_SIZE_ARRAY_TYPE_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35210);
pub static GL11_GL_POINT_SIZE_MAX: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33063);
pub static GL11_GL_POINT_SIZE_MIN: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33062);
pub static GL11_GL_POINT_SPRITE_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34913);
pub static GL11_GL_POLYGON_OFFSET_FACTOR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32824);
pub static GL11_GL_POLYGON_OFFSET_UNITS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 10752);
pub static GL11_GL_PREVIOUS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34168);
pub static GL11_GL_PRIMARY_COLOR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34167);
pub static GL11_GL_PROJECTION_MATRIX: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2983);
pub static GL11_GL_PROJECTION_MATRIX_FLOAT_AS_INT_BITS_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35214);
pub static GL11_GL_PROJECTION_STACK_DEPTH: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2980);
pub static GL11_GL_RGB_SCALE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34163);
pub static GL11_GL_SAMPLE_BUFFERS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32936);
pub static GL11_GL_SAMPLE_COVERAGE_INVERT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32939);
pub static GL11_GL_SAMPLE_COVERAGE_VALUE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32938);
pub static GL11_GL_SAMPLES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32937);
pub static GL11_GL_SCISSOR_BOX: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3088);
pub static GL11_GL_SHADE_MODEL: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2900);
pub static GL11_GL_SRC0_ALPHA: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34184);
pub static GL11_GL_SRC0_RGB: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34176);
pub static GL11_GL_SRC1_ALPHA: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34185);
pub static GL11_GL_SRC1_RGB: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34177);
pub static GL11_GL_SRC2_ALPHA: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34186);
pub static GL11_GL_SRC2_RGB: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34178);
pub static GL11_GL_STATIC_DRAW: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35044);
pub static GL11_GL_STENCIL_CLEAR_VALUE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2961);
pub static GL11_GL_STENCIL_FAIL: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2964);
pub static GL11_GL_STENCIL_FUNC: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2962);
pub static GL11_GL_STENCIL_PASS_DEPTH_FAIL: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2965);
pub static GL11_GL_STENCIL_PASS_DEPTH_PASS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2966);
pub static GL11_GL_STENCIL_REF: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2967);
pub static GL11_GL_STENCIL_VALUE_MASK: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2963);
pub static GL11_GL_STENCIL_WRITEMASK: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2968);
pub static GL11_GL_SUBTRACT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34023);
pub static GL11_GL_TEXTURE_BINDING_2D: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32873);
pub static GL11_GL_TEXTURE_COORD_ARRAY_BUFFER_BINDING: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34970);
pub static GL11_GL_TEXTURE_COORD_ARRAY_POINTER: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32914);
pub static GL11_GL_TEXTURE_COORD_ARRAY_SIZE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32904);
pub static GL11_GL_TEXTURE_COORD_ARRAY_STRIDE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32906);
pub static GL11_GL_TEXTURE_COORD_ARRAY_TYPE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32905);
pub static GL11_GL_TEXTURE_MATRIX: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2984);
pub static GL11_GL_TEXTURE_MATRIX_FLOAT_AS_INT_BITS_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35215);
pub static GL11_GL_TEXTURE_STACK_DEPTH: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2981);
pub static GL11_GL_VERTEX_ARRAY_BUFFER_BINDING: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34966);
pub static GL11_GL_VERTEX_ARRAY_POINTER: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32910);
pub static GL11_GL_VERTEX_ARRAY_SIZE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32890);
pub static GL11_GL_VERTEX_ARRAY_STRIDE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32892);
pub static GL11_GL_VERTEX_ARRAY_TYPE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32891);
pub static GL11_GL_VIEWPORT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2978);
pub static GL11_GL_WRITE_ONLY: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35001);

