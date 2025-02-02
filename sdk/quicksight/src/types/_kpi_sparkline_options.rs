// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The options that determine the visibility, color, type, and tooltip visibility of the sparkline of a KPI visual.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct KpiSparklineOptions {
    /// <p>The visibility of the sparkline.</p>
    pub visibility: ::std::option::Option<crate::types::Visibility>,
    /// <p>The type of the sparkline.</p>
    pub r#type: ::std::option::Option<crate::types::KpiSparklineType>,
    /// <p>The color of the sparkline.</p>
    pub color: ::std::option::Option<::std::string::String>,
    /// <p>The tooltip visibility of the sparkline.</p>
    pub tooltip_visibility: ::std::option::Option<crate::types::Visibility>,
}
impl KpiSparklineOptions {
    /// <p>The visibility of the sparkline.</p>
    pub fn visibility(&self) -> ::std::option::Option<&crate::types::Visibility> {
        self.visibility.as_ref()
    }
    /// <p>The type of the sparkline.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::KpiSparklineType> {
        self.r#type.as_ref()
    }
    /// <p>The color of the sparkline.</p>
    pub fn color(&self) -> ::std::option::Option<&str> {
        self.color.as_deref()
    }
    /// <p>The tooltip visibility of the sparkline.</p>
    pub fn tooltip_visibility(&self) -> ::std::option::Option<&crate::types::Visibility> {
        self.tooltip_visibility.as_ref()
    }
}
impl KpiSparklineOptions {
    /// Creates a new builder-style object to manufacture [`KpiSparklineOptions`](crate::types::KpiSparklineOptions).
    pub fn builder() -> crate::types::builders::KpiSparklineOptionsBuilder {
        crate::types::builders::KpiSparklineOptionsBuilder::default()
    }
}

/// A builder for [`KpiSparklineOptions`](crate::types::KpiSparklineOptions).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct KpiSparklineOptionsBuilder {
    pub(crate) visibility: ::std::option::Option<crate::types::Visibility>,
    pub(crate) r#type: ::std::option::Option<crate::types::KpiSparklineType>,
    pub(crate) color: ::std::option::Option<::std::string::String>,
    pub(crate) tooltip_visibility: ::std::option::Option<crate::types::Visibility>,
}
impl KpiSparklineOptionsBuilder {
    /// <p>The visibility of the sparkline.</p>
    pub fn visibility(mut self, input: crate::types::Visibility) -> Self {
        self.visibility = ::std::option::Option::Some(input);
        self
    }
    /// <p>The visibility of the sparkline.</p>
    pub fn set_visibility(mut self, input: ::std::option::Option<crate::types::Visibility>) -> Self {
        self.visibility = input;
        self
    }
    /// <p>The visibility of the sparkline.</p>
    pub fn get_visibility(&self) -> &::std::option::Option<crate::types::Visibility> {
        &self.visibility
    }
    /// <p>The type of the sparkline.</p>
    pub fn r#type(mut self, input: crate::types::KpiSparklineType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of the sparkline.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::KpiSparklineType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The type of the sparkline.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::KpiSparklineType> {
        &self.r#type
    }
    /// <p>The color of the sparkline.</p>
    pub fn color(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.color = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The color of the sparkline.</p>
    pub fn set_color(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.color = input;
        self
    }
    /// <p>The color of the sparkline.</p>
    pub fn get_color(&self) -> &::std::option::Option<::std::string::String> {
        &self.color
    }
    /// <p>The tooltip visibility of the sparkline.</p>
    pub fn tooltip_visibility(mut self, input: crate::types::Visibility) -> Self {
        self.tooltip_visibility = ::std::option::Option::Some(input);
        self
    }
    /// <p>The tooltip visibility of the sparkline.</p>
    pub fn set_tooltip_visibility(mut self, input: ::std::option::Option<crate::types::Visibility>) -> Self {
        self.tooltip_visibility = input;
        self
    }
    /// <p>The tooltip visibility of the sparkline.</p>
    pub fn get_tooltip_visibility(&self) -> &::std::option::Option<crate::types::Visibility> {
        &self.tooltip_visibility
    }
    /// Consumes the builder and constructs a [`KpiSparklineOptions`](crate::types::KpiSparklineOptions).
    pub fn build(self) -> crate::types::KpiSparklineOptions {
        crate::types::KpiSparklineOptions {
            visibility: self.visibility,
            r#type: self.r#type,
            color: self.color,
            tooltip_visibility: self.tooltip_visibility,
        }
    }
}
