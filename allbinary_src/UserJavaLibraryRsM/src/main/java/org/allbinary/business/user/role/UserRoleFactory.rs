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
// package org.allbinary.business.user.role
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::modules::UserFactory::UserFactory;
use crate::org::allbinary::business::user::modules::admin::AdminUserFactory::AdminUserFactory;
use crate::org::allbinary::business::user::modules::admin::store::StoreAdminUserFactory::StoreAdminUserFactory;
use crate::org::allbinary::business::user::modules::admin::store::customer::CustomerStoreAdminUserFactory::CustomerStoreAdminUserFactory;
use crate::org::allbinary::business::user::modules::admin::store::order::OrderStoreAdminUserFactory::OrderStoreAdminUserFactory;
use crate::org::allbinary::business::user::modules::admin::store::order::adjust::AdjustOrderStoreAdminUserFactory::AdjustOrderStoreAdminUserFactory;
use crate::org::allbinary::business::user::modules::admin::store::order::review::ReviewOrderStoreAdminUserFactory::ReviewOrderStoreAdminUserFactory;
use crate::org::allbinary::business::user::modules::admin::store::order::ship::ShipOrderStoreAdminUserFactory::ShipOrderStoreAdminUserFactory;
use crate::org::allbinary::business::user::modules::admin::store::payment::PaymentStoreAdminUserFactory::PaymentStoreAdminUserFactory;
use crate::org::allbinary::business::user::modules::admin::store::product::ProductStoreAdminUserFactory::ProductStoreAdminUserFactory;
use crate::org::allbinary::business::user::modules::admin::store::web::WebStoreAdminUserFactory::WebStoreAdminUserFactory;
use crate::org::allbinary::business::user::modules::admin::store::web::view::editor::ViewEditorWebStoreAdminUserFactory::ViewEditorWebStoreAdminUserFactory;
use crate::org::allbinary::business::user::modules::admin::store::web::wizard::WizardWebStoreAdminUserFactory::WizardWebStoreAdminUserFactory;
use crate::org::allbinary::business::user::modules::admin::store::web::workflow::editor::WorkflowEditorWebStoreAdminUserFactory::WorkflowEditorWebStoreAdminUserFactory;
use crate::org::allbinary::business::user::modules::customer::subscriber::SubscriberUserFactory::SubscriberUserFactory;
use crate::org::allbinary::business::user::modules::customer::wholesale::WholesaleUserFactory::WholesaleUserFactory;
use super::BasicUserRoleFactory::BasicUserRoleFactory;
use super::UserRoleB::UserRoleB;
use super::UserRole::UserRole;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserRoleFactory {
    pub basicUserRoleFactory: Option<Rc<RefCell<BasicUserRoleFactory>>>,
    pub INSTALLER: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub ADMINISTRATOR: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub CUSTOMER: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub SUBSCRIBERCUSTOMER: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub WHOLESALECUSTOMER: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub STOREMANAGER: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub PRODUCTMANAGER: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub CUSTOMERMANAGER: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub PAYMENTMANAGER: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub ORDERMANAGER: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub WEBMANAGER: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub REVIEWER: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub SHIPPING: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub ADJUSTER: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub WORKFLOWEDITOR: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub WIZARD: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub VIEWEDITOR: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
}

thread_local! { pub static UserRoleFactory_instance: Option<Rc<RefCell<UserRoleFactory>>> = UserRoleFactory::new(); }

impl UserRoleFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            basicUserRoleFactory: None,
            INSTALLER: None,
            ADMINISTRATOR: None,
            CUSTOMER: None,
            SUBSCRIBERCUSTOMER: None,
            WHOLESALECUSTOMER: None,
            STOREMANAGER: None,
            PRODUCTMANAGER: None,
            CUSTOMERMANAGER: None,
            PAYMENTMANAGER: None,
            ORDERMANAGER: None,
            WEBMANAGER: None,
            REVIEWER: None,
            SHIPPING: None,
            ADJUSTER: None,
            WORKFLOWEDITOR: None,
            WIZARD: None,
            VIEWEDITOR: None,
        };
        this.basicUserRoleFactory = BasicUserRoleFactory::getInstance();
        this.INSTALLER = UserRoleB::new(jcall!(this.basicUserRoleFactory).INSTALLER.clone(), None).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.ADMINISTRATOR = UserRoleB::new(jcall!(this.basicUserRoleFactory).ADMINISTRATOR.clone(), AdminUserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.CUSTOMER = UserRoleB::new(jcall!(this.basicUserRoleFactory).CUSTOMER.clone(), UserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.SUBSCRIBERCUSTOMER = UserRoleB::new(jcall!(this.basicUserRoleFactory).SUBSCRIBERCUSTOMER.clone(), SubscriberUserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.WHOLESALECUSTOMER = UserRoleB::new(jcall!(this.basicUserRoleFactory).WHOLESALECUSTOMER.clone(), WholesaleUserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.STOREMANAGER = UserRoleB::new(jcall!(this.basicUserRoleFactory).STOREMANAGER.clone(), StoreAdminUserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.PRODUCTMANAGER = UserRoleB::new(jcall!(this.basicUserRoleFactory).PRODUCTMANAGER.clone(), ProductStoreAdminUserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.CUSTOMERMANAGER = UserRoleB::new(jcall!(this.basicUserRoleFactory).CUSTOMERMANAGER.clone(), CustomerStoreAdminUserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.PAYMENTMANAGER = UserRoleB::new(jcall!(this.basicUserRoleFactory).PAYMENTMANAGER.clone(), PaymentStoreAdminUserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.ORDERMANAGER = UserRoleB::new(jcall!(this.basicUserRoleFactory).ORDERMANAGER.clone(), OrderStoreAdminUserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.WEBMANAGER = UserRoleB::new(jcall!(this.basicUserRoleFactory).WEBMANAGER.clone(), WebStoreAdminUserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.REVIEWER = UserRoleB::new(jcall!(this.basicUserRoleFactory).REVIEWER.clone(), ReviewOrderStoreAdminUserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.SHIPPING = UserRoleB::new(jcall!(this.basicUserRoleFactory).SHIPPING.clone(), ShipOrderStoreAdminUserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.ADJUSTER = UserRoleB::new(jcall!(this.basicUserRoleFactory).ADJUSTER.clone(), AdjustOrderStoreAdminUserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.WORKFLOWEDITOR = UserRoleB::new(jcall!(this.basicUserRoleFactory).WORKFLOWEDITOR.clone(), WorkflowEditorWebStoreAdminUserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.WIZARD = UserRoleB::new(jcall!(this.basicUserRoleFactory).WIZARD.clone(), WizardWebStoreAdminUserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.VIEWEDITOR = UserRoleB::new(jcall!(this.basicUserRoleFactory).VIEWEDITOR.clone(), ViewEditorWebStoreAdminUserFactory::new()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<UserRoleFactory>>> {
        return UserRoleFactory_instance.with(Clone::clone).clone();
    }
}

