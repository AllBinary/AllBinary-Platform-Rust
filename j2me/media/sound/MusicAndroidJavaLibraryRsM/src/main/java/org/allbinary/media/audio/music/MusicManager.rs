// Generated Code Do Not Modify
// package org.allbinary.media.audio.music
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::app::Activity::Activity;
use crate::android::content::Context::Context;
use crate::android::content::Intent::Intent;
use crate::org::allbinary::android::AndroidServicesUtil::AndroidServicesUtil;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::media::audio::NoSound::NoSound;
use crate::org::allbinary::media::audio::Sound::Sound;
use crate::org::allbinary::string::CommonStateStrings::CommonStateStrings;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::GameTickTimeDelayHelper::GameTickTimeDelayHelper;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use super::MusicStrings::MusicStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MusicManager {
    pub PLAY: String,
    pub FOR: String,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub musicStrings: Option<Rc<RefCell<MusicStrings>>>,
    pub commonStateStrings: Option<Rc<RefCell<CommonStateStrings>>>,
    pub basicArrayListUtil: Option<Rc<RefCell<BasicArrayListUtil>>>,
    pub resourceUtil: Option<Rc<RefCell<ResourceUtil>>>,
    pub androidServicesUtil: Option<Rc<RefCell<AndroidServicesUtil>>>,
    pub gameTickTimeDelayHelper: Option<Rc<RefCell<GameTickTimeDelayHelper>>>,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub timeDelayHelper2: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub songList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub currentSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>,
    pub nextSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>,
    pub leftVolume: i32,
    pub rightVolume: i32,
    pub musicServiceClass: &'static str,
    pub currentIntent: Option<Rc<RefCell<Intent>>>,
}


impl MusicManager {
    

