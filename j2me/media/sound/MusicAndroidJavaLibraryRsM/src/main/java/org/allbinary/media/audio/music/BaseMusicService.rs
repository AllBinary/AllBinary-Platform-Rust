// Generated Code Do Not Modify
// package org.allbinary.media.audio.music
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use crate::android::app::Service::Service;
use crate::android::content::Intent::Intent;
use crate::android::media::MediaPlayer::MediaPlayer;
use crate::android::os::IBinder::IBinder;
use crate::org::allbinary::android::NullAndroidCanvas::NullAndroidCanvas;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStateStrings::CommonStateStrings;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::thread::ARunnable::ARunnable;
use crate::java::lang::Thread::*;
use super::MusicStrings::MusicStrings;
use super::MusicRunnable::MusicRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseMusicService {
    pub base: Service,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub commonStateStrings: Option<Rc<RefCell<CommonStateStrings>>>,
    pub ALREADY_PLAYING: String,
    pub WAITING_FOR_MUSIC_TO_END: String,
    pub player: Option<Rc<RefCell<MediaPlayer>>>,
    pub songId: i32,
    pub leftVolume: i32,
    pub rightVolume: i32,
}


impl BaseMusicService {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Service::new_struct(),
            logUtil: None,
            commonStrings: None,
            commonStateStrings: None,
            ALREADY_PLAYING: String::new(),
            WAITING_FOR_MUSIC_TO_END: String::new(),
            player: None,
            songId: 0,
            leftVolume: 0,
            rightVolume: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.commonStateStrings = CommonStateStrings::getInstance();
        this.ALREADY_PLAYING = String::from("This is one song per music service");
        this.WAITING_FOR_MUSIC_TO_END = String::from("Waiting for music to end");
        this.player = crate::org::allbinary::android::NullAndroidCanvas::NullAndroidCanvas_NULL_MEDIA_PLAYER.with(Clone::clone).clone();
        this.songId = -1;
        this.leftVolume = -1;
        this.rightVolume = -1;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn onBind(&mut self, mut intent: Option<Rc<RefCell<Intent>>>) -> Option<Rc<RefCell<IBinder>>> {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStateStrings).BIND.clone());
        return None;
    }
    

    pub fn onCreate(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStateStrings).CREATE.clone());
    }
    

    pub fn onDestroy(&mut self) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStateStrings).DESTROY.clone());
        if !match (&self.player, &crate::org::allbinary::android::NullAndroidCanvas::NullAndroidCanvas_NULL_MEDIA_PLAYER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStateStrings).PAUSE.clone());
            jcall!(self.player).stop();
            jcall!(self.player).reset();
            jcall!(self.player).release();
        }
    }
    

    pub fn pause(&mut self) {
        if !match (&self.player, &crate::org::allbinary::android::NullAndroidCanvas::NullAndroidCanvas_NULL_MEDIA_PLAYER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStateStrings).PAUSE.clone());
            jcall!(self.player).pause();
        }
    }
    

    pub fn resume(&mut self) {
        if !match (&self.player, &crate::org::allbinary::android::NullAndroidCanvas::NullAndroidCanvas_NULL_MEDIA_PLAYER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && !jcall!(self.player).isPlaying() {
            jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStateStrings).RESUME.clone());
            jcall!(self.player).start();
        }
    }
    

    pub fn start(&mut self) {
        self.player = MediaPlayer::create(None /* Java this: a &mut self method has no Rc of itself */, self.songId.clone());
        jcall!(self.player).setVolume((((self.leftVolume) as f32)) / 100.0_f32, (((self.rightVolume) as f32)) / 100.0_f32);
        jcall!(self.player).setLooping(false);
        jcall!(self.player).start();
    }
    

    pub fn onStart(&mut self, mut intent: Option<Rc<RefCell<Intent>>>, mut startid: i32) {
        self.onStartCommandIntent(intent.clone());
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStateStrings).START.clone());
    }
    

    pub fn onStartCommand(&mut self, mut intent: Option<Rc<RefCell<Intent>>>, mut flags: i32, mut startId: i32) -> i32 {
        self.onStartCommandIntent(intent.clone());
        return unsafe { crate::android::app::Service::Service_START_STICKY.clone() }.clone();
    }
    

    pub fn onStartCommandIntent(&mut self, mut intent: Option<Rc<RefCell<Intent>>>) {
        jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStateStrings).ON_START_COMMAND.clone());
        let mut musicStrings: Option<Rc<RefCell<MusicStrings>>> = MusicStrings::getInstance();
        if intent.is_some() {
            let mut command: i32 = jcall!(intent).getIntExtra(jcall!(self.commonStateStrings).ON_START_COMMAND.clone(), -1);
            { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", jcall!(CommonLabels::getInstance()).COMMAND_LABEL, command), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStateStrings).ON_START_COMMAND.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if command == 1 {
                self.pause();
                return;
            } else if command == 2 {
                self.resume();
                return;
            } else {
                self.songId = jcall!(intent).getIntExtra(jcall!(musicStrings).SONG_EXTRA.clone(), -1);
                self.leftVolume = jcall!(intent).getIntExtra(jcall!(musicStrings).LEFT_VOLUME.clone(), -1);
                self.rightVolume = jcall!(intent).getIntExtra(jcall!(musicStrings).RIGHT_VOLUME.clone(), -1);
            }
        } else {
            panic!("{:?}", format!("{}: {:?}", "RuntimeException", (String::from("Started service without intent"))));
        }
        if self.songId != -1 {
            ();
            if !match (&self.player, &crate::org::allbinary::android::NullAndroidCanvas::NullAndroidCanvas_NULL_MEDIA_PLAYER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && jcall!(self.player).isPlaying() {
                let mut player: Option<Rc<RefCell<MediaPlayer>>> = self.player.clone();
                jcall!(self.logUtil).putF(self.ALREADY_PLAYING.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStateStrings).ON_START_COMMAND.clone());
                /* Java local class: move it out of the function
pub struct MusicRunnable {
    pub base: ARunnable,
    pub baseMusicService: Option<Rc<RefCell<dyn BaseMusicServiceDyn>>>,
}


impl MusicRunnable {
                    

                    pub fn new_struct(mut baseMusicService: Option<Rc<RefCell<dyn BaseMusicServiceDyn>>>) -> Self {
                        let mut this = Self {
            base: ARunnable::new_struct(),
            baseMusicService: None,
                        };
                        this.baseMusicService = baseMusicService.clone();
                        this
                    }

                    pub fn new(mut baseMusicService: Option<Rc<RefCell<dyn BaseMusicServiceDyn>>>) -> Option<Rc<RefCell<Self>>> {
                        Some(Rc::new(RefCell::new(Self::new_struct(baseMusicService))))
                    }
                    

                    pub fn run(&mut self) {
                        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                        let mut commonStateStrings: Option<Rc<RefCell<CommonStateStrings>>> = CommonStateStrings::getInstance();
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            while jcall!(self.player).isPlaying() {
                                jcall!(logUtil).putF(jcall!(self.baseMusicService).as_BaseMusicService_mut().WAITING_FOR_MUSIC_TO_END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStateStrings).ON_START_COMMAND.clone());
                                std::thread::sleep(std::time::Duration::from_millis((1200) as u64));
                            }
                            jcall!(self.baseMusicService).onStartCommandIntent(self.base.intent.clone());
                        })) {
                            Ok(_) => {}
                            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                            jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStateStrings).ON_START_COMMAND.clone(), crate::java_into_object(e.clone()));
                            }
                        }
                    }
}
impl std::ops::Deref for MusicRunnable {
    type Target = ARunnable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MusicRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thread::ARunnable::ARunnableDyn for MusicRunnable {
    fn as_ARunnable(&self) -> &crate::org::allbinary::thread::ARunnable::ARunnable { &self }
    fn as_ARunnable_mut(&mut self) -> &mut crate::org::allbinary::thread::ARunnable::ARunnable { &mut self }
}
impl crate::Runnable for MusicRunnable {
    fn run(&mut self) { Self::run(self) }
}

*/
                let mut runnable: Option<Rc<RefCell<dyn Runnable>>> = MusicRunnable::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn Runnable>>);
                let mut thread: Option<Rc<RefCell<Thread>>> = Thread::new1(runnable.clone());
                jcall!(thread).start();
                return;
            }
            self.start();
        }
    }
}
impl std::ops::Deref for BaseMusicService {
    type Target = Service;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BaseMusicService {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BaseMusicServiceDyn {
    fn as_BaseMusicService(&self) -> &BaseMusicService;
    fn as_BaseMusicService_mut(&mut self) -> &mut BaseMusicService;
    fn onBind(&mut self, mut intent: Option<Rc<RefCell<Intent>>>) -> Option<Rc<RefCell<IBinder>>> { BaseMusicService::onBind(self.as_BaseMusicService_mut(), intent) }
    fn onCreate(&mut self) { BaseMusicService::onCreate(self.as_BaseMusicService_mut()) }
    fn onDestroy(&mut self) { BaseMusicService::onDestroy(self.as_BaseMusicService_mut()) }
    fn pause(&mut self) { BaseMusicService::pause(self.as_BaseMusicService_mut()) }
    fn resume(&mut self) { BaseMusicService::resume(self.as_BaseMusicService_mut()) }
    fn start(&mut self) { BaseMusicService::start(self.as_BaseMusicService_mut()) }
    fn onStart(&mut self, mut intent: Option<Rc<RefCell<Intent>>>, mut startid: i32) { BaseMusicService::onStart(self.as_BaseMusicService_mut(), intent, startid) }
    fn onStartCommand(&mut self, mut intent: Option<Rc<RefCell<Intent>>>, mut flags: i32, mut startId: i32) -> i32 { BaseMusicService::onStartCommand(self.as_BaseMusicService_mut(), intent, flags, startId) }
    fn onStartCommandIntent(&mut self, mut intent: Option<Rc<RefCell<Intent>>>) { BaseMusicService::onStartCommandIntent(self.as_BaseMusicService_mut(), intent) }
}
impl BaseMusicServiceDyn for BaseMusicService {
    fn as_BaseMusicService(&self) -> &BaseMusicService { self }
    fn as_BaseMusicService_mut(&mut self) -> &mut BaseMusicService { self }
}



