#![doc = "# Resources and Methods\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [notes](resources/projects/notes/struct.NotesActions.html)\n        * [*batchCreate*](resources/projects/notes/struct.BatchCreateRequestBuilder.html), [*create*](resources/projects/notes/struct.CreateRequestBuilder.html), [*delete*](resources/projects/notes/struct.DeleteRequestBuilder.html), [*get*](resources/projects/notes/struct.GetRequestBuilder.html), [*getIamPolicy*](resources/projects/notes/struct.GetIamPolicyRequestBuilder.html), [*list*](resources/projects/notes/struct.ListRequestBuilder.html), [*patch*](resources/projects/notes/struct.PatchRequestBuilder.html), [*setIamPolicy*](resources/projects/notes/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/projects/notes/struct.TestIamPermissionsRequestBuilder.html)\n        * [occurrences](resources/projects/notes/occurrences/struct.OccurrencesActions.html)\n          * [*list*](resources/projects/notes/occurrences/struct.ListRequestBuilder.html)\n      * [occurrences](resources/projects/occurrences/struct.OccurrencesActions.html)\n        * [*batchCreate*](resources/projects/occurrences/struct.BatchCreateRequestBuilder.html), [*create*](resources/projects/occurrences/struct.CreateRequestBuilder.html), [*delete*](resources/projects/occurrences/struct.DeleteRequestBuilder.html), [*get*](resources/projects/occurrences/struct.GetRequestBuilder.html), [*getIamPolicy*](resources/projects/occurrences/struct.GetIamPolicyRequestBuilder.html), [*getNotes*](resources/projects/occurrences/struct.GetNotesRequestBuilder.html), [*getVulnerabilitySummary*](resources/projects/occurrences/struct.GetVulnerabilitySummaryRequestBuilder.html), [*list*](resources/projects/occurrences/struct.ListRequestBuilder.html), [*patch*](resources/projects/occurrences/struct.PatchRequestBuilder.html), [*setIamPolicy*](resources/projects/occurrences/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/projects/occurrences/struct.TestIamPermissionsRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
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
    pub struct AliasContext {
        #[doc = "The alias kind."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::AliasContextKind>,
        #[doc = "The alias name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AliasContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AliasContext {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AliasContextKind {
        #[doc = "Git tag."]
        Fixed,
        #[doc = "Unknown."]
        KindUnspecified,
        #[doc = "Git branch."]
        Movable,
        #[doc = "Used to specify non-standard aliases. For example, if a Git repo has a ref named \"refs/foo/bar\"."]
        Other,
    }
    impl AliasContextKind {
        pub fn as_str(self) -> &'static str {
            match self {
                AliasContextKind::Fixed => "FIXED",
                AliasContextKind::KindUnspecified => "KIND_UNSPECIFIED",
                AliasContextKind::Movable => "MOVABLE",
                AliasContextKind::Other => "OTHER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AliasContextKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AliasContextKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AliasContextKind, ()> {
            Ok(match s {
                "FIXED" => AliasContextKind::Fixed,
                "KIND_UNSPECIFIED" => AliasContextKind::KindUnspecified,
                "MOVABLE" => AliasContextKind::Movable,
                "OTHER" => AliasContextKind::Other,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AliasContextKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AliasContextKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AliasContextKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FIXED" => AliasContextKind::Fixed,
                "KIND_UNSPECIFIED" => AliasContextKind::KindUnspecified,
                "MOVABLE" => AliasContextKind::Movable,
                "OTHER" => AliasContextKind::Other,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AliasContextKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AliasContextKind {
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
    pub struct Artifact {
        #[doc = "Hash or checksum value of a binary, or Docker Registry 2.0 digest of a container."]
        #[serde(
            rename = "checksum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub checksum: ::std::option::Option<String>,
        #[doc = "Artifact ID, if any; for container images, this will be a URL by digest like `gcr.io/projectID/imagename@sha256:123456`."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Related artifact names. This may be the path to a binary or jar file, or in the case of a container build, the name used to push the container image to Google Container Registry, as presented to `docker push`. Note that a single Artifact ID can have multiple names, for example if two tags are applied to one image."]
        #[serde(
            rename = "names",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub names: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Artifact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Artifact {
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
    pub struct AttestationNote {
        #[doc = "Hint hints at the purpose of the attestation authority."]
        #[serde(
            rename = "hint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hint: ::std::option::Option<crate::schemas::Hint>,
    }
    impl ::google_field_selector::FieldSelector for AttestationNote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AttestationNote {
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
    pub struct AttestationOccurrence {
        #[doc = "One or more JWTs encoding a self-contained attestation. Each JWT encodes the payload that it verifies within the JWT itself. Verifier implementation SHOULD ignore the `serialized_payload` field when verifying these JWTs. If only JWTs are present on this AttestationOccurrence, then the `serialized_payload` SHOULD be left empty. Each JWT SHOULD encode a claim specific to the `resource_uri` of this Occurrence, but this is not validated by Grafeas metadata API implementations. The JWT itself is opaque to Grafeas."]
        #[serde(
            rename = "jwts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jwts: ::std::option::Option<Vec<crate::schemas::Jwt>>,
        #[doc = "Required. The serialized payload that is verified by one or more `signatures`."]
        #[serde(
            rename = "serializedPayload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serialized_payload: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "One or more signatures over `serialized_payload`. Verifier implementations should consider this attestation message verified if at least one `signature` verifies `serialized_payload`. See `Signature` in common.proto for more details on signature structure and verification."]
        #[serde(
            rename = "signatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signatures: ::std::option::Option<Vec<crate::schemas::Signature>>,
    }
    impl ::google_field_selector::FieldSelector for AttestationOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AttestationOccurrence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BatchCreateNotesRequest {
        #[doc = "Required. The notes to create. Max allowed length is 1000."]
        #[serde(
            rename = "notes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub notes:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::Note>>,
    }
    impl ::google_field_selector::FieldSelector for BatchCreateNotesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchCreateNotesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BatchCreateNotesResponse {
        #[doc = "The notes that were created."]
        #[serde(
            rename = "notes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub notes: ::std::option::Option<Vec<crate::schemas::Note>>,
    }
    impl ::google_field_selector::FieldSelector for BatchCreateNotesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchCreateNotesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchCreateOccurrencesRequest {
        #[doc = "Required. The occurrences to create. Max allowed length is 1000."]
        #[serde(
            rename = "occurrences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub occurrences: ::std::option::Option<Vec<crate::schemas::Occurrence>>,
    }
    impl ::google_field_selector::FieldSelector for BatchCreateOccurrencesRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchCreateOccurrencesRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchCreateOccurrencesResponse {
        #[doc = "The occurrences that were created."]
        #[serde(
            rename = "occurrences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub occurrences: ::std::option::Option<Vec<crate::schemas::Occurrence>>,
    }
    impl ::google_field_selector::FieldSelector for BatchCreateOccurrencesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchCreateOccurrencesResponse {
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
    pub struct Binding {
        #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "condition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub condition: ::std::option::Option<crate::schemas::Expr>,
        #[doc = "Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. "]
        #[serde(
            rename = "members",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub members: ::std::option::Option<Vec<String>>,
        #[doc = "Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
        #[serde(
            rename = "role",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub role: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Binding {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Binding {
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
    pub struct BuildNote {
        #[doc = "Required. Immutable. Version of the builder which produced this build."]
        #[serde(
            rename = "builderVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub builder_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BuildNote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BuildNote {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BuildOccurrence {
        #[doc = "Deprecated. See InTotoStatement for the replacement. In-toto Provenance representation as defined in spec."]
        #[serde(
            rename = "intotoProvenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub intoto_provenance: ::std::option::Option<crate::schemas::InTotoProvenance>,
        #[doc = "In-toto Statement representation as defined in spec. The intoto_statement can contain any type of provenance. The serialized payload of the statement can be stored and signed in the Occurrence's envelope."]
        #[serde(
            rename = "intotoStatement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub intoto_statement: ::std::option::Option<crate::schemas::InTotoStatement>,
        #[doc = "The actual provenance for the build."]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance: ::std::option::Option<crate::schemas::BuildProvenance>,
        #[doc = "Serialized JSON representation of the provenance, used in generating the build signature in the corresponding build note. After verifying the signature, `provenance_bytes` can be unmarshalled and compared to the provenance to confirm that it is unchanged. A base64-encoded string representation of the provenance bytes is used for the signature in order to interoperate with openssl which expects this format for signature verification. The serialized form is captured both to avoid ambiguity in how the provenance is marshalled to json as well to prevent incompatibilities with future changes."]
        #[serde(
            rename = "provenanceBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance_bytes: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BuildOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BuildOccurrence {
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
    pub struct BuildProvenance {
        #[doc = "Special options applied to this build. This is a catch-all field where build providers can enter any desired additional details."]
        #[serde(
            rename = "buildOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_options: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Version string of the builder at the time this build was executed."]
        #[serde(
            rename = "builderVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub builder_version: ::std::option::Option<String>,
        #[doc = "Output of the build."]
        #[serde(
            rename = "builtArtifacts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub built_artifacts: ::std::option::Option<Vec<crate::schemas::Artifact>>,
        #[doc = "Commands requested by the build."]
        #[serde(
            rename = "commands",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub commands: ::std::option::Option<Vec<crate::schemas::Command>>,
        #[doc = "Time at which the build was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "E-mail address of the user who initiated this build. Note that this was the user's e-mail address at the time the build was initiated; this address may not represent the same end-user for all time."]
        #[serde(
            rename = "creator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creator: ::std::option::Option<String>,
        #[doc = "Time at which execution of the build was finished."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Required. Unique identifier of the build."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "URI where any logs for this provenance were written."]
        #[serde(
            rename = "logsUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logs_uri: ::std::option::Option<String>,
        #[doc = "ID of the project."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "Details of the Source input to the build."]
        #[serde(
            rename = "sourceProvenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_provenance: ::std::option::Option<crate::schemas::Source>,
        #[doc = "Time at which execution of the build was started."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Trigger identifier if the build was triggered automatically; empty if not."]
        #[serde(
            rename = "triggerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trigger_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BuildProvenance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BuildProvenance {
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
    pub struct BuilderConfig {
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BuilderConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BuilderConfig {
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
    pub struct Category {
        #[doc = "The identifier of the category."]
        #[serde(
            rename = "categoryId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category_id: ::std::option::Option<String>,
        #[doc = "The localized name of the category."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Category {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Category {
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
    pub struct CisBenchmark {
        #[serde(
            rename = "profileLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub profile_level: ::std::option::Option<i32>,
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<crate::schemas::CisBenchmarkSeverity>,
    }
    impl ::google_field_selector::FieldSelector for CisBenchmark {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CisBenchmark {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CisBenchmarkSeverity {
        #[doc = "Critical severity."]
        Critical,
        #[doc = "High severity."]
        High,
        #[doc = "Low severity."]
        Low,
        #[doc = "Medium severity."]
        Medium,
        #[doc = "Minimal severity."]
        Minimal,
        #[doc = "Unknown."]
        SeverityUnspecified,
    }
    impl CisBenchmarkSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                CisBenchmarkSeverity::Critical => "CRITICAL",
                CisBenchmarkSeverity::High => "HIGH",
                CisBenchmarkSeverity::Low => "LOW",
                CisBenchmarkSeverity::Medium => "MEDIUM",
                CisBenchmarkSeverity::Minimal => "MINIMAL",
                CisBenchmarkSeverity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CisBenchmarkSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CisBenchmarkSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CisBenchmarkSeverity, ()> {
            Ok(match s {
                "CRITICAL" => CisBenchmarkSeverity::Critical,
                "HIGH" => CisBenchmarkSeverity::High,
                "LOW" => CisBenchmarkSeverity::Low,
                "MEDIUM" => CisBenchmarkSeverity::Medium,
                "MINIMAL" => CisBenchmarkSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => CisBenchmarkSeverity::SeverityUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CisBenchmarkSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CisBenchmarkSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CisBenchmarkSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRITICAL" => CisBenchmarkSeverity::Critical,
                "HIGH" => CisBenchmarkSeverity::High,
                "LOW" => CisBenchmarkSeverity::Low,
                "MEDIUM" => CisBenchmarkSeverity::Medium,
                "MINIMAL" => CisBenchmarkSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => CisBenchmarkSeverity::SeverityUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CisBenchmarkSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CisBenchmarkSeverity {
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
    pub struct CloudRepoSourceContext {
        #[doc = "An alias, which may be a branch or tag."]
        #[serde(
            rename = "aliasContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alias_context: ::std::option::Option<crate::schemas::AliasContext>,
        #[doc = "The ID of the repo."]
        #[serde(
            rename = "repoId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repo_id: ::std::option::Option<crate::schemas::RepoId>,
        #[doc = "A revision ID."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CloudRepoSourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudRepoSourceContext {
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
    pub struct Command {
        #[doc = "Command-line arguments used when executing this command."]
        #[serde(
            rename = "args",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub args: ::std::option::Option<Vec<String>>,
        #[doc = "Working directory (relative to project source root) used when running this command."]
        #[serde(
            rename = "dir",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dir: ::std::option::Option<String>,
        #[doc = "Environment variables set before running this command."]
        #[serde(
            rename = "env",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub env: ::std::option::Option<Vec<String>>,
        #[doc = "Optional unique identifier for this command, used in wait_for to reference this command as a dependency."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Required. Name of the command, as presented on the command line, or if the command is packaged as a Docker container, as presented to `docker pull`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The ID(s) of the command(s) that this command depends on."]
        #[serde(
            rename = "waitFor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wait_for: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Command {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Command {
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
    pub struct Completeness {
        #[doc = "If true, the builder claims that recipe.arguments is complete, meaning that all external inputs are properly captured in the recipe."]
        #[serde(
            rename = "arguments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arguments: ::std::option::Option<bool>,
        #[doc = "If true, the builder claims that recipe.environment is claimed to be complete."]
        #[serde(
            rename = "environment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment: ::std::option::Option<bool>,
        #[doc = "If true, the builder claims that materials are complete, usually through some controls to prevent network access. Sometimes called \"hermetic\"."]
        #[serde(
            rename = "materials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub materials: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for Completeness {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Completeness {
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
    pub struct ComplianceNote {
        #[serde(
            rename = "cisBenchmark",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cis_benchmark: ::std::option::Option<crate::schemas::CisBenchmark>,
        #[doc = "A description about this compliance check."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "A rationale for the existence of this compliance check."]
        #[serde(
            rename = "rationale",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rationale: ::std::option::Option<String>,
        #[doc = "A description of remediation steps if the compliance check fails."]
        #[serde(
            rename = "remediation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub remediation: ::std::option::Option<String>,
        #[doc = "Serialized scan instructions with a predefined format."]
        #[serde(
            rename = "scanInstructions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scan_instructions: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The title that identifies this compliance check."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "The OS and config versions the benchmark applies to."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<Vec<crate::schemas::ComplianceVersion>>,
    }
    impl ::google_field_selector::FieldSelector for ComplianceNote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ComplianceNote {
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
    pub struct ComplianceOccurrence {
        #[serde(
            rename = "nonComplianceReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_compliance_reason: ::std::option::Option<String>,
        #[serde(
            rename = "nonCompliantFiles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_compliant_files: ::std::option::Option<Vec<crate::schemas::NonCompliantFile>>,
    }
    impl ::google_field_selector::FieldSelector for ComplianceOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ComplianceOccurrence {
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
    pub struct ComplianceVersion {
        #[doc = "The name of the document that defines this benchmark, e.g. \"CIS Container-Optimized OS\"."]
        #[serde(
            rename = "benchmarkDocument",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub benchmark_document: ::std::option::Option<String>,
        #[doc = "The CPE URI (https://cpe.mitre.org/specification/) this benchmark is applicable to."]
        #[serde(
            rename = "cpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpe_uri: ::std::option::Option<String>,
        #[doc = "The version of the benchmark. This is set to the version of the OS-specific CIS document the benchmark is defined in."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ComplianceVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ComplianceVersion {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalConfig {
        #[doc = "Whether or not approval is needed. If this is set on a build, it will become pending when created, and will need to be explicitly approved to start."]
        #[serde(
            rename = "approvalRequired",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub approval_required: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalConfig
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResult {
        #[doc = "Output only. The time when the approval decision was made."]
        #[serde(
            rename = "approvalTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub approval_time: ::std::option::Option<String>,
        #[doc = "Output only. Email of the user that called the ApproveBuild API to approve or reject a build at the time that the API was called."]
        #[serde(
            rename = "approverAccount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub approver_account: ::std::option::Option<String>,
        #[doc = "Optional. An optional comment for this manual approval result."]
        #[serde(
            rename = "comment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub comment: ::std::option::Option<String>,
        #[doc = "Required. The decision of this manual approval."]
        #[serde(
            rename = "decision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub decision: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision,
        >,
        #[doc = "Optional. An optional URL tied to this manual approval result. This field is essentially the same as comment, except that it will be rendered by the UI differently. An example use case is a link to an external job that approved this Build."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResult
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResult
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision {
        #[doc = "Build is approved."]
        Approved,
        #[doc = "Default enum type. This should not be used."]
        DecisionUnspecified,
        #[doc = "Build is rejected."]
        Rejected,
    }
    impl ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision {
        pub fn as_str(self) -> &'static str {
            match self { ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision :: Approved => "APPROVED" , ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision :: DecisionUnspecified => "DECISION_UNSPECIFIED" , ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision :: Rejected => "REJECTED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision,
            (),
        > {
            Ok (match s { "APPROVED" => ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision :: Approved , "DECISION_UNSPECIFIED" => ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision :: DecisionUnspecified , "REJECTED" => ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision :: Rejected , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "APPROVED" => ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision :: Approved , "DECISION_UNSPECIFIED" => ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision :: DecisionUnspecified , "REJECTED" => ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision :: Rejected , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResultDecision
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1Artifacts {
        #[doc = "A list of images to be pushed upon the successful completion of all build steps. The images will be pushed using the builder service account's credentials. The digests of the pushed images will be stored in the Build resource's results field. If any of the images fail to be pushed, the build is marked FAILURE."]
        #[serde(
            rename = "images",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub images: ::std::option::Option<Vec<String>>,
        #[doc = "A list of objects to be uploaded to Cloud Storage upon successful completion of all build steps. Files in the workspace matching specified paths globs will be uploaded to the specified Cloud Storage location using the builder service account's credentials. The location and generation of the uploaded objects will be stored in the Build resource's results field. If any objects fail to be pushed, the build is marked FAILURE."]
        #[serde(
            rename = "objects",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub objects: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1ArtifactsArtifactObjects,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1Artifacts
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContaineranalysisGoogleDevtoolsCloudbuildV1Artifacts {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1ArtifactsArtifactObjects {
        #[doc = "Cloud Storage bucket and optional object path, in the form \"gs://bucket/path/to/somewhere/\". (see [Bucket Name Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)). Files in the workspace matching any path pattern will be uploaded to Cloud Storage with this location as a prefix."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Path globs used to match files in the build's workspace."]
        #[serde(
            rename = "paths",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub paths: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. Stores timing information for pushing all artifact objects."]
        #[serde(
            rename = "timing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timing: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1TimeSpan,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1ArtifactsArtifactObjects
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1ArtifactsArtifactObjects
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1Build {
        #[doc = "Output only. Describes this build's approval configuration, status, and result."]
        #[serde(
            rename = "approval",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub approval: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApproval,
        >,
        #[doc = "Artifacts produced by the build that should be uploaded upon successful completion of all build steps."]
        #[serde(
            rename = "artifacts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub artifacts: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1Artifacts,
        >,
        #[doc = "Secrets and secret environment variables."]
        #[serde(
            rename = "availableSecrets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub available_secrets: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1Secrets,
        >,
        #[doc = "Output only. The ID of the `BuildTrigger` that triggered this build, if it was triggered automatically."]
        #[serde(
            rename = "buildTriggerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_trigger_id: ::std::option::Option<String>,
        #[doc = "Output only. Time at which the request to create the build was received."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. Contains information about the build when status=FAILURE."]
        #[serde(
            rename = "failureInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failure_info: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfo,
        >,
        #[doc = "Output only. Time at which execution of the build was finished. The difference between finish_time and start_time is the duration of the build's execution."]
        #[serde(
            rename = "finishTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub finish_time: ::std::option::Option<String>,
        #[doc = "Output only. Unique identifier of the build."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "A list of images to be pushed upon the successful completion of all build steps. The images are pushed using the builder service account's credentials. The digests of the pushed images will be stored in the `Build` resource's results field. If any of the images fail to be pushed, the build status is marked `FAILURE`."]
        #[serde(
            rename = "images",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub images: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. URL to logs for this build in Google Cloud Console."]
        #[serde(
            rename = "logUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_url: ::std::option::Option<String>,
        #[doc = "Google Cloud Storage bucket where logs should be written (see [Bucket Name Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)). Logs file names will be of the format `${logs_bucket}/log-${build_id}.txt`."]
        #[serde(
            rename = "logsBucket",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logs_bucket: ::std::option::Option<String>,
        #[doc = "Output only. The 'Build' name with format: `projects/{project}/locations/{location}/builds/{build}`, where {build} is a unique identifier generated by the service."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Special options for this build."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptions,
        >,
        #[doc = "Output only. ID of the project."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "TTL in queue for this build. If provided and the build is enqueued longer than this value, the build will expire and the build status will be `EXPIRED`. The TTL starts ticking from create_time."]
        #[serde(
            rename = "queueTtl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub queue_ttl: ::std::option::Option<String>,
        #[doc = "Output only. Results of the build."]
        #[serde(
            rename = "results",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub results: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1Results,
        >,
        #[doc = "Secrets to decrypt using Cloud Key Management Service. Note: Secret Manager is the recommended technique for managing sensitive data with Cloud Build. Use `available_secrets` to configure builds to access secrets from Secret Manager. For instructions, see: https://cloud.google.com/cloud-build/docs/securing-builds/use-secrets"]
        #[serde(
            rename = "secrets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub secrets: ::std::option::Option<
            Vec<crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1Secret>,
        >,
        #[doc = "IAM service account whose credentials will be used at build runtime. Must be of the format `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`. ACCOUNT can be email address or uniqueId of the service account. "]
        #[serde(
            rename = "serviceAccount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_account: ::std::option::Option<String>,
        #[doc = "The location of the source files to build."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1Source,
        >,
        #[doc = "Output only. A permanent fixed identifier for source."]
        #[serde(
            rename = "sourceProvenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_provenance: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1SourceProvenance,
        >,
        #[doc = "Output only. Time at which execution of the build was started."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Output only. Status of the build."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus,
        >,
        #[doc = "Output only. Customer-readable message about the current status."]
        #[serde(
            rename = "statusDetail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status_detail: ::std::option::Option<String>,
        #[doc = "Required. The operations to be performed on the workspace."]
        #[serde(
            rename = "steps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub steps: ::std::option::Option<
            Vec<crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStep>,
        >,
        #[doc = "Substitutions data for `Build` resource."]
        #[serde(
            rename = "substitutions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub substitutions: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Tags for annotation of a `Build`. These are not docker tags."]
        #[serde(
            rename = "tags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tags: ::std::option::Option<Vec<String>>,
        #[doc = "Amount of time that this build should be allowed to run, to second granularity. If this amount of time elapses, work on the build will cease and the build status will be `TIMEOUT`. `timeout` starts ticking from `startTime`. Default time is ten minutes."]
        #[serde(
            rename = "timeout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timeout: ::std::option::Option<String>,
        #[doc = "Output only. Stores timing information for phases of the build. Valid keys are: * BUILD: time to execute all build steps. * PUSH: time to push all specified images. * FETCHSOURCE: time to fetch source. * SETUPBUILD: time to set up build. If the build does not specify source or images, these keys will not be included."]
        #[serde(
            rename = "timing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timing: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1TimeSpan,
            >,
        >,
        #[doc = "Output only. Non-fatal problems encountered during the execution of the build."]
        #[serde(
            rename = "warnings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warnings: ::std::option::Option<
            Vec<crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarning>,
        >,
    }
    impl ::google_field_selector::FieldSelector for ContaineranalysisGoogleDevtoolsCloudbuildV1Build {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContaineranalysisGoogleDevtoolsCloudbuildV1Build {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus {
        #[doc = "Build or step was canceled by a user."]
        Cancelled,
        #[doc = "Build was enqueued for longer than the value of `queue_ttl`."]
        Expired,
        #[doc = "Build or step failed to complete successfully."]
        Failure,
        #[doc = "Build or step failed due to an internal cause."]
        InternalError,
        #[doc = "Build has been created and is pending execution and queuing. It has not been queued."]
        Pending,
        #[doc = "Build or step is queued; work has not yet begun."]
        Queued,
        #[doc = "Status of the build is unknown."]
        StatusUnknown,
        #[doc = "Build or step finished successfully."]
        Success,
        #[doc = "Build or step took longer than was allowed."]
        Timeout,
        #[doc = "Build or step is being executed."]
        Working,
    }
    impl ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Cancelled => "CANCELLED",
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Expired => "EXPIRED",
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Failure => "FAILURE",
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::InternalError => {
                    "INTERNAL_ERROR"
                }
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Pending => "PENDING",
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Queued => "QUEUED",
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::StatusUnknown => {
                    "STATUS_UNKNOWN"
                }
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Success => "SUCCESS",
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Timeout => "TIMEOUT",
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Working => "WORKING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus, ()>
        {
            Ok(match s {
                "CANCELLED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Cancelled,
                "EXPIRED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Expired,
                "FAILURE" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Failure,
                "INTERNAL_ERROR" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::InternalError
                }
                "PENDING" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Pending,
                "QUEUED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Queued,
                "STATUS_UNKNOWN" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::StatusUnknown
                }
                "SUCCESS" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Success,
                "TIMEOUT" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Timeout,
                "WORKING" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Working,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Cancelled,
                "EXPIRED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Expired,
                "FAILURE" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Failure,
                "INTERNAL_ERROR" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::InternalError
                }
                "PENDING" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Pending,
                "QUEUED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Queued,
                "STATUS_UNKNOWN" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::StatusUnknown
                }
                "SUCCESS" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Success,
                "TIMEOUT" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Timeout,
                "WORKING" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus::Working,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStatus
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApproval {
        #[doc = "Output only. Configuration for manual approval of this build."]
        #[serde(
            rename = "config",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalConfig,
        >,
        #[doc = "Output only. Result of manual approval for this Build."]
        #[serde(
            rename = "result",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub result: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1ApprovalResult,
        >,
        #[doc = "Output only. The state of this build's approval."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApproval
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApproval
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState {
        #[doc = "Build approval has been approved."]
        Approved,
        #[doc = "Build was cancelled while it was still pending approval."]
        Cancelled,
        #[doc = "Build approval is pending."]
        Pending,
        #[doc = "Build approval has been rejected."]
        Rejected,
        #[doc = "Default enum type. This should not be used."]
        StateUnspecified,
    }
    impl ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState {
        pub fn as_str(self) -> &'static str {
            match self {
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState::Approved => {
                    "APPROVED"
                }
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState::Cancelled => {
                    "CANCELLED"
                }
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState::Pending => "PENDING",
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState::Rejected => {
                    "REJECTED"
                }
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState, ()>
        {
            Ok(match s {
                "APPROVED" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState::Approved
                }
                "CANCELLED" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState::Cancelled
                }
                "PENDING" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState::Pending,
                "REJECTED" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState::Rejected
                }
                "STATE_UNSPECIFIED" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState::StateUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPROVED" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState::Approved
                }
                "CANCELLED" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState::Cancelled
                }
                "PENDING" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState::Pending,
                "REJECTED" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState::Rejected
                }
                "STATE_UNSPECIFIED" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState::StateUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildApprovalState
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfo {
        #[doc = "Explains the failure issue in more detail using hard-coded text."]
        #[serde(
            rename = "detail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detail: ::std::option::Option<String>,
        #[doc = "The name of the failure."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfo
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfo
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType {
        #[doc = "Type unspecified"]
        FailureTypeUnspecified,
        #[doc = "The source fetching has failed."]
        FetchSourceFailed,
        #[doc = "Backend logging failures. Should retry."]
        LoggingFailure,
        #[doc = "Unable to push the image to the repository."]
        PushFailed,
        #[doc = "Final image not found."]
        PushImageNotFound,
        #[doc = "Unauthorized push of the final image."]
        PushNotAuthorized,
        #[doc = "A build step has failed."]
        UserBuildStep,
    }
    impl ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType {
        pub fn as_str(self) -> &'static str {
            match self { ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: FailureTypeUnspecified => "FAILURE_TYPE_UNSPECIFIED" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: FetchSourceFailed => "FETCH_SOURCE_FAILED" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: LoggingFailure => "LOGGING_FAILURE" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: PushFailed => "PUSH_FAILED" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: PushImageNotFound => "PUSH_IMAGE_NOT_FOUND" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: PushNotAuthorized => "PUSH_NOT_AUTHORIZED" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: UserBuildStep => "USER_BUILD_STEP" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType,
            (),
        > {
            Ok (match s { "FAILURE_TYPE_UNSPECIFIED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: FailureTypeUnspecified , "FETCH_SOURCE_FAILED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: FetchSourceFailed , "LOGGING_FAILURE" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: LoggingFailure , "PUSH_FAILED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: PushFailed , "PUSH_IMAGE_NOT_FOUND" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: PushImageNotFound , "PUSH_NOT_AUTHORIZED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: PushNotAuthorized , "USER_BUILD_STEP" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: UserBuildStep , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "FAILURE_TYPE_UNSPECIFIED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: FailureTypeUnspecified , "FETCH_SOURCE_FAILED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: FetchSourceFailed , "LOGGING_FAILURE" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: LoggingFailure , "PUSH_FAILED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: PushFailed , "PUSH_IMAGE_NOT_FOUND" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: PushImageNotFound , "PUSH_NOT_AUTHORIZED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: PushNotAuthorized , "USER_BUILD_STEP" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType :: UserBuildStep , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildFailureInfoType
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptions { # [doc = "Requested disk size for the VM that runs the build. Note that this is *NOT* \"disk free\"; some of the space will be used by the operating system and build utilities. Also note that this is the minimum disk size that will be allocated for the build -- the build may run with a larger disk than requested. At present, the maximum disk size is 1000GB; builds that request more than the maximum are rejected with an error."] # [serde (rename = "diskSizeGb" , default , skip_serializing_if = "std::option::Option::is_none")] # [serde (with = "crate::parsed_string")] pub disk_size_gb : :: std :: option :: Option < i64 > , # [doc = "Option to specify whether or not to apply bash style string operations to the substitutions. NOTE: this is always enabled for triggered builds and cannot be overridden in the build configuration file."] # [serde (rename = "dynamicSubstitutions" , default , skip_serializing_if = "std::option::Option::is_none")] pub dynamic_substitutions : :: std :: option :: Option < bool > , # [doc = "A list of global environment variable definitions that will exist for all build steps in this build. If a variable is defined in both globally and in a build step, the variable will use the build step value. The elements are of the form \"KEY=VALUE\" for the environment variable \"KEY\" being given the value \"VALUE\"."] # [serde (rename = "env" , default , skip_serializing_if = "std::option::Option::is_none")] pub env : :: std :: option :: Option < Vec < String > > , # [doc = "Option to define build log streaming behavior to Google Cloud Storage."] # [serde (rename = "logStreamingOption" , default , skip_serializing_if = "std::option::Option::is_none")] pub log_streaming_option : :: std :: option :: Option < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption > , # [doc = "Option to specify the logging mode, which determines if and where build logs are stored."] # [serde (rename = "logging" , default , skip_serializing_if = "std::option::Option::is_none")] pub logging : :: std :: option :: Option < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging > , # [doc = "Compute Engine machine type on which to run the build."] # [serde (rename = "machineType" , default , skip_serializing_if = "std::option::Option::is_none")] pub machine_type : :: std :: option :: Option < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType > , # [doc = "Optional. Specification for execution on a `WorkerPool`. See [running builds in a private pool](https://cloud.google.com/build/docs/private-pools/run-builds-in-private-pool) for more information."] # [serde (rename = "pool" , default , skip_serializing_if = "std::option::Option::is_none")] pub pool : :: std :: option :: Option < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsPoolOption > , # [doc = "Requested verifiability options."] # [serde (rename = "requestedVerifyOption" , default , skip_serializing_if = "std::option::Option::is_none")] pub requested_verify_option : :: std :: option :: Option < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption > , # [doc = "A list of global environment variables, which are encrypted using a Cloud Key Management Service crypto key. These values must be specified in the build's `Secret`. These variables will be available to all build steps in this build."] # [serde (rename = "secretEnv" , default , skip_serializing_if = "std::option::Option::is_none")] pub secret_env : :: std :: option :: Option < Vec < String > > , # [doc = "Requested hash for SourceProvenance."] # [serde (rename = "sourceProvenanceHash" , default , skip_serializing_if = "std::option::Option::is_none")] pub source_provenance_hash : :: std :: option :: Option < Vec < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems > > , # [doc = "Option to specify behavior when there is an error in the substitution checks. NOTE: this is always set to ALLOW_LOOSE for triggered builds and cannot be overridden in the build configuration file."] # [serde (rename = "substitutionOption" , default , skip_serializing_if = "std::option::Option::is_none")] pub substitution_option : :: std :: option :: Option < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption > , # [doc = "Global list of volumes to mount for ALL build steps Each volume is created as an empty volume prior to starting the build process. Upon completion of the build, volumes and their contents are discarded. Global volume names and paths cannot conflict with the volumes defined a build step. Using a global volume in a build with only one step is not valid as it is indicative of a build request with an incorrect configuration."] # [serde (rename = "volumes" , default , skip_serializing_if = "std::option::Option::is_none")] pub volumes : :: std :: option :: Option < Vec < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1Volume > > , # [doc = "This field deprecated; please use `pool.name` instead."] # [serde (rename = "workerPool" , default , skip_serializing_if = "std::option::Option::is_none")] pub worker_pool : :: std :: option :: Option < String > , }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptions
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptions
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption {
        #[doc = "Service may automatically determine build log streaming behavior."]
        StreamDefault,
        #[doc = "Build logs should not be streamed to Google Cloud Storage; they will be written when the build is completed."]
        StreamOff,
        #[doc = "Build logs should be streamed to Google Cloud Storage."]
        StreamOn,
    }
    impl ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption {
        pub fn as_str(self) -> &'static str {
            match self { ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption :: StreamDefault => "STREAM_DEFAULT" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption :: StreamOff => "STREAM_OFF" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption :: StreamOn => "STREAM_ON" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption,
            (),
        > {
            Ok (match s { "STREAM_DEFAULT" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption :: StreamDefault , "STREAM_OFF" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption :: StreamOff , "STREAM_ON" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption :: StreamOn , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "STREAM_DEFAULT" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption :: StreamDefault , "STREAM_OFF" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption :: StreamOff , "STREAM_ON" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption :: StreamOn , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging {
        #[doc = "Build logs are stored in Cloud Logging. Selecting this option will not allow [logs streaming](https://cloud.google.com/sdk/gcloud/reference/builds/log)."]
        CloudLoggingOnly,
        #[doc = "Build logs are stored in Cloud Storage."]
        GcsOnly,
        #[doc = "Build logs are stored in Cloud Logging and Cloud Storage."]
        Legacy,
        #[doc = "The service determines the logging mode. The default is `LEGACY`. Do not rely on the default logging behavior as it may change in the future."]
        LoggingUnspecified,
        #[doc = "Turn off all logging. No build logs will be captured."]
        None,
        #[doc = "This option is the same as CLOUD_LOGGING_ONLY."]
        StackdriverOnly,
    }
    impl ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging {
        pub fn as_str(self) -> &'static str {
            match self { ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: CloudLoggingOnly => "CLOUD_LOGGING_ONLY" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: GcsOnly => "GCS_ONLY" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: Legacy => "LEGACY" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: LoggingUnspecified => "LOGGING_UNSPECIFIED" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: None => "NONE" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: StackdriverOnly => "STACKDRIVER_ONLY" , }
        }
    }
    impl ::std::convert::AsRef<str> for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging, ()>
        {
            Ok (match s { "CLOUD_LOGGING_ONLY" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: CloudLoggingOnly , "GCS_ONLY" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: GcsOnly , "LEGACY" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: Legacy , "LOGGING_UNSPECIFIED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: LoggingUnspecified , "NONE" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: None , "STACKDRIVER_ONLY" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: StackdriverOnly , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CLOUD_LOGGING_ONLY" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: CloudLoggingOnly , "GCS_ONLY" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: GcsOnly , "LEGACY" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: Legacy , "LOGGING_UNSPECIFIED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: LoggingUnspecified , "NONE" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: None , "STACKDRIVER_ONLY" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging :: StackdriverOnly , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType {
        #[doc = "Highcpu e2 machine with 32 CPUs."]
        E2Highcpu32,
        #[doc = "Highcpu e2 machine with 8 CPUs."]
        E2Highcpu8,
        #[doc = "Highcpu machine with 32 CPUs."]
        N1Highcpu32,
        #[doc = "Highcpu machine with 8 CPUs."]
        N1Highcpu8,
        #[doc = "Standard machine type."]
        Unspecified,
    }
    impl ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType {
        pub fn as_str(self) -> &'static str {
            match self {
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType::E2Highcpu32 => {
                    "E2_HIGHCPU_32"
                }
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType::E2Highcpu8 => {
                    "E2_HIGHCPU_8"
                }
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType::N1Highcpu32 => {
                    "N1_HIGHCPU_32"
                }
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType::N1Highcpu8 => {
                    "N1_HIGHCPU_8"
                }
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType::Unspecified => {
                    "UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType,
            (),
        > {
            Ok(match s {
                "E2_HIGHCPU_32" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType::E2Highcpu32
                }
                "E2_HIGHCPU_8" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType::E2Highcpu8
                }
                "N1_HIGHCPU_32" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType::N1Highcpu32
                }
                "N1_HIGHCPU_8" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType::N1Highcpu8
                }
                "UNSPECIFIED" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType::Unspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "E2_HIGHCPU_32" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType::E2Highcpu32
                }
                "E2_HIGHCPU_8" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType::E2Highcpu8
                }
                "N1_HIGHCPU_32" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType::N1Highcpu32
                }
                "N1_HIGHCPU_8" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType::N1Highcpu8
                }
                "UNSPECIFIED" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType::Unspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption {
        #[doc = "Not a verifiable build. (default)"]
        NotVerified,
        #[doc = "Verified build."]
        Verified,
    }
    impl ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption {
        pub fn as_str(self) -> &'static str {
            match self { ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption :: NotVerified => "NOT_VERIFIED" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption :: Verified => "VERIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption,
            (),
        > {
            Ok (match s { "NOT_VERIFIED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption :: NotVerified , "VERIFIED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption :: Verified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "NOT_VERIFIED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption :: NotVerified , "VERIFIED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption :: Verified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems {
        #[doc = "Use a md5 hash."]
        Md5,
        #[doc = "No hash requested."]
        None,
        #[doc = "Use a sha256 hash."]
        Sha256,
    }
    impl ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems {
        pub fn as_str(self) -> &'static str {
            match self { ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems :: Md5 => "MD5" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems :: None => "NONE" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems :: Sha256 => "SHA256" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems,
            (),
        > {
            Ok (match s { "MD5" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems :: Md5 , "NONE" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems :: None , "SHA256" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems :: Sha256 , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "MD5" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems :: Md5 , "NONE" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems :: None , "SHA256" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems :: Sha256 , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption {
        #[doc = "Do not fail the build if error in substitutions checks."]
        AllowLoose,
        #[doc = "Fails the build if error in substitutions checks, like missing a substitution in the template or in the map."]
        MustMatch,
    }
    impl ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption {
        pub fn as_str(self) -> &'static str {
            match self { ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption :: AllowLoose => "ALLOW_LOOSE" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption :: MustMatch => "MUST_MATCH" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption,
            (),
        > {
            Ok (match s { "ALLOW_LOOSE" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption :: AllowLoose , "MUST_MATCH" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption :: MustMatch , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ALLOW_LOOSE" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption :: AllowLoose , "MUST_MATCH" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption :: MustMatch , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsPoolOption {
        #[doc = "The `WorkerPool` resource to execute the build on. You must have `cloudbuild.workerpools.use` on the project hosting the WorkerPool. Format projects/{project}/locations/{location}/workerPools/{workerPoolId}"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsPoolOption
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsPoolOption
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStep {
        #[doc = "A list of arguments that will be presented to the step when it is started. If the image used to run the step's container has an entrypoint, the `args` are used as arguments to that entrypoint. If the image does not define an entrypoint, the first element in args is used as the entrypoint, and the remainder will be used as arguments."]
        #[serde(
            rename = "args",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub args: ::std::option::Option<Vec<String>>,
        #[doc = "Working directory to use when running this step's container. If this value is a relative path, it is relative to the build's working directory. If this value is absolute, it may be outside the build's working directory, in which case the contents of the path may not be persisted across build step executions, unless a `volume` for that path is specified. If the build specifies a `RepoSource` with `dir` and a step with a `dir`, which specifies an absolute path, the `RepoSource` `dir` is ignored for the step's execution."]
        #[serde(
            rename = "dir",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dir: ::std::option::Option<String>,
        #[doc = "Entrypoint to be used instead of the build step image's default entrypoint. If unset, the image's default entrypoint is used."]
        #[serde(
            rename = "entrypoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entrypoint: ::std::option::Option<String>,
        #[doc = "A list of environment variable definitions to be used when running a step. The elements are of the form \"KEY=VALUE\" for the environment variable \"KEY\" being given the value \"VALUE\"."]
        #[serde(
            rename = "env",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub env: ::std::option::Option<Vec<String>>,
        #[doc = "Unique identifier for this build step, used in `wait_for` to reference this build step as a dependency."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Required. The name of the container image that will run this particular build step. If the image is available in the host's Docker daemon's cache, it will be run directly. If not, the host will attempt to pull the image first, using the builder service account's credentials if necessary. The Docker daemon's cache will already have the latest versions of all of the officially supported build steps ([https://github.com/GoogleCloudPlatform/cloud-builders](https://github.com/GoogleCloudPlatform/cloud-builders)). The Docker daemon will also have cached many of the layers for some popular images, like \"ubuntu\", \"debian\", but they will be refreshed at the time you attempt to use them. If you built an image in a previous build step, it will be stored in the host's Docker daemon's cache and is available to use as the name for a later build step."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Stores timing information for pulling this build step's builder image only."]
        #[serde(
            rename = "pullTiming",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pull_timing: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1TimeSpan,
        >,
        #[doc = "A shell script to be executed in the step. When script is provided, the user cannot specify the entrypoint or args."]
        #[serde(
            rename = "script",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub script: ::std::option::Option<String>,
        #[doc = "A list of environment variables which are encrypted using a Cloud Key Management Service crypto key. These values must be specified in the build's `Secret`."]
        #[serde(
            rename = "secretEnv",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub secret_env: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. Status of the build step. At this time, build step status is only updated on build completion; step status is not updated in real-time as the build progresses."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus,
        >,
        #[doc = "Time limit for executing this build step. If not defined, the step has no time limit and will be allowed to continue to run until either it completes or the build itself times out."]
        #[serde(
            rename = "timeout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timeout: ::std::option::Option<String>,
        #[doc = "Output only. Stores timing information for executing this build step."]
        #[serde(
            rename = "timing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timing: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1TimeSpan,
        >,
        #[doc = "List of volumes to mount into the build step. Each volume is created as an empty volume prior to execution of the build step. Upon completion of the build, volumes and their contents are discarded. Using a named volume in only one step is not valid as it is indicative of a build request with an incorrect configuration."]
        #[serde(
            rename = "volumes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub volumes: ::std::option::Option<
            Vec<crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1Volume>,
        >,
        #[doc = "The ID(s) of the step(s) that this build step depends on. This build step will not start until all the build steps in `wait_for` have completed successfully. If `wait_for` is empty, this build step will start when all previous build steps in the `Build.Steps` list have completed successfully."]
        #[serde(
            rename = "waitFor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wait_for: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStep
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStep {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus {
        #[doc = "Build or step was canceled by a user."]
        Cancelled,
        #[doc = "Build was enqueued for longer than the value of `queue_ttl`."]
        Expired,
        #[doc = "Build or step failed to complete successfully."]
        Failure,
        #[doc = "Build or step failed due to an internal cause."]
        InternalError,
        #[doc = "Build has been created and is pending execution and queuing. It has not been queued."]
        Pending,
        #[doc = "Build or step is queued; work has not yet begun."]
        Queued,
        #[doc = "Status of the build is unknown."]
        StatusUnknown,
        #[doc = "Build or step finished successfully."]
        Success,
        #[doc = "Build or step took longer than was allowed."]
        Timeout,
        #[doc = "Build or step is being executed."]
        Working,
    }
    impl ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Cancelled => {
                    "CANCELLED"
                }
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Expired => "EXPIRED",
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Failure => "FAILURE",
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::InternalError => {
                    "INTERNAL_ERROR"
                }
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Pending => "PENDING",
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Queued => "QUEUED",
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::StatusUnknown => {
                    "STATUS_UNKNOWN"
                }
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Success => "SUCCESS",
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Timeout => "TIMEOUT",
                ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Working => "WORKING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus, ()>
        {
            Ok(match s {
                "CANCELLED" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Cancelled
                }
                "EXPIRED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Expired,
                "FAILURE" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Failure,
                "INTERNAL_ERROR" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::InternalError
                }
                "PENDING" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Pending,
                "QUEUED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Queued,
                "STATUS_UNKNOWN" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::StatusUnknown
                }
                "SUCCESS" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Success,
                "TIMEOUT" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Timeout,
                "WORKING" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Working,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELLED" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Cancelled
                }
                "EXPIRED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Expired,
                "FAILURE" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Failure,
                "INTERNAL_ERROR" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::InternalError
                }
                "PENDING" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Pending,
                "QUEUED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Queued,
                "STATUS_UNKNOWN" => {
                    ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::StatusUnknown
                }
                "SUCCESS" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Success,
                "TIMEOUT" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Timeout,
                "WORKING" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus::Working,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildStepStatus
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarning {
        #[doc = "The priority for this warning."]
        #[serde(
            rename = "priority",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub priority: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority,
        >,
        #[doc = "Explanation of the warning generated."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarning
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarning
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority {
        #[doc = "e.g. alerts that a feature used in the build is pending removal"]
        Alert,
        #[doc = "e.g. deprecation warnings and alternative feature highlights."]
        Info,
        #[doc = "Should not be used."]
        PriorityUnspecified,
        #[doc = "e.g. automated detection of possible issues with the build."]
        Warning,
    }
    impl ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority {
        pub fn as_str(self) -> &'static str {
            match self { ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority :: Alert => "ALERT" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority :: Info => "INFO" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority :: PriorityUnspecified => "PRIORITY_UNSPECIFIED" , ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority :: Warning => "WARNING" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority,
            (),
        > {
            Ok (match s { "ALERT" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority :: Alert , "INFO" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority :: Info , "PRIORITY_UNSPECIFIED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority :: PriorityUnspecified , "WARNING" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority :: Warning , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ALERT" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority :: Alert , "INFO" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority :: Info , "PRIORITY_UNSPECIFIED" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority :: PriorityUnspecified , "WARNING" => ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority :: Warning , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuildWarningPriority
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1BuiltImage {
        #[doc = "Docker Registry 2.0 digest."]
        #[serde(
            rename = "digest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub digest: ::std::option::Option<String>,
        #[doc = "Name used to push the container image to Google Container Registry, as presented to `docker push`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Stores timing information for pushing the specified image."]
        #[serde(
            rename = "pushTiming",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub push_timing: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1TimeSpan,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuiltImage
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1BuiltImage
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1FileHashes {
        #[doc = "Collection of file hashes."]
        #[serde(
            rename = "fileHash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_hash: ::std::option::Option<
            Vec<crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1Hash>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1FileHashes
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1FileHashes
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1Hash {
        #[doc = "The type of hash that was performed."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1HashType,
        >,
        #[doc = "The hash value."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for ContaineranalysisGoogleDevtoolsCloudbuildV1Hash {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContaineranalysisGoogleDevtoolsCloudbuildV1Hash {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContaineranalysisGoogleDevtoolsCloudbuildV1HashType {
        #[doc = "Use a md5 hash."]
        Md5,
        #[doc = "No hash requested."]
        None,
        #[doc = "Use a sha256 hash."]
        Sha256,
    }
    impl ContaineranalysisGoogleDevtoolsCloudbuildV1HashType {
        pub fn as_str(self) -> &'static str {
            match self {
                ContaineranalysisGoogleDevtoolsCloudbuildV1HashType::Md5 => "MD5",
                ContaineranalysisGoogleDevtoolsCloudbuildV1HashType::None => "NONE",
                ContaineranalysisGoogleDevtoolsCloudbuildV1HashType::Sha256 => "SHA256",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ContaineranalysisGoogleDevtoolsCloudbuildV1HashType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ContaineranalysisGoogleDevtoolsCloudbuildV1HashType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ContaineranalysisGoogleDevtoolsCloudbuildV1HashType, ()>
        {
            Ok(match s {
                "MD5" => ContaineranalysisGoogleDevtoolsCloudbuildV1HashType::Md5,
                "NONE" => ContaineranalysisGoogleDevtoolsCloudbuildV1HashType::None,
                "SHA256" => ContaineranalysisGoogleDevtoolsCloudbuildV1HashType::Sha256,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ContaineranalysisGoogleDevtoolsCloudbuildV1HashType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ContaineranalysisGoogleDevtoolsCloudbuildV1HashType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ContaineranalysisGoogleDevtoolsCloudbuildV1HashType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MD5" => ContaineranalysisGoogleDevtoolsCloudbuildV1HashType::Md5,
                "NONE" => ContaineranalysisGoogleDevtoolsCloudbuildV1HashType::None,
                "SHA256" => ContaineranalysisGoogleDevtoolsCloudbuildV1HashType::Sha256,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1HashType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContaineranalysisGoogleDevtoolsCloudbuildV1HashType {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1InlineSecret {
        #[doc = "Map of environment variable name to its encrypted value. Secret environment variables must be unique across all of a build's secrets, and must be used by at least one build step. Values can be at most 64 KB in size. There can be at most 100 secret values across all of a build's secrets."]
        #[serde(
            rename = "envMap",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub env_map:
            ::std::option::Option<::std::collections::BTreeMap<String, ::google_api_bytes::Bytes>>,
        #[doc = "Resource name of Cloud KMS crypto key to decrypt the encrypted value. In format: projects/*/locations/*/keyRings/*/cryptoKeys/*"]
        #[serde(
            rename = "kmsKeyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_key_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1InlineSecret
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1InlineSecret
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1RepoSource {
        #[doc = "Regex matching branches to build. The syntax of the regular expressions accepted is the syntax accepted by RE2 and described at https://github.com/google/re2/wiki/Syntax"]
        #[serde(
            rename = "branchName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub branch_name: ::std::option::Option<String>,
        #[doc = "Explicit commit SHA to build."]
        #[serde(
            rename = "commitSha",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub commit_sha: ::std::option::Option<String>,
        #[doc = "Directory, relative to the source root, in which to run the build. This must be a relative path. If a step's `dir` is specified and is an absolute path, this value is ignored for that step's execution."]
        #[serde(
            rename = "dir",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dir: ::std::option::Option<String>,
        #[doc = "Only trigger a build if the revision regex does NOT match the revision regex."]
        #[serde(
            rename = "invertRegex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub invert_regex: ::std::option::Option<bool>,
        #[doc = "ID of the project that owns the Cloud Source Repository. If omitted, the project ID requesting the build is assumed."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "Name of the Cloud Source Repository."]
        #[serde(
            rename = "repoName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repo_name: ::std::option::Option<String>,
        #[doc = "Substitutions to use in a triggered build. Should only be used with RunBuildTrigger"]
        #[serde(
            rename = "substitutions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub substitutions: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Regex matching tags to build. The syntax of the regular expressions accepted is the syntax accepted by RE2 and described at https://github.com/google/re2/wiki/Syntax"]
        #[serde(
            rename = "tagName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tag_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1RepoSource
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1RepoSource
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1Results {
        #[doc = "Path to the artifact manifest. Only populated when artifacts are uploaded."]
        #[serde(
            rename = "artifactManifest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub artifact_manifest: ::std::option::Option<String>,
        #[doc = "Time to push all non-container artifacts."]
        #[serde(
            rename = "artifactTiming",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub artifact_timing: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1TimeSpan,
        >,
        #[doc = "List of build step digests, in the order corresponding to build step indices."]
        #[serde(
            rename = "buildStepImages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_step_images: ::std::option::Option<Vec<String>>,
        #[doc = "List of build step outputs, produced by builder images, in the order corresponding to build step indices. [Cloud Builders](https://cloud.google.com/cloud-build/docs/cloud-builders) can produce this output by writing to `$BUILDER_OUTPUT/output`. Only the first 4KB of data is stored."]
        #[serde(
            rename = "buildStepOutputs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_step_outputs: ::std::option::Option<Vec<::google_api_bytes::Bytes>>,
        #[doc = "Container images that were built as a part of the build."]
        #[serde(
            rename = "images",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub images: ::std::option::Option<
            Vec<crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1BuiltImage>,
        >,
        #[doc = "Number of artifacts uploaded. Only populated when artifacts are uploaded."]
        #[serde(
            rename = "numArtifacts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_artifacts: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for ContaineranalysisGoogleDevtoolsCloudbuildV1Results {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContaineranalysisGoogleDevtoolsCloudbuildV1Results {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1Secret {
        #[doc = "Cloud KMS key name to use to decrypt these envs."]
        #[serde(
            rename = "kmsKeyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kms_key_name: ::std::option::Option<String>,
        #[doc = "Map of environment variable name to its encrypted value. Secret environment variables must be unique across all of a build's secrets, and must be used by at least one build step. Values can be at most 64 KB in size. There can be at most 100 secret values across all of a build's secrets."]
        #[serde(
            rename = "secretEnv",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub secret_env:
            ::std::option::Option<::std::collections::BTreeMap<String, ::google_api_bytes::Bytes>>,
    }
    impl ::google_field_selector::FieldSelector for ContaineranalysisGoogleDevtoolsCloudbuildV1Secret {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContaineranalysisGoogleDevtoolsCloudbuildV1Secret {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1SecretManagerSecret {
        #[doc = "Environment variable name to associate with the secret. Secret environment variables must be unique across all of a build's secrets, and must be used by at least one build step."]
        #[serde(
            rename = "env",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub env: ::std::option::Option<String>,
        #[doc = "Resource name of the SecretVersion. In format: projects/*/secrets/*/versions/*"]
        #[serde(
            rename = "versionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1SecretManagerSecret
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1SecretManagerSecret
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1Secrets {
        #[doc = "Secrets encrypted with KMS key and the associated secret environment variable."]
        #[serde(
            rename = "inline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inline: ::std::option::Option<
            Vec<crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1InlineSecret>,
        >,
        #[doc = "Secrets in Secret Manager and associated secret environment variable."]
        #[serde(
            rename = "secretManager",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub secret_manager: ::std::option::Option<
            Vec<crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1SecretManagerSecret>,
        >,
    }
    impl ::google_field_selector::FieldSelector for ContaineranalysisGoogleDevtoolsCloudbuildV1Secrets {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContaineranalysisGoogleDevtoolsCloudbuildV1Secrets {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1Source {
        #[doc = "If provided, get the source from this location in a Cloud Source Repository."]
        #[serde(
            rename = "repoSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repo_source: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1RepoSource,
        >,
        #[doc = "If provided, get the source from this location in Google Cloud Storage."]
        #[serde(
            rename = "storageSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub storage_source: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1StorageSource,
        >,
        #[doc = "If provided, get the source from this manifest in Google Cloud Storage. This feature is in Preview; see description [here](https://github.com/GoogleCloudPlatform/cloud-builders/tree/master/gcs-fetcher)."]
        #[serde(
            rename = "storageSourceManifest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub storage_source_manifest: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1StorageSourceManifest,
        >,
    }
    impl ::google_field_selector::FieldSelector for ContaineranalysisGoogleDevtoolsCloudbuildV1Source {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContaineranalysisGoogleDevtoolsCloudbuildV1Source {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1SourceProvenance {
        #[doc = "Output only. Hash(es) of the build source, which can be used to verify that the original source integrity was maintained in the build. Note that `FileHashes` will only be populated if `BuildOptions` has requested a `SourceProvenanceHash`. The keys to this map are file paths used as build source and the values contain the hash values for those files. If the build source came in a single package such as a gzipped tarfile (`.tar.gz`), the `FileHash` will be for the single path to that file."]
        #[serde(
            rename = "fileHashes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_hashes: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1FileHashes,
            >,
        >,
        #[doc = "A copy of the build's `source.repo_source`, if exists, with any revisions resolved."]
        #[serde(
            rename = "resolvedRepoSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolved_repo_source: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1RepoSource,
        >,
        #[doc = "A copy of the build's `source.storage_source`, if exists, with any generations resolved."]
        #[serde(
            rename = "resolvedStorageSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolved_storage_source: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1StorageSource,
        >,
        #[doc = "A copy of the build's `source.storage_source_manifest`, if exists, with any revisions resolved. This feature is in Preview."]
        #[serde(
            rename = "resolvedStorageSourceManifest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolved_storage_source_manifest: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1StorageSourceManifest,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1SourceProvenance
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1SourceProvenance
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1StorageSource {
        #[doc = "Google Cloud Storage bucket containing the source (see [Bucket Name Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements))."]
        #[serde(
            rename = "bucket",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket: ::std::option::Option<String>,
        #[doc = "Google Cloud Storage generation for the object. If the generation is omitted, the latest generation will be used."]
        #[serde(
            rename = "generation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub generation: ::std::option::Option<i64>,
        #[doc = "Google Cloud Storage object containing the source. This object must be a zipped (`.zip`) or gzipped archive file (`.tar.gz`) containing source to build."]
        #[serde(
            rename = "object",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1StorageSource
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1StorageSource
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1StorageSourceManifest {
        #[doc = "Google Cloud Storage bucket containing the source manifest (see [Bucket Name Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements))."]
        #[serde(
            rename = "bucket",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket: ::std::option::Option<String>,
        #[doc = "Google Cloud Storage generation for the object. If the generation is omitted, the latest generation will be used."]
        #[serde(
            rename = "generation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub generation: ::std::option::Option<i64>,
        #[doc = "Google Cloud Storage object containing the source manifest. This object must be a JSON file."]
        #[serde(
            rename = "object",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1StorageSourceManifest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for ContaineranalysisGoogleDevtoolsCloudbuildV1StorageSourceManifest
    {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1TimeSpan {
        #[doc = "End of time span."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Start of time span."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for ContaineranalysisGoogleDevtoolsCloudbuildV1TimeSpan
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContaineranalysisGoogleDevtoolsCloudbuildV1TimeSpan {
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1Volume {
        #[doc = "Name of the volume to mount. Volume names must be unique per build step and must be valid names for Docker volumes. Each named volume must be used by at least two build steps."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Path at which to mount the volume. Paths must be absolute and cannot conflict with other volume paths on the same build step or with certain reserved volume paths."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ContaineranalysisGoogleDevtoolsCloudbuildV1Volume {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContaineranalysisGoogleDevtoolsCloudbuildV1Volume {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Cvss {
        #[serde(
            rename = "attackComplexity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attack_complexity: ::std::option::Option<crate::schemas::CvssAttackComplexity>,
        #[doc = "Base Metrics Represents the intrinsic characteristics of a vulnerability that are constant over time and across user environments."]
        #[serde(
            rename = "attackVector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attack_vector: ::std::option::Option<crate::schemas::CvssAttackVector>,
        #[serde(
            rename = "authentication",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authentication: ::std::option::Option<crate::schemas::CvssAuthentication>,
        #[serde(
            rename = "availabilityImpact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub availability_impact: ::std::option::Option<crate::schemas::CvssAvailabilityImpact>,
        #[doc = "The base score is a function of the base metric scores."]
        #[serde(
            rename = "baseScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub base_score: ::std::option::Option<f32>,
        #[serde(
            rename = "confidentialityImpact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidentiality_impact:
            ::std::option::Option<crate::schemas::CvssConfidentialityImpact>,
        #[serde(
            rename = "exploitabilityScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exploitability_score: ::std::option::Option<f32>,
        #[serde(
            rename = "impactScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub impact_score: ::std::option::Option<f32>,
        #[serde(
            rename = "integrityImpact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub integrity_impact: ::std::option::Option<crate::schemas::CvssIntegrityImpact>,
        #[serde(
            rename = "privilegesRequired",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub privileges_required: ::std::option::Option<crate::schemas::CvssPrivilegesRequired>,
        #[serde(
            rename = "scope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scope: ::std::option::Option<crate::schemas::CvssScope>,
        #[serde(
            rename = "userInteraction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_interaction: ::std::option::Option<crate::schemas::CvssUserInteraction>,
    }
    impl ::google_field_selector::FieldSelector for Cvss {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvss {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssAttackComplexity {
        AttackComplexityHigh,
        AttackComplexityLow,
        AttackComplexityUnspecified,
    }
    impl CvssAttackComplexity {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssAttackComplexity::AttackComplexityHigh => "ATTACK_COMPLEXITY_HIGH",
                CvssAttackComplexity::AttackComplexityLow => "ATTACK_COMPLEXITY_LOW",
                CvssAttackComplexity::AttackComplexityUnspecified => {
                    "ATTACK_COMPLEXITY_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssAttackComplexity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssAttackComplexity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssAttackComplexity, ()> {
            Ok(match s {
                "ATTACK_COMPLEXITY_HIGH" => CvssAttackComplexity::AttackComplexityHigh,
                "ATTACK_COMPLEXITY_LOW" => CvssAttackComplexity::AttackComplexityLow,
                "ATTACK_COMPLEXITY_UNSPECIFIED" => {
                    CvssAttackComplexity::AttackComplexityUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssAttackComplexity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssAttackComplexity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssAttackComplexity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTACK_COMPLEXITY_HIGH" => CvssAttackComplexity::AttackComplexityHigh,
                "ATTACK_COMPLEXITY_LOW" => CvssAttackComplexity::AttackComplexityLow,
                "ATTACK_COMPLEXITY_UNSPECIFIED" => {
                    CvssAttackComplexity::AttackComplexityUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssAttackComplexity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssAttackComplexity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssAttackVector {
        AttackVectorAdjacent,
        AttackVectorLocal,
        AttackVectorNetwork,
        AttackVectorPhysical,
        AttackVectorUnspecified,
    }
    impl CvssAttackVector {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssAttackVector::AttackVectorAdjacent => "ATTACK_VECTOR_ADJACENT",
                CvssAttackVector::AttackVectorLocal => "ATTACK_VECTOR_LOCAL",
                CvssAttackVector::AttackVectorNetwork => "ATTACK_VECTOR_NETWORK",
                CvssAttackVector::AttackVectorPhysical => "ATTACK_VECTOR_PHYSICAL",
                CvssAttackVector::AttackVectorUnspecified => "ATTACK_VECTOR_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssAttackVector {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssAttackVector {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssAttackVector, ()> {
            Ok(match s {
                "ATTACK_VECTOR_ADJACENT" => CvssAttackVector::AttackVectorAdjacent,
                "ATTACK_VECTOR_LOCAL" => CvssAttackVector::AttackVectorLocal,
                "ATTACK_VECTOR_NETWORK" => CvssAttackVector::AttackVectorNetwork,
                "ATTACK_VECTOR_PHYSICAL" => CvssAttackVector::AttackVectorPhysical,
                "ATTACK_VECTOR_UNSPECIFIED" => CvssAttackVector::AttackVectorUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssAttackVector {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssAttackVector {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssAttackVector {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTACK_VECTOR_ADJACENT" => CvssAttackVector::AttackVectorAdjacent,
                "ATTACK_VECTOR_LOCAL" => CvssAttackVector::AttackVectorLocal,
                "ATTACK_VECTOR_NETWORK" => CvssAttackVector::AttackVectorNetwork,
                "ATTACK_VECTOR_PHYSICAL" => CvssAttackVector::AttackVectorPhysical,
                "ATTACK_VECTOR_UNSPECIFIED" => CvssAttackVector::AttackVectorUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssAttackVector {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssAttackVector {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssAuthentication {
        AuthenticationMultiple,
        AuthenticationNone,
        AuthenticationSingle,
        AuthenticationUnspecified,
    }
    impl CvssAuthentication {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssAuthentication::AuthenticationMultiple => "AUTHENTICATION_MULTIPLE",
                CvssAuthentication::AuthenticationNone => "AUTHENTICATION_NONE",
                CvssAuthentication::AuthenticationSingle => "AUTHENTICATION_SINGLE",
                CvssAuthentication::AuthenticationUnspecified => "AUTHENTICATION_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssAuthentication {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssAuthentication {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssAuthentication, ()> {
            Ok(match s {
                "AUTHENTICATION_MULTIPLE" => CvssAuthentication::AuthenticationMultiple,
                "AUTHENTICATION_NONE" => CvssAuthentication::AuthenticationNone,
                "AUTHENTICATION_SINGLE" => CvssAuthentication::AuthenticationSingle,
                "AUTHENTICATION_UNSPECIFIED" => CvssAuthentication::AuthenticationUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssAuthentication {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssAuthentication {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssAuthentication {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTHENTICATION_MULTIPLE" => CvssAuthentication::AuthenticationMultiple,
                "AUTHENTICATION_NONE" => CvssAuthentication::AuthenticationNone,
                "AUTHENTICATION_SINGLE" => CvssAuthentication::AuthenticationSingle,
                "AUTHENTICATION_UNSPECIFIED" => CvssAuthentication::AuthenticationUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssAuthentication {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssAuthentication {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssAvailabilityImpact {
        ImpactHigh,
        ImpactLow,
        ImpactNone,
        ImpactUnspecified,
    }
    impl CvssAvailabilityImpact {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssAvailabilityImpact::ImpactHigh => "IMPACT_HIGH",
                CvssAvailabilityImpact::ImpactLow => "IMPACT_LOW",
                CvssAvailabilityImpact::ImpactNone => "IMPACT_NONE",
                CvssAvailabilityImpact::ImpactUnspecified => "IMPACT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssAvailabilityImpact {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssAvailabilityImpact {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssAvailabilityImpact, ()> {
            Ok(match s {
                "IMPACT_HIGH" => CvssAvailabilityImpact::ImpactHigh,
                "IMPACT_LOW" => CvssAvailabilityImpact::ImpactLow,
                "IMPACT_NONE" => CvssAvailabilityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => CvssAvailabilityImpact::ImpactUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssAvailabilityImpact {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssAvailabilityImpact {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssAvailabilityImpact {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMPACT_HIGH" => CvssAvailabilityImpact::ImpactHigh,
                "IMPACT_LOW" => CvssAvailabilityImpact::ImpactLow,
                "IMPACT_NONE" => CvssAvailabilityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => CvssAvailabilityImpact::ImpactUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssAvailabilityImpact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssAvailabilityImpact {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssConfidentialityImpact {
        ImpactHigh,
        ImpactLow,
        ImpactNone,
        ImpactUnspecified,
    }
    impl CvssConfidentialityImpact {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssConfidentialityImpact::ImpactHigh => "IMPACT_HIGH",
                CvssConfidentialityImpact::ImpactLow => "IMPACT_LOW",
                CvssConfidentialityImpact::ImpactNone => "IMPACT_NONE",
                CvssConfidentialityImpact::ImpactUnspecified => "IMPACT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssConfidentialityImpact {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssConfidentialityImpact {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssConfidentialityImpact, ()> {
            Ok(match s {
                "IMPACT_HIGH" => CvssConfidentialityImpact::ImpactHigh,
                "IMPACT_LOW" => CvssConfidentialityImpact::ImpactLow,
                "IMPACT_NONE" => CvssConfidentialityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => CvssConfidentialityImpact::ImpactUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssConfidentialityImpact {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssConfidentialityImpact {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssConfidentialityImpact {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMPACT_HIGH" => CvssConfidentialityImpact::ImpactHigh,
                "IMPACT_LOW" => CvssConfidentialityImpact::ImpactLow,
                "IMPACT_NONE" => CvssConfidentialityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => CvssConfidentialityImpact::ImpactUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssConfidentialityImpact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssConfidentialityImpact {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssIntegrityImpact {
        ImpactHigh,
        ImpactLow,
        ImpactNone,
        ImpactUnspecified,
    }
    impl CvssIntegrityImpact {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssIntegrityImpact::ImpactHigh => "IMPACT_HIGH",
                CvssIntegrityImpact::ImpactLow => "IMPACT_LOW",
                CvssIntegrityImpact::ImpactNone => "IMPACT_NONE",
                CvssIntegrityImpact::ImpactUnspecified => "IMPACT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssIntegrityImpact {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssIntegrityImpact {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssIntegrityImpact, ()> {
            Ok(match s {
                "IMPACT_HIGH" => CvssIntegrityImpact::ImpactHigh,
                "IMPACT_LOW" => CvssIntegrityImpact::ImpactLow,
                "IMPACT_NONE" => CvssIntegrityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => CvssIntegrityImpact::ImpactUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssIntegrityImpact {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssIntegrityImpact {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssIntegrityImpact {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMPACT_HIGH" => CvssIntegrityImpact::ImpactHigh,
                "IMPACT_LOW" => CvssIntegrityImpact::ImpactLow,
                "IMPACT_NONE" => CvssIntegrityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => CvssIntegrityImpact::ImpactUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssIntegrityImpact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssIntegrityImpact {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssPrivilegesRequired {
        PrivilegesRequiredHigh,
        PrivilegesRequiredLow,
        PrivilegesRequiredNone,
        PrivilegesRequiredUnspecified,
    }
    impl CvssPrivilegesRequired {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssPrivilegesRequired::PrivilegesRequiredHigh => "PRIVILEGES_REQUIRED_HIGH",
                CvssPrivilegesRequired::PrivilegesRequiredLow => "PRIVILEGES_REQUIRED_LOW",
                CvssPrivilegesRequired::PrivilegesRequiredNone => "PRIVILEGES_REQUIRED_NONE",
                CvssPrivilegesRequired::PrivilegesRequiredUnspecified => {
                    "PRIVILEGES_REQUIRED_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssPrivilegesRequired {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssPrivilegesRequired {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssPrivilegesRequired, ()> {
            Ok(match s {
                "PRIVILEGES_REQUIRED_HIGH" => CvssPrivilegesRequired::PrivilegesRequiredHigh,
                "PRIVILEGES_REQUIRED_LOW" => CvssPrivilegesRequired::PrivilegesRequiredLow,
                "PRIVILEGES_REQUIRED_NONE" => CvssPrivilegesRequired::PrivilegesRequiredNone,
                "PRIVILEGES_REQUIRED_UNSPECIFIED" => {
                    CvssPrivilegesRequired::PrivilegesRequiredUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssPrivilegesRequired {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssPrivilegesRequired {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssPrivilegesRequired {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PRIVILEGES_REQUIRED_HIGH" => CvssPrivilegesRequired::PrivilegesRequiredHigh,
                "PRIVILEGES_REQUIRED_LOW" => CvssPrivilegesRequired::PrivilegesRequiredLow,
                "PRIVILEGES_REQUIRED_NONE" => CvssPrivilegesRequired::PrivilegesRequiredNone,
                "PRIVILEGES_REQUIRED_UNSPECIFIED" => {
                    CvssPrivilegesRequired::PrivilegesRequiredUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssPrivilegesRequired {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssPrivilegesRequired {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssScope {
        ScopeChanged,
        ScopeUnchanged,
        ScopeUnspecified,
    }
    impl CvssScope {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssScope::ScopeChanged => "SCOPE_CHANGED",
                CvssScope::ScopeUnchanged => "SCOPE_UNCHANGED",
                CvssScope::ScopeUnspecified => "SCOPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssScope {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssScope {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssScope, ()> {
            Ok(match s {
                "SCOPE_CHANGED" => CvssScope::ScopeChanged,
                "SCOPE_UNCHANGED" => CvssScope::ScopeUnchanged,
                "SCOPE_UNSPECIFIED" => CvssScope::ScopeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssScope {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssScope {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssScope {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SCOPE_CHANGED" => CvssScope::ScopeChanged,
                "SCOPE_UNCHANGED" => CvssScope::ScopeUnchanged,
                "SCOPE_UNSPECIFIED" => CvssScope::ScopeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssScope {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssScope {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CvssUserInteraction {
        UserInteractionNone,
        UserInteractionRequired,
        UserInteractionUnspecified,
    }
    impl CvssUserInteraction {
        pub fn as_str(self) -> &'static str {
            match self {
                CvssUserInteraction::UserInteractionNone => "USER_INTERACTION_NONE",
                CvssUserInteraction::UserInteractionRequired => "USER_INTERACTION_REQUIRED",
                CvssUserInteraction::UserInteractionUnspecified => "USER_INTERACTION_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CvssUserInteraction {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CvssUserInteraction {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CvssUserInteraction, ()> {
            Ok(match s {
                "USER_INTERACTION_NONE" => CvssUserInteraction::UserInteractionNone,
                "USER_INTERACTION_REQUIRED" => CvssUserInteraction::UserInteractionRequired,
                "USER_INTERACTION_UNSPECIFIED" => CvssUserInteraction::UserInteractionUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CvssUserInteraction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CvssUserInteraction {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CvssUserInteraction {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "USER_INTERACTION_NONE" => CvssUserInteraction::UserInteractionNone,
                "USER_INTERACTION_REQUIRED" => CvssUserInteraction::UserInteractionRequired,
                "USER_INTERACTION_UNSPECIFIED" => CvssUserInteraction::UserInteractionUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CvssUserInteraction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CvssUserInteraction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Cvssv3 {
        #[serde(
            rename = "attackComplexity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attack_complexity: ::std::option::Option<crate::schemas::Cvssv3AttackComplexity>,
        #[doc = "Base Metrics Represents the intrinsic characteristics of a vulnerability that are constant over time and across user environments."]
        #[serde(
            rename = "attackVector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attack_vector: ::std::option::Option<crate::schemas::Cvssv3AttackVector>,
        #[serde(
            rename = "availabilityImpact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub availability_impact: ::std::option::Option<crate::schemas::Cvssv3AvailabilityImpact>,
        #[doc = "The base score is a function of the base metric scores."]
        #[serde(
            rename = "baseScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub base_score: ::std::option::Option<f32>,
        #[serde(
            rename = "confidentialityImpact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidentiality_impact:
            ::std::option::Option<crate::schemas::Cvssv3ConfidentialityImpact>,
        #[serde(
            rename = "exploitabilityScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exploitability_score: ::std::option::Option<f32>,
        #[serde(
            rename = "impactScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub impact_score: ::std::option::Option<f32>,
        #[serde(
            rename = "integrityImpact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub integrity_impact: ::std::option::Option<crate::schemas::Cvssv3IntegrityImpact>,
        #[serde(
            rename = "privilegesRequired",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub privileges_required: ::std::option::Option<crate::schemas::Cvssv3PrivilegesRequired>,
        #[serde(
            rename = "scope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scope: ::std::option::Option<crate::schemas::Cvssv3Scope>,
        #[serde(
            rename = "userInteraction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_interaction: ::std::option::Option<crate::schemas::Cvssv3UserInteraction>,
    }
    impl ::google_field_selector::FieldSelector for Cvssv3 {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3 {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3AttackComplexity {
        AttackComplexityHigh,
        AttackComplexityLow,
        AttackComplexityUnspecified,
    }
    impl Cvssv3AttackComplexity {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3AttackComplexity::AttackComplexityHigh => "ATTACK_COMPLEXITY_HIGH",
                Cvssv3AttackComplexity::AttackComplexityLow => "ATTACK_COMPLEXITY_LOW",
                Cvssv3AttackComplexity::AttackComplexityUnspecified => {
                    "ATTACK_COMPLEXITY_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3AttackComplexity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3AttackComplexity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3AttackComplexity, ()> {
            Ok(match s {
                "ATTACK_COMPLEXITY_HIGH" => Cvssv3AttackComplexity::AttackComplexityHigh,
                "ATTACK_COMPLEXITY_LOW" => Cvssv3AttackComplexity::AttackComplexityLow,
                "ATTACK_COMPLEXITY_UNSPECIFIED" => {
                    Cvssv3AttackComplexity::AttackComplexityUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3AttackComplexity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3AttackComplexity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3AttackComplexity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTACK_COMPLEXITY_HIGH" => Cvssv3AttackComplexity::AttackComplexityHigh,
                "ATTACK_COMPLEXITY_LOW" => Cvssv3AttackComplexity::AttackComplexityLow,
                "ATTACK_COMPLEXITY_UNSPECIFIED" => {
                    Cvssv3AttackComplexity::AttackComplexityUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Cvssv3AttackComplexity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3AttackComplexity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3AttackVector {
        AttackVectorAdjacent,
        AttackVectorLocal,
        AttackVectorNetwork,
        AttackVectorPhysical,
        AttackVectorUnspecified,
    }
    impl Cvssv3AttackVector {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3AttackVector::AttackVectorAdjacent => "ATTACK_VECTOR_ADJACENT",
                Cvssv3AttackVector::AttackVectorLocal => "ATTACK_VECTOR_LOCAL",
                Cvssv3AttackVector::AttackVectorNetwork => "ATTACK_VECTOR_NETWORK",
                Cvssv3AttackVector::AttackVectorPhysical => "ATTACK_VECTOR_PHYSICAL",
                Cvssv3AttackVector::AttackVectorUnspecified => "ATTACK_VECTOR_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3AttackVector {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3AttackVector {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3AttackVector, ()> {
            Ok(match s {
                "ATTACK_VECTOR_ADJACENT" => Cvssv3AttackVector::AttackVectorAdjacent,
                "ATTACK_VECTOR_LOCAL" => Cvssv3AttackVector::AttackVectorLocal,
                "ATTACK_VECTOR_NETWORK" => Cvssv3AttackVector::AttackVectorNetwork,
                "ATTACK_VECTOR_PHYSICAL" => Cvssv3AttackVector::AttackVectorPhysical,
                "ATTACK_VECTOR_UNSPECIFIED" => Cvssv3AttackVector::AttackVectorUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3AttackVector {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3AttackVector {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3AttackVector {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTACK_VECTOR_ADJACENT" => Cvssv3AttackVector::AttackVectorAdjacent,
                "ATTACK_VECTOR_LOCAL" => Cvssv3AttackVector::AttackVectorLocal,
                "ATTACK_VECTOR_NETWORK" => Cvssv3AttackVector::AttackVectorNetwork,
                "ATTACK_VECTOR_PHYSICAL" => Cvssv3AttackVector::AttackVectorPhysical,
                "ATTACK_VECTOR_UNSPECIFIED" => Cvssv3AttackVector::AttackVectorUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Cvssv3AttackVector {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3AttackVector {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3AvailabilityImpact {
        ImpactHigh,
        ImpactLow,
        ImpactNone,
        ImpactUnspecified,
    }
    impl Cvssv3AvailabilityImpact {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3AvailabilityImpact::ImpactHigh => "IMPACT_HIGH",
                Cvssv3AvailabilityImpact::ImpactLow => "IMPACT_LOW",
                Cvssv3AvailabilityImpact::ImpactNone => "IMPACT_NONE",
                Cvssv3AvailabilityImpact::ImpactUnspecified => "IMPACT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3AvailabilityImpact {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3AvailabilityImpact {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3AvailabilityImpact, ()> {
            Ok(match s {
                "IMPACT_HIGH" => Cvssv3AvailabilityImpact::ImpactHigh,
                "IMPACT_LOW" => Cvssv3AvailabilityImpact::ImpactLow,
                "IMPACT_NONE" => Cvssv3AvailabilityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => Cvssv3AvailabilityImpact::ImpactUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3AvailabilityImpact {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3AvailabilityImpact {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3AvailabilityImpact {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMPACT_HIGH" => Cvssv3AvailabilityImpact::ImpactHigh,
                "IMPACT_LOW" => Cvssv3AvailabilityImpact::ImpactLow,
                "IMPACT_NONE" => Cvssv3AvailabilityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => Cvssv3AvailabilityImpact::ImpactUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Cvssv3AvailabilityImpact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3AvailabilityImpact {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3ConfidentialityImpact {
        ImpactHigh,
        ImpactLow,
        ImpactNone,
        ImpactUnspecified,
    }
    impl Cvssv3ConfidentialityImpact {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3ConfidentialityImpact::ImpactHigh => "IMPACT_HIGH",
                Cvssv3ConfidentialityImpact::ImpactLow => "IMPACT_LOW",
                Cvssv3ConfidentialityImpact::ImpactNone => "IMPACT_NONE",
                Cvssv3ConfidentialityImpact::ImpactUnspecified => "IMPACT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3ConfidentialityImpact {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3ConfidentialityImpact {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3ConfidentialityImpact, ()> {
            Ok(match s {
                "IMPACT_HIGH" => Cvssv3ConfidentialityImpact::ImpactHigh,
                "IMPACT_LOW" => Cvssv3ConfidentialityImpact::ImpactLow,
                "IMPACT_NONE" => Cvssv3ConfidentialityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => Cvssv3ConfidentialityImpact::ImpactUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3ConfidentialityImpact {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3ConfidentialityImpact {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3ConfidentialityImpact {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMPACT_HIGH" => Cvssv3ConfidentialityImpact::ImpactHigh,
                "IMPACT_LOW" => Cvssv3ConfidentialityImpact::ImpactLow,
                "IMPACT_NONE" => Cvssv3ConfidentialityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => Cvssv3ConfidentialityImpact::ImpactUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Cvssv3ConfidentialityImpact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3ConfidentialityImpact {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3IntegrityImpact {
        ImpactHigh,
        ImpactLow,
        ImpactNone,
        ImpactUnspecified,
    }
    impl Cvssv3IntegrityImpact {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3IntegrityImpact::ImpactHigh => "IMPACT_HIGH",
                Cvssv3IntegrityImpact::ImpactLow => "IMPACT_LOW",
                Cvssv3IntegrityImpact::ImpactNone => "IMPACT_NONE",
                Cvssv3IntegrityImpact::ImpactUnspecified => "IMPACT_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3IntegrityImpact {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3IntegrityImpact {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3IntegrityImpact, ()> {
            Ok(match s {
                "IMPACT_HIGH" => Cvssv3IntegrityImpact::ImpactHigh,
                "IMPACT_LOW" => Cvssv3IntegrityImpact::ImpactLow,
                "IMPACT_NONE" => Cvssv3IntegrityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => Cvssv3IntegrityImpact::ImpactUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3IntegrityImpact {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3IntegrityImpact {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3IntegrityImpact {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMPACT_HIGH" => Cvssv3IntegrityImpact::ImpactHigh,
                "IMPACT_LOW" => Cvssv3IntegrityImpact::ImpactLow,
                "IMPACT_NONE" => Cvssv3IntegrityImpact::ImpactNone,
                "IMPACT_UNSPECIFIED" => Cvssv3IntegrityImpact::ImpactUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Cvssv3IntegrityImpact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3IntegrityImpact {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3PrivilegesRequired {
        PrivilegesRequiredHigh,
        PrivilegesRequiredLow,
        PrivilegesRequiredNone,
        PrivilegesRequiredUnspecified,
    }
    impl Cvssv3PrivilegesRequired {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3PrivilegesRequired::PrivilegesRequiredHigh => "PRIVILEGES_REQUIRED_HIGH",
                Cvssv3PrivilegesRequired::PrivilegesRequiredLow => "PRIVILEGES_REQUIRED_LOW",
                Cvssv3PrivilegesRequired::PrivilegesRequiredNone => "PRIVILEGES_REQUIRED_NONE",
                Cvssv3PrivilegesRequired::PrivilegesRequiredUnspecified => {
                    "PRIVILEGES_REQUIRED_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3PrivilegesRequired {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3PrivilegesRequired {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3PrivilegesRequired, ()> {
            Ok(match s {
                "PRIVILEGES_REQUIRED_HIGH" => Cvssv3PrivilegesRequired::PrivilegesRequiredHigh,
                "PRIVILEGES_REQUIRED_LOW" => Cvssv3PrivilegesRequired::PrivilegesRequiredLow,
                "PRIVILEGES_REQUIRED_NONE" => Cvssv3PrivilegesRequired::PrivilegesRequiredNone,
                "PRIVILEGES_REQUIRED_UNSPECIFIED" => {
                    Cvssv3PrivilegesRequired::PrivilegesRequiredUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3PrivilegesRequired {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3PrivilegesRequired {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3PrivilegesRequired {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PRIVILEGES_REQUIRED_HIGH" => Cvssv3PrivilegesRequired::PrivilegesRequiredHigh,
                "PRIVILEGES_REQUIRED_LOW" => Cvssv3PrivilegesRequired::PrivilegesRequiredLow,
                "PRIVILEGES_REQUIRED_NONE" => Cvssv3PrivilegesRequired::PrivilegesRequiredNone,
                "PRIVILEGES_REQUIRED_UNSPECIFIED" => {
                    Cvssv3PrivilegesRequired::PrivilegesRequiredUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Cvssv3PrivilegesRequired {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3PrivilegesRequired {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3Scope {
        ScopeChanged,
        ScopeUnchanged,
        ScopeUnspecified,
    }
    impl Cvssv3Scope {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3Scope::ScopeChanged => "SCOPE_CHANGED",
                Cvssv3Scope::ScopeUnchanged => "SCOPE_UNCHANGED",
                Cvssv3Scope::ScopeUnspecified => "SCOPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3Scope {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3Scope {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3Scope, ()> {
            Ok(match s {
                "SCOPE_CHANGED" => Cvssv3Scope::ScopeChanged,
                "SCOPE_UNCHANGED" => Cvssv3Scope::ScopeUnchanged,
                "SCOPE_UNSPECIFIED" => Cvssv3Scope::ScopeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3Scope {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3Scope {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3Scope {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SCOPE_CHANGED" => Cvssv3Scope::ScopeChanged,
                "SCOPE_UNCHANGED" => Cvssv3Scope::ScopeUnchanged,
                "SCOPE_UNSPECIFIED" => Cvssv3Scope::ScopeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Cvssv3Scope {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3Scope {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Cvssv3UserInteraction {
        UserInteractionNone,
        UserInteractionRequired,
        UserInteractionUnspecified,
    }
    impl Cvssv3UserInteraction {
        pub fn as_str(self) -> &'static str {
            match self {
                Cvssv3UserInteraction::UserInteractionNone => "USER_INTERACTION_NONE",
                Cvssv3UserInteraction::UserInteractionRequired => "USER_INTERACTION_REQUIRED",
                Cvssv3UserInteraction::UserInteractionUnspecified => "USER_INTERACTION_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Cvssv3UserInteraction {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Cvssv3UserInteraction {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Cvssv3UserInteraction, ()> {
            Ok(match s {
                "USER_INTERACTION_NONE" => Cvssv3UserInteraction::UserInteractionNone,
                "USER_INTERACTION_REQUIRED" => Cvssv3UserInteraction::UserInteractionRequired,
                "USER_INTERACTION_UNSPECIFIED" => Cvssv3UserInteraction::UserInteractionUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Cvssv3UserInteraction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Cvssv3UserInteraction {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Cvssv3UserInteraction {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "USER_INTERACTION_NONE" => Cvssv3UserInteraction::UserInteractionNone,
                "USER_INTERACTION_REQUIRED" => Cvssv3UserInteraction::UserInteractionRequired,
                "USER_INTERACTION_UNSPECIFIED" => Cvssv3UserInteraction::UserInteractionUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Cvssv3UserInteraction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cvssv3UserInteraction {
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
    pub struct DeploymentNote {
        #[doc = "Required. Resource URI for the artifact being deployed."]
        #[serde(
            rename = "resourceUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_uri: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for DeploymentNote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeploymentNote {
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
    pub struct DeploymentOccurrence {
        #[doc = "Address of the runtime element hosting this deployment."]
        #[serde(
            rename = "address",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address: ::std::option::Option<String>,
        #[doc = "Configuration used to create this deployment."]
        #[serde(
            rename = "config",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config: ::std::option::Option<String>,
        #[doc = "Required. Beginning of the lifetime of this deployment."]
        #[serde(
            rename = "deployTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deploy_time: ::std::option::Option<String>,
        #[doc = "Platform hosting this deployment."]
        #[serde(
            rename = "platform",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform: ::std::option::Option<crate::schemas::DeploymentOccurrencePlatform>,
        #[doc = "Output only. Resource URI for the artifact being deployed taken from the deployable field with the same name."]
        #[serde(
            rename = "resourceUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_uri: ::std::option::Option<Vec<String>>,
        #[doc = "End of the lifetime of this deployment."]
        #[serde(
            rename = "undeployTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub undeploy_time: ::std::option::Option<String>,
        #[doc = "Identity of the user that triggered this deployment."]
        #[serde(
            rename = "userEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_email: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeploymentOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeploymentOccurrence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeploymentOccurrencePlatform {
        #[doc = "Custom user-defined platform."]
        Custom,
        #[doc = "Google App Engine: Flexible Environment."]
        Flex,
        #[doc = "Google Container Engine."]
        Gke,
        #[doc = "Unknown."]
        PlatformUnspecified,
    }
    impl DeploymentOccurrencePlatform {
        pub fn as_str(self) -> &'static str {
            match self {
                DeploymentOccurrencePlatform::Custom => "CUSTOM",
                DeploymentOccurrencePlatform::Flex => "FLEX",
                DeploymentOccurrencePlatform::Gke => "GKE",
                DeploymentOccurrencePlatform::PlatformUnspecified => "PLATFORM_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeploymentOccurrencePlatform {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeploymentOccurrencePlatform {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeploymentOccurrencePlatform, ()> {
            Ok(match s {
                "CUSTOM" => DeploymentOccurrencePlatform::Custom,
                "FLEX" => DeploymentOccurrencePlatform::Flex,
                "GKE" => DeploymentOccurrencePlatform::Gke,
                "PLATFORM_UNSPECIFIED" => DeploymentOccurrencePlatform::PlatformUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeploymentOccurrencePlatform {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeploymentOccurrencePlatform {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeploymentOccurrencePlatform {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CUSTOM" => DeploymentOccurrencePlatform::Custom,
                "FLEX" => DeploymentOccurrencePlatform::Flex,
                "GKE" => DeploymentOccurrencePlatform::Gke,
                "PLATFORM_UNSPECIFIED" => DeploymentOccurrencePlatform::PlatformUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeploymentOccurrencePlatform {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeploymentOccurrencePlatform {
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
    pub struct Detail {
        #[doc = "Required. The [CPE URI](https://cpe.mitre.org/specification/) this vulnerability affects."]
        #[serde(
            rename = "affectedCpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub affected_cpe_uri: ::std::option::Option<String>,
        #[doc = "Required. The package this vulnerability affects."]
        #[serde(
            rename = "affectedPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub affected_package: ::std::option::Option<String>,
        #[doc = "The version number at the end of an interval in which this vulnerability exists. A vulnerability can affect a package between version numbers that are disjoint sets of intervals (example: [1.0.0-1.1.0], [2.4.6-2.4.8] and [4.5.6-4.6.8]) each of which will be represented in its own Detail. If a specific affected version is provided by a vulnerability database, affected_version_start and affected_version_end will be the same in that Detail."]
        #[serde(
            rename = "affectedVersionEnd",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub affected_version_end: ::std::option::Option<crate::schemas::Version>,
        #[doc = "The version number at the start of an interval in which this vulnerability exists. A vulnerability can affect a package between version numbers that are disjoint sets of intervals (example: [1.0.0-1.1.0], [2.4.6-2.4.8] and [4.5.6-4.6.8]) each of which will be represented in its own Detail. If a specific affected version is provided by a vulnerability database, affected_version_start and affected_version_end will be the same in that Detail."]
        #[serde(
            rename = "affectedVersionStart",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub affected_version_start: ::std::option::Option<crate::schemas::Version>,
        #[doc = "A vendor-specific description of this vulnerability."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The distro recommended [CPE URI](https://cpe.mitre.org/specification/) to update to that contains a fix for this vulnerability. It is possible for this to be different from the affected_cpe_uri."]
        #[serde(
            rename = "fixedCpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_cpe_uri: ::std::option::Option<String>,
        #[doc = "The distro recommended package to update to that contains a fix for this vulnerability. It is possible for this to be different from the affected_package."]
        #[serde(
            rename = "fixedPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_package: ::std::option::Option<String>,
        #[doc = "The distro recommended version to update to that contains a fix for this vulnerability. Setting this to VersionKind.MAXIMUM means no such version is yet available."]
        #[serde(
            rename = "fixedVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_version: ::std::option::Option<crate::schemas::Version>,
        #[doc = "Whether this detail is obsolete. Occurrences are expected not to point to obsolete details."]
        #[serde(
            rename = "isObsolete",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_obsolete: ::std::option::Option<bool>,
        #[doc = "The type of package; whether native or non native (e.g., ruby gems, node.js packages, etc.)."]
        #[serde(
            rename = "packageType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_type: ::std::option::Option<String>,
        #[doc = "The distro assigned severity of this vulnerability."]
        #[serde(
            rename = "severityName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity_name: ::std::option::Option<String>,
        #[doc = "The source from which the information in this Detail was obtained."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<String>,
        #[doc = "The time this information was last changed at the source. This is an upstream timestamp from the underlying information source - e.g. Ubuntu security tracker."]
        #[serde(
            rename = "sourceUpdateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_update_time: ::std::option::Option<String>,
        #[doc = "The name of the vendor of the product."]
        #[serde(
            rename = "vendor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vendor: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Detail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Detail {
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
    pub struct DiscoveryNote {
        #[doc = "Required. Immutable. The kind of analysis that is handled by this discovery."]
        #[serde(
            rename = "analysisKind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_kind: ::std::option::Option<crate::schemas::DiscoveryNoteAnalysisKind>,
    }
    impl ::google_field_selector::FieldSelector for DiscoveryNote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiscoveryNote {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DiscoveryNoteAnalysisKind {
        #[doc = "This represents a logical \"role\" that can attest to artifacts."]
        Attestation,
        #[doc = "The note and occurrence assert build provenance."]
        Build,
        #[doc = "This represents a Compliance Note"]
        Compliance,
        #[doc = "The note and occurrence track deployment events."]
        Deployment,
        #[doc = "The note and occurrence track the initial discovery status of a resource."]
        Discovery,
        #[doc = "This represents a DSSE attestation Note"]
        DsseAttestation,
        #[doc = "This represents an image basis relationship."]
        Image,
        #[doc = "Default value. This value is unused."]
        NoteKindUnspecified,
        #[doc = "This represents a package installed via a package manager."]
        Package,
        #[doc = "This represents an available package upgrade."]
        Upgrade,
        #[doc = "The note and occurrence represent a package vulnerability."]
        Vulnerability,
    }
    impl DiscoveryNoteAnalysisKind {
        pub fn as_str(self) -> &'static str {
            match self {
                DiscoveryNoteAnalysisKind::Attestation => "ATTESTATION",
                DiscoveryNoteAnalysisKind::Build => "BUILD",
                DiscoveryNoteAnalysisKind::Compliance => "COMPLIANCE",
                DiscoveryNoteAnalysisKind::Deployment => "DEPLOYMENT",
                DiscoveryNoteAnalysisKind::Discovery => "DISCOVERY",
                DiscoveryNoteAnalysisKind::DsseAttestation => "DSSE_ATTESTATION",
                DiscoveryNoteAnalysisKind::Image => "IMAGE",
                DiscoveryNoteAnalysisKind::NoteKindUnspecified => "NOTE_KIND_UNSPECIFIED",
                DiscoveryNoteAnalysisKind::Package => "PACKAGE",
                DiscoveryNoteAnalysisKind::Upgrade => "UPGRADE",
                DiscoveryNoteAnalysisKind::Vulnerability => "VULNERABILITY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DiscoveryNoteAnalysisKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DiscoveryNoteAnalysisKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DiscoveryNoteAnalysisKind, ()> {
            Ok(match s {
                "ATTESTATION" => DiscoveryNoteAnalysisKind::Attestation,
                "BUILD" => DiscoveryNoteAnalysisKind::Build,
                "COMPLIANCE" => DiscoveryNoteAnalysisKind::Compliance,
                "DEPLOYMENT" => DiscoveryNoteAnalysisKind::Deployment,
                "DISCOVERY" => DiscoveryNoteAnalysisKind::Discovery,
                "DSSE_ATTESTATION" => DiscoveryNoteAnalysisKind::DsseAttestation,
                "IMAGE" => DiscoveryNoteAnalysisKind::Image,
                "NOTE_KIND_UNSPECIFIED" => DiscoveryNoteAnalysisKind::NoteKindUnspecified,
                "PACKAGE" => DiscoveryNoteAnalysisKind::Package,
                "UPGRADE" => DiscoveryNoteAnalysisKind::Upgrade,
                "VULNERABILITY" => DiscoveryNoteAnalysisKind::Vulnerability,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DiscoveryNoteAnalysisKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DiscoveryNoteAnalysisKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DiscoveryNoteAnalysisKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTESTATION" => DiscoveryNoteAnalysisKind::Attestation,
                "BUILD" => DiscoveryNoteAnalysisKind::Build,
                "COMPLIANCE" => DiscoveryNoteAnalysisKind::Compliance,
                "DEPLOYMENT" => DiscoveryNoteAnalysisKind::Deployment,
                "DISCOVERY" => DiscoveryNoteAnalysisKind::Discovery,
                "DSSE_ATTESTATION" => DiscoveryNoteAnalysisKind::DsseAttestation,
                "IMAGE" => DiscoveryNoteAnalysisKind::Image,
                "NOTE_KIND_UNSPECIFIED" => DiscoveryNoteAnalysisKind::NoteKindUnspecified,
                "PACKAGE" => DiscoveryNoteAnalysisKind::Package,
                "UPGRADE" => DiscoveryNoteAnalysisKind::Upgrade,
                "VULNERABILITY" => DiscoveryNoteAnalysisKind::Vulnerability,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DiscoveryNoteAnalysisKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiscoveryNoteAnalysisKind {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DiscoveryOccurrence {
        #[doc = "The status of discovery for the resource."]
        #[serde(
            rename = "analysisStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_status:
            ::std::option::Option<crate::schemas::DiscoveryOccurrenceAnalysisStatus>,
        #[doc = "When an error is encountered this will contain a LocalizedMessage under details to show to the user. The LocalizedMessage is output only and populated by the API."]
        #[serde(
            rename = "analysisStatusError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_status_error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "Output only. The time occurrences related to this discovery occurrence were archived."]
        #[serde(
            rename = "archiveTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub archive_time: ::std::option::Option<String>,
        #[doc = "Whether the resource is continuously analyzed."]
        #[serde(
            rename = "continuousAnalysis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub continuous_analysis:
            ::std::option::Option<crate::schemas::DiscoveryOccurrenceContinuousAnalysis>,
        #[doc = "The CPE of the resource being scanned."]
        #[serde(
            rename = "cpe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpe: ::std::option::Option<String>,
        #[doc = "The last time this resource was scanned."]
        #[serde(
            rename = "lastScanTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_scan_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DiscoveryOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiscoveryOccurrence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DiscoveryOccurrenceAnalysisStatus {
        #[doc = "Unknown."]
        AnalysisStatusUnspecified,
        #[doc = "Analysis has finished unsuccessfully, the analysis itself is in a bad state."]
        FinishedFailed,
        #[doc = "Analysis has finished successfully."]
        FinishedSuccess,
        #[doc = "The resource is known not to be supported"]
        FinishedUnsupported,
        #[doc = "Resource is known but no action has been taken yet."]
        Pending,
        #[doc = "Resource is being analyzed."]
        Scanning,
    }
    impl DiscoveryOccurrenceAnalysisStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                DiscoveryOccurrenceAnalysisStatus::AnalysisStatusUnspecified => {
                    "ANALYSIS_STATUS_UNSPECIFIED"
                }
                DiscoveryOccurrenceAnalysisStatus::FinishedFailed => "FINISHED_FAILED",
                DiscoveryOccurrenceAnalysisStatus::FinishedSuccess => "FINISHED_SUCCESS",
                DiscoveryOccurrenceAnalysisStatus::FinishedUnsupported => "FINISHED_UNSUPPORTED",
                DiscoveryOccurrenceAnalysisStatus::Pending => "PENDING",
                DiscoveryOccurrenceAnalysisStatus::Scanning => "SCANNING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DiscoveryOccurrenceAnalysisStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DiscoveryOccurrenceAnalysisStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DiscoveryOccurrenceAnalysisStatus, ()> {
            Ok(match s {
                "ANALYSIS_STATUS_UNSPECIFIED" => {
                    DiscoveryOccurrenceAnalysisStatus::AnalysisStatusUnspecified
                }
                "FINISHED_FAILED" => DiscoveryOccurrenceAnalysisStatus::FinishedFailed,
                "FINISHED_SUCCESS" => DiscoveryOccurrenceAnalysisStatus::FinishedSuccess,
                "FINISHED_UNSUPPORTED" => DiscoveryOccurrenceAnalysisStatus::FinishedUnsupported,
                "PENDING" => DiscoveryOccurrenceAnalysisStatus::Pending,
                "SCANNING" => DiscoveryOccurrenceAnalysisStatus::Scanning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DiscoveryOccurrenceAnalysisStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DiscoveryOccurrenceAnalysisStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DiscoveryOccurrenceAnalysisStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANALYSIS_STATUS_UNSPECIFIED" => {
                    DiscoveryOccurrenceAnalysisStatus::AnalysisStatusUnspecified
                }
                "FINISHED_FAILED" => DiscoveryOccurrenceAnalysisStatus::FinishedFailed,
                "FINISHED_SUCCESS" => DiscoveryOccurrenceAnalysisStatus::FinishedSuccess,
                "FINISHED_UNSUPPORTED" => DiscoveryOccurrenceAnalysisStatus::FinishedUnsupported,
                "PENDING" => DiscoveryOccurrenceAnalysisStatus::Pending,
                "SCANNING" => DiscoveryOccurrenceAnalysisStatus::Scanning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DiscoveryOccurrenceAnalysisStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiscoveryOccurrenceAnalysisStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DiscoveryOccurrenceContinuousAnalysis {
        #[doc = "The resource is continuously analyzed."]
        Active,
        #[doc = "Unknown."]
        ContinuousAnalysisUnspecified,
        #[doc = "The resource is ignored for continuous analysis."]
        Inactive,
    }
    impl DiscoveryOccurrenceContinuousAnalysis {
        pub fn as_str(self) -> &'static str {
            match self {
                DiscoveryOccurrenceContinuousAnalysis::Active => "ACTIVE",
                DiscoveryOccurrenceContinuousAnalysis::ContinuousAnalysisUnspecified => {
                    "CONTINUOUS_ANALYSIS_UNSPECIFIED"
                }
                DiscoveryOccurrenceContinuousAnalysis::Inactive => "INACTIVE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DiscoveryOccurrenceContinuousAnalysis {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DiscoveryOccurrenceContinuousAnalysis {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DiscoveryOccurrenceContinuousAnalysis, ()> {
            Ok(match s {
                "ACTIVE" => DiscoveryOccurrenceContinuousAnalysis::Active,
                "CONTINUOUS_ANALYSIS_UNSPECIFIED" => {
                    DiscoveryOccurrenceContinuousAnalysis::ContinuousAnalysisUnspecified
                }
                "INACTIVE" => DiscoveryOccurrenceContinuousAnalysis::Inactive,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DiscoveryOccurrenceContinuousAnalysis {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DiscoveryOccurrenceContinuousAnalysis {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DiscoveryOccurrenceContinuousAnalysis {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => DiscoveryOccurrenceContinuousAnalysis::Active,
                "CONTINUOUS_ANALYSIS_UNSPECIFIED" => {
                    DiscoveryOccurrenceContinuousAnalysis::ContinuousAnalysisUnspecified
                }
                "INACTIVE" => DiscoveryOccurrenceContinuousAnalysis::Inactive,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DiscoveryOccurrenceContinuousAnalysis {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiscoveryOccurrenceContinuousAnalysis {
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
    pub struct Distribution {
        #[doc = "The CPU architecture for which packages in this distribution channel were built."]
        #[serde(
            rename = "architecture",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub architecture: ::std::option::Option<crate::schemas::DistributionArchitecture>,
        #[doc = "Required. The cpe_uri in [CPE format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package."]
        #[serde(
            rename = "cpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpe_uri: ::std::option::Option<String>,
        #[doc = "The distribution channel-specific description of this package."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The latest available version of this package in this distribution channel."]
        #[serde(
            rename = "latestVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub latest_version: ::std::option::Option<crate::schemas::Version>,
        #[doc = "A freeform string denoting the maintainer of this package."]
        #[serde(
            rename = "maintainer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maintainer: ::std::option::Option<String>,
        #[doc = "The distribution channel-specific homepage for this package."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Distribution {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Distribution {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DistributionArchitecture {
        #[doc = "Unknown architecture."]
        ArchitectureUnspecified,
        #[doc = "X64 architecture."]
        X64,
        #[doc = "X86 architecture."]
        X86,
    }
    impl DistributionArchitecture {
        pub fn as_str(self) -> &'static str {
            match self {
                DistributionArchitecture::ArchitectureUnspecified => "ARCHITECTURE_UNSPECIFIED",
                DistributionArchitecture::X64 => "X64",
                DistributionArchitecture::X86 => "X86",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DistributionArchitecture {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DistributionArchitecture {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DistributionArchitecture, ()> {
            Ok(match s {
                "ARCHITECTURE_UNSPECIFIED" => DistributionArchitecture::ArchitectureUnspecified,
                "X64" => DistributionArchitecture::X64,
                "X86" => DistributionArchitecture::X86,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DistributionArchitecture {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DistributionArchitecture {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DistributionArchitecture {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARCHITECTURE_UNSPECIFIED" => DistributionArchitecture::ArchitectureUnspecified,
                "X64" => DistributionArchitecture::X64,
                "X86" => DistributionArchitecture::X86,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DistributionArchitecture {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DistributionArchitecture {
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
    pub struct DsseattestationNote {
        #[doc = "DSSEHint hints at the purpose of the attestation authority."]
        #[serde(
            rename = "hint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hint: ::std::option::Option<crate::schemas::Dssehint>,
    }
    impl ::google_field_selector::FieldSelector for DsseattestationNote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DsseattestationNote {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DsseattestationOccurrence {
        #[doc = "If doing something security critical, make sure to verify the signatures in this metadata."]
        #[serde(
            rename = "envelope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub envelope: ::std::option::Option<crate::schemas::Envelope>,
        #[serde(
            rename = "statement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub statement: ::std::option::Option<crate::schemas::InTotoStatement>,
    }
    impl ::google_field_selector::FieldSelector for DsseattestationOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DsseattestationOccurrence {
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
    pub struct Dssehint {
        #[doc = "Required. The human readable name of this attestation authority, for example \"cloudbuild-prod\"."]
        #[serde(
            rename = "humanReadableName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub human_readable_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Dssehint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Dssehint {
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
    pub struct Empty {}
    impl ::google_field_selector::FieldSelector for Empty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Empty {
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
    pub struct Envelope {
        #[serde(
            rename = "payload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub payload: ::std::option::Option<::google_api_bytes::Bytes>,
        #[serde(
            rename = "payloadType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub payload_type: ::std::option::Option<String>,
        #[serde(
            rename = "signatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signatures: ::std::option::Option<Vec<crate::schemas::EnvelopeSignature>>,
    }
    impl ::google_field_selector::FieldSelector for Envelope {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Envelope {
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
    pub struct EnvelopeSignature {
        #[serde(
            rename = "keyid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keyid: ::std::option::Option<String>,
        #[serde(
            rename = "sig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sig: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for EnvelopeSignature {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnvelopeSignature {
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
    pub struct Expr {
        #[doc = "Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Textual representation of an expression in Common Expression Language syntax."]
        #[serde(
            rename = "expression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expression: ::std::option::Option<String>,
        #[doc = "Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Expr {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Expr {
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
    pub struct FileHashes {
        #[doc = "Required. Collection of file hashes."]
        #[serde(
            rename = "fileHash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_hash: ::std::option::Option<Vec<crate::schemas::Hash>>,
    }
    impl ::google_field_selector::FieldSelector for FileHashes {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FileHashes {
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
    pub struct Fingerprint {
        #[doc = "Required. The layer ID of the final layer in the Docker image's v1 representation."]
        #[serde(
            rename = "v1Name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub v_1_name: ::std::option::Option<String>,
        #[doc = "Required. The ordered list of v2 blobs that represent a given image."]
        #[serde(
            rename = "v2Blob",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub v_2_blob: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. The name of the image's v2 blobs computed via: [bottom] := v2_blobbottom := sha256(v2_blob[N] + \" \" + v2_name[N+1]) Only the name of the final blob is kept."]
        #[serde(
            rename = "v2Name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub v_2_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Fingerprint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Fingerprint {
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
    pub struct FixableTotalByDigest {
        #[doc = "The number of fixable vulnerabilities associated with this resource."]
        #[serde(
            rename = "fixableCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub fixable_count: ::std::option::Option<i64>,
        #[doc = "The affected resource."]
        #[serde(
            rename = "resourceUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_uri: ::std::option::Option<String>,
        #[doc = "The severity for this count. SEVERITY_UNSPECIFIED indicates total across all severities."]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<crate::schemas::FixableTotalByDigestSeverity>,
        #[doc = "The total number of vulnerabilities associated with this resource."]
        #[serde(
            rename = "totalCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for FixableTotalByDigest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FixableTotalByDigest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FixableTotalByDigestSeverity {
        #[doc = "Critical severity."]
        Critical,
        #[doc = "High severity."]
        High,
        #[doc = "Low severity."]
        Low,
        #[doc = "Medium severity."]
        Medium,
        #[doc = "Minimal severity."]
        Minimal,
        #[doc = "Unknown."]
        SeverityUnspecified,
    }
    impl FixableTotalByDigestSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                FixableTotalByDigestSeverity::Critical => "CRITICAL",
                FixableTotalByDigestSeverity::High => "HIGH",
                FixableTotalByDigestSeverity::Low => "LOW",
                FixableTotalByDigestSeverity::Medium => "MEDIUM",
                FixableTotalByDigestSeverity::Minimal => "MINIMAL",
                FixableTotalByDigestSeverity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FixableTotalByDigestSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FixableTotalByDigestSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FixableTotalByDigestSeverity, ()> {
            Ok(match s {
                "CRITICAL" => FixableTotalByDigestSeverity::Critical,
                "HIGH" => FixableTotalByDigestSeverity::High,
                "LOW" => FixableTotalByDigestSeverity::Low,
                "MEDIUM" => FixableTotalByDigestSeverity::Medium,
                "MINIMAL" => FixableTotalByDigestSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => FixableTotalByDigestSeverity::SeverityUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FixableTotalByDigestSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FixableTotalByDigestSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FixableTotalByDigestSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRITICAL" => FixableTotalByDigestSeverity::Critical,
                "HIGH" => FixableTotalByDigestSeverity::High,
                "LOW" => FixableTotalByDigestSeverity::Low,
                "MEDIUM" => FixableTotalByDigestSeverity::Medium,
                "MINIMAL" => FixableTotalByDigestSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => FixableTotalByDigestSeverity::SeverityUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FixableTotalByDigestSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FixableTotalByDigestSeverity {
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
    pub struct GerritSourceContext {
        #[doc = "An alias, which may be a branch or tag."]
        #[serde(
            rename = "aliasContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alias_context: ::std::option::Option<crate::schemas::AliasContext>,
        #[doc = "The full project name within the host. Projects may be nested, so \"project/subproject\" is a valid project name. The \"repo name\" is the hostURI/project."]
        #[serde(
            rename = "gerritProject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gerrit_project: ::std::option::Option<String>,
        #[doc = "The URI of a running Gerrit instance."]
        #[serde(
            rename = "hostUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub host_uri: ::std::option::Option<String>,
        #[doc = "A revision (commit) ID."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GerritSourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GerritSourceContext {
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
    pub struct GetIamPolicyRequest {
        #[doc = "OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<crate::schemas::GetPolicyOptions>,
    }
    impl ::google_field_selector::FieldSelector for GetIamPolicyRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetIamPolicyRequest {
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
    pub struct GetPolicyOptions {
        #[doc = "Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "requestedPolicyVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requested_policy_version: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GetPolicyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetPolicyOptions {
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
    pub struct GitSourceContext {
        #[doc = "Git commit hash."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
        #[doc = "Git repository URL."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GitSourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GitSourceContext {
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
    pub struct GoogleDevtoolsContaineranalysisV1Alpha1OperationMetadata {
        #[doc = "Output only. The time this operation was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. The time that this operation was marked completed or failed."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleDevtoolsContaineranalysisV1Alpha1OperationMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleDevtoolsContaineranalysisV1Alpha1OperationMetadata
    {
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
    pub struct Hash {
        #[doc = "Required. The type of hash that was performed, e.g. \"SHA-256\"."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Required. The hash value."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for Hash {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Hash {
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
    pub struct Hint {
        #[doc = "Required. The human readable name of this attestation authority, for example \"qa\"."]
        #[serde(
            rename = "humanReadableName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub human_readable_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Hint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Hint {
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
    pub struct Identity {
        #[doc = "The revision number of the update."]
        #[serde(
            rename = "revision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision: ::std::option::Option<i32>,
        #[doc = "The revision independent identifier of the update."]
        #[serde(
            rename = "updateId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Identity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Identity {
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
    pub struct ImageNote {
        #[doc = "Required. Immutable. The fingerprint of the base image."]
        #[serde(
            rename = "fingerprint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fingerprint: ::std::option::Option<crate::schemas::Fingerprint>,
        #[doc = "Required. Immutable. The resource_url for the resource representing the basis of associated occurrence images."]
        #[serde(
            rename = "resourceUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ImageNote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImageNote {
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
    pub struct ImageOccurrence {
        #[doc = "Output only. This contains the base image URL for the derived image occurrence."]
        #[serde(
            rename = "baseResourceUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub base_resource_url: ::std::option::Option<String>,
        #[doc = "Output only. The number of layers by which this image differs from the associated image basis."]
        #[serde(
            rename = "distance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distance: ::std::option::Option<i32>,
        #[doc = "Required. The fingerprint of the derived image."]
        #[serde(
            rename = "fingerprint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fingerprint: ::std::option::Option<crate::schemas::Fingerprint>,
        #[doc = "This contains layer-specific metadata, if populated it has length \"distance\" and is ordered with [distance] being the layer immediately following the base image and [1] being the final layer."]
        #[serde(
            rename = "layerInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layer_info: ::std::option::Option<Vec<crate::schemas::Layer>>,
    }
    impl ::google_field_selector::FieldSelector for ImageOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImageOccurrence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct InTotoProvenance {
        #[doc = "required"]
        #[serde(
            rename = "builderConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub builder_config: ::std::option::Option<crate::schemas::BuilderConfig>,
        #[doc = "The collection of artifacts that influenced the build including sources, dependencies, build tools, base images, and so on. This is considered to be incomplete unless metadata.completeness.materials is true. Unset or null is equivalent to empty."]
        #[serde(
            rename = "materials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub materials: ::std::option::Option<Vec<String>>,
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::Metadata>,
        #[doc = "Identifies the configuration used for the build. When combined with materials, this SHOULD fully describe the build, such that re-running this recipe results in bit-for-bit identical output (if the build is reproducible). required"]
        #[serde(
            rename = "recipe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recipe: ::std::option::Option<crate::schemas::Recipe>,
    }
    impl ::google_field_selector::FieldSelector for InTotoProvenance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InTotoProvenance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct InTotoStatement {
        #[doc = "`https://slsa.dev/provenance/v0.1` for SlsaProvenance."]
        #[serde(
            rename = "predicateType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub predicate_type: ::std::option::Option<String>,
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance: ::std::option::Option<crate::schemas::InTotoProvenance>,
        #[doc = "Always `https://in-toto.io/Statement/v0.1`."]
        #[serde(
            rename = "_type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[serde(
            rename = "slsaProvenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slsa_provenance: ::std::option::Option<crate::schemas::SlsaProvenance>,
        #[serde(
            rename = "subject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subject: ::std::option::Option<Vec<crate::schemas::Subject>>,
    }
    impl ::google_field_selector::FieldSelector for InTotoStatement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InTotoStatement {
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
    pub struct Jwt {
        #[doc = "The compact encoding of a JWS, which is always three base64 encoded strings joined by periods. For details, see: https://tools.ietf.org/html/rfc7515.html#section-3.1"]
        #[serde(
            rename = "compactJwt",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compact_jwt: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Jwt {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Jwt {
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
    pub struct KnowledgeBase {
        #[doc = "The KB name (generally of the form KB[0-9]+ (e.g., KB123456))."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A link to the KB in the [Windows update catalog] (https://www.catalog.update.microsoft.com/)."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for KnowledgeBase {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KnowledgeBase {
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
    pub struct Layer {
        #[doc = "The recovered arguments to the Dockerfile directive."]
        #[serde(
            rename = "arguments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arguments: ::std::option::Option<String>,
        #[doc = "Required. The recovered Dockerfile directive used to construct this layer. See https://docs.docker.com/engine/reference/builder/ for more information."]
        #[serde(
            rename = "directive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub directive: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Layer {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Layer {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListNoteOccurrencesResponse {
        #[doc = "Token to provide to skip to a particular spot in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The occurrences attached to the specified note."]
        #[serde(
            rename = "occurrences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub occurrences: ::std::option::Option<Vec<crate::schemas::Occurrence>>,
    }
    impl ::google_field_selector::FieldSelector for ListNoteOccurrencesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListNoteOccurrencesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListNotesResponse {
        #[doc = "The next pagination token in the list response. It should be used as `page_token` for the following request. An empty value means no more results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The notes requested."]
        #[serde(
            rename = "notes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub notes: ::std::option::Option<Vec<crate::schemas::Note>>,
    }
    impl ::google_field_selector::FieldSelector for ListNotesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListNotesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListOccurrencesResponse {
        #[doc = "The next pagination token in the list response. It should be used as `page_token` for the following request. An empty value means no more results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The occurrences requested."]
        #[serde(
            rename = "occurrences",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub occurrences: ::std::option::Option<Vec<crate::schemas::Occurrence>>,
    }
    impl ::google_field_selector::FieldSelector for ListOccurrencesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListOccurrencesResponse {
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
    pub struct Location {
        #[doc = "Required. The CPE URI in [CPE format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package."]
        #[serde(
            rename = "cpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpe_uri: ::std::option::Option<String>,
        #[doc = "The path from which we gathered that this package/version is installed."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
        #[doc = "The version installed at this location."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<crate::schemas::Version>,
    }
    impl ::google_field_selector::FieldSelector for Location {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Location {
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
    pub struct Material {
        #[serde(
            rename = "digest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub digest: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Material {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Material {
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
    pub struct Metadata {
        #[doc = "The timestamp of when the build completed."]
        #[serde(
            rename = "buildFinishedOn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_finished_on: ::std::option::Option<String>,
        #[doc = "Identifies the particular build invocation, which can be useful for finding associated logs or other ad-hoc analysis. The value SHOULD be globally unique, per in-toto Provenance spec."]
        #[serde(
            rename = "buildInvocationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_invocation_id: ::std::option::Option<String>,
        #[doc = "The timestamp of when the build started."]
        #[serde(
            rename = "buildStartedOn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_started_on: ::std::option::Option<String>,
        #[doc = "Indicates that the builder claims certain fields in this message to be complete."]
        #[serde(
            rename = "completeness",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub completeness: ::std::option::Option<crate::schemas::Completeness>,
        #[doc = "If true, the builder claims that running the recipe on materials will produce bit-for-bit identical output."]
        #[serde(
            rename = "reproducible",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reproducible: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for Metadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Metadata {
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
    pub struct NonCompliantFile {
        #[doc = "Command to display the non-compliant files."]
        #[serde(
            rename = "displayCommand",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_command: ::std::option::Option<String>,
        #[doc = "Empty if `display_command` is set."]
        #[serde(
            rename = "path",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path: ::std::option::Option<String>,
        #[doc = "Explains why a file is non compliant for a CIS check."]
        #[serde(
            rename = "reason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reason: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for NonCompliantFile {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NonCompliantFile {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Note {
        #[doc = "A note describing an attestation role."]
        #[serde(
            rename = "attestation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attestation: ::std::option::Option<crate::schemas::AttestationNote>,
        #[doc = "A note describing build provenance for a verifiable build."]
        #[serde(
            rename = "build",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build: ::std::option::Option<crate::schemas::BuildNote>,
        #[doc = "A note describing a compliance check."]
        #[serde(
            rename = "compliance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compliance: ::std::option::Option<crate::schemas::ComplianceNote>,
        #[doc = "Output only. The time this note was created. This field can be used as a filter in list requests."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "A note describing something that can be deployed."]
        #[serde(
            rename = "deployment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deployment: ::std::option::Option<crate::schemas::DeploymentNote>,
        #[doc = "A note describing the initial analysis of a resource."]
        #[serde(
            rename = "discovery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discovery: ::std::option::Option<crate::schemas::DiscoveryNote>,
        #[doc = "A note describing a dsse attestation note."]
        #[serde(
            rename = "dsseAttestation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dsse_attestation: ::std::option::Option<crate::schemas::DsseattestationNote>,
        #[doc = "Time of expiration for this note. Empty if note does not expire."]
        #[serde(
            rename = "expirationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expiration_time: ::std::option::Option<String>,
        #[doc = "A note describing a base image."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::ImageNote>,
        #[doc = "Output only. The type of analysis. This field can be used as a filter in list requests."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::NoteKind>,
        #[doc = "A detailed description of this note."]
        #[serde(
            rename = "longDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub long_description: ::std::option::Option<String>,
        #[doc = "Output only. The name of the note in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A note describing a package hosted by various package managers."]
        #[serde(
            rename = "package",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package: ::std::option::Option<crate::schemas::PackageNote>,
        #[doc = "Other notes related to this note."]
        #[serde(
            rename = "relatedNoteNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_note_names: ::std::option::Option<Vec<String>>,
        #[doc = "URLs associated with this note."]
        #[serde(
            rename = "relatedUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_url: ::std::option::Option<Vec<crate::schemas::RelatedUrl>>,
        #[doc = "A one sentence description of this note."]
        #[serde(
            rename = "shortDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub short_description: ::std::option::Option<String>,
        #[doc = "Output only. The time this note was last updated. This field can be used as a filter in list requests."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
        #[doc = "A note describing available package upgrades."]
        #[serde(
            rename = "upgrade",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upgrade: ::std::option::Option<crate::schemas::UpgradeNote>,
        #[doc = "A note describing a package vulnerability."]
        #[serde(
            rename = "vulnerability",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vulnerability: ::std::option::Option<crate::schemas::VulnerabilityNote>,
    }
    impl ::google_field_selector::FieldSelector for Note {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Note {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NoteKind {
        #[doc = "This represents a logical \"role\" that can attest to artifacts."]
        Attestation,
        #[doc = "The note and occurrence assert build provenance."]
        Build,
        #[doc = "This represents a Compliance Note"]
        Compliance,
        #[doc = "The note and occurrence track deployment events."]
        Deployment,
        #[doc = "The note and occurrence track the initial discovery status of a resource."]
        Discovery,
        #[doc = "This represents a DSSE attestation Note"]
        DsseAttestation,
        #[doc = "This represents an image basis relationship."]
        Image,
        #[doc = "Default value. This value is unused."]
        NoteKindUnspecified,
        #[doc = "This represents a package installed via a package manager."]
        Package,
        #[doc = "This represents an available package upgrade."]
        Upgrade,
        #[doc = "The note and occurrence represent a package vulnerability."]
        Vulnerability,
    }
    impl NoteKind {
        pub fn as_str(self) -> &'static str {
            match self {
                NoteKind::Attestation => "ATTESTATION",
                NoteKind::Build => "BUILD",
                NoteKind::Compliance => "COMPLIANCE",
                NoteKind::Deployment => "DEPLOYMENT",
                NoteKind::Discovery => "DISCOVERY",
                NoteKind::DsseAttestation => "DSSE_ATTESTATION",
                NoteKind::Image => "IMAGE",
                NoteKind::NoteKindUnspecified => "NOTE_KIND_UNSPECIFIED",
                NoteKind::Package => "PACKAGE",
                NoteKind::Upgrade => "UPGRADE",
                NoteKind::Vulnerability => "VULNERABILITY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NoteKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NoteKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<NoteKind, ()> {
            Ok(match s {
                "ATTESTATION" => NoteKind::Attestation,
                "BUILD" => NoteKind::Build,
                "COMPLIANCE" => NoteKind::Compliance,
                "DEPLOYMENT" => NoteKind::Deployment,
                "DISCOVERY" => NoteKind::Discovery,
                "DSSE_ATTESTATION" => NoteKind::DsseAttestation,
                "IMAGE" => NoteKind::Image,
                "NOTE_KIND_UNSPECIFIED" => NoteKind::NoteKindUnspecified,
                "PACKAGE" => NoteKind::Package,
                "UPGRADE" => NoteKind::Upgrade,
                "VULNERABILITY" => NoteKind::Vulnerability,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NoteKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NoteKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NoteKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTESTATION" => NoteKind::Attestation,
                "BUILD" => NoteKind::Build,
                "COMPLIANCE" => NoteKind::Compliance,
                "DEPLOYMENT" => NoteKind::Deployment,
                "DISCOVERY" => NoteKind::Discovery,
                "DSSE_ATTESTATION" => NoteKind::DsseAttestation,
                "IMAGE" => NoteKind::Image,
                "NOTE_KIND_UNSPECIFIED" => NoteKind::NoteKindUnspecified,
                "PACKAGE" => NoteKind::Package,
                "UPGRADE" => NoteKind::Upgrade,
                "VULNERABILITY" => NoteKind::Vulnerability,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NoteKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NoteKind {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Occurrence {
        #[doc = "Describes an attestation of an artifact."]
        #[serde(
            rename = "attestation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attestation: ::std::option::Option<crate::schemas::AttestationOccurrence>,
        #[doc = "Describes a verifiable build."]
        #[serde(
            rename = "build",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build: ::std::option::Option<crate::schemas::BuildOccurrence>,
        #[doc = "Describes a compliance violation on a linked resource."]
        #[serde(
            rename = "compliance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compliance: ::std::option::Option<crate::schemas::ComplianceOccurrence>,
        #[doc = "Output only. The time this occurrence was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Describes the deployment of an artifact on a runtime."]
        #[serde(
            rename = "deployment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deployment: ::std::option::Option<crate::schemas::DeploymentOccurrence>,
        #[doc = "Describes when a resource was discovered."]
        #[serde(
            rename = "discovery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discovery: ::std::option::Option<crate::schemas::DiscoveryOccurrence>,
        #[doc = "Describes an attestation of an artifact using dsse."]
        #[serde(
            rename = "dsseAttestation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dsse_attestation: ::std::option::Option<crate::schemas::DsseattestationOccurrence>,
        #[doc = "https://github.com/secure-systems-lab/dsse"]
        #[serde(
            rename = "envelope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub envelope: ::std::option::Option<crate::schemas::Envelope>,
        #[doc = "Describes how this resource derives from the basis in the associated note."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::ImageOccurrence>,
        #[doc = "Output only. This explicitly denotes which of the occurrence details are specified. This field can be used as a filter in list requests."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::OccurrenceKind>,
        #[doc = "Output only. The name of the occurrence in the form of `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Required. Immutable. The analysis note associated with this occurrence, in the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be used as a filter in list requests."]
        #[serde(
            rename = "noteName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub note_name: ::std::option::Option<String>,
        #[doc = "Describes the installation of a package on the linked resource."]
        #[serde(
            rename = "package",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package: ::std::option::Option<crate::schemas::PackageOccurrence>,
        #[doc = "A description of actions that can be taken to remedy the note."]
        #[serde(
            rename = "remediation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub remediation: ::std::option::Option<String>,
        #[doc = "Required. Immutable. A URI that represents the resource for which the occurrence applies. For example, `https://gcr.io/project/image@sha256:123abc` for a Docker image."]
        #[serde(
            rename = "resourceUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_uri: ::std::option::Option<String>,
        #[doc = "Output only. The time this occurrence was last updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
        #[doc = "Describes an available package upgrade on the linked resource."]
        #[serde(
            rename = "upgrade",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upgrade: ::std::option::Option<crate::schemas::UpgradeOccurrence>,
        #[doc = "Describes a security vulnerability."]
        #[serde(
            rename = "vulnerability",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vulnerability: ::std::option::Option<crate::schemas::VulnerabilityOccurrence>,
    }
    impl ::google_field_selector::FieldSelector for Occurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Occurrence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OccurrenceKind {
        #[doc = "This represents a logical \"role\" that can attest to artifacts."]
        Attestation,
        #[doc = "The note and occurrence assert build provenance."]
        Build,
        #[doc = "This represents a Compliance Note"]
        Compliance,
        #[doc = "The note and occurrence track deployment events."]
        Deployment,
        #[doc = "The note and occurrence track the initial discovery status of a resource."]
        Discovery,
        #[doc = "This represents a DSSE attestation Note"]
        DsseAttestation,
        #[doc = "This represents an image basis relationship."]
        Image,
        #[doc = "Default value. This value is unused."]
        NoteKindUnspecified,
        #[doc = "This represents a package installed via a package manager."]
        Package,
        #[doc = "This represents an available package upgrade."]
        Upgrade,
        #[doc = "The note and occurrence represent a package vulnerability."]
        Vulnerability,
    }
    impl OccurrenceKind {
        pub fn as_str(self) -> &'static str {
            match self {
                OccurrenceKind::Attestation => "ATTESTATION",
                OccurrenceKind::Build => "BUILD",
                OccurrenceKind::Compliance => "COMPLIANCE",
                OccurrenceKind::Deployment => "DEPLOYMENT",
                OccurrenceKind::Discovery => "DISCOVERY",
                OccurrenceKind::DsseAttestation => "DSSE_ATTESTATION",
                OccurrenceKind::Image => "IMAGE",
                OccurrenceKind::NoteKindUnspecified => "NOTE_KIND_UNSPECIFIED",
                OccurrenceKind::Package => "PACKAGE",
                OccurrenceKind::Upgrade => "UPGRADE",
                OccurrenceKind::Vulnerability => "VULNERABILITY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OccurrenceKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OccurrenceKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OccurrenceKind, ()> {
            Ok(match s {
                "ATTESTATION" => OccurrenceKind::Attestation,
                "BUILD" => OccurrenceKind::Build,
                "COMPLIANCE" => OccurrenceKind::Compliance,
                "DEPLOYMENT" => OccurrenceKind::Deployment,
                "DISCOVERY" => OccurrenceKind::Discovery,
                "DSSE_ATTESTATION" => OccurrenceKind::DsseAttestation,
                "IMAGE" => OccurrenceKind::Image,
                "NOTE_KIND_UNSPECIFIED" => OccurrenceKind::NoteKindUnspecified,
                "PACKAGE" => OccurrenceKind::Package,
                "UPGRADE" => OccurrenceKind::Upgrade,
                "VULNERABILITY" => OccurrenceKind::Vulnerability,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OccurrenceKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OccurrenceKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OccurrenceKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTESTATION" => OccurrenceKind::Attestation,
                "BUILD" => OccurrenceKind::Build,
                "COMPLIANCE" => OccurrenceKind::Compliance,
                "DEPLOYMENT" => OccurrenceKind::Deployment,
                "DISCOVERY" => OccurrenceKind::Discovery,
                "DSSE_ATTESTATION" => OccurrenceKind::DsseAttestation,
                "IMAGE" => OccurrenceKind::Image,
                "NOTE_KIND_UNSPECIFIED" => OccurrenceKind::NoteKindUnspecified,
                "PACKAGE" => OccurrenceKind::Package,
                "UPGRADE" => OccurrenceKind::Upgrade,
                "VULNERABILITY" => OccurrenceKind::Vulnerability,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OccurrenceKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OccurrenceKind {
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
    pub struct PackageIssue {
        #[doc = "Required. The [CPE URI](https://cpe.mitre.org/specification/) this vulnerability was found in."]
        #[serde(
            rename = "affectedCpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub affected_cpe_uri: ::std::option::Option<String>,
        #[doc = "Required. The package this vulnerability was found in."]
        #[serde(
            rename = "affectedPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub affected_package: ::std::option::Option<String>,
        #[doc = "Required. The version of the package that is installed on the resource affected by this vulnerability."]
        #[serde(
            rename = "affectedVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub affected_version: ::std::option::Option<crate::schemas::Version>,
        #[doc = "Output only. The distro or language system assigned severity for this vulnerability when that is available and note provider assigned severity when it is not available."]
        #[serde(
            rename = "effectiveSeverity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub effective_severity:
            ::std::option::Option<crate::schemas::PackageIssueEffectiveSeverity>,
        #[doc = "Output only. Whether a fix is available for this package."]
        #[serde(
            rename = "fixAvailable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fix_available: ::std::option::Option<bool>,
        #[doc = "The [CPE URI](https://cpe.mitre.org/specification/) this vulnerability was fixed in. It is possible for this to be different from the affected_cpe_uri."]
        #[serde(
            rename = "fixedCpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_cpe_uri: ::std::option::Option<String>,
        #[doc = "The package this vulnerability was fixed in. It is possible for this to be different from the affected_package."]
        #[serde(
            rename = "fixedPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_package: ::std::option::Option<String>,
        #[doc = "Required. The version of the package this vulnerability was fixed in. Setting this to VersionKind.MAXIMUM means no fix is yet available."]
        #[serde(
            rename = "fixedVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_version: ::std::option::Option<crate::schemas::Version>,
        #[doc = "The type of package (e.g. OS, MAVEN, GO)."]
        #[serde(
            rename = "packageType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PackageIssue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PackageIssue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PackageIssueEffectiveSeverity {
        #[doc = "Critical severity."]
        Critical,
        #[doc = "High severity."]
        High,
        #[doc = "Low severity."]
        Low,
        #[doc = "Medium severity."]
        Medium,
        #[doc = "Minimal severity."]
        Minimal,
        #[doc = "Unknown."]
        SeverityUnspecified,
    }
    impl PackageIssueEffectiveSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                PackageIssueEffectiveSeverity::Critical => "CRITICAL",
                PackageIssueEffectiveSeverity::High => "HIGH",
                PackageIssueEffectiveSeverity::Low => "LOW",
                PackageIssueEffectiveSeverity::Medium => "MEDIUM",
                PackageIssueEffectiveSeverity::Minimal => "MINIMAL",
                PackageIssueEffectiveSeverity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PackageIssueEffectiveSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PackageIssueEffectiveSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PackageIssueEffectiveSeverity, ()> {
            Ok(match s {
                "CRITICAL" => PackageIssueEffectiveSeverity::Critical,
                "HIGH" => PackageIssueEffectiveSeverity::High,
                "LOW" => PackageIssueEffectiveSeverity::Low,
                "MEDIUM" => PackageIssueEffectiveSeverity::Medium,
                "MINIMAL" => PackageIssueEffectiveSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => PackageIssueEffectiveSeverity::SeverityUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PackageIssueEffectiveSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PackageIssueEffectiveSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PackageIssueEffectiveSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRITICAL" => PackageIssueEffectiveSeverity::Critical,
                "HIGH" => PackageIssueEffectiveSeverity::High,
                "LOW" => PackageIssueEffectiveSeverity::Low,
                "MEDIUM" => PackageIssueEffectiveSeverity::Medium,
                "MINIMAL" => PackageIssueEffectiveSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => PackageIssueEffectiveSeverity::SeverityUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PackageIssueEffectiveSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PackageIssueEffectiveSeverity {
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
    pub struct PackageNote {
        #[doc = "The various channels by which a package is distributed."]
        #[serde(
            rename = "distribution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distribution: ::std::option::Option<Vec<crate::schemas::Distribution>>,
        #[doc = "Required. Immutable. The name of the package."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PackageNote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PackageNote {
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
    pub struct PackageOccurrence {
        #[doc = "Required. All of the places within the filesystem versions of this package have been found."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<Vec<crate::schemas::Location>>,
        #[doc = "Output only. The name of the installed package."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PackageOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PackageOccurrence {
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
    pub struct Policy {
        #[doc = "Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`."]
        #[serde(
            rename = "bindings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bindings: ::std::option::Option<Vec<crate::schemas::Binding>>,
        #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Policy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Policy {
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
    pub struct ProjectRepoId {
        #[doc = "The ID of the project."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "The name of the repo. Leave empty for the default repo."]
        #[serde(
            rename = "repoName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repo_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ProjectRepoId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProjectRepoId {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Recipe {
        #[doc = "Collection of all external inputs that influenced the build on top of recipe.definedInMaterial and recipe.entryPoint. For example, if the recipe type were \"make\", then this might be the flags passed to make aside from the target, which is captured in recipe.entryPoint. Since the arguments field can greatly vary in structure, depending on the builder and recipe type, this is of form \"Any\"."]
        #[serde(
            rename = "arguments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arguments:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "Index in materials containing the recipe steps that are not implied by recipe.type. For example, if the recipe type were \"make\", then this would point to the source containing the Makefile, not the make program itself. Set to -1 if the recipe doesn't come from a material, as zero is default unset value for int64."]
        #[serde(
            rename = "definedInMaterial",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub defined_in_material: ::std::option::Option<i64>,
        #[doc = "String identifying the entry point into the build. This is often a path to a configuration file and/or a target label within that file. The syntax and meaning are defined by recipe.type. For example, if the recipe type were \"make\", then this would reference the directory in which to run make as well as which target to use."]
        #[serde(
            rename = "entryPoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entry_point: ::std::option::Option<String>,
        #[doc = "Any other builder-controlled inputs necessary for correctly evaluating the recipe. Usually only needed for reproducing the build but not evaluated as part of policy. Since the environment field can greatly vary in structure, depending on the builder and recipe type, this is of form \"Any\"."]
        #[serde(
            rename = "environment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "URI indicating what type of recipe was performed. It determines the meaning of recipe.entryPoint, recipe.arguments, recipe.environment, and materials."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Recipe {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Recipe {
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
    pub struct RelatedUrl {
        #[doc = "Label to describe usage of the URL."]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
        #[doc = "Specific URL associated with the resource."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RelatedUrl {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RelatedUrl {
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
    pub struct RepoId {
        #[doc = "A combination of a project ID and a repo name."]
        #[serde(
            rename = "projectRepoId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_repo_id: ::std::option::Option<crate::schemas::ProjectRepoId>,
        #[doc = "A server-assigned, globally unique identifier."]
        #[serde(
            rename = "uid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uid: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RepoId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RepoId {
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
    pub struct SetIamPolicyRequest {
        #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them."]
        #[serde(
            rename = "policy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub policy: ::std::option::Option<crate::schemas::Policy>,
    }
    impl ::google_field_selector::FieldSelector for SetIamPolicyRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SetIamPolicyRequest {
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
    pub struct Signature {
        #[doc = "The identifier for the public key that verifies this signature. * The `public_key_id` is required. * The `public_key_id` SHOULD be an RFC3986 conformant URI. * When possible, the `public_key_id` SHOULD be an immutable reference, such as a cryptographic digest. Examples of valid `public_key_id`s: OpenPGP V4 public key fingerprint: * \"openpgp4fpr:74FAF3B861BDA0870C7B6DEF607E48D2A663AEEA\" See https://www.iana.org/assignments/uri-schemes/prov/openpgp4fpr for more details on this scheme. RFC6920 digest-named SubjectPublicKeyInfo (digest of the DER serialization): * \"ni:///sha-256;cD9o9Cq6LG3jD0iKXqEi_vdjJGecm_iXkbqVoScViaU\" * \"nih:///sha-256;703f68f42aba2c6de30f488a5ea122fef76324679c9bf89791ba95a1271589a5\""]
        #[serde(
            rename = "publicKeyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub public_key_id: ::std::option::Option<String>,
        #[doc = "The content of the signature, an opaque bytestring. The payload that this signature verifies MUST be unambiguously provided with the Signature during verification. A wrapper message might provide the payload explicitly. Alternatively, a message might have a canonical serialization that can always be unambiguously computed to derive the payload."]
        #[serde(
            rename = "signature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signature: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for Signature {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Signature {
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
    pub struct SlsaBuilder {
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SlsaBuilder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlsaBuilder {
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
    pub struct SlsaCompleteness {
        #[doc = "If true, the builder claims that recipe.arguments is complete, meaning that all external inputs are properly captured in the recipe."]
        #[serde(
            rename = "arguments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arguments: ::std::option::Option<bool>,
        #[doc = "If true, the builder claims that recipe.environment is claimed to be complete."]
        #[serde(
            rename = "environment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment: ::std::option::Option<bool>,
        #[doc = "If true, the builder claims that materials are complete, usually through some controls to prevent network access. Sometimes called \"hermetic\"."]
        #[serde(
            rename = "materials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub materials: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for SlsaCompleteness {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlsaCompleteness {
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
    pub struct SlsaMetadata {
        #[doc = "The timestamp of when the build completed."]
        #[serde(
            rename = "buildFinishedOn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_finished_on: ::std::option::Option<String>,
        #[doc = "Identifies the particular build invocation, which can be useful for finding associated logs or other ad-hoc analysis. The value SHOULD be globally unique, per in-toto Provenance spec."]
        #[serde(
            rename = "buildInvocationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_invocation_id: ::std::option::Option<String>,
        #[doc = "The timestamp of when the build started."]
        #[serde(
            rename = "buildStartedOn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_started_on: ::std::option::Option<String>,
        #[doc = "Indicates that the builder claims certain fields in this message to be complete."]
        #[serde(
            rename = "completeness",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub completeness: ::std::option::Option<crate::schemas::SlsaCompleteness>,
        #[doc = "If true, the builder claims that running the recipe on materials will produce bit-for-bit identical output."]
        #[serde(
            rename = "reproducible",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reproducible: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for SlsaMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlsaMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SlsaProvenance {
        #[doc = "required"]
        #[serde(
            rename = "builder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub builder: ::std::option::Option<crate::schemas::SlsaBuilder>,
        #[doc = "The collection of artifacts that influenced the build including sources, dependencies, build tools, base images, and so on. This is considered to be incomplete unless metadata.completeness.materials is true. Unset or null is equivalent to empty."]
        #[serde(
            rename = "materials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub materials: ::std::option::Option<Vec<crate::schemas::Material>>,
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::SlsaMetadata>,
        #[doc = "Identifies the configuration used for the build. When combined with materials, this SHOULD fully describe the build, such that re-running this recipe results in bit-for-bit identical output (if the build is reproducible). required"]
        #[serde(
            rename = "recipe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recipe: ::std::option::Option<crate::schemas::SlsaRecipe>,
    }
    impl ::google_field_selector::FieldSelector for SlsaProvenance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlsaProvenance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SlsaRecipe {
        #[doc = "Collection of all external inputs that influenced the build on top of recipe.definedInMaterial and recipe.entryPoint. For example, if the recipe type were \"make\", then this might be the flags passed to make aside from the target, which is captured in recipe.entryPoint. Depending on the recipe Type, the structure may be different."]
        #[serde(
            rename = "arguments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arguments:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Index in materials containing the recipe steps that are not implied by recipe.type. For example, if the recipe type were \"make\", then this would point to the source containing the Makefile, not the make program itself. Set to -1 if the recipe doesn't come from a material, as zero is default unset value for int64."]
        #[serde(
            rename = "definedInMaterial",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub defined_in_material: ::std::option::Option<i64>,
        #[doc = "String identifying the entry point into the build. This is often a path to a configuration file and/or a target label within that file. The syntax and meaning are defined by recipe.type. For example, if the recipe type were \"make\", then this would reference the directory in which to run make as well as which target to use."]
        #[serde(
            rename = "entryPoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entry_point: ::std::option::Option<String>,
        #[doc = "Any other builder-controlled inputs necessary for correctly evaluating the recipe. Usually only needed for reproducing the build but not evaluated as part of policy. Depending on the recipe Type, the structure may be different."]
        #[serde(
            rename = "environment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub environment:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "URI indicating what type of recipe was performed. It determines the meaning of recipe.entryPoint, recipe.arguments, recipe.environment, and materials."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SlsaRecipe {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlsaRecipe {
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
    pub struct Source {
        #[doc = "If provided, some of the source code used for the build may be found in these locations, in the case where the source repository had multiple remotes or submodules. This list will not include the context specified in the context field."]
        #[serde(
            rename = "additionalContexts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_contexts: ::std::option::Option<Vec<crate::schemas::SourceContext>>,
        #[doc = "If provided, the input binary artifacts for the build came from this location."]
        #[serde(
            rename = "artifactStorageSourceUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub artifact_storage_source_uri: ::std::option::Option<String>,
        #[doc = "If provided, the source code used for the build came from this location."]
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context: ::std::option::Option<crate::schemas::SourceContext>,
        #[doc = "Hash(es) of the build source, which can be used to verify that the original source integrity was maintained in the build. The keys to this map are file paths used as build source and the values contain the hash values for those files. If the build source came in a single package such as a gzipped tarfile (.tar.gz), the FileHash will be for the single path to that file."]
        #[serde(
            rename = "fileHashes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_hashes:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::FileHashes>>,
    }
    impl ::google_field_selector::FieldSelector for Source {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Source {
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
    pub struct SourceContext {
        #[doc = "A SourceContext referring to a revision in a Google Cloud Source Repo."]
        #[serde(
            rename = "cloudRepo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_repo: ::std::option::Option<crate::schemas::CloudRepoSourceContext>,
        #[doc = "A SourceContext referring to a Gerrit project."]
        #[serde(
            rename = "gerrit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gerrit: ::std::option::Option<crate::schemas::GerritSourceContext>,
        #[doc = "A SourceContext referring to any third party Git repo (e.g., GitHub)."]
        #[serde(
            rename = "git",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub git: ::std::option::Option<crate::schemas::GitSourceContext>,
        #[doc = "Labels with user defined metadata."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::google_field_selector::FieldSelector for SourceContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceContext {
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
    pub struct Subject {
        #[doc = "`\"\": \"\"` Algorithms can be e.g. sha256, sha512 See https://github.com/in-toto/attestation/blob/main/spec/field_types.md#DigestSet"]
        #[serde(
            rename = "digest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub digest: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Subject {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Subject {
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
    pub struct TestIamPermissionsRequest {
        #[doc = "The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TestIamPermissionsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestIamPermissionsRequest {
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
    pub struct TestIamPermissionsResponse {
        #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is allowed."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TestIamPermissionsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TestIamPermissionsResponse {
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
    pub struct UpgradeDistribution {
        #[doc = "The operating system classification of this Upgrade, as specified by the upstream operating system upgrade feed. For Windows the classification is one of the category_ids listed at https://docs.microsoft.com/en-us/previous-versions/windows/desktop/ff357803(v=vs.85)"]
        #[serde(
            rename = "classification",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub classification: ::std::option::Option<String>,
        #[doc = "Required - The specific operating system this metadata applies to. See https://cpe.mitre.org/specification/."]
        #[serde(
            rename = "cpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpe_uri: ::std::option::Option<String>,
        #[doc = "The cve tied to this Upgrade."]
        #[serde(
            rename = "cve",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cve: ::std::option::Option<Vec<String>>,
        #[doc = "The severity as specified by the upstream operating system."]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpgradeDistribution {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpgradeDistribution {
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
    pub struct UpgradeNote {
        #[doc = "Metadata about the upgrade for each specific operating system."]
        #[serde(
            rename = "distributions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distributions: ::std::option::Option<Vec<crate::schemas::UpgradeDistribution>>,
        #[doc = "Required for non-Windows OS. The package this Upgrade is for."]
        #[serde(
            rename = "package",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package: ::std::option::Option<String>,
        #[doc = "Required for non-Windows OS. The version of the package in machine + human readable form."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<crate::schemas::Version>,
        #[doc = "Required for Windows OS. Represents the metadata about the Windows update."]
        #[serde(
            rename = "windowsUpdate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub windows_update: ::std::option::Option<crate::schemas::WindowsUpdate>,
    }
    impl ::google_field_selector::FieldSelector for UpgradeNote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpgradeNote {
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
    pub struct UpgradeOccurrence {
        #[doc = "Metadata about the upgrade for available for the specific operating system for the resource_url. This allows efficient filtering, as well as making it easier to use the occurrence."]
        #[serde(
            rename = "distribution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distribution: ::std::option::Option<crate::schemas::UpgradeDistribution>,
        #[doc = "Required for non-Windows OS. The package this Upgrade is for."]
        #[serde(
            rename = "package",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package: ::std::option::Option<String>,
        #[doc = "Required for non-Windows OS. The version of the package in a machine + human readable form."]
        #[serde(
            rename = "parsedVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parsed_version: ::std::option::Option<crate::schemas::Version>,
        #[doc = "Required for Windows OS. Represents the metadata about the Windows update."]
        #[serde(
            rename = "windowsUpdate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub windows_update: ::std::option::Option<crate::schemas::WindowsUpdate>,
    }
    impl ::google_field_selector::FieldSelector for UpgradeOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpgradeOccurrence {
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
    pub struct Version {
        #[doc = "Used to correct mistakes in the version numbering scheme."]
        #[serde(
            rename = "epoch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub epoch: ::std::option::Option<i32>,
        #[doc = "Human readable version string. This string is of the form :- and is only set when kind is NORMAL."]
        #[serde(
            rename = "fullName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub full_name: ::std::option::Option<String>,
        #[doc = "Whether this version is specifying part of an inclusive range. Grafeas does not have the capability to specify version ranges; instead we have fields that specify start version and end versions. At times this is insufficient - we also need to specify whether the version is included in the range or is excluded from the range. This boolean is expected to be set to true when the version is included in a range."]
        #[serde(
            rename = "inclusive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inclusive: ::std::option::Option<bool>,
        #[doc = "Required. Distinguishes between sentinel MIN/MAX versions and normal versions."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::VersionKind>,
        #[doc = "Required only when version kind is NORMAL. The main part of the version name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The iteration of the package build from the above version."]
        #[serde(
            rename = "revision",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Version {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Version {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VersionKind {
        #[doc = "A special version representing positive infinity."]
        Maximum,
        #[doc = "A special version representing negative infinity."]
        Minimum,
        #[doc = "A standard package version."]
        Normal,
        #[doc = "Unknown."]
        VersionKindUnspecified,
    }
    impl VersionKind {
        pub fn as_str(self) -> &'static str {
            match self {
                VersionKind::Maximum => "MAXIMUM",
                VersionKind::Minimum => "MINIMUM",
                VersionKind::Normal => "NORMAL",
                VersionKind::VersionKindUnspecified => "VERSION_KIND_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for VersionKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VersionKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<VersionKind, ()> {
            Ok(match s {
                "MAXIMUM" => VersionKind::Maximum,
                "MINIMUM" => VersionKind::Minimum,
                "NORMAL" => VersionKind::Normal,
                "VERSION_KIND_UNSPECIFIED" => VersionKind::VersionKindUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VersionKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VersionKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VersionKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MAXIMUM" => VersionKind::Maximum,
                "MINIMUM" => VersionKind::Minimum,
                "NORMAL" => VersionKind::Normal,
                "VERSION_KIND_UNSPECIFIED" => VersionKind::VersionKindUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for VersionKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VersionKind {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VulnerabilityNote {
        #[doc = "The CVSS score of this vulnerability. CVSS score is on a scale of 0 - 10 where 0 indicates low severity and 10 indicates high severity."]
        #[serde(
            rename = "cvssScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cvss_score: ::std::option::Option<f32>,
        #[doc = "The full description of the CVSSv3 for this vulnerability."]
        #[serde(
            rename = "cvssV3",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cvss_v3: ::std::option::Option<crate::schemas::Cvssv3>,
        #[doc = "Details of all known distros and packages affected by this vulnerability."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details: ::std::option::Option<Vec<crate::schemas::Detail>>,
        #[doc = "The note provider assigned severity of this vulnerability."]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<crate::schemas::VulnerabilityNoteSeverity>,
        #[doc = "The time this information was last changed at the source. This is an upstream timestamp from the underlying information source - e.g. Ubuntu security tracker."]
        #[serde(
            rename = "sourceUpdateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_update_time: ::std::option::Option<String>,
        #[doc = "Windows details get their own format because the information format and model don't match a normal detail. Specifically Windows updates are done as patches, thus Windows vulnerabilities really are a missing package, rather than a package being at an incorrect version."]
        #[serde(
            rename = "windowsDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub windows_details: ::std::option::Option<Vec<crate::schemas::WindowsDetail>>,
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityNote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityNote {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VulnerabilityNoteSeverity {
        #[doc = "Critical severity."]
        Critical,
        #[doc = "High severity."]
        High,
        #[doc = "Low severity."]
        Low,
        #[doc = "Medium severity."]
        Medium,
        #[doc = "Minimal severity."]
        Minimal,
        #[doc = "Unknown."]
        SeverityUnspecified,
    }
    impl VulnerabilityNoteSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                VulnerabilityNoteSeverity::Critical => "CRITICAL",
                VulnerabilityNoteSeverity::High => "HIGH",
                VulnerabilityNoteSeverity::Low => "LOW",
                VulnerabilityNoteSeverity::Medium => "MEDIUM",
                VulnerabilityNoteSeverity::Minimal => "MINIMAL",
                VulnerabilityNoteSeverity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for VulnerabilityNoteSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VulnerabilityNoteSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<VulnerabilityNoteSeverity, ()> {
            Ok(match s {
                "CRITICAL" => VulnerabilityNoteSeverity::Critical,
                "HIGH" => VulnerabilityNoteSeverity::High,
                "LOW" => VulnerabilityNoteSeverity::Low,
                "MEDIUM" => VulnerabilityNoteSeverity::Medium,
                "MINIMAL" => VulnerabilityNoteSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => VulnerabilityNoteSeverity::SeverityUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VulnerabilityNoteSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VulnerabilityNoteSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VulnerabilityNoteSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRITICAL" => VulnerabilityNoteSeverity::Critical,
                "HIGH" => VulnerabilityNoteSeverity::High,
                "LOW" => VulnerabilityNoteSeverity::Low,
                "MEDIUM" => VulnerabilityNoteSeverity::Medium,
                "MINIMAL" => VulnerabilityNoteSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => VulnerabilityNoteSeverity::SeverityUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityNoteSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityNoteSeverity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VulnerabilityOccurrence {
        #[doc = "Output only. The CVSS score of this vulnerability. CVSS score is on a scale of 0 - 10 where 0 indicates low severity and 10 indicates high severity."]
        #[serde(
            rename = "cvssScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cvss_score: ::std::option::Option<f32>,
        #[doc = "The cvss v3 score for the vulnerability."]
        #[serde(
            rename = "cvssv3",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cvssv_3: ::std::option::Option<crate::schemas::Cvss>,
        #[doc = "The distro assigned severity for this vulnerability when it is available, otherwise this is the note provider assigned severity. When there are multiple PackageIssues for this vulnerability, they can have different effective severities because some might be provided by the distro while others are provided by the language ecosystem for a language pack. For this reason, it is advised to use the effective severity on the PackageIssue level. In the case where multiple PackageIssues have differing effective severities, this field should be the highest severity for any of the PackageIssues."]
        #[serde(
            rename = "effectiveSeverity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub effective_severity:
            ::std::option::Option<crate::schemas::VulnerabilityOccurrenceEffectiveSeverity>,
        #[doc = "Output only. Whether at least one of the affected packages has a fix available."]
        #[serde(
            rename = "fixAvailable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fix_available: ::std::option::Option<bool>,
        #[doc = "Output only. A detailed description of this vulnerability."]
        #[serde(
            rename = "longDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub long_description: ::std::option::Option<String>,
        #[doc = "Required. The set of affected locations and their fixes (if available) within the associated resource."]
        #[serde(
            rename = "packageIssue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_issue: ::std::option::Option<Vec<crate::schemas::PackageIssue>>,
        #[doc = "The type of package; whether native or non native (e.g., ruby gems, node.js packages, etc.)."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Output only. URLs related to this vulnerability."]
        #[serde(
            rename = "relatedUrls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_urls: ::std::option::Option<Vec<crate::schemas::RelatedUrl>>,
        #[doc = "Output only. The note provider assigned severity of this vulnerability."]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<crate::schemas::VulnerabilityOccurrenceSeverity>,
        #[doc = "Output only. A one sentence description of this vulnerability."]
        #[serde(
            rename = "shortDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub short_description: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityOccurrence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VulnerabilityOccurrenceEffectiveSeverity {
        #[doc = "Critical severity."]
        Critical,
        #[doc = "High severity."]
        High,
        #[doc = "Low severity."]
        Low,
        #[doc = "Medium severity."]
        Medium,
        #[doc = "Minimal severity."]
        Minimal,
        #[doc = "Unknown."]
        SeverityUnspecified,
    }
    impl VulnerabilityOccurrenceEffectiveSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                VulnerabilityOccurrenceEffectiveSeverity::Critical => "CRITICAL",
                VulnerabilityOccurrenceEffectiveSeverity::High => "HIGH",
                VulnerabilityOccurrenceEffectiveSeverity::Low => "LOW",
                VulnerabilityOccurrenceEffectiveSeverity::Medium => "MEDIUM",
                VulnerabilityOccurrenceEffectiveSeverity::Minimal => "MINIMAL",
                VulnerabilityOccurrenceEffectiveSeverity::SeverityUnspecified => {
                    "SEVERITY_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for VulnerabilityOccurrenceEffectiveSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VulnerabilityOccurrenceEffectiveSeverity {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<VulnerabilityOccurrenceEffectiveSeverity, ()> {
            Ok(match s {
                "CRITICAL" => VulnerabilityOccurrenceEffectiveSeverity::Critical,
                "HIGH" => VulnerabilityOccurrenceEffectiveSeverity::High,
                "LOW" => VulnerabilityOccurrenceEffectiveSeverity::Low,
                "MEDIUM" => VulnerabilityOccurrenceEffectiveSeverity::Medium,
                "MINIMAL" => VulnerabilityOccurrenceEffectiveSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => {
                    VulnerabilityOccurrenceEffectiveSeverity::SeverityUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VulnerabilityOccurrenceEffectiveSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VulnerabilityOccurrenceEffectiveSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VulnerabilityOccurrenceEffectiveSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRITICAL" => VulnerabilityOccurrenceEffectiveSeverity::Critical,
                "HIGH" => VulnerabilityOccurrenceEffectiveSeverity::High,
                "LOW" => VulnerabilityOccurrenceEffectiveSeverity::Low,
                "MEDIUM" => VulnerabilityOccurrenceEffectiveSeverity::Medium,
                "MINIMAL" => VulnerabilityOccurrenceEffectiveSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => {
                    VulnerabilityOccurrenceEffectiveSeverity::SeverityUnspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityOccurrenceEffectiveSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityOccurrenceEffectiveSeverity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VulnerabilityOccurrenceSeverity {
        #[doc = "Critical severity."]
        Critical,
        #[doc = "High severity."]
        High,
        #[doc = "Low severity."]
        Low,
        #[doc = "Medium severity."]
        Medium,
        #[doc = "Minimal severity."]
        Minimal,
        #[doc = "Unknown."]
        SeverityUnspecified,
    }
    impl VulnerabilityOccurrenceSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                VulnerabilityOccurrenceSeverity::Critical => "CRITICAL",
                VulnerabilityOccurrenceSeverity::High => "HIGH",
                VulnerabilityOccurrenceSeverity::Low => "LOW",
                VulnerabilityOccurrenceSeverity::Medium => "MEDIUM",
                VulnerabilityOccurrenceSeverity::Minimal => "MINIMAL",
                VulnerabilityOccurrenceSeverity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for VulnerabilityOccurrenceSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VulnerabilityOccurrenceSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<VulnerabilityOccurrenceSeverity, ()> {
            Ok(match s {
                "CRITICAL" => VulnerabilityOccurrenceSeverity::Critical,
                "HIGH" => VulnerabilityOccurrenceSeverity::High,
                "LOW" => VulnerabilityOccurrenceSeverity::Low,
                "MEDIUM" => VulnerabilityOccurrenceSeverity::Medium,
                "MINIMAL" => VulnerabilityOccurrenceSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => VulnerabilityOccurrenceSeverity::SeverityUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VulnerabilityOccurrenceSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VulnerabilityOccurrenceSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VulnerabilityOccurrenceSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRITICAL" => VulnerabilityOccurrenceSeverity::Critical,
                "HIGH" => VulnerabilityOccurrenceSeverity::High,
                "LOW" => VulnerabilityOccurrenceSeverity::Low,
                "MEDIUM" => VulnerabilityOccurrenceSeverity::Medium,
                "MINIMAL" => VulnerabilityOccurrenceSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => VulnerabilityOccurrenceSeverity::SeverityUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityOccurrenceSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityOccurrenceSeverity {
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
    pub struct VulnerabilityOccurrencesSummary {
        #[doc = "A listing by resource of the number of fixable and total vulnerabilities."]
        #[serde(
            rename = "counts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub counts: ::std::option::Option<Vec<crate::schemas::FixableTotalByDigest>>,
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityOccurrencesSummary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityOccurrencesSummary {
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
    pub struct WindowsDetail {
        #[doc = "Required. The [CPE URI](https://cpe.mitre.org/specification/) this vulnerability affects."]
        #[serde(
            rename = "cpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpe_uri: ::std::option::Option<String>,
        #[doc = "The description of this vulnerability."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Required. The names of the KBs which have hotfixes to mitigate this vulnerability. Note that there may be multiple hotfixes (and thus multiple KBs) that mitigate a given vulnerability. Currently any listed KBs presence is considered a fix."]
        #[serde(
            rename = "fixingKbs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixing_kbs: ::std::option::Option<Vec<crate::schemas::KnowledgeBase>>,
        #[doc = "Required. The name of this vulnerability."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WindowsDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WindowsDetail {
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
    pub struct WindowsUpdate {
        #[doc = "The list of categories to which the update belongs."]
        #[serde(
            rename = "categories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categories: ::std::option::Option<Vec<crate::schemas::Category>>,
        #[doc = "The localized description of the update."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Required - The unique identifier for the update."]
        #[serde(
            rename = "identity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub identity: ::std::option::Option<crate::schemas::Identity>,
        #[doc = "The Microsoft Knowledge Base article IDs that are associated with the update."]
        #[serde(
            rename = "kbArticleIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kb_article_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The last published timestamp of the update."]
        #[serde(
            rename = "lastPublishedTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_published_timestamp: ::std::option::Option<String>,
        #[doc = "The hyperlink to the support information for the update."]
        #[serde(
            rename = "supportUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub support_url: ::std::option::Option<String>,
        #[doc = "The localized title of the update."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for WindowsUpdate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WindowsUpdate {
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
    reqwest: ::reqwest::blocking::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client::with_reqwest_client(
            auth,
            ::reqwest::blocking::Client::builder()
                .timeout(None)
                .build()
                .unwrap(),
        )
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::blocking::Client) -> Self
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
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a> {
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProjectsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the notes resource"]
            pub fn notes(&self) -> crate::resources::projects::notes::NotesActions {
                crate::resources::projects::notes::NotesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the occurrences resource"]
            pub fn occurrences(
                &self,
            ) -> crate::resources::projects::occurrences::OccurrencesActions {
                crate::resources::projects::occurrences::OccurrencesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod notes {
            pub mod params {}
            pub struct NotesActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> NotesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates new notes in batch."]
                pub fn batch_create(
                    &self,
                    request: crate::schemas::BatchCreateNotesRequest,
                    parent: impl Into<String>,
                ) -> BatchCreateRequestBuilder {
                    BatchCreateRequestBuilder {
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Creates a new note."]
                pub fn create(
                    &self,
                    request: crate::schemas::Note,
                    parent: impl Into<String>,
                ) -> CreateRequestBuilder {
                    CreateRequestBuilder {
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
                        parent: parent.into(),
                        note_id: None,
                    }
                }
                #[doc = "Deletes the specified note."]
                pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                    DeleteRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
                        name: name.into(),
                    }
                }
                #[doc = "Gets the specified note."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
                        name: name.into(),
                    }
                }
                #[doc = "Gets the access control policy for a note or an occurrence resource. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."]
                pub fn get_iam_policy(
                    &self,
                    request: crate::schemas::GetIamPolicyRequest,
                    resource: impl Into<String>,
                ) -> GetIamPolicyRequestBuilder {
                    GetIamPolicyRequestBuilder {
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
                        resource: resource.into(),
                    }
                }
                #[doc = "Lists notes for the specified project."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
                        parent: parent.into(),
                        filter: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates the specified note."]
                pub fn patch(
                    &self,
                    request: crate::schemas::Note,
                    name: impl Into<String>,
                ) -> PatchRequestBuilder {
                    PatchRequestBuilder {
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
                        name: name.into(),
                        update_mask: None,
                    }
                }
                #[doc = "Sets the access control policy on the specified note or occurrence. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or an occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."]
                pub fn set_iam_policy(
                    &self,
                    request: crate::schemas::SetIamPolicyRequest,
                    resource: impl Into<String>,
                ) -> SetIamPolicyRequestBuilder {
                    SetIamPolicyRequestBuilder {
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
                        resource: resource.into(),
                    }
                }
                #[doc = "Returns the permissions that a caller has on the specified note or occurrence. Requires list permission on the project (for example, `containeranalysis.notes.list`). The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."]
                pub fn test_iam_permissions(
                    &self,
                    request: crate::schemas::TestIamPermissionsRequest,
                    resource: impl Into<String>,
                ) -> TestIamPermissionsRequestBuilder {
                    TestIamPermissionsRequestBuilder {
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
                        resource: resource.into(),
                    }
                }
                #[doc = "Actions that can be performed on the occurrences resource"]
                pub fn occurrences(
                    &self,
                ) -> crate::resources::projects::notes::occurrences::OccurrencesActions
                {
                    crate::resources::projects::notes::occurrences::OccurrencesActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [NotesActions::batch_create()](struct.NotesActions.html#method.batch_create)"]
            #[derive(Debug, Clone)]
            pub struct BatchCreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::BatchCreateNotesRequest,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> BatchCreateRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::BatchCreateNotesResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::BatchCreateNotesResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/notes:batchCreate");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [NotesActions::create()](struct.NotesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Note,
                parent: String,
                note_id: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> CreateRequestBuilder<'a> {
                #[doc = "Required. The ID to use for this note."]
                pub fn note_id(mut self, value: impl Into<String>) -> Self {
                    self.note_id = Some(value.into());
                    self
                }
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Note, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(self) -> Result<crate::schemas::Note, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/notes");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("noteId", &self.note_id)]);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [NotesActions::delete()](struct.NotesActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> DeleteRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [NotesActions::get()](struct.NotesActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Note, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(self) -> Result<crate::schemas::Note, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [NotesActions::get_iam_policy()](struct.NotesActions.html#method.get_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct GetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GetIamPolicyRequest,
                resource: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetIamPolicyRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":getIamPolicy");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [NotesActions::list()](struct.NotesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The filter expression."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Number of notes to return in the list. Must be positive. Max allowed page size is 1000. If not specified, page size defaults to 20."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Token to provide to skip to a particular spot in the list."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_notes<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_notes_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_notes_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Note> {
                    self.iter_notes_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_notes_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Note> {
                    self.iter_notes_with_fields(Some("*"))
                }
                pub fn iter_notes_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "notes").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "notes")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListNotesResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListNotesResponse>
                {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                    if !fields.is_empty() {
                        match fields.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => fields.push_str(","),
                        }
                        fields.push_str("nextPageToken");
                        self.fields = Some(fields);
                    }
                    crate::iter::PageIter::new(self)
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::ListNotesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListNotesResponse, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/notes");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("pageSize", &self.page_size)]);
                    req = req.query(&[("pageToken", &self.page_token)]);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
            #[doc = "Created via [NotesActions::patch()](struct.NotesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Note,
                name: String,
                update_mask: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> PatchRequestBuilder<'a> {
                #[doc = "The fields to update."]
                pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                    self.update_mask = Some(value.into());
                    self
                }
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Note, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(self) -> Result<crate::schemas::Note, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                    req = req.query(&[("updateMask", &self.update_mask)]);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [NotesActions::set_iam_policy()](struct.NotesActions.html#method.set_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct SetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::SetIamPolicyRequest,
                resource: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> SetIamPolicyRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":setIamPolicy");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [NotesActions::test_iam_permissions()](struct.NotesActions.html#method.test_iam_permissions)"]
            #[derive(Debug, Clone)]
            pub struct TestIamPermissionsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::TestIamPermissionsRequest,
                resource: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> TestIamPermissionsRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":testIamPermissions");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            pub mod occurrences {
                pub mod params {}
                pub struct OccurrencesActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> OccurrencesActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Lists occurrences referencing the specified note. Provider projects can use this method to get all occurrences across consumer projects referencing the specified note."]
                    pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
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
                            name: name.into(),
                            filter: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[doc = "Created via [OccurrencesActions::list()](struct.OccurrencesActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    filter: Option<String>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> ListRequestBuilder<'a> {
                    #[doc = "The filter expression."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "Number of occurrences to return in the list."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Token to provide to skip to a particular spot in the list."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
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
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_occurrences<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_occurrences_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_occurrences_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::Occurrence>
                    {
                        self.iter_occurrences_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_occurrences_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::Occurrence>
                    {
                        self.iter_occurrences_with_fields(Some("*"))
                    }
                    pub fn iter_occurrences_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector = concat!("nextPageToken,", "occurrences").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "occurrences")
                    }
                    pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_with_fields(fields)
                    }
                    pub fn iter_with_default_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListNoteOccurrencesResponse>
                    {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListNoteOccurrencesResponse>
                    {
                        self.iter_with_fields(Some("*"))
                    }
                    pub fn iter_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        let mut fields =
                            fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                        if !fields.is_empty() {
                            match fields.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => fields.push_str(","),
                            }
                            fields.push_str("nextPageToken");
                            self.fields = Some(fields);
                        }
                        crate::iter::PageIter::new(self)
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::ListNoteOccurrencesResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListNoteOccurrencesResponse, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/occurrences");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("filter", &self.filter)]);
                        req = req.query(&[("pageSize", &self.page_size)]);
                        req = req.query(&[("pageToken", &self.page_token)]);
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        self._execute()
                    }
                }
            }
        }
        pub mod occurrences {
            pub mod params {}
            pub struct OccurrencesActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> OccurrencesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates new occurrences in batch."]
                pub fn batch_create(
                    &self,
                    request: crate::schemas::BatchCreateOccurrencesRequest,
                    parent: impl Into<String>,
                ) -> BatchCreateRequestBuilder {
                    BatchCreateRequestBuilder {
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Creates a new occurrence."]
                pub fn create(
                    &self,
                    request: crate::schemas::Occurrence,
                    parent: impl Into<String>,
                ) -> CreateRequestBuilder {
                    CreateRequestBuilder {
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Deletes the specified occurrence. For example, use this method to delete an occurrence when the occurrence is no longer applicable for the given resource."]
                pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                    DeleteRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
                        name: name.into(),
                    }
                }
                #[doc = "Gets the specified occurrence."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
                        name: name.into(),
                    }
                }
                #[doc = "Gets the access control policy for a note or an occurrence resource. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."]
                pub fn get_iam_policy(
                    &self,
                    request: crate::schemas::GetIamPolicyRequest,
                    resource: impl Into<String>,
                ) -> GetIamPolicyRequestBuilder {
                    GetIamPolicyRequestBuilder {
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
                        resource: resource.into(),
                    }
                }
                #[doc = "Gets the note attached to the specified occurrence. Consumer projects can use this method to get a note that belongs to a provider project."]
                pub fn get_notes(&self, name: impl Into<String>) -> GetNotesRequestBuilder {
                    GetNotesRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
                        name: name.into(),
                    }
                }
                #[doc = "Gets a summary of the number and severity of occurrences."]
                pub fn get_vulnerability_summary(
                    &self,
                    parent: impl Into<String>,
                ) -> GetVulnerabilitySummaryRequestBuilder {
                    GetVulnerabilitySummaryRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
                        parent: parent.into(),
                        filter: None,
                    }
                }
                #[doc = "Lists occurrences for the specified project."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
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
                        parent: parent.into(),
                        filter: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates the specified occurrence."]
                pub fn patch(
                    &self,
                    request: crate::schemas::Occurrence,
                    name: impl Into<String>,
                ) -> PatchRequestBuilder {
                    PatchRequestBuilder {
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
                        name: name.into(),
                        update_mask: None,
                    }
                }
                #[doc = "Sets the access control policy on the specified note or occurrence. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or an occurrence, respectively. The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."]
                pub fn set_iam_policy(
                    &self,
                    request: crate::schemas::SetIamPolicyRequest,
                    resource: impl Into<String>,
                ) -> SetIamPolicyRequestBuilder {
                    SetIamPolicyRequestBuilder {
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
                        resource: resource.into(),
                    }
                }
                #[doc = "Returns the permissions that a caller has on the specified note or occurrence. Requires list permission on the project (for example, `containeranalysis.notes.list`). The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for occurrences."]
                pub fn test_iam_permissions(
                    &self,
                    request: crate::schemas::TestIamPermissionsRequest,
                    resource: impl Into<String>,
                ) -> TestIamPermissionsRequestBuilder {
                    TestIamPermissionsRequestBuilder {
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
                        resource: resource.into(),
                    }
                }
            }
            #[doc = "Created via [OccurrencesActions::batch_create()](struct.OccurrencesActions.html#method.batch_create)"]
            #[derive(Debug, Clone)]
            pub struct BatchCreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::BatchCreateOccurrencesRequest,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> BatchCreateRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::BatchCreateOccurrencesResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::BatchCreateOccurrencesResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/occurrences:batchCreate");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [OccurrencesActions::create()](struct.OccurrencesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Occurrence,
                parent: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> CreateRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Occurrence, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Occurrence, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/occurrences");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [OccurrencesActions::delete()](struct.OccurrencesActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> DeleteRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [OccurrencesActions::get()](struct.OccurrencesActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Occurrence, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Occurrence, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [OccurrencesActions::get_iam_policy()](struct.OccurrencesActions.html#method.get_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct GetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GetIamPolicyRequest,
                resource: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetIamPolicyRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":getIamPolicy");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [OccurrencesActions::get_notes()](struct.OccurrencesActions.html#method.get_notes)"]
            #[derive(Debug, Clone)]
            pub struct GetNotesRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetNotesRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Note, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(self) -> Result<crate::schemas::Note, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/notes");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [OccurrencesActions::get_vulnerability_summary()](struct.OccurrencesActions.html#method.get_vulnerability_summary)"]
            #[derive(Debug, Clone)]
            pub struct GetVulnerabilitySummaryRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> GetVulnerabilitySummaryRequestBuilder<'a> {
                #[doc = "The filter expression."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::VulnerabilityOccurrencesSummary, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::VulnerabilityOccurrencesSummary, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/occurrences:vulnerabilitySummary");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [OccurrencesActions::list()](struct.OccurrencesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The filter expression."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Number of occurrences to return in the list. Must be positive. Max allowed page size is 1000. If not specified, page size defaults to 20."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Token to provide to skip to a particular spot in the list."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_occurrences<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_occurrences_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_occurrences_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Occurrence> {
                    self.iter_occurrences_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_occurrences_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Occurrence> {
                    self.iter_occurrences_with_fields(Some("*"))
                }
                pub fn iter_occurrences_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "occurrences").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "occurrences")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListOccurrencesResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListOccurrencesResponse>
                {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                    if !fields.is_empty() {
                        match fields.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => fields.push_str(","),
                        }
                        fields.push_str("nextPageToken");
                        self.fields = Some(fields);
                    }
                    crate::iter::PageIter::new(self)
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::ListOccurrencesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListOccurrencesResponse, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/occurrences");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("pageSize", &self.page_size)]);
                    req = req.query(&[("pageToken", &self.page_token)]);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
            #[doc = "Created via [OccurrencesActions::patch()](struct.OccurrencesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Occurrence,
                name: String,
                update_mask: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> PatchRequestBuilder<'a> {
                #[doc = "The fields to update."]
                pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                    self.update_mask = Some(value.into());
                    self
                }
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Occurrence, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Occurrence, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                    req = req.query(&[("updateMask", &self.update_mask)]);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [OccurrencesActions::set_iam_policy()](struct.OccurrencesActions.html#method.set_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct SetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::SetIamPolicyRequest,
                resource: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> SetIamPolicyRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":setIamPolicy");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [OccurrencesActions::test_iam_permissions()](struct.OccurrencesActions.html#method.test_iam_permissions)"]
            #[derive(Debug, Clone)]
            pub struct TestIamPermissionsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::TestIamPermissionsRequest,
                resource: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a> TestIamPermissionsRequestBuilder<'a> {
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
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
                {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.resource;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":testIamPermissions");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
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
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
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

/// Check the response to see if the status code represents an error. If so
/// convert it into the Reqwest variant of Error.
fn error_from_response(
    response: ::reqwest::blocking::Response,
) -> Result<::reqwest::blocking::Response, Error> {
    match response.error_for_status_ref() {
        Err(reqwest_err) => {
            let body = response.text().ok();
            Err(Error::Reqwest { reqwest_err, body })
        }
        Ok(_) => Ok(response),
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
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
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
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
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
                        let written = body.read(rem_buf)?;
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
            Ok(bytes_written)
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
pub mod iter {
    pub trait IterableMethod {
        fn set_page_token(&mut self, value: String);
        fn execute<T>(&mut self) -> Result<T, crate::Error>
        where
            T: ::serde::de::DeserializeOwned;
    }

    pub struct PageIter<M, T> {
        pub method: M,
        pub finished: bool,
        pub _phantom: ::std::marker::PhantomData<T>,
    }

    impl<M, T> PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M) -> Self {
            PageIter {
                method,
                finished: false,
                _phantom: ::std::marker::PhantomData,
            }
        }
    }

    impl<M, T> Iterator for PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
            if self.finished {
                return None;
            }
            let paginated_result: ::serde_json::Map<String, ::serde_json::Value> =
                match self.method.execute() {
                    Ok(r) => r,
                    Err(err) => return Some(Err(err)),
                };
            if let Some(next_page_token) = paginated_result
                .get("nextPageToken")
                .and_then(|t| t.as_str())
            {
                self.method.set_page_token(next_page_token.to_owned());
            } else {
                self.finished = true;
            }

            Some(
                match ::serde_json::from_value(::serde_json::Value::Object(paginated_result)) {
                    Ok(resp) => Ok(resp),
                    Err(err) => Err(err.into()),
                },
            )
        }
    }

    pub struct PageItemIter<M, T> {
        items_field: &'static str,
        page_iter: PageIter<M, ::serde_json::Map<String, ::serde_json::Value>>,
        items: ::std::vec::IntoIter<T>,
    }

    impl<M, T> PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M, items_field: &'static str) -> Self {
            PageItemIter {
                items_field,
                page_iter: PageIter::new(method),
                items: Vec::new().into_iter(),
            }
        }
    }

    impl<M, T> Iterator for PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
            loop {
                if let Some(v) = self.items.next() {
                    return Some(Ok(v));
                }

                let next_page = self.page_iter.next();
                match next_page {
                    None => return None,
                    Some(Err(err)) => return Some(Err(err)),
                    Some(Ok(next_page)) => {
                        let mut next_page: ::serde_json::Map<String, ::serde_json::Value> =
                            next_page;
                        let items_array = match next_page.remove(self.items_field) {
                            Some(items) => items,
                            None => {
                                return Some(Err(crate::Error::Other(
                                    format!("no {} field found in iter response", self.items_field)
                                        .into(),
                                )))
                            }
                        };
                        let items_vec: Result<Vec<T>, _> = ::serde_json::from_value(items_array);
                        match items_vec {
                            Ok(items) => self.items = items.into_iter(),
                            Err(err) => return Some(Err(err.into())),
                        }
                    }
                }
            }
        }
    }
}
