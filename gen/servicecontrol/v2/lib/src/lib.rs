#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [services](resources/services/struct.ServicesActions.html)\n  * [*check*](resources/services/struct.CheckRequestBuilder.html), [*report*](resources/services/struct.ReportRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
    #[doc = "Manage your Google Service Control data\n\n`https://www.googleapis.com/auth/servicecontrol`"]
    pub const SERVICECONTROL: &str = "https://www.googleapis.com/auth/servicecontrol";
}
pub mod schemas {
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Api {
        #[doc = "The API operation name. For gRPC requests, it is the fully qualified API method name, such as “google.pubsub.v1.Publisher.Publish”. For OpenAPI requests, it is the `operationId`, such as “getPet”."]
        #[serde(
            rename = "operation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation: ::std::option::Option<String>,
        #[doc = "The API protocol used for sending the request, such as “http”, “https”, “grpc”, or “internal”."]
        #[serde(
            rename = "protocol",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protocol: ::std::option::Option<String>,
        #[doc = "The API service name. It is a logical identifier for a networked API, such as “pubsub.googleapis.com”. The naming syntax depends on the API management system being used for handling the request."]
        #[serde(
            rename = "service",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service: ::std::option::Option<String>,
        #[doc = "The API version associated with the API operation above, such as “v1” or “v1alpha1”."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Api {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Api {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct AttributeContext {
        #[doc = "Represents an API operation that is involved to a network activity."]
        #[serde(
            rename = "api",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api: ::std::option::Option<crate::schemas::Api>,
        #[doc = "The destination of a network activity, such as accepting a TCP connection. In a multi hop network activity, the destination represents the receiver of the last hop."]
        #[serde(
            rename = "destination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination: ::std::option::Option<crate::schemas::Peer>,
        #[doc = "Supports extensions for advanced use cases, such as logs and metrics."]
        #[serde(
            rename = "extensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extensions:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "The origin of a network activity. In a multi hop network activity, the origin represents the sender of the first hop. For the first hop, the `source` and the `origin` must have the same content."]
        #[serde(
            rename = "origin",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub origin: ::std::option::Option<crate::schemas::Peer>,
        #[doc = "Represents a network request, such as an HTTP request."]
        #[serde(
            rename = "request",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request: ::std::option::Option<crate::schemas::Request>,
        #[doc = "Represents a target resource that is involved with a network activity. If multiple resources are involved with an activity, this must be the primary one."]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource: ::std::option::Option<crate::schemas::Resource>,
        #[doc = "Represents a network response, such as an HTTP response."]
        #[serde(
            rename = "response",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response: ::std::option::Option<crate::schemas::Response>,
        #[doc = "The source of a network activity, such as starting a TCP connection. In a multi hop network activity, the source represents the sender of the last hop."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::Peer>,
    }
    impl ::google_field_selector::FieldSelector for AttributeContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AttributeContext {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct AuditLog {
        #[doc = "Authentication information."]
        #[serde(
            rename = "authenticationInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authentication_info: ::std::option::Option<crate::schemas::AuthenticationInfo>,
        #[doc = "Authorization information. If there are multiple resources or permissions involved, then there is one AuthorizationInfo element for each {resource, permission} tuple."]
        #[serde(
            rename = "authorizationInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authorization_info: ::std::option::Option<Vec<crate::schemas::AuthorizationInfo>>,
        #[doc = "Other service-specific data about the request, response, and other information associated with the current audited event."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The name of the service method or operation. For API calls, this should be the name of the API method. For example, “google.cloud.bigquery.v2.TableService.InsertTable” “google.logging.v2.ConfigServiceV2.CreateSink”"]
        #[serde(
            rename = "methodName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub method_name: ::std::option::Option<String>,
        #[doc = "The number of items returned from a List or Query API method, if applicable."]
        #[serde(
            rename = "numResponseItems",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_response_items: ::std::option::Option<i64>,
        #[doc = "Indicates the policy violations for this request. If the request is denied by the policy, violation information will be logged here."]
        #[serde(
            rename = "policyViolationInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_violation_info: ::std::option::Option<crate::schemas::PolicyViolationInfo>,
        #[doc = "The operation request. This may not include all request parameters, such as those that are too large, privacy-sensitive, or duplicated elsewhere in the log record. It should never include user-generated data, such as file contents. When the JSON object represented here has a proto equivalent, the proto name will be indicated in the `@type` property."]
        #[serde(
            rename = "request",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Metadata about the operation."]
        #[serde(
            rename = "requestMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_metadata: ::std::option::Option<crate::schemas::RequestMetadata>,
        #[doc = "The resource location information."]
        #[serde(
            rename = "resourceLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_location: ::std::option::Option<crate::schemas::ResourceLocation>,
        #[doc = "The resource or collection that is the target of the operation. The name is a scheme-less URI, not including the API service name. For example: “projects/PROJECT_ID/zones/us-central1-a/instances” “projects/PROJECT_ID/datasets/DATASET_ID”"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "The resource’s original state before mutation. Present only for operations which have successfully modified the targeted resource(s). In general, this field should contain all changed fields, except those that are already been included in `request`, `response`, `metadata` or `service_data` fields. When the JSON object represented here has a proto equivalent, the proto name will be indicated in the `@type` property."]
        #[serde(
            rename = "resourceOriginalState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_original_state:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The operation response. This may not include all response elements, such as those that are too large, privacy-sensitive, or duplicated elsewhere in the log record. It should never include user-generated data, such as file contents. When the JSON object represented here has a proto equivalent, the proto name will be indicated in the `@type` property."]
        #[serde(
            rename = "response",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Deprecated. Use the `metadata` field instead. Other service-specific data about the request, response, and other activities."]
        #[serde(
            rename = "serviceData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_data:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The name of the API service performing the operation. For example, `\"compute.googleapis.com\"`."]
        #[serde(
            rename = "serviceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_name: ::std::option::Option<String>,
        #[doc = "The status of the overall operation."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::Status>,
    }
    impl ::google_field_selector::FieldSelector for AuditLog {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuditLog {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Auth {
        #[doc = "A list of access level resource names that allow resources to be accessed by authenticated requester. It is part of Secure GCP processing for the incoming request. An access level string has the format: “//{api_service_name}/accessPolicies/{policy_id}/accessLevels/{short_name}” Example: “//accesscontextmanager.googleapis.com/accessPolicies/MY_POLICY_ID/accessLevels/MY_LEVEL”"]
        #[serde(
            rename = "accessLevels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_levels: ::std::option::Option<Vec<String>>,
        #[doc = "The intended audience(s) for this authentication information. Reflects the audience (`aud`) claim within a JWT. The audience value(s) depends on the `issuer`, but typically include one or more of the following pieces of information: * The services intended to receive the credential. For example, \\[“https://pubsub.googleapis.com/”, “https://storage.googleapis.com/”\\]. * A set of service-based scopes. For example, \\[“https://www.googleapis.com/auth/cloud-platform”\\]. * The client id of an app, such as the Firebase project id for JWTs from Firebase Auth. Consult the documentation for the credential issuer to determine the information provided."]
        #[serde(
            rename = "audiences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audiences: ::std::option::Option<Vec<String>>,
        #[doc = "Structured claims presented with the credential. JWTs include `{key: value}` pairs for standard and private claims. The following is a subset of the standard required and optional claims that would typically be presented for a Google-based JWT: {‘iss’: ‘accounts.google.com’, ‘sub’: ‘113289723416554971153’, ‘aud’: \\[‘123456789012’, ‘pubsub.googleapis.com’\\], ‘azp’: ‘123456789012.apps.googleusercontent.com’, ‘email’: ‘jsmith@example.com’, ‘iat’: 1353601026, ‘exp’: 1353604926} SAML assertions are similarly specified, but with an identity provider dependent structure."]
        #[serde(
            rename = "claims",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub claims:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The authorized presenter of the credential. Reflects the optional Authorized Presenter (`azp`) claim within a JWT or the OAuth client id. For example, a Google Cloud Platform client id looks as follows: “123456789012.apps.googleusercontent.com”."]
        #[serde(
            rename = "presenter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub presenter: ::std::option::Option<String>,
        #[doc = "The authenticated principal. Reflects the issuer (`iss`) and subject (`sub`) claims within a JWT. The issuer and subject should be `/` delimited, with `/` percent-encoded within the subject fragment. For Google accounts, the principal format is: “https://accounts.google.com/{id}”"]
        #[serde(
            rename = "principal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub principal: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Auth {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Auth {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct AuthenticationInfo {
        #[doc = "The authority selector specified by the requestor, if any. It is not guaranteed that the principal was allowed to use this authority."]
        #[serde(
            rename = "authoritySelector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authority_selector: ::std::option::Option<String>,
        #[doc = "The email address of the authenticated user (or service account on behalf of third party principal) making the request. For third party identity callers, the `principal_subject` field is populated instead of this field. For privacy reasons, the principal email address is sometimes redacted. For more information, see [Caller identities in audit logs](https://cloud.google.com/logging/docs/audit#user-id)."]
        #[serde(
            rename = "principalEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub principal_email: ::std::option::Option<String>,
        #[doc = "String representation of identity of requesting party. Populated for both first and third party identities."]
        #[serde(
            rename = "principalSubject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub principal_subject: ::std::option::Option<String>,
        #[doc = "Identity delegation history of an authenticated service account that makes the request. It contains information on the real authorities that try to access GCP resources by delegating on a service account. When multiple authorities present, they are guaranteed to be sorted based on the original ordering of the identity delegation events."]
        #[serde(
            rename = "serviceAccountDelegationInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_account_delegation_info:
            ::std::option::Option<Vec<crate::schemas::ServiceAccountDelegationInfo>>,
        #[doc = "The name of the service account key used to create or exchange credentials for authenticating the service account making the request. This is a scheme-less URI full resource name. For example: “//iam.googleapis.com/projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}”"]
        #[serde(
            rename = "serviceAccountKeyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_account_key_name: ::std::option::Option<String>,
        #[doc = "The third party identification (if any) of the authenticated user making the request. When the JSON object represented here has a proto equivalent, the proto name will be indicated in the `@type` property."]
        #[serde(
            rename = "thirdPartyPrincipal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub third_party_principal:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for AuthenticationInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuthenticationInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AuthorizationInfo {
        #[doc = "Whether or not authorization for `resource` and `permission` was granted."]
        #[serde(
            rename = "granted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub granted: ::std::option::Option<bool>,
        #[doc = "The required IAM permission."]
        #[serde(
            rename = "permission",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission: ::std::option::Option<String>,
        #[doc = "The resource being accessed, as a REST-style or cloud resource string. For example: bigquery.googleapis.com/projects/PROJECTID/datasets/DATASETID or projects/PROJECTID/datasets/DATASETID"]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource: ::std::option::Option<String>,
        #[doc = "Resource attributes used in IAM condition evaluation. This field contains resource attributes like resource type and resource name. To get the whole view of the attributes used in IAM condition evaluation, the user must also look into `AuditLog.request_metadata.request_attributes`."]
        #[serde(
            rename = "resourceAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_attributes: ::std::option::Option<crate::schemas::Resource>,
    }
    impl ::google_field_selector::FieldSelector for AuthorizationInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuthorizationInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct CheckRequest {
        #[doc = "Describes attributes about the operation being executed by the service."]
        #[serde(
            rename = "attributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributes: ::std::option::Option<crate::schemas::AttributeContext>,
        #[doc = "Optional. Contains a comma-separated list of flags."]
        #[serde(
            rename = "flags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub flags: ::std::option::Option<String>,
        #[doc = "Describes the resources and the policies applied to each resource."]
        #[serde(
            rename = "resources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resources: ::std::option::Option<Vec<crate::schemas::ResourceInfo>>,
        #[doc = "Specifies the version of the service configuration that should be used to process the request. Must not be empty. Set this field to ‘latest’ to specify using the latest configuration."]
        #[serde(
            rename = "serviceConfigId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_config_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CheckRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CheckRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct CheckResponse {
        #[doc = "Returns a set of request contexts generated from the `CheckRequest`."]
        #[serde(
            rename = "headers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub headers: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Operation is allowed when this field is not set. Any non-‘OK’ status indicates a denial; google.rpc.Status.details would contain additional details about the denial."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::Status>,
    }
    impl ::google_field_selector::FieldSelector for CheckResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CheckResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct FirstPartyPrincipal {
        #[doc = "The email address of a Google account. ."]
        #[serde(
            rename = "principalEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub principal_email: ::std::option::Option<String>,
        #[doc = "Metadata about the service that uses the service account. ."]
        #[serde(
            rename = "serviceMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for FirstPartyPrincipal {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FirstPartyPrincipal {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct OrgPolicyViolationInfo {
        #[doc = "Optional. Resource payload that is currently in scope and is subjected to orgpolicy conditions. This payload may be the subset of the actual Resource that may come in the request. This payload should not contain any core content."]
        #[serde(
            rename = "payload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Optional. Tags referenced on the resource at the time of evaluation. These also include the federated tags, if they are supplied in the CheckOrgPolicy or CheckCustomConstraints Requests. Optional field as of now. These tags are the Cloud tags that are available on the resource during the policy evaluation and will be available as part of the OrgPolicy check response for logging purposes."]
        #[serde(
            rename = "resourceTags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_tags: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Optional. Resource type that the orgpolicy is checked against. Example: compute.googleapis.com/Instance, store.googleapis.com/bucket"]
        #[serde(
            rename = "resourceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_type: ::std::option::Option<String>,
        #[doc = "Optional. Policy violations"]
        #[serde(
            rename = "violationInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub violation_info: ::std::option::Option<Vec<crate::schemas::ViolationInfo>>,
    }
    impl ::google_field_selector::FieldSelector for OrgPolicyViolationInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OrgPolicyViolationInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Peer {
        #[doc = "The IP address of the peer."]
        #[serde(
            rename = "ip",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ip: ::std::option::Option<String>,
        #[doc = "The labels associated with the peer."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The network port of the peer."]
        #[serde(
            rename = "port",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub port: ::std::option::Option<i64>,
        #[doc = "The identity of this peer. Similar to `Request.auth.principal`, but relative to the peer instead of the request. For example, the identity associated with a load balancer that forwarded the request."]
        #[serde(
            rename = "principal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub principal: ::std::option::Option<String>,
        #[doc = "The CLDR country/region code associated with the above IP address. If the IP address is private, the `region_code` should reflect the physical location where this peer is running."]
        #[serde(
            rename = "regionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Peer {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Peer {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct PolicyViolationInfo {
        #[doc = "Indicates the orgpolicy violations for this resource."]
        #[serde(
            rename = "orgPolicyViolationInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub org_policy_violation_info:
            ::std::option::Option<crate::schemas::OrgPolicyViolationInfo>,
    }
    impl ::google_field_selector::FieldSelector for PolicyViolationInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PolicyViolationInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ReportRequest {
        #[doc = "Describes the list of operations to be reported. Each operation is represented as an AttributeContext, and contains all attributes around an API access."]
        #[serde(
            rename = "operations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operations: ::std::option::Option<Vec<crate::schemas::AttributeContext>>,
        #[doc = "Specifies the version of the service configuration that should be used to process the request. Must not be empty. Set this field to ‘latest’ to specify using the latest configuration."]
        #[serde(
            rename = "serviceConfigId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_config_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ReportRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReportResponse {}
    impl ::google_field_selector::FieldSelector for ReportResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Request {
        #[doc = "The request authentication. May be absent for unauthenticated requests. Derived from the HTTP request `Authorization` header or equivalent."]
        #[serde(
            rename = "auth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auth: ::std::option::Option<crate::schemas::Auth>,
        #[doc = "The HTTP request headers. If multiple headers share the same key, they must be merged according to the HTTP spec. All header keys must be lowercased, because HTTP header keys are case-insensitive."]
        #[serde(
            rename = "headers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub headers: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The HTTP request `Host` header value."]
        #[serde(
            rename = "host",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub host: ::std::option::Option<String>,
        #[doc = "The unique ID for a request, which can be propagated to downstream systems. The ID should have low probability of collision within a single day for a specific service."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The HTTP request method, such as `GET`, `POST`."]
        #[serde(
            rename = "method",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub method: ::std::option::Option<String>,
        #[doc = "The HTTP URL path, excluding the query parameters."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
        #[doc = "The network protocol used with the request, such as “http/1.1”, “spdy/3”, “h2”, “h2c”, “webrtc”, “tcp”, “udp”, “quic”. See https://www.iana.org/assignments/tls-extensiontype-values/tls-extensiontype-values.xhtml#alpn-protocol-ids for details."]
        #[serde(
            rename = "protocol",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protocol: ::std::option::Option<String>,
        #[doc = "The HTTP URL query in the format of `name1=value1&name2=value2`, as it appears in the first line of the HTTP request. No decoding is performed."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<String>,
        #[doc = "A special parameter for request reason. It is used by security systems to associate auditing information with a request."]
        #[serde(
            rename = "reason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reason: ::std::option::Option<String>,
        #[doc = "The HTTP URL scheme, such as `http` and `https`."]
        #[serde(
            rename = "scheme",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scheme: ::std::option::Option<String>,
        #[doc = "The HTTP request size in bytes. If unknown, it must be -1."]
        #[serde(
            rename = "size",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub size: ::std::option::Option<i64>,
        #[doc = "The timestamp when the `destination` service receives the last byte of the request."]
        #[serde(
            rename = "time",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Request {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Request {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct RequestMetadata {
        #[doc = "The IP address of the caller. For a caller from the internet, this will be the public IPv4 or IPv6 address. For calls made from inside Google’s internal production network from one GCP service to another, `caller_ip` will be redacted to “private”. For a caller from a Compute Engine VM with a external IP address, `caller_ip` will be the VM’s external IP address. For a caller from a Compute Engine VM without a external IP address, if the VM is in the same organization (or project) as the accessed resource, `caller_ip` will be the VM’s internal IPv4 address, otherwise `caller_ip` will be redacted to “gce-internal-ip”. See https://cloud.google.com/compute/docs/vpc/ for more information."]
        #[serde(
            rename = "callerIp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub caller_ip: ::std::option::Option<String>,
        #[doc = "The network of the caller. Set only if the network host project is part of the same GCP organization (or project) as the accessed resource. See https://cloud.google.com/compute/docs/vpc/ for more information. This is a scheme-less URI full resource name. For example: “//compute.googleapis.com/projects/PROJECT_ID/global/networks/NETWORK_ID”"]
        #[serde(
            rename = "callerNetwork",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub caller_network: ::std::option::Option<String>,
        #[doc = "The user agent of the caller. This information is not authenticated and should be treated accordingly. For example: + `google-api-python-client/1.4.0`: The request was made by the Google API client for Python. + `Cloud SDK Command Line Tool apitools-client/1.0 gcloud/0.9.62`: The request was made by the Google Cloud SDK CLI (gcloud). + `AppEngine-Google; (+http://code.google.com/appengine; appid: s~my-project`: The request was made from the `my-project` App Engine app."]
        #[serde(
            rename = "callerSuppliedUserAgent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub caller_supplied_user_agent: ::std::option::Option<String>,
        #[doc = "The destination of a network activity, such as accepting a TCP connection. In a multi hop network activity, the destination represents the receiver of the last hop. Only two fields are used in this message, Peer.port and Peer.ip. These fields are optionally populated by those services utilizing the IAM condition feature."]
        #[serde(
            rename = "destinationAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_attributes: ::std::option::Option<crate::schemas::Peer>,
        #[doc = "Request attributes used in IAM condition evaluation. This field contains request attributes like request time and access levels associated with the request. To get the whole view of the attributes used in IAM condition evaluation, the user must also look into `AuditLog.authentication_info.resource_attributes`."]
        #[serde(
            rename = "requestAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_attributes: ::std::option::Option<crate::schemas::Request>,
    }
    impl ::google_field_selector::FieldSelector for RequestMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RequestMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Resource {
        #[doc = "Annotations is an unstructured key-value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: https://kubernetes.io/docs/user-guide/annotations"]
        #[serde(
            rename = "annotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotations: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Output only. The timestamp when the resource was created. This may be either the time creation was initiated or when it was completed."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. The timestamp when the resource was deleted. If the resource is not deleted, this must be empty."]
        #[serde(
            rename = "deleteTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delete_time: ::std::option::Option<String>,
        #[doc = "Mutable. The display name set by clients. Must be \\<= 63 characters."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Output only. An opaque value that uniquely identifies a version or generation of a resource. It can be used to confirm that the client and server agree on the ordering of a resource being written."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "The labels or tags on the resource, such as AWS resource tags and Kubernetes resource labels."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Immutable. The location of the resource. The location encoding is specific to the service provider, and new encoding may be introduced as the service evolves. For Google Cloud products, the encoding is what is used by Google Cloud APIs, such as `us-east1`, `aws-us-east-1`, and `azure-eastus2`. The semantics of `location` is identical to the `cloud.googleapis.com/location` label used by some Google Cloud APIs."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "The stable identifier (name) of a resource on the `service`. A resource can be logically identified as “//{resource.service}/{resource.name}”. The differences between a resource name and a URI are: * Resource name is a logical identifier, independent of network protocol and API version. For example, `//pubsub.googleapis.com/projects/123/topics/news-feed`. * URI often includes protocol and version information, so it can be used directly by applications. For example, `https://pubsub.googleapis.com/v1/projects/123/topics/news-feed`. See https://cloud.google.com/apis/design/resource_names for details."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The type of the resource. The syntax is platform-specific because different platforms define their resources differently. For Google APIs, the type format must be “{service}/{kind}”, such as “pubsub.googleapis.com/Topic”."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The name of the service that this resource belongs to, such as `pubsub.googleapis.com`. The service may be different from the DNS hostname that actually serves the request."]
        #[serde(
            rename = "service",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service: ::std::option::Option<String>,
        #[doc = "The unique identifier of the resource. UID is unique in the time and space for this resource within the scope of the service. It is typically generated by the server on successful creation of a resource and must not be changed. UID is used to uniquely identify resources with resource name reuses. This should be a UUID4."]
        #[serde(
            rename = "uid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uid: ::std::option::Option<String>,
        #[doc = "Output only. The timestamp when the resource was last updated. Any change to the resource made by users must refresh this value. Changes to a resource made by the service should refresh this value."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Resource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Resource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResourceInfo {
        #[doc = "Optional. The identifier of the container of this resource. For Google Cloud APIs, the resource container must be one of the following formats: - `projects/` - `folders/` - `organizations/` For the policy enforcement on the container level (VPCSC and Location Policy check), this field takes precedence on the container extracted from name when presents."]
        #[serde(
            rename = "container",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub container: ::std::option::Option<String>,
        #[doc = "Optional. The location of the resource. The value must be a valid zone, region or multiregion. For example: “europe-west4” or “northamerica-northeast1-a”"]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "The name of the resource referenced in the request."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The resource permission needed for this request. The format must be “{service}/{plural}.{verb}”."]
        #[serde(
            rename = "permission",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permission: ::std::option::Option<String>,
        #[doc = "The resource type in the format of “{service}/{kind}”."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResourceInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResourceInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResourceLocation {
        #[doc = "The locations of a resource after the execution of the operation. Requests to create or delete a location based resource must populate the ‘current_locations’ field and not the ‘original_locations’ field. For example: “europe-west1-a” “us-east1” “nam3”"]
        #[serde(
            rename = "currentLocations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub current_locations: ::std::option::Option<Vec<String>>,
        #[doc = "The locations of a resource prior to the execution of the operation. Requests that mutate the resource’s location must populate both the ‘original_locations’ as well as the ‘current_locations’ fields. For example: “europe-west1-a” “us-east1” “nam3”"]
        #[serde(
            rename = "originalLocations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub original_locations: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ResourceLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResourceLocation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Response {
        #[doc = "The amount of time it takes the backend service to fully respond to a request. Measured from when the destination service starts to send the request to the backend until when the destination service receives the complete response from the backend."]
        #[serde(
            rename = "backendLatency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub backend_latency: ::std::option::Option<String>,
        #[doc = "The HTTP response status code, such as `200` and `404`."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub code: ::std::option::Option<i64>,
        #[doc = "The HTTP response headers. If multiple headers share the same key, they must be merged according to HTTP spec. All header keys must be lowercased, because HTTP header keys are case-insensitive."]
        #[serde(
            rename = "headers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub headers: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The HTTP response size in bytes. If unknown, it must be -1."]
        #[serde(
            rename = "size",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub size: ::std::option::Option<i64>,
        #[doc = "The timestamp when the `destination` service sends the last byte of the response."]
        #[serde(
            rename = "time",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Response {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Response {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ServiceAccountDelegationInfo {
        #[doc = "First party (Google) identity as the real authority."]
        #[serde(
            rename = "firstPartyPrincipal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_party_principal: ::std::option::Option<crate::schemas::FirstPartyPrincipal>,
        #[doc = "A string representing the principal_subject associated with the identity. For most identities, the format will be `principal://iam.googleapis.com/{identity pool name}/subject/{subject)` except for some GKE identities (GKE_WORKLOAD, FREEFORM, GKE_HUB_WORKLOAD) that are still in the legacy format `serviceAccount:{identity pool name}[{subject}]`"]
        #[serde(
            rename = "principalSubject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub principal_subject: ::std::option::Option<String>,
        #[doc = "Third party identity as the real authority."]
        #[serde(
            rename = "thirdPartyPrincipal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub third_party_principal: ::std::option::Option<crate::schemas::ThirdPartyPrincipal>,
    }
    impl ::google_field_selector::FieldSelector for ServiceAccountDelegationInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ServiceAccountDelegationInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SpanContext {
        #[doc = "The resource name of the span. The format is: projects/\\[PROJECT_ID_OR_NUMBER\\]/traces/\\[TRACE_ID\\]/spans/\\[SPAN_ID\\] `[TRACE_ID]` is a unique identifier for a trace within a project; it is a 32-character hexadecimal encoding of a 16-byte array. `[SPAN_ID]` is a unique identifier for a span within a trace; it is a 16-character hexadecimal encoding of an 8-byte array."]
        #[serde(
            rename = "spanName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub span_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SpanContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpanContext {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Status {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Status {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ThirdPartyPrincipal {
        #[doc = "Metadata about third party identity."]
        #[serde(
            rename = "thirdPartyClaims",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub third_party_claims:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for ThirdPartyPrincipal {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ThirdPartyPrincipal {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct V2HttpRequest {
        #[doc = "The number of HTTP response bytes inserted into cache. Set only when a cache fill was attempted."]
        #[serde(
            rename = "cacheFillBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cache_fill_bytes: ::std::option::Option<i64>,
        #[doc = "Whether or not an entity was served from cache (with or without validation)."]
        #[serde(
            rename = "cacheHit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_hit: ::std::option::Option<bool>,
        #[doc = "Whether or not a cache lookup was attempted."]
        #[serde(
            rename = "cacheLookup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_lookup: ::std::option::Option<bool>,
        #[doc = "Whether or not the response was validated with the origin server before being served from cache. This field is only meaningful if `cache_hit` is True."]
        #[serde(
            rename = "cacheValidatedWithOriginServer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_validated_with_origin_server: ::std::option::Option<bool>,
        #[doc = "The request processing latency on the server, from the time the request was received until the response was sent."]
        #[serde(
            rename = "latency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub latency: ::std::option::Option<String>,
        #[doc = "Protocol used for the request. Examples: “HTTP/1.1”, “HTTP/2”, “websocket”"]
        #[serde(
            rename = "protocol",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protocol: ::std::option::Option<String>,
        #[doc = "The referer URL of the request, as defined in [HTTP/1.1 Header Field Definitions](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html)."]
        #[serde(
            rename = "referer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referer: ::std::option::Option<String>,
        #[doc = "The IP address (IPv4 or IPv6) of the client that issued the HTTP request. Examples: `\"192.168.1.1\"`, `\"FE80::0202:B3FF:FE1E:8329\"`."]
        #[serde(
            rename = "remoteIp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub remote_ip: ::std::option::Option<String>,
        #[doc = "The request method. Examples: `\"GET\"`, `\"HEAD\"`, `\"PUT\"`, `\"POST\"`."]
        #[serde(
            rename = "requestMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_method: ::std::option::Option<String>,
        #[doc = "The size of the HTTP request message in bytes, including the request headers and the request body."]
        #[serde(
            rename = "requestSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub request_size: ::std::option::Option<i64>,
        #[doc = "The scheme (http, https), the host name, the path, and the query portion of the URL that was requested. Example: `\"http://example.com/some/info?color=red\"`."]
        #[serde(
            rename = "requestUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_url: ::std::option::Option<String>,
        #[doc = "The size of the HTTP response message sent back to the client, in bytes, including the response headers and the response body."]
        #[serde(
            rename = "responseSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub response_size: ::std::option::Option<i64>,
        #[doc = "The IP address (IPv4 or IPv6) of the origin server that the request was sent to."]
        #[serde(
            rename = "serverIp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub server_ip: ::std::option::Option<String>,
        #[doc = "The response code indicating the status of the response. Examples: 200, 404."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<i32>,
        #[doc = "The user agent sent by the client. Example: `\"Mozilla/4.0 (compatible; MSIE 6.0; Windows 98; Q312461; .NET CLR 1.0.3705)\"`."]
        #[serde(
            rename = "userAgent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_agent: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for V2HttpRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for V2HttpRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct V2LogEntry {
        #[doc = "Optional. Information about the HTTP request associated with this log entry, if applicable."]
        #[serde(
            rename = "httpRequest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http_request: ::std::option::Option<crate::schemas::V2HttpRequest>,
        #[doc = "A unique ID for the log entry used for deduplication. If omitted, the implementation will generate one based on operation_id."]
        #[serde(
            rename = "insertId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_id: ::std::option::Option<String>,
        #[doc = "A set of user-defined (key, value) data that provides additional information about the log entry."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "A set of user-defined (key, value) data that provides additional information about the moniotored resource that the log entry belongs to."]
        #[serde(
            rename = "monitoredResourceLabels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monitored_resource_labels:
            ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Required. The log to which this log entry belongs. Examples: `\"syslog\"`, `\"book_log\"`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. Information about an operation associated with the log entry, if applicable."]
        #[serde(
            rename = "operation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation: ::std::option::Option<crate::schemas::V2LogEntryOperation>,
        #[doc = "The log entry payload, represented as a protocol buffer that is expressed as a JSON object. The only accepted type currently is AuditLog."]
        #[serde(
            rename = "protoPayload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub proto_payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The severity of the log entry. The default value is `LogSeverity.DEFAULT`."]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<crate::schemas::V2LogEntrySeverity>,
        #[doc = "Optional. Source code location information associated with the log entry, if any."]
        #[serde(
            rename = "sourceLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_location: ::std::option::Option<crate::schemas::V2LogEntrySourceLocation>,
        #[doc = "The log entry payload, represented as a structure that is expressed as a JSON object."]
        #[serde(
            rename = "structPayload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub struct_payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The log entry payload, represented as a Unicode string (UTF-8)."]
        #[serde(
            rename = "textPayload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_payload: ::std::option::Option<String>,
        #[doc = "The time the event described by the log entry occurred. If omitted, defaults to operation start time."]
        #[serde(
            rename = "timestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamp: ::std::option::Option<String>,
        #[doc = "Optional. Resource name of the trace associated with the log entry, if any. If this field contains a relative resource name, you can assume the name is relative to `//tracing.googleapis.com`. Example: `projects/my-projectid/traces/06796866738c859f2f19b7cfb3214824`"]
        #[serde(
            rename = "trace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trace: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for V2LogEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for V2LogEntry {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum V2LogEntrySeverity {
        #[doc = "(700) A person must take an action immediately."]
        Alert,
        #[doc = "(600) Critical events cause more severe problems or outages."]
        Critical,
        #[doc = "(100) Debug or trace information."]
        Debug,
        #[doc = "(0) The log entry has no assigned severity level."]
        Default,
        #[doc = "(800) One or more systems are unusable."]
        Emergency,
        #[doc = "(500) Error events are likely to cause problems."]
        Error,
        #[doc = "(200) Routine information, such as ongoing status or performance."]
        Info,
        #[doc = "(300) Normal but significant events, such as start up, shut down, or a configuration change."]
        Notice,
        #[doc = "(400) Warning events might cause problems."]
        Warning,
    }
    impl V2LogEntrySeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                V2LogEntrySeverity::Alert => "ALERT",
                V2LogEntrySeverity::Critical => "CRITICAL",
                V2LogEntrySeverity::Debug => "DEBUG",
                V2LogEntrySeverity::Default => "DEFAULT",
                V2LogEntrySeverity::Emergency => "EMERGENCY",
                V2LogEntrySeverity::Error => "ERROR",
                V2LogEntrySeverity::Info => "INFO",
                V2LogEntrySeverity::Notice => "NOTICE",
                V2LogEntrySeverity::Warning => "WARNING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for V2LogEntrySeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for V2LogEntrySeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<V2LogEntrySeverity, ()> {
            Ok(match s {
                "ALERT" => V2LogEntrySeverity::Alert,
                "CRITICAL" => V2LogEntrySeverity::Critical,
                "DEBUG" => V2LogEntrySeverity::Debug,
                "DEFAULT" => V2LogEntrySeverity::Default,
                "EMERGENCY" => V2LogEntrySeverity::Emergency,
                "ERROR" => V2LogEntrySeverity::Error,
                "INFO" => V2LogEntrySeverity::Info,
                "NOTICE" => V2LogEntrySeverity::Notice,
                "WARNING" => V2LogEntrySeverity::Warning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for V2LogEntrySeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for V2LogEntrySeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for V2LogEntrySeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALERT" => V2LogEntrySeverity::Alert,
                "CRITICAL" => V2LogEntrySeverity::Critical,
                "DEBUG" => V2LogEntrySeverity::Debug,
                "DEFAULT" => V2LogEntrySeverity::Default,
                "EMERGENCY" => V2LogEntrySeverity::Emergency,
                "ERROR" => V2LogEntrySeverity::Error,
                "INFO" => V2LogEntrySeverity::Info,
                "NOTICE" => V2LogEntrySeverity::Notice,
                "WARNING" => V2LogEntrySeverity::Warning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for V2LogEntrySeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for V2LogEntrySeverity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct V2LogEntryOperation {
        #[doc = "Optional. Set this to True if this is the first log entry in the operation."]
        #[serde(
            rename = "first",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first: ::std::option::Option<bool>,
        #[doc = "Optional. An arbitrary operation identifier. Log entries with the same identifier are assumed to be part of the same operation."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Optional. Set this to True if this is the last log entry in the operation."]
        #[serde(
            rename = "last",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last: ::std::option::Option<bool>,
        #[doc = "Optional. An arbitrary producer identifier. The combination of `id` and `producer` must be globally unique. Examples for `producer`: `\"MyDivision.MyBigCompany.com\"`, `\"github.com/MyProject/MyApplication\"`."]
        #[serde(
            rename = "producer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub producer: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for V2LogEntryOperation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for V2LogEntryOperation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct V2LogEntrySourceLocation {
        #[doc = "Optional. Source file name. Depending on the runtime environment, this might be a simple name or a fully-qualified name."]
        #[serde(
            rename = "file",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file: ::std::option::Option<String>,
        #[doc = "Optional. Human-readable name of the function or method being invoked, with optional context such as the class or package name. This information may be used in contexts such as the logs viewer, where a file and line number are less meaningful. The format can vary by language. For example: `qual.if.ied.Class.method` (Java), `dir/package.func` (Go), `function` (Python)."]
        #[serde(
            rename = "function",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub function: ::std::option::Option<String>,
        #[doc = "Optional. Line within the source file. 1-based; 0 indicates no line number available."]
        #[serde(
            rename = "line",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub line: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for V2LogEntrySourceLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for V2LogEntrySourceLocation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ViolationInfo {
        #[doc = "Optional. Value that is being checked for the policy. This could be in encrypted form (if pii sensitive). This field will only be emitted in LIST_POLICY types"]
        #[serde(
            rename = "checkedValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub checked_value: ::std::option::Option<String>,
        #[doc = "Optional. Constraint name"]
        #[serde(
            rename = "constraint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub constraint: ::std::option::Option<String>,
        #[doc = "Optional. Error message that policy is indicating."]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[doc = "Optional. Indicates the type of the policy."]
        #[serde(
            rename = "policyType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy_type: ::std::option::Option<crate::schemas::ViolationInfoPolicyType>,
    }
    impl ::google_field_selector::FieldSelector for ViolationInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ViolationInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ViolationInfoPolicyType {
        #[doc = "Indicates boolean policy constraint"]
        BooleanConstraint,
        #[doc = "Indicates custom policy constraint"]
        CustomConstraint,
        #[doc = "Indicates list policy constraint"]
        ListConstraint,
        #[doc = "Default value. This value should not be used."]
        PolicyTypeUnspecified,
    }
    impl ViolationInfoPolicyType {
        pub fn as_str(self) -> &'static str {
            match self {
                ViolationInfoPolicyType::BooleanConstraint => "BOOLEAN_CONSTRAINT",
                ViolationInfoPolicyType::CustomConstraint => "CUSTOM_CONSTRAINT",
                ViolationInfoPolicyType::ListConstraint => "LIST_CONSTRAINT",
                ViolationInfoPolicyType::PolicyTypeUnspecified => "POLICY_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ViolationInfoPolicyType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ViolationInfoPolicyType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ViolationInfoPolicyType, ()> {
            Ok(match s {
                "BOOLEAN_CONSTRAINT" => ViolationInfoPolicyType::BooleanConstraint,
                "CUSTOM_CONSTRAINT" => ViolationInfoPolicyType::CustomConstraint,
                "LIST_CONSTRAINT" => ViolationInfoPolicyType::ListConstraint,
                "POLICY_TYPE_UNSPECIFIED" => ViolationInfoPolicyType::PolicyTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ViolationInfoPolicyType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ViolationInfoPolicyType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ViolationInfoPolicyType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOOLEAN_CONSTRAINT" => ViolationInfoPolicyType::BooleanConstraint,
                "CUSTOM_CONSTRAINT" => ViolationInfoPolicyType::CustomConstraint,
                "LIST_CONSTRAINT" => ViolationInfoPolicyType::ListConstraint,
                "POLICY_TYPE_UNSPECIFIED" => ViolationInfoPolicyType::PolicyTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ViolationInfoPolicyType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ViolationInfoPolicyType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
        #[doc = "Media download with context-dependent Content-Type"]
        Media,
        #[doc = "Responses with Content-Type of application/x-protobuf"]
        Proto,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
                Alt::Proto => "proto",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Alt {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Alt {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Alt, ()> {
            Ok(match s {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Alt {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Alt {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Xgafv {
        #[doc = "v1 error format"]
        _1,
        #[doc = "v2 error format"]
        _2,
    }
    impl Xgafv {
        pub fn as_str(self) -> &'static str {
            match self {
                Xgafv::_1 => "1",
                Xgafv::_2 => "2",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Xgafv {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Xgafv {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Xgafv, ()> {
            Ok(match s {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Xgafv {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Xgafv {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub struct Client {
    reqwest: ::reqwest::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client::with_reqwest_client(auth, ::reqwest::Client::builder().build().unwrap())
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::Client) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client {
            reqwest,
            auth: Box::new(auth),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the services resource"]
    pub fn services(&self) -> crate::resources::services::ServicesActions {
        crate::resources::services::ServicesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod services {
        pub mod params {}
        pub struct ServicesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ServicesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Private Preview. This feature is only available for approved services. This method provides admission control for services that are integrated with [Service Infrastructure](https://cloud.google.com/service-infrastructure). It checks whether an operation should be allowed based on the service configuration and relevant policies. It must be called before the operation is executed. For more information, see [Admission Control](https://cloud.google.com/service-infrastructure/docs/admission-control). NOTE: The admission control has an expected policy propagation delay of 60s. The caller **must** not depend on the most recent policy changes. NOTE: The admission control has a hard limit of 1 referenced resources per call. If an operation refers to more than 1 resources, the caller must call the Check method multiple times. This method requires the `servicemanagement.services.check` permission on the specified service. For more information, see [Service Control API Access Control](https://cloud.google.com/service-infrastructure/docs/service-control/access-control)."]
            pub fn check(
                &self,
                request: crate::schemas::CheckRequest,
                service_name: impl Into<String>,
            ) -> CheckRequestBuilder {
                CheckRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    service_name: service_name.into(),
                }
            }
            #[doc = "Private Preview. This feature is only available for approved services. This method provides telemetry reporting for services that are integrated with [Service Infrastructure](https://cloud.google.com/service-infrastructure). It reports a list of operations that have occurred on a service. It must be called after the operations have been executed. For more information, see [Telemetry Reporting](https://cloud.google.com/service-infrastructure/docs/telemetry-reporting). NOTE: The telemetry reporting has a hard limit of 1000 operations and 1MB per Report call. It is recommended to have no more than 100 operations per call. This method requires the `servicemanagement.services.report` permission on the specified service. For more information, see [Service Control API Access Control](https://cloud.google.com/service-infrastructure/docs/service-control/access-control)."]
            pub fn report(
                &self,
                request: crate::schemas::ReportRequest,
                service_name: impl Into<String>,
            ) -> ReportRequestBuilder {
                ReportRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    service_name: service_name.into(),
                }
            }
        }
        #[doc = "Created via [ServicesActions::check()](struct.ServicesActions.html#method.check)"]
        #[derive(Debug, Clone)]
        pub struct CheckRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::CheckRequest,
            service_name: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> CheckRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. “raw”, “multipart”)."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. “media”, “multipart”)."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::CheckResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::CheckResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://servicecontrol.googleapis.com/".to_owned();
                output.push_str("v2/services/");
                {
                    let var_as_str = &self.service_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":check");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
        #[doc = "Created via [ServicesActions::report()](struct.ServicesActions.html#method.report)"]
        #[derive(Debug, Clone)]
        pub struct ReportRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::ReportRequest,
            service_name: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> ReportRequestBuilder<'a> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. “raw”, “multipart”)."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. “media”, “multipart”)."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub async fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: ::std::option::Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields).await
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::ReportResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ReportResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute().await
            }
            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://servicecontrol.googleapis.com/".to_owned();
                output.push_str("v2/services/");
                {
                    let var_as_str = &self.service_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":report");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("access_token", &self.access_token)]);
                req = req.query(&[("alt", &self.alt)]);
                req = req.query(&[("callback", &self.callback)]);
                req = req.query(&[("fields", &self.fields)]);
                req = req.query(&[("key", &self.key)]);
                req = req.query(&[("oauth_token", &self.oauth_token)]);
                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                req = req.query(&[("quotaUser", &self.quota_user)]);
                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                req = req.query(&[("uploadType", &self.upload_type)]);
                req = req.query(&[("$.xgafv", &self.xgafv)]);
                let access_token = self
                    .auth
                    .access_token()
                    .await
                    .map_err(|err| crate::Error::OAuth2(err))?;
                req = req.bearer_auth(access_token);
                Ok(req)
            }
        }
    }
}
#[derive(Debug)]
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error + Send + Sync>),
    JSON(::serde_json::Error),
    Reqwest {
        reqwest_err: ::reqwest::Error,
        body: Option<String>,
    },
    IO(std::io::Error),
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
            Error::IO(_) => None,
            Error::Other(_) => None,
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::OAuth2(err) => write!(f, "OAuth2 Error: {}", err),
            Error::JSON(err) => write!(f, "JSON Error: {}", err),
            Error::Reqwest { reqwest_err, body } => {
                write!(f, "Reqwest Error: {}", reqwest_err)?;
                if let Some(body) = body {
                    write!(f, ": {}", body)?;
                }
                Ok(())
            }
            Error::IO(err) => write!(f, "IO Error: {}", err),
            Error::Other(err) => write!(f, "Uknown Error: {}", err),
        }
    }
}

impl ::std::error::Error for Error {}

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::JSON(err)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(reqwest_err: ::reqwest::Error) -> Error {
        Error::Reqwest {
            reqwest_err,
            body: None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}
#[allow(dead_code)]
const SIMPLE: &::percent_encoding::AsciiSet = &::percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

#[allow(dead_code)]
const RESERVED: &::percent_encoding::AsciiSet = &SIMPLE
    .remove(b'%')
    .remove(b':')
    .remove(b'/')
    .remove(b'?')
    .remove(b'#')
    .remove(b'[')
    .remove(b']')
    .remove(b'@')
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b';')
    .remove(b'=');
#[allow(dead_code)]
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
        },
    }

    impl futures::io::AsyncRead for RelatedMultiPartReader {
        fn poll_read(
            mut self: std::pin::Pin<&mut Self>,
            ctx: &mut futures::task::Context,
            buf: &mut [u8],
        ) -> futures::task::Poll<Result<usize, futures::io::Error>> {
            use RelatedMultiPartReaderState::*;

            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let body = std::pin::Pin::new(body);
                        let written = match futures::io::AsyncRead::poll_read(body, ctx, rem_buf) {
                            futures::task::Poll::Ready(Ok(n)) => n,
                            futures::task::Poll::Ready(Err(err)) => {
                                return futures::task::Poll::Ready(Err(err));
                            }
                            futures::task::Poll::Pending => return futures::task::Poll::Pending,
                        };
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }

            futures::task::Poll::Ready(Ok(bytes_written))
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
#[allow(dead_code)]
mod parsed_string {
    pub fn serialize<T, S>(
        value: &Option<T>,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> ::std::result::Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}
/// Represent the ability to extract the `nextPageToken` from a response.
pub trait GetNextPageToken<T> {
    /// Get the `nextPageToken` from a response if present.
    fn next_page_token(&self) -> ::std::option::Option<T>;
}

impl<T: ::std::convert::From<::std::string::String>> GetNextPageToken<T>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn next_page_token(&self) -> ::std::option::Option<T> {
        self.get("nextPageToken")
            .and_then(|t| t.as_str())
            .map(|s| s.to_owned().into())
    }
}