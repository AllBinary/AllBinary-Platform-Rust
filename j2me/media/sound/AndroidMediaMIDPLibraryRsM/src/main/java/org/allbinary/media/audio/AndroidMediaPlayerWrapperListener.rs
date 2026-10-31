// Generated Code Do Not Modify
// package org.allbinary.media.audio
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::media::PlayerListener::PlayerListener;
use crate::android::media::MediaPlayer::MediaPlayer;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::AndroidMediaPlayerWrapper::AndroidMediaPlayerWrapper;
use super::OnBufferingUpdateListener::OnBufferingUpdateListener;
use super::OnPreparedListener::OnPreparedListener;
use super::OnErrorListener::OnErrorListener;
use super::OnCompletionListener::OnCompletionListener;
use super::MediaPlayerOnBufferingUpdateListener::MediaPlayerOnBufferingUpdateListener;
use super::AndroidMediaPlayerOnBufferingUpdateListener::AndroidMediaPlayerOnBufferingUpdateListener;
use super::MediaPlayerOnPreparedListener::MediaPlayerOnPreparedListener;
use super::AndroidMediaPlayerOnPreparedListener::AndroidMediaPlayerOnPreparedListener;
use super::MediaPlayerOnErrorListener::MediaPlayerOnErrorListener;
use super::AndroidMediaPlayerOnErrorListener::AndroidMediaPlayerOnErrorListener;
use super::MediaPlayerOnCompletionListener::MediaPlayerOnCompletionListener;
use super::AndroidMediaPlayerOnCompletionListener::AndroidMediaPlayerOnCompletionListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidMediaPlayerWrapperListener {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub androidMediaPlayerWrapper: Option<Rc<RefCell<AndroidMediaPlayerWrapper>>>,
    pub mOnBufferingUpdateListener: Option<Rc<RefCell<OnBufferingUpdateListener>>>,
    pub mOnPreparedListener: Option<Rc<RefCell<OnPreparedListener>>>,
    pub mOnErrorListener: Option<Rc<RefCell<OnErrorListener>>>,
    pub mOnCompletionListener: Option<Rc<RefCell<OnCompletionListener>>>,
}


impl AndroidMediaPlayerWrapperListener {
    

    pub fn new_struct(mut androidMediaPlayerWrapper: Option<Rc<RefCell<AndroidMediaPlayerWrapper>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            androidMediaPlayerWrapper: None,
            mOnBufferingUpdateListener: None,
            mOnPreparedListener: None,
            mOnErrorListener: None,
            mOnCompletionListener: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.androidMediaPlayerWrapper = AndroidMediaPlayerWrapper_NULL_ANDROID_MEDIA_PLAYER_WRAPPER.with(Clone::clone).clone();
        this.mOnBufferingUpdateListener = AndroidMediaPlayerOnBufferingUpdateListener::new(this);
        this.mOnPreparedListener = AndroidMediaPlayerOnPreparedListener::new(this);
        this.mOnErrorListener = AndroidMediaPlayerOnErrorListener::new(this);
        this.mOnCompletionListener = AndroidMediaPlayerOnCompletionListener::new(this);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
            this.androidMediaPlayerWrapper = androidMediaPlayerWrapper.clone();
            let mut mediaPlayer: Option<Rc<RefCell<MediaPlayer>>> = jcall!(androidMediaPlayerWrapper).getMediaPlayer();
            jcall!(mediaPlayer).setOnCompletionListener(this.mOnCompletionListener.clone());
            jcall!(mediaPlayer).setOnBufferingUpdateListener(this.mOnBufferingUpdateListener.clone());
            jcall!(mediaPlayer).setOnPreparedListener(this.mOnPreparedListener.clone());
            jcall!(mediaPlayer).setOnErrorListener(this.mOnErrorListener.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(this.logUtil).put(jcall!(this.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            }
        }
        this
    }

    pub fn new(mut androidMediaPlayerWrapper: Option<Rc<RefCell<AndroidMediaPlayerWrapper>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(androidMediaPlayerWrapper))))
    }
}

pub struct MediaPlayerOnBufferingUpdateListener {
}

// impl OnBufferingUpdateListener for MediaPlayerOnBufferingUpdateListener with the methods of the trait

impl MediaPlayerOnBufferingUpdateListener {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
        

