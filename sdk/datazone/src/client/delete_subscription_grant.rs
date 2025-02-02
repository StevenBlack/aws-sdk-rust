// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteSubscriptionGrant`](crate::operation::delete_subscription_grant::builders::DeleteSubscriptionGrantFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_identifier(impl Into<String>)`](crate::operation::delete_subscription_grant::builders::DeleteSubscriptionGrantFluentBuilder::domain_identifier) / [`set_domain_identifier(Option<String>)`](crate::operation::delete_subscription_grant::builders::DeleteSubscriptionGrantFluentBuilder::set_domain_identifier): <p>The ID of the Amazon DataZone domain where the subscription grant is deleted.</p>
    ///   - [`identifier(impl Into<String>)`](crate::operation::delete_subscription_grant::builders::DeleteSubscriptionGrantFluentBuilder::identifier) / [`set_identifier(Option<String>)`](crate::operation::delete_subscription_grant::builders::DeleteSubscriptionGrantFluentBuilder::set_identifier): <p>The ID of the subscription grant that is deleted.</p>
    /// - On success, responds with [`DeleteSubscriptionGrantOutput`](crate::operation::delete_subscription_grant::DeleteSubscriptionGrantOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::operation::delete_subscription_grant::DeleteSubscriptionGrantOutput::id): <p>The ID of the subscription grant that is deleted.</p>
    ///   - [`created_by(Option<String>)`](crate::operation::delete_subscription_grant::DeleteSubscriptionGrantOutput::created_by): <p>The Amazon DataZone user who created the subscription grant that is deleted.</p>
    ///   - [`updated_by(Option<String>)`](crate::operation::delete_subscription_grant::DeleteSubscriptionGrantOutput::updated_by): <p>The Amazon DataZone user who updated the subscription grant that is deleted.</p>
    ///   - [`domain_id(Option<String>)`](crate::operation::delete_subscription_grant::DeleteSubscriptionGrantOutput::domain_id): <p>The ID of the Amazon DataZone domain in which the subscription grant is deleted.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::operation::delete_subscription_grant::DeleteSubscriptionGrantOutput::created_at): <p>The timestamp of when the subscription grant that is deleted was created.</p>
    ///   - [`updated_at(Option<DateTime>)`](crate::operation::delete_subscription_grant::DeleteSubscriptionGrantOutput::updated_at): <p>The timestamp of when the subscription grant that is deleted was updated.</p>
    ///   - [`subscription_target_id(Option<String>)`](crate::operation::delete_subscription_grant::DeleteSubscriptionGrantOutput::subscription_target_id): <p>The ID of the subscription target associated with the subscription grant that is deleted.</p>
    ///   - [`granted_entity(Option<GrantedEntity>)`](crate::operation::delete_subscription_grant::DeleteSubscriptionGrantOutput::granted_entity): <p>The entity to which the subscription is deleted.</p>
    ///   - [`status(Option<SubscriptionGrantOverallStatus>)`](crate::operation::delete_subscription_grant::DeleteSubscriptionGrantOutput::status): <p>The status of the subscription grant that is deleted.</p>
    ///   - [`assets(Option<Vec<SubscribedAsset>>)`](crate::operation::delete_subscription_grant::DeleteSubscriptionGrantOutput::assets): <p>The assets for which the subsctiption grant that is deleted gave access.</p>
    ///   - [`subscription_id(Option<String>)`](crate::operation::delete_subscription_grant::DeleteSubscriptionGrantOutput::subscription_id): <p>The identifier of the subsctiption whose subscription grant is to be deleted.</p>
    /// - On failure, responds with [`SdkError<DeleteSubscriptionGrantError>`](crate::operation::delete_subscription_grant::DeleteSubscriptionGrantError)
    pub fn delete_subscription_grant(&self) -> crate::operation::delete_subscription_grant::builders::DeleteSubscriptionGrantFluentBuilder {
        crate::operation::delete_subscription_grant::builders::DeleteSubscriptionGrantFluentBuilder::new(self.handle.clone())
    }
}
