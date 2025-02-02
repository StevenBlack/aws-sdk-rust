// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details of the subscription grant.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SubscriptionGrantSummary {
    /// <p>The identifier of the subscription grant.</p>
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The datazone user who created the subscription grant.</p>
    pub created_by: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon DataZone user who updated the subscription grant.</p>
    pub updated_by: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the Amazon DataZone domain in which a subscription grant exists.</p>
    pub domain_id: ::std::option::Option<::std::string::String>,
    /// <p>The timestamp of when a subscription grant was created.</p>
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The timestampf of when the subscription grant was updated.</p>
    pub updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The identifier of the target of the subscription grant.</p>
    pub subscription_target_id: ::std::option::Option<::std::string::String>,
    /// <p>The entity to which the subscription is granted.</p>
    pub granted_entity: ::std::option::Option<crate::types::GrantedEntity>,
    /// <p>The status of the subscription grant.</p>
    pub status: ::std::option::Option<crate::types::SubscriptionGrantOverallStatus>,
    /// <p>The assets included in the subscription grant.</p>
    pub assets: ::std::option::Option<::std::vec::Vec<crate::types::SubscribedAsset>>,
    /// <p>The ID of the subscription grant.</p>
    pub subscription_id: ::std::option::Option<::std::string::String>,
}
impl SubscriptionGrantSummary {
    /// <p>The identifier of the subscription grant.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The datazone user who created the subscription grant.</p>
    pub fn created_by(&self) -> ::std::option::Option<&str> {
        self.created_by.as_deref()
    }
    /// <p>The Amazon DataZone user who updated the subscription grant.</p>
    pub fn updated_by(&self) -> ::std::option::Option<&str> {
        self.updated_by.as_deref()
    }
    /// <p>The identifier of the Amazon DataZone domain in which a subscription grant exists.</p>
    pub fn domain_id(&self) -> ::std::option::Option<&str> {
        self.domain_id.as_deref()
    }
    /// <p>The timestamp of when a subscription grant was created.</p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>The timestampf of when the subscription grant was updated.</p>
    pub fn updated_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.updated_at.as_ref()
    }
    /// <p>The identifier of the target of the subscription grant.</p>
    pub fn subscription_target_id(&self) -> ::std::option::Option<&str> {
        self.subscription_target_id.as_deref()
    }
    /// <p>The entity to which the subscription is granted.</p>
    pub fn granted_entity(&self) -> ::std::option::Option<&crate::types::GrantedEntity> {
        self.granted_entity.as_ref()
    }
    /// <p>The status of the subscription grant.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::SubscriptionGrantOverallStatus> {
        self.status.as_ref()
    }
    /// <p>The assets included in the subscription grant.</p>
    pub fn assets(&self) -> ::std::option::Option<&[crate::types::SubscribedAsset]> {
        self.assets.as_deref()
    }
    /// <p>The ID of the subscription grant.</p>
    pub fn subscription_id(&self) -> ::std::option::Option<&str> {
        self.subscription_id.as_deref()
    }
}
impl SubscriptionGrantSummary {
    /// Creates a new builder-style object to manufacture [`SubscriptionGrantSummary`](crate::types::SubscriptionGrantSummary).
    pub fn builder() -> crate::types::builders::SubscriptionGrantSummaryBuilder {
        crate::types::builders::SubscriptionGrantSummaryBuilder::default()
    }
}

