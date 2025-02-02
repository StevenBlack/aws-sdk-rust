// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details of the Amazon Redshift cluster storage.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RedshiftClusterStorage {
    /// <p>The name of an Amazon Redshift cluster.</p>
    pub cluster_name: ::std::option::Option<::std::string::String>,
}
impl RedshiftClusterStorage {
    /// <p>The name of an Amazon Redshift cluster.</p>
    pub fn cluster_name(&self) -> ::std::option::Option<&str> {
        self.cluster_name.as_deref()
    }
}
impl RedshiftClusterStorage {
    /// Creates a new builder-style object to manufacture [`RedshiftClusterStorage`](crate::types::RedshiftClusterStorage).
    pub fn builder() -> crate::types::builders::RedshiftClusterStorageBuilder {
        crate::types::builders::RedshiftClusterStorageBuilder::default()
    }
}

/// A builder for [`RedshiftClusterStorage`](crate::types::RedshiftClusterStorage).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RedshiftClusterStorageBuilder {
    pub(crate) cluster_name: ::std::option::Option<::std::string::String>,
}
impl RedshiftClusterStorageBuilder {
    /// <p>The name of an Amazon Redshift cluster.</p>
    pub fn cluster_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cluster_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of an Amazon Redshift cluster.</p>
    pub fn set_cluster_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cluster_name = input;
        self
    }
    /// <p>The name of an Amazon Redshift cluster.</p>
    pub fn get_cluster_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.cluster_name
    }
    /// Consumes the builder and constructs a [`RedshiftClusterStorage`](crate::types::RedshiftClusterStorage).
    pub fn build(self) -> crate::types::RedshiftClusterStorage {
        crate::types::RedshiftClusterStorage {
            cluster_name: self.cluster_name,
        }
    }
}
