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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

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
        this.INSTALLER = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).INSTALLER.clone(); java_field }.clone(), None).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.ADMINISTRATOR = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).ADMINISTRATOR.clone(); java_field }.clone(), AdminUserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.CUSTOMER = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).CUSTOMER.clone(); java_field }.clone(), UserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.SUBSCRIBERCUSTOMER = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).SUBSCRIBERCUSTOMER.clone(); java_field }.clone(), SubscriberUserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.WHOLESALECUSTOMER = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).WHOLESALECUSTOMER.clone(); java_field }.clone(), WholesaleUserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.STOREMANAGER = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).STOREMANAGER.clone(); java_field }.clone(), StoreAdminUserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.PRODUCTMANAGER = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).PRODUCTMANAGER.clone(); java_field }.clone(), ProductStoreAdminUserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.CUSTOMERMANAGER = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).CUSTOMERMANAGER.clone(); java_field }.clone(), CustomerStoreAdminUserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.PAYMENTMANAGER = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).PAYMENTMANAGER.clone(); java_field }.clone(), PaymentStoreAdminUserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.ORDERMANAGER = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).ORDERMANAGER.clone(); java_field }.clone(), OrderStoreAdminUserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.WEBMANAGER = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).WEBMANAGER.clone(); java_field }.clone(), WebStoreAdminUserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.REVIEWER = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).REVIEWER.clone(); java_field }.clone(), ReviewOrderStoreAdminUserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.SHIPPING = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).SHIPPING.clone(); java_field }.clone(), ShipOrderStoreAdminUserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.ADJUSTER = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).ADJUSTER.clone(); java_field }.clone(), AdjustOrderStoreAdminUserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.WORKFLOWEDITOR = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).WORKFLOWEDITOR.clone(); java_field }.clone(), WorkflowEditorWebStoreAdminUserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.WIZARD = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).WIZARD.clone(); java_field }.clone(), WizardWebStoreAdminUserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this.VIEWEDITOR = UserRoleB::new({ let java_field = jcall!(this.basicUserRoleFactory).VIEWEDITOR.clone(); java_field }.clone(), ViewEditorWebStoreAdminUserFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::UserFactoryInterface::UserFactoryInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<UserRoleFactory>>> {
        return UserRoleFactory_instance.with(Clone::clone).clone();
    }
}

