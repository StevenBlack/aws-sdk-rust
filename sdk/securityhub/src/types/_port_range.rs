// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A range of ports.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PortRange {
    /// <p>The first port in the port range.</p>
    pub begin: i32,
    /// <p>The last port in the port range.</p>
    pub end: i32,
}
impl PortRange {
    /// <p>The first port in the port range.</p>
    pub fn begin(&self) -> i32 {
        self.begin
    }
    /// <p>The last port in the port range.</p>
    pub fn end(&self) -> i32 {
        self.end
    }
}
impl PortRange {
    /// Creates a new builder-style object to manufacture [`PortRange`](crate::types::PortRange).
    pub fn builder() -> crate::types::builders::PortRangeBuilder {
        crate::types::builders::PortRangeBuilder::default()
    }
}

/// A builder for [`PortRange`](crate::types::PortRange).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PortRangeBuilder {
    pub(crate) begin: ::std::option::Option<i32>,
    pub(crate) end: ::std::option::Option<i32>,
}
impl PortRangeBuilder {
    /// <p>The first port in the port range.</p>
    pub fn begin(mut self, input: i32) -> Self {
        self.begin = ::std::option::Option::Some(input);
        self
    }
    /// <p>The first port in the port range.</p>
    pub fn set_begin(mut self, input: ::std::option::Option<i32>) -> Self {
        self.begin = input;
        self
    }
    /// <p>The first port in the port range.</p>
    pub fn get_begin(&self) -> &::std::option::Option<i32> {
        &self.begin
    }
    /// <p>The last port in the port range.</p>
    pub fn end(mut self, input: i32) -> Self {
        self.end = ::std::option::Option::Some(input);
        self
    }
    /// <p>The last port in the port range.</p>
    pub fn set_end(mut self, input: ::std::option::Option<i32>) -> Self {
        self.end = input;
        self
    }
    /// <p>The last port in the port range.</p>
    pub fn get_end(&self) -> &::std::option::Option<i32> {
        &self.end
    }
    /// Consumes the builder and constructs a [`PortRange`](crate::types::PortRange).
    pub fn build(self) -> crate::types::PortRange {
        crate::types::PortRange {
            begin: self.begin.unwrap_or_default(),
            end: self.end.unwrap_or_default(),
        }
    }
}
