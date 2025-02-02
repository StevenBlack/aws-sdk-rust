// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeTemplateAlias`](crate::operation::describe_template_alias::builders::DescribeTemplateAliasFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::operation::describe_template_alias::builders::DescribeTemplateAliasFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::describe_template_alias::builders::DescribeTemplateAliasFluentBuilder::set_aws_account_id): <p>The ID of the Amazon Web Services account that contains the template alias that you're describing.</p>
    ///   - [`template_id(impl Into<String>)`](crate::operation::describe_template_alias::builders::DescribeTemplateAliasFluentBuilder::template_id) / [`set_template_id(Option<String>)`](crate::operation::describe_template_alias::builders::DescribeTemplateAliasFluentBuilder::set_template_id): <p>The ID for the template.</p>
    ///   - [`alias_name(impl Into<String>)`](crate::operation::describe_template_alias::builders::DescribeTemplateAliasFluentBuilder::alias_name) / [`set_alias_name(Option<String>)`](crate::operation::describe_template_alias::builders::DescribeTemplateAliasFluentBuilder::set_alias_name): <p>The name of the template alias that you want to describe. If you name a specific alias, you describe the version that the alias points to. You can specify the latest version of the template by providing the keyword <code>$LATEST</code> in the <code>AliasName</code> parameter. The keyword <code>$PUBLISHED</code> doesn't apply to templates.</p>
    /// - On success, responds with [`DescribeTemplateAliasOutput`](crate::operation::describe_template_alias::DescribeTemplateAliasOutput) with field(s):
    ///   - [`template_alias(Option<TemplateAlias>)`](crate::operation::describe_template_alias::DescribeTemplateAliasOutput::template_alias): <p>Information about the template alias.</p>
    ///   - [`status(i32)`](crate::operation::describe_template_alias::DescribeTemplateAliasOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::describe_template_alias::DescribeTemplateAliasOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    /// - On failure, responds with [`SdkError<DescribeTemplateAliasError>`](crate::operation::describe_template_alias::DescribeTemplateAliasError)
    pub fn describe_template_alias(&self) -> crate::operation::describe_template_alias::builders::DescribeTemplateAliasFluentBuilder {
        crate::operation::describe_template_alias::builders::DescribeTemplateAliasFluentBuilder::new(self.handle.clone())
    }
}
