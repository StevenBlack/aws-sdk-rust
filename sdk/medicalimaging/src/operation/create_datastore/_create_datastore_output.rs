// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateDatastoreOutput {
    /// <p>The data store identifier.</p>
    pub datastore_id: ::std::option::Option<::std::string::String>,
    /// <p>The data store status.</p>
    pub datastore_status: ::std::option::Option<crate::types::DatastoreStatus>,
    _request_id: Option<String>,
}
impl CreateDatastoreOutput {
    /// <p>The data store identifier.</p>
    pub fn datastore_id(&self) -> ::std::option::Option<&str> {
        self.datastore_id.as_deref()
    }
    /// <p>The data store status.</p>
    pub fn datastore_status(&self) -> ::std::option::Option<&crate::types::DatastoreStatus> {
        self.datastore_status.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for CreateDatastoreOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateDatastoreOutput {
    /// Creates a new builder-style object to manufacture [`CreateDatastoreOutput`](crate::operation::create_datastore::CreateDatastoreOutput).
    pub fn builder() -> crate::operation::create_datastore::builders::CreateDatastoreOutputBuilder {
        crate::operation::create_datastore::builders::CreateDatastoreOutputBuilder::default()
    }
}

/// A builder for [`CreateDatastoreOutput`](crate::operation::create_datastore::CreateDatastoreOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateDatastoreOutputBuilder {
    pub(crate) datastore_id: ::std::option::Option<::std::string::String>,
    pub(crate) datastore_status: ::std::option::Option<crate::types::DatastoreStatus>,
    _request_id: Option<String>,
}
impl CreateDatastoreOutputBuilder {
    /// <p>The data store identifier.</p>
    pub fn datastore_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.datastore_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The data store identifier.</p>
    pub fn set_datastore_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.datastore_id = input;
        self
    }
    /// <p>The data store identifier.</p>
    pub fn get_datastore_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.datastore_id
    }
    /// <p>The data store status.</p>
    pub fn datastore_status(mut self, input: crate::types::DatastoreStatus) -> Self {
        self.datastore_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The data store status.</p>
    pub fn set_datastore_status(mut self, input: ::std::option::Option<crate::types::DatastoreStatus>) -> Self {
        self.datastore_status = input;
        self
    }
    /// <p>The data store status.</p>
    pub fn get_datastore_status(&self) -> &::std::option::Option<crate::types::DatastoreStatus> {
        &self.datastore_status
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateDatastoreOutput`](crate::operation::create_datastore::CreateDatastoreOutput).
    pub fn build(self) -> crate::operation::create_datastore::CreateDatastoreOutput {
        crate::operation::create_datastore::CreateDatastoreOutput {
            datastore_id: self.datastore_id,
            datastore_status: self.datastore_status,
            _request_id: self._request_id,
        }
    }
}
