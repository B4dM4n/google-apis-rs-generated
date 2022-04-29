#![doc = "# Resources and Methods\n* [projects](resources/projects/struct.ProjectsActions.html)\n  * [notes](resources/projects/notes/struct.NotesActions.html)\n    * [*create*](resources/projects/notes/struct.CreateRequestBuilder.html), [*delete*](resources/projects/notes/struct.DeleteRequestBuilder.html), [*get*](resources/projects/notes/struct.GetRequestBuilder.html), [*getIamPolicy*](resources/projects/notes/struct.GetIamPolicyRequestBuilder.html), [*list*](resources/projects/notes/struct.ListRequestBuilder.html), [*patch*](resources/projects/notes/struct.PatchRequestBuilder.html), [*setIamPolicy*](resources/projects/notes/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/projects/notes/struct.TestIamPermissionsRequestBuilder.html)\n    * [occurrences](resources/projects/notes/occurrences/struct.OccurrencesActions.html)\n      * [*list*](resources/projects/notes/occurrences/struct.ListRequestBuilder.html)\n  * [occurrences](resources/projects/occurrences/struct.OccurrencesActions.html)\n    * [*create*](resources/projects/occurrences/struct.CreateRequestBuilder.html), [*delete*](resources/projects/occurrences/struct.DeleteRequestBuilder.html), [*get*](resources/projects/occurrences/struct.GetRequestBuilder.html), [*getIamPolicy*](resources/projects/occurrences/struct.GetIamPolicyRequestBuilder.html), [*getNotes*](resources/projects/occurrences/struct.GetNotesRequestBuilder.html), [*getVulnerabilitySummary*](resources/projects/occurrences/struct.GetVulnerabilitySummaryRequestBuilder.html), [*list*](resources/projects/occurrences/struct.ListRequestBuilder.html), [*patch*](resources/projects/occurrences/struct.PatchRequestBuilder.html), [*setIamPolicy*](resources/projects/occurrences/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/projects/occurrences/struct.TestIamPermissionsRequestBuilder.html)\n  * [operations](resources/projects/operations/struct.OperationsActions.html)\n    * [*create*](resources/projects/operations/struct.CreateRequestBuilder.html), [*patch*](resources/projects/operations/struct.PatchRequestBuilder.html)\n  * [scan_configs](resources/projects/scan_configs/struct.ScanConfigsActions.html)\n    * [*get*](resources/projects/scan_configs/struct.GetRequestBuilder.html), [*list*](resources/projects/scan_configs/struct.ListRequestBuilder.html), [*patch*](resources/projects/scan_configs/struct.PatchRequestBuilder.html)\n* [providers](resources/providers/struct.ProvidersActions.html)\n  * [notes](resources/providers/notes/struct.NotesActions.html)\n    * [*create*](resources/providers/notes/struct.CreateRequestBuilder.html), [*delete*](resources/providers/notes/struct.DeleteRequestBuilder.html), [*get*](resources/providers/notes/struct.GetRequestBuilder.html), [*getIamPolicy*](resources/providers/notes/struct.GetIamPolicyRequestBuilder.html), [*list*](resources/providers/notes/struct.ListRequestBuilder.html), [*patch*](resources/providers/notes/struct.PatchRequestBuilder.html), [*setIamPolicy*](resources/providers/notes/struct.SetIamPolicyRequestBuilder.html), [*testIamPermissions*](resources/providers/notes/struct.TestIamPermissionsRequestBuilder.html)\n    * [occurrences](resources/providers/notes/occurrences/struct.OccurrencesActions.html)\n      * [*list*](resources/providers/notes/occurrences/struct.ListRequestBuilder.html)\n"]
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
    pub struct Artifact {
        #[doc = "Hash or checksum value of a binary, or Docker Registry 2.0 digest of a container."]
        #[serde(
            rename = "checksum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub checksum: ::std::option::Option<String>,
        #[doc = "Artifact ID, if any; for container images, this will be a URL by digest like gcr.io/projectID/imagename@sha256:123456"]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Name of the artifact. This may be the path to a binary or jar file, or in the case of a container build, the name used to push the container image to Google Container Registry, as presented to `docker push`. This field is deprecated in favor of the plural `names` field; it continues to exist here to allow existing BuildProvenance serialized to json in google.devtools.containeranalysis.v1alpha1.BuildDetails.provenance_bytes to deserialize back into proto."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
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
    pub struct Attestation {
        #[serde(
            rename = "pgpSignedAttestation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pgp_signed_attestation: ::std::option::Option<crate::schemas::PgpSignedAttestation>,
    }
    impl ::google_field_selector::FieldSelector for Attestation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Attestation {
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
    pub struct AttestationAuthority {
        #[serde(
            rename = "hint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hint: ::std::option::Option<crate::schemas::AttestationAuthorityHint>,
    }
    impl ::google_field_selector::FieldSelector for AttestationAuthority {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AttestationAuthority {
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
    pub struct AttestationAuthorityHint {
        #[doc = "The human readable name of this Attestation Authority, for example “qa”."]
        #[serde(
            rename = "humanReadableName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub human_readable_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AttestationAuthorityHint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AttestationAuthorityHint {
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
    pub struct Basis {
        #[doc = "The fingerprint of the base image."]
        #[serde(
            rename = "fingerprint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fingerprint: ::std::option::Option<crate::schemas::Fingerprint>,
        #[doc = "The resource_url for the resource representing the basis of associated occurrence images."]
        #[serde(
            rename = "resourceUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Basis {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Basis {
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BuildDetails {
        #[doc = "Deprecated. See InTotoStatement for the replacement. In-toto Provenance representation as defined in spec."]
        #[serde(
            rename = "intotoProvenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub intoto_provenance: ::std::option::Option<crate::schemas::InTotoProvenance>,
        #[doc = "In-toto Statement representation as defined in spec. The intoto_statement can contain any type of provenance. The serialized payload of the statement can be stored and signed in the Occurrence’s envelope."]
        #[serde(
            rename = "intotoStatement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub intoto_statement: ::std::option::Option<crate::schemas::InTotoStatement>,
        #[doc = "The actual provenance"]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance: ::std::option::Option<crate::schemas::BuildProvenance>,
        #[doc = "Serialized JSON representation of the provenance, used in generating the `BuildSignature` in the corresponding Result. After verifying the signature, `provenance_bytes` can be unmarshalled and compared to the provenance to confirm that it is unchanged. A base64-encoded string representation of the provenance bytes is used for the signature in order to interoperate with openssl which expects this format for signature verification. The serialized form is captured both to avoid ambiguity in how the provenance is marshalled to json as well to prevent incompatibilities with future changes."]
        #[serde(
            rename = "provenanceBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance_bytes: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BuildDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BuildDetails {
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
        #[doc = "E-mail address of the user who initiated this build. Note that this was the user’s e-mail address at the time the build was initiated; this address may not represent the same end-user for all time."]
        #[serde(
            rename = "creator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creator: ::std::option::Option<String>,
        #[doc = "Time at which execution of the build was finished."]
        #[serde(
            rename = "finishTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub finish_time: ::std::option::Option<String>,
        #[doc = "Unique identifier of the build."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Google Cloud Storage bucket where logs were written."]
        #[serde(
            rename = "logsBucket",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logs_bucket: ::std::option::Option<String>,
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
    pub struct BuildSignature {
        #[doc = "An Id for the key used to sign. This could be either an Id for the key stored in `public_key` (such as the Id or fingerprint for a PGP key, or the CN for a cert), or a reference to an external key (such as a reference to a key in Cloud Key Management Service)."]
        #[serde(
            rename = "keyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key_id: ::std::option::Option<String>,
        #[doc = "The type of the key, either stored in `public_key` or referenced in `key_id`"]
        #[serde(
            rename = "keyType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key_type: ::std::option::Option<crate::schemas::BuildSignatureKeyType>,
        #[doc = "Public key of the builder which can be used to verify that the related findings are valid and unchanged. If `key_type` is empty, this defaults to PEM encoded public keys. This field may be empty if `key_id` references an external key. For Cloud Build based signatures, this is a PEM encoded public key. To verify the Cloud Build signature, place the contents of this field into a file (public.pem). The signature field is base64-decoded into its binary representation in signature.bin, and the provenance bytes from `BuildDetails` are base64-decoded into a binary representation in signed.bin. OpenSSL can then verify the signature: `openssl sha256 -verify public.pem -signature signature.bin signed.bin`"]
        #[serde(
            rename = "publicKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub public_key: ::std::option::Option<String>,
        #[doc = "Signature of the related `BuildProvenance`, encoded in a base64 string."]
        #[serde(
            rename = "signature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signature: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BuildSignature {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BuildSignature {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BuildSignatureKeyType {
        #[doc = "`KeyType` is not set."]
        KeyTypeUnspecified,
        #[doc = "`PGP ASCII Armored` public key."]
        PgpAsciiArmored,
        #[doc = "`PKIX PEM` public key."]
        PkixPem,
    }
    impl BuildSignatureKeyType {
        pub fn as_str(self) -> &'static str {
            match self {
                BuildSignatureKeyType::KeyTypeUnspecified => "KEY_TYPE_UNSPECIFIED",
                BuildSignatureKeyType::PgpAsciiArmored => "PGP_ASCII_ARMORED",
                BuildSignatureKeyType::PkixPem => "PKIX_PEM",
            }
        }
    }
    impl ::std::convert::AsRef<str> for BuildSignatureKeyType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for BuildSignatureKeyType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<BuildSignatureKeyType, ()> {
            Ok(match s {
                "KEY_TYPE_UNSPECIFIED" => BuildSignatureKeyType::KeyTypeUnspecified,
                "PGP_ASCII_ARMORED" => BuildSignatureKeyType::PgpAsciiArmored,
                "PKIX_PEM" => BuildSignatureKeyType::PkixPem,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for BuildSignatureKeyType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BuildSignatureKeyType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BuildSignatureKeyType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "KEY_TYPE_UNSPECIFIED" => BuildSignatureKeyType::KeyTypeUnspecified,
                "PGP_ASCII_ARMORED" => BuildSignatureKeyType::PgpAsciiArmored,
                "PKIX_PEM" => BuildSignatureKeyType::PkixPem,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for BuildSignatureKeyType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BuildSignatureKeyType {
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
    pub struct BuildType {
        #[doc = "Version of the builder which produced this Note."]
        #[serde(
            rename = "builderVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub builder_version: ::std::option::Option<String>,
        #[doc = "Signature of the build in Occurrences pointing to the Note containing this `BuilderDetails`."]
        #[serde(
            rename = "signature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signature: ::std::option::Option<crate::schemas::BuildSignature>,
    }
    impl ::google_field_selector::FieldSelector for BuildType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BuildType {
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
    pub struct CisBenchmark {
        #[doc = "The profile level of this CIS benchmark check."]
        #[serde(
            rename = "profileLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub profile_level: ::std::option::Option<i32>,
        #[doc = "The severity level of this CIS benchmark check."]
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
        #[doc = "Critical Impact"]
        Critical,
        #[doc = "High Impact"]
        High,
        #[doc = "Low Impact"]
        Low,
        #[doc = "Medium Impact"]
        Medium,
        #[doc = "Minimal Impact"]
        Minimal,
        #[doc = "Unknown Impact"]
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
    pub struct Command {
        #[doc = "Command-line arguments used when executing this Command."]
        #[serde(
            rename = "args",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub args: ::std::option::Option<Vec<String>>,
        #[doc = "Working directory (relative to project source root) used when running this Command."]
        #[serde(
            rename = "dir",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dir: ::std::option::Option<String>,
        #[doc = "Environment variables set before running this Command."]
        #[serde(
            rename = "env",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub env: ::std::option::Option<Vec<String>>,
        #[doc = "Optional unique identifier for this Command, used in wait_for to reference this Command as a dependency."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Name of the command, as presented on the command line, or if the command is packaged as a Docker container, as presented to `docker pull`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The ID(s) of the Command(s) that this Command depends on."]
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
        #[doc = "If true, the builder claims that materials are complete, usually through some controls to prevent network access. Sometimes called “hermetic”."]
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
        #[doc = "Right now we only have one compliance type, but we may add additional types in the future."]
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
        #[doc = "The reason for non compliance of these files."]
        #[serde(
            rename = "nonComplianceReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub non_compliance_reason: ::std::option::Option<String>,
        #[doc = "A list of files which are violating compliance checks."]
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
        #[doc = "A list of images to be pushed upon the successful completion of all build steps. The images will be pushed using the builder service account’s credentials. The digests of the pushed images will be stored in the Build resource’s results field. If any of the images fail to be pushed, the build is marked FAILURE."]
        #[serde(
            rename = "images",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub images: ::std::option::Option<Vec<String>>,
        #[doc = "A list of objects to be uploaded to Cloud Storage upon successful completion of all build steps. Files in the workspace matching specified paths globs will be uploaded to the specified Cloud Storage location using the builder service account’s credentials. The location and generation of the uploaded objects will be stored in the Build resource’s results field. If any objects fail to be pushed, the build is marked FAILURE."]
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
        #[doc = "Cloud Storage bucket and optional object path, in the form “gs://bucket/path/to/somewhere/”. (see [Bucket Name Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)). Files in the workspace matching any path pattern will be uploaded to Cloud Storage with this location as a prefix."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<String>,
        #[doc = "Path globs used to match files in the build’s workspace."]
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
        #[doc = "Output only. Describes this build’s approval configuration, status, and result."]
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
        #[doc = "Output only. Time at which execution of the build was finished. The difference between finish_time and start_time is the duration of the build’s execution."]
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
        #[doc = "A list of images to be pushed upon the successful completion of all build steps. The images are pushed using the builder service account’s credentials. The digests of the pushed images will be stored in the `Build` resource’s results field. If any of the images fail to be pushed, the build status is marked `FAILURE`."]
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
        #[doc = "Output only. The ‘Build’ name with format: `projects/{project}/locations/{location}/builds/{build}`, where {build} is a unique identifier generated by the service."]
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
        #[doc = "Output only. The state of this build’s approval."]
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
    pub struct ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptions { # [doc = "Requested disk size for the VM that runs the build. Note that this is *NOT* “disk free”; some of the space will be used by the operating system and build utilities. Also note that this is the minimum disk size that will be allocated for the build – the build may run with a larger disk than requested. At present, the maximum disk size is 1000GB; builds that request more than the maximum are rejected with an error."] # [serde (rename = "diskSizeGb" , default , skip_serializing_if = "std::option::Option::is_none")] # [serde (with = "crate::parsed_string")] pub disk_size_gb : :: std :: option :: Option < i64 > , # [doc = "Option to specify whether or not to apply bash style string operations to the substitutions. NOTE: this is always enabled for triggered builds and cannot be overridden in the build configuration file."] # [serde (rename = "dynamicSubstitutions" , default , skip_serializing_if = "std::option::Option::is_none")] pub dynamic_substitutions : :: std :: option :: Option < bool > , # [doc = "A list of global environment variable definitions that will exist for all build steps in this build. If a variable is defined in both globally and in a build step, the variable will use the build step value. The elements are of the form “KEY=VALUE” for the environment variable “KEY” being given the value “VALUE”."] # [serde (rename = "env" , default , skip_serializing_if = "std::option::Option::is_none")] pub env : :: std :: option :: Option < Vec < String > > , # [doc = "Option to define build log streaming behavior to Google Cloud Storage."] # [serde (rename = "logStreamingOption" , default , skip_serializing_if = "std::option::Option::is_none")] pub log_streaming_option : :: std :: option :: Option < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogStreamingOption > , # [doc = "Option to specify the logging mode, which determines if and where build logs are stored."] # [serde (rename = "logging" , default , skip_serializing_if = "std::option::Option::is_none")] pub logging : :: std :: option :: Option < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsLogging > , # [doc = "Compute Engine machine type on which to run the build."] # [serde (rename = "machineType" , default , skip_serializing_if = "std::option::Option::is_none")] pub machine_type : :: std :: option :: Option < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsMachineType > , # [doc = "Optional. Specification for execution on a `WorkerPool`. See [running builds in a private pool](https://cloud.google.com/build/docs/private-pools/run-builds-in-private-pool) for more information."] # [serde (rename = "pool" , default , skip_serializing_if = "std::option::Option::is_none")] pub pool : :: std :: option :: Option < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsPoolOption > , # [doc = "Requested verifiability options."] # [serde (rename = "requestedVerifyOption" , default , skip_serializing_if = "std::option::Option::is_none")] pub requested_verify_option : :: std :: option :: Option < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsRequestedVerifyOption > , # [doc = "A list of global environment variables, which are encrypted using a Cloud Key Management Service crypto key. These values must be specified in the build’s `Secret`. These variables will be available to all build steps in this build."] # [serde (rename = "secretEnv" , default , skip_serializing_if = "std::option::Option::is_none")] pub secret_env : :: std :: option :: Option < Vec < String > > , # [doc = "Requested hash for SourceProvenance."] # [serde (rename = "sourceProvenanceHash" , default , skip_serializing_if = "std::option::Option::is_none")] pub source_provenance_hash : :: std :: option :: Option < Vec < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSourceProvenanceHashItems > > , # [doc = "Option to specify behavior when there is an error in the substitution checks. NOTE: this is always set to ALLOW_LOOSE for triggered builds and cannot be overridden in the build configuration file."] # [serde (rename = "substitutionOption" , default , skip_serializing_if = "std::option::Option::is_none")] pub substitution_option : :: std :: option :: Option < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1BuildOptionsSubstitutionOption > , # [doc = "Global list of volumes to mount for ALL build steps Each volume is created as an empty volume prior to starting the build process. Upon completion of the build, volumes and their contents are discarded. Global volume names and paths cannot conflict with the volumes defined a build step. Using a global volume in a build with only one step is not valid as it is indicative of a build request with an incorrect configuration."] # [serde (rename = "volumes" , default , skip_serializing_if = "std::option::Option::is_none")] pub volumes : :: std :: option :: Option < Vec < crate :: schemas :: ContaineranalysisGoogleDevtoolsCloudbuildV1Volume > > , # [doc = "This field deprecated; please use `pool.name` instead."] # [serde (rename = "workerPool" , default , skip_serializing_if = "std::option::Option::is_none")] pub worker_pool : :: std :: option :: Option < String > , }
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
        #[doc = "A list of arguments that will be presented to the step when it is started. If the image used to run the step’s container has an entrypoint, the `args` are used as arguments to that entrypoint. If the image does not define an entrypoint, the first element in args is used as the entrypoint, and the remainder will be used as arguments."]
        #[serde(
            rename = "args",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub args: ::std::option::Option<Vec<String>>,
        #[doc = "Working directory to use when running this step’s container. If this value is a relative path, it is relative to the build’s working directory. If this value is absolute, it may be outside the build’s working directory, in which case the contents of the path may not be persisted across build step executions, unless a `volume` for that path is specified. If the build specifies a `RepoSource` with `dir` and a step with a `dir`, which specifies an absolute path, the `RepoSource` `dir` is ignored for the step’s execution."]
        #[serde(
            rename = "dir",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dir: ::std::option::Option<String>,
        #[doc = "Entrypoint to be used instead of the build step image’s default entrypoint. If unset, the image’s default entrypoint is used."]
        #[serde(
            rename = "entrypoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entrypoint: ::std::option::Option<String>,
        #[doc = "A list of environment variable definitions to be used when running a step. The elements are of the form “KEY=VALUE” for the environment variable “KEY” being given the value “VALUE”."]
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
        #[doc = "Required. The name of the container image that will run this particular build step. If the image is available in the host’s Docker daemon’s cache, it will be run directly. If not, the host will attempt to pull the image first, using the builder service account’s credentials if necessary. The Docker daemon’s cache will already have the latest versions of all of the officially supported build steps ([https://github.com/GoogleCloudPlatform/cloud-builders](https://github.com/GoogleCloudPlatform/cloud-builders)). The Docker daemon will also have cached many of the layers for some popular images, like “ubuntu”, “debian”, but they will be refreshed at the time you attempt to use them. If you built an image in a previous build step, it will be stored in the host’s Docker daemon’s cache and is available to use as the name for a later build step."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Stores timing information for pulling this build step’s builder image only."]
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
        #[doc = "A list of environment variables which are encrypted using a Cloud Key Management Service crypto key. These values must be specified in the build’s `Secret`."]
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
        #[doc = "Map of environment variable name to its encrypted value. Secret environment variables must be unique across all of a build’s secrets, and must be used by at least one build step. Values can be at most 64 KB in size. There can be at most 100 secret values across all of a build’s secrets."]
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
        #[doc = "Directory, relative to the source root, in which to run the build. This must be a relative path. If a step’s `dir` is specified and is an absolute path, this value is ignored for that step’s execution."]
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
        #[doc = "Map of environment variable name to its encrypted value. Secret environment variables must be unique across all of a build’s secrets, and must be used by at least one build step. Values can be at most 64 KB in size. There can be at most 100 secret values across all of a build’s secrets."]
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
        #[doc = "Environment variable name to associate with the secret. Secret environment variables must be unique across all of a build’s secrets, and must be used by at least one build step."]
        #[serde(
            rename = "env",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub env: ::std::option::Option<String>,
        #[doc = "Resource name of the SecretVersion. In format: projects/*/secrets/*/versions/\\*"]
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
        #[doc = "A copy of the build’s `source.repo_source`, if exists, with any revisions resolved."]
        #[serde(
            rename = "resolvedRepoSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolved_repo_source: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1RepoSource,
        >,
        #[doc = "A copy of the build’s `source.storage_source`, if exists, with any generations resolved."]
        #[serde(
            rename = "resolvedStorageSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolved_storage_source: ::std::option::Option<
            crate::schemas::ContaineranalysisGoogleDevtoolsCloudbuildV1StorageSource,
        >,
        #[doc = "A copy of the build’s `source.storage_source_manifest`, if exists, with any revisions resolved. This feature is in Preview."]
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct CreateOperationRequest {
        #[doc = "The operation to create."]
        #[serde(
            rename = "operation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation: ::std::option::Option<crate::schemas::Operation>,
        #[doc = "The ID to use for this operation."]
        #[serde(
            rename = "operationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreateOperationRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreateOperationRequest {
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
    pub struct Deployable {
        #[doc = "Resource URI for the artifact being deployed."]
        #[serde(
            rename = "resourceUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_uri: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Deployable {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Deployable {
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
    pub struct Deployment {
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
        #[doc = "Beginning of the lifetime of this deployment."]
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
        pub platform: ::std::option::Option<crate::schemas::DeploymentPlatform>,
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
    impl ::google_field_selector::FieldSelector for Deployment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Deployment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeploymentPlatform {
        #[doc = "Custom user-defined platform"]
        Custom,
        #[doc = "Google App Engine: Flexible Environment"]
        Flex,
        #[doc = "Google Container Engine"]
        Gke,
        #[doc = "Unknown"]
        PlatformUnspecified,
    }
    impl DeploymentPlatform {
        pub fn as_str(self) -> &'static str {
            match self {
                DeploymentPlatform::Custom => "CUSTOM",
                DeploymentPlatform::Flex => "FLEX",
                DeploymentPlatform::Gke => "GKE",
                DeploymentPlatform::PlatformUnspecified => "PLATFORM_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeploymentPlatform {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeploymentPlatform {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeploymentPlatform, ()> {
            Ok(match s {
                "CUSTOM" => DeploymentPlatform::Custom,
                "FLEX" => DeploymentPlatform::Flex,
                "GKE" => DeploymentPlatform::Gke,
                "PLATFORM_UNSPECIFIED" => DeploymentPlatform::PlatformUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeploymentPlatform {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeploymentPlatform {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeploymentPlatform {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CUSTOM" => DeploymentPlatform::Custom,
                "FLEX" => DeploymentPlatform::Flex,
                "GKE" => DeploymentPlatform::Gke,
                "PLATFORM_UNSPECIFIED" => DeploymentPlatform::PlatformUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeploymentPlatform {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeploymentPlatform {
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
    pub struct Derived {
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
        pub distance: ::std::option::Option<u32>,
        #[doc = "The fingerprint of the derived image."]
        #[serde(
            rename = "fingerprint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fingerprint: ::std::option::Option<crate::schemas::Fingerprint>,
        #[doc = "This contains layer-specific metadata, if populated it has length “distance” and is ordered with \\[distance\\] being the layer immediately following the base image and \\[1\\] being the final layer."]
        #[serde(
            rename = "layerInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layer_info: ::std::option::Option<Vec<crate::schemas::Layer>>,
    }
    impl ::google_field_selector::FieldSelector for Derived {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Derived {
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
        #[doc = "The cpe_uri in \\[cpe format\\] (https://cpe.mitre.org/specification/) in which the vulnerability manifests. Examples include distro or storage location for vulnerable jar. This field can be used as a filter in list requests."]
        #[serde(
            rename = "cpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpe_uri: ::std::option::Option<String>,
        #[doc = "A vendor-specific description of this note."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The fix for this specific package version."]
        #[serde(
            rename = "fixedLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_location: ::std::option::Option<crate::schemas::VulnerabilityLocation>,
        #[doc = "Whether this Detail is obsolete. Occurrences are expected not to point to obsolete details."]
        #[serde(
            rename = "isObsolete",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_obsolete: ::std::option::Option<bool>,
        #[doc = "The max version of the package in which the vulnerability exists."]
        #[serde(
            rename = "maxAffectedVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_affected_version: ::std::option::Option<crate::schemas::Version>,
        #[doc = "The min version of the package in which the vulnerability exists."]
        #[serde(
            rename = "minAffectedVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_affected_version: ::std::option::Option<crate::schemas::Version>,
        #[doc = "The name of the package where the vulnerability was found. This field can be used as a filter in list requests."]
        #[serde(
            rename = "package",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package: ::std::option::Option<String>,
        #[doc = "The type of package; whether native or non native(ruby gems, node.js packages etc)"]
        #[serde(
            rename = "packageType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_type: ::std::option::Option<String>,
        #[doc = "The severity (eg: distro assigned severity) for this vulnerability."]
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
        #[doc = "The vendor of the product. e.g. “google”"]
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Discovered {
        #[doc = "The status of discovery for the resource."]
        #[serde(
            rename = "analysisStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_status: ::std::option::Option<crate::schemas::DiscoveredAnalysisStatus>,
        #[doc = "When an error is encountered this will contain a LocalizedMessage under details to show to the user. The LocalizedMessage output only and populated by the API."]
        #[serde(
            rename = "analysisStatusError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_status_error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "The time occurrences related to this discovery occurrence were archived."]
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
            ::std::option::Option<crate::schemas::DiscoveredContinuousAnalysis>,
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
        #[doc = "Output only. An operation that indicates the status of the current scan. This field is deprecated, do not use."]
        #[serde(
            rename = "operation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation: ::std::option::Option<crate::schemas::Operation>,
    }
    impl ::google_field_selector::FieldSelector for Discovered {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Discovered {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DiscoveredAnalysisStatus {
        #[doc = "Unknown"]
        AnalysisStatusUnspecified,
        #[doc = "Analysis has finished unsuccessfully, the analysis itself is in a bad state."]
        FinishedFailed,
        #[doc = "Analysis has finished successfully."]
        FinishedSuccess,
        #[doc = "The resource is known not to be supported."]
        FinishedUnsupported,
        #[doc = "Resource is known but no action has been taken yet."]
        Pending,
        #[doc = "Resource is being analyzed."]
        Scanning,
    }
    impl DiscoveredAnalysisStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                DiscoveredAnalysisStatus::AnalysisStatusUnspecified => {
                    "ANALYSIS_STATUS_UNSPECIFIED"
                }
                DiscoveredAnalysisStatus::FinishedFailed => "FINISHED_FAILED",
                DiscoveredAnalysisStatus::FinishedSuccess => "FINISHED_SUCCESS",
                DiscoveredAnalysisStatus::FinishedUnsupported => "FINISHED_UNSUPPORTED",
                DiscoveredAnalysisStatus::Pending => "PENDING",
                DiscoveredAnalysisStatus::Scanning => "SCANNING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DiscoveredAnalysisStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DiscoveredAnalysisStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DiscoveredAnalysisStatus, ()> {
            Ok(match s {
                "ANALYSIS_STATUS_UNSPECIFIED" => {
                    DiscoveredAnalysisStatus::AnalysisStatusUnspecified
                }
                "FINISHED_FAILED" => DiscoveredAnalysisStatus::FinishedFailed,
                "FINISHED_SUCCESS" => DiscoveredAnalysisStatus::FinishedSuccess,
                "FINISHED_UNSUPPORTED" => DiscoveredAnalysisStatus::FinishedUnsupported,
                "PENDING" => DiscoveredAnalysisStatus::Pending,
                "SCANNING" => DiscoveredAnalysisStatus::Scanning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DiscoveredAnalysisStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DiscoveredAnalysisStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DiscoveredAnalysisStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANALYSIS_STATUS_UNSPECIFIED" => {
                    DiscoveredAnalysisStatus::AnalysisStatusUnspecified
                }
                "FINISHED_FAILED" => DiscoveredAnalysisStatus::FinishedFailed,
                "FINISHED_SUCCESS" => DiscoveredAnalysisStatus::FinishedSuccess,
                "FINISHED_UNSUPPORTED" => DiscoveredAnalysisStatus::FinishedUnsupported,
                "PENDING" => DiscoveredAnalysisStatus::Pending,
                "SCANNING" => DiscoveredAnalysisStatus::Scanning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DiscoveredAnalysisStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiscoveredAnalysisStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DiscoveredContinuousAnalysis {
        #[doc = "The resource is continuously analyzed."]
        Active,
        #[doc = "Unknown"]
        ContinuousAnalysisUnspecified,
        #[doc = "The resource is ignored for continuous analysis."]
        Inactive,
    }
    impl DiscoveredContinuousAnalysis {
        pub fn as_str(self) -> &'static str {
            match self {
                DiscoveredContinuousAnalysis::Active => "ACTIVE",
                DiscoveredContinuousAnalysis::ContinuousAnalysisUnspecified => {
                    "CONTINUOUS_ANALYSIS_UNSPECIFIED"
                }
                DiscoveredContinuousAnalysis::Inactive => "INACTIVE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DiscoveredContinuousAnalysis {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DiscoveredContinuousAnalysis {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DiscoveredContinuousAnalysis, ()> {
            Ok(match s {
                "ACTIVE" => DiscoveredContinuousAnalysis::Active,
                "CONTINUOUS_ANALYSIS_UNSPECIFIED" => {
                    DiscoveredContinuousAnalysis::ContinuousAnalysisUnspecified
                }
                "INACTIVE" => DiscoveredContinuousAnalysis::Inactive,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DiscoveredContinuousAnalysis {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DiscoveredContinuousAnalysis {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DiscoveredContinuousAnalysis {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => DiscoveredContinuousAnalysis::Active,
                "CONTINUOUS_ANALYSIS_UNSPECIFIED" => {
                    DiscoveredContinuousAnalysis::ContinuousAnalysisUnspecified
                }
                "INACTIVE" => DiscoveredContinuousAnalysis::Inactive,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DiscoveredContinuousAnalysis {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiscoveredContinuousAnalysis {
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
    pub struct Discovery {
        #[doc = "The kind of analysis that is handled by this discovery."]
        #[serde(
            rename = "analysisKind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analysis_kind: ::std::option::Option<crate::schemas::DiscoveryAnalysisKind>,
    }
    impl ::google_field_selector::FieldSelector for Discovery {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Discovery {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DiscoveryAnalysisKind {
        #[doc = "This represents a logical “role” that can attest to artifacts."]
        AttestationAuthority,
        #[doc = "The note and occurrence assert build provenance."]
        BuildDetails,
        #[doc = "This represents a compliance check that can be applied to a resource."]
        Compliance,
        #[doc = "The note and occurrence track deployment events."]
        Deployable,
        #[doc = "The note and occurrence track the initial discovery status of a resource."]
        Discovery,
        #[doc = "This represents a DSSE attestation Note"]
        DsseAttestation,
        #[doc = "This represents an image basis relationship."]
        ImageBasis,
        #[doc = "Unknown"]
        KindUnspecified,
        #[doc = "This represents a package installed via a package manager."]
        PackageManager,
        #[doc = "The note and occurrence represent a package vulnerability."]
        PackageVulnerability,
        #[doc = "This represents a software bill of materials."]
        Sbom,
        #[doc = "This represents an SPDX File."]
        SpdxFile,
        #[doc = "This represents an SPDX Package."]
        SpdxPackage,
        #[doc = "This represents an SPDX Relationship."]
        SpdxRelationship,
        #[doc = "This represents an available software upgrade."]
        Upgrade,
    }
    impl DiscoveryAnalysisKind {
        pub fn as_str(self) -> &'static str {
            match self {
                DiscoveryAnalysisKind::AttestationAuthority => "ATTESTATION_AUTHORITY",
                DiscoveryAnalysisKind::BuildDetails => "BUILD_DETAILS",
                DiscoveryAnalysisKind::Compliance => "COMPLIANCE",
                DiscoveryAnalysisKind::Deployable => "DEPLOYABLE",
                DiscoveryAnalysisKind::Discovery => "DISCOVERY",
                DiscoveryAnalysisKind::DsseAttestation => "DSSE_ATTESTATION",
                DiscoveryAnalysisKind::ImageBasis => "IMAGE_BASIS",
                DiscoveryAnalysisKind::KindUnspecified => "KIND_UNSPECIFIED",
                DiscoveryAnalysisKind::PackageManager => "PACKAGE_MANAGER",
                DiscoveryAnalysisKind::PackageVulnerability => "PACKAGE_VULNERABILITY",
                DiscoveryAnalysisKind::Sbom => "SBOM",
                DiscoveryAnalysisKind::SpdxFile => "SPDX_FILE",
                DiscoveryAnalysisKind::SpdxPackage => "SPDX_PACKAGE",
                DiscoveryAnalysisKind::SpdxRelationship => "SPDX_RELATIONSHIP",
                DiscoveryAnalysisKind::Upgrade => "UPGRADE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DiscoveryAnalysisKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DiscoveryAnalysisKind {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DiscoveryAnalysisKind, ()> {
            Ok(match s {
                "ATTESTATION_AUTHORITY" => DiscoveryAnalysisKind::AttestationAuthority,
                "BUILD_DETAILS" => DiscoveryAnalysisKind::BuildDetails,
                "COMPLIANCE" => DiscoveryAnalysisKind::Compliance,
                "DEPLOYABLE" => DiscoveryAnalysisKind::Deployable,
                "DISCOVERY" => DiscoveryAnalysisKind::Discovery,
                "DSSE_ATTESTATION" => DiscoveryAnalysisKind::DsseAttestation,
                "IMAGE_BASIS" => DiscoveryAnalysisKind::ImageBasis,
                "KIND_UNSPECIFIED" => DiscoveryAnalysisKind::KindUnspecified,
                "PACKAGE_MANAGER" => DiscoveryAnalysisKind::PackageManager,
                "PACKAGE_VULNERABILITY" => DiscoveryAnalysisKind::PackageVulnerability,
                "SBOM" => DiscoveryAnalysisKind::Sbom,
                "SPDX_FILE" => DiscoveryAnalysisKind::SpdxFile,
                "SPDX_PACKAGE" => DiscoveryAnalysisKind::SpdxPackage,
                "SPDX_RELATIONSHIP" => DiscoveryAnalysisKind::SpdxRelationship,
                "UPGRADE" => DiscoveryAnalysisKind::Upgrade,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DiscoveryAnalysisKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DiscoveryAnalysisKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DiscoveryAnalysisKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTESTATION_AUTHORITY" => DiscoveryAnalysisKind::AttestationAuthority,
                "BUILD_DETAILS" => DiscoveryAnalysisKind::BuildDetails,
                "COMPLIANCE" => DiscoveryAnalysisKind::Compliance,
                "DEPLOYABLE" => DiscoveryAnalysisKind::Deployable,
                "DISCOVERY" => DiscoveryAnalysisKind::Discovery,
                "DSSE_ATTESTATION" => DiscoveryAnalysisKind::DsseAttestation,
                "IMAGE_BASIS" => DiscoveryAnalysisKind::ImageBasis,
                "KIND_UNSPECIFIED" => DiscoveryAnalysisKind::KindUnspecified,
                "PACKAGE_MANAGER" => DiscoveryAnalysisKind::PackageManager,
                "PACKAGE_VULNERABILITY" => DiscoveryAnalysisKind::PackageVulnerability,
                "SBOM" => DiscoveryAnalysisKind::Sbom,
                "SPDX_FILE" => DiscoveryAnalysisKind::SpdxFile,
                "SPDX_PACKAGE" => DiscoveryAnalysisKind::SpdxPackage,
                "SPDX_RELATIONSHIP" => DiscoveryAnalysisKind::SpdxRelationship,
                "UPGRADE" => DiscoveryAnalysisKind::Upgrade,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DiscoveryAnalysisKind {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DiscoveryAnalysisKind {
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
        #[doc = "The CPU architecture for which packages in this distribution channel were built"]
        #[serde(
            rename = "architecture",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub architecture: ::std::option::Option<crate::schemas::DistributionArchitecture>,
        #[doc = "The cpe_uri in [cpe format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package."]
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
        #[doc = "Unknown architecture"]
        ArchitectureUnspecified,
        #[doc = "X64 architecture"]
        X64,
        #[doc = "X86 architecture"]
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
    pub struct DocumentNote {
        #[doc = "Compliance with the SPDX specification includes populating the SPDX fields therein with data related to such fields (“SPDX-Metadata”)"]
        #[serde(
            rename = "dataLicence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_licence: ::std::option::Option<String>,
        #[doc = "Provide a reference number that can be used to understand how to parse and interpret the rest of the file"]
        #[serde(
            rename = "spdxVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spdx_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DocumentNote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DocumentNote {
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
    pub struct DocumentOccurrence {
        #[doc = "Identify when the SPDX file was originally created. The date is to be specified according to combined date and time in UTC format as specified in ISO 8601 standard"]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "A field for creators of the SPDX file to provide general comments about the creation of the SPDX file or any other relevant comment not included in the other fields"]
        #[serde(
            rename = "creatorComment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creator_comment: ::std::option::Option<String>,
        #[doc = "Identify who (or what, in the case of a tool) created the SPDX file. If the SPDX file was created by an individual, indicate the person’s name"]
        #[serde(
            rename = "creators",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creators: ::std::option::Option<Vec<String>>,
        #[doc = "A field for creators of the SPDX file content to provide comments to the consumers of the SPDX document"]
        #[serde(
            rename = "documentComment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub document_comment: ::std::option::Option<String>,
        #[doc = "Identify any external SPDX documents referenced within this SPDX document"]
        #[serde(
            rename = "externalDocumentRefs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub external_document_refs: ::std::option::Option<Vec<String>>,
        #[doc = "Identify the current SPDX document which may be referenced in relationships by other files, packages internally and documents externally"]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "A field for creators of the SPDX file to provide the version of the SPDX License List used when the SPDX file was created"]
        #[serde(
            rename = "licenseListVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub license_list_version: ::std::option::Option<String>,
        #[doc = "Provide an SPDX document specific namespace as a unique absolute Uniform Resource Identifier (URI) as specified in RFC-3986, with the exception of the ‘#’ delimiter"]
        #[serde(
            rename = "namespace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub namespace: ::std::option::Option<String>,
        #[doc = "Identify name of this document as designated by creator"]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DocumentOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DocumentOccurrence {
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
        #[doc = "Required. The human readable name of this attestation authority, for example “cloudbuild-prod”."]
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
        #[doc = "The bytes being signed"]
        #[serde(
            rename = "payload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub payload: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The type of payload being signed"]
        #[serde(
            rename = "payloadType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub payload_type: ::std::option::Option<String>,
        #[doc = "The signatures over the payload"]
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
        #[doc = "A reference id to the key being used for signing"]
        #[serde(
            rename = "keyid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keyid: ::std::option::Option<String>,
        #[doc = "The signature itself"]
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
    pub struct ExternalRef {
        #[doc = "An External Reference allows a Package to reference an external source of additional information, metadata, enumerations, asset identifiers, or downloadable content believed to be relevant to the Package"]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<crate::schemas::ExternalRefCategory>,
        #[doc = "Human-readable information about the purpose and target of the reference"]
        #[serde(
            rename = "comment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub comment: ::std::option::Option<String>,
        #[doc = "The unique string with no spaces necessary to access the package-specific information, metadata, or content within the target location"]
        #[serde(
            rename = "locator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locator: ::std::option::Option<String>,
        #[doc = "Type of category (e.g. ‘npm’ for the PACKAGE_MANAGER category)"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ExternalRef {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExternalRef {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ExternalRefCategory {
        #[doc = "Unspecified"]
        CategoryUnspecified,
        #[doc = "Other"]
        Other,
        #[doc = "Package Manager (e.g. maven-central, npm, nuget, bower, purl)"]
        PackageManager,
        #[doc = "Persistent-Id (e.g. swh)"]
        PersistentId,
        #[doc = "Security (e.g. cpe22Type, cpe23Type)"]
        Security,
    }
    impl ExternalRefCategory {
        pub fn as_str(self) -> &'static str {
            match self {
                ExternalRefCategory::CategoryUnspecified => "CATEGORY_UNSPECIFIED",
                ExternalRefCategory::Other => "OTHER",
                ExternalRefCategory::PackageManager => "PACKAGE_MANAGER",
                ExternalRefCategory::PersistentId => "PERSISTENT_ID",
                ExternalRefCategory::Security => "SECURITY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ExternalRefCategory {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ExternalRefCategory {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ExternalRefCategory, ()> {
            Ok(match s {
                "CATEGORY_UNSPECIFIED" => ExternalRefCategory::CategoryUnspecified,
                "OTHER" => ExternalRefCategory::Other,
                "PACKAGE_MANAGER" => ExternalRefCategory::PackageManager,
                "PERSISTENT_ID" => ExternalRefCategory::PersistentId,
                "SECURITY" => ExternalRefCategory::Security,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ExternalRefCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ExternalRefCategory {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExternalRefCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CATEGORY_UNSPECIFIED" => ExternalRefCategory::CategoryUnspecified,
                "OTHER" => ExternalRefCategory::Other,
                "PACKAGE_MANAGER" => ExternalRefCategory::PackageManager,
                "PERSISTENT_ID" => ExternalRefCategory::PersistentId,
                "SECURITY" => ExternalRefCategory::Security,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ExternalRefCategory {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ExternalRefCategory {
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
        #[doc = "Collection of file hashes."]
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
    pub struct FileNote {
        #[doc = "Provide a unique identifier to match analysis information on each specific file in a package"]
        #[serde(
            rename = "checksum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub checksum: ::std::option::Option<Vec<String>>,
        #[doc = "This field provides information about the type of file identified"]
        #[serde(
            rename = "fileType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_type: ::std::option::Option<crate::schemas::FileNoteFileType>,
        #[doc = "Identify the full path and filename that corresponds to the file information in this section"]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FileNote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FileNote {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FileNoteFileType {
        #[doc = "The file is associated with a specific application type (MIME type of application/\\*)"]
        Application,
        #[doc = "The file represents an archive (.tar, .jar, etc.)"]
        Archive,
        #[doc = "The file is associated with an audio file (MIME type of audio/\\* , e.g. .mp3)"]
        Audio,
        #[doc = "The file is a compiled object, target image or binary executable (.o, .a, etc.)"]
        Binary,
        #[doc = "The file serves as documentation"]
        Documentation,
        #[doc = "Unspecified"]
        FileTypeUnspecified,
        #[doc = "The file is associated with an picture image file (MIME type of image/\\*, e.g., .jpg, .gif)"]
        Image,
        #[doc = "The file doesn’t fit into the above categories (generated artifacts, data files, etc.)"]
        Other,
        #[doc = "The file is human readable source code (.c, .html, etc.)"]
        Source,
        #[doc = "The file is an SPDX document"]
        Spdx,
        #[doc = "The file is human readable text file (MIME type of text/\\*)"]
        Text,
        #[doc = "The file is associated with a video file type (MIME type of video/\\*)"]
        Video,
    }
    impl FileNoteFileType {
        pub fn as_str(self) -> &'static str {
            match self {
                FileNoteFileType::Application => "APPLICATION",
                FileNoteFileType::Archive => "ARCHIVE",
                FileNoteFileType::Audio => "AUDIO",
                FileNoteFileType::Binary => "BINARY",
                FileNoteFileType::Documentation => "DOCUMENTATION",
                FileNoteFileType::FileTypeUnspecified => "FILE_TYPE_UNSPECIFIED",
                FileNoteFileType::Image => "IMAGE",
                FileNoteFileType::Other => "OTHER",
                FileNoteFileType::Source => "SOURCE",
                FileNoteFileType::Spdx => "SPDX",
                FileNoteFileType::Text => "TEXT",
                FileNoteFileType::Video => "VIDEO",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FileNoteFileType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FileNoteFileType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FileNoteFileType, ()> {
            Ok(match s {
                "APPLICATION" => FileNoteFileType::Application,
                "ARCHIVE" => FileNoteFileType::Archive,
                "AUDIO" => FileNoteFileType::Audio,
                "BINARY" => FileNoteFileType::Binary,
                "DOCUMENTATION" => FileNoteFileType::Documentation,
                "FILE_TYPE_UNSPECIFIED" => FileNoteFileType::FileTypeUnspecified,
                "IMAGE" => FileNoteFileType::Image,
                "OTHER" => FileNoteFileType::Other,
                "SOURCE" => FileNoteFileType::Source,
                "SPDX" => FileNoteFileType::Spdx,
                "TEXT" => FileNoteFileType::Text,
                "VIDEO" => FileNoteFileType::Video,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FileNoteFileType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FileNoteFileType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FileNoteFileType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPLICATION" => FileNoteFileType::Application,
                "ARCHIVE" => FileNoteFileType::Archive,
                "AUDIO" => FileNoteFileType::Audio,
                "BINARY" => FileNoteFileType::Binary,
                "DOCUMENTATION" => FileNoteFileType::Documentation,
                "FILE_TYPE_UNSPECIFIED" => FileNoteFileType::FileTypeUnspecified,
                "IMAGE" => FileNoteFileType::Image,
                "OTHER" => FileNoteFileType::Other,
                "SOURCE" => FileNoteFileType::Source,
                "SPDX" => FileNoteFileType::Spdx,
                "TEXT" => FileNoteFileType::Text,
                "VIDEO" => FileNoteFileType::Video,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FileNoteFileType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FileNoteFileType {
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
    pub struct FileOccurrence {
        #[doc = "This field provides a place for the SPDX data creator to record, at the file level, acknowledgements that may be needed to be communicated in some contexts"]
        #[serde(
            rename = "attributions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributions: ::std::option::Option<Vec<String>>,
        #[doc = "This field provides a place for the SPDX file creator to record any general comments about the file"]
        #[serde(
            rename = "comment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub comment: ::std::option::Option<String>,
        #[doc = "This field provides a place for the SPDX file creator to record file contributors"]
        #[serde(
            rename = "contributors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contributors: ::std::option::Option<Vec<String>>,
        #[doc = "Identify the copyright holder of the file, as well as any dates present"]
        #[serde(
            rename = "copyright",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub copyright: ::std::option::Option<String>,
        #[doc = "This field contains the license information actually found in the file, if any"]
        #[serde(
            rename = "filesLicenseInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub files_license_info: ::std::option::Option<Vec<String>>,
        #[doc = "Uniquely identify any element in an SPDX document which may be referenced by other elements"]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "This field contains the license the SPDX file creator has concluded as governing the file or alternative values if the governing license cannot be determined"]
        #[serde(
            rename = "licenseConcluded",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub license_concluded: ::std::option::Option<crate::schemas::License>,
        #[doc = "This field provides a place for the SPDX file creator to record license notices or other such related notices found in the file"]
        #[serde(
            rename = "notice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub notice: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FileOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FileOccurrence {
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
        #[doc = "The layer-id of the final layer in the Docker image’s v1 representation. This field can be used as a filter in list requests."]
        #[serde(
            rename = "v1Name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub v_1_name: ::std::option::Option<String>,
        #[doc = "The ordered list of v2 blobs that represent a given image."]
        #[serde(
            rename = "v2Blob",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub v_2_blob: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. The name of the image’s v2 blobs computed via: \\[bottom\\] := v2_blobbottom := sha256(v2_blob\\[N\\] + “ “ + v2_name\\[N+1\\]) Only the name of the final blob is kept. This field can be used as a filter in list requests."]
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
    pub struct GetVulnzOccurrencesSummaryResponse {
        #[doc = "A map of how many occurrences were found for each severity."]
        #[serde(
            rename = "counts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub counts: ::std::option::Option<Vec<crate::schemas::SeverityCount>>,
    }
    impl ::google_field_selector::FieldSelector for GetVulnzOccurrencesSummaryResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetVulnzOccurrencesSummaryResponse {
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
    pub struct GoogleDevtoolsContaineranalysisV1Alpha1AliasContext {
        #[doc = "The alias kind."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<
            crate::schemas::GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind,
        >,
        #[doc = "The alias name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleDevtoolsContaineranalysisV1Alpha1AliasContext
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDevtoolsContaineranalysisV1Alpha1AliasContext {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind {
        #[doc = "Git tag."]
        Fixed,
        #[doc = "Unknown."]
        KindUnspecified,
        #[doc = "Git branch."]
        Movable,
        #[doc = "Used to specify non-standard aliases. For example, if a Git repo has a ref named “refs/foo/bar”."]
        Other,
    }
    impl GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind::Fixed => "FIXED",
                GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind::KindUnspecified => {
                    "KIND_UNSPECIFIED"
                }
                GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind::Movable => "MOVABLE",
                GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind::Other => "OTHER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind, ()>
        {
            Ok(match s {
                "FIXED" => GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind::Fixed,
                "KIND_UNSPECIFIED" => {
                    GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind::KindUnspecified
                }
                "MOVABLE" => GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind::Movable,
                "OTHER" => GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind::Other,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FIXED" => GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind::Fixed,
                "KIND_UNSPECIFIED" => {
                    GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind::KindUnspecified
                }
                "MOVABLE" => GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind::Movable,
                "OTHER" => GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind::Other,
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
        for GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleDevtoolsContaineranalysisV1Alpha1AliasContextKind
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
    pub struct GoogleDevtoolsContaineranalysisV1Alpha1CloudRepoSourceContext {
        #[doc = "An alias, which may be a branch or tag."]
        #[serde(
            rename = "aliasContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alias_context: ::std::option::Option<
            crate::schemas::GoogleDevtoolsContaineranalysisV1Alpha1AliasContext,
        >,
        #[doc = "The ID of the repo."]
        #[serde(
            rename = "repoId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repo_id:
            ::std::option::Option<crate::schemas::GoogleDevtoolsContaineranalysisV1Alpha1RepoId>,
        #[doc = "A revision ID."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleDevtoolsContaineranalysisV1Alpha1CloudRepoSourceContext
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleDevtoolsContaineranalysisV1Alpha1CloudRepoSourceContext
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
    pub struct GoogleDevtoolsContaineranalysisV1Alpha1GerritSourceContext {
        #[doc = "An alias, which may be a branch or tag."]
        #[serde(
            rename = "aliasContext",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alias_context: ::std::option::Option<
            crate::schemas::GoogleDevtoolsContaineranalysisV1Alpha1AliasContext,
        >,
        #[doc = "The full project name within the host. Projects may be nested, so “project/subproject” is a valid project name. The “repo name” is the hostURI/project."]
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
    impl ::google_field_selector::FieldSelector
        for GoogleDevtoolsContaineranalysisV1Alpha1GerritSourceContext
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleDevtoolsContaineranalysisV1Alpha1GerritSourceContext
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
    pub struct GoogleDevtoolsContaineranalysisV1Alpha1GitSourceContext {
        #[doc = "Required. Git commit hash."]
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
    impl ::google_field_selector::FieldSelector
        for GoogleDevtoolsContaineranalysisV1Alpha1GitSourceContext
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleDevtoolsContaineranalysisV1Alpha1GitSourceContext
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
    pub struct GoogleDevtoolsContaineranalysisV1Alpha1ProjectRepoId {
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
    impl ::google_field_selector::FieldSelector
        for GoogleDevtoolsContaineranalysisV1Alpha1ProjectRepoId
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDevtoolsContaineranalysisV1Alpha1ProjectRepoId {
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
    pub struct GoogleDevtoolsContaineranalysisV1Alpha1RepoId {
        #[doc = "A combination of a project ID and a repo name."]
        #[serde(
            rename = "projectRepoId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_repo_id: ::std::option::Option<
            crate::schemas::GoogleDevtoolsContaineranalysisV1Alpha1ProjectRepoId,
        >,
        #[doc = "A server-assigned, globally unique identifier."]
        #[serde(
            rename = "uid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uid: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDevtoolsContaineranalysisV1Alpha1RepoId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDevtoolsContaineranalysisV1Alpha1RepoId {
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
    pub struct GoogleDevtoolsContaineranalysisV1Alpha1SourceContext {
        #[doc = "A SourceContext referring to a revision in a Google Cloud Source Repo."]
        #[serde(
            rename = "cloudRepo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_repo: ::std::option::Option<
            crate::schemas::GoogleDevtoolsContaineranalysisV1Alpha1CloudRepoSourceContext,
        >,
        #[doc = "A SourceContext referring to a Gerrit project."]
        #[serde(
            rename = "gerrit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gerrit: ::std::option::Option<
            crate::schemas::GoogleDevtoolsContaineranalysisV1Alpha1GerritSourceContext,
        >,
        #[doc = "A SourceContext referring to any third party Git repo (e.g., GitHub)."]
        #[serde(
            rename = "git",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub git: ::std::option::Option<
            crate::schemas::GoogleDevtoolsContaineranalysisV1Alpha1GitSourceContext,
        >,
        #[doc = "Labels with user defined metadata."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleDevtoolsContaineranalysisV1Alpha1SourceContext
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDevtoolsContaineranalysisV1Alpha1SourceContext {
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
        #[doc = "The type of hash that was performed."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::HashType>,
        #[doc = "The hash value."]
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum HashType {
        #[doc = "No hash requested."]
        None,
        #[doc = "A sha256 hash."]
        Sha256,
    }
    impl HashType {
        pub fn as_str(self) -> &'static str {
            match self {
                HashType::None => "NONE",
                HashType::Sha256 => "SHA256",
            }
        }
    }
    impl ::std::convert::AsRef<str> for HashType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for HashType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<HashType, ()> {
            Ok(match s {
                "NONE" => HashType::None,
                "SHA256" => HashType::Sha256,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for HashType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for HashType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for HashType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => HashType::None,
                "SHA256" => HashType::Sha256,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for HashType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HashType {
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
        #[doc = "“https://slsa.dev/provenance/v0.1” for SlsaProvenance."]
        #[serde(
            rename = "predicateType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub predicate_type: ::std::option::Option<String>,
        #[doc = "provenance is a predicate of type intotoprovenance"]
        #[serde(
            rename = "provenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provenance: ::std::option::Option<crate::schemas::InTotoProvenance>,
        #[doc = "Always “https://in-toto.io/Statement/v0.1”."]
        #[serde(
            rename = "_type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "slsa_provenance is a predicate of type slsaProvenance"]
        #[serde(
            rename = "slsaProvenance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slsa_provenance: ::std::option::Option<crate::schemas::SlsaProvenance>,
        #[doc = "subject is the subjects of the intoto statement"]
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
    pub struct Installation {
        #[doc = "All of the places within the filesystem versions of this package have been found."]
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
    impl ::google_field_selector::FieldSelector for Installation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Installation {
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
        #[doc = "The recovered Dockerfile directive used to construct this layer."]
        #[serde(
            rename = "directive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub directive: ::std::option::Option<crate::schemas::LayerDirective>,
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LayerDirective {
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Add,
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Arg,
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Cmd,
        #[doc = "https://docs.docker.com/reference/builder/#copy"]
        Copy,
        #[doc = "Default value for unsupported/missing directive"]
        DirectiveUnspecified,
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Entrypoint,
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Env,
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Expose,
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Healthcheck,
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Label,
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Maintainer,
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Onbuild,
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Run,
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Shell,
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Stopsignal,
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        User,
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Volume,
        #[doc = "https://docs.docker.com/engine/reference/builder/"]
        Workdir,
    }
    impl LayerDirective {
        pub fn as_str(self) -> &'static str {
            match self {
                LayerDirective::Add => "ADD",
                LayerDirective::Arg => "ARG",
                LayerDirective::Cmd => "CMD",
                LayerDirective::Copy => "COPY",
                LayerDirective::DirectiveUnspecified => "DIRECTIVE_UNSPECIFIED",
                LayerDirective::Entrypoint => "ENTRYPOINT",
                LayerDirective::Env => "ENV",
                LayerDirective::Expose => "EXPOSE",
                LayerDirective::Healthcheck => "HEALTHCHECK",
                LayerDirective::Label => "LABEL",
                LayerDirective::Maintainer => "MAINTAINER",
                LayerDirective::Onbuild => "ONBUILD",
                LayerDirective::Run => "RUN",
                LayerDirective::Shell => "SHELL",
                LayerDirective::Stopsignal => "STOPSIGNAL",
                LayerDirective::User => "USER",
                LayerDirective::Volume => "VOLUME",
                LayerDirective::Workdir => "WORKDIR",
            }
        }
    }
    impl ::std::convert::AsRef<str> for LayerDirective {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for LayerDirective {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<LayerDirective, ()> {
            Ok(match s {
                "ADD" => LayerDirective::Add,
                "ARG" => LayerDirective::Arg,
                "CMD" => LayerDirective::Cmd,
                "COPY" => LayerDirective::Copy,
                "DIRECTIVE_UNSPECIFIED" => LayerDirective::DirectiveUnspecified,
                "ENTRYPOINT" => LayerDirective::Entrypoint,
                "ENV" => LayerDirective::Env,
                "EXPOSE" => LayerDirective::Expose,
                "HEALTHCHECK" => LayerDirective::Healthcheck,
                "LABEL" => LayerDirective::Label,
                "MAINTAINER" => LayerDirective::Maintainer,
                "ONBUILD" => LayerDirective::Onbuild,
                "RUN" => LayerDirective::Run,
                "SHELL" => LayerDirective::Shell,
                "STOPSIGNAL" => LayerDirective::Stopsignal,
                "USER" => LayerDirective::User,
                "VOLUME" => LayerDirective::Volume,
                "WORKDIR" => LayerDirective::Workdir,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for LayerDirective {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LayerDirective {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LayerDirective {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADD" => LayerDirective::Add,
                "ARG" => LayerDirective::Arg,
                "CMD" => LayerDirective::Cmd,
                "COPY" => LayerDirective::Copy,
                "DIRECTIVE_UNSPECIFIED" => LayerDirective::DirectiveUnspecified,
                "ENTRYPOINT" => LayerDirective::Entrypoint,
                "ENV" => LayerDirective::Env,
                "EXPOSE" => LayerDirective::Expose,
                "HEALTHCHECK" => LayerDirective::Healthcheck,
                "LABEL" => LayerDirective::Label,
                "MAINTAINER" => LayerDirective::Maintainer,
                "ONBUILD" => LayerDirective::Onbuild,
                "RUN" => LayerDirective::Run,
                "SHELL" => LayerDirective::Shell,
                "STOPSIGNAL" => LayerDirective::Stopsignal,
                "USER" => LayerDirective::User,
                "VOLUME" => LayerDirective::Volume,
                "WORKDIR" => LayerDirective::Workdir,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for LayerDirective {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LayerDirective {
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
    pub struct License {
        #[doc = "Comments"]
        #[serde(
            rename = "comments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub comments: ::std::option::Option<String>,
        #[doc = "Expression: https://spdx.github.io/spdx-spec/appendix-IV-SPDX-license-expressions/"]
        #[serde(
            rename = "expression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expression: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for License {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for License {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListNoteOccurrencesResponse {
        #[doc = "Token to receive the next page of notes."]
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
    impl crate::GetNextPageToken for ListNoteOccurrencesResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListNotesResponse {
        #[doc = "The next pagination token in the list response. It should be used as page_token for the following request. An empty value means no more result."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The occurrences requested"]
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
    impl crate::GetNextPageToken for ListNotesResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
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
    impl crate::GetNextPageToken for ListOccurrencesResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
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
    pub struct ListScanConfigsResponse {
        #[doc = "A page token to pass in order to get more scan configs."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The set of scan configs."]
        #[serde(
            rename = "scanConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scan_configs: ::std::option::Option<Vec<crate::schemas::ScanConfig>>,
    }
    impl ::google_field_selector::FieldSelector for ListScanConfigsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListScanConfigsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListScanConfigsResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
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
        #[doc = "The cpe_uri in [cpe format](https://cpe.mitre.org/specification/) denoting the package manager version distributing a package."]
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
        #[doc = "digest is a map from a hash algorithm (e.g. sha256) to the value in the material"]
        #[serde(
            rename = "digest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub digest: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "uri is the uri of the material"]
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
        #[doc = "display_command is a single command that can be used to display a list of non compliant files. When there is no such command, we can also iterate a list of non compliant file using ‘path’. Empty if `display_command` is set."]
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
            rename = "attestationAuthority",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attestation_authority: ::std::option::Option<crate::schemas::AttestationAuthority>,
        #[doc = "A note describing a base image."]
        #[serde(
            rename = "baseImage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub base_image: ::std::option::Option<crate::schemas::Basis>,
        #[doc = "Build provenance type for a verifiable build."]
        #[serde(
            rename = "buildType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_type: ::std::option::Option<crate::schemas::BuildType>,
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
            rename = "deployable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deployable: ::std::option::Option<crate::schemas::Deployable>,
        #[doc = "A note describing a provider/analysis type."]
        #[serde(
            rename = "discovery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discovery: ::std::option::Option<crate::schemas::Discovery>,
        #[doc = "A note describing a dsse attestation note."]
        #[serde(
            rename = "dsseAttestation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dsse_attestation: ::std::option::Option<crate::schemas::DsseattestationNote>,
        #[doc = "Time of expiration for this note, null if note does not expire."]
        #[serde(
            rename = "expirationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expiration_time: ::std::option::Option<String>,
        #[doc = "Output only. This explicitly denotes which kind of note is specified. This field can be used as a filter in list requests."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::NoteKind>,
        #[doc = "A detailed description of this `Note`."]
        #[serde(
            rename = "longDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub long_description: ::std::option::Option<String>,
        #[doc = "The name of the note in the form “projects/{provider_project_id}/notes/{NOTE_ID}”"]
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
        pub package: ::std::option::Option<crate::schemas::Package>,
        #[doc = "URLs associated with this note"]
        #[serde(
            rename = "relatedUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_url: ::std::option::Option<Vec<crate::schemas::RelatedUrl>>,
        #[doc = "A note describing a software bill of materials."]
        #[serde(
            rename = "sbom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sbom: ::std::option::Option<crate::schemas::DocumentNote>,
        #[doc = "A one sentence description of this `Note`."]
        #[serde(
            rename = "shortDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub short_description: ::std::option::Option<String>,
        #[doc = "A note describing an SPDX File."]
        #[serde(
            rename = "spdxFile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spdx_file: ::std::option::Option<crate::schemas::FileNote>,
        #[doc = "A note describing an SPDX Package."]
        #[serde(
            rename = "spdxPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spdx_package: ::std::option::Option<crate::schemas::PackageInfoNote>,
        #[doc = "A note describing a relationship between SPDX elements."]
        #[serde(
            rename = "spdxRelationship",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spdx_relationship: ::std::option::Option<crate::schemas::RelationshipNote>,
        #[doc = "Output only. The time this note was last updated. This field can be used as a filter in list requests."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
        #[doc = "A note describing an upgrade."]
        #[serde(
            rename = "upgrade",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upgrade: ::std::option::Option<crate::schemas::UpgradeNote>,
        #[doc = "A package vulnerability type of note."]
        #[serde(
            rename = "vulnerabilityType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vulnerability_type: ::std::option::Option<crate::schemas::VulnerabilityType>,
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
        #[doc = "This represents a logical “role” that can attest to artifacts."]
        AttestationAuthority,
        #[doc = "The note and occurrence assert build provenance."]
        BuildDetails,
        #[doc = "This represents a compliance check that can be applied to a resource."]
        Compliance,
        #[doc = "The note and occurrence track deployment events."]
        Deployable,
        #[doc = "The note and occurrence track the initial discovery status of a resource."]
        Discovery,
        #[doc = "This represents a DSSE attestation Note"]
        DsseAttestation,
        #[doc = "This represents an image basis relationship."]
        ImageBasis,
        #[doc = "Unknown"]
        KindUnspecified,
        #[doc = "This represents a package installed via a package manager."]
        PackageManager,
        #[doc = "The note and occurrence represent a package vulnerability."]
        PackageVulnerability,
        #[doc = "This represents a software bill of materials."]
        Sbom,
        #[doc = "This represents an SPDX File."]
        SpdxFile,
        #[doc = "This represents an SPDX Package."]
        SpdxPackage,
        #[doc = "This represents an SPDX Relationship."]
        SpdxRelationship,
        #[doc = "This represents an available software upgrade."]
        Upgrade,
    }
    impl NoteKind {
        pub fn as_str(self) -> &'static str {
            match self {
                NoteKind::AttestationAuthority => "ATTESTATION_AUTHORITY",
                NoteKind::BuildDetails => "BUILD_DETAILS",
                NoteKind::Compliance => "COMPLIANCE",
                NoteKind::Deployable => "DEPLOYABLE",
                NoteKind::Discovery => "DISCOVERY",
                NoteKind::DsseAttestation => "DSSE_ATTESTATION",
                NoteKind::ImageBasis => "IMAGE_BASIS",
                NoteKind::KindUnspecified => "KIND_UNSPECIFIED",
                NoteKind::PackageManager => "PACKAGE_MANAGER",
                NoteKind::PackageVulnerability => "PACKAGE_VULNERABILITY",
                NoteKind::Sbom => "SBOM",
                NoteKind::SpdxFile => "SPDX_FILE",
                NoteKind::SpdxPackage => "SPDX_PACKAGE",
                NoteKind::SpdxRelationship => "SPDX_RELATIONSHIP",
                NoteKind::Upgrade => "UPGRADE",
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
                "ATTESTATION_AUTHORITY" => NoteKind::AttestationAuthority,
                "BUILD_DETAILS" => NoteKind::BuildDetails,
                "COMPLIANCE" => NoteKind::Compliance,
                "DEPLOYABLE" => NoteKind::Deployable,
                "DISCOVERY" => NoteKind::Discovery,
                "DSSE_ATTESTATION" => NoteKind::DsseAttestation,
                "IMAGE_BASIS" => NoteKind::ImageBasis,
                "KIND_UNSPECIFIED" => NoteKind::KindUnspecified,
                "PACKAGE_MANAGER" => NoteKind::PackageManager,
                "PACKAGE_VULNERABILITY" => NoteKind::PackageVulnerability,
                "SBOM" => NoteKind::Sbom,
                "SPDX_FILE" => NoteKind::SpdxFile,
                "SPDX_PACKAGE" => NoteKind::SpdxPackage,
                "SPDX_RELATIONSHIP" => NoteKind::SpdxRelationship,
                "UPGRADE" => NoteKind::Upgrade,
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
                "ATTESTATION_AUTHORITY" => NoteKind::AttestationAuthority,
                "BUILD_DETAILS" => NoteKind::BuildDetails,
                "COMPLIANCE" => NoteKind::Compliance,
                "DEPLOYABLE" => NoteKind::Deployable,
                "DISCOVERY" => NoteKind::Discovery,
                "DSSE_ATTESTATION" => NoteKind::DsseAttestation,
                "IMAGE_BASIS" => NoteKind::ImageBasis,
                "KIND_UNSPECIFIED" => NoteKind::KindUnspecified,
                "PACKAGE_MANAGER" => NoteKind::PackageManager,
                "PACKAGE_VULNERABILITY" => NoteKind::PackageVulnerability,
                "SBOM" => NoteKind::Sbom,
                "SPDX_FILE" => NoteKind::SpdxFile,
                "SPDX_PACKAGE" => NoteKind::SpdxPackage,
                "SPDX_RELATIONSHIP" => NoteKind::SpdxRelationship,
                "UPGRADE" => NoteKind::Upgrade,
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
        pub attestation: ::std::option::Option<crate::schemas::Attestation>,
        #[doc = "Build details for a verifiable build."]
        #[serde(
            rename = "buildDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub build_details: ::std::option::Option<crate::schemas::BuildDetails>,
        #[doc = "Describes whether or not a resource passes compliance checks."]
        #[serde(
            rename = "compliance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compliance: ::std::option::Option<crate::schemas::ComplianceOccurrence>,
        #[doc = "Output only. The time this `Occurrence` was created."]
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
        pub deployment: ::std::option::Option<crate::schemas::Deployment>,
        #[doc = "Describes how this resource derives from the basis in the associated note."]
        #[serde(
            rename = "derivedImage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub derived_image: ::std::option::Option<crate::schemas::Derived>,
        #[doc = "Describes the initial scan status for this resource."]
        #[serde(
            rename = "discovered",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discovered: ::std::option::Option<crate::schemas::Discovered>,
        #[doc = "This represents a DSSE attestation occurrence"]
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
        #[doc = "Describes the installation of a package on the linked resource."]
        #[serde(
            rename = "installation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub installation: ::std::option::Option<crate::schemas::Installation>,
        #[doc = "Output only. This explicitly denotes which of the `Occurrence` details are specified. This field can be used as a filter in list requests."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::OccurrenceKind>,
        #[doc = "Output only. The name of the `Occurrence` in the form “projects/{project_id}/occurrences/{OCCURRENCE_ID}”"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "An analysis note associated with this image, in the form “providers/{provider_id}/notes/{NOTE_ID}” This field can be used as a filter in list requests."]
        #[serde(
            rename = "noteName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub note_name: ::std::option::Option<String>,
        #[doc = "A description of actions that can be taken to remedy the `Note`"]
        #[serde(
            rename = "remediation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub remediation: ::std::option::Option<String>,
        #[doc = "The resource for which the `Occurrence` applies."]
        #[serde(
            rename = "resource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource: ::std::option::Option<crate::schemas::Resource>,
        #[doc = "The unique URL of the image or the container for which the `Occurrence` applies. For example, https://gcr.io/project/image@sha256:foo This field can be used as a filter in list requests."]
        #[serde(
            rename = "resourceUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_url: ::std::option::Option<String>,
        #[doc = "Describes a specific software bill of materials document."]
        #[serde(
            rename = "sbom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sbom: ::std::option::Option<crate::schemas::DocumentOccurrence>,
        #[doc = "Describes a specific SPDX File."]
        #[serde(
            rename = "spdxFile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spdx_file: ::std::option::Option<crate::schemas::FileOccurrence>,
        #[doc = "Describes a specific SPDX Package."]
        #[serde(
            rename = "spdxPackage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spdx_package: ::std::option::Option<crate::schemas::PackageInfoOccurrence>,
        #[doc = "Describes a specific relationship between SPDX elements."]
        #[serde(
            rename = "spdxRelationship",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spdx_relationship: ::std::option::Option<crate::schemas::RelationshipOccurrence>,
        #[doc = "Output only. The time this `Occurrence` was last updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
        #[doc = "Describes an upgrade."]
        #[serde(
            rename = "upgrade",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upgrade: ::std::option::Option<crate::schemas::UpgradeOccurrence>,
        #[doc = "Details of a security vulnerability note."]
        #[serde(
            rename = "vulnerabilityDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vulnerability_details: ::std::option::Option<crate::schemas::VulnerabilityDetails>,
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
        #[doc = "This represents a logical “role” that can attest to artifacts."]
        AttestationAuthority,
        #[doc = "The note and occurrence assert build provenance."]
        BuildDetails,
        #[doc = "This represents a compliance check that can be applied to a resource."]
        Compliance,
        #[doc = "The note and occurrence track deployment events."]
        Deployable,
        #[doc = "The note and occurrence track the initial discovery status of a resource."]
        Discovery,
        #[doc = "This represents a DSSE attestation Note"]
        DsseAttestation,
        #[doc = "This represents an image basis relationship."]
        ImageBasis,
        #[doc = "Unknown"]
        KindUnspecified,
        #[doc = "This represents a package installed via a package manager."]
        PackageManager,
        #[doc = "The note and occurrence represent a package vulnerability."]
        PackageVulnerability,
        #[doc = "This represents a software bill of materials."]
        Sbom,
        #[doc = "This represents an SPDX File."]
        SpdxFile,
        #[doc = "This represents an SPDX Package."]
        SpdxPackage,
        #[doc = "This represents an SPDX Relationship."]
        SpdxRelationship,
        #[doc = "This represents an available software upgrade."]
        Upgrade,
    }
    impl OccurrenceKind {
        pub fn as_str(self) -> &'static str {
            match self {
                OccurrenceKind::AttestationAuthority => "ATTESTATION_AUTHORITY",
                OccurrenceKind::BuildDetails => "BUILD_DETAILS",
                OccurrenceKind::Compliance => "COMPLIANCE",
                OccurrenceKind::Deployable => "DEPLOYABLE",
                OccurrenceKind::Discovery => "DISCOVERY",
                OccurrenceKind::DsseAttestation => "DSSE_ATTESTATION",
                OccurrenceKind::ImageBasis => "IMAGE_BASIS",
                OccurrenceKind::KindUnspecified => "KIND_UNSPECIFIED",
                OccurrenceKind::PackageManager => "PACKAGE_MANAGER",
                OccurrenceKind::PackageVulnerability => "PACKAGE_VULNERABILITY",
                OccurrenceKind::Sbom => "SBOM",
                OccurrenceKind::SpdxFile => "SPDX_FILE",
                OccurrenceKind::SpdxPackage => "SPDX_PACKAGE",
                OccurrenceKind::SpdxRelationship => "SPDX_RELATIONSHIP",
                OccurrenceKind::Upgrade => "UPGRADE",
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
                "ATTESTATION_AUTHORITY" => OccurrenceKind::AttestationAuthority,
                "BUILD_DETAILS" => OccurrenceKind::BuildDetails,
                "COMPLIANCE" => OccurrenceKind::Compliance,
                "DEPLOYABLE" => OccurrenceKind::Deployable,
                "DISCOVERY" => OccurrenceKind::Discovery,
                "DSSE_ATTESTATION" => OccurrenceKind::DsseAttestation,
                "IMAGE_BASIS" => OccurrenceKind::ImageBasis,
                "KIND_UNSPECIFIED" => OccurrenceKind::KindUnspecified,
                "PACKAGE_MANAGER" => OccurrenceKind::PackageManager,
                "PACKAGE_VULNERABILITY" => OccurrenceKind::PackageVulnerability,
                "SBOM" => OccurrenceKind::Sbom,
                "SPDX_FILE" => OccurrenceKind::SpdxFile,
                "SPDX_PACKAGE" => OccurrenceKind::SpdxPackage,
                "SPDX_RELATIONSHIP" => OccurrenceKind::SpdxRelationship,
                "UPGRADE" => OccurrenceKind::Upgrade,
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
                "ATTESTATION_AUTHORITY" => OccurrenceKind::AttestationAuthority,
                "BUILD_DETAILS" => OccurrenceKind::BuildDetails,
                "COMPLIANCE" => OccurrenceKind::Compliance,
                "DEPLOYABLE" => OccurrenceKind::Deployable,
                "DISCOVERY" => OccurrenceKind::Discovery,
                "DSSE_ATTESTATION" => OccurrenceKind::DsseAttestation,
                "IMAGE_BASIS" => OccurrenceKind::ImageBasis,
                "KIND_UNSPECIFIED" => OccurrenceKind::KindUnspecified,
                "PACKAGE_MANAGER" => OccurrenceKind::PackageManager,
                "PACKAGE_VULNERABILITY" => OccurrenceKind::PackageVulnerability,
                "SBOM" => OccurrenceKind::Sbom,
                "SPDX_FILE" => OccurrenceKind::SpdxFile,
                "SPDX_PACKAGE" => OccurrenceKind::SpdxPackage,
                "SPDX_RELATIONSHIP" => OccurrenceKind::SpdxRelationship,
                "UPGRADE" => OccurrenceKind::Upgrade,
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Operation {
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        #[serde(
            rename = "done",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub done: ::std::option::Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
        #[serde(
            rename = "response",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for Operation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Operation {
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
    pub struct Package {
        #[doc = "The various channels by which a package is distributed."]
        #[serde(
            rename = "distribution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distribution: ::std::option::Option<Vec<crate::schemas::Distribution>>,
        #[doc = "The name of the package."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Package {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Package {
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
    pub struct PackageInfoNote {
        #[doc = "Indicates whether the file content of this package has been available for or subjected to analysis when creating the SPDX document"]
        #[serde(
            rename = "analyzed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub analyzed: ::std::option::Option<bool>,
        #[doc = "A place for the SPDX data creator to record, at the package level, acknowledgements that may be needed to be communicated in some contexts"]
        #[serde(
            rename = "attribution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attribution: ::std::option::Option<String>,
        #[doc = "Provide an independently reproducible mechanism that permits unique identification of a specific package that correlates to the data in this SPDX file"]
        #[serde(
            rename = "checksum",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub checksum: ::std::option::Option<String>,
        #[doc = "Identify the copyright holders of the package, as well as any dates present"]
        #[serde(
            rename = "copyright",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub copyright: ::std::option::Option<String>,
        #[doc = "A more detailed description of the package"]
        #[serde(
            rename = "detailedDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detailed_description: ::std::option::Option<String>,
        #[doc = "This section identifies the download Universal Resource Locator (URL), or a specific location within a version control system (VCS) for the package at the time that the SPDX file was created"]
        #[serde(
            rename = "downloadLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub download_location: ::std::option::Option<String>,
        #[doc = "ExternalRef"]
        #[serde(
            rename = "externalRefs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub external_refs: ::std::option::Option<Vec<crate::schemas::ExternalRef>>,
        #[doc = "Contain the license the SPDX file creator has concluded as governing the This field is to contain a list of all licenses found in the package. The relationship between licenses (i.e., conjunctive, disjunctive) is not specified in this field – it is simply a listing of all licenses found"]
        #[serde(
            rename = "filesLicenseInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub files_license_info: ::std::option::Option<Vec<String>>,
        #[doc = "Provide a place for the SPDX file creator to record a web site that serves as the package’s home page"]
        #[serde(
            rename = "homePage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub home_page: ::std::option::Option<String>,
        #[doc = "List the licenses that have been declared by the authors of the package"]
        #[serde(
            rename = "licenseDeclared",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub license_declared: ::std::option::Option<crate::schemas::License>,
        #[doc = "If the package identified in the SPDX file originated from a different person or organization than identified as Package Supplier, this field identifies from where or whom the package originally came"]
        #[serde(
            rename = "originator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub originator: ::std::option::Option<String>,
        #[doc = "The type of package: OS, MAVEN, GO, GO_STDLIB, etc."]
        #[serde(
            rename = "packageType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_type: ::std::option::Option<String>,
        #[doc = "A short description of the package"]
        #[serde(
            rename = "summaryDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub summary_description: ::std::option::Option<String>,
        #[doc = "Identify the actual distribution source for the package/directory identified in the SPDX file"]
        #[serde(
            rename = "supplier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub supplier: ::std::option::Option<String>,
        #[doc = "Identify the full name of the package as given by the Package Originator"]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "This field provides an independently reproducible mechanism identifying specific contents of a package based on the actual files (except the SPDX file itself, if it is included in the package) that make up each package and that correlates to the data in this SPDX file"]
        #[serde(
            rename = "verificationCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub verification_code: ::std::option::Option<String>,
        #[doc = "Identify the version of the package"]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PackageInfoNote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PackageInfoNote {
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
    pub struct PackageInfoOccurrence {
        #[doc = "A place for the SPDX file creator to record any general comments about the package being described"]
        #[serde(
            rename = "comment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub comment: ::std::option::Option<String>,
        #[doc = "Provide the actual file name of the package, or path of the directory being treated as a package"]
        #[serde(
            rename = "filename",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filename: ::std::option::Option<String>,
        #[doc = "Output only. Provide a place for the SPDX file creator to record a web site that serves as the package’s home page"]
        #[serde(
            rename = "homePage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub home_page: ::std::option::Option<String>,
        #[doc = "Uniquely identify any element in an SPDX document which may be referenced by other elements"]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "package or alternative values, if the governing license cannot be determined"]
        #[serde(
            rename = "licenseConcluded",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub license_concluded: ::std::option::Option<crate::schemas::License>,
        #[doc = "Output only. The type of package: OS, MAVEN, GO, GO_STDLIB, etc."]
        #[serde(
            rename = "packageType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_type: ::std::option::Option<String>,
        #[doc = "Provide a place for the SPDX file creator to record any relevant background information or additional comments about the origin of the package"]
        #[serde(
            rename = "sourceInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_info: ::std::option::Option<String>,
        #[doc = "Output only. A short description of the package"]
        #[serde(
            rename = "summaryDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub summary_description: ::std::option::Option<String>,
        #[doc = "Output only. Identify the full name of the package as given by the Package Originator"]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Output only. Identify the version of the package"]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PackageInfoOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PackageInfoOccurrence {
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
        #[doc = "The location of the vulnerability."]
        #[serde(
            rename = "affectedLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub affected_location: ::std::option::Option<crate::schemas::VulnerabilityLocation>,
        #[doc = "Output only. The distro or language system assigned severity for this vulnerability when that is available and note provider assigned severity when distro or language system has not yet assigned a severity for this vulnerability."]
        #[serde(
            rename = "effectiveSeverity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub effective_severity:
            ::std::option::Option<crate::schemas::PackageIssueEffectiveSeverity>,
        #[doc = "The location of the available fix for vulnerability."]
        #[serde(
            rename = "fixedLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_location: ::std::option::Option<crate::schemas::VulnerabilityLocation>,
        #[doc = "The type of package (e.g. OS, MAVEN, GO)."]
        #[serde(
            rename = "packageType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_type: ::std::option::Option<String>,
        #[serde(
            rename = "severityName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity_name: ::std::option::Option<String>,
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
        #[doc = "Critical Impact"]
        Critical,
        #[doc = "High Impact"]
        High,
        #[doc = "Low Impact"]
        Low,
        #[doc = "Medium Impact"]
        Medium,
        #[doc = "Minimal Impact"]
        Minimal,
        #[doc = "Unknown Impact"]
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
    pub struct PgpSignedAttestation {
        #[doc = "Type (for example schema) of the attestation payload that was signed. The verifier must ensure that the provided type is one that the verifier supports, and that the attestation payload is a valid instantiation of that type (for example by validating a JSON schema)."]
        #[serde(
            rename = "contentType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_type: ::std::option::Option<crate::schemas::PgpSignedAttestationContentType>,
        #[doc = "The cryptographic fingerprint of the key used to generate the signature, as output by, e.g. `gpg --list-keys`. This should be the version 4, full 160-bit fingerprint, expressed as a 40 character hexadecimal string. See https://tools.ietf.org/html/rfc4880#section-12.2 for details. Implementations may choose to acknowledge “LONG”, “SHORT”, or other abbreviated key IDs, but only the full fingerprint is guaranteed to work. In gpg, the full fingerprint can be retrieved from the `fpr` field returned when calling –list-keys with –with-colons. For example: `gpg --with-colons --with-fingerprint --force-v4-certs \\ --list-keys attester@example.com tru::1:1513631572:0:3:1:5 pub:...... fpr:::::::::24FF6481B76AC91E66A00AC657A93A81EF3AE6FB:` Above, the fingerprint is `24FF6481B76AC91E66A00AC657A93A81EF3AE6FB`."]
        #[serde(
            rename = "pgpKeyId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pgp_key_id: ::std::option::Option<String>,
        #[doc = "The raw content of the signature, as output by GNU Privacy Guard (GPG) or equivalent. Since this message only supports attached signatures, the payload that was signed must be attached. While the signature format supported is dependent on the verification implementation, currently only ASCII-armored (`--armor` to gpg), non-clearsigned (`--sign` rather than `--clearsign` to gpg) are supported. Concretely, `gpg --sign --armor --output=signature.gpg payload.json` will create the signature content expected in this field in `signature.gpg` for the `payload.json` attestation payload."]
        #[serde(
            rename = "signature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signature: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PgpSignedAttestation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PgpSignedAttestation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PgpSignedAttestationContentType {
        #[doc = "`ContentType` is not set."]
        ContentTypeUnspecified,
        #[doc = "Atomic format attestation signature. See https://github.com/containers/image/blob/8a5d2f82a6e3263290c8e0276c3e0f64e77723e7/docs/atomic-signature.md The payload extracted from `signature` is a JSON blob conforming to the linked schema."]
        SimpleSigningJson,
    }
    impl PgpSignedAttestationContentType {
        pub fn as_str(self) -> &'static str {
            match self {
                PgpSignedAttestationContentType::ContentTypeUnspecified => {
                    "CONTENT_TYPE_UNSPECIFIED"
                }
                PgpSignedAttestationContentType::SimpleSigningJson => "SIMPLE_SIGNING_JSON",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PgpSignedAttestationContentType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PgpSignedAttestationContentType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PgpSignedAttestationContentType, ()> {
            Ok(match s {
                "CONTENT_TYPE_UNSPECIFIED" => {
                    PgpSignedAttestationContentType::ContentTypeUnspecified
                }
                "SIMPLE_SIGNING_JSON" => PgpSignedAttestationContentType::SimpleSigningJson,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PgpSignedAttestationContentType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PgpSignedAttestationContentType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PgpSignedAttestationContentType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTENT_TYPE_UNSPECIFIED" => {
                    PgpSignedAttestationContentType::ContentTypeUnspecified
                }
                "SIMPLE_SIGNING_JSON" => PgpSignedAttestationContentType::SimpleSigningJson,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PgpSignedAttestationContentType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PgpSignedAttestationContentType {
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Recipe {
        #[doc = "Collection of all external inputs that influenced the build on top of recipe.definedInMaterial and recipe.entryPoint. For example, if the recipe type were “make”, then this might be the flags passed to make aside from the target, which is captured in recipe.entryPoint."]
        #[serde(
            rename = "arguments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arguments:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "Index in materials containing the recipe steps that are not implied by recipe.type. For example, if the recipe type were “make”, then this would point to the source containing the Makefile, not the make program itself. Set to -1 if the recipe doesn’t come from a material, as zero is default unset value for int64."]
        #[serde(
            rename = "definedInMaterial",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub defined_in_material: ::std::option::Option<i64>,
        #[doc = "String identifying the entry point into the build. This is often a path to a configuration file and/or a target label within that file. The syntax and meaning are defined by recipe.type. For example, if the recipe type were “make”, then this would reference the directory in which to run make as well as which target to use."]
        #[serde(
            rename = "entryPoint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entry_point: ::std::option::Option<String>,
        #[doc = "Any other builder-controlled inputs necessary for correctly evaluating the recipe. Usually only needed for reproducing the build but not evaluated as part of policy."]
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
        #[doc = "Label to describe usage of the URL"]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
        #[doc = "Specific URL to associate with the note"]
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
    pub struct RelationshipNote {
        #[doc = "The type of relationship between the source and target SPDX elements"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::RelationshipNoteType>,
    }
    impl ::google_field_selector::FieldSelector for RelationshipNote {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RelationshipNote {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RelationshipNoteType {
        #[doc = "Is to be used when (current) SPDXRef-DOCUMENT amends the SPDX information in SPDXRef-B"]
        Amends,
        #[doc = "Is to be used when SPDXRef-A is an ancestor (same lineage but pre-dates) SPDXRef-B"]
        AncestorOf,
        #[doc = "Is to be used when SPDXRef-A is a build dependency of SPDXRef-B"]
        BuildDependencyOf,
        #[doc = "Is to be used when SPDXRef-A is used to build SPDXRef-B"]
        BuildToolOf,
        #[doc = "Is to be used when SPDXRef-A is contained by SPDXRef-B"]
        ContainedBy,
        #[doc = "Is to be used when SPDXRef-A contains SPDXRef-B"]
        Contains,
        #[doc = "Is to be used when SPDXRef-A is an exact copy of SPDXRef-B"]
        CopyOf,
        #[doc = "Is to be used when SPDXRef-A is a data file used in SPDXRef-B"]
        DataFileOf,
        #[doc = "Is to be used when SPDXRef-A is a manifest file that lists a set of dependencies for SPDXRef-B"]
        DependencyManifestOf,
        #[doc = "Is to be used when SPDXRef-A is dependency of SPDXRef-B"]
        DependencyOf,
        #[doc = "Is to be used when SPDXRef-A depends on SPDXRef-B"]
        DependsOn,
        #[doc = "Is to be used when SPDXRef-A is a descendant of (same lineage but postdates) SPDXRef-B"]
        DescendantOf,
        #[doc = "Is to be used when SPDXRef-A is described by SPDXREF-Document"]
        DescribedBy,
        #[doc = "Is to be used when SPDXRef-DOCUMENT describes SPDXRef-A"]
        Describes,
        #[doc = "Is to be used when SPDXRef-A is a development dependency of SPDXRef-B"]
        DevDependencyOf,
        #[doc = "Is to be used when SPDXRef-A is used as a development tool for SPDXRef-B"]
        DevToolOf,
        #[doc = "Is to be used when distributing SPDXRef-A requires that SPDXRef-B also be distributed"]
        DistributionArtifact,
        #[doc = "Is to be used when SPDXRef-A provides documentation of SPDXRef-B"]
        DocumentationOf,
        #[doc = "Is to be used when SPDXRef-A dynamically links to SPDXRef-B"]
        DynamicLink,
        #[doc = "Is to be used when SPDXRef-A is an example of SPDXRef-B"]
        ExampleOf,
        #[doc = "Is to be used when SPDXRef-A is expanded from the archive SPDXRef-B"]
        ExpandedFromArchive,
        #[doc = "Is to be used when SPDXRef-A is a file that was added to SPDXRef-B"]
        FileAdded,
        #[doc = "Is to be used when SPDXRef-A is a file that was deleted from SPDXRef-B"]
        FileDeleted,
        #[doc = "Is to be used when SPDXRef-A is a file that was modified from SPDXRef-B"]
        FileModified,
        #[doc = "Is to be used when SPDXRef-A was generated from SPDXRef-B"]
        GeneratedFrom,
        #[doc = "Is to be used when SPDXRef-A generates SPDXRef-B"]
        Generates,
        #[doc = "Is to be used when SPDXRef-A has as a prerequisite SPDXRef-B"]
        HasPrerequisite,
        #[doc = "Is to be used when SPDXRef-A is a metafile of SPDXRef-B"]
        MetafileOf,
        #[doc = "Is to be used when SPDXRef-A is an optional component of SPDXRef-B"]
        OptionalComponentOf,
        #[doc = "Is to be used when SPDXRef-A is an optional dependency of SPDXRef-B"]
        OptionalDependencyOf,
        #[doc = "Is to be used for a relationship which has not been defined in the formal SPDX specification. A description of the relationship should be included in the Relationship comments field"]
        Other,
        #[doc = "Is to be used when SPDXRef-A is used as a package as part of SPDXRef-B"]
        PackageOf,
        #[doc = "Is to be used when SPDXRef-A is a patch file that has been applied to SPDXRef-B"]
        PatchApplied,
        #[doc = "Is to be used when SPDXRef-A is a patch file for (to be applied to) SPDXRef-B"]
        PatchFor,
        #[doc = "Is to be used when SPDXRef-A is a prerequisite for SPDXRef-B"]
        PrerequisiteFor,
        #[doc = "Is to be used when SPDXRef-A is a to be provided dependency of SPDXRef-B"]
        ProvidedDependencyOf,
        #[doc = "Unspecified"]
        RelationshipTypeUnspecified,
        #[doc = "Is to be used when SPDXRef-A is a dependency required for the execution of SPDXRef-B"]
        RuntimeDependencyOf,
        #[doc = "Is to be used when SPDXRef-A statically links to SPDXRef-B"]
        StaticLink,
        #[doc = "Is to be used when SPDXRef-A is a test case used in testing SPDXRef-B"]
        TestCaseOf,
        #[doc = "Is to be used when SPDXRef-A is a test dependency of SPDXRef-B"]
        TestDependencyOf,
        #[doc = "Is to be used when SPDXRef-A is used for testing SPDXRef-B"]
        TestOf,
        #[doc = "Is to be used when SPDXRef-A is used as a test tool for SPDXRef-B"]
        TestToolOf,
        #[doc = "Is to be used when SPDXRef-A is a variant of (same lineage but not clear which came first) SPDXRef-B"]
        VariantOf,
    }
    impl RelationshipNoteType {
        pub fn as_str(self) -> &'static str {
            match self {
                RelationshipNoteType::Amends => "AMENDS",
                RelationshipNoteType::AncestorOf => "ANCESTOR_OF",
                RelationshipNoteType::BuildDependencyOf => "BUILD_DEPENDENCY_OF",
                RelationshipNoteType::BuildToolOf => "BUILD_TOOL_OF",
                RelationshipNoteType::ContainedBy => "CONTAINED_BY",
                RelationshipNoteType::Contains => "CONTAINS",
                RelationshipNoteType::CopyOf => "COPY_OF",
                RelationshipNoteType::DataFileOf => "DATA_FILE_OF",
                RelationshipNoteType::DependencyManifestOf => "DEPENDENCY_MANIFEST_OF",
                RelationshipNoteType::DependencyOf => "DEPENDENCY_OF",
                RelationshipNoteType::DependsOn => "DEPENDS_ON",
                RelationshipNoteType::DescendantOf => "DESCENDANT_OF",
                RelationshipNoteType::DescribedBy => "DESCRIBED_BY",
                RelationshipNoteType::Describes => "DESCRIBES",
                RelationshipNoteType::DevDependencyOf => "DEV_DEPENDENCY_OF",
                RelationshipNoteType::DevToolOf => "DEV_TOOL_OF",
                RelationshipNoteType::DistributionArtifact => "DISTRIBUTION_ARTIFACT",
                RelationshipNoteType::DocumentationOf => "DOCUMENTATION_OF",
                RelationshipNoteType::DynamicLink => "DYNAMIC_LINK",
                RelationshipNoteType::ExampleOf => "EXAMPLE_OF",
                RelationshipNoteType::ExpandedFromArchive => "EXPANDED_FROM_ARCHIVE",
                RelationshipNoteType::FileAdded => "FILE_ADDED",
                RelationshipNoteType::FileDeleted => "FILE_DELETED",
                RelationshipNoteType::FileModified => "FILE_MODIFIED",
                RelationshipNoteType::GeneratedFrom => "GENERATED_FROM",
                RelationshipNoteType::Generates => "GENERATES",
                RelationshipNoteType::HasPrerequisite => "HAS_PREREQUISITE",
                RelationshipNoteType::MetafileOf => "METAFILE_OF",
                RelationshipNoteType::OptionalComponentOf => "OPTIONAL_COMPONENT_OF",
                RelationshipNoteType::OptionalDependencyOf => "OPTIONAL_DEPENDENCY_OF",
                RelationshipNoteType::Other => "OTHER",
                RelationshipNoteType::PackageOf => "PACKAGE_OF",
                RelationshipNoteType::PatchApplied => "PATCH_APPLIED",
                RelationshipNoteType::PatchFor => "PATCH_FOR",
                RelationshipNoteType::PrerequisiteFor => "PREREQUISITE_FOR",
                RelationshipNoteType::ProvidedDependencyOf => "PROVIDED_DEPENDENCY_OF",
                RelationshipNoteType::RelationshipTypeUnspecified => {
                    "RELATIONSHIP_TYPE_UNSPECIFIED"
                }
                RelationshipNoteType::RuntimeDependencyOf => "RUNTIME_DEPENDENCY_OF",
                RelationshipNoteType::StaticLink => "STATIC_LINK",
                RelationshipNoteType::TestCaseOf => "TEST_CASE_OF",
                RelationshipNoteType::TestDependencyOf => "TEST_DEPENDENCY_OF",
                RelationshipNoteType::TestOf => "TEST_OF",
                RelationshipNoteType::TestToolOf => "TEST_TOOL_OF",
                RelationshipNoteType::VariantOf => "VARIANT_OF",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RelationshipNoteType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RelationshipNoteType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RelationshipNoteType, ()> {
            Ok(match s {
                "AMENDS" => RelationshipNoteType::Amends,
                "ANCESTOR_OF" => RelationshipNoteType::AncestorOf,
                "BUILD_DEPENDENCY_OF" => RelationshipNoteType::BuildDependencyOf,
                "BUILD_TOOL_OF" => RelationshipNoteType::BuildToolOf,
                "CONTAINED_BY" => RelationshipNoteType::ContainedBy,
                "CONTAINS" => RelationshipNoteType::Contains,
                "COPY_OF" => RelationshipNoteType::CopyOf,
                "DATA_FILE_OF" => RelationshipNoteType::DataFileOf,
                "DEPENDENCY_MANIFEST_OF" => RelationshipNoteType::DependencyManifestOf,
                "DEPENDENCY_OF" => RelationshipNoteType::DependencyOf,
                "DEPENDS_ON" => RelationshipNoteType::DependsOn,
                "DESCENDANT_OF" => RelationshipNoteType::DescendantOf,
                "DESCRIBED_BY" => RelationshipNoteType::DescribedBy,
                "DESCRIBES" => RelationshipNoteType::Describes,
                "DEV_DEPENDENCY_OF" => RelationshipNoteType::DevDependencyOf,
                "DEV_TOOL_OF" => RelationshipNoteType::DevToolOf,
                "DISTRIBUTION_ARTIFACT" => RelationshipNoteType::DistributionArtifact,
                "DOCUMENTATION_OF" => RelationshipNoteType::DocumentationOf,
                "DYNAMIC_LINK" => RelationshipNoteType::DynamicLink,
                "EXAMPLE_OF" => RelationshipNoteType::ExampleOf,
                "EXPANDED_FROM_ARCHIVE" => RelationshipNoteType::ExpandedFromArchive,
                "FILE_ADDED" => RelationshipNoteType::FileAdded,
                "FILE_DELETED" => RelationshipNoteType::FileDeleted,
                "FILE_MODIFIED" => RelationshipNoteType::FileModified,
                "GENERATED_FROM" => RelationshipNoteType::GeneratedFrom,
                "GENERATES" => RelationshipNoteType::Generates,
                "HAS_PREREQUISITE" => RelationshipNoteType::HasPrerequisite,
                "METAFILE_OF" => RelationshipNoteType::MetafileOf,
                "OPTIONAL_COMPONENT_OF" => RelationshipNoteType::OptionalComponentOf,
                "OPTIONAL_DEPENDENCY_OF" => RelationshipNoteType::OptionalDependencyOf,
                "OTHER" => RelationshipNoteType::Other,
                "PACKAGE_OF" => RelationshipNoteType::PackageOf,
                "PATCH_APPLIED" => RelationshipNoteType::PatchApplied,
                "PATCH_FOR" => RelationshipNoteType::PatchFor,
                "PREREQUISITE_FOR" => RelationshipNoteType::PrerequisiteFor,
                "PROVIDED_DEPENDENCY_OF" => RelationshipNoteType::ProvidedDependencyOf,
                "RELATIONSHIP_TYPE_UNSPECIFIED" => {
                    RelationshipNoteType::RelationshipTypeUnspecified
                }
                "RUNTIME_DEPENDENCY_OF" => RelationshipNoteType::RuntimeDependencyOf,
                "STATIC_LINK" => RelationshipNoteType::StaticLink,
                "TEST_CASE_OF" => RelationshipNoteType::TestCaseOf,
                "TEST_DEPENDENCY_OF" => RelationshipNoteType::TestDependencyOf,
                "TEST_OF" => RelationshipNoteType::TestOf,
                "TEST_TOOL_OF" => RelationshipNoteType::TestToolOf,
                "VARIANT_OF" => RelationshipNoteType::VariantOf,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RelationshipNoteType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RelationshipNoteType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RelationshipNoteType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AMENDS" => RelationshipNoteType::Amends,
                "ANCESTOR_OF" => RelationshipNoteType::AncestorOf,
                "BUILD_DEPENDENCY_OF" => RelationshipNoteType::BuildDependencyOf,
                "BUILD_TOOL_OF" => RelationshipNoteType::BuildToolOf,
                "CONTAINED_BY" => RelationshipNoteType::ContainedBy,
                "CONTAINS" => RelationshipNoteType::Contains,
                "COPY_OF" => RelationshipNoteType::CopyOf,
                "DATA_FILE_OF" => RelationshipNoteType::DataFileOf,
                "DEPENDENCY_MANIFEST_OF" => RelationshipNoteType::DependencyManifestOf,
                "DEPENDENCY_OF" => RelationshipNoteType::DependencyOf,
                "DEPENDS_ON" => RelationshipNoteType::DependsOn,
                "DESCENDANT_OF" => RelationshipNoteType::DescendantOf,
                "DESCRIBED_BY" => RelationshipNoteType::DescribedBy,
                "DESCRIBES" => RelationshipNoteType::Describes,
                "DEV_DEPENDENCY_OF" => RelationshipNoteType::DevDependencyOf,
                "DEV_TOOL_OF" => RelationshipNoteType::DevToolOf,
                "DISTRIBUTION_ARTIFACT" => RelationshipNoteType::DistributionArtifact,
                "DOCUMENTATION_OF" => RelationshipNoteType::DocumentationOf,
                "DYNAMIC_LINK" => RelationshipNoteType::DynamicLink,
                "EXAMPLE_OF" => RelationshipNoteType::ExampleOf,
                "EXPANDED_FROM_ARCHIVE" => RelationshipNoteType::ExpandedFromArchive,
                "FILE_ADDED" => RelationshipNoteType::FileAdded,
                "FILE_DELETED" => RelationshipNoteType::FileDeleted,
                "FILE_MODIFIED" => RelationshipNoteType::FileModified,
                "GENERATED_FROM" => RelationshipNoteType::GeneratedFrom,
                "GENERATES" => RelationshipNoteType::Generates,
                "HAS_PREREQUISITE" => RelationshipNoteType::HasPrerequisite,
                "METAFILE_OF" => RelationshipNoteType::MetafileOf,
                "OPTIONAL_COMPONENT_OF" => RelationshipNoteType::OptionalComponentOf,
                "OPTIONAL_DEPENDENCY_OF" => RelationshipNoteType::OptionalDependencyOf,
                "OTHER" => RelationshipNoteType::Other,
                "PACKAGE_OF" => RelationshipNoteType::PackageOf,
                "PATCH_APPLIED" => RelationshipNoteType::PatchApplied,
                "PATCH_FOR" => RelationshipNoteType::PatchFor,
                "PREREQUISITE_FOR" => RelationshipNoteType::PrerequisiteFor,
                "PROVIDED_DEPENDENCY_OF" => RelationshipNoteType::ProvidedDependencyOf,
                "RELATIONSHIP_TYPE_UNSPECIFIED" => {
                    RelationshipNoteType::RelationshipTypeUnspecified
                }
                "RUNTIME_DEPENDENCY_OF" => RelationshipNoteType::RuntimeDependencyOf,
                "STATIC_LINK" => RelationshipNoteType::StaticLink,
                "TEST_CASE_OF" => RelationshipNoteType::TestCaseOf,
                "TEST_DEPENDENCY_OF" => RelationshipNoteType::TestDependencyOf,
                "TEST_OF" => RelationshipNoteType::TestOf,
                "TEST_TOOL_OF" => RelationshipNoteType::TestToolOf,
                "VARIANT_OF" => RelationshipNoteType::VariantOf,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RelationshipNoteType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RelationshipNoteType {
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
    pub struct RelationshipOccurrence {
        #[doc = "A place for the SPDX file creator to record any general comments about the relationship"]
        #[serde(
            rename = "comment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub comment: ::std::option::Option<String>,
        #[doc = "Output only. The type of relationship between the source and target SPDX elements"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::RelationshipOccurrenceType>,
        #[doc = "Also referred to as SPDXRef-A The source SPDX element (file, package, etc)"]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<String>,
        #[doc = "Also referred to as SPDXRef-B The target SPDC element (file, package, etc) In cases where there are “known unknowns”, the use of the keyword NOASSERTION can be used The keywords NONE can be used to indicate that an SPDX element (package/file/snippet) has no other elements connected by some relationship to it"]
        #[serde(
            rename = "target",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RelationshipOccurrence {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RelationshipOccurrence {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RelationshipOccurrenceType {
        #[doc = "Is to be used when (current) SPDXRef-DOCUMENT amends the SPDX information in SPDXRef-B"]
        Amends,
        #[doc = "Is to be used when SPDXRef-A is an ancestor (same lineage but pre-dates) SPDXRef-B"]
        AncestorOf,
        #[doc = "Is to be used when SPDXRef-A is a build dependency of SPDXRef-B"]
        BuildDependencyOf,
        #[doc = "Is to be used when SPDXRef-A is used to build SPDXRef-B"]
        BuildToolOf,
        #[doc = "Is to be used when SPDXRef-A is contained by SPDXRef-B"]
        ContainedBy,
        #[doc = "Is to be used when SPDXRef-A contains SPDXRef-B"]
        Contains,
        #[doc = "Is to be used when SPDXRef-A is an exact copy of SPDXRef-B"]
        CopyOf,
        #[doc = "Is to be used when SPDXRef-A is a data file used in SPDXRef-B"]
        DataFileOf,
        #[doc = "Is to be used when SPDXRef-A is a manifest file that lists a set of dependencies for SPDXRef-B"]
        DependencyManifestOf,
        #[doc = "Is to be used when SPDXRef-A is dependency of SPDXRef-B"]
        DependencyOf,
        #[doc = "Is to be used when SPDXRef-A depends on SPDXRef-B"]
        DependsOn,
        #[doc = "Is to be used when SPDXRef-A is a descendant of (same lineage but postdates) SPDXRef-B"]
        DescendantOf,
        #[doc = "Is to be used when SPDXRef-A is described by SPDXREF-Document"]
        DescribedBy,
        #[doc = "Is to be used when SPDXRef-DOCUMENT describes SPDXRef-A"]
        Describes,
        #[doc = "Is to be used when SPDXRef-A is a development dependency of SPDXRef-B"]
        DevDependencyOf,
        #[doc = "Is to be used when SPDXRef-A is used as a development tool for SPDXRef-B"]
        DevToolOf,
        #[doc = "Is to be used when distributing SPDXRef-A requires that SPDXRef-B also be distributed"]
        DistributionArtifact,
        #[doc = "Is to be used when SPDXRef-A provides documentation of SPDXRef-B"]
        DocumentationOf,
        #[doc = "Is to be used when SPDXRef-A dynamically links to SPDXRef-B"]
        DynamicLink,
        #[doc = "Is to be used when SPDXRef-A is an example of SPDXRef-B"]
        ExampleOf,
        #[doc = "Is to be used when SPDXRef-A is expanded from the archive SPDXRef-B"]
        ExpandedFromArchive,
        #[doc = "Is to be used when SPDXRef-A is a file that was added to SPDXRef-B"]
        FileAdded,
        #[doc = "Is to be used when SPDXRef-A is a file that was deleted from SPDXRef-B"]
        FileDeleted,
        #[doc = "Is to be used when SPDXRef-A is a file that was modified from SPDXRef-B"]
        FileModified,
        #[doc = "Is to be used when SPDXRef-A was generated from SPDXRef-B"]
        GeneratedFrom,
        #[doc = "Is to be used when SPDXRef-A generates SPDXRef-B"]
        Generates,
        #[doc = "Is to be used when SPDXRef-A has as a prerequisite SPDXRef-B"]
        HasPrerequisite,
        #[doc = "Is to be used when SPDXRef-A is a metafile of SPDXRef-B"]
        MetafileOf,
        #[doc = "Is to be used when SPDXRef-A is an optional component of SPDXRef-B"]
        OptionalComponentOf,
        #[doc = "Is to be used when SPDXRef-A is an optional dependency of SPDXRef-B"]
        OptionalDependencyOf,
        #[doc = "Is to be used for a relationship which has not been defined in the formal SPDX specification. A description of the relationship should be included in the Relationship comments field"]
        Other,
        #[doc = "Is to be used when SPDXRef-A is used as a package as part of SPDXRef-B"]
        PackageOf,
        #[doc = "Is to be used when SPDXRef-A is a patch file that has been applied to SPDXRef-B"]
        PatchApplied,
        #[doc = "Is to be used when SPDXRef-A is a patch file for (to be applied to) SPDXRef-B"]
        PatchFor,
        #[doc = "Is to be used when SPDXRef-A is a prerequisite for SPDXRef-B"]
        PrerequisiteFor,
        #[doc = "Is to be used when SPDXRef-A is a to be provided dependency of SPDXRef-B"]
        ProvidedDependencyOf,
        #[doc = "Unspecified"]
        RelationshipTypeUnspecified,
        #[doc = "Is to be used when SPDXRef-A is a dependency required for the execution of SPDXRef-B"]
        RuntimeDependencyOf,
        #[doc = "Is to be used when SPDXRef-A statically links to SPDXRef-B"]
        StaticLink,
        #[doc = "Is to be used when SPDXRef-A is a test case used in testing SPDXRef-B"]
        TestCaseOf,
        #[doc = "Is to be used when SPDXRef-A is a test dependency of SPDXRef-B"]
        TestDependencyOf,
        #[doc = "Is to be used when SPDXRef-A is used for testing SPDXRef-B"]
        TestOf,
        #[doc = "Is to be used when SPDXRef-A is used as a test tool for SPDXRef-B"]
        TestToolOf,
        #[doc = "Is to be used when SPDXRef-A is a variant of (same lineage but not clear which came first) SPDXRef-B"]
        VariantOf,
    }
    impl RelationshipOccurrenceType {
        pub fn as_str(self) -> &'static str {
            match self {
                RelationshipOccurrenceType::Amends => "AMENDS",
                RelationshipOccurrenceType::AncestorOf => "ANCESTOR_OF",
                RelationshipOccurrenceType::BuildDependencyOf => "BUILD_DEPENDENCY_OF",
                RelationshipOccurrenceType::BuildToolOf => "BUILD_TOOL_OF",
                RelationshipOccurrenceType::ContainedBy => "CONTAINED_BY",
                RelationshipOccurrenceType::Contains => "CONTAINS",
                RelationshipOccurrenceType::CopyOf => "COPY_OF",
                RelationshipOccurrenceType::DataFileOf => "DATA_FILE_OF",
                RelationshipOccurrenceType::DependencyManifestOf => "DEPENDENCY_MANIFEST_OF",
                RelationshipOccurrenceType::DependencyOf => "DEPENDENCY_OF",
                RelationshipOccurrenceType::DependsOn => "DEPENDS_ON",
                RelationshipOccurrenceType::DescendantOf => "DESCENDANT_OF",
                RelationshipOccurrenceType::DescribedBy => "DESCRIBED_BY",
                RelationshipOccurrenceType::Describes => "DESCRIBES",
                RelationshipOccurrenceType::DevDependencyOf => "DEV_DEPENDENCY_OF",
                RelationshipOccurrenceType::DevToolOf => "DEV_TOOL_OF",
                RelationshipOccurrenceType::DistributionArtifact => "DISTRIBUTION_ARTIFACT",
                RelationshipOccurrenceType::DocumentationOf => "DOCUMENTATION_OF",
                RelationshipOccurrenceType::DynamicLink => "DYNAMIC_LINK",
                RelationshipOccurrenceType::ExampleOf => "EXAMPLE_OF",
                RelationshipOccurrenceType::ExpandedFromArchive => "EXPANDED_FROM_ARCHIVE",
                RelationshipOccurrenceType::FileAdded => "FILE_ADDED",
                RelationshipOccurrenceType::FileDeleted => "FILE_DELETED",
                RelationshipOccurrenceType::FileModified => "FILE_MODIFIED",
                RelationshipOccurrenceType::GeneratedFrom => "GENERATED_FROM",
                RelationshipOccurrenceType::Generates => "GENERATES",
                RelationshipOccurrenceType::HasPrerequisite => "HAS_PREREQUISITE",
                RelationshipOccurrenceType::MetafileOf => "METAFILE_OF",
                RelationshipOccurrenceType::OptionalComponentOf => "OPTIONAL_COMPONENT_OF",
                RelationshipOccurrenceType::OptionalDependencyOf => "OPTIONAL_DEPENDENCY_OF",
                RelationshipOccurrenceType::Other => "OTHER",
                RelationshipOccurrenceType::PackageOf => "PACKAGE_OF",
                RelationshipOccurrenceType::PatchApplied => "PATCH_APPLIED",
                RelationshipOccurrenceType::PatchFor => "PATCH_FOR",
                RelationshipOccurrenceType::PrerequisiteFor => "PREREQUISITE_FOR",
                RelationshipOccurrenceType::ProvidedDependencyOf => "PROVIDED_DEPENDENCY_OF",
                RelationshipOccurrenceType::RelationshipTypeUnspecified => {
                    "RELATIONSHIP_TYPE_UNSPECIFIED"
                }
                RelationshipOccurrenceType::RuntimeDependencyOf => "RUNTIME_DEPENDENCY_OF",
                RelationshipOccurrenceType::StaticLink => "STATIC_LINK",
                RelationshipOccurrenceType::TestCaseOf => "TEST_CASE_OF",
                RelationshipOccurrenceType::TestDependencyOf => "TEST_DEPENDENCY_OF",
                RelationshipOccurrenceType::TestOf => "TEST_OF",
                RelationshipOccurrenceType::TestToolOf => "TEST_TOOL_OF",
                RelationshipOccurrenceType::VariantOf => "VARIANT_OF",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RelationshipOccurrenceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RelationshipOccurrenceType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RelationshipOccurrenceType, ()> {
            Ok(match s {
                "AMENDS" => RelationshipOccurrenceType::Amends,
                "ANCESTOR_OF" => RelationshipOccurrenceType::AncestorOf,
                "BUILD_DEPENDENCY_OF" => RelationshipOccurrenceType::BuildDependencyOf,
                "BUILD_TOOL_OF" => RelationshipOccurrenceType::BuildToolOf,
                "CONTAINED_BY" => RelationshipOccurrenceType::ContainedBy,
                "CONTAINS" => RelationshipOccurrenceType::Contains,
                "COPY_OF" => RelationshipOccurrenceType::CopyOf,
                "DATA_FILE_OF" => RelationshipOccurrenceType::DataFileOf,
                "DEPENDENCY_MANIFEST_OF" => RelationshipOccurrenceType::DependencyManifestOf,
                "DEPENDENCY_OF" => RelationshipOccurrenceType::DependencyOf,
                "DEPENDS_ON" => RelationshipOccurrenceType::DependsOn,
                "DESCENDANT_OF" => RelationshipOccurrenceType::DescendantOf,
                "DESCRIBED_BY" => RelationshipOccurrenceType::DescribedBy,
                "DESCRIBES" => RelationshipOccurrenceType::Describes,
                "DEV_DEPENDENCY_OF" => RelationshipOccurrenceType::DevDependencyOf,
                "DEV_TOOL_OF" => RelationshipOccurrenceType::DevToolOf,
                "DISTRIBUTION_ARTIFACT" => RelationshipOccurrenceType::DistributionArtifact,
                "DOCUMENTATION_OF" => RelationshipOccurrenceType::DocumentationOf,
                "DYNAMIC_LINK" => RelationshipOccurrenceType::DynamicLink,
                "EXAMPLE_OF" => RelationshipOccurrenceType::ExampleOf,
                "EXPANDED_FROM_ARCHIVE" => RelationshipOccurrenceType::ExpandedFromArchive,
                "FILE_ADDED" => RelationshipOccurrenceType::FileAdded,
                "FILE_DELETED" => RelationshipOccurrenceType::FileDeleted,
                "FILE_MODIFIED" => RelationshipOccurrenceType::FileModified,
                "GENERATED_FROM" => RelationshipOccurrenceType::GeneratedFrom,
                "GENERATES" => RelationshipOccurrenceType::Generates,
                "HAS_PREREQUISITE" => RelationshipOccurrenceType::HasPrerequisite,
                "METAFILE_OF" => RelationshipOccurrenceType::MetafileOf,
                "OPTIONAL_COMPONENT_OF" => RelationshipOccurrenceType::OptionalComponentOf,
                "OPTIONAL_DEPENDENCY_OF" => RelationshipOccurrenceType::OptionalDependencyOf,
                "OTHER" => RelationshipOccurrenceType::Other,
                "PACKAGE_OF" => RelationshipOccurrenceType::PackageOf,
                "PATCH_APPLIED" => RelationshipOccurrenceType::PatchApplied,
                "PATCH_FOR" => RelationshipOccurrenceType::PatchFor,
                "PREREQUISITE_FOR" => RelationshipOccurrenceType::PrerequisiteFor,
                "PROVIDED_DEPENDENCY_OF" => RelationshipOccurrenceType::ProvidedDependencyOf,
                "RELATIONSHIP_TYPE_UNSPECIFIED" => {
                    RelationshipOccurrenceType::RelationshipTypeUnspecified
                }
                "RUNTIME_DEPENDENCY_OF" => RelationshipOccurrenceType::RuntimeDependencyOf,
                "STATIC_LINK" => RelationshipOccurrenceType::StaticLink,
                "TEST_CASE_OF" => RelationshipOccurrenceType::TestCaseOf,
                "TEST_DEPENDENCY_OF" => RelationshipOccurrenceType::TestDependencyOf,
                "TEST_OF" => RelationshipOccurrenceType::TestOf,
                "TEST_TOOL_OF" => RelationshipOccurrenceType::TestToolOf,
                "VARIANT_OF" => RelationshipOccurrenceType::VariantOf,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RelationshipOccurrenceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RelationshipOccurrenceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RelationshipOccurrenceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AMENDS" => RelationshipOccurrenceType::Amends,
                "ANCESTOR_OF" => RelationshipOccurrenceType::AncestorOf,
                "BUILD_DEPENDENCY_OF" => RelationshipOccurrenceType::BuildDependencyOf,
                "BUILD_TOOL_OF" => RelationshipOccurrenceType::BuildToolOf,
                "CONTAINED_BY" => RelationshipOccurrenceType::ContainedBy,
                "CONTAINS" => RelationshipOccurrenceType::Contains,
                "COPY_OF" => RelationshipOccurrenceType::CopyOf,
                "DATA_FILE_OF" => RelationshipOccurrenceType::DataFileOf,
                "DEPENDENCY_MANIFEST_OF" => RelationshipOccurrenceType::DependencyManifestOf,
                "DEPENDENCY_OF" => RelationshipOccurrenceType::DependencyOf,
                "DEPENDS_ON" => RelationshipOccurrenceType::DependsOn,
                "DESCENDANT_OF" => RelationshipOccurrenceType::DescendantOf,
                "DESCRIBED_BY" => RelationshipOccurrenceType::DescribedBy,
                "DESCRIBES" => RelationshipOccurrenceType::Describes,
                "DEV_DEPENDENCY_OF" => RelationshipOccurrenceType::DevDependencyOf,
                "DEV_TOOL_OF" => RelationshipOccurrenceType::DevToolOf,
                "DISTRIBUTION_ARTIFACT" => RelationshipOccurrenceType::DistributionArtifact,
                "DOCUMENTATION_OF" => RelationshipOccurrenceType::DocumentationOf,
                "DYNAMIC_LINK" => RelationshipOccurrenceType::DynamicLink,
                "EXAMPLE_OF" => RelationshipOccurrenceType::ExampleOf,
                "EXPANDED_FROM_ARCHIVE" => RelationshipOccurrenceType::ExpandedFromArchive,
                "FILE_ADDED" => RelationshipOccurrenceType::FileAdded,
                "FILE_DELETED" => RelationshipOccurrenceType::FileDeleted,
                "FILE_MODIFIED" => RelationshipOccurrenceType::FileModified,
                "GENERATED_FROM" => RelationshipOccurrenceType::GeneratedFrom,
                "GENERATES" => RelationshipOccurrenceType::Generates,
                "HAS_PREREQUISITE" => RelationshipOccurrenceType::HasPrerequisite,
                "METAFILE_OF" => RelationshipOccurrenceType::MetafileOf,
                "OPTIONAL_COMPONENT_OF" => RelationshipOccurrenceType::OptionalComponentOf,
                "OPTIONAL_DEPENDENCY_OF" => RelationshipOccurrenceType::OptionalDependencyOf,
                "OTHER" => RelationshipOccurrenceType::Other,
                "PACKAGE_OF" => RelationshipOccurrenceType::PackageOf,
                "PATCH_APPLIED" => RelationshipOccurrenceType::PatchApplied,
                "PATCH_FOR" => RelationshipOccurrenceType::PatchFor,
                "PREREQUISITE_FOR" => RelationshipOccurrenceType::PrerequisiteFor,
                "PROVIDED_DEPENDENCY_OF" => RelationshipOccurrenceType::ProvidedDependencyOf,
                "RELATIONSHIP_TYPE_UNSPECIFIED" => {
                    RelationshipOccurrenceType::RelationshipTypeUnspecified
                }
                "RUNTIME_DEPENDENCY_OF" => RelationshipOccurrenceType::RuntimeDependencyOf,
                "STATIC_LINK" => RelationshipOccurrenceType::StaticLink,
                "TEST_CASE_OF" => RelationshipOccurrenceType::TestCaseOf,
                "TEST_DEPENDENCY_OF" => RelationshipOccurrenceType::TestDependencyOf,
                "TEST_OF" => RelationshipOccurrenceType::TestOf,
                "TEST_TOOL_OF" => RelationshipOccurrenceType::TestToolOf,
                "VARIANT_OF" => RelationshipOccurrenceType::VariantOf,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RelationshipOccurrenceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RelationshipOccurrenceType {
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
    pub struct RepoSource {
        #[doc = "Name of the branch to build."]
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
        #[doc = "ID of the project that owns the repo."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "Name of the repo."]
        #[serde(
            rename = "repoName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repo_name: ::std::option::Option<String>,
        #[doc = "Name of the tag to build."]
        #[serde(
            rename = "tagName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tag_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RepoSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RepoSource {
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
        #[doc = "The hash of the resource content. E.g., the Docker digest."]
        #[serde(
            rename = "contentHash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_hash: ::std::option::Option<crate::schemas::Hash>,
        #[doc = "The name of the resource. E.g., the name of a Docker image - “Debian”."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The unique URI of the resource. E.g., “https://gcr.io/project/image@sha256:foo” for a Docker image."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
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
    pub struct ScanConfig {
        #[doc = "Output only. The time this scan config was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. A human-readable description of what the `ScanConfig` does."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Indicates whether the Scan is enabled."]
        #[serde(
            rename = "enabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enabled: ::std::option::Option<bool>,
        #[doc = "Output only. The name of the ScanConfig in the form “projects/{project_id}/scanConfigs/{scan_config_id}“."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. The time this scan config was last updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ScanConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScanConfig {
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
    pub struct SeverityCount {
        #[doc = "The number of occurrences with the severity."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "The severity of the occurrences."]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<crate::schemas::SeverityCountSeverity>,
    }
    impl ::google_field_selector::FieldSelector for SeverityCount {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SeverityCount {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SeverityCountSeverity {
        #[doc = "Critical Impact"]
        Critical,
        #[doc = "High Impact"]
        High,
        #[doc = "Low Impact"]
        Low,
        #[doc = "Medium Impact"]
        Medium,
        #[doc = "Minimal Impact"]
        Minimal,
        #[doc = "Unknown Impact"]
        SeverityUnspecified,
    }
    impl SeverityCountSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                SeverityCountSeverity::Critical => "CRITICAL",
                SeverityCountSeverity::High => "HIGH",
                SeverityCountSeverity::Low => "LOW",
                SeverityCountSeverity::Medium => "MEDIUM",
                SeverityCountSeverity::Minimal => "MINIMAL",
                SeverityCountSeverity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SeverityCountSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SeverityCountSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SeverityCountSeverity, ()> {
            Ok(match s {
                "CRITICAL" => SeverityCountSeverity::Critical,
                "HIGH" => SeverityCountSeverity::High,
                "LOW" => SeverityCountSeverity::Low,
                "MEDIUM" => SeverityCountSeverity::Medium,
                "MINIMAL" => SeverityCountSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => SeverityCountSeverity::SeverityUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SeverityCountSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SeverityCountSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SeverityCountSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRITICAL" => SeverityCountSeverity::Critical,
                "HIGH" => SeverityCountSeverity::High,
                "LOW" => SeverityCountSeverity::Low,
                "MEDIUM" => SeverityCountSeverity::Medium,
                "MINIMAL" => SeverityCountSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => SeverityCountSeverity::SeverityUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SeverityCountSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SeverityCountSeverity {
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
        #[doc = "id is the id of the slsa provenance builder"]
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
        #[doc = "If true, the builder claims that materials are complete, usually through some controls to prevent network access. Sometimes called “hermetic”."]
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
        #[doc = "builder is the builder of this provenance"]
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
        #[doc = "metadata is the metadata of the provenance"]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::SlsaMetadata>,
        #[doc = "Identifies the configuration used for the build. When combined with materials, this SHOULD fully describe the build, such that re-running this recipe results in bit-for-bit identical output (if the build is reproducible)."]
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
        #[doc = "Collection of all external inputs that influenced the build on top of recipe.definedInMaterial and recipe.entryPoint. For example, if the recipe type were “make”, then this might be the flags passed to make aside from the target, which is captured in recipe.entryPoint. Depending on the recipe Type, the structure may be different."]
        #[serde(
            rename = "arguments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub arguments:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Index in materials containing the recipe steps that are not implied by recipe.type. For example, if the recipe type were “make”, then this would point to the source containing the Makefile, not the make program itself. Set to -1 if the recipe doesn’t come from a material, as zero is default unset value for int64."]
        #[serde(
            rename = "definedInMaterial",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub defined_in_material: ::std::option::Option<i64>,
        #[doc = "String identifying the entry point into the build. This is often a path to a configuration file and/or a target label within that file. The syntax and meaning are defined by recipe.type. For example, if the recipe type were “make”, then this would reference the directory in which to run make as well as which target to use."]
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
        pub additional_contexts: ::std::option::Option<
            Vec<crate::schemas::GoogleDevtoolsContaineranalysisV1Alpha1SourceContext>,
        >,
        #[doc = "If provided, the input binary artifacts for the build came from this location."]
        #[serde(
            rename = "artifactStorageSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub artifact_storage_source: ::std::option::Option<crate::schemas::StorageSource>,
        #[doc = "If provided, the source code used for the build came from this location."]
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context: ::std::option::Option<
            crate::schemas::GoogleDevtoolsContaineranalysisV1Alpha1SourceContext,
        >,
        #[doc = "Hash(es) of the build source, which can be used to verify that the original source integrity was maintained in the build. The keys to this map are file paths used as build source and the values contain the hash values for those files. If the build source came in a single package such as a gzipped tarfile (.tar.gz), the FileHash will be for the single path to that file."]
        #[serde(
            rename = "fileHashes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_hashes:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::FileHashes>>,
        #[doc = "If provided, get source from this location in a Cloud Repo."]
        #[serde(
            rename = "repoSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repo_source: ::std::option::Option<crate::schemas::RepoSource>,
        #[doc = "If provided, get the source from this location in in Google Cloud Storage."]
        #[serde(
            rename = "storageSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub storage_source: ::std::option::Option<crate::schemas::StorageSource>,
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
    pub struct StorageSource {
        #[doc = "Google Cloud Storage bucket containing source (see \\[Bucket Name Requirements\\] (https://cloud.google.com/storage/docs/bucket-naming#requirements))."]
        #[serde(
            rename = "bucket",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bucket: ::std::option::Option<String>,
        #[doc = "Google Cloud Storage generation for the object."]
        #[serde(
            rename = "generation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub generation: ::std::option::Option<i64>,
        #[doc = "Google Cloud Storage object containing source."]
        #[serde(
            rename = "object",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for StorageSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StorageSource {
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
        #[doc = "“”: “” Algorithms can be e.g. sha256, sha512 See https://github.com/in-toto/attestation/blob/main/spec/field_types.md#DigestSet"]
        #[serde(
            rename = "digest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub digest: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "name is the name of the Subject used here"]
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
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct UpdateOperationRequest {
        #[doc = "The operation to create."]
        #[serde(
            rename = "operation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation: ::std::option::Option<crate::schemas::Operation>,
        #[doc = "The fields to update."]
        #[serde(
            rename = "updateMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UpdateOperationRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateOperationRequest {
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
        #[doc = "The operating system classification of this Upgrade, as specified by the upstream operating system upgrade feed."]
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
        #[doc = "The cve that would be resolved by this upgrade."]
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
        #[doc = "Required - The package this Upgrade is for."]
        #[serde(
            rename = "package",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package: ::std::option::Option<String>,
        #[doc = "Required - The version of the package in machine + human readable form."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<crate::schemas::Version>,
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
        #[doc = "Required - The package this Upgrade is for."]
        #[serde(
            rename = "package",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package: ::std::option::Option<String>,
        #[doc = "Required - The version of the package in a machine + human readable form."]
        #[serde(
            rename = "parsedVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parsed_version: ::std::option::Option<crate::schemas::Version>,
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
        #[doc = "Whether this version is vulnerable, when defining the version bounds. For example, if the minimum version is 2.0, inclusive=true would say 2.0 is vulnerable, while inclusive=false would say it’s not"]
        #[serde(
            rename = "inclusive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inclusive: ::std::option::Option<bool>,
        #[doc = "Distinguish between sentinel MIN/MAX versions and normal versions. If kind is not NORMAL, then the other fields are ignored."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<crate::schemas::VersionKind>,
        #[doc = "The main part of the version name."]
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
        #[doc = "A special version representing positive infinity, other fields are ignored."]
        Maximum,
        #[doc = "A special version representing negative infinity, other fields are ignored."]
        Minimum,
        #[doc = "A standard package version, defined by the other fields."]
        Normal,
    }
    impl VersionKind {
        pub fn as_str(self) -> &'static str {
            match self {
                VersionKind::Maximum => "MAXIMUM",
                VersionKind::Minimum => "MINIMUM",
                VersionKind::Normal => "NORMAL",
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
    pub struct VulnerabilityDetails {
        #[doc = "Output only. The CVSS score of this vulnerability. CVSS score is on a scale of 0-10 where 0 indicates low severity and 10 indicates high severity."]
        #[serde(
            rename = "cvssScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cvss_score: ::std::option::Option<f32>,
        #[doc = "The CVSS v3 score of this vulnerability."]
        #[serde(
            rename = "cvssV3",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cvss_v3: ::std::option::Option<crate::schemas::Cvss>,
        #[doc = "The distro assigned severity for this vulnerability when that is available and note provider assigned severity when distro has not yet assigned a severity for this vulnerability. When there are multiple package issues for this vulnerability, they can have different effective severities because some might come from the distro and some might come from installed language packs (e.g. Maven JARs or Go binaries). For this reason, it is advised to use the effective severity on the PackageIssue level, as this field may eventually be deprecated. In the case where multiple PackageIssues have different effective severities, the one set here will be the highest severity of any of the PackageIssues."]
        #[serde(
            rename = "effectiveSeverity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub effective_severity:
            ::std::option::Option<crate::schemas::VulnerabilityDetailsEffectiveSeverity>,
        #[doc = "The set of affected locations and their fixes (if available) within the associated resource."]
        #[serde(
            rename = "packageIssue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package_issue: ::std::option::Option<Vec<crate::schemas::PackageIssue>>,
        #[doc = "The type of package; whether native or non native(ruby gems, node.js packages etc). This may be deprecated in the future because we can have multiple PackageIssues with different package types."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Output only. The note provider assigned Severity of the vulnerability."]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<crate::schemas::VulnerabilityDetailsSeverity>,
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityDetails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VulnerabilityDetailsEffectiveSeverity {
        #[doc = "Critical Impact"]
        Critical,
        #[doc = "High Impact"]
        High,
        #[doc = "Low Impact"]
        Low,
        #[doc = "Medium Impact"]
        Medium,
        #[doc = "Minimal Impact"]
        Minimal,
        #[doc = "Unknown Impact"]
        SeverityUnspecified,
    }
    impl VulnerabilityDetailsEffectiveSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                VulnerabilityDetailsEffectiveSeverity::Critical => "CRITICAL",
                VulnerabilityDetailsEffectiveSeverity::High => "HIGH",
                VulnerabilityDetailsEffectiveSeverity::Low => "LOW",
                VulnerabilityDetailsEffectiveSeverity::Medium => "MEDIUM",
                VulnerabilityDetailsEffectiveSeverity::Minimal => "MINIMAL",
                VulnerabilityDetailsEffectiveSeverity::SeverityUnspecified => {
                    "SEVERITY_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for VulnerabilityDetailsEffectiveSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VulnerabilityDetailsEffectiveSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<VulnerabilityDetailsEffectiveSeverity, ()> {
            Ok(match s {
                "CRITICAL" => VulnerabilityDetailsEffectiveSeverity::Critical,
                "HIGH" => VulnerabilityDetailsEffectiveSeverity::High,
                "LOW" => VulnerabilityDetailsEffectiveSeverity::Low,
                "MEDIUM" => VulnerabilityDetailsEffectiveSeverity::Medium,
                "MINIMAL" => VulnerabilityDetailsEffectiveSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => {
                    VulnerabilityDetailsEffectiveSeverity::SeverityUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VulnerabilityDetailsEffectiveSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VulnerabilityDetailsEffectiveSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VulnerabilityDetailsEffectiveSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRITICAL" => VulnerabilityDetailsEffectiveSeverity::Critical,
                "HIGH" => VulnerabilityDetailsEffectiveSeverity::High,
                "LOW" => VulnerabilityDetailsEffectiveSeverity::Low,
                "MEDIUM" => VulnerabilityDetailsEffectiveSeverity::Medium,
                "MINIMAL" => VulnerabilityDetailsEffectiveSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => {
                    VulnerabilityDetailsEffectiveSeverity::SeverityUnspecified
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
    impl ::google_field_selector::FieldSelector for VulnerabilityDetailsEffectiveSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityDetailsEffectiveSeverity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VulnerabilityDetailsSeverity {
        #[doc = "Critical Impact"]
        Critical,
        #[doc = "High Impact"]
        High,
        #[doc = "Low Impact"]
        Low,
        #[doc = "Medium Impact"]
        Medium,
        #[doc = "Minimal Impact"]
        Minimal,
        #[doc = "Unknown Impact"]
        SeverityUnspecified,
    }
    impl VulnerabilityDetailsSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                VulnerabilityDetailsSeverity::Critical => "CRITICAL",
                VulnerabilityDetailsSeverity::High => "HIGH",
                VulnerabilityDetailsSeverity::Low => "LOW",
                VulnerabilityDetailsSeverity::Medium => "MEDIUM",
                VulnerabilityDetailsSeverity::Minimal => "MINIMAL",
                VulnerabilityDetailsSeverity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for VulnerabilityDetailsSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VulnerabilityDetailsSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<VulnerabilityDetailsSeverity, ()> {
            Ok(match s {
                "CRITICAL" => VulnerabilityDetailsSeverity::Critical,
                "HIGH" => VulnerabilityDetailsSeverity::High,
                "LOW" => VulnerabilityDetailsSeverity::Low,
                "MEDIUM" => VulnerabilityDetailsSeverity::Medium,
                "MINIMAL" => VulnerabilityDetailsSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => VulnerabilityDetailsSeverity::SeverityUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VulnerabilityDetailsSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VulnerabilityDetailsSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VulnerabilityDetailsSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRITICAL" => VulnerabilityDetailsSeverity::Critical,
                "HIGH" => VulnerabilityDetailsSeverity::High,
                "LOW" => VulnerabilityDetailsSeverity::Low,
                "MEDIUM" => VulnerabilityDetailsSeverity::Medium,
                "MINIMAL" => VulnerabilityDetailsSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => VulnerabilityDetailsSeverity::SeverityUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityDetailsSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityDetailsSeverity {
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
    pub struct VulnerabilityLocation {
        #[doc = "The cpe_uri in \\[cpe format\\] (https://cpe.mitre.org/specification/) format. Examples include distro or storage location for vulnerable jar. This field can be used as a filter in list requests."]
        #[serde(
            rename = "cpeUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpe_uri: ::std::option::Option<String>,
        #[doc = "The package being described."]
        #[serde(
            rename = "package",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub package: ::std::option::Option<String>,
        #[doc = "The version of the package being described. This field can be used as a filter in list requests."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<crate::schemas::Version>,
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityLocation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VulnerabilityType {
        #[doc = "The CVSS score for this Vulnerability."]
        #[serde(
            rename = "cvssScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cvss_score: ::std::option::Option<f32>,
        #[doc = "All information about the package to specifically identify this vulnerability. One entry per (version range and cpe_uri) the package vulnerability has manifested in."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details: ::std::option::Option<Vec<crate::schemas::Detail>>,
        #[doc = "Note provider assigned impact of the vulnerability"]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<crate::schemas::VulnerabilityTypeSeverity>,
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VulnerabilityTypeSeverity {
        #[doc = "Critical Impact"]
        Critical,
        #[doc = "High Impact"]
        High,
        #[doc = "Low Impact"]
        Low,
        #[doc = "Medium Impact"]
        Medium,
        #[doc = "Minimal Impact"]
        Minimal,
        #[doc = "Unknown Impact"]
        SeverityUnspecified,
    }
    impl VulnerabilityTypeSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                VulnerabilityTypeSeverity::Critical => "CRITICAL",
                VulnerabilityTypeSeverity::High => "HIGH",
                VulnerabilityTypeSeverity::Low => "LOW",
                VulnerabilityTypeSeverity::Medium => "MEDIUM",
                VulnerabilityTypeSeverity::Minimal => "MINIMAL",
                VulnerabilityTypeSeverity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for VulnerabilityTypeSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VulnerabilityTypeSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<VulnerabilityTypeSeverity, ()> {
            Ok(match s {
                "CRITICAL" => VulnerabilityTypeSeverity::Critical,
                "HIGH" => VulnerabilityTypeSeverity::High,
                "LOW" => VulnerabilityTypeSeverity::Low,
                "MEDIUM" => VulnerabilityTypeSeverity::Medium,
                "MINIMAL" => VulnerabilityTypeSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => VulnerabilityTypeSeverity::SeverityUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VulnerabilityTypeSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VulnerabilityTypeSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VulnerabilityTypeSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRITICAL" => VulnerabilityTypeSeverity::Critical,
                "HIGH" => VulnerabilityTypeSeverity::High,
                "LOW" => VulnerabilityTypeSeverity::Low,
                "MEDIUM" => VulnerabilityTypeSeverity::Medium,
                "MINIMAL" => VulnerabilityTypeSeverity::Minimal,
                "SEVERITY_UNSPECIFIED" => VulnerabilityTypeSeverity::SeverityUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for VulnerabilityTypeSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VulnerabilityTypeSeverity {
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
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the providers resource"]
    pub fn providers(&self) -> crate::resources::providers::ProvidersActions {
        crate::resources::providers::ProvidersActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
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
            #[doc = "Actions that can be performed on the operations resource"]
            pub fn operations(&self) -> crate::resources::projects::operations::OperationsActions {
                crate::resources::projects::operations::OperationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the scan_configs resource"]
            pub fn scan_configs(
                &self,
            ) -> crate::resources::projects::scan_configs::ScanConfigsActions {
                crate::resources::projects::scan_configs::ScanConfigsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod notes {
            pub mod params {}
            pub struct NotesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> NotesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a new `Note`."]
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
                        name: None,
                        note_id: None,
                    }
                }
                #[doc = "Deletes the given `Note` from the system."]
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
                #[doc = "Returns the requested `Note`."]
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
                #[doc = "Gets the access control policy for a note or an `Occurrence` resource. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or occurrence, respectively. Attempting to call this method on a resource without the required permission will result in a `PERMISSION_DENIED` error. Attempting to call this method on a non-existent resource will result in a `NOT_FOUND` error if the user has list permission on the project, or a `PERMISSION_DENIED` error otherwise. The resource takes the following formats: `projects/{PROJECT_ID}/occurrences/{OCCURRENCE_ID}` for occurrences and projects/{PROJECT_ID}/notes/{NOTE_ID} for notes"]
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
                #[doc = "Lists all `Notes` for a given project."]
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
                        name: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates an existing `Note`."]
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
                #[doc = "Sets the access control policy on the specified `Note` or `Occurrence`. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a `Note` or an `Occurrence`, respectively. Attempting to call this method without these permissions will result in a ` `PERMISSION_DENIED`error. Attempting to call this method on a non-existent resource will result in a`NOT_FOUND`error if the user has`containeranalysis.notes.list`permission on a`Note`or`containeranalysis.occurrences.list`on an`Occurrence`, or a `PERMISSION_DENIED`error otherwise. The resource takes the following formats:`projects/{projectid}/occurrences/{occurrenceid}\\` for occurrences and projects/{projectid}/notes/{noteid} for notes"]
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
                #[doc = "Returns the permissions that a caller has on the specified note or occurrence resource. Requires list permission on the project (for example, “storage.objects.list” on the containing bucket for testing permission of an object). Attempting to call this method on a non-existent resource will result in a `NOT_FOUND` error if the user has list permission on the project, or a `PERMISSION_DENIED` error otherwise. The resource takes the following formats: `projects/{PROJECT_ID}/occurrences/{OCCURRENCE_ID}` for `Occurrences` and `projects/{PROJECT_ID}/notes/{NOTE_ID}` for `Notes`"]
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
            #[doc = "Created via [NotesActions::create()](struct.NotesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Note,
                parent: String,
                name: ::std::option::Option<String>,
                note_id: ::std::option::Option<String>,
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
            impl<'a> CreateRequestBuilder<'a> {
                #[doc = "The name of the project. Should be of the form “providers/{provider_id}”. @Deprecated"]
                pub fn name(mut self, value: impl Into<String>) -> Self {
                    self.name = Some(value.into());
                    self
                }
                #[doc = "The ID to use for this note."]
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
                ) -> Result<crate::schemas::Note, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Note, crate::Error> {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("name", &self.name)]);
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [NotesActions::delete()](struct.NotesActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [NotesActions::get()](struct.NotesActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
                ) -> Result<crate::schemas::Note, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Note, crate::Error> {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [NotesActions::get_iam_policy()](struct.NotesActions.html#method.get_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct GetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GetIamPolicyRequest,
                resource: String,
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
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
            #[doc = "Created via [NotesActions::list()](struct.NotesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: ::std::option::Option<String>,
                name: ::std::option::Option<String>,
                page_size: ::std::option::Option<i32>,
                page_token: ::std::option::Option<String>,
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
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The filter expression."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "The name field will contain the project Id for example: “providers/{provider_id} @Deprecated"]
                pub fn name(mut self, value: impl Into<String>) -> Self {
                    self.name = Some(value.into());
                    self
                }
                #[doc = "Number of notes to return in the list."]
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
                #[doc = "\nExecute the request and yield each item in the `notes` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_notes<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_notes_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `notes` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_notes_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Note, crate::Error>> + 'a
                {
                    self.stream_notes_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `notes` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_notes_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Note, crate::Error>> + 'a
                {
                    self.stream_notes_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `notes` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_notes_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + 'a,
                    F: AsRef<str>,
                {
                    #[derive(:: serde :: Deserialize, :: serde :: Serialize)]
                    struct Page<T> {
                        #[serde(rename = "nextPageToken")]
                        pub next_page_token: ::std::option::Option<String>,
                        #[serde(rename = "notes")]
                        pub items: Vec<T>,
                    }
                    impl<T> crate::GetNextPageToken for Page<T> {
                        fn next_page_token(&self) -> ::std::option::Option<String> {
                            self.next_page_token.to_owned()
                        }
                    }
                    impl<T> crate::stream::IntoPageItems for Page<T> {
                        type Items = Vec<T>;
                        fn into_page_items(self) -> Self::Items {
                            self.items
                        }
                    }
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
                    crate::stream::page_item_stream::<_, Page<T>>(self)
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Requests the field given by the [`FieldSelector`] implementation from the server."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                #[doc = r" [`FieldSelector`]: ::google_field_selector::FieldSelector"]
                pub fn stream<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken
                        + ::serde::de::DeserializeOwned
                        + ::google_field_selector::FieldSelector
                        + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_with_fields(fields)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests the default set of fields from the server."]
                pub fn stream_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::ListNotesResponse, crate::Error>,
                > + 'a {
                    self.stream_with_fields(None::<&str>)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests all fields from the server."]
                pub fn stream_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::ListNotesResponse, crate::Error>,
                > + 'a {
                    self.stream_with_fields(Some("*"))
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Only the given `fields` are requested from the server. If the list of fields is not"]
                #[doc = r" empty, the `nextPageToken` field will be added to the list."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                pub fn stream_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken + ::serde::de::DeserializeOwned + 'a,
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
                    crate::stream::page_stream(self)
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
                ) -> Result<crate::schemas::ListNotesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListNotesResponse, crate::Error> {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("name", &self.name)]);
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[async_trait::async_trait]
            impl<'a> crate::stream::StreamableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                async fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: crate::GetNextPageToken + ::serde::de::DeserializeOwned,
                {
                    self._execute().await
                }
            }
            #[doc = "Created via [NotesActions::patch()](struct.NotesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Note,
                name: String,
                update_mask: ::std::option::Option<String>,
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
                ) -> Result<crate::schemas::Note, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Note, crate::Error> {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [NotesActions::set_iam_policy()](struct.NotesActions.html#method.set_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct SetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::SetIamPolicyRequest,
                resource: String,
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
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
            #[doc = "Created via [NotesActions::test_iam_permissions()](struct.NotesActions.html#method.test_iam_permissions)"]
            #[derive(Debug, Clone)]
            pub struct TestIamPermissionsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::TestIamPermissionsRequest,
                resource: String,
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
                ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
                {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
            pub mod occurrences {
                pub mod params {}
                pub struct OccurrencesActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> OccurrencesActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Lists `Occurrences` referencing the specified `Note`. Use this method to get all occurrences referencing your `Note` across all your customer projects."]
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
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    filter: ::std::option::Option<String>,
                    page_size: ::std::option::Option<i32>,
                    page_token: ::std::option::Option<String>,
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
                impl<'a> ListRequestBuilder<'a> {
                    #[doc = "The filter expression."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "Number of notes to return in the list."]
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
                    #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_occurrences<T>(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned
                            + ::google_field_selector::FieldSelector
                            + 'a,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: ::std::option::Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.stream_occurrences_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_occurrences_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::Occurrence, crate::Error>,
                    > + 'a {
                        self.stream_occurrences_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_occurrences_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::Occurrence, crate::Error>,
                    > + 'a {
                        self.stream_occurrences_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_occurrences_with_fields<T, F>(
                        mut self,
                        fields: ::std::option::Option<F>,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + 'a,
                        F: AsRef<str>,
                    {
                        #[derive(:: serde :: Deserialize, :: serde :: Serialize)]
                        struct Page<T> {
                            #[serde(rename = "nextPageToken")]
                            pub next_page_token: ::std::option::Option<String>,
                            #[serde(rename = "occurrences")]
                            pub items: Vec<T>,
                        }
                        impl<T> crate::GetNextPageToken for Page<T> {
                            fn next_page_token(&self) -> ::std::option::Option<String> {
                                self.next_page_token.to_owned()
                            }
                        }
                        impl<T> crate::stream::IntoPageItems for Page<T> {
                            type Items = Vec<T>;
                            fn into_page_items(self) -> Self::Items {
                                self.items
                            }
                        }
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
                        crate::stream::page_item_stream::<_, Page<T>>(self)
                    }
                    #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                    #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                    #[doc = r" token is returned."]
                    #[doc = r""]
                    #[doc = r" Requests the field given by the [`FieldSelector`] implementation from the server."]
                    #[doc = r""]
                    #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                    #[doc = r" [`FieldSelector`]: ::google_field_selector::FieldSelector"]
                    pub fn stream<T>(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: crate::GetNextPageToken
                            + ::serde::de::DeserializeOwned
                            + ::google_field_selector::FieldSelector
                            + 'a,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: ::std::option::Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.stream_with_fields(fields)
                    }
                    #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                    #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                    #[doc = r" repeated until no page token is returned."]
                    #[doc = r""]
                    #[doc = r" Requests the default set of fields from the server."]
                    pub fn stream_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::ListNoteOccurrencesResponse, crate::Error>,
                    > + 'a {
                        self.stream_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                    #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                    #[doc = r" repeated until no page token is returned."]
                    #[doc = r""]
                    #[doc = r" Requests all fields from the server."]
                    pub fn stream_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::ListNoteOccurrencesResponse, crate::Error>,
                    > + 'a {
                        self.stream_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                    #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                    #[doc = r" token is returned."]
                    #[doc = r""]
                    #[doc = r" Only the given `fields` are requested from the server. If the list of fields is not"]
                    #[doc = r" empty, the `nextPageToken` field will be added to the list."]
                    #[doc = r""]
                    #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                    pub fn stream_with_fields<T, F>(
                        mut self,
                        fields: ::std::option::Option<F>,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: crate::GetNextPageToken + ::serde::de::DeserializeOwned + 'a,
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
                        crate::stream::page_stream(self)
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
                    ) -> Result<crate::schemas::ListNoteOccurrencesResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListNoteOccurrencesResponse, crate::Error>
                    {
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                        output.push_str("v1alpha1/");
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
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                        let access_token = self
                            .auth
                            .access_token()
                            .await
                            .map_err(|err| crate::Error::OAuth2(err))?;
                        req = req.bearer_auth(access_token);
                        Ok(req)
                    }
                }
                #[async_trait::async_trait]
                impl<'a> crate::stream::StreamableMethod for ListRequestBuilder<'a> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    async fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: crate::GetNextPageToken + ::serde::de::DeserializeOwned,
                    {
                        self._execute().await
                    }
                }
            }
        }
        pub mod occurrences {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListKind {
                    #[doc = "This represents a logical “role” that can attest to artifacts."]
                    AttestationAuthority,
                    #[doc = "The note and occurrence assert build provenance."]
                    BuildDetails,
                    #[doc = "This represents a compliance check that can be applied to a resource."]
                    Compliance,
                    #[doc = "The note and occurrence track deployment events."]
                    Deployable,
                    #[doc = "The note and occurrence track the initial discovery status of a resource."]
                    Discovery,
                    #[doc = "This represents a DSSE attestation Note"]
                    DsseAttestation,
                    #[doc = "This represents an image basis relationship."]
                    ImageBasis,
                    #[doc = "Unknown"]
                    KindUnspecified,
                    #[doc = "This represents a package installed via a package manager."]
                    PackageManager,
                    #[doc = "The note and occurrence represent a package vulnerability."]
                    PackageVulnerability,
                    #[doc = "This represents a software bill of materials."]
                    Sbom,
                    #[doc = "This represents an SPDX File."]
                    SpdxFile,
                    #[doc = "This represents an SPDX Package."]
                    SpdxPackage,
                    #[doc = "This represents an SPDX Relationship."]
                    SpdxRelationship,
                    #[doc = "This represents an available software upgrade."]
                    Upgrade,
                }
                impl ListKind {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListKind::AttestationAuthority => "ATTESTATION_AUTHORITY",
                            ListKind::BuildDetails => "BUILD_DETAILS",
                            ListKind::Compliance => "COMPLIANCE",
                            ListKind::Deployable => "DEPLOYABLE",
                            ListKind::Discovery => "DISCOVERY",
                            ListKind::DsseAttestation => "DSSE_ATTESTATION",
                            ListKind::ImageBasis => "IMAGE_BASIS",
                            ListKind::KindUnspecified => "KIND_UNSPECIFIED",
                            ListKind::PackageManager => "PACKAGE_MANAGER",
                            ListKind::PackageVulnerability => "PACKAGE_VULNERABILITY",
                            ListKind::Sbom => "SBOM",
                            ListKind::SpdxFile => "SPDX_FILE",
                            ListKind::SpdxPackage => "SPDX_PACKAGE",
                            ListKind::SpdxRelationship => "SPDX_RELATIONSHIP",
                            ListKind::Upgrade => "UPGRADE",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListKind {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListKind {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListKind, ()> {
                        Ok(match s {
                            "ATTESTATION_AUTHORITY" => ListKind::AttestationAuthority,
                            "BUILD_DETAILS" => ListKind::BuildDetails,
                            "COMPLIANCE" => ListKind::Compliance,
                            "DEPLOYABLE" => ListKind::Deployable,
                            "DISCOVERY" => ListKind::Discovery,
                            "DSSE_ATTESTATION" => ListKind::DsseAttestation,
                            "IMAGE_BASIS" => ListKind::ImageBasis,
                            "KIND_UNSPECIFIED" => ListKind::KindUnspecified,
                            "PACKAGE_MANAGER" => ListKind::PackageManager,
                            "PACKAGE_VULNERABILITY" => ListKind::PackageVulnerability,
                            "SBOM" => ListKind::Sbom,
                            "SPDX_FILE" => ListKind::SpdxFile,
                            "SPDX_PACKAGE" => ListKind::SpdxPackage,
                            "SPDX_RELATIONSHIP" => ListKind::SpdxRelationship,
                            "UPGRADE" => ListKind::Upgrade,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListKind {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListKind {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListKind {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ATTESTATION_AUTHORITY" => ListKind::AttestationAuthority,
                            "BUILD_DETAILS" => ListKind::BuildDetails,
                            "COMPLIANCE" => ListKind::Compliance,
                            "DEPLOYABLE" => ListKind::Deployable,
                            "DISCOVERY" => ListKind::Discovery,
                            "DSSE_ATTESTATION" => ListKind::DsseAttestation,
                            "IMAGE_BASIS" => ListKind::ImageBasis,
                            "KIND_UNSPECIFIED" => ListKind::KindUnspecified,
                            "PACKAGE_MANAGER" => ListKind::PackageManager,
                            "PACKAGE_VULNERABILITY" => ListKind::PackageVulnerability,
                            "SBOM" => ListKind::Sbom,
                            "SPDX_FILE" => ListKind::SpdxFile,
                            "SPDX_PACKAGE" => ListKind::SpdxPackage,
                            "SPDX_RELATIONSHIP" => ListKind::SpdxRelationship,
                            "UPGRADE" => ListKind::Upgrade,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListKind {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListKind {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct OccurrencesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> OccurrencesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a new `Occurrence`. Use this method to create `Occurrences` for a resource."]
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
                        name: None,
                    }
                }
                #[doc = "Deletes the given `Occurrence` from the system. Use this when an `Occurrence` is no longer applicable for the given resource."]
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
                #[doc = "Returns the requested `Occurrence`."]
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
                #[doc = "Gets the access control policy for a note or an `Occurrence` resource. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or occurrence, respectively. Attempting to call this method on a resource without the required permission will result in a `PERMISSION_DENIED` error. Attempting to call this method on a non-existent resource will result in a `NOT_FOUND` error if the user has list permission on the project, or a `PERMISSION_DENIED` error otherwise. The resource takes the following formats: `projects/{PROJECT_ID}/occurrences/{OCCURRENCE_ID}` for occurrences and projects/{PROJECT_ID}/notes/{NOTE_ID} for notes"]
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
                #[doc = "Gets the `Note` attached to the given `Occurrence`."]
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
                #[doc = "Lists active `Occurrences` for a given project matching the filters."]
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
                        kind: None,
                        name: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates an existing occurrence."]
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
                #[doc = "Sets the access control policy on the specified `Note` or `Occurrence`. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a `Note` or an `Occurrence`, respectively. Attempting to call this method without these permissions will result in a ` `PERMISSION_DENIED`error. Attempting to call this method on a non-existent resource will result in a`NOT_FOUND`error if the user has`containeranalysis.notes.list`permission on a`Note`or`containeranalysis.occurrences.list`on an`Occurrence`, or a `PERMISSION_DENIED`error otherwise. The resource takes the following formats:`projects/{projectid}/occurrences/{occurrenceid}\\` for occurrences and projects/{projectid}/notes/{noteid} for notes"]
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
                #[doc = "Returns the permissions that a caller has on the specified note or occurrence resource. Requires list permission on the project (for example, “storage.objects.list” on the containing bucket for testing permission of an object). Attempting to call this method on a non-existent resource will result in a `NOT_FOUND` error if the user has list permission on the project, or a `PERMISSION_DENIED` error otherwise. The resource takes the following formats: `projects/{PROJECT_ID}/occurrences/{OCCURRENCE_ID}` for `Occurrences` and `projects/{PROJECT_ID}/notes/{NOTE_ID}` for `Notes`"]
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
            #[doc = "Created via [OccurrencesActions::create()](struct.OccurrencesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Occurrence,
                parent: String,
                name: ::std::option::Option<String>,
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
            impl<'a> CreateRequestBuilder<'a> {
                #[doc = "The name of the project. Should be of the form “projects/{project_id}”. @Deprecated"]
                pub fn name(mut self, value: impl Into<String>) -> Self {
                    self.name = Some(value.into());
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
                ) -> Result<crate::schemas::Occurrence, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Occurrence, crate::Error> {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("name", &self.name)]);
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
            #[doc = "Created via [OccurrencesActions::delete()](struct.OccurrencesActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [OccurrencesActions::get()](struct.OccurrencesActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
                ) -> Result<crate::schemas::Occurrence, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Occurrence, crate::Error> {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [OccurrencesActions::get_iam_policy()](struct.OccurrencesActions.html#method.get_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct GetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GetIamPolicyRequest,
                resource: String,
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
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
            #[doc = "Created via [OccurrencesActions::get_notes()](struct.OccurrencesActions.html#method.get_notes)"]
            #[derive(Debug, Clone)]
            pub struct GetNotesRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
                ) -> Result<crate::schemas::Note, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Note, crate::Error> {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [OccurrencesActions::get_vulnerability_summary()](struct.OccurrencesActions.html#method.get_vulnerability_summary)"]
            #[derive(Debug, Clone)]
            pub struct GetVulnerabilitySummaryRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: ::std::option::Option<String>,
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
                ) -> Result<crate::schemas::GetVulnzOccurrencesSummaryResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GetVulnzOccurrencesSummaryResponse, crate::Error>
                {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [OccurrencesActions::list()](struct.OccurrencesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: ::std::option::Option<String>,
                kind: ::std::option::Option<
                    crate::resources::projects::occurrences::params::ListKind,
                >,
                name: ::std::option::Option<String>,
                page_size: ::std::option::Option<i32>,
                page_token: ::std::option::Option<String>,
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
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The filter expression."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "The kind of occurrences to filter on."]
                pub fn kind(
                    mut self,
                    value: crate::resources::projects::occurrences::params::ListKind,
                ) -> Self {
                    self.kind = Some(value);
                    self
                }
                #[doc = "The name field contains the project Id. For example: “projects/{project_id} @Deprecated"]
                pub fn name(mut self, value: impl Into<String>) -> Self {
                    self.name = Some(value.into());
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
                #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_occurrences<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_occurrences_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_occurrences_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Occurrence, crate::Error>> + 'a
                {
                    self.stream_occurrences_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_occurrences_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Occurrence, crate::Error>> + 'a
                {
                    self.stream_occurrences_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_occurrences_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + 'a,
                    F: AsRef<str>,
                {
                    #[derive(:: serde :: Deserialize, :: serde :: Serialize)]
                    struct Page<T> {
                        #[serde(rename = "nextPageToken")]
                        pub next_page_token: ::std::option::Option<String>,
                        #[serde(rename = "occurrences")]
                        pub items: Vec<T>,
                    }
                    impl<T> crate::GetNextPageToken for Page<T> {
                        fn next_page_token(&self) -> ::std::option::Option<String> {
                            self.next_page_token.to_owned()
                        }
                    }
                    impl<T> crate::stream::IntoPageItems for Page<T> {
                        type Items = Vec<T>;
                        fn into_page_items(self) -> Self::Items {
                            self.items
                        }
                    }
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
                    crate::stream::page_item_stream::<_, Page<T>>(self)
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Requests the field given by the [`FieldSelector`] implementation from the server."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                #[doc = r" [`FieldSelector`]: ::google_field_selector::FieldSelector"]
                pub fn stream<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken
                        + ::serde::de::DeserializeOwned
                        + ::google_field_selector::FieldSelector
                        + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_with_fields(fields)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests the default set of fields from the server."]
                pub fn stream_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::ListOccurrencesResponse, crate::Error>,
                > + 'a {
                    self.stream_with_fields(None::<&str>)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests all fields from the server."]
                pub fn stream_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::ListOccurrencesResponse, crate::Error>,
                > + 'a {
                    self.stream_with_fields(Some("*"))
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Only the given `fields` are requested from the server. If the list of fields is not"]
                #[doc = r" empty, the `nextPageToken` field will be added to the list."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                pub fn stream_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken + ::serde::de::DeserializeOwned + 'a,
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
                    crate::stream::page_stream(self)
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
                ) -> Result<crate::schemas::ListOccurrencesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListOccurrencesResponse, crate::Error> {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("kind", &self.kind)]);
                    req = req.query(&[("name", &self.name)]);
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[async_trait::async_trait]
            impl<'a> crate::stream::StreamableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                async fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: crate::GetNextPageToken + ::serde::de::DeserializeOwned,
                {
                    self._execute().await
                }
            }
            #[doc = "Created via [OccurrencesActions::patch()](struct.OccurrencesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Occurrence,
                name: String,
                update_mask: ::std::option::Option<String>,
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
                ) -> Result<crate::schemas::Occurrence, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Occurrence, crate::Error> {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [OccurrencesActions::set_iam_policy()](struct.OccurrencesActions.html#method.set_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct SetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::SetIamPolicyRequest,
                resource: String,
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
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
            #[doc = "Created via [OccurrencesActions::test_iam_permissions()](struct.OccurrencesActions.html#method.test_iam_permissions)"]
            #[derive(Debug, Clone)]
            pub struct TestIamPermissionsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::TestIamPermissionsRequest,
                resource: String,
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
                ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
                {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
        pub mod operations {
            pub mod params {}
            pub struct OperationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> OperationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a new `Operation`."]
                pub fn create(
                    &self,
                    request: crate::schemas::CreateOperationRequest,
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
                #[doc = "Updates an existing operation returns an error if operation does not exist. The only valid operations are to update mark the done bit change the result."]
                pub fn patch(
                    &self,
                    request: crate::schemas::UpdateOperationRequest,
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
                    }
                }
            }
            #[doc = "Created via [OperationsActions::create()](struct.OperationsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::CreateOperationRequest,
                parent: String,
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
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/operations");
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
            #[doc = "Created via [OperationsActions::patch()](struct.OperationsActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::UpdateOperationRequest,
                name: String,
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
            impl<'a> PatchRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
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
        pub mod scan_configs {
            pub mod params {}
            pub struct ScanConfigsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ScanConfigsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Gets a specific scan configuration for a project."]
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
                #[doc = "Lists scan configurations for a project."]
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
                #[doc = "Updates the scan configuration to a new value."]
                pub fn patch(
                    &self,
                    request: crate::schemas::ScanConfig,
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
            }
            #[doc = "Created via [ScanConfigsActions::get()](struct.ScanConfigsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
                ) -> Result<crate::schemas::ScanConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ScanConfig, crate::Error> {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [ScanConfigsActions::list()](struct.ScanConfigsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: ::std::option::Option<String>,
                page_size: ::std::option::Option<i32>,
                page_token: ::std::option::Option<String>,
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
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The filter expression."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "The number of items to return."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The page token to use for the next request."]
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
                #[doc = "\nExecute the request and yield each item in the `scanConfigs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_scan_configs<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_scan_configs_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `scanConfigs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_scan_configs_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::ScanConfig, crate::Error>> + 'a
                {
                    self.stream_scan_configs_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `scanConfigs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_scan_configs_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::ScanConfig, crate::Error>> + 'a
                {
                    self.stream_scan_configs_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `scanConfigs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_scan_configs_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + 'a,
                    F: AsRef<str>,
                {
                    #[derive(:: serde :: Deserialize, :: serde :: Serialize)]
                    struct Page<T> {
                        #[serde(rename = "nextPageToken")]
                        pub next_page_token: ::std::option::Option<String>,
                        #[serde(rename = "scanConfigs")]
                        pub items: Vec<T>,
                    }
                    impl<T> crate::GetNextPageToken for Page<T> {
                        fn next_page_token(&self) -> ::std::option::Option<String> {
                            self.next_page_token.to_owned()
                        }
                    }
                    impl<T> crate::stream::IntoPageItems for Page<T> {
                        type Items = Vec<T>;
                        fn into_page_items(self) -> Self::Items {
                            self.items
                        }
                    }
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "scanConfigs").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::stream::page_item_stream::<_, Page<T>>(self)
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Requests the field given by the [`FieldSelector`] implementation from the server."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                #[doc = r" [`FieldSelector`]: ::google_field_selector::FieldSelector"]
                pub fn stream<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken
                        + ::serde::de::DeserializeOwned
                        + ::google_field_selector::FieldSelector
                        + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_with_fields(fields)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests the default set of fields from the server."]
                pub fn stream_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::ListScanConfigsResponse, crate::Error>,
                > + 'a {
                    self.stream_with_fields(None::<&str>)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests all fields from the server."]
                pub fn stream_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::ListScanConfigsResponse, crate::Error>,
                > + 'a {
                    self.stream_with_fields(Some("*"))
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Only the given `fields` are requested from the server. If the list of fields is not"]
                #[doc = r" empty, the `nextPageToken` field will be added to the list."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                pub fn stream_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken + ::serde::de::DeserializeOwned + 'a,
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
                    crate::stream::page_stream(self)
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
                ) -> Result<crate::schemas::ListScanConfigsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListScanConfigsResponse, crate::Error> {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/scanConfigs");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[async_trait::async_trait]
            impl<'a> crate::stream::StreamableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                async fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: crate::GetNextPageToken + ::serde::de::DeserializeOwned,
                {
                    self._execute().await
                }
            }
            #[doc = "Created via [ScanConfigsActions::patch()](struct.ScanConfigsActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::ScanConfig,
                name: String,
                update_mask: ::std::option::Option<String>,
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
                ) -> Result<crate::schemas::ScanConfig, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ScanConfig, crate::Error> {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
    pub mod providers {
        pub mod params {}
        pub struct ProvidersActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProvidersActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the notes resource"]
            pub fn notes(&self) -> crate::resources::providers::notes::NotesActions {
                crate::resources::providers::notes::NotesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod notes {
            pub mod params {}
            pub struct NotesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> NotesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a new `Note`."]
                pub fn create(
                    &self,
                    request: crate::schemas::Note,
                    name: impl Into<String>,
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
                        name: name.into(),
                        note_id: None,
                        parent: None,
                    }
                }
                #[doc = "Deletes the given `Note` from the system."]
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
                #[doc = "Returns the requested `Note`."]
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
                #[doc = "Gets the access control policy for a note or an `Occurrence` resource. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a note or occurrence, respectively. Attempting to call this method on a resource without the required permission will result in a `PERMISSION_DENIED` error. Attempting to call this method on a non-existent resource will result in a `NOT_FOUND` error if the user has list permission on the project, or a `PERMISSION_DENIED` error otherwise. The resource takes the following formats: `projects/{PROJECT_ID}/occurrences/{OCCURRENCE_ID}` for occurrences and projects/{PROJECT_ID}/notes/{NOTE_ID} for notes"]
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
                #[doc = "Lists all `Notes` for a given project."]
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
                        parent: None,
                    }
                }
                #[doc = "Updates an existing `Note`."]
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
                #[doc = "Sets the access control policy on the specified `Note` or `Occurrence`. Requires `containeranalysis.notes.setIamPolicy` or `containeranalysis.occurrences.setIamPolicy` permission if the resource is a `Note` or an `Occurrence`, respectively. Attempting to call this method without these permissions will result in a ` `PERMISSION_DENIED`error. Attempting to call this method on a non-existent resource will result in a`NOT_FOUND`error if the user has`containeranalysis.notes.list`permission on a`Note`or`containeranalysis.occurrences.list`on an`Occurrence`, or a `PERMISSION_DENIED`error otherwise. The resource takes the following formats:`projects/{projectid}/occurrences/{occurrenceid}\\` for occurrences and projects/{projectid}/notes/{noteid} for notes"]
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
                #[doc = "Returns the permissions that a caller has on the specified note or occurrence resource. Requires list permission on the project (for example, “storage.objects.list” on the containing bucket for testing permission of an object). Attempting to call this method on a non-existent resource will result in a `NOT_FOUND` error if the user has list permission on the project, or a `PERMISSION_DENIED` error otherwise. The resource takes the following formats: `projects/{PROJECT_ID}/occurrences/{OCCURRENCE_ID}` for `Occurrences` and `projects/{PROJECT_ID}/notes/{NOTE_ID}` for `Notes`"]
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
                ) -> crate::resources::providers::notes::occurrences::OccurrencesActions
                {
                    crate::resources::providers::notes::occurrences::OccurrencesActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [NotesActions::create()](struct.NotesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Note,
                name: String,
                note_id: ::std::option::Option<String>,
                parent: ::std::option::Option<String>,
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
            impl<'a> CreateRequestBuilder<'a> {
                #[doc = "The ID to use for this note."]
                pub fn note_id(mut self, value: impl Into<String>) -> Self {
                    self.note_id = Some(value.into());
                    self
                }
                #[doc = "This field contains the project Id for example: “projects/{project_id}"]
                pub fn parent(mut self, value: impl Into<String>) -> Self {
                    self.parent = Some(value.into());
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
                ) -> Result<crate::schemas::Note, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Note, crate::Error> {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("noteId", &self.note_id)]);
                    req = req.query(&[("parent", &self.parent)]);
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
            #[doc = "Created via [NotesActions::delete()](struct.NotesActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
                ) -> Result<crate::schemas::Empty, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Empty, crate::Error> {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [NotesActions::get()](struct.NotesActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
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
                ) -> Result<crate::schemas::Note, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Note, crate::Error> {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [NotesActions::get_iam_policy()](struct.NotesActions.html#method.get_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct GetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GetIamPolicyRequest,
                resource: String,
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
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
            #[doc = "Created via [NotesActions::list()](struct.NotesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                filter: ::std::option::Option<String>,
                page_size: ::std::option::Option<i32>,
                page_token: ::std::option::Option<String>,
                parent: ::std::option::Option<String>,
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
            impl<'a> ListRequestBuilder<'a> {
                #[doc = "The filter expression."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Number of notes to return in the list."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Token to provide to skip to a particular spot in the list."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "This field contains the project Id for example: “projects/{PROJECT_ID}”."]
                pub fn parent(mut self, value: impl Into<String>) -> Self {
                    self.parent = Some(value.into());
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
                #[doc = "\nExecute the request and yield each item in the `notes` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_notes<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_notes_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `notes` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_notes_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Note, crate::Error>> + 'a
                {
                    self.stream_notes_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `notes` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_notes_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Note, crate::Error>> + 'a
                {
                    self.stream_notes_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `notes` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_notes_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + 'a,
                    F: AsRef<str>,
                {
                    #[derive(:: serde :: Deserialize, :: serde :: Serialize)]
                    struct Page<T> {
                        #[serde(rename = "nextPageToken")]
                        pub next_page_token: ::std::option::Option<String>,
                        #[serde(rename = "notes")]
                        pub items: Vec<T>,
                    }
                    impl<T> crate::GetNextPageToken for Page<T> {
                        fn next_page_token(&self) -> ::std::option::Option<String> {
                            self.next_page_token.to_owned()
                        }
                    }
                    impl<T> crate::stream::IntoPageItems for Page<T> {
                        type Items = Vec<T>;
                        fn into_page_items(self) -> Self::Items {
                            self.items
                        }
                    }
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
                    crate::stream::page_item_stream::<_, Page<T>>(self)
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Requests the field given by the [`FieldSelector`] implementation from the server."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                #[doc = r" [`FieldSelector`]: ::google_field_selector::FieldSelector"]
                pub fn stream<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken
                        + ::serde::de::DeserializeOwned
                        + ::google_field_selector::FieldSelector
                        + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_with_fields(fields)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests the default set of fields from the server."]
                pub fn stream_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::ListNotesResponse, crate::Error>,
                > + 'a {
                    self.stream_with_fields(None::<&str>)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests all fields from the server."]
                pub fn stream_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::ListNotesResponse, crate::Error>,
                > + 'a {
                    self.stream_with_fields(Some("*"))
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Only the given `fields` are requested from the server. If the list of fields is not"]
                #[doc = r" empty, the `nextPageToken` field will be added to the list."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                pub fn stream_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken + ::serde::de::DeserializeOwned + 'a,
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
                    crate::stream::page_stream(self)
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
                ) -> Result<crate::schemas::ListNotesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListNotesResponse, crate::Error> {
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
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("pageSize", &self.page_size)]);
                    req = req.query(&[("pageToken", &self.page_token)]);
                    req = req.query(&[("parent", &self.parent)]);
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
            #[async_trait::async_trait]
            impl<'a> crate::stream::StreamableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                async fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: crate::GetNextPageToken + ::serde::de::DeserializeOwned,
                {
                    self._execute().await
                }
            }
            #[doc = "Created via [NotesActions::patch()](struct.NotesActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Note,
                name: String,
                update_mask: ::std::option::Option<String>,
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
                ) -> Result<crate::schemas::Note, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Note, crate::Error> {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                    let access_token = self
                        .auth
                        .access_token()
                        .await
                        .map_err(|err| crate::Error::OAuth2(err))?;
                    req = req.bearer_auth(access_token);
                    Ok(req)
                }
            }
            #[doc = "Created via [NotesActions::set_iam_policy()](struct.NotesActions.html#method.set_iam_policy)"]
            #[derive(Debug, Clone)]
            pub struct SetIamPolicyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::SetIamPolicyRequest,
                resource: String,
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
                ) -> Result<crate::schemas::Policy, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Policy, crate::Error> {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
            #[doc = "Created via [NotesActions::test_iam_permissions()](struct.NotesActions.html#method.test_iam_permissions)"]
            #[derive(Debug, Clone)]
            pub struct TestIamPermissionsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::TestIamPermissionsRequest,
                resource: String,
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
                ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::TestIamPermissionsResponse, crate::Error>
                {
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
                    let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                    output.push_str("v1alpha1/");
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
            pub mod occurrences {
                pub mod params {}
                pub struct OccurrencesActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> OccurrencesActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Lists `Occurrences` referencing the specified `Note`. Use this method to get all occurrences referencing your `Note` across all your customer projects."]
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
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    filter: ::std::option::Option<String>,
                    page_size: ::std::option::Option<i32>,
                    page_token: ::std::option::Option<String>,
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
                impl<'a> ListRequestBuilder<'a> {
                    #[doc = "The filter expression."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "Number of notes to return in the list."]
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
                    #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_occurrences<T>(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned
                            + ::google_field_selector::FieldSelector
                            + 'a,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: ::std::option::Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.stream_occurrences_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_occurrences_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::Occurrence, crate::Error>,
                    > + 'a {
                        self.stream_occurrences_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_occurrences_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::Occurrence, crate::Error>,
                    > + 'a {
                        self.stream_occurrences_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `occurrences` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_occurrences_with_fields<T, F>(
                        mut self,
                        fields: ::std::option::Option<F>,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: ::serde::de::DeserializeOwned + 'a,
                        F: AsRef<str>,
                    {
                        #[derive(:: serde :: Deserialize, :: serde :: Serialize)]
                        struct Page<T> {
                            #[serde(rename = "nextPageToken")]
                            pub next_page_token: ::std::option::Option<String>,
                            #[serde(rename = "occurrences")]
                            pub items: Vec<T>,
                        }
                        impl<T> crate::GetNextPageToken for Page<T> {
                            fn next_page_token(&self) -> ::std::option::Option<String> {
                                self.next_page_token.to_owned()
                            }
                        }
                        impl<T> crate::stream::IntoPageItems for Page<T> {
                            type Items = Vec<T>;
                            fn into_page_items(self) -> Self::Items {
                                self.items
                            }
                        }
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
                        crate::stream::page_item_stream::<_, Page<T>>(self)
                    }
                    #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                    #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                    #[doc = r" token is returned."]
                    #[doc = r""]
                    #[doc = r" Requests the field given by the [`FieldSelector`] implementation from the server."]
                    #[doc = r""]
                    #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                    #[doc = r" [`FieldSelector`]: ::google_field_selector::FieldSelector"]
                    pub fn stream<T>(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: crate::GetNextPageToken
                            + ::serde::de::DeserializeOwned
                            + ::google_field_selector::FieldSelector
                            + 'a,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: ::std::option::Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.stream_with_fields(fields)
                    }
                    #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                    #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                    #[doc = r" repeated until no page token is returned."]
                    #[doc = r""]
                    #[doc = r" Requests the default set of fields from the server."]
                    pub fn stream_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::ListNoteOccurrencesResponse, crate::Error>,
                    > + 'a {
                        self.stream_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                    #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                    #[doc = r" repeated until no page token is returned."]
                    #[doc = r""]
                    #[doc = r" Requests all fields from the server."]
                    pub fn stream_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::ListNoteOccurrencesResponse, crate::Error>,
                    > + 'a {
                        self.stream_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                    #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                    #[doc = r" token is returned."]
                    #[doc = r""]
                    #[doc = r" Only the given `fields` are requested from the server. If the list of fields is not"]
                    #[doc = r" empty, the `nextPageToken` field will be added to the list."]
                    #[doc = r""]
                    #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                    pub fn stream_with_fields<T, F>(
                        mut self,
                        fields: ::std::option::Option<F>,
                    ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                    where
                        T: crate::GetNextPageToken + ::serde::de::DeserializeOwned + 'a,
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
                        crate::stream::page_stream(self)
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
                    ) -> Result<crate::schemas::ListNoteOccurrencesResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListNoteOccurrencesResponse, crate::Error>
                    {
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
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://containeranalysis.googleapis.com/".to_owned();
                        output.push_str("v1alpha1/");
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
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                        let access_token = self
                            .auth
                            .access_token()
                            .await
                            .map_err(|err| crate::Error::OAuth2(err))?;
                        req = req.bearer_auth(access_token);
                        Ok(req)
                    }
                }
                #[async_trait::async_trait]
                impl<'a> crate::stream::StreamableMethod for ListRequestBuilder<'a> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    async fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: crate::GetNextPageToken + ::serde::de::DeserializeOwned,
                    {
                        self._execute().await
                    }
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
pub trait GetNextPageToken {
    /// Get the `nextPageToken` from a response if present.
    fn next_page_token(&self) -> ::std::option::Option<String>;
}

impl GetNextPageToken for ::serde_json::Map<String, ::serde_json::Value> {
    fn next_page_token(&self) -> ::std::option::Option<String> {
        self.get("nextPageToken")
            .and_then(|t| t.as_str())
            .map(|s| s.to_owned())
    }
}
/// Traits and functions to improve streamable (multiple page) API method handling.
pub mod stream {
    use super::GetNextPageToken;

    /// Extract the items embedded in a page like response.
    pub trait IntoPageItems {
        /// Type of the items list in the page.
        type Items: IntoIterator;

        /// Consume the response and return the embedded items.
        fn into_page_items(self) -> Self::Items;
    }

    /// Represent a API method which can be invoked multiple times to retrieve
    /// multiple pages of items.
    #[async_trait::async_trait]
    pub trait StreamableMethod {
        /// Update the current page token of the request.
        fn set_page_token(&mut self, value: String);

        /// Execute the request.
        async fn execute<T>(&mut self) -> Result<T, crate::Error>
        where
            T: GetNextPageToken + ::serde::de::DeserializeOwned;
    }

    /// Return a [`Stream`](::futures::Stream) over all pages of the given API
    /// method.
    pub fn page_stream<M, T>(method: M) -> impl ::futures::Stream<Item = Result<T, crate::Error>>
    where
        M: StreamableMethod,
        T: GetNextPageToken + ::serde::de::DeserializeOwned,
    {
        ::futures::stream::unfold((method, false), |(mut method, mut finished)| async move {
            if finished {
                return None;
            }
            let response = match method.execute::<T>().await {
                Ok(r) => r,
                Err(err) => return Some((Err(err), (method, false))),
            };
            if let Some(next_page_token) = response.next_page_token() {
                method.set_page_token(next_page_token);
            } else {
                finished = true;
            }

            Some((Ok(response), (method, finished)))
        })
    }

    /// Return a [`Stream`](::futures::Stream) over the items in all pages of
    /// the given API method.
    pub fn page_item_stream<M, T>(
        method: M,
    ) -> impl ::futures::Stream<Item = Result<<T::Items as IntoIterator>::Item, crate::Error>>
    where
        M: StreamableMethod,
        T: GetNextPageToken + ::serde::de::DeserializeOwned + IntoPageItems,
    {
        use ::futures::StreamExt;
        use ::futures::TryStreamExt;

        page_stream::<M, T>(method)
            .map_ok(|page| ::futures::stream::iter(page.into_page_items()).map(Ok))
            .try_flatten()
    }
}
