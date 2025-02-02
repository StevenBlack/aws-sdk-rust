// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListExportErrors`](crate::operation::list_export_errors::builders::ListExportErrorsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_export_errors::builders::ListExportErrorsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`export_id(impl Into<String>)`](crate::operation::list_export_errors::builders::ListExportErrorsFluentBuilder::export_id) / [`set_export_id(Option<String>)`](crate::operation::list_export_errors::builders::ListExportErrorsFluentBuilder::set_export_id): <p>List export errors request export id.</p>
    ///   - [`max_results(i32)`](crate::operation::list_export_errors::builders::ListExportErrorsFluentBuilder::max_results) / [`set_max_results(i32)`](crate::operation::list_export_errors::builders::ListExportErrorsFluentBuilder::set_max_results): <p>List export errors request max results.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_export_errors::builders::ListExportErrorsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_export_errors::builders::ListExportErrorsFluentBuilder::set_next_token): <p>List export errors request next token.</p>
    /// - On success, responds with [`ListExportErrorsOutput`](crate::operation::list_export_errors::ListExportErrorsOutput) with field(s):
    ///   - [`items(Option<Vec<ExportTaskError>>)`](crate::operation::list_export_errors::ListExportErrorsOutput::items): <p>List export errors response items.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_export_errors::ListExportErrorsOutput::next_token): <p>List export errors response next token.</p>
    /// - On failure, responds with [`SdkError<ListExportErrorsError>`](crate::operation::list_export_errors::ListExportErrorsError)
    pub fn list_export_errors(&self) -> crate::operation::list_export_errors::builders::ListExportErrorsFluentBuilder {
        crate::operation::list_export_errors::builders::ListExportErrorsFluentBuilder::new(self.handle.clone())
    }
}
