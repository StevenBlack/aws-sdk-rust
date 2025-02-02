// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The details of a single calculated attribute definition.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListCalculatedAttributeDefinitionItem {
    /// <p>The unique name of the calculated attribute.</p>
    pub calculated_attribute_name: ::std::option::Option<::std::string::String>,
    /// <p>The display name of the calculated attribute.</p>
    pub display_name: ::std::option::Option<::std::string::String>,
    /// <p>The threshold for the calculated attribute.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The threshold for the calculated attribute.</p>
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The timestamp of when the calculated attribute definition was most recently edited.</p>
    pub last_updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl ListCalculatedAttributeDefinitionItem {
    /// <p>The unique name of the calculated attribute.</p>
    pub fn calculated_attribute_name(&self) -> ::std::option::Option<&str> {
        self.calculated_attribute_name.as_deref()
    }
    /// <p>The display name of the calculated attribute.</p>
    pub fn display_name(&self) -> ::std::option::Option<&str> {
        self.display_name.as_deref()
    }
    /// <p>The threshold for the calculated attribute.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The threshold for the calculated attribute.</p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>The timestamp of when the calculated attribute definition was most recently edited.</p>
    pub fn last_updated_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_updated_at.as_ref()
    }
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn tags(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.tags.as_ref()
    }
}
impl ListCalculatedAttributeDefinitionItem {
    /// Creates a new builder-style object to manufacture [`ListCalculatedAttributeDefinitionItem`](crate::types::ListCalculatedAttributeDefinitionItem).
    pub fn builder() -> crate::types::builders::ListCalculatedAttributeDefinitionItemBuilder {
        crate::types::builders::ListCalculatedAttributeDefinitionItemBuilder::default()
    }
}

/// A builder for [`ListCalculatedAttributeDefinitionItem`](crate::types::ListCalculatedAttributeDefinitionItem).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListCalculatedAttributeDefinitionItemBuilder {
    pub(crate) calculated_attribute_name: ::std::option::Option<::std::string::String>,
    pub(crate) display_name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl ListCalculatedAttributeDefinitionItemBuilder {
    /// <p>The unique name of the calculated attribute.</p>
    pub fn calculated_attribute_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.calculated_attribute_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique name of the calculated attribute.</p>
    pub fn set_calculated_attribute_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.calculated_attribute_name = input;
        self
    }
    /// <p>The unique name of the calculated attribute.</p>
    pub fn get_calculated_attribute_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.calculated_attribute_name
    }
    /// <p>The display name of the calculated attribute.</p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.display_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The display name of the calculated attribute.</p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.display_name = input;
        self
    }
    /// <p>The display name of the calculated attribute.</p>
    pub fn get_display_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.display_name
    }
    /// <p>The threshold for the calculated attribute.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The threshold for the calculated attribute.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The threshold for the calculated attribute.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>The threshold for the calculated attribute.</p>
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The threshold for the calculated attribute.</p>
    pub fn set_created_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_at = input;
        self
    }
    /// <p>The threshold for the calculated attribute.</p>
    pub fn get_created_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_at
    }
    /// <p>The timestamp of when the calculated attribute definition was most recently edited.</p>
    pub fn last_updated_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp of when the calculated attribute definition was most recently edited.</p>
    pub fn set_last_updated_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_updated_at = input;
        self
    }
    /// <p>The timestamp of when the calculated attribute definition was most recently edited.</p>
    pub fn get_last_updated_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_updated_at
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`ListCalculatedAttributeDefinitionItem`](crate::types::ListCalculatedAttributeDefinitionItem).
    pub fn build(self) -> crate::types::ListCalculatedAttributeDefinitionItem {
        crate::types::ListCalculatedAttributeDefinitionItem {
            calculated_attribute_name: self.calculated_attribute_name,
            display_name: self.display_name,
            description: self.description,
            created_at: self.created_at,
            last_updated_at: self.last_updated_at,
            tags: self.tags,
        }
    }
}