    pub fn new_struct(mut musicServiceClass: &'static str, mut songList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Self {
        let mut this = Self {
            PLAY: String::new(),
            FOR: String::new(),
            commonStrings: None,
            musicStrings: None,
            commonStateStrings: None,
            basicArrayListUtil: None,
            resourceUtil: None,
            androidServicesUtil: None,
            gameTickTimeDelayHelper: None,
            timeDelayHelper: None,
            timeDelayHelper2: None,
            songList: None,
            currentSongSound: None,
            nextSongSound: None,
            leftVolume: 0,
            rightVolume: 0,
            musicServiceClass: "",
            currentIntent: None,
        };
        this.PLAY = String::from("Play ");
        this.FOR = String::from(" for: ");
        this.commonStrings = CommonStrings::getInstance();
        this.musicStrings = MusicStrings::getInstance();
        this.commonStateStrings = CommonStateStrings::getInstance();
        this.basicArrayListUtil = BasicArrayListUtil::getInstance();
        this.resourceUtil = ResourceUtil::getInstance();
        this.androidServicesUtil = AndroidServicesUtil::getInstance();
        this.gameTickTimeDelayHelper = GameTickTimeDelayHelperFactory::getInstance();
        this.timeDelayHelper = TimeDelayHelper::new(0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.timeDelayHelper2 = TimeDelayHelper::new(1200).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.currentSongSound = NoSound::getInstance();
        this.nextSongSound = NoSound::getInstance();
        this.leftVolume = 100;
        this.rightVolume = 100;
        PreLogUtil::put(format!("{}{}", jcall!(this.commonStateStrings).CONTEXT, jcall!(this.resourceUtil).getContext()), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.musicServiceClass = musicServiceClass.clone();
        this.currentIntent = Intent::new(jcall!(this.resourceUtil).getContext(), musicServiceClass.clone());
        this.songList = songList.clone();
        this
    }

    pub fn new(mut musicServiceClass: &'static str, mut songList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(musicServiceClass, songList))))
    }
    

    pub fn nextSong(&mut self, mut nextSongSound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>, mut leftVolume: i32, mut rightVolume: i32) {
        if nextSongSound.is_none() {
            nextSongSound = NoSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>);
        }
        self.nextSongSound = nextSongSound.clone();
        self.leftVolume = leftVolume.clone();
        self.rightVolume = rightVolume.clone();
        self.reset();
    }
    

    pub fn reset(&mut self) {
        jcall!(self.timeDelayHelper).as_TimeDelayHelper_mut().delay = 0;
    }
    

    pub fn process(&mut self) {
        if jcall!(self.songList).size() == 0 {
            return;
        }
        if jcall!(self.timeDelayHelper).isTime(jcall!(self.gameTickTimeDelayHelper).startTime.clone()) {
            self.startNewSong();
            return;
        }
        if jcall!(self.timeDelayHelper2).isTime(jcall!(self.gameTickTimeDelayHelper).startTime.clone()) {
            if { let java_receiver = self.androidServicesUtil.clone(); let java_arguments = (self.musicServiceClass.to_string(),); let java_result = jcall!(java_receiver).isServiceRunning(java_arguments.0); java_result } {
            } else {
                self.startNewSong();
            }
        }
    }
    

    pub fn show(&mut self) {
        {
            let mut index: i32 = jcall!(self.songList).size();
            while { index -= 1; index } >= 0 {
            let mut sound: Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>> = { let java_object = (jcall!(self.songList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>>().expect("ClassCastException").clone()) };
            let mut duration: i64 = ((jcall!(sound).getDuration()) as i64);
            PreLogUtil::put(jcall!(StringMaker::new()).append(self.PLAY.clone()).append(jcall!(sound).getResource()).append(self.FOR.clone()).appendlong(duration.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone());
            }
        }
    }
    

    pub fn startNewSong(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(jcall!(self.resourceUtil).getContext()).stopService(self.currentIntent.clone());
            if match (&self.nextSongSound, &NoSound::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.currentSongSound = { let java_object = (jcall!(self.basicArrayListUtil).getRandom(self.songList.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>>>().expect("ClassCastException").clone()) };
            } else {
                self.currentSongSound = self.nextSongSound.clone();
                self.nextSongSound = NoSound::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::audio::Sound::SoundDyn>>);
            }
            let mut duration: i64 = ((jcall!(self.currentSongSound).getDuration()) as i64);
            PreLogUtil::put(jcall!(StringMaker::new()).append(self.PLAY.clone()).append(jcall!(self.currentSongSound).getResource()).append(self.FOR.clone()).appendlong(duration.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone());
            jcall!(self.timeDelayHelper).as_TimeDelayHelper_mut().delay = ((duration) as i32);
            { let java_receiver = self.currentIntent.clone(); let java_arguments = (jcall!(self.musicStrings).SONG_EXTRA.clone(), (({ let java_receiver = self.resourceUtil.clone(); let java_arguments = (jcall!(self.currentSongSound).getResource(),); let java_result = jcall!(java_receiver).getResourceId(java_arguments.0); java_result }).unwrap() as i32),); jcall!(java_receiver).putExtra(java_arguments.0, java_arguments.1); };
            jcall!(self.currentIntent).putExtra(jcall!(self.musicStrings).LEFT_VOLUME.clone(), self.leftVolume.clone());
            jcall!(self.currentIntent).putExtra(jcall!(self.musicStrings).RIGHT_VOLUME.clone(), self.rightVolume.clone());
            jcall!(jcall!(self.resourceUtil).getContext()).startService(self.currentIntent.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut resource: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            if self.currentSongSound.is_some() {
                resource = jcall!(self.currentSongSound).getResource();
            }
            PreLogUtil::putOE(format!("{}{}", jcall!(self.commonStrings).EXCEPTION_LABEL, resource), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone(), e.clone());
            }
        }
    }
    

    // throws Exception
    pub fn stop(&mut self) {
        jcall!(jcall!(self.resourceUtil).getContext()).stopService(self.currentIntent.clone());
    }
    

    pub fn pause(mut activity: Option<Rc<RefCell<Activity>>>, mut musicServiceClass: &'static str) {
        let mut commonStateStrings: Option<Rc<RefCell<CommonStateStrings>>> = CommonStateStrings::getInstance();
        let mut musicPauseIntent: Option<Rc<RefCell<Intent>>> = Intent::new((activity).clone() /* as Context */, musicServiceClass.clone());
        jcall!(musicPauseIntent).putExtra(jcall!(commonStateStrings).ON_START_COMMAND.clone(), 1);
        jcall!(activity).startService(musicPauseIntent.clone());
    }
    

    pub fn resume(mut activity: Option<Rc<RefCell<Activity>>>, mut musicServiceClass: &'static str) {
        if jcall!(AndroidServicesUtil::getInstance()).isServiceRunning(musicServiceClass.to_string()) {
            let mut commonStateStrings: Option<Rc<RefCell<CommonStateStrings>>> = CommonStateStrings::getInstance();
            let mut musicResumeIntent: Option<Rc<RefCell<Intent>>> = Intent::new(activity.clone(), musicServiceClass.clone());
            jcall!(musicResumeIntent).putExtra(jcall!(commonStateStrings).ON_START_COMMAND.clone(), 2);
            jcall!(activity).startService(musicResumeIntent.clone());
        }
    }
}

