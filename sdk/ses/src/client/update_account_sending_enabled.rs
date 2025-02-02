// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateAccountSendingEnabled`](crate::operation::update_account_sending_enabled::builders::UpdateAccountSendingEnabledFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`enabled(bool)`](crate::operation::update_account_sending_enabled::builders::UpdateAccountSendingEnabledFluentBuilder::enabled) / [`set_enabled(Option<bool>)`](crate::operation::update_account_sending_enabled::builders::UpdateAccountSendingEnabledFluentBuilder::set_enabled): <p>Describes whether email sending is enabled or disabled for your Amazon SES account in the current Amazon Web Services Region.</p>
    /// - On success, responds with [`UpdateAccountSendingEnabledOutput`](crate::operation::update_account_sending_enabled::UpdateAccountSendingEnabledOutput)
    /// - On failure, responds with [`SdkError<UpdateAccountSendingEnabledError>`](crate::operation::update_account_sending_enabled::UpdateAccountSendingEnabledError)
    pub fn update_account_sending_enabled(
        &self,
    ) -> crate::operation::update_account_sending_enabled::builders::UpdateAccountSendingEnabledFluentBuilder {
        crate::operation::update_account_sending_enabled::builders::UpdateAccountSendingEnabledFluentBuilder::new(self.handle.clone())
    }
}