        pub fn onBufferingUpdate(&mut self, mut mediaPlayer: Option<Rc<RefCell<MediaPlayer>>>, mut i: i32) {
        }
}

pub struct MediaPlayerOnPreparedListener {
}

// impl OnPreparedListener for MediaPlayerOnPreparedListener with the methods of the trait

impl MediaPlayerOnPreparedListener {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
        

        pub fn onPrepared(&mut self, mut mp: Option<Rc<RefCell<MediaPlayer>>>) {
        }
}

pub struct MediaPlayerOnErrorListener {
}

// impl OnErrorListener for MediaPlayerOnErrorListener with the methods of the trait

impl MediaPlayerOnErrorListener {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
        

        pub fn onError(&mut self, mut mp: Option<Rc<RefCell<MediaPlayer>>>, mut what: i32, mut extra: i32) -> bool {
            return false;
        }
}

pub struct MediaPlayerOnCompletionListener {
}

// impl OnCompletionListener for MediaPlayerOnCompletionListener with the methods of the trait

impl MediaPlayerOnCompletionListener {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
        

        pub fn onCompletion(&mut self, mut mp: Option<Rc<RefCell<MediaPlayer>>>) {
        }
}

pub struct AndroidMediaPlayerOnBufferingUpdateListener {
    pub base: MediaPlayerOnBufferingUpdateListener,
    pub ON_BUFFERING_UPDATE: String,
    pub androidMediaPlayerWrapperListener: Option<Rc<RefCell<AndroidMediaPlayerWrapperListener>>>,
}


impl AndroidMediaPlayerOnBufferingUpdateListener {
        

        pub fn new_struct(mut androidMediaPlayerWrapperListener: Option<Rc<RefCell<AndroidMediaPlayerWrapperListener>>>) -> Self {
            let mut this = Self {
            base: MediaPlayerOnBufferingUpdateListener::new_struct(),
            ON_BUFFERING_UPDATE: String::new(),
            androidMediaPlayerWrapperListener: None,
            };
        this.ON_BUFFERING_UPDATE = String::from("onBufferingUpdate()");
            this.androidMediaPlayerWrapperListener = androidMediaPlayerWrapperListener.clone();
            this
        }

