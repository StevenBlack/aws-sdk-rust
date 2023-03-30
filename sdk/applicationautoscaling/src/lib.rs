#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(rustdoc::bare_urls)]

#![warn(missing_docs)]
//! <p>With Application Auto Scaling, you can configure automatic scaling for the following
//! resources:</p>
//! <ul>
//! <li>
//! <p>Amazon AppStream 2.0 fleets</p>
//! </li>
//! <li>
//! <p>Amazon Aurora Replicas</p>
//! </li>
//! <li>
//! <p>Amazon Comprehend document classification and entity recognizer endpoints</p>
//! </li>
//! <li>
//! <p>Amazon DynamoDB tables and global secondary indexes throughput capacity</p>
//! </li>
//! <li>
//! <p>Amazon ECS services</p>
//! </li>
//! <li>
//! <p>Amazon ElastiCache for Redis clusters (replication groups)</p>
//! </li>
//! <li>
//! <p>Amazon EMR clusters</p>
//! </li>
//! <li>
//! <p>Amazon Keyspaces (for Apache Cassandra) tables</p>
//! </li>
//! <li>
//! <p>Lambda function provisioned concurrency</p>
//! </li>
//! <li>
//! <p>Amazon Managed Streaming for Apache Kafka broker storage</p>
//! </li>
//! <li>
//! <p>Amazon Neptune clusters</p>
//! </li>
//! <li>
//! <p>Amazon SageMaker endpoint variants</p>
//! </li>
//! <li>
//! <p>Spot Fleets (Amazon EC2)</p>
//! </li>
//! <li>
//! <p>Custom resources provided by your own applications or services</p>
//! </li>
//! </ul>
//! <p>
//! <b>API Summary</b>
//! </p>
//! <p>The Application Auto Scaling service API includes three key sets of actions: </p>
//! <ul>
//! <li>
//! <p>Register and manage scalable targets - Register Amazon Web Services or custom resources as scalable
//! targets (a resource that Application Auto Scaling can scale), set minimum and maximum capacity limits, and
//! retrieve information on existing scalable targets.</p>
//! </li>
//! <li>
//! <p>Configure and manage automatic scaling - Define scaling policies to dynamically scale
//! your resources in response to CloudWatch alarms, schedule one-time or recurring scaling actions,
//! and retrieve your recent scaling activity history.</p>
//! </li>
//! <li>
//! <p>Suspend and resume scaling - Temporarily suspend and later resume automatic scaling by
//! calling the <a href="https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html">RegisterScalableTarget</a> API action for any Application Auto Scaling scalable target. You can
//! suspend and resume (individually or in combination) scale-out activities that are
//! triggered by a scaling policy, scale-in activities that are triggered by a scaling policy,
//! and scheduled scaling.</p>
//! </li>
//! </ul>
//! 
//! 
//! <p>To learn more about Application Auto Scaling, including information about granting IAM users required
//! permissions for Application Auto Scaling actions, see the <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/what-is-application-auto-scaling.html">Application Auto Scaling User
//! Guide</a>.</p>
//! 
//! # Crate Organization
//! 
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//! 
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//! 
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//! 
//! The other modules within this crate are not required for normal usage.
//! 
//! # Examples
//! Examples can be found [here](https://github.com/awslabs/aws-sdk-rust/tree/main/examples/applicationautoscaling).


// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

pub use aws_credential_types::Credentials;

pub use aws_types::region::Region;

pub(crate) static API_METADATA: aws_http::user_agent::ApiMetadata =
                    aws_http::user_agent::ApiMetadata::new("applicationautoscaling", crate::PKG_VERSION);

pub use aws_types::app_name::AppName;

pub use aws_smithy_http::endpoint::Endpoint;


/// Crate version number.
                pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Client and fluent builders for calling the service.
pub mod client;

/// Configuration for the service.
pub mod config;

/// Endpoint resolution functionality
pub mod endpoint;

/// All error types that operations can return. Documentation on these types is copied from the model.
pub mod error;

mod error_meta;

/// Input structures for operations. Documentation on these types is copied from the model.
pub mod input;

/// Data structures used by operation inputs/outputs. Documentation on these types is copied from the model.
pub mod model;

/// All operations that this crate can perform.
pub mod operation;

/// Output structures for operations. Documentation on these types is copied from the model.
pub mod output;

/// Data primitives referenced by other data types.
pub mod types;

pub mod middleware;

mod no_credentials;

mod operation_deser;

mod operation_ser;

/// Paginators for the service
pub mod paginator;

mod json_deser;

mod json_ser;

/// Generated accessors for nested fields
mod lens;

/// Endpoints standard library functions
mod endpoint_lib;

mod json_errors;

#[doc(inline)]
pub use client::Client;

