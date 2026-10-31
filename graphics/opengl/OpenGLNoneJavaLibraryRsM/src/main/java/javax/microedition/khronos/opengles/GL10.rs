// Generated Code Do Not Modify
// package javax.microedition.khronos.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::nio::Buffer::Buffer;
// use crate::java::nio::FloatBuffer::FloatBuffer;
// use crate::java::nio::IntBuffer::IntBuffer;
use super::GL::GL;

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

thread_local! { pub static GL10_GL_ADD: i32 = 260; }
thread_local! { pub static GL10_GL_ALIASED_LINE_WIDTH_RANGE: i32 = 33902; }
thread_local! { pub static GL10_GL_ALIASED_POINT_SIZE_RANGE: i32 = 33901; }
thread_local! { pub static GL10_GL_ALPHA: i32 = 6406; }
thread_local! { pub static GL10_GL_ALPHA_BITS: i32 = 3413; }
thread_local! { pub static GL10_GL_ALPHA_TEST: i32 = 3008; }
thread_local! { pub static GL10_GL_ALWAYS: i32 = 519; }
thread_local! { pub static GL10_GL_AMBIENT: i32 = 4608; }
thread_local! { pub static GL10_GL_AMBIENT_AND_DIFFUSE: i32 = 5634; }
thread_local! { pub static GL10_GL_AND: i32 = 5377; }
thread_local! { pub static GL10_GL_AND_INVERTED: i32 = 5380; }
thread_local! { pub static GL10_GL_AND_REVERSE: i32 = 5378; }
thread_local! { pub static GL10_GL_BACK: i32 = 1029; }
thread_local! { pub static GL10_GL_BLEND: i32 = 3042; }
thread_local! { pub static GL10_GL_BLUE_BITS: i32 = 3412; }
thread_local! { pub static GL10_GL_BYTE: i32 = 5120; }
thread_local! { pub static GL10_GL_CCW: i32 = 2305; }
thread_local! { pub static GL10_GL_CLAMP_TO_EDGE: i32 = 33071; }
thread_local! { pub static GL10_GL_CLEAR: i32 = 5376; }
thread_local! { pub static GL10_GL_COLOR_ARRAY: i32 = 32886; }
thread_local! { pub static GL10_GL_COLOR_BUFFER_BIT: i32 = 16384; }
thread_local! { pub static GL10_GL_COLOR_LOGIC_OP: i32 = 3058; }
thread_local! { pub static GL10_GL_COLOR_MATERIAL: i32 = 2903; }
thread_local! { pub static GL10_GL_COMPRESSED_TEXTURE_FORMATS: i32 = 34467; }
thread_local! { pub static GL10_GL_CONSTANT_ATTENUATION: i32 = 4615; }
thread_local! { pub static GL10_GL_COPY: i32 = 5379; }
thread_local! { pub static GL10_GL_COPY_INVERTED: i32 = 5388; }
thread_local! { pub static GL10_GL_CULL_FACE: i32 = 2884; }
thread_local! { pub static GL10_GL_CW: i32 = 2304; }
thread_local! { pub static GL10_GL_DECAL: i32 = 8449; }
thread_local! { pub static GL10_GL_DECR: i32 = 7683; }
thread_local! { pub static GL10_GL_DEPTH_BITS: i32 = 3414; }
thread_local! { pub static GL10_GL_DEPTH_BUFFER_BIT: i32 = 256; }
thread_local! { pub static GL10_GL_DEPTH_TEST: i32 = 2929; }
thread_local! { pub static GL10_GL_DIFFUSE: i32 = 4609; }
thread_local! { pub static GL10_GL_DITHER: i32 = 3024; }
thread_local! { pub static GL10_GL_DONT_CARE: i32 = 4352; }
thread_local! { pub static GL10_GL_DST_ALPHA: i32 = 772; }
thread_local! { pub static GL10_GL_DST_COLOR: i32 = 774; }
thread_local! { pub static GL10_GL_EMISSION: i32 = 5632; }
thread_local! { pub static GL10_GL_EQUAL: i32 = 514; }
thread_local! { pub static GL10_GL_EQUIV: i32 = 5385; }
thread_local! { pub static GL10_GL_EXP: i32 = 2048; }
thread_local! { pub static GL10_GL_EXP2: i32 = 2049; }
thread_local! { pub static GL10_GL_EXTENSIONS: i32 = 7939; }
thread_local! { pub static GL10_GL_FALSE: i32 = 0; }
thread_local! { pub static GL10_GL_FASTEST: i32 = 4353; }
thread_local! { pub static GL10_GL_FIXED: i32 = 5132; }
thread_local! { pub static GL10_GL_FLAT: i32 = 7424; }
thread_local! { pub static GL10_GL_FLOAT: i32 = 5126; }
thread_local! { pub static GL10_GL_FOG: i32 = 2912; }
thread_local! { pub static GL10_GL_FOG_COLOR: i32 = 2918; }
thread_local! { pub static GL10_GL_FOG_DENSITY: i32 = 2914; }
thread_local! { pub static GL10_GL_FOG_END: i32 = 2916; }
thread_local! { pub static GL10_GL_FOG_HINT: i32 = 3156; }
thread_local! { pub static GL10_GL_FOG_MODE: i32 = 2917; }
thread_local! { pub static GL10_GL_FOG_START: i32 = 2915; }
thread_local! { pub static GL10_GL_FRONT: i32 = 1028; }
thread_local! { pub static GL10_GL_FRONT_AND_BACK: i32 = 1032; }
thread_local! { pub static GL10_GL_GEQUAL: i32 = 518; }
thread_local! { pub static GL10_GL_GREATER: i32 = 516; }
thread_local! { pub static GL10_GL_GREEN_BITS: i32 = 3411; }
thread_local! { pub static GL10_GL_IMPLEMENTATION_COLOR_READ_FORMAT_OES: i32 = 35739; }
thread_local! { pub static GL10_GL_IMPLEMENTATION_COLOR_READ_TYPE_OES: i32 = 35738; }
thread_local! { pub static GL10_GL_INCR: i32 = 7682; }
thread_local! { pub static GL10_GL_INVALID_ENUM: i32 = 1280; }
thread_local! { pub static GL10_GL_INVALID_OPERATION: i32 = 1282; }
thread_local! { pub static GL10_GL_INVALID_VALUE: i32 = 1281; }
thread_local! { pub static GL10_GL_INVERT: i32 = 5386; }
thread_local! { pub static GL10_GL_KEEP: i32 = 7680; }
thread_local! { pub static GL10_GL_LEQUAL: i32 = 515; }
thread_local! { pub static GL10_GL_LESS: i32 = 513; }
thread_local! { pub static GL10_GL_LIGHT_MODEL_AMBIENT: i32 = 2899; }
thread_local! { pub static GL10_GL_LIGHT_MODEL_TWO_SIDE: i32 = 2898; }
thread_local! { pub static GL10_GL_LIGHT0: i32 = 16384; }
thread_local! { pub static GL10_GL_LIGHT1: i32 = 16385; }
thread_local! { pub static GL10_GL_LIGHT2: i32 = 16386; }
thread_local! { pub static GL10_GL_LIGHT3: i32 = 16387; }
thread_local! { pub static GL10_GL_LIGHT4: i32 = 16388; }
thread_local! { pub static GL10_GL_LIGHT5: i32 = 16389; }
thread_local! { pub static GL10_GL_LIGHT6: i32 = 16390; }
thread_local! { pub static GL10_GL_LIGHT7: i32 = 16391; }
thread_local! { pub static GL10_GL_LIGHTING: i32 = 2896; }
thread_local! { pub static GL10_GL_LINE_LOOP: i32 = 2; }
thread_local! { pub static GL10_GL_LINE_SMOOTH: i32 = 2848; }
thread_local! { pub static GL10_GL_LINE_SMOOTH_HINT: i32 = 3154; }
thread_local! { pub static GL10_GL_LINE_STRIP: i32 = 3; }
thread_local! { pub static GL10_GL_LINEAR: i32 = 9729; }
thread_local! { pub static GL10_GL_LINEAR_ATTENUATION: i32 = 4616; }
thread_local! { pub static GL10_GL_LINEAR_MIPMAP_LINEAR: i32 = 9987; }
thread_local! { pub static GL10_GL_LINEAR_MIPMAP_NEAREST: i32 = 9985; }
thread_local! { pub static GL10_GL_LINES: i32 = 1; }
thread_local! { pub static GL10_GL_LUMINANCE: i32 = 6409; }
thread_local! { pub static GL10_GL_LUMINANCE_ALPHA: i32 = 6410; }
thread_local! { pub static GL10_GL_MAX_ELEMENTS_INDICES: i32 = 33001; }
thread_local! { pub static GL10_GL_MAX_ELEMENTS_VERTICES: i32 = 33000; }
thread_local! { pub static GL10_GL_MAX_LIGHTS: i32 = 3377; }
thread_local! { pub static GL10_GL_MAX_MODELVIEW_STACK_DEPTH: i32 = 3382; }
thread_local! { pub static GL10_GL_MAX_PROJECTION_STACK_DEPTH: i32 = 3384; }
thread_local! { pub static GL10_GL_MAX_TEXTURE_SIZE: i32 = 3379; }
thread_local! { pub static GL10_GL_MAX_TEXTURE_STACK_DEPTH: i32 = 3385; }
thread_local! { pub static GL10_GL_MAX_TEXTURE_UNITS: i32 = 34018; }
thread_local! { pub static GL10_GL_MAX_VIEWPORT_DIMS: i32 = 3386; }
thread_local! { pub static GL10_GL_MODELVIEW: i32 = 5888; }
thread_local! { pub static GL10_GL_MODULATE: i32 = 8448; }
thread_local! { pub static GL10_GL_MULTISAMPLE: i32 = 32925; }
thread_local! { pub static GL10_GL_NAND: i32 = 5390; }
thread_local! { pub static GL10_GL_NEAREST: i32 = 9728; }
thread_local! { pub static GL10_GL_NEAREST_MIPMAP_LINEAR: i32 = 9986; }
thread_local! { pub static GL10_GL_NEAREST_MIPMAP_NEAREST: i32 = 9984; }
thread_local! { pub static GL10_GL_NEVER: i32 = 512; }
thread_local! { pub static GL10_GL_NICEST: i32 = 4354; }
thread_local! { pub static GL10_GL_NO_ERROR: i32 = 0; }
thread_local! { pub static GL10_GL_NOOP: i32 = 5381; }
thread_local! { pub static GL10_GL_NOR: i32 = 5384; }
thread_local! { pub static GL10_GL_NORMAL_ARRAY: i32 = 32885; }
thread_local! { pub static GL10_GL_NORMALIZE: i32 = 2977; }
thread_local! { pub static GL10_GL_NOTEQUAL: i32 = 517; }
thread_local! { pub static GL10_GL_NUM_COMPRESSED_TEXTURE_FORMATS: i32 = 34466; }
thread_local! { pub static GL10_GL_ONE: i32 = 1; }
thread_local! { pub static GL10_GL_ONE_MINUS_DST_ALPHA: i32 = 773; }
thread_local! { pub static GL10_GL_ONE_MINUS_DST_COLOR: i32 = 775; }
thread_local! { pub static GL10_GL_ONE_MINUS_SRC_ALPHA: i32 = 771; }
thread_local! { pub static GL10_GL_ONE_MINUS_SRC_COLOR: i32 = 769; }
thread_local! { pub static GL10_GL_OR: i32 = 5383; }
thread_local! { pub static GL10_GL_OR_INVERTED: i32 = 5389; }
thread_local! { pub static GL10_GL_OR_REVERSE: i32 = 5387; }
thread_local! { pub static GL10_GL_OUT_OF_MEMORY: i32 = 1285; }
thread_local! { pub static GL10_GL_PACK_ALIGNMENT: i32 = 3333; }
thread_local! { pub static GL10_GL_PALETTE4_R5_G6_B5_OES: i32 = 35730; }
thread_local! { pub static GL10_GL_PALETTE4_RGB5_A1_OES: i32 = 35732; }
thread_local! { pub static GL10_GL_PALETTE4_RGB8_OES: i32 = 35728; }
thread_local! { pub static GL10_GL_PALETTE4_RGBA4_OES: i32 = 35731; }
thread_local! { pub static GL10_GL_PALETTE4_RGBA8_OES: i32 = 35729; }
thread_local! { pub static GL10_GL_PALETTE8_R5_G6_B5_OES: i32 = 35735; }
thread_local! { pub static GL10_GL_PALETTE8_RGB5_A1_OES: i32 = 35737; }
thread_local! { pub static GL10_GL_PALETTE8_RGB8_OES: i32 = 35733; }
thread_local! { pub static GL10_GL_PALETTE8_RGBA4_OES: i32 = 35736; }
thread_local! { pub static GL10_GL_PALETTE8_RGBA8_OES: i32 = 35734; }
thread_local! { pub static GL10_GL_PERSPECTIVE_CORRECTION_HINT: i32 = 3152; }
thread_local! { pub static GL10_GL_POINT_SMOOTH: i32 = 2832; }
thread_local! { pub static GL10_GL_POINT_SMOOTH_HINT: i32 = 3153; }
thread_local! { pub static GL10_GL_POINTS: i32 = 0; }
thread_local! { pub static GL10_GL_POINT_FADE_THRESHOLD_SIZE: i32 = 33064; }
thread_local! { pub static GL10_GL_POINT_SIZE: i32 = 2833; }
thread_local! { pub static GL10_GL_POLYGON_OFFSET_FILL: i32 = 32823; }
thread_local! { pub static GL10_GL_POLYGON_SMOOTH_HINT: i32 = 3155; }
thread_local! { pub static GL10_GL_POSITION: i32 = 4611; }
thread_local! { pub static GL10_GL_PROJECTION: i32 = 5889; }
thread_local! { pub static GL10_GL_QUADRATIC_ATTENUATION: i32 = 4617; }
thread_local! { pub static GL10_GL_RED_BITS: i32 = 3410; }
thread_local! { pub static GL10_GL_RENDERER: i32 = 7937; }
thread_local! { pub static GL10_GL_REPEAT: i32 = 10497; }
thread_local! { pub static GL10_GL_REPLACE: i32 = 7681; }
thread_local! { pub static GL10_GL_RESCALE_NORMAL: i32 = 32826; }
thread_local! { pub static GL10_GL_RGB: i32 = 6407; }
thread_local! { pub static GL10_GL_RGBA: i32 = 6408; }
thread_local! { pub static GL10_GL_SAMPLE_ALPHA_TO_COVERAGE: i32 = 32926; }
thread_local! { pub static GL10_GL_SAMPLE_ALPHA_TO_ONE: i32 = 32927; }
thread_local! { pub static GL10_GL_SAMPLE_COVERAGE: i32 = 32928; }
thread_local! { pub static GL10_GL_SCISSOR_TEST: i32 = 3089; }
thread_local! { pub static GL10_GL_SET: i32 = 5391; }
thread_local! { pub static GL10_GL_SHININESS: i32 = 5633; }
thread_local! { pub static GL10_GL_SHORT: i32 = 5122; }
thread_local! { pub static GL10_GL_SMOOTH: i32 = 7425; }
thread_local! { pub static GL10_GL_SMOOTH_LINE_WIDTH_RANGE: i32 = 2850; }
thread_local! { pub static GL10_GL_SMOOTH_POINT_SIZE_RANGE: i32 = 2834; }
thread_local! { pub static GL10_GL_SPECULAR: i32 = 4610; }
thread_local! { pub static GL10_GL_SPOT_CUTOFF: i32 = 4614; }
thread_local! { pub static GL10_GL_SPOT_DIRECTION: i32 = 4612; }
thread_local! { pub static GL10_GL_SPOT_EXPONENT: i32 = 4613; }
thread_local! { pub static GL10_GL_SRC_ALPHA: i32 = 770; }
thread_local! { pub static GL10_GL_SRC_ALPHA_SATURATE: i32 = 776; }
thread_local! { pub static GL10_GL_SRC_COLOR: i32 = 768; }
thread_local! { pub static GL10_GL_STACK_OVERFLOW: i32 = 1283; }
thread_local! { pub static GL10_GL_STACK_UNDERFLOW: i32 = 1284; }
thread_local! { pub static GL10_GL_STENCIL_BITS: i32 = 3415; }
thread_local! { pub static GL10_GL_STENCIL_BUFFER_BIT: i32 = 1024; }
thread_local! { pub static GL10_GL_STENCIL_TEST: i32 = 2960; }
thread_local! { pub static GL10_GL_SUBPIXEL_BITS: i32 = 3408; }
thread_local! { pub static GL10_GL_TEXTURE: i32 = 5890; }
thread_local! { pub static GL10_GL_TEXTURE_2D: i32 = 3553; }
thread_local! { pub static GL10_GL_TEXTURE_COORD_ARRAY: i32 = 32888; }
thread_local! { pub static GL10_GL_TEXTURE_ENV: i32 = 8960; }
thread_local! { pub static GL10_GL_TEXTURE_ENV_COLOR: i32 = 8705; }
thread_local! { pub static GL10_GL_TEXTURE_ENV_MODE: i32 = 8704; }
thread_local! { pub static GL10_GL_TEXTURE_MAG_FILTER: i32 = 10240; }
thread_local! { pub static GL10_GL_TEXTURE_MIN_FILTER: i32 = 10241; }
thread_local! { pub static GL10_GL_TEXTURE_WRAP_S: i32 = 10242; }
thread_local! { pub static GL10_GL_TEXTURE_WRAP_T: i32 = 10243; }
thread_local! { pub static GL10_GL_TEXTURE0: i32 = 33984; }
thread_local! { pub static GL10_GL_TEXTURE1: i32 = 33985; }
thread_local! { pub static GL10_GL_TEXTURE2: i32 = 33986; }
thread_local! { pub static GL10_GL_TEXTURE3: i32 = 33987; }
thread_local! { pub static GL10_GL_TEXTURE4: i32 = 33988; }
thread_local! { pub static GL10_GL_TEXTURE5: i32 = 33989; }
thread_local! { pub static GL10_GL_TEXTURE6: i32 = 33990; }
thread_local! { pub static GL10_GL_TEXTURE7: i32 = 33991; }
thread_local! { pub static GL10_GL_TEXTURE8: i32 = 33992; }
thread_local! { pub static GL10_GL_TEXTURE9: i32 = 33993; }
thread_local! { pub static GL10_GL_TEXTURE10: i32 = 33994; }
thread_local! { pub static GL10_GL_TEXTURE11: i32 = 33995; }
thread_local! { pub static GL10_GL_TEXTURE12: i32 = 33996; }
thread_local! { pub static GL10_GL_TEXTURE13: i32 = 33997; }
thread_local! { pub static GL10_GL_TEXTURE14: i32 = 33998; }
thread_local! { pub static GL10_GL_TEXTURE15: i32 = 33999; }
thread_local! { pub static GL10_GL_TEXTURE16: i32 = 34000; }
thread_local! { pub static GL10_GL_TEXTURE17: i32 = 34001; }
thread_local! { pub static GL10_GL_TEXTURE18: i32 = 34002; }
thread_local! { pub static GL10_GL_TEXTURE19: i32 = 34003; }
thread_local! { pub static GL10_GL_TEXTURE20: i32 = 34004; }
thread_local! { pub static GL10_GL_TEXTURE21: i32 = 34005; }
thread_local! { pub static GL10_GL_TEXTURE22: i32 = 34006; }
thread_local! { pub static GL10_GL_TEXTURE23: i32 = 34007; }
thread_local! { pub static GL10_GL_TEXTURE24: i32 = 34008; }
thread_local! { pub static GL10_GL_TEXTURE25: i32 = 34009; }
thread_local! { pub static GL10_GL_TEXTURE26: i32 = 34010; }
thread_local! { pub static GL10_GL_TEXTURE27: i32 = 34011; }
thread_local! { pub static GL10_GL_TEXTURE28: i32 = 34012; }
thread_local! { pub static GL10_GL_TEXTURE29: i32 = 34013; }
thread_local! { pub static GL10_GL_TEXTURE30: i32 = 34014; }
thread_local! { pub static GL10_GL_TEXTURE31: i32 = 34015; }
thread_local! { pub static GL10_GL_TRIANGLE_FAN: i32 = 6; }
thread_local! { pub static GL10_GL_TRIANGLE_STRIP: i32 = 5; }
thread_local! { pub static GL10_GL_TRIANGLES: i32 = 4; }
thread_local! { pub static GL10_GL_TRUE: i32 = 1; }
thread_local! { pub static GL10_GL_UNPACK_ALIGNMENT: i32 = 3317; }
thread_local! { pub static GL10_GL_UNSIGNED_BYTE: i32 = 5121; }
thread_local! { pub static GL10_GL_UNSIGNED_SHORT: i32 = 5123; }
thread_local! { pub static GL10_GL_UNSIGNED_SHORT_4_4_4_4: i32 = 32819; }
thread_local! { pub static GL10_GL_UNSIGNED_SHORT_5_5_5_1: i32 = 32820; }
thread_local! { pub static GL10_GL_UNSIGNED_SHORT_5_6_5: i32 = 33635; }
thread_local! { pub static GL10_GL_VENDOR: i32 = 7936; }
thread_local! { pub static GL10_GL_VERSION: i32 = 7938; }
thread_local! { pub static GL10_GL_VERTEX_ARRAY: i32 = 32884; }
thread_local! { pub static GL10_GL_XOR: i32 = 5382; }
thread_local! { pub static GL10_GL_ZERO: i32 = 0; }