        pub fn new(mut androidMediaPlayerWrapperListener: Option<Rc<RefCell<AndroidMediaPlayerWrapperListener>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(androidMediaPlayerWrapperListener))))
        }
        

        pub fn onBufferingUpdate(&mut self, mut mediaPlayer: Option<Rc<RefCell<MediaPlayer>>>, mut i: i32) {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            jcall!(logUtil).putF(jcall!(StringMaker::new()).append(String::from("Update buffer: ")).appendint(i.clone()).append(String::from("%")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ON_BUFFERING_UPDATE.clone());
            jcall!(jcall!(self.androidMediaPlayerWrapperListener).androidMediaPlayerWrapper).update(unsafe { crate::javax::microedition::media::PlayerListener::PlayerListener_DEVICE_UNAVAILABLE.clone() }.clone());
        }
}
impl std::ops::Deref for AndroidMediaPlayerOnBufferingUpdateListener {
    type Target = MediaPlayerOnBufferingUpdateListener;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AndroidMediaPlayerOnBufferingUpdateListener {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



pub struct AndroidMediaPlayerOnPreparedListener {
    pub base: MediaPlayerOnPreparedListener,
    pub ON_PREPARE: String,
    pub androidMediaPlayerWrapperListener: Option<Rc<RefCell<AndroidMediaPlayerWrapperListener>>>,
}


impl AndroidMediaPlayerOnPreparedListener {
        

        pub fn new_struct(mut androidMediaPlayerWrapperListener: Option<Rc<RefCell<AndroidMediaPlayerWrapperListener>>>) -> Self {
            let mut this = Self {
            base: MediaPlayerOnPreparedListener::new_struct(),
            ON_PREPARE: String::new(),
            androidMediaPlayerWrapperListener: None,
            };
        this.ON_PREPARE = String::from("onPrepare()");
            this.androidMediaPlayerWrapperListener = androidMediaPlayerWrapperListener.clone();
            this
        }

        pub fn new(mut androidMediaPlayerWrapperListener: Option<Rc<RefCell<AndroidMediaPlayerWrapperListener>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(androidMediaPlayerWrapperListener))))
        }
        

        pub fn onPrepared(&mut self, mut mp: Option<Rc<RefCell<MediaPlayer>>>) {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ON_PREPARE.clone());
            jcall!(jcall!(self.androidMediaPlayerWrapperListener).androidMediaPlayerWrapper).update(unsafe { crate::javax::microedition::media::PlayerListener::PlayerListener_DEVICE_AVAILABLE.clone() }.clone());
        }
}
impl std::ops::Deref for AndroidMediaPlayerOnPreparedListener {
    type Target = MediaPlayerOnPreparedListener;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AndroidMediaPlayerOnPreparedListener {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



pub struct AndroidMediaPlayerOnErrorListener {
    pub base: MediaPlayerOnErrorListener,
    pub ON_ERROR: String,
    pub androidMediaPlayerWrapperListener: Option<Rc<RefCell<AndroidMediaPlayerWrapperListener>>>,
}


impl AndroidMediaPlayerOnErrorListener {
        

        pub fn new_struct(mut androidMediaPlayerWrapperListener: Option<Rc<RefCell<AndroidMediaPlayerWrapperListener>>>) -> Self {
            let mut this = Self {
            base: MediaPlayerOnErrorListener::new_struct(),
            ON_ERROR: String::new(),
            androidMediaPlayerWrapperListener: None,
            };
        this.ON_ERROR = String::from("onError()");
            this.androidMediaPlayerWrapperListener = androidMediaPlayerWrapperListener.clone();
            this
        }

        pub fn new(mut androidMediaPlayerWrapperListener: Option<Rc<RefCell<AndroidMediaPlayerWrapperListener>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(androidMediaPlayerWrapperListener))))
        }
        

        pub fn onError(&mut self, mut mp: Option<Rc<RefCell<MediaPlayer>>>, mut what: i32, mut extra: i32) -> bool {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            jcall!(logUtil).putF(jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).START_LABEL.clone()).append(String::from("What: ")).appendint(what.clone()).append(String::from(" Extra: ")).appendint(extra.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ON_ERROR.clone());
            jcall!(jcall!(self.androidMediaPlayerWrapperListener).androidMediaPlayerWrapper).update(unsafe { crate::javax::microedition::media::PlayerListener::PlayerListener_ERROR.clone() }.clone());
            return true;
        }
}
impl std::ops::Deref for AndroidMediaPlayerOnErrorListener {
    type Target = MediaPlayerOnErrorListener;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AndroidMediaPlayerOnErrorListener {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



pub struct AndroidMediaPlayerOnCompletionListener {
    pub base: MediaPlayerOnCompletionListener,
    pub ON_COMPLETE: String,
    pub androidMediaPlayerWrapperListener: Option<Rc<RefCell<AndroidMediaPlayerWrapperListener>>>,
}


impl AndroidMediaPlayerOnCompletionListener {
        

        pub fn new_struct(mut androidMediaPlayerWrapperListener: Option<Rc<RefCell<AndroidMediaPlayerWrapperListener>>>) -> Self {
            let mut this = Self {
            base: MediaPlayerOnCompletionListener::new_struct(),
            ON_COMPLETE: String::new(),
            androidMediaPlayerWrapperListener: None,
            };
        this.ON_COMPLETE = String::from("onComplete()");
            this.androidMediaPlayerWrapperListener = androidMediaPlayerWrapperListener.clone();
            this
        }

        pub fn new(mut androidMediaPlayerWrapperListener: Option<Rc<RefCell<AndroidMediaPlayerWrapperListener>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(androidMediaPlayerWrapperListener))))
        }
        

        pub fn onCompletion(&mut self, mut mp: Option<Rc<RefCell<MediaPlayer>>>) {
            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.ON_COMPLETE.clone());
            jcall!(jcall!(self.androidMediaPlayerWrapperListener).androidMediaPlayerWrapper).update(unsafe { crate::javax::microedition::media::PlayerListener::PlayerListener_END_OF_MEDIA.clone() }.clone());
        }
}
impl std::ops::Deref for AndroidMediaPlayerOnCompletionListener {
    type Target = MediaPlayerOnCompletionListener;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AndroidMediaPlayerOnCompletionListener {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



