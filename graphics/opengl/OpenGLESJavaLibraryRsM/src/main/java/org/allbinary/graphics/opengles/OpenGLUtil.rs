// Generated Code Do Not Modify
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use crate::javax::microedition::khronos::opengles::GL10::GL10;
use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::device::LoadTextures::LoadTextures;
use crate::org::allbinary::device::OpenGLESGraphics::OpenGLESGraphics;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvas;
use crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvasFactory::ProgressCanvasFactory;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::opengles::renderer::RendererStrings::RendererStrings;
use crate::org::allbinary::image::PreResourceImageUtil::PreResourceImageUtil;
use crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage;
use crate::org::allbinary::image::opengles::OpenGLImageCache::OpenGLImageCache;
use crate::org::allbinary::image::opengles::OpenGLImageCacheFactory::OpenGLImageCacheFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::SynchObject::SynchObject;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub renderStrings: Option<Rc<RefCell<RendererStrings>>>,
    pub canvasStrings: Option<Rc<RefCell<CanvasStrings>>>,
    pub displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub preResourceImageUtil: Option<Rc<RefCell<PreResourceImageUtil>>>,
    pub lockObject: Option<Rc<RefCell<SynchObject>>>,
    pub anyType: Option<Rc<RefCell<SynchObject>>>,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub runnableList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub created: bool,
    pub surfaceCreatedAndInitialized: bool,
}

thread_local! { pub static OpenGLUtil_instance: Option<Rc<RefCell<OpenGLUtil>>> = OpenGLUtil::new(); }

impl OpenGLUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            renderStrings: None,
            canvasStrings: None,
            displayInfoSingleton: None,
            preResourceImageUtil: None,
            lockObject: None,
            anyType: None,
            list: None,
            runnableList: None,
            created: false,
            surfaceCreatedAndInitialized: false,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.renderStrings = RendererStrings::getInstance();
        this.canvasStrings = CanvasStrings::getInstance();
        this.displayInfoSingleton = DisplayInfoSingleton::getInstance();
        this.preResourceImageUtil = PreResourceImageUtil::getInstance();
        this.lockObject = SynchObject::new();
        this.anyType = SynchObject::new();
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.runnableList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.created = false;
        this.surfaceCreatedAndInitialized = false;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLUtil>>> {
        return OpenGLUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn onSurfaceCreated(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !self.created {
                self.created = true;
                PreLogUtil::put(format!("{}{}", { let java_field = jcall!(CommonLabels::getInstance()).START_LABEL.clone(); java_field }, jcall!(OpenGLCapabilities::OpenGLCapabilities::getInstance()).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.renderStrings).ON_SURFACE_CREATED.clone(); java_field }.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.renderStrings).ON_SURFACE_CREATED.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn onSurfaceCreated2(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut loadTextures: Option<Rc<RefCell<dyn crate::org::allbinary::device::LoadTextures::LoadTexturesDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(loadTextures).load(gl.clone());
            jcall!(OpenGLLogUtil::OpenGLLogUtil::getInstance()).logError(gl.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            jcall!(self.logUtil).put({ let java_field = jcall!(self.commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.renderStrings).ON_SURFACE_CREATED.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    // throws Exception
    pub fn onSurfaceChanged(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut graphics: Option<Rc<RefCell<dyn crate::org::allbinary::device::OpenGLESGraphics::OpenGLESGraphicsDyn>>>) {
        if !self.surfaceCreatedAndInitialized {
            jcall!(graphics).init();
            self.surfaceCreatedAndInitialized = true;
        }
        jcall!(graphics).update();
        let mut progressCanvas: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::canvas::transition::progress::ProgressCanvas::ProgressCanvasDyn>>> = ProgressCanvasFactory::getInstance();
        jcall!(progressCanvas).update(graphics.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>));
        jcall!(((OpenGLImageCacheFactory::getInstance()).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<OpenGLImageCache>() }, "ClassCastException: not a OpenGLImageCache"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<OpenGLImageCache>) } }))).update(gl.clone());
        self.processRunnables();
        self.set(gl.clone());
    }
    

    pub fn addImage(&mut self, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        let mut encapsulateImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(self.preResourceImageUtil).encapsulate(image.clone());
        if !match (&encapsulateImage, &image) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            { // synchronized (self.anyType)
                jcall!(self.list).add(crate::java_into_object(encapsulateImage.clone()));
            }
            return encapsulateImage.clone();
        }
        return image.clone();
    }
    

    pub fn clear(&mut self) {
        { // synchronized (self.anyType)
            jcall!(self.list).clear();
        }
    }
    

    pub fn addRunnable(&mut self, mut runnable: Option<Rc<RefCell<dyn Runnable>>>) {
        { // synchronized (self.lockObject)
            if !jcall!(self.runnableList).contains(crate::java_into_object(runnable.clone())) {
                jcall!(self.runnableList).add(crate::java_into_object(runnable.clone()));
                jcall!(self.displayInfoSingleton).add({ let java_field = jcall!(self.canvasStrings).SCALED_IMAGES.clone(); java_field }.clone());
            }
        }
    }
    

    pub fn processRunnables(&mut self) {
        { // synchronized (self.lockObject)
            let mut runnable: Option<Rc<RefCell<dyn Runnable>>> = None;
            let mut size: i32 = jcall!(self.runnableList).size();
            for index in (0..size) {
                runnable = { let java_value = ({ let java_object = (jcall!(self.runnableList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn Runnable>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
                jcall!(runnable).run();
            }
            jcall!(self.runnableList).clear();
        }
    }
    

    // throws Exception
    pub fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) {
        let mut size: i32 = jcall!(self.list).size();
        let mut image: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> = None;
        for index2 in (0..size) {
            image = { let java_value = ({ let java_object = (jcall!(self.list).get(index2.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>>>().expect("ClassCastException").clone()) }).clone(); java_value };
            jcall!(image).as_OpenGLESImage_mut().set(gl.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>));
        }
    }
}

