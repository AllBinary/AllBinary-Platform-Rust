// Generated Code Do Not Modify
// package javax.microedition.khronos.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::GL::GL;
use super::Buffer::Buffer;
use super::IntBuffer::IntBuffer;
use super::FloatBuffer::FloatBuffer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait GL10: GL {
    

    fn glActiveTexture(&mut self, texture: i32);
    

    fn glAlphaFunc(&mut self, func: i32, r#ref: f32);
    

    fn glBindTexture(&mut self, target: i32, texture: i32);
    

    fn glBlendFunc(&mut self, sfactor: i32, dfactor: i32);
    

    fn glClear(&mut self, mask: i32);
    

    fn glClearColor(&mut self, red: f32, green: f32, blue: f32, alpha: f32);
    

    fn glClearDepthf(&mut self, depth: f32);
    

    fn glClearStencil(&mut self, s: i32);
    

    fn glClientActiveTexture(&mut self, texture: i32);
    

    fn glColor4f(&mut self, red: f32, green: f32, blue: f32, alpha: f32);
    

    fn glColorMask(&mut self, red: bool, green: bool, blue: bool, alpha: bool);
    

    fn glColorPointer(&mut self, size: i32, r#type: i32, stride: i32, pointer: Option<Rc<RefCell<Buffer>>>);
    

    fn glCompressedTexImage2D(&mut self, target: i32, level: i32, internalformat: i32, width: i32, height: i32, border: i32, imageSize: i32, data: Option<Rc<RefCell<Buffer>>>);
    

    fn glCompressedTexSubImage2D(&mut self, target: i32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: i32, imageSize: i32, data: Option<Rc<RefCell<Buffer>>>);
    

    fn glCopyTexImage2D(&mut self, target: i32, level: i32, internalformat: i32, x: i32, y: i32, width: i32, height: i32, border: i32);
    

    fn glCopyTexSubImage2D(&mut self, target: i32, level: i32, xoffset: i32, yoffset: i32, x: i32, y: i32, width: i32, height: i32);
    

    fn glCullFace(&mut self, mode: i32);
    

    fn glDeleteTextures3(&mut self, n: i32, textures: Vec<i32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glDeleteTextures(&mut self, n: i32, textures: Option<Rc<RefCell<IntBuffer>>>);
    

    fn glDepthFunc(&mut self, func: i32);
    

    fn glDepthMask(&mut self, flag: bool);
    

    fn glDepthRangef(&mut self, zNear: f32, zFar: f32);
    

    fn glDisable(&mut self, cap: i32);
    

    fn glDisableClientState(&mut self, array: i32);
    

    fn glDrawArrays(&mut self, mode: i32, first: i32, count: i32);
    

    fn glDrawElements(&mut self, mode: i32, count: i32, r#type: i32, indices: Option<Rc<RefCell<Buffer>>>);
    

    fn glEnable(&mut self, cap: i32);
    

    fn glEnableClientState(&mut self, array: i32);
    

    fn glFinish(&mut self);
    

    fn glFlush(&mut self);
    

    fn glFogf(&mut self, pname: i32, param: f32);
    

    fn glFogfv3(&mut self, pname: i32, params: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glFogfv(&mut self, pname: i32, params: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glFrontFace(&mut self, mode: i32);
    

    fn glFrustumf(&mut self, left: f32, right: f32, bottom: f32, top: f32, zNear: f32, zFar: f32);
    

    fn glGenTextures3(&mut self, n: i32, textures: Vec<i32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glGenTextures(&mut self, n: i32, textures: Option<Rc<RefCell<IntBuffer>>>);
    

    fn glGetError(&mut self) -> i32;
    

    fn glGetIntegerv3(&mut self, pname: i32, params: Vec<i32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glGetIntegerv(&mut self, pname: i32, params: Option<Rc<RefCell<IntBuffer>>>);
    

    fn glGetString(&mut self, name: i32) -> String;
    

    fn glHint(&mut self, target: i32, mode: i32);
    

    fn glLightModelf(&mut self, pname: i32, param: f32);
    

    fn glLightModelfv3(&mut self, pname: i32, params: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glLightModelfv(&mut self, pname: i32, params: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glLightf(&mut self, light: i32, pname: i32, param: f32);
    

    fn glLightfv4(&mut self, light: i32, pname: i32, params: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glLightfv(&mut self, light: i32, pname: i32, params: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glLineWidth(&mut self, width: f32);
    

    fn glLoadIdentity(&mut self);
    

    fn glLoadMatrixf2(&mut self, m: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glLoadMatrixf(&mut self, m: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glLogicOp(&mut self, opcode: i32);
    

    fn glMaterialf(&mut self, face: i32, pname: i32, param: f32);
    

    fn glMaterialfv4(&mut self, face: i32, pname: i32, params: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glMaterialfv(&mut self, face: i32, pname: i32, params: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glMatrixMode(&mut self, mode: i32);
    

    fn glMultMatrixf2(&mut self, m: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glMultMatrixf(&mut self, m: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glMultiTexCoord4f(&mut self, target: i32, s: f32, t: f32, r: f32, q: f32);
    

    fn glNormal3f(&mut self, nx: f32, ny: f32, nz: f32);
    

    fn glNormalPointer(&mut self, r#type: i32, stride: i32, pointer: Option<Rc<RefCell<Buffer>>>);
    

    fn glOrthof(&mut self, left: f32, right: f32, bottom: f32, top: f32, zNear: f32, zFar: f32);
    

    fn glPixelStorei(&mut self, pname: i32, param: i32);
    

    fn glPointSize(&mut self, size: f32);
    

    fn glPolygonOffset(&mut self, factor: f32, units: f32);
    

    fn glPopMatrix(&mut self);
    

    fn glPushMatrix(&mut self);
    

    fn glReadPixels(&mut self, x: i32, y: i32, width: i32, height: i32, format: i32, r#type: i32, pixels: Option<Rc<RefCell<Buffer>>>);
    

    fn glRotatef(&mut self, angle: f32, x: f32, y: f32, z: f32);
    

    fn glRotatex(&mut self, angle: i32, x: i32, y: i32, z: i32);
    

    fn glSampleCoverage(&mut self, value: f32, invert: bool);
    

    fn glScalef(&mut self, x: f32, y: f32, z: f32);
    

    fn glScalex(&mut self, x: i32, y: i32, z: i32);
    

    fn glScissor(&mut self, x: i32, y: i32, width: i32, height: i32);
    

    fn glShadeModel(&mut self, mode: i32);
    

    fn glStencilFunc(&mut self, func: i32, r#ref: i32, mask: i32);
    

    fn glStencilMask(&mut self, mask: i32);
    

    fn glStencilOp(&mut self, fail: i32, zfail: i32, zpass: i32);
    

    fn glTexCoordPointer(&mut self, size: i32, r#type: i32, stride: i32, pointer: Option<Rc<RefCell<Buffer>>>);
    

    fn glTexEnvf(&mut self, target: i32, pname: i32, param: f32);
    

    fn glTexEnvfv4(&mut self, target: i32, pname: i32, params: Vec<f32>, offset: i32);
    

    // Java overloaded method: Rust needs a different name
    fn glTexEnvfv(&mut self, target: i32, pname: i32, params: Option<Rc<RefCell<FloatBuffer>>>);
    

    fn glTexImage2D(&mut self, target: i32, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: i32, r#type: i32, pixels: Option<Rc<RefCell<Buffer>>>);
    

    fn glTexParameterf(&mut self, target: i32, pname: i32, param: f32);
    

    fn glTexParameteri(&mut self, target: i32, pname: i32, param: i32);
    

    fn glTexSubImage2D(&mut self, target: i32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: i32, r#type: i32, pixels: Option<Rc<RefCell<Buffer>>>);
    

    fn glTranslatef(&mut self, x: f32, y: f32, z: f32);
    

    fn glVertexPointer(&mut self, size: i32, r#type: i32, stride: i32, pointer: Option<Rc<RefCell<Buffer>>>);
    

    fn glViewport(&mut self, x: i32, y: i32, width: i32, height: i32);
    

    fn glBegin(&mut self, i: i32);
    

    fn glEnd(&mut self);
    

    fn glVertex3f(&mut self, f: f32, f2: f32, f3: f32);
    

    fn glPolygonMode(&mut self, i: i32, i1: i32);
}

pub static GL10_GL_ADD: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 260);
pub static GL10_GL_ALIASED_LINE_WIDTH_RANGE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33902);
pub static GL10_GL_ALIASED_POINT_SIZE_RANGE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33901);
pub static GL10_GL_ALPHA: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 6406);
pub static GL10_GL_ALPHA_BITS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3413);
pub static GL10_GL_ALPHA_TEST: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3008);
pub static GL10_GL_ALWAYS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 519);
pub static GL10_GL_AMBIENT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 4608);
pub static GL10_GL_AMBIENT_AND_DIFFUSE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5634);
pub static GL10_GL_AND: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5377);
pub static GL10_GL_AND_INVERTED: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5380);
pub static GL10_GL_AND_REVERSE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5378);
pub static GL10_GL_BACK: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 1029);
pub static GL10_GL_BLEND: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3042);
pub static GL10_GL_BLUE_BITS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3412);
pub static GL10_GL_BYTE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5120);
pub static GL10_GL_CCW: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2305);
pub static GL10_GL_CLAMP_TO_EDGE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33071);
pub static GL10_GL_CLEAR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5376);
pub static GL10_GL_COLOR_ARRAY: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32886);
pub static GL10_GL_COLOR_BUFFER_BIT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 16384);
pub static GL10_GL_COLOR_LOGIC_OP: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3058);
pub static GL10_GL_COLOR_MATERIAL: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2903);
pub static GL10_GL_COMPRESSED_TEXTURE_FORMATS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34467);
pub static GL10_GL_CONSTANT_ATTENUATION: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 4615);
pub static GL10_GL_COPY: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5379);
pub static GL10_GL_COPY_INVERTED: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5388);
pub static GL10_GL_CULL_FACE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2884);
pub static GL10_GL_CW: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2304);
pub static GL10_GL_DECAL: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 8449);
pub static GL10_GL_DECR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 7683);
pub static GL10_GL_DEPTH_BITS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3414);
pub static GL10_GL_DEPTH_BUFFER_BIT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 256);
pub static GL10_GL_DEPTH_TEST: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2929);
pub static GL10_GL_DIFFUSE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 4609);
pub static GL10_GL_DITHER: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3024);
pub static GL10_GL_DONT_CARE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 4352);
pub static GL10_GL_DST_ALPHA: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 772);
pub static GL10_GL_DST_COLOR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 774);
pub static GL10_GL_EMISSION: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5632);
pub static GL10_GL_EQUAL: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 514);
pub static GL10_GL_EQUIV: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5385);
pub static GL10_GL_EXP: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2048);
pub static GL10_GL_EXP2: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2049);
pub static GL10_GL_EXTENSIONS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 7939);
pub static GL10_GL_FALSE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 0);
pub static GL10_GL_FASTEST: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 4353);
pub static GL10_GL_FIXED: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5132);
pub static GL10_GL_FLAT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 7424);
pub static GL10_GL_FLOAT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5126);
pub static GL10_GL_FOG: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2912);
pub static GL10_GL_FOG_COLOR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2918);
pub static GL10_GL_FOG_DENSITY: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2914);
pub static GL10_GL_FOG_END: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2916);
pub static GL10_GL_FOG_HINT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3156);
pub static GL10_GL_FOG_MODE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2917);
pub static GL10_GL_FOG_START: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2915);
pub static GL10_GL_FRONT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 1028);
pub static GL10_GL_FRONT_AND_BACK: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 1032);
pub static GL10_GL_GEQUAL: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 518);
pub static GL10_GL_GREATER: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 516);
pub static GL10_GL_GREEN_BITS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3411);
pub static GL10_GL_IMPLEMENTATION_COLOR_READ_FORMAT_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35739);
pub static GL10_GL_IMPLEMENTATION_COLOR_READ_TYPE_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35738);
pub static GL10_GL_INCR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 7682);
pub static GL10_GL_INVALID_ENUM: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 1280);
pub static GL10_GL_INVALID_OPERATION: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 1282);
pub static GL10_GL_INVALID_VALUE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 1281);
pub static GL10_GL_INVERT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5386);
pub static GL10_GL_KEEP: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 7680);
pub static GL10_GL_LEQUAL: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 515);
pub static GL10_GL_LESS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 513);
pub static GL10_GL_LIGHT_MODEL_AMBIENT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2899);
pub static GL10_GL_LIGHT_MODEL_TWO_SIDE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2898);
pub static GL10_GL_LIGHT0: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 16384);
pub static GL10_GL_LIGHT1: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 16385);
pub static GL10_GL_LIGHT2: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 16386);
pub static GL10_GL_LIGHT3: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 16387);
pub static GL10_GL_LIGHT4: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 16388);
pub static GL10_GL_LIGHT5: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 16389);
pub static GL10_GL_LIGHT6: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 16390);
pub static GL10_GL_LIGHT7: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 16391);
pub static GL10_GL_LIGHTING: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2896);
pub static GL10_GL_LINE_LOOP: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2);
pub static GL10_GL_LINE_SMOOTH: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2848);
pub static GL10_GL_LINE_SMOOTH_HINT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3154);
pub static GL10_GL_LINE_STRIP: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3);
pub static GL10_GL_LINEAR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 9729);
pub static GL10_GL_LINEAR_ATTENUATION: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 4616);
pub static GL10_GL_LINEAR_MIPMAP_LINEAR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 9987);
pub static GL10_GL_LINEAR_MIPMAP_NEAREST: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 9985);
pub static GL10_GL_LINES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 1);
pub static GL10_GL_LUMINANCE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 6409);
pub static GL10_GL_LUMINANCE_ALPHA: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 6410);
pub static GL10_GL_MAX_ELEMENTS_INDICES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33001);
pub static GL10_GL_MAX_ELEMENTS_VERTICES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33000);
pub static GL10_GL_MAX_LIGHTS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3377);
pub static GL10_GL_MAX_MODELVIEW_STACK_DEPTH: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3382);
pub static GL10_GL_MAX_PROJECTION_STACK_DEPTH: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3384);
pub static GL10_GL_MAX_TEXTURE_SIZE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3379);
pub static GL10_GL_MAX_TEXTURE_STACK_DEPTH: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3385);
pub static GL10_GL_MAX_TEXTURE_UNITS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34018);
pub static GL10_GL_MAX_VIEWPORT_DIMS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3386);
pub static GL10_GL_MODELVIEW: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5888);
pub static GL10_GL_MODULATE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 8448);
pub static GL10_GL_MULTISAMPLE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32925);
pub static GL10_GL_NAND: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5390);
pub static GL10_GL_NEAREST: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 9728);
pub static GL10_GL_NEAREST_MIPMAP_LINEAR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 9986);
pub static GL10_GL_NEAREST_MIPMAP_NEAREST: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 9984);
pub static GL10_GL_NEVER: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 512);
pub static GL10_GL_NICEST: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 4354);
pub static GL10_GL_NO_ERROR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 0);
pub static GL10_GL_NOOP: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5381);
pub static GL10_GL_NOR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5384);
pub static GL10_GL_NORMAL_ARRAY: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32885);
pub static GL10_GL_NORMALIZE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2977);
pub static GL10_GL_NOTEQUAL: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 517);
pub static GL10_GL_NUM_COMPRESSED_TEXTURE_FORMATS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34466);
pub static GL10_GL_ONE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 1);
pub static GL10_GL_ONE_MINUS_DST_ALPHA: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 773);
pub static GL10_GL_ONE_MINUS_DST_COLOR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 775);
pub static GL10_GL_ONE_MINUS_SRC_ALPHA: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 771);
pub static GL10_GL_ONE_MINUS_SRC_COLOR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 769);
pub static GL10_GL_OR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5383);
pub static GL10_GL_OR_INVERTED: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5389);
pub static GL10_GL_OR_REVERSE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5387);
pub static GL10_GL_OUT_OF_MEMORY: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 1285);
pub static GL10_GL_PACK_ALIGNMENT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3333);
pub static GL10_GL_PALETTE4_R5_G6_B5_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35730);
pub static GL10_GL_PALETTE4_RGB5_A1_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35732);
pub static GL10_GL_PALETTE4_RGB8_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35728);
pub static GL10_GL_PALETTE4_RGBA4_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35731);
pub static GL10_GL_PALETTE4_RGBA8_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35729);
pub static GL10_GL_PALETTE8_R5_G6_B5_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35735);
pub static GL10_GL_PALETTE8_RGB5_A1_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35737);
pub static GL10_GL_PALETTE8_RGB8_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35733);
pub static GL10_GL_PALETTE8_RGBA4_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35736);
pub static GL10_GL_PALETTE8_RGBA8_OES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 35734);
pub static GL10_GL_PERSPECTIVE_CORRECTION_HINT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3152);
pub static GL10_GL_POINT_SMOOTH: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2832);
pub static GL10_GL_POINT_SMOOTH_HINT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3153);
pub static GL10_GL_POINTS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 0);
pub static GL10_GL_POINT_FADE_THRESHOLD_SIZE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33064);
pub static GL10_GL_POINT_SIZE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2833);
pub static GL10_GL_POLYGON_OFFSET_FILL: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32823);
pub static GL10_GL_POLYGON_SMOOTH_HINT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3155);
pub static GL10_GL_POSITION: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 4611);
pub static GL10_GL_PROJECTION: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5889);
pub static GL10_GL_QUADRATIC_ATTENUATION: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 4617);
pub static GL10_GL_RED_BITS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3410);
pub static GL10_GL_RENDERER: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 7937);
pub static GL10_GL_REPEAT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 10497);
pub static GL10_GL_REPLACE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 7681);
pub static GL10_GL_RESCALE_NORMAL: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32826);
pub static GL10_GL_RGB: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 6407);
pub static GL10_GL_RGBA: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 6408);
pub static GL10_GL_SAMPLE_ALPHA_TO_COVERAGE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32926);
pub static GL10_GL_SAMPLE_ALPHA_TO_ONE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32927);
pub static GL10_GL_SAMPLE_COVERAGE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32928);
pub static GL10_GL_SCISSOR_TEST: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3089);
pub static GL10_GL_SET: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5391);
pub static GL10_GL_SHININESS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5633);
pub static GL10_GL_SHORT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5122);
pub static GL10_GL_SMOOTH: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 7425);
pub static GL10_GL_SMOOTH_LINE_WIDTH_RANGE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2850);
pub static GL10_GL_SMOOTH_POINT_SIZE_RANGE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2834);
pub static GL10_GL_SPECULAR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 4610);
pub static GL10_GL_SPOT_CUTOFF: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 4614);
pub static GL10_GL_SPOT_DIRECTION: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 4612);
pub static GL10_GL_SPOT_EXPONENT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 4613);
pub static GL10_GL_SRC_ALPHA: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 770);
pub static GL10_GL_SRC_ALPHA_SATURATE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 776);
pub static GL10_GL_SRC_COLOR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 768);
pub static GL10_GL_STACK_OVERFLOW: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 1283);
pub static GL10_GL_STACK_UNDERFLOW: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 1284);
pub static GL10_GL_STENCIL_BITS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3415);
pub static GL10_GL_STENCIL_BUFFER_BIT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 1024);
pub static GL10_GL_STENCIL_TEST: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 2960);
pub static GL10_GL_SUBPIXEL_BITS: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3408);
pub static GL10_GL_TEXTURE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5890);
pub static GL10_GL_TEXTURE_2D: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3553);
pub static GL10_GL_TEXTURE_COORD_ARRAY: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32888);
pub static GL10_GL_TEXTURE_ENV: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 8960);
pub static GL10_GL_TEXTURE_ENV_COLOR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 8705);
pub static GL10_GL_TEXTURE_ENV_MODE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 8704);
pub static GL10_GL_TEXTURE_MAG_FILTER: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 10240);
pub static GL10_GL_TEXTURE_MIN_FILTER: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 10241);
pub static GL10_GL_TEXTURE_WRAP_S: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 10242);
pub static GL10_GL_TEXTURE_WRAP_T: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 10243);
pub static GL10_GL_TEXTURE0: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33984);
pub static GL10_GL_TEXTURE1: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33985);
pub static GL10_GL_TEXTURE2: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33986);
pub static GL10_GL_TEXTURE3: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33987);
pub static GL10_GL_TEXTURE4: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33988);
pub static GL10_GL_TEXTURE5: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33989);
pub static GL10_GL_TEXTURE6: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33990);
pub static GL10_GL_TEXTURE7: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33991);
pub static GL10_GL_TEXTURE8: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33992);
pub static GL10_GL_TEXTURE9: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33993);
pub static GL10_GL_TEXTURE10: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33994);
pub static GL10_GL_TEXTURE11: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33995);
pub static GL10_GL_TEXTURE12: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33996);
pub static GL10_GL_TEXTURE13: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33997);
pub static GL10_GL_TEXTURE14: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33998);
pub static GL10_GL_TEXTURE15: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33999);
pub static GL10_GL_TEXTURE16: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34000);
pub static GL10_GL_TEXTURE17: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34001);
pub static GL10_GL_TEXTURE18: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34002);
pub static GL10_GL_TEXTURE19: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34003);
pub static GL10_GL_TEXTURE20: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34004);
pub static GL10_GL_TEXTURE21: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34005);
pub static GL10_GL_TEXTURE22: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34006);
pub static GL10_GL_TEXTURE23: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34007);
pub static GL10_GL_TEXTURE24: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34008);
pub static GL10_GL_TEXTURE25: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34009);
pub static GL10_GL_TEXTURE26: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34010);
pub static GL10_GL_TEXTURE27: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34011);
pub static GL10_GL_TEXTURE28: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34012);
pub static GL10_GL_TEXTURE29: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34013);
pub static GL10_GL_TEXTURE30: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34014);
pub static GL10_GL_TEXTURE31: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 34015);
pub static GL10_GL_TRIANGLE_FAN: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 6);
pub static GL10_GL_TRIANGLE_STRIP: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5);
pub static GL10_GL_TRIANGLES: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 4);
pub static GL10_GL_TRUE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 1);
pub static GL10_GL_UNPACK_ALIGNMENT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 3317);
pub static GL10_GL_UNSIGNED_BYTE: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5121);
pub static GL10_GL_UNSIGNED_SHORT: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5123);
pub static GL10_GL_UNSIGNED_SHORT_4_4_4_4: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32819);
pub static GL10_GL_UNSIGNED_SHORT_5_5_5_1: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32820);
pub static GL10_GL_UNSIGNED_SHORT_5_6_5: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 33635);
pub static GL10_GL_VENDOR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 7936);
pub static GL10_GL_VERSION: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 7938);
pub static GL10_GL_VERTEX_ARRAY: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 32884);
pub static GL10_GL_XOR: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 5382);
pub static GL10_GL_ZERO: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 0);

