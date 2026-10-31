// Generated Code Do Not Modify
// package org.allbinary.game.layer.weapon
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::game::combat::weapon::WeaponProperties::WeaponProperties;
use crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::physics::movement::Movement::Movement;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::view::ViewPositionBase::ViewPositionBase;
use super::SimpleWeaponLayer::SimpleWeaponLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TempExplosionWeaponLayer {
    pub base: SimpleWeaponLayer,
    pub weaponProperties: Option<Rc<RefCell<WeaponProperties>>>,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
}

impl crate::org::allbinary::layer::NamedInterface::NamedInterface for TempExplosionWeaponLayer {
    fn getName(&mut self) -> String { self.base.getName() }
}
impl crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface for TempExplosionWeaponLayer {
    fn isDestroyed(&mut self) -> bool { self.base.isDestroyed() }
}
impl crate::org::allbinary::game::tick::TickableInterface::TickableInterface for TempExplosionWeaponLayer {
    fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processTick(self, allBinaryLayerManager) }
}

impl TempExplosionWeaponLayer {
    

    pub fn new_struct(mut name: String, mut movement: Option<Rc<RefCell<dyn crate::org::allbinary::physics::movement::Movement::MovementDyn>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>, mut timeDelay: i32) -> Self {
        let mut this = Self {
            base: SimpleWeaponLayer::new_struct(name.clone(), crate::org::allbinary::game::multiplayer::layer::RemoteInfo::RemoteInfo_REMOTE_INFO.with(Clone::clone).clone(), -1, movement.clone(), animationInterface.clone(), SimpleWeaponLayer::createDestroyed(), rectangle.clone(), viewPosition.clone()),
            weaponProperties: None,
            timeDelayHelper: None,
        };
        this.weaponProperties = WeaponProperties::new(-1_i64, -1_i64, 0, 1000, ((0) as i16));
        this.timeDelayHelper = TimeDelayHelper::new(0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        // super(...) is in the struct literal above
        jcall!(this.timeDelayHelper).as_TimeDelayHelper_mut().delay = timeDelay.clone();
        this
    }

    pub fn new(mut name: String, mut movement: Option<Rc<RefCell<dyn crate::org::allbinary::physics::movement::Movement::MovementDyn>>>, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut rectangle: Option<Rc<RefCell<Rectangle>>>, mut viewPosition: Option<Rc<RefCell<dyn crate::org::allbinary::view::ViewPositionBase::ViewPositionBaseDyn>>>, mut timeDelay: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, movement, animationInterface, rectangle, viewPosition, timeDelay))))
    }
    

    // throws Exception
    pub fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        if { let java_receiver = self.timeDelayHelper.clone(); let java_arguments = (jcall!(GameTickTimeDelayHelperFactory::getInstance()).startTime.clone(),); let java_result = jcall!(java_receiver).isTime(java_arguments.0); java_result } {
            jcall!(self.getCollidableInferface()).collide(None /* Java this: a &mut self method has no Rc of itself */, None /* Java this: a &mut self method has no Rc of itself */);
            self.totalDamage = self.getInitDamage() + 1;
        }
        self.base.processTick(allBinaryLayerManager.clone());
    }
}
impl std::ops::Deref for TempExplosionWeaponLayer {
    type Target = SimpleWeaponLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TempExplosionWeaponLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::weapon::SimpleWeaponLayer::SimpleWeaponLayerDyn for TempExplosionWeaponLayer {
    fn as_SimpleWeaponLayer(&self) -> &crate::org::allbinary::game::layer::weapon::SimpleWeaponLayer::SimpleWeaponLayer { &self.base }
    fn as_SimpleWeaponLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::weapon::SimpleWeaponLayer::SimpleWeaponLayer { &mut self.base }
}
impl crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn for TempExplosionWeaponLayer {
    fn as_WeaponLayer(&self) -> &crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer { &self.base.base }
    fn as_WeaponLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer { &mut self.base.base }
}
impl crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn for TempExplosionWeaponLayer {
    fn as_CollidableCompositeLayer(&self) -> &crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &self.base.base.base.base.base }
    fn as_CollidableCompositeLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer { &mut self.base.base.base.base.base }
    fn setCollidableInferface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>>) { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::setCollidableInferface(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn for TempExplosionWeaponLayer {
    fn as_CollidableDestroyableDamageableLayer(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &self.base.base.base.base }
    fn as_CollidableDestroyableDamageableLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer { &mut self.base.base.base.base }
    fn damage(&mut self, p1: i32, p2: i32) { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::damage(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self), p1, p2) }
    fn getDamage(&mut self, p1: i32) -> i32 { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::getDamage(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self), p1) }
    fn isDestroyed(&mut self) -> bool { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::isDestroyed(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self)) }
}
impl crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn for TempExplosionWeaponLayer {
    fn as_AllBinaryLayer(&self) -> &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &self.base.base.base.base.base.base.base }
    fn as_AllBinaryLayer_mut(&mut self) -> &mut crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer { &mut self.base.base.base.base.base.base.base }
    fn onChangeEvent(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::view::event::ViewPositionEvent::ViewPositionEvent>>>) { crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer::onChangeEvent(crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn::as_AllBinaryGameLayer_mut(self), p1) }
    fn implmentsTickableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::implmentsTickableInterface(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self)) }
    fn implmentsCollidableInterface(&mut self) -> bool { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::implmentsCollidableInterface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
    fn getType(&mut self) -> i32 { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::getType(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self)) }
    fn getMultiPlayerType(&mut self) -> i32 { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::getMultiPlayerType(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self)) }
    fn toString(&mut self) -> String { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toString(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for TempExplosionWeaponLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base.base.base.base.base.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base.base.base.base.base.base.base }
    fn toStringAppend(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::logic::string::StringMaker::StringMaker>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::toStringAppend(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn for TempExplosionWeaponLayer {
    fn as_MultiPlayerGameLayer(&self) -> &crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &self.base.base.base }
    fn as_MultiPlayerGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer { &mut self.base.base.base }
}
impl crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn for TempExplosionWeaponLayer {
    fn as_AllBinaryGameLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &self.base.base.base.base.base.base }
    fn as_AllBinaryGameLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer { &mut self.base.base.base.base.base.base }
    fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processTick(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface for TempExplosionWeaponLayer {
    fn getCollidableInferface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> { crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer::getCollidableInferface(crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn::as_CollidableCompositeLayer_mut(self)) }
}
impl crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayerInterface::MultiPlayerGameLayerInterface for TempExplosionWeaponLayer {
    fn getMultiplayerBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::multiplayer::layer::MultiplayerBehavior::MultiplayerBehaviorDyn>>> { crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayer::getMultiplayerBehavior(crate::org::allbinary::game::multiplayer::layer::MultiPlayerGameLayer::MultiPlayerGameLayerDyn::as_MultiPlayerGameLayer_mut(self)) }
}
impl crate::org::allbinary::game::layer::pickup::PickupCompositeInterface::PickupCompositeInterface for TempExplosionWeaponLayer {
    fn getPickupBehavior(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>> { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::getPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self)) }
    fn setPickupBehavior(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::game::layer::pickup::PickupBehavior::PickupBehavior>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer::setPickupBehavior(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn::as_CollidableDestroyableDamageableLayer_mut(self), p1) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for TempExplosionWeaponLayer {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer::set(crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn::as_WeaponLayer_mut(self), p1) }
}


