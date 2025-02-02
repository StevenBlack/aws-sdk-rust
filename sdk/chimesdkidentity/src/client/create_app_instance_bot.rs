// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateAppInstanceBot`](crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_instance_arn(impl Into<String>)`](crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotFluentBuilder::app_instance_arn) / [`set_app_instance_arn(Option<String>)`](crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotFluentBuilder::set_app_instance_arn): <p>The ARN of the <code>AppInstance</code> request.</p>
    ///   - [`name(impl Into<String>)`](crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotFluentBuilder::set_name): <p>The user's name.</p>
    ///   - [`metadata(impl Into<String>)`](crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotFluentBuilder::metadata) / [`set_metadata(Option<String>)`](crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotFluentBuilder::set_metadata): <p>The request metadata. Limited to a 1KB string in UTF-8.</p>
    ///   - [`client_request_token(impl Into<String>)`](crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotFluentBuilder::set_client_request_token): <p>The unique ID for the client making the request. Use different tokens for different <code>AppInstanceBots</code>.</p>
    ///   - [`tags(Tag)`](crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotFluentBuilder::set_tags): <p>The tags assigned to the <code>AppInstanceBot</code>.</p>
    ///   - [`configuration(Configuration)`](crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotFluentBuilder::configuration) / [`set_configuration(Option<Configuration>)`](crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotFluentBuilder::set_configuration): <p>Configuration information about the Amazon Lex V2 V2 bot.</p>
    /// - On success, responds with [`CreateAppInstanceBotOutput`](crate::operation::create_app_instance_bot::CreateAppInstanceBotOutput) with field(s):
    ///   - [`app_instance_bot_arn(Option<String>)`](crate::operation::create_app_instance_bot::CreateAppInstanceBotOutput::app_instance_bot_arn): <p>The ARN of the <code>AppinstanceBot</code>.</p>
    /// - On failure, responds with [`SdkError<CreateAppInstanceBotError>`](crate::operation::create_app_instance_bot::CreateAppInstanceBotError)
    pub fn create_app_instance_bot(&self) -> crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotFluentBuilder {
        crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotFluentBuilder::new(self.handle.clone())
    }
}
