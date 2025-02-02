// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListExperimentTemplates`](crate::operation::list_experiment_templates::builders::ListExperimentTemplatesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_experiment_templates::builders::ListExperimentTemplatesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_experiment_templates::builders::ListExperimentTemplatesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_experiment_templates::builders::ListExperimentTemplatesFluentBuilder::set_max_results): <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_experiment_templates::builders::ListExperimentTemplatesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_experiment_templates::builders::ListExperimentTemplatesFluentBuilder::set_next_token): <p>The token for the next page of results.</p>
    /// - On success, responds with [`ListExperimentTemplatesOutput`](crate::operation::list_experiment_templates::ListExperimentTemplatesOutput) with field(s):
    ///   - [`experiment_templates(Option<Vec<ExperimentTemplateSummary>>)`](crate::operation::list_experiment_templates::ListExperimentTemplatesOutput::experiment_templates): <p>The experiment templates.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_experiment_templates::ListExperimentTemplatesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<ListExperimentTemplatesError>`](crate::operation::list_experiment_templates::ListExperimentTemplatesError)
    pub fn list_experiment_templates(&self) -> crate::operation::list_experiment_templates::builders::ListExperimentTemplatesFluentBuilder {
        crate::operation::list_experiment_templates::builders::ListExperimentTemplatesFluentBuilder::new(self.handle.clone())
    }
}