/// A builder for [`SubscriptionGrantSummary`](crate::types::SubscriptionGrantSummary).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct SubscriptionGrantSummaryBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) created_by: ::std::option::Option<::std::string::String>,
    pub(crate) updated_by: ::std::option::Option<::std::string::String>,
    pub(crate) domain_id: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) subscription_target_id: ::std::option::Option<::std::string::String>,
    pub(crate) granted_entity: ::std::option::Option<crate::types::GrantedEntity>,
    pub(crate) status: ::std::option::Option<crate::types::SubscriptionGrantOverallStatus>,
    pub(crate) assets: ::std::option::Option<::std::vec::Vec<crate::types::SubscribedAsset>>,
    pub(crate) subscription_id: ::std::option::Option<::std::string::String>,
}
impl SubscriptionGrantSummaryBuilder {
    /// <p>The identifier of the subscription grant.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the subscription grant.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The identifier of the subscription grant.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// <p>The datazone user who created the subscription grant.</p>
    pub fn created_by(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.created_by = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The datazone user who created the subscription grant.</p>
    pub fn set_created_by(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.created_by = input;
        self
    }
    /// <p>The datazone user who created the subscription grant.</p>
    pub fn get_created_by(&self) -> &::std::option::Option<::std::string::String> {
        &self.created_by
    }
    /// <p>The Amazon DataZone user who updated the subscription grant.</p>
    pub fn updated_by(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.updated_by = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon DataZone user who updated the subscription grant.</p>
    pub fn set_updated_by(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.updated_by = input;
        self
    }
    /// <p>The Amazon DataZone user who updated the subscription grant.</p>
    pub fn get_updated_by(&self) -> &::std::option::Option<::std::string::String> {
        &self.updated_by
    }
    /// <p>The identifier of the Amazon DataZone domain in which a subscription grant exists.</p>
    pub fn domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the Amazon DataZone domain in which a subscription grant exists.</p>
    pub fn set_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_id = input;
        self
    }
    /// <p>The identifier of the Amazon DataZone domain in which a subscription grant exists.</p>
    pub fn get_domain_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.domain_id
    }
    /// <p>The timestamp of when a subscription grant was created.</p>
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp of when a subscription grant was created.</p>
    pub fn set_created_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_at = input;
        self
    }
    /// <p>The timestamp of when a subscription grant was created.</p>
    pub fn get_created_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_at
    }
    /// <p>The timestampf of when the subscription grant was updated.</p>
    pub fn updated_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.updated_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestampf of when the subscription grant was updated.</p>
    pub fn set_updated_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.updated_at = input;
        self
    }
    /// <p>The timestampf of when the subscription grant was updated.</p>
    pub fn get_updated_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.updated_at
    }
    /// <p>The identifier of the target of the subscription grant.</p>
    pub fn subscription_target_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.subscription_target_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the target of the subscription grant.</p>
    pub fn set_subscription_target_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.subscription_target_id = input;
        self
    }
    /// <p>The identifier of the target of the subscription grant.</p>
    pub fn get_subscription_target_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.subscription_target_id
    }
    /// <p>The entity to which the subscription is granted.</p>
    pub fn granted_entity(mut self, input: crate::types::GrantedEntity) -> Self {
        self.granted_entity = ::std::option::Option::Some(input);
        self
    }
    /// <p>The entity to which the subscription is granted.</p>
    pub fn set_granted_entity(mut self, input: ::std::option::Option<crate::types::GrantedEntity>) -> Self {
        self.granted_entity = input;
        self
    }
    /// <p>The entity to which the subscription is granted.</p>
    pub fn get_granted_entity(&self) -> &::std::option::Option<crate::types::GrantedEntity> {
        &self.granted_entity
    }
    /// <p>The status of the subscription grant.</p>
    pub fn status(mut self, input: crate::types::SubscriptionGrantOverallStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the subscription grant.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::SubscriptionGrantOverallStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The status of the subscription grant.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::SubscriptionGrantOverallStatus> {
        &self.status
    }
    /// Appends an item to `assets`.
    ///
    /// To override the contents of this collection use [`set_assets`](Self::set_assets).
    ///
    /// <p>The assets included in the subscription grant.</p>
    pub fn assets(mut self, input: crate::types::SubscribedAsset) -> Self {
        let mut v = self.assets.unwrap_or_default();
        v.push(input);
        self.assets = ::std::option::Option::Some(v);
        self
    }
    /// <p>The assets included in the subscription grant.</p>
    pub fn set_assets(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SubscribedAsset>>) -> Self {
        self.assets = input;
        self
    }
    /// <p>The assets included in the subscription grant.</p>
    pub fn get_assets(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SubscribedAsset>> {
        &self.assets
    }
    /// <p>The ID of the subscription grant.</p>
    pub fn subscription_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.subscription_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the subscription grant.</p>
    pub fn set_subscription_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.subscription_id = input;
        self
    }
    /// <p>The ID of the subscription grant.</p>
    pub fn get_subscription_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.subscription_id
    }
    /// Consumes the builder and constructs a [`SubscriptionGrantSummary`](crate::types::SubscriptionGrantSummary).
    pub fn build(self) -> crate::types::SubscriptionGrantSummary {
        crate::types::SubscriptionGrantSummary {
            id: self.id,
            created_by: self.created_by,
            updated_by: self.updated_by,
            domain_id: self.domain_id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            subscription_target_id: self.subscription_target_id,
            granted_entity: self.granted_entity,
            status: self.status,
            assets: self.assets,
            subscription_id: self.subscription_id,
        }
    }
}
