// Generated Code Do Not Modify
// package javax.microedition.khronos.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::GL10::GL10;
use super::opengles::opengles;
use super::khronos::khronos;
use super::microedition::microedition;
use super::javax::javax;
use super::Buffer::Buffer;
use super::nio::nio;
use super::java::java;
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

thread_local! { pub static GL11_GL_ACTIVE_TEXTURE: i32 = 34016; }
thread_local! { pub static GL11_GL_ADD_SIGNED: i32 = 34164; }
thread_local! { pub static GL11_GL_ALPHA_SCALE: i32 = 3356; }
thread_local! { pub static GL11_GL_ALPHA_TEST_FUNC: i32 = 3009; }
thread_local! { pub static GL11_GL_ALPHA_TEST_REF: i32 = 3010; }
thread_local! { pub static GL11_GL_ARRAY_BUFFER: i32 = 34962; }
thread_local! { pub static GL11_GL_ARRAY_BUFFER_BINDING: i32 = 34964; }
thread_local! { pub static GL11_GL_BLEND_DST: i32 = 3040; }
thread_local! { pub static GL11_GL_BLEND_SRC: i32 = 3041; }
thread_local! { pub static GL11_GL_BUFFER_ACCESS: i32 = 35003; }
thread_local! { pub static GL11_GL_BUFFER_SIZE: i32 = 34660; }
thread_local! { pub static GL11_GL_BUFFER_USAGE: i32 = 34661; }
thread_local! { pub static GL11_GL_CLIENT_ACTIVE_TEXTURE: i32 = 34017; }
thread_local! { pub static GL11_GL_CLIP_PLANE0: i32 = 12288; }
thread_local! { pub static GL11_GL_CLIP_PLANE1: i32 = 12289; }
thread_local! { pub static GL11_GL_CLIP_PLANE2: i32 = 12290; }
thread_local! { pub static GL11_GL_CLIP_PLANE3: i32 = 12291; }
thread_local! { pub static GL11_GL_CLIP_PLANE4: i32 = 12292; }
thread_local! { pub static GL11_GL_CLIP_PLANE5: i32 = 12293; }
thread_local! { pub static GL11_GL_COLOR_ARRAY_BUFFER_BINDING: i32 = 34968; }
thread_local! { pub static GL11_GL_COLOR_ARRAY_POINTER: i32 = 32912; }
thread_local! { pub static GL11_GL_COLOR_ARRAY_SIZE: i32 = 32897; }
thread_local! { pub static GL11_GL_COLOR_ARRAY_STRIDE: i32 = 32899; }
thread_local! { pub static GL11_GL_COLOR_ARRAY_TYPE: i32 = 32898; }
thread_local! { pub static GL11_GL_COLOR_CLEAR_VALUE: i32 = 3106; }
thread_local! { pub static GL11_GL_COLOR_WRITEMASK: i32 = 3107; }
thread_local! { pub static GL11_GL_COMBINE: i32 = 34160; }
thread_local! { pub static GL11_GL_COMBINE_ALPHA: i32 = 34162; }
thread_local! { pub static GL11_GL_COMBINE_RGB: i32 = 34161; }
thread_local! { pub static GL11_GL_CONSTANT: i32 = 34166; }
thread_local! { pub static GL11_GL_COORD_REPLACE_OES: i32 = 34914; }
thread_local! { pub static GL11_GL_CULL_FACE_MODE: i32 = 2885; }
thread_local! { pub static GL11_GL_CURRENT_COLOR: i32 = 2816; }
thread_local! { pub static GL11_GL_CURRENT_NORMAL: i32 = 2818; }
thread_local! { pub static GL11_GL_CURRENT_TEXTURE_COORDS: i32 = 2819; }
thread_local! { pub static GL11_GL_DEPTH_CLEAR_VALUE: i32 = 2931; }
thread_local! { pub static GL11_GL_DEPTH_FUNC: i32 = 2932; }
thread_local! { pub static GL11_GL_DEPTH_RANGE: i32 = 2928; }
thread_local! { pub static GL11_GL_DEPTH_WRITEMASK: i32 = 2930; }
thread_local! { pub static GL11_GL_DOT3_RGB: i32 = 34478; }
thread_local! { pub static GL11_GL_DOT3_RGBA: i32 = 34479; }
thread_local! { pub static GL11_GL_DYNAMIC_DRAW: i32 = 35048; }
thread_local! { pub static GL11_GL_ELEMENT_ARRAY_BUFFER: i32 = 34963; }
thread_local! { pub static GL11_GL_ELEMENT_ARRAY_BUFFER_BINDING: i32 = 34965; }
thread_local! { pub static GL11_GL_FRONT_FACE: i32 = 2886; }
thread_local! { pub static GL11_GL_GENERATE_MIPMAP: i32 = 33169; }
thread_local! { pub static GL11_GL_GENERATE_MIPMAP_HINT: i32 = 33170; }
thread_local! { pub static GL11_GL_INTERPOLATE: i32 = 34165; }
thread_local! { pub static GL11_GL_LINE_WIDTH: i32 = 2849; }
thread_local! { pub static GL11_GL_LOGIC_OP_MODE: i32 = 3056; }
thread_local! { pub static GL11_GL_MATRIX_MODE: i32 = 2976; }
thread_local! { pub static GL11_GL_MAX_CLIP_PLANES: i32 = 3378; }
thread_local! { pub static GL11_GL_MODELVIEW_MATRIX: i32 = 2982; }
thread_local! { pub static GL11_GL_MODELVIEW_MATRIX_FLOAT_AS_INT_BITS_OES: i32 = 35213; }
thread_local! { pub static GL11_GL_MODELVIEW_STACK_DEPTH: i32 = 2979; }
thread_local! { pub static GL11_GL_NORMAL_ARRAY_BUFFER_BINDING: i32 = 34967; }
thread_local! { pub static GL11_GL_NORMAL_ARRAY_POINTER: i32 = 32911; }
thread_local! { pub static GL11_GL_NORMAL_ARRAY_STRIDE: i32 = 32895; }
thread_local! { pub static GL11_GL_NORMAL_ARRAY_TYPE: i32 = 32894; }
thread_local! { pub static GL11_GL_OPERAND0_ALPHA: i32 = 34200; }
thread_local! { pub static GL11_GL_OPERAND0_RGB: i32 = 34192; }
thread_local! { pub static GL11_GL_OPERAND1_ALPHA: i32 = 34201; }
thread_local! { pub static GL11_GL_OPERAND1_RGB: i32 = 34193; }
thread_local! { pub static GL11_GL_OPERAND2_ALPHA: i32 = 34202; }
thread_local! { pub static GL11_GL_OPERAND2_RGB: i32 = 34194; }
thread_local! { pub static GL11_GL_POINT_DISTANCE_ATTENUATION: i32 = 33065; }
thread_local! { pub static GL11_GL_POINT_FADE_THRESHOLD_SIZE: i32 = 33064; }
thread_local! { pub static GL11_GL_POINT_SIZE: i32 = 2833; }
thread_local! { pub static GL11_GL_POINT_SIZE_ARRAY_BUFFER_BINDING_OES: i32 = 35743; }
thread_local! { pub static GL11_GL_POINT_SIZE_ARRAY_OES: i32 = 35740; }
thread_local! { pub static GL11_GL_POINT_SIZE_ARRAY_POINTER_OES: i32 = 35212; }
thread_local! { pub static GL11_GL_POINT_SIZE_ARRAY_STRIDE_OES: i32 = 35211; }
thread_local! { pub static GL11_GL_POINT_SIZE_ARRAY_TYPE_OES: i32 = 35210; }
thread_local! { pub static GL11_GL_POINT_SIZE_MAX: i32 = 33063; }
thread_local! { pub static GL11_GL_POINT_SIZE_MIN: i32 = 33062; }
thread_local! { pub static GL11_GL_POINT_SPRITE_OES: i32 = 34913; }
thread_local! { pub static GL11_GL_POLYGON_OFFSET_FACTOR: i32 = 32824; }
thread_local! { pub static GL11_GL_POLYGON_OFFSET_UNITS: i32 = 10752; }
thread_local! { pub static GL11_GL_PREVIOUS: i32 = 34168; }
thread_local! { pub static GL11_GL_PRIMARY_COLOR: i32 = 34167; }
thread_local! { pub static GL11_GL_PROJECTION_MATRIX: i32 = 2983; }
thread_local! { pub static GL11_GL_PROJECTION_MATRIX_FLOAT_AS_INT_BITS_OES: i32 = 35214; }
thread_local! { pub static GL11_GL_PROJECTION_STACK_DEPTH: i32 = 2980; }
thread_local! { pub static GL11_GL_RGB_SCALE: i32 = 34163; }
thread_local! { pub static GL11_GL_SAMPLE_BUFFERS: i32 = 32936; }
thread_local! { pub static GL11_GL_SAMPLE_COVERAGE_INVERT: i32 = 32939; }
thread_local! { pub static GL11_GL_SAMPLE_COVERAGE_VALUE: i32 = 32938; }
thread_local! { pub static GL11_GL_SAMPLES: i32 = 32937; }
thread_local! { pub static GL11_GL_SCISSOR_BOX: i32 = 3088; }
thread_local! { pub static GL11_GL_SHADE_MODEL: i32 = 2900; }
thread_local! { pub static GL11_GL_SRC0_ALPHA: i32 = 34184; }
thread_local! { pub static GL11_GL_SRC0_RGB: i32 = 34176; }
thread_local! { pub static GL11_GL_SRC1_ALPHA: i32 = 34185; }
thread_local! { pub static GL11_GL_SRC1_RGB: i32 = 34177; }
thread_local! { pub static GL11_GL_SRC2_ALPHA: i32 = 34186; }
thread_local! { pub static GL11_GL_SRC2_RGB: i32 = 34178; }
thread_local! { pub static GL11_GL_STATIC_DRAW: i32 = 35044; }
thread_local! { pub static GL11_GL_STENCIL_CLEAR_VALUE: i32 = 2961; }
thread_local! { pub static GL11_GL_STENCIL_FAIL: i32 = 2964; }
thread_local! { pub static GL11_GL_STENCIL_FUNC: i32 = 2962; }
thread_local! { pub static GL11_GL_STENCIL_PASS_DEPTH_FAIL: i32 = 2965; }
thread_local! { pub static GL11_GL_STENCIL_PASS_DEPTH_PASS: i32 = 2966; }
thread_local! { pub static GL11_GL_STENCIL_REF: i32 = 2967; }
thread_local! { pub static GL11_GL_STENCIL_VALUE_MASK: i32 = 2963; }
thread_local! { pub static GL11_GL_STENCIL_WRITEMASK: i32 = 2968; }
thread_local! { pub static GL11_GL_SUBTRACT: i32 = 34023; }
thread_local! { pub static GL11_GL_TEXTURE_BINDING_2D: i32 = 32873; }
thread_local! { pub static GL11_GL_TEXTURE_COORD_ARRAY_BUFFER_BINDING: i32 = 34970; }
thread_local! { pub static GL11_GL_TEXTURE_COORD_ARRAY_POINTER: i32 = 32914; }
thread_local! { pub static GL11_GL_TEXTURE_COORD_ARRAY_SIZE: i32 = 32904; }
thread_local! { pub static GL11_GL_TEXTURE_COORD_ARRAY_STRIDE: i32 = 32906; }
thread_local! { pub static GL11_GL_TEXTURE_COORD_ARRAY_TYPE: i32 = 32905; }
thread_local! { pub static GL11_GL_TEXTURE_MATRIX: i32 = 2984; }
thread_local! { pub static GL11_GL_TEXTURE_MATRIX_FLOAT_AS_INT_BITS_OES: i32 = 35215; }
thread_local! { pub static GL11_GL_TEXTURE_STACK_DEPTH: i32 = 2981; }
thread_local! { pub static GL11_GL_VERTEX_ARRAY_BUFFER_BINDING: i32 = 34966; }
thread_local! { pub static GL11_GL_VERTEX_ARRAY_POINTER: i32 = 32910; }
thread_local! { pub static GL11_GL_VERTEX_ARRAY_SIZE: i32 = 32890; }
thread_local! { pub static GL11_GL_VERTEX_ARRAY_STRIDE: i32 = 32892; }
thread_local! { pub static GL11_GL_VERTEX_ARRAY_TYPE: i32 = 32891; }
thread_local! { pub static GL11_GL_VIEWPORT: i32 = 2978; }
thread_local! { pub static GL11_GL_WRITE_ONLY: i32 = 35001; }

