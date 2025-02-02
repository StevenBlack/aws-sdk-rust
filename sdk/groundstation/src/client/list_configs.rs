// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListConfigs`](crate::operation::list_configs::builders::ListConfigsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_configs::builders::ListConfigsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_configs::builders::ListConfigsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_configs::builders::ListConfigsFluentBuilder::set_max_results): <p>Maximum number of <code>Configs</code> returned.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_configs::builders::ListConfigsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_configs::builders::ListConfigsFluentBuilder::set_next_token): <p>Next token returned in the request of a previous <code>ListConfigs</code> call. Used to get the next page of results.</p>
    /// - On success, responds with [`ListConfigsOutput`](crate::operation::list_configs::ListConfigsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_configs::ListConfigsOutput::next_token): <p>Next token returned in the response of a previous <code>ListConfigs</code> call. Used to get the next page of results.</p>
    ///   - [`config_list(Option<Vec<ConfigListItem>>)`](crate::operation::list_configs::ListConfigsOutput::config_list): <p>List of <code>Config</code> items.</p>
    /// - On failure, responds with [`SdkError<ListConfigsError>`](crate::operation::list_configs::ListConfigsError)
    pub fn list_configs(&self) -> crate::operation::list_configs::builders::ListConfigsFluentBuilder {
        crate::operation::list_configs::builders::ListConfigsFluentBuilder::new(self.handle.clone())
    }
}
