#![doc = "# Resources and Methods\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [assessments](resources/projects/assessments/struct.AssessmentsActions.html)\n        * [*annotate*](resources/projects/assessments/struct.AnnotateRequestBuilder.html), [*create*](resources/projects/assessments/struct.CreateRequestBuilder.html)\n      * [keys](resources/projects/keys/struct.KeysActions.html)\n        * [*create*](resources/projects/keys/struct.CreateRequestBuilder.html), [*delete*](resources/projects/keys/struct.DeleteRequestBuilder.html), [*get*](resources/projects/keys/struct.GetRequestBuilder.html), [*getMetrics*](resources/projects/keys/struct.GetMetricsRequestBuilder.html), [*list*](resources/projects/keys/struct.ListRequestBuilder.html), [*migrate*](resources/projects/keys/struct.MigrateRequestBuilder.html), [*patch*](resources/projects/keys/struct.PatchRequestBuilder.html)\n      * [relatedaccountgroupmemberships](resources/projects/relatedaccountgroupmemberships/struct.RelatedaccountgroupmembershipsActions.html)\n        * [*search*](resources/projects/relatedaccountgroupmemberships/struct.SearchRequestBuilder.html)\n      * [relatedaccountgroups](resources/projects/relatedaccountgroups/struct.RelatedaccountgroupsActions.html)\n        * [*list*](resources/projects/relatedaccountgroups/struct.ListRequestBuilder.html)\n        * [memberships](resources/projects/relatedaccountgroups/memberships/struct.MembershipsActions.html)\n          * [*list*](resources/projects/relatedaccountgroups/memberships/struct.ListRequestBuilder.html)\n"]
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
    pub struct GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessment { # [doc = "Labels for this request."] # [serde (rename = "labels" , default , skip_serializing_if = "std::option::Option::is_none")] pub labels : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems > > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessment
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessment
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems {
        #[doc = "Default unspecified type."]
        AccountDefenderLabelUnspecified,
        #[doc = "The request matches a known good profile for the user."]
        ProfileMatch,
        #[doc = "The account in the request has a high number of related accounts. It does not necessarily imply that the account is bad but could require investigating."]
        RelatedAccountsNumberHigh,
        #[doc = "The request matched a profile that previously had suspicious account creation behavior. This could mean this is a fake account."]
        SuspiciousAccountCreation,
        #[doc = "The request is potentially a suspicious login event and should be further verified either via multi-factor authentication or another system."]
        SuspiciousLoginActivity,
    }
    impl GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems :: AccountDefenderLabelUnspecified => "ACCOUNT_DEFENDER_LABEL_UNSPECIFIED" , GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems :: ProfileMatch => "PROFILE_MATCH" , GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems :: RelatedAccountsNumberHigh => "RELATED_ACCOUNTS_NUMBER_HIGH" , GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems :: SuspiciousAccountCreation => "SUSPICIOUS_ACCOUNT_CREATION" , GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems :: SuspiciousLoginActivity => "SUSPICIOUS_LOGIN_ACTIVITY" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems,
            (),
        > {
            Ok (match s { "ACCOUNT_DEFENDER_LABEL_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems :: AccountDefenderLabelUnspecified , "PROFILE_MATCH" => GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems :: ProfileMatch , "RELATED_ACCOUNTS_NUMBER_HIGH" => GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems :: RelatedAccountsNumberHigh , "SUSPICIOUS_ACCOUNT_CREATION" => GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems :: SuspiciousAccountCreation , "SUSPICIOUS_LOGIN_ACTIVITY" => GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems :: SuspiciousLoginActivity , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ACCOUNT_DEFENDER_LABEL_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems :: AccountDefenderLabelUnspecified , "PROFILE_MATCH" => GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems :: ProfileMatch , "RELATED_ACCOUNTS_NUMBER_HIGH" => GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems :: RelatedAccountsNumberHigh , "SUSPICIOUS_ACCOUNT_CREATION" => GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems :: SuspiciousAccountCreation , "SUSPICIOUS_LOGIN_ACTIVITY" => GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems :: SuspiciousLoginActivity , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessmentLabelsItems
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
    pub struct GoogleCloudRecaptchaenterpriseV1AndroidKeySettings {
        #[doc = "If set to true, allowed_package_names are not enforced."]
        #[serde(
            rename = "allowAllPackageNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_all_package_names: ::std::option::Option<bool>,
        #[doc = "Android package names of apps allowed to use the key. Example: 'com.companyname.appname'"]
        #[serde(
            rename = "allowedPackageNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_package_names: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1AndroidKeySettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1AndroidKeySettings {
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
    pub struct GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequest { # [doc = "Optional. The annotation that will be assigned to the Event. This field can be left empty to provide reasons that apply to an event without concluding whether the event is legitimate or fraudulent."] # [serde (rename = "annotation" , default , skip_serializing_if = "std::option::Option::is_none")] pub annotation : :: std :: option :: Option < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation > , # [doc = "Optional. Optional unique stable hashed user identifier to apply to the assessment. This is an alternative to setting the hashed_account_id in CreateAssessment, for example when the account identifier is not yet known in the initial request. It is recommended that the identifier is hashed using hmac-sha256 with stable secret."] # [serde (rename = "hashedAccountId" , default , skip_serializing_if = "std::option::Option::is_none")] pub hashed_account_id : :: std :: option :: Option < :: google_api_bytes :: Bytes > , # [doc = "Optional. Optional reasons for the annotation that will be assigned to the Event."] # [serde (rename = "reasons" , default , skip_serializing_if = "std::option::Option::is_none")] pub reasons : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems > > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation {
        #[doc = "Default unspecified type."]
        AnnotationUnspecified,
        #[doc = "Provides information that the event turned out to be fraudulent."]
        Fraudulent,
        #[doc = "Provides information that the event turned out to be legitimate."]
        Legitimate,
        #[doc = "Provides information that the event was related to a login event in which the user typed the correct password. Deprecated, prefer indicating CORRECT_PASSWORD through the reasons field instead."]
        PasswordCorrect,
        #[doc = "Provides information that the event was related to a login event in which the user typed the incorrect password. Deprecated, prefer indicating INCORRECT_PASSWORD through the reasons field instead."]
        PasswordIncorrect,
    }
    impl GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation :: AnnotationUnspecified => "ANNOTATION_UNSPECIFIED" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation :: Fraudulent => "FRAUDULENT" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation :: Legitimate => "LEGITIMATE" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation :: PasswordCorrect => "PASSWORD_CORRECT" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation :: PasswordIncorrect => "PASSWORD_INCORRECT" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation,
            (),
        > {
            Ok (match s { "ANNOTATION_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation :: AnnotationUnspecified , "FRAUDULENT" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation :: Fraudulent , "LEGITIMATE" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation :: Legitimate , "PASSWORD_CORRECT" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation :: PasswordCorrect , "PASSWORD_INCORRECT" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation :: PasswordIncorrect , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ANNOTATION_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation :: AnnotationUnspecified , "FRAUDULENT" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation :: Fraudulent , "LEGITIMATE" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation :: Legitimate , "PASSWORD_CORRECT" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation :: PasswordCorrect , "PASSWORD_INCORRECT" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation :: PasswordIncorrect , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems {
        #[doc = "Indicates a chargeback issued for the transaction with no other details. When possible, specify the type by using CHARGEBACK_FRAUD or CHARGEBACK_DISPUTE instead."]
        Chargeback,
        #[doc = "Indicates a chargeback related to the cardholder having provided their card details but allegedly not being satisfied with the purchase (for example, misrepresentation, attempted cancellation)."]
        ChargebackDispute,
        #[doc = "Indicates a chargeback related to an alleged unauthorized transaction from the cardholder's perspective (for example, the card number was stolen)."]
        ChargebackFraud,
        #[doc = "Indicates the user provided the correct password."]
        CorrectPassword,
        #[doc = "Indicates that the user failed a 2FA challenge."]
        FailedTwoFactor,
        #[doc = "Indicates the user provided an incorrect password."]
        IncorrectPassword,
        #[doc = "Indicates that the user was served a 2FA challenge. An old assessment with `ENUM_VALUES.INITIATED_TWO_FACTOR` reason that has not been overwritten with `PASSED_TWO_FACTOR` is treated as an abandoned 2FA flow. This is equivalent to `FAILED_TWO_FACTOR`."]
        InitiatedTwoFactor,
        #[doc = "Indicates that the user passed a 2FA challenge."]
        PassedTwoFactor,
        #[doc = "Indicates the transaction associated with the assessment is suspected of being fraudulent based on the payment method, billing details, shipping address or other transaction information."]
        PaymentHeuristics,
        #[doc = "Default unspecified reason."]
        ReasonUnspecified,
    }
    impl GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: Chargeback => "CHARGEBACK" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ChargebackDispute => "CHARGEBACK_DISPUTE" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ChargebackFraud => "CHARGEBACK_FRAUD" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: CorrectPassword => "CORRECT_PASSWORD" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: FailedTwoFactor => "FAILED_TWO_FACTOR" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: IncorrectPassword => "INCORRECT_PASSWORD" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: InitiatedTwoFactor => "INITIATED_TWO_FACTOR" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: PassedTwoFactor => "PASSED_TWO_FACTOR" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: PaymentHeuristics => "PAYMENT_HEURISTICS" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ReasonUnspecified => "REASON_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems,
            (),
        > {
            Ok (match s { "CHARGEBACK" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: Chargeback , "CHARGEBACK_DISPUTE" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ChargebackDispute , "CHARGEBACK_FRAUD" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ChargebackFraud , "CORRECT_PASSWORD" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: CorrectPassword , "FAILED_TWO_FACTOR" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: FailedTwoFactor , "INCORRECT_PASSWORD" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: IncorrectPassword , "INITIATED_TWO_FACTOR" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: InitiatedTwoFactor , "PASSED_TWO_FACTOR" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: PassedTwoFactor , "PAYMENT_HEURISTICS" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: PaymentHeuristics , "REASON_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ReasonUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CHARGEBACK" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: Chargeback , "CHARGEBACK_DISPUTE" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ChargebackDispute , "CHARGEBACK_FRAUD" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ChargebackFraud , "CORRECT_PASSWORD" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: CorrectPassword , "FAILED_TWO_FACTOR" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: FailedTwoFactor , "INCORRECT_PASSWORD" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: IncorrectPassword , "INITIATED_TWO_FACTOR" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: InitiatedTwoFactor , "PASSED_TWO_FACTOR" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: PassedTwoFactor , "PAYMENT_HEURISTICS" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: PaymentHeuristics , "REASON_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ReasonUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecaptchaenterpriseV1Assessment {
        #[doc = "Assessment returned by Account Defender when a hashed_account_id is provided."]
        #[serde(
            rename = "accountDefenderAssessment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_defender_assessment: ::std::option::Option<
            crate::schemas::GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessment,
        >,
        #[doc = "The event being assessed."]
        #[serde(
            rename = "event",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event: ::std::option::Option<crate::schemas::GoogleCloudRecaptchaenterpriseV1Event>,
        #[doc = "Output only. The resource name for the Assessment in the format \"projects/{project}/assessments/{assessment}\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. The risk analysis result for the event being assessed."]
        #[serde(
            rename = "riskAnalysis",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub risk_analysis:
            ::std::option::Option<crate::schemas::GoogleCloudRecaptchaenterpriseV1RiskAnalysis>,
        #[doc = "Output only. Properties of the provided event token."]
        #[serde(
            rename = "tokenProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub token_properties:
            ::std::option::Option<crate::schemas::GoogleCloudRecaptchaenterpriseV1TokenProperties>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1Assessment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1Assessment {
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
    pub struct GoogleCloudRecaptchaenterpriseV1ChallengeMetrics {
        #[doc = "Count of submitted challenge solutions that were incorrect or otherwise deemed suspicious such that a subsequent challenge was triggered."]
        #[serde(
            rename = "failedCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub failed_count: ::std::option::Option<i64>,
        #[doc = "Count of nocaptchas (successful verification without a challenge) issued."]
        #[serde(
            rename = "nocaptchaCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub nocaptcha_count: ::std::option::Option<i64>,
        #[doc = "Count of reCAPTCHA checkboxes or badges rendered. This is mostly equivalent to a count of pageloads for pages that include reCAPTCHA."]
        #[serde(
            rename = "pageloadCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub pageload_count: ::std::option::Option<i64>,
        #[doc = "Count of nocaptchas (successful verification without a challenge) plus submitted challenge solutions that were correct and resulted in verification."]
        #[serde(
            rename = "passedCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub passed_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1ChallengeMetrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1ChallengeMetrics {
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
    pub struct GoogleCloudRecaptchaenterpriseV1Event {
        #[doc = "Optional. The expected action for this type of event. This should be the same action provided at token generation time on client-side platforms already integrated with recaptcha enterprise."]
        #[serde(
            rename = "expectedAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expected_action: ::std::option::Option<String>,
        #[doc = "Optional. Optional unique stable hashed user identifier for the request. The identifier should ideally be hashed using sha256 with stable secret."]
        #[serde(
            rename = "hashedAccountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hashed_account_id: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. The site key that was used to invoke reCAPTCHA on your site and generate the token."]
        #[serde(
            rename = "siteKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_key: ::std::option::Option<String>,
        #[doc = "Optional. The user response token provided by the reCAPTCHA client-side integration on your site."]
        #[serde(
            rename = "token",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub token: ::std::option::Option<String>,
        #[doc = "Optional. The user agent present in the request from the user's device related to this event."]
        #[serde(
            rename = "userAgent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_agent: ::std::option::Option<String>,
        #[doc = "Optional. The IP address in the request from the user's device related to this event."]
        #[serde(
            rename = "userIpAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_ip_address: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1Event {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1Event {
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
    pub struct GoogleCloudRecaptchaenterpriseV1IOSKeySettings {
        #[doc = "If set to true, allowed_bundle_ids are not enforced."]
        #[serde(
            rename = "allowAllBundleIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_all_bundle_ids: ::std::option::Option<bool>,
        #[doc = "iOS bundle ids of apps allowed to use the key. Example: 'com.companyname.productname.appname'"]
        #[serde(
            rename = "allowedBundleIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allowed_bundle_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1IOSKeySettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1IOSKeySettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecaptchaenterpriseV1Key {
        #[doc = "Settings for keys that can be used by Android apps."]
        #[serde(
            rename = "androidSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_settings: ::std::option::Option<
            crate::schemas::GoogleCloudRecaptchaenterpriseV1AndroidKeySettings,
        >,
        #[doc = "The timestamp corresponding to the creation of this Key."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Human-readable display name of this key. Modifiable by user."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Settings for keys that can be used by iOS apps."]
        #[serde(
            rename = "iosSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_settings:
            ::std::option::Option<crate::schemas::GoogleCloudRecaptchaenterpriseV1IOSKeySettings>,
        #[doc = "See Creating and managing labels."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The resource name for the Key in the format \"projects/{project}/keys/{key}\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Options for user acceptance testing."]
        #[serde(
            rename = "testingOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub testing_options:
            ::std::option::Option<crate::schemas::GoogleCloudRecaptchaenterpriseV1TestingOptions>,
        #[doc = "Settings for WAF"]
        #[serde(
            rename = "wafSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub waf_settings:
            ::std::option::Option<crate::schemas::GoogleCloudRecaptchaenterpriseV1WafSettings>,
        #[doc = "Settings for keys that can be used by websites."]
        #[serde(
            rename = "webSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub web_settings:
            ::std::option::Option<crate::schemas::GoogleCloudRecaptchaenterpriseV1WebKeySettings>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1Key {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1Key {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecaptchaenterpriseV1ListKeysResponse {
        #[doc = "Key details."]
        #[serde(
            rename = "keys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keys: ::std::option::Option<Vec<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key>>,
        #[doc = "Token to retrieve the next page of results. It is set to empty if no keys remain in results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1ListKeysResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1ListKeysResponse {
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
    pub struct GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse {
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The memberships listed by the query."]
        #[serde(
            rename = "relatedAccountGroupMemberships",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_account_group_memberships: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse
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
    pub struct GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse {
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The groups of related accounts listed by the query."]
        #[serde(
            rename = "relatedAccountGroups",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_account_groups: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecaptchaenterpriseV1RelatedAccountGroup>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse
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
    pub struct GoogleCloudRecaptchaenterpriseV1Metrics {
        #[doc = "Metrics will be continuous and in order by dates, and in the granularity of day. Only challenge-based keys (CHECKBOX, INVISIBLE), will have challenge-based data."]
        #[serde(
            rename = "challengeMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub challenge_metrics: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecaptchaenterpriseV1ChallengeMetrics>,
        >,
        #[doc = "Output only. The name of the metrics, in the format \"projects/{project}/keys/{key}/metrics\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Metrics will be continuous and in order by dates, and in the granularity of day. All Key types should have score-based data."]
        #[serde(
            rename = "scoreMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub score_metrics: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecaptchaenterpriseV1ScoreMetrics>,
        >,
        #[doc = "Inclusive start time aligned to a day (UTC)."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1Metrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1Metrics {
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
    pub struct GoogleCloudRecaptchaenterpriseV1MigrateKeyRequest {}
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1MigrateKeyRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1MigrateKeyRequest {
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
    pub struct GoogleCloudRecaptchaenterpriseV1RelatedAccountGroup {
        #[doc = "Required. The resource name for the related account group in the format `projects/{project}/relatedaccountgroups/{related_account_group}`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1RelatedAccountGroup
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1RelatedAccountGroup {
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
    pub struct GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership {
        #[doc = "The unique stable hashed user identifier of the member. The identifier corresponds to a `hashed_account_id` provided in a previous CreateAssessment or AnnotateAssessment call."]
        #[serde(
            rename = "hashedAccountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hashed_account_id: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Required. The resource name for this membership in the format `projects/{project}/relatedaccountgroups/{relatedaccountgroup}/memberships/{membership}`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecaptchaenterpriseV1RiskAnalysis {
        #[doc = "Reasons contributing to the risk analysis verdict."]
        #[serde(
            rename = "reasons",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reasons: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems>,
        >,
        #[doc = "Legitimate event score from 0.0 to 1.0. (1.0 means very likely legitimate traffic while 0.0 means very likely non-legitimate traffic)."]
        #[serde(
            rename = "score",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub score: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1RiskAnalysis {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1RiskAnalysis {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems {
        #[doc = "Interactions matched the behavior of an automated agent."]
        Automation,
        #[doc = "Default unspecified type."]
        ClassificationReasonUnspecified,
        #[doc = "Too little traffic has been received from this site thus far to generate quality risk analysis."]
        LowConfidenceScore,
        #[doc = "Traffic volume from the event source is higher than normal."]
        TooMuchTraffic,
        #[doc = "The event originated from an illegitimate environment."]
        UnexpectedEnvironment,
        #[doc = "Interactions with the site were significantly different than expected patterns."]
        UnexpectedUsagePatterns,
    }
    impl GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: Automation => "AUTOMATION" , GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: ClassificationReasonUnspecified => "CLASSIFICATION_REASON_UNSPECIFIED" , GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: LowConfidenceScore => "LOW_CONFIDENCE_SCORE" , GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: TooMuchTraffic => "TOO_MUCH_TRAFFIC" , GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: UnexpectedEnvironment => "UNEXPECTED_ENVIRONMENT" , GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: UnexpectedUsagePatterns => "UNEXPECTED_USAGE_PATTERNS" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems, ()>
        {
            Ok (match s { "AUTOMATION" => GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: Automation , "CLASSIFICATION_REASON_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: ClassificationReasonUnspecified , "LOW_CONFIDENCE_SCORE" => GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: LowConfidenceScore , "TOO_MUCH_TRAFFIC" => GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: TooMuchTraffic , "UNEXPECTED_ENVIRONMENT" => GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: UnexpectedEnvironment , "UNEXPECTED_USAGE_PATTERNS" => GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: UnexpectedUsagePatterns , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "AUTOMATION" => GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: Automation , "CLASSIFICATION_REASON_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: ClassificationReasonUnspecified , "LOW_CONFIDENCE_SCORE" => GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: LowConfidenceScore , "TOO_MUCH_TRAFFIC" => GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: TooMuchTraffic , "UNEXPECTED_ENVIRONMENT" => GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: UnexpectedEnvironment , "UNEXPECTED_USAGE_PATTERNS" => GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems :: UnexpectedUsagePatterns , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1RiskAnalysisReasonsItems
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
    pub struct GoogleCloudRecaptchaenterpriseV1ScoreDistribution {
        #[doc = "Map key is score value multiplied by 100. The scores are discrete values between [0, 1]. The maximum number of buckets is on order of a few dozen, but typically much lower (ie. 10)."]
        #[serde(
            rename = "scoreBuckets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub score_buckets: ::std::option::Option<::std::collections::BTreeMap<String, i64>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1ScoreDistribution {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1ScoreDistribution {
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
    pub struct GoogleCloudRecaptchaenterpriseV1ScoreMetrics {
        #[doc = "Action-based metrics. The map key is the action name which specified by the site owners at time of the \"execute\" client-side call. Populated only for SCORE keys."]
        #[serde(
            rename = "actionMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action_metrics: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::GoogleCloudRecaptchaenterpriseV1ScoreDistribution,
            >,
        >,
        #[doc = "Aggregated score metrics for all traffic."]
        #[serde(
            rename = "overallMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overall_metrics: ::std::option::Option<
            crate::schemas::GoogleCloudRecaptchaenterpriseV1ScoreDistribution,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1ScoreMetrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1ScoreMetrics {
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
    pub struct GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsRequest {
        #[doc = "Optional. The unique stable hashed user identifier we should search connections to. The identifier should correspond to a `hashed_account_id` provided in a previous CreateAssessment or AnnotateAssessment call."]
        #[serde(
            rename = "hashedAccountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hashed_account_id: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. The maximum number of groups to return. The service may return fewer than this value. If unspecified, at most 50 groups will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000."]
        #[serde(
            rename = "pageSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_size: ::std::option::Option<i32>,
        #[doc = "Optional. A page token, received from a previous `SearchRelatedAccountGroupMemberships` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `SearchRelatedAccountGroupMemberships` must match the call that provided the page token."]
        #[serde(
            rename = "pageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsRequest
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
    pub struct GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsResponse {
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The queried memberships."]
        #[serde(
            rename = "relatedAccountGroupMemberships",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_account_group_memberships: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecaptchaenterpriseV1TestingOptions {
        #[doc = "For challenge-based keys only (CHECKBOX, INVISIBLE), all challenge requests for this site will return nocaptcha if NOCAPTCHA, or an unsolvable challenge if CHALLENGE."]
        #[serde(
            rename = "testingChallenge",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub testing_challenge: ::std::option::Option<
            crate::schemas::GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge,
        >,
        #[doc = "All assessments for this Key will return this score. Must be between 0 (likely not legitimate) and 1 (likely legitimate) inclusive."]
        #[serde(
            rename = "testingScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub testing_score: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1TestingOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1TestingOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge {
        #[doc = "Challenge requests for this key always return a nocaptcha, which does not require a solution."]
        Nocaptcha,
        #[doc = "Perform the normal risk analysis and return either nocaptcha or a challenge depending on risk and trust factors."]
        TestingChallengeUnspecified,
        #[doc = "Challenge requests for this key always return an unsolvable challenge."]
        UnsolvableChallenge,
    }
    impl GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge :: Nocaptcha => "NOCAPTCHA" , GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge :: TestingChallengeUnspecified => "TESTING_CHALLENGE_UNSPECIFIED" , GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge :: UnsolvableChallenge => "UNSOLVABLE_CHALLENGE" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge, ()>
        {
            Ok (match s { "NOCAPTCHA" => GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge :: Nocaptcha , "TESTING_CHALLENGE_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge :: TestingChallengeUnspecified , "UNSOLVABLE_CHALLENGE" => GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge :: UnsolvableChallenge , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "NOCAPTCHA" => GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge :: Nocaptcha , "TESTING_CHALLENGE_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge :: TestingChallengeUnspecified , "UNSOLVABLE_CHALLENGE" => GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge :: UnsolvableChallenge , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1TestingOptionsTestingChallenge
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
    pub struct GoogleCloudRecaptchaenterpriseV1TokenProperties {
        #[doc = "Action name provided at token generation."]
        #[serde(
            rename = "action",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action: ::std::option::Option<String>,
        #[doc = "The timestamp corresponding to the generation of the token."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The hostname of the page on which the token was generated."]
        #[serde(
            rename = "hostname",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hostname: ::std::option::Option<String>,
        #[doc = "Reason associated with the response when valid = false."]
        #[serde(
            rename = "invalidReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub invalid_reason: ::std::option::Option<
            crate::schemas::GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason,
        >,
        #[doc = "Whether the provided user response token is valid. When valid = false, the reason could be specified in invalid_reason or it could also be due to a user failing to solve a challenge or a sitekey mismatch (i.e the sitekey used to generate the token was different than the one specified in the assessment)."]
        #[serde(
            rename = "valid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub valid: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1TokenProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1TokenProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason {
        #[doc = "A retriable error (such as network failure) occurred on the browser. Could easily be simulated by an attacker."]
        BrowserError,
        #[doc = "The user verification had already been seen."]
        Dupe,
        #[doc = "The user verification token had expired."]
        Expired,
        #[doc = "Default unspecified type."]
        InvalidReasonUnspecified,
        #[doc = "The provided user verification token was malformed."]
        Malformed,
        #[doc = "The user verification token was not present."]
        Missing,
        #[doc = "If the failure reason was not accounted for."]
        UnknownInvalidReason,
    }
    impl GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: BrowserError => "BROWSER_ERROR" , GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: Dupe => "DUPE" , GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: Expired => "EXPIRED" , GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: InvalidReasonUnspecified => "INVALID_REASON_UNSPECIFIED" , GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: Malformed => "MALFORMED" , GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: Missing => "MISSING" , GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: UnknownInvalidReason => "UNKNOWN_INVALID_REASON" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason, ()>
        {
            Ok (match s { "BROWSER_ERROR" => GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: BrowserError , "DUPE" => GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: Dupe , "EXPIRED" => GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: Expired , "INVALID_REASON_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: InvalidReasonUnspecified , "MALFORMED" => GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: Malformed , "MISSING" => GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: Missing , "UNKNOWN_INVALID_REASON" => GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: UnknownInvalidReason , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "BROWSER_ERROR" => GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: BrowserError , "DUPE" => GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: Dupe , "EXPIRED" => GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: Expired , "INVALID_REASON_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: InvalidReasonUnspecified , "MALFORMED" => GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: Malformed , "MISSING" => GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: Missing , "UNKNOWN_INVALID_REASON" => GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason :: UnknownInvalidReason , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1TokenPropertiesInvalidReason
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
    pub struct GoogleCloudRecaptchaenterpriseV1WafSettings {
        #[doc = "Required. The WAF feature for which this key is enabled."]
        #[serde(
            rename = "wafFeature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub waf_feature: ::std::option::Option<
            crate::schemas::GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature,
        >,
        #[doc = "Required. The WAF service that uses this key."]
        #[serde(
            rename = "wafService",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub waf_service: ::std::option::Option<
            crate::schemas::GoogleCloudRecaptchaenterpriseV1WafSettingsWafService,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1WafSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1WafSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature {
        #[doc = "Use reCAPTCHA action-tokens to protect user actions."]
        ActionToken,
        #[doc = "Redirects suspicious traffic to reCAPTCHA."]
        ChallengePage,
        #[doc = "Use reCAPTCHA session-tokens to protect the whole user session on the site's domain."]
        SessionToken,
        #[doc = "Undefined feature."]
        WafFeatureUnspecified,
    }
    impl GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature::ActionToken => {
                    "ACTION_TOKEN"
                }
                GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature::ChallengePage => {
                    "CHALLENGE_PAGE"
                }
                GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature::SessionToken => {
                    "SESSION_TOKEN"
                }
                GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature::WafFeatureUnspecified => {
                    "WAF_FEATURE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature, ()>
        {
            Ok(match s {
                "ACTION_TOKEN" => {
                    GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature::ActionToken
                }
                "CHALLENGE_PAGE" => {
                    GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature::ChallengePage
                }
                "SESSION_TOKEN" => {
                    GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature::SessionToken
                }
                "WAF_FEATURE_UNSPECIFIED" => {
                    GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature::WafFeatureUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTION_TOKEN" => {
                    GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature::ActionToken
                }
                "CHALLENGE_PAGE" => {
                    GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature::ChallengePage
                }
                "SESSION_TOKEN" => {
                    GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature::SessionToken
                }
                "WAF_FEATURE_UNSPECIFIED" => {
                    GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature::WafFeatureUnspecified
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
        for GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1WafSettingsWafFeature
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecaptchaenterpriseV1WafSettingsWafService {
        #[doc = "Cloud Armor"]
        Ca,
        #[doc = "Undefined WAF"]
        WafServiceUnspecified,
    }
    impl GoogleCloudRecaptchaenterpriseV1WafSettingsWafService {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudRecaptchaenterpriseV1WafSettingsWafService::Ca => "CA",
                GoogleCloudRecaptchaenterpriseV1WafSettingsWafService::WafServiceUnspecified => {
                    "WAF_SERVICE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudRecaptchaenterpriseV1WafSettingsWafService {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudRecaptchaenterpriseV1WafSettingsWafService {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudRecaptchaenterpriseV1WafSettingsWafService, ()>
        {
            Ok(match s {
                "CA" => GoogleCloudRecaptchaenterpriseV1WafSettingsWafService::Ca,
                "WAF_SERVICE_UNSPECIFIED" => {
                    GoogleCloudRecaptchaenterpriseV1WafSettingsWafService::WafServiceUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudRecaptchaenterpriseV1WafSettingsWafService {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudRecaptchaenterpriseV1WafSettingsWafService {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudRecaptchaenterpriseV1WafSettingsWafService {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CA" => GoogleCloudRecaptchaenterpriseV1WafSettingsWafService::Ca,
                "WAF_SERVICE_UNSPECIFIED" => {
                    GoogleCloudRecaptchaenterpriseV1WafSettingsWafService::WafServiceUnspecified
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
        for GoogleCloudRecaptchaenterpriseV1WafSettingsWafService
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1WafSettingsWafService
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
    pub struct GoogleCloudRecaptchaenterpriseV1WebKeySettings { # [doc = "If set to true, it means allowed_domains will not be enforced."] # [serde (rename = "allowAllDomains" , default , skip_serializing_if = "std::option::Option::is_none")] pub allow_all_domains : :: std :: option :: Option < bool > , # [doc = "If set to true, the key can be used on AMP (Accelerated Mobile Pages) websites. This is supported only for the SCORE integration type."] # [serde (rename = "allowAmpTraffic" , default , skip_serializing_if = "std::option::Option::is_none")] pub allow_amp_traffic : :: std :: option :: Option < bool > , # [doc = "Domains or subdomains of websites allowed to use the key. All subdomains of an allowed domain are automatically allowed. A valid domain requires a host and must not include any path, port, query or fragment. Examples: 'example.com' or 'subdomain.example.com'"] # [serde (rename = "allowedDomains" , default , skip_serializing_if = "std::option::Option::is_none")] pub allowed_domains : :: std :: option :: Option < Vec < String > > , # [doc = "Settings for the frequency and difficulty at which this key triggers captcha challenges. This should only be specified for IntegrationTypes CHECKBOX and INVISIBLE."] # [serde (rename = "challengeSecurityPreference" , default , skip_serializing_if = "std::option::Option::is_none")] pub challenge_security_preference : :: std :: option :: Option < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference > , # [doc = "Required. Describes how this key is integrated with the website."] # [serde (rename = "integrationType" , default , skip_serializing_if = "std::option::Option::is_none")] pub integration_type : :: std :: option :: Option < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType > , }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1WebKeySettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1WebKeySettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference {
        #[doc = "Key tends to show balanced (in amount and difficulty) challenges."]
        Balance,
        #[doc = "Default type that indicates this enum hasn't been specified."]
        ChallengeSecurityPreferenceUnspecified,
        #[doc = "Key tends to show more and harder challenges."]
        Security,
        #[doc = "Key tends to show fewer and easier challenges."]
        Usability,
    }
    impl GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference :: Balance => "BALANCE" , GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference :: ChallengeSecurityPreferenceUnspecified => "CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED" , GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference :: Security => "SECURITY" , GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference :: Usability => "USABILITY" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference,
            (),
        > {
            Ok (match s { "BALANCE" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference :: Balance , "CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference :: ChallengeSecurityPreferenceUnspecified , "SECURITY" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference :: Security , "USABILITY" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference :: Usability , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "BALANCE" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference :: Balance , "CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference :: ChallengeSecurityPreferenceUnspecified , "SECURITY" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference :: Security , "USABILITY" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference :: Usability , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType {
        #[doc = "Displays the \"I'm not a robot\" checkbox and may show captcha challenges after it is checked."]
        Checkbox,
        #[doc = "Default type that indicates this enum hasn't been specified. This is not a valid IntegrationType, one of the other types must be specified instead."]
        IntegrationTypeUnspecified,
        #[doc = "Doesn't display the \"I'm not a robot\" checkbox, but may show captcha challenges after risk analysis."]
        Invisible,
        #[doc = "Only used to produce scores. It doesn't display the \"I'm not a robot\" checkbox and never shows captcha challenges."]
        Score,
    }
    impl GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType :: Checkbox => "CHECKBOX" , GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType :: IntegrationTypeUnspecified => "INTEGRATION_TYPE_UNSPECIFIED" , GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType :: Invisible => "INVISIBLE" , GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType :: Score => "SCORE" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType, ()>
        {
            Ok (match s { "CHECKBOX" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType :: Checkbox , "INTEGRATION_TYPE_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType :: IntegrationTypeUnspecified , "INVISIBLE" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType :: Invisible , "SCORE" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType :: Score , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CHECKBOX" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType :: Checkbox , "INTEGRATION_TYPE_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType :: IntegrationTypeUnspecified , "INVISIBLE" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType :: Invisible , "SCORE" => GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType :: Score , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleProtobufEmpty {}
    impl ::google_field_selector::FieldSelector for GoogleProtobufEmpty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleProtobufEmpty {
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
            #[doc = "Actions that can be performed on the assessments resource"]
            pub fn assessments(
                &self,
            ) -> crate::resources::projects::assessments::AssessmentsActions {
                crate::resources::projects::assessments::AssessmentsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the keys resource"]
            pub fn keys(&self) -> crate::resources::projects::keys::KeysActions {
                crate::resources::projects::keys::KeysActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the relatedaccountgroupmemberships resource"]            pub fn relatedaccountgroupmemberships (& self) -> crate :: resources :: projects :: relatedaccountgroupmemberships :: RelatedaccountgroupmembershipsActions{
                crate :: resources :: projects :: relatedaccountgroupmemberships :: RelatedaccountgroupmembershipsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
            }
            #[doc = "Actions that can be performed on the relatedaccountgroups resource"]
            pub fn relatedaccountgroups(
                &self,
            ) -> crate::resources::projects::relatedaccountgroups::RelatedaccountgroupsActions
            {
                crate::resources::projects::relatedaccountgroups::RelatedaccountgroupsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod assessments {
            pub mod params {}
            pub struct AssessmentsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> AssessmentsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Annotates a previously created Assessment to provide additional information on whether the event turned out to be authentic or fraudulent."]
                pub fn annotate(
                    &self,
                    request : crate :: schemas :: GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequest,
                    name: impl Into<String>,
                ) -> AnnotateRequestBuilder {
                    AnnotateRequestBuilder {
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
                #[doc = "Creates an Assessment of the likelihood an event is legitimate."]
                pub fn create(
                    &self,
                    request: crate::schemas::GoogleCloudRecaptchaenterpriseV1Assessment,
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
            }
            #[doc = "Created via [AssessmentsActions::annotate()](struct.AssessmentsActions.html#method.annotate)"]
            #[derive(Debug, Clone)]
            pub struct AnnotateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequest,
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
            impl<'a> AnnotateRequestBuilder<'a> {
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
                ) -> Result<
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentResponse,
                    crate::Error,
                > {
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
                    let mut output = "https://recaptchaenterprise.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":annotate");
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
            #[doc = "Created via [AssessmentsActions::create()](struct.AssessmentsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudRecaptchaenterpriseV1Assessment,
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
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Assessment, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Assessment, crate::Error>
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
                    let mut output = "https://recaptchaenterprise.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/assessments");
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
        pub mod keys {
            pub mod params {}
            pub struct KeysActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> KeysActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a new reCAPTCHA Enterprise key."]
                pub fn create(
                    &self,
                    request: crate::schemas::GoogleCloudRecaptchaenterpriseV1Key,
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
                #[doc = "Deletes the specified key."]
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
                #[doc = "Returns the specified key."]
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
                #[doc = "Get some aggregated metrics for a Key. This data can be used to build dashboards."]
                pub fn get_metrics(&self, name: impl Into<String>) -> GetMetricsRequestBuilder {
                    GetMetricsRequestBuilder {
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
                #[doc = "Returns the list of all keys that belong to a project."]
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
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Migrates an existing key from reCAPTCHA to reCAPTCHA Enterprise. Once a key is migrated, it can be used from either product. SiteVerify requests are billed as CreateAssessment calls. You must be authenticated as one of the current owners of the reCAPTCHA Site Key, and your user must have the reCAPTCHA Enterprise Admin IAM role in the destination project."]
                pub fn migrate(
                    &self,
                    request: crate::schemas::GoogleCloudRecaptchaenterpriseV1MigrateKeyRequest,
                    name: impl Into<String>,
                ) -> MigrateRequestBuilder {
                    MigrateRequestBuilder {
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
                #[doc = "Updates the specified key."]
                pub fn patch(
                    &self,
                    request: crate::schemas::GoogleCloudRecaptchaenterpriseV1Key,
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
            #[doc = "Created via [KeysActions::create()](struct.KeysActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudRecaptchaenterpriseV1Key,
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
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
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
                    let mut output = "https://recaptchaenterprise.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/keys");
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
            #[doc = "Created via [KeysActions::delete()](struct.KeysActions.html#method.delete)"]
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
                ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
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
                    let mut output = "https://recaptchaenterprise.googleapis.com/".to_owned();
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
            #[doc = "Created via [KeysActions::get()](struct.KeysActions.html#method.get)"]
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
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
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
                    let mut output = "https://recaptchaenterprise.googleapis.com/".to_owned();
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
            #[doc = "Created via [KeysActions::get_metrics()](struct.KeysActions.html#method.get_metrics)"]
            #[derive(Debug, Clone)]
            pub struct GetMetricsRequestBuilder<'a> {
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
            impl<'a> GetMetricsRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Metrics, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Metrics, crate::Error>
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
                    let mut output = "https://recaptchaenterprise.googleapis.com/".to_owned();
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
            #[doc = "Created via [KeysActions::list()](struct.KeysActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
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
                #[doc = "Optional. The maximum number of keys to return. Default is 10. Max limit is 1000."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. The next_page_token value returned from a previous. ListKeysRequest, if any."]
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
                pub fn iter_keys<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_keys_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_keys_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1Key,
                > {
                    self.iter_keys_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_keys_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1Key,
                > {
                    self.iter_keys_with_fields(Some("*"))
                }
                pub fn iter_keys_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "keys").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "keys")
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
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1ListKeysResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1ListKeysResponse,
                > {
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
                ) -> Result<
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1ListKeysResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1ListKeysResponse,
                    crate::Error,
                > {
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
                    let mut output = "https://recaptchaenterprise.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/keys");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
            #[doc = "Created via [KeysActions::migrate()](struct.KeysActions.html#method.migrate)"]
            #[derive(Debug, Clone)]
            pub struct MigrateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudRecaptchaenterpriseV1MigrateKeyRequest,
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
            impl<'a> MigrateRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
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
                    let mut output = "https://recaptchaenterprise.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":migrate");
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
            #[doc = "Created via [KeysActions::patch()](struct.KeysActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudRecaptchaenterpriseV1Key,
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
                #[doc = "Optional. The mask to control which fields of the key get updated. If the mask is not present, all fields will be updated."]
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
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
                {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
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
                    let mut output = "https://recaptchaenterprise.googleapis.com/".to_owned();
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
        }
        pub mod relatedaccountgroupmemberships {
            pub mod params {}
            pub struct RelatedaccountgroupmembershipsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> RelatedaccountgroupmembershipsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Search group memberships related to a given account."]
                pub fn search(
                    &self,
                    request : crate :: schemas :: GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsRequest,
                    project: impl Into<String>,
                ) -> SearchRequestBuilder {
                    SearchRequestBuilder {
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
                        project: project.into(),
                    }
                }
            }
            #[doc = "Created via [RelatedaccountgroupmembershipsActions::search()](struct.RelatedaccountgroupmembershipsActions.html#method.search)"]
            #[derive(Debug, Clone)]
            pub struct SearchRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: blocking :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsRequest , project : String , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
            impl<'a> SearchRequestBuilder<'a> {
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
                #[doc = r" the response resource."]                pub fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsResponse , crate :: Error >{
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]                pub fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsResponse , crate :: Error >{
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
                    let mut output = "https://recaptchaenterprise.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.project;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/relatedaccountgroupmemberships:search");
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
        pub mod relatedaccountgroups {
            pub mod params {}
            pub struct RelatedaccountgroupsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> RelatedaccountgroupsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "List groups of related accounts."]
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
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Actions that can be performed on the memberships resource"]
                pub fn memberships(
                    &self,
                ) -> crate::resources::projects::relatedaccountgroups::memberships::MembershipsActions
                {
                    crate :: resources :: projects :: relatedaccountgroups :: memberships :: MembershipsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                }
            }
            #[doc = "Created via [RelatedaccountgroupsActions::list()](struct.RelatedaccountgroupsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
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
                #[doc = "Optional. The maximum number of groups to return. The service may return fewer than this value. If unspecified, at most 50 groups will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. A page token, received from a previous `ListRelatedAccountGroups` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListRelatedAccountGroups` must match the call that provided the page token."]
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
                pub fn iter_related_account_groups<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_related_account_groups_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_related_account_groups_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1RelatedAccountGroup,
                > {
                    self.iter_related_account_groups_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_related_account_groups_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1RelatedAccountGroup,
                > {
                    self.iter_related_account_groups_with_fields(Some("*"))
                }
                pub fn iter_related_account_groups_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector =
                            concat!("nextPageToken,", "relatedAccountGroups").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "relatedAccountGroups")
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
                }                pub fn iter_with_default_fields (self) -> crate :: iter :: PageIter < Self , crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse >{
                    self.iter_with_fields(None::<&str>)
                }                pub fn iter_with_all_fields (self) -> crate :: iter :: PageIter < Self , crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse >{
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
                #[doc = r" the response resource."]                pub fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse , crate :: Error >{
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]                pub fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse , crate :: Error >{
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
                    let mut output = "https://recaptchaenterprise.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/relatedaccountgroups");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
            pub mod memberships {
                pub mod params {}
                pub struct MembershipsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> MembershipsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Get the memberships in a group of related accounts."]
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
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[doc = "Created via [MembershipsActions::list()](struct.MembershipsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
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
                    #[doc = "Optional. The maximum number of accounts to return. The service may return fewer than this value. If unspecified, at most 50 accounts will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Optional. A page token, received from a previous `ListRelatedAccountGroupMemberships` call. When paginating, all other parameters provided to `ListRelatedAccountGroupMemberships` must match the call that provided the page token."]
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
                    pub fn iter_related_account_group_memberships<T>(
                        self,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_related_account_group_memberships_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]                    pub fn iter_related_account_group_memberships_with_default_fields (self) -> crate :: iter :: PageItemIter < Self , crate :: schemas :: GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership >{
                        self.iter_related_account_group_memberships_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]                    pub fn iter_related_account_group_memberships_with_all_fields (self) -> crate :: iter :: PageItemIter < Self , crate :: schemas :: GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership >{
                        self.iter_related_account_group_memberships_with_fields(Some("*"))
                    }
                    pub fn iter_related_account_group_memberships_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector =
                                concat!("nextPageToken,", "relatedAccountGroupMemberships")
                                    .to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "relatedAccountGroupMemberships")
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
                    }                    pub fn iter_with_default_fields (self) -> crate :: iter :: PageIter < Self , crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse >{
                        self.iter_with_fields(None::<&str>)
                    }                    pub fn iter_with_all_fields (self) -> crate :: iter :: PageIter < Self , crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse >{
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
                    #[doc = r" the response resource."]                    pub fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse , crate :: Error >{
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]                    pub fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse , crate :: Error >{
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
                        let mut output = "https://recaptchaenterprise.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/memberships");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
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
