#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [projects](resources/projects/struct.ProjectsActions.html)\n  * [assessments](resources/projects/assessments/struct.AssessmentsActions.html)\n    * [*annotate*](resources/projects/assessments/struct.AnnotateRequestBuilder.html), [*create*](resources/projects/assessments/struct.CreateRequestBuilder.html)\n  * [keys](resources/projects/keys/struct.KeysActions.html)\n    * [*create*](resources/projects/keys/struct.CreateRequestBuilder.html), [*delete*](resources/projects/keys/struct.DeleteRequestBuilder.html), [*get*](resources/projects/keys/struct.GetRequestBuilder.html), [*getMetrics*](resources/projects/keys/struct.GetMetricsRequestBuilder.html), [*list*](resources/projects/keys/struct.ListRequestBuilder.html), [*migrate*](resources/projects/keys/struct.MigrateRequestBuilder.html), [*patch*](resources/projects/keys/struct.PatchRequestBuilder.html), [*retrieveLegacySecretKey*](resources/projects/keys/struct.RetrieveLegacySecretKeyRequestBuilder.html)\n  * [relatedaccountgroupmemberships](resources/projects/relatedaccountgroupmemberships/struct.RelatedaccountgroupmembershipsActions.html)\n    * [*search*](resources/projects/relatedaccountgroupmemberships/struct.SearchRequestBuilder.html)\n  * [relatedaccountgroups](resources/projects/relatedaccountgroups/struct.RelatedaccountgroupsActions.html)\n    * [*list*](resources/projects/relatedaccountgroups/struct.ListRequestBuilder.html)\n    * [memberships](resources/projects/relatedaccountgroups/memberships/struct.MembershipsActions.html)\n      * [*list*](resources/projects/relatedaccountgroups/memberships/struct.ListRequestBuilder.html)\n"]
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
        #[doc = "The account in the request has a high number of related accounts. It does not necessarily imply that the account is bad but can require further investigation."]
        RelatedAccountsNumberHigh,
        #[doc = "The request matched a profile that previously had suspicious account creation behavior. This can mean that this is a fake account."]
        SuspiciousAccountCreation,
        #[doc = "The request is potentially a suspicious login event and must be further verified either through multi-factor authentication or another system."]
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
    pub struct GoogleCloudRecaptchaenterpriseV1AccountVerificationInfo { # [doc = "Endpoints that can be used for identity verification."] # [serde (rename = "endpoints" , default , skip_serializing_if = "std::option::Option::is_none")] pub endpoints : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1EndpointVerificationInfo > > , # [doc = "Language code preference for the verification message, set as a IETF BCP 47 language code."] # [serde (rename = "languageCode" , default , skip_serializing_if = "std::option::Option::is_none")] pub language_code : :: std :: option :: Option < String > , # [doc = "Output only. Result of the latest account verification challenge."] # [serde (rename = "latestVerificationResult" , default , skip_serializing_if = "std::option::Option::is_none")] pub latest_verification_result : :: std :: option :: Option < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult > , # [doc = "Username of the account that is being verified. Deprecated. Customers should now provide the hashed account ID field in Event."] # [serde (rename = "username" , default , skip_serializing_if = "std::option::Option::is_none")] pub username : :: std :: option :: Option < String > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1AccountVerificationInfo
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1AccountVerificationInfo
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult {
        #[doc = "The verification flow could not be completed due to a critical internal error."]
        ErrorCriticalInternal,
        #[doc = "The client has exceeded their two factor request quota for this period of time."]
        ErrorCustomerQuotaExhausted,
        #[doc = "The recipient has already been sent too many verification codes in a short amount of time."]
        ErrorRecipientAbuseLimitExhausted,
        #[doc = "The recipient is not allowed for account verification. This can occur during integration but should not occur in production."]
        ErrorRecipientNotAllowed,
        #[doc = "The site is not properly onboarded to use the account verification feature."]
        ErrorSiteOnboardingIncomplete,
        #[doc = "The user failed the verification challenge."]
        ErrorUserNotVerified,
        #[doc = "The request parameters do not match with the token provided and cannot be processed."]
        ErrorVerdictMismatch,
        #[doc = "The request cannot be processed at the time because of an incident. This bypass can be restricted to a problematic destination email domain, a customer, or could affect the entire service."]
        ErrorVerificationBypassed,
        #[doc = "No information about the latest account verification."]
        ResultUnspecified,
        #[doc = "The user was successfully verified. This means the account verification challenge was successfully completed."]
        SuccessUserVerified,
    }
    impl GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorCriticalInternal => "ERROR_CRITICAL_INTERNAL" , GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorCustomerQuotaExhausted => "ERROR_CUSTOMER_QUOTA_EXHAUSTED" , GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorRecipientAbuseLimitExhausted => "ERROR_RECIPIENT_ABUSE_LIMIT_EXHAUSTED" , GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorRecipientNotAllowed => "ERROR_RECIPIENT_NOT_ALLOWED" , GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorSiteOnboardingIncomplete => "ERROR_SITE_ONBOARDING_INCOMPLETE" , GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorUserNotVerified => "ERROR_USER_NOT_VERIFIED" , GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorVerdictMismatch => "ERROR_VERDICT_MISMATCH" , GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorVerificationBypassed => "ERROR_VERIFICATION_BYPASSED" , GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ResultUnspecified => "RESULT_UNSPECIFIED" , GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: SuccessUserVerified => "SUCCESS_USER_VERIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult,
            (),
        > {
            Ok (match s { "ERROR_CRITICAL_INTERNAL" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorCriticalInternal , "ERROR_CUSTOMER_QUOTA_EXHAUSTED" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorCustomerQuotaExhausted , "ERROR_RECIPIENT_ABUSE_LIMIT_EXHAUSTED" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorRecipientAbuseLimitExhausted , "ERROR_RECIPIENT_NOT_ALLOWED" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorRecipientNotAllowed , "ERROR_SITE_ONBOARDING_INCOMPLETE" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorSiteOnboardingIncomplete , "ERROR_USER_NOT_VERIFIED" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorUserNotVerified , "ERROR_VERDICT_MISMATCH" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorVerdictMismatch , "ERROR_VERIFICATION_BYPASSED" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorVerificationBypassed , "RESULT_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ResultUnspecified , "SUCCESS_USER_VERIFIED" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: SuccessUserVerified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ERROR_CRITICAL_INTERNAL" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorCriticalInternal , "ERROR_CUSTOMER_QUOTA_EXHAUSTED" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorCustomerQuotaExhausted , "ERROR_RECIPIENT_ABUSE_LIMIT_EXHAUSTED" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorRecipientAbuseLimitExhausted , "ERROR_RECIPIENT_NOT_ALLOWED" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorRecipientNotAllowed , "ERROR_SITE_ONBOARDING_INCOMPLETE" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorSiteOnboardingIncomplete , "ERROR_USER_NOT_VERIFIED" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorUserNotVerified , "ERROR_VERDICT_MISMATCH" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorVerdictMismatch , "ERROR_VERIFICATION_BYPASSED" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ErrorVerificationBypassed , "RESULT_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: ResultUnspecified , "SUCCESS_USER_VERIFIED" => GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult :: SuccessUserVerified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1AccountVerificationInfoLatestVerificationResult
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
        #[doc = "Android package names of apps allowed to use the key. Example: ‘com.companyname.appname’"]
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequest { # [doc = "Optional. The annotation that will be assigned to the Event. This field can be left empty to provide reasons that apply to an event without concluding whether the event is legitimate or fraudulent."] # [serde (rename = "annotation" , default , skip_serializing_if = "std::option::Option::is_none")] pub annotation : :: std :: option :: Option < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestAnnotation > , # [doc = "Optional. Unique stable hashed user identifier to apply to the assessment. This is an alternative to setting the hashed_account_id in CreateAssessment, for example when the account identifier is not yet known in the initial request. It is recommended that the identifier is hashed using hmac-sha256 with stable secret."] # [serde (rename = "hashedAccountId" , default , skip_serializing_if = "std::option::Option::is_none")] pub hashed_account_id : :: std :: option :: Option < :: google_api_bytes :: Bytes > , # [doc = "Optional. Optional reasons for the annotation that will be assigned to the Event."] # [serde (rename = "reasons" , default , skip_serializing_if = "std::option::Option::is_none")] pub reasons : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems > > , # [doc = "Optional. If the Assessment is part of a Payment Transaction, provide details on Payment Lifecycle Events that occur in the Transaction."] # [serde (rename = "transactionEvent" , default , skip_serializing_if = "std::option::Option::is_none")] pub transaction_event : :: std :: option :: Option < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1TransactionEvent > , }
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
        #[doc = "Indicates that the transaction had a chargeback issued with no other details. When possible, specify the type by using CHARGEBACK_FRAUD or CHARGEBACK_DISPUTE instead."]
        Chargeback,
        #[doc = "Indicates that the transaction had a chargeback issued related to the cardholder having provided their card details but allegedly not being satisfied with the purchase (for example, misrepresentation, attempted cancellation)."]
        ChargebackDispute,
        #[doc = "Indicates that the transaction had a chargeback issued related to an alleged unauthorized transaction from the cardholder’s perspective (for example, the card number was stolen)."]
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
        #[doc = "Indicates that the completed payment transaction was refunded by the seller."]
        Refund,
        #[doc = "Indicates that the completed payment transaction was determined to be fraudulent by the seller, and was cancelled and refunded as a result."]
        RefundFraud,
        #[doc = "Indicates that the user sent unwanted and abusive messages to other users of the platform, such as spam, scams, phishing, or social engineering."]
        SocialSpam,
        #[doc = "Indicates that the payment transaction was accepted, and the user was charged."]
        TransactionAccepted,
        #[doc = "Indicates that the payment transaction was declined, for example due to invalid card details."]
        TransactionDeclined,
    }
    impl GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: Chargeback => "CHARGEBACK" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ChargebackDispute => "CHARGEBACK_DISPUTE" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ChargebackFraud => "CHARGEBACK_FRAUD" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: CorrectPassword => "CORRECT_PASSWORD" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: FailedTwoFactor => "FAILED_TWO_FACTOR" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: IncorrectPassword => "INCORRECT_PASSWORD" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: InitiatedTwoFactor => "INITIATED_TWO_FACTOR" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: PassedTwoFactor => "PASSED_TWO_FACTOR" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: PaymentHeuristics => "PAYMENT_HEURISTICS" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ReasonUnspecified => "REASON_UNSPECIFIED" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: Refund => "REFUND" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: RefundFraud => "REFUND_FRAUD" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: SocialSpam => "SOCIAL_SPAM" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: TransactionAccepted => "TRANSACTION_ACCEPTED" , GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: TransactionDeclined => "TRANSACTION_DECLINED" , }
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
            Ok (match s { "CHARGEBACK" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: Chargeback , "CHARGEBACK_DISPUTE" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ChargebackDispute , "CHARGEBACK_FRAUD" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ChargebackFraud , "CORRECT_PASSWORD" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: CorrectPassword , "FAILED_TWO_FACTOR" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: FailedTwoFactor , "INCORRECT_PASSWORD" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: IncorrectPassword , "INITIATED_TWO_FACTOR" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: InitiatedTwoFactor , "PASSED_TWO_FACTOR" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: PassedTwoFactor , "PAYMENT_HEURISTICS" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: PaymentHeuristics , "REASON_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ReasonUnspecified , "REFUND" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: Refund , "REFUND_FRAUD" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: RefundFraud , "SOCIAL_SPAM" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: SocialSpam , "TRANSACTION_ACCEPTED" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: TransactionAccepted , "TRANSACTION_DECLINED" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: TransactionDeclined , _ => return Err (()) , })
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
            Ok (match value { "CHARGEBACK" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: Chargeback , "CHARGEBACK_DISPUTE" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ChargebackDispute , "CHARGEBACK_FRAUD" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ChargebackFraud , "CORRECT_PASSWORD" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: CorrectPassword , "FAILED_TWO_FACTOR" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: FailedTwoFactor , "INCORRECT_PASSWORD" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: IncorrectPassword , "INITIATED_TWO_FACTOR" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: InitiatedTwoFactor , "PASSED_TWO_FACTOR" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: PassedTwoFactor , "PAYMENT_HEURISTICS" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: PaymentHeuristics , "REASON_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: ReasonUnspecified , "REFUND" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: Refund , "REFUND_FRAUD" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: RefundFraud , "SOCIAL_SPAM" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: SocialSpam , "TRANSACTION_ACCEPTED" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: TransactionAccepted , "TRANSACTION_DECLINED" => GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequestReasonsItems :: TransactionDeclined , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
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
        #[doc = "Assessment returned by account defender when a hashed_account_id is provided."]
        #[serde(
            rename = "accountDefenderAssessment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_defender_assessment: ::std::option::Option<
            crate::schemas::GoogleCloudRecaptchaenterpriseV1AccountDefenderAssessment,
        >,
        #[doc = "Account verification information for identity verification. The assessment event must include a token and site key to use this feature."]
        #[serde(
            rename = "accountVerification",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_verification: ::std::option::Option<
            crate::schemas::GoogleCloudRecaptchaenterpriseV1AccountVerificationInfo,
        >,
        #[doc = "The event being assessed."]
        #[serde(
            rename = "event",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event: ::std::option::Option<crate::schemas::GoogleCloudRecaptchaenterpriseV1Event>,
        #[doc = "Output only. The resource name for the Assessment in the format “projects/{project}/assessments/{assessment}”."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The private password leak verification field contains the parameters that are used to to check for leaks privately without sharing user credentials."]
        #[serde(
            rename = "privatePasswordLeakVerification",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub private_password_leak_verification: ::std::option::Option<
            crate::schemas::GoogleCloudRecaptchaenterpriseV1PrivatePasswordLeakVerification,
        >,
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
    pub struct GoogleCloudRecaptchaenterpriseV1EndpointVerificationInfo {
        #[doc = "Email address for which to trigger a verification request."]
        #[serde(
            rename = "emailAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email_address: ::std::option::Option<String>,
        #[doc = "Output only. Timestamp of the last successful verification for the endpoint, if any."]
        #[serde(
            rename = "lastVerificationTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_verification_time: ::std::option::Option<String>,
        #[doc = "Phone number for which to trigger a verification request. Should be given in E.164 format."]
        #[serde(
            rename = "phoneNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone_number: ::std::option::Option<String>,
        #[doc = "Output only. Token to provide to the client to trigger endpoint verification. It must be used within 15 minutes."]
        #[serde(
            rename = "requestToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1EndpointVerificationInfo
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1EndpointVerificationInfo
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
    pub struct GoogleCloudRecaptchaenterpriseV1Event {
        #[doc = "Optional. The expected action for this type of event. This should be the same action provided at token generation time on client-side platforms already integrated with recaptcha enterprise."]
        #[serde(
            rename = "expectedAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expected_action: ::std::option::Option<String>,
        #[doc = "Optional. Unique stable hashed user identifier for the request. The identifier must be hashed using hmac-sha256 with stable secret."]
        #[serde(
            rename = "hashedAccountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hashed_account_id: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. The site key that was used to invoke reCAPTCHA Enterprise on your site and generate the token."]
        #[serde(
            rename = "siteKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_key: ::std::option::Option<String>,
        #[doc = "Optional. The user response token provided by the reCAPTCHA Enterprise client-side integration on your site."]
        #[serde(
            rename = "token",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub token: ::std::option::Option<String>,
        #[doc = "Optional. The user agent present in the request from the user’s device related to this event."]
        #[serde(
            rename = "userAgent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_agent: ::std::option::Option<String>,
        #[doc = "Optional. The IP address in the request from the user’s device related to this event."]
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
        #[doc = "iOS bundle ids of apps allowed to use the key. Example: ‘com.companyname.productname.appname’"]
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
        #[doc = "The resource name for the Key in the format “projects/{project}/keys/{key}”."]
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
    impl crate::GetNextPageToken<String> for GoogleCloudRecaptchaenterpriseV1ListKeysResponse {
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
    impl crate::GetNextPageToken<String>
        for GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse
    {
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
    impl crate::GetNextPageToken<String>
        for GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse
    {
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
        #[doc = "Output only. The name of the metrics, in the format “projects/{project}/keys/{key}/metrics”."]
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
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecaptchaenterpriseV1MigrateKeyRequest {
        #[doc = "Optional. If true, skips the billing check. A reCAPTCHA Enterprise key or migrated key behaves differently than a reCAPTCHA (non-Enterprise version) key when you reach a quota limit (see https://cloud.google.com/recaptcha-enterprise/quotas#quota_limit). To avoid any disruption of your usage, we check that a billing account is present. If your usage of reCAPTCHA is under the free quota, you can safely skip the billing check and proceed with the migration. See https://cloud.google.com/recaptcha-enterprise/docs/billing-information."]
        #[serde(
            rename = "skipBillingCheck",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skip_billing_check: ::std::option::Option<bool>,
    }
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
    pub struct GoogleCloudRecaptchaenterpriseV1PrivatePasswordLeakVerification {
        #[doc = "Output only. List of prefixes of the encrypted potential password leaks that matched the given parameters. They must be compared with the client-side decryption prefix of `reencrypted_user_credentials_hash`"]
        #[serde(
            rename = "encryptedLeakMatchPrefixes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encrypted_leak_match_prefixes: ::std::option::Option<Vec<::google_api_bytes::Bytes>>,
        #[doc = "Optional. Encrypted Scrypt hash of the canonicalized username+password. It is re-encrypted by the server and returned through `reencrypted_user_credentials_hash`."]
        #[serde(
            rename = "encryptedUserCredentialsHash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encrypted_user_credentials_hash: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. Exactly 26-bit prefix of the SHA-256 hash of the canonicalized username. It is used to look up password leaks associated with that hash prefix."]
        #[serde(
            rename = "lookupHashPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lookup_hash_prefix: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Output only. Corresponds to the re-encryption of the `encrypted_user_credentials_hash` field. It is used to match potential password leaks within `encrypted_leak_match_prefixes`."]
        #[serde(
            rename = "reencryptedUserCredentialsHash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reencrypted_user_credentials_hash: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1PrivatePasswordLeakVerification
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1PrivatePasswordLeakVerification
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
        #[doc = "The unique stable hashed user identifier of the member. The identifier corresponds to a `hashed_account_id` provided in a previous `CreateAssessment` or `AnnotateAssessment` call."]
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
    pub struct GoogleCloudRecaptchaenterpriseV1RetrieveLegacySecretKeyResponse {
        #[doc = "The secret key (also known as shared secret) authorizes communication between your application backend and the reCAPTCHA Enterprise server to create an assessment. The secret key needs to be kept safe for security purposes."]
        #[serde(
            rename = "legacySecretKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub legacy_secret_key: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1RetrieveLegacySecretKeyResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1RetrieveLegacySecretKeyResponse
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
        #[doc = "Map key is score value multiplied by 100. The scores are discrete values between \\[0, 1\\]. The maximum number of buckets is on order of a few dozen, but typically much lower (ie. 10)."]
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
        #[doc = "Action-based metrics. The map key is the action name which specified by the site owners at time of the “execute” client-side call."]
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
        #[doc = "Optional. The unique stable hashed user identifier we should search connections to. The identifier should correspond to a `hashed_account_id` provided in a previous `CreateAssessment` or `AnnotateAssessment` call."]
        #[serde(
            rename = "hashedAccountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hashed_account_id: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Optional. The maximum number of groups to return. The service might return fewer than this value. If unspecified, at most 50 groups are returned. The maximum value is 1000; values above 1000 are coerced to 1000."]
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
    impl crate::GetNextPageToken<String>
        for GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsResponse
    {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
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
        #[doc = "The name of the Android package with which the token was generated (Android keys only)."]
        #[serde(
            rename = "androidPackageName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_package_name: ::std::option::Option<String>,
        #[doc = "The timestamp corresponding to the generation of the token."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The hostname of the page on which the token was generated (Web keys only)."]
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
        #[doc = "The ID of the iOS bundle with which the token was generated (iOS keys only)."]
        #[serde(
            rename = "iosBundleId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_bundle_id: ::std::option::Option<String>,
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecaptchaenterpriseV1TransactionEvent {
        #[doc = "Optional. Timestamp when this transaction event occurred; otherwise assumed to be the time of the API call."]
        #[serde(
            rename = "eventTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_time: ::std::option::Option<String>,
        #[doc = "Optional. The type of this transaction event."]
        #[serde(
            rename = "eventType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_type: ::std::option::Option<
            crate::schemas::GoogleCloudRecaptchaenterpriseV1TransactionEventEventType,
        >,
        #[doc = "Optional. The reason or standardized code which corresponds with this transaction event, if one exists. E.g. a CHARGEBACK Event with code 4553."]
        #[serde(
            rename = "reason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reason: ::std::option::Option<String>,
        #[doc = "Optional. The value that corresponds with this transaction event, if one exists. E.g. A refund event where $5.00 was refunded. Currency is obtained from the original transaction data."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecaptchaenterpriseV1TransactionEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecaptchaenterpriseV1TransactionEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecaptchaenterpriseV1TransactionEventEventType {
        #[doc = "Indicates that the authorization attempt with the card issuer succeeded."]
        Authorization,
        #[doc = "Indicates that the authorization attempt with the card issuer failed. The accompanying reasons can include Visa’s ‘54’ indicating that the card is expired or ‘82’ indicating that the CVV is incorrect."]
        AuthorizationDecline,
        #[doc = "Indicates that the transaction has been canceled. Specify the reason for the cancellation. For example, ‘INSUFFICIENT_INVENTORY’."]
        Cancel,
        #[doc = "Indicates that the merchant is informed by the payment network that the transaction has entered the chargeback process. Reason code examples include Discover’s ‘4553’ and ‘6041’. For partial chargebacks, we recommend that you include an amount in the `value` field."]
        Chargeback,
        #[doc = "Indicates that the merchant has received a chargeback alert for the transaction. The process of resolving the dispute without involving the payment network is started."]
        ChargebackAlert,
        #[doc = "Indicates that the merchant has received a chargeback inquiry for the transaction, requesting additional information before a chargeback is officially issued and a formal chargeback notification is sent."]
        ChargebackInquiry,
        #[doc = "Indicates that the transaction has entered the chargeback process, and that the merchant has chosen to enter representment. Reason examples include Discover’s ‘4553’ and ‘6041’. For partial chargebacks, we recommend that you include an amount in the `value` field."]
        ChargebackRepresentment,
        #[doc = "Indicates that the transaction has had a chargeback which was illegitimate and was reversed as a result. For partial chargebacks, we recommend that you include an amount in the `value` field."]
        ChargebackReverse,
        #[doc = "Indicates that a fraud notification is issued for the transaction, sent by the payment instrument’s issuing bank because the transaction appears to be fraudulent. We recommend including TC40 or SAFE data in the `reason` field for this event type. For partial chargebacks, we recommend that you include an amount in the `value` field."]
        FraudNotification,
        #[doc = "Indicates that the transaction is being evaluated by a human, due to suspicion or risk."]
        ManualReview,
        #[doc = "Indicates that the transaction is approved by the merchant’s risk engine. The accompanying reasons can include ‘INHOUSE’, ‘ACCERTIFY’, or ‘RECAPTCHA’."]
        MerchantApprove,
        #[doc = "Indicates that the transaction is denied and concluded due to risks detected by the merchant’s risk engine. The accompanying reasons can include ‘INHOUSE’, ‘ACCERTIFY’, ‘MANUAL_REVIEW’, or ‘RECAPTCHA’."]
        MerchantDeny,
        #[doc = "Indicates that the transaction is completed because the funds were settled."]
        PaymentCapture,
        #[doc = "Indicates that the transaction could not be completed because the funds were not settled."]
        PaymentCaptureDecline,
        #[doc = "Indicates that the completed transaction was refunded by the merchant. For partial refunds, we recommend that you include an amount in the `value` field. Reason example: ‘TAX_EXEMPT’ (partial refund of exempt tax)"]
        Refund,
        #[doc = "Indicates that the merchant has received a refund request for this transaction, but that they have declined it. For partial refunds, we recommend that you include an amount in the `value` field. Reason example: ‘TAX_EXEMPT’ (partial refund of exempt tax)"]
        RefundDecline,
        #[doc = "Indicates that the merchant has received a refund for a completed transaction. For partial refunds, we recommend that you include an amount in the `value` field. Reason example: ‘TAX_EXEMPT’ (partial refund of exempt tax)"]
        RefundRequest,
        #[doc = "Indicates that the completed transaction was refunded by the merchant, and that this refund was reversed. For partial refunds, we recommend that you include an amount in the `value` field."]
        RefundReverse,
        #[doc = "Default, unspecified event type."]
        TransactionEventTypeUnspecified,
    }
    impl GoogleCloudRecaptchaenterpriseV1TransactionEventEventType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: Authorization => "AUTHORIZATION" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: AuthorizationDecline => "AUTHORIZATION_DECLINE" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: Cancel => "CANCEL" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: Chargeback => "CHARGEBACK" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: ChargebackAlert => "CHARGEBACK_ALERT" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: ChargebackInquiry => "CHARGEBACK_INQUIRY" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: ChargebackRepresentment => "CHARGEBACK_REPRESENTMENT" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: ChargebackReverse => "CHARGEBACK_REVERSE" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: FraudNotification => "FRAUD_NOTIFICATION" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: ManualReview => "MANUAL_REVIEW" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: MerchantApprove => "MERCHANT_APPROVE" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: MerchantDeny => "MERCHANT_DENY" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: PaymentCapture => "PAYMENT_CAPTURE" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: PaymentCaptureDecline => "PAYMENT_CAPTURE_DECLINE" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: Refund => "REFUND" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: RefundDecline => "REFUND_DECLINE" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: RefundRequest => "REFUND_REQUEST" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: RefundReverse => "REFUND_REVERSE" , GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: TransactionEventTypeUnspecified => "TRANSACTION_EVENT_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudRecaptchaenterpriseV1TransactionEventEventType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudRecaptchaenterpriseV1TransactionEventEventType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudRecaptchaenterpriseV1TransactionEventEventType, ()>
        {
            Ok (match s { "AUTHORIZATION" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: Authorization , "AUTHORIZATION_DECLINE" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: AuthorizationDecline , "CANCEL" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: Cancel , "CHARGEBACK" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: Chargeback , "CHARGEBACK_ALERT" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: ChargebackAlert , "CHARGEBACK_INQUIRY" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: ChargebackInquiry , "CHARGEBACK_REPRESENTMENT" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: ChargebackRepresentment , "CHARGEBACK_REVERSE" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: ChargebackReverse , "FRAUD_NOTIFICATION" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: FraudNotification , "MANUAL_REVIEW" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: ManualReview , "MERCHANT_APPROVE" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: MerchantApprove , "MERCHANT_DENY" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: MerchantDeny , "PAYMENT_CAPTURE" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: PaymentCapture , "PAYMENT_CAPTURE_DECLINE" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: PaymentCaptureDecline , "REFUND" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: Refund , "REFUND_DECLINE" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: RefundDecline , "REFUND_REQUEST" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: RefundRequest , "REFUND_REVERSE" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: RefundReverse , "TRANSACTION_EVENT_TYPE_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: TransactionEventTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudRecaptchaenterpriseV1TransactionEventEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudRecaptchaenterpriseV1TransactionEventEventType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudRecaptchaenterpriseV1TransactionEventEventType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "AUTHORIZATION" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: Authorization , "AUTHORIZATION_DECLINE" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: AuthorizationDecline , "CANCEL" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: Cancel , "CHARGEBACK" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: Chargeback , "CHARGEBACK_ALERT" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: ChargebackAlert , "CHARGEBACK_INQUIRY" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: ChargebackInquiry , "CHARGEBACK_REPRESENTMENT" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: ChargebackRepresentment , "CHARGEBACK_REVERSE" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: ChargebackReverse , "FRAUD_NOTIFICATION" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: FraudNotification , "MANUAL_REVIEW" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: ManualReview , "MERCHANT_APPROVE" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: MerchantApprove , "MERCHANT_DENY" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: MerchantDeny , "PAYMENT_CAPTURE" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: PaymentCapture , "PAYMENT_CAPTURE_DECLINE" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: PaymentCaptureDecline , "REFUND" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: Refund , "REFUND_DECLINE" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: RefundDecline , "REFUND_REQUEST" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: RefundRequest , "REFUND_REVERSE" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: RefundReverse , "TRANSACTION_EVENT_TYPE_UNSPECIFIED" => GoogleCloudRecaptchaenterpriseV1TransactionEventEventType :: TransactionEventTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecaptchaenterpriseV1TransactionEventEventType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecaptchaenterpriseV1TransactionEventEventType
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
        #[doc = "Use reCAPTCHA session-tokens to protect the whole user session on the site’s domain."]
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
    pub struct GoogleCloudRecaptchaenterpriseV1WebKeySettings { # [doc = "If set to true, it means allowed_domains will not be enforced."] # [serde (rename = "allowAllDomains" , default , skip_serializing_if = "std::option::Option::is_none")] pub allow_all_domains : :: std :: option :: Option < bool > , # [doc = "If set to true, the key can be used on AMP (Accelerated Mobile Pages) websites. This is supported only for the SCORE integration type."] # [serde (rename = "allowAmpTraffic" , default , skip_serializing_if = "std::option::Option::is_none")] pub allow_amp_traffic : :: std :: option :: Option < bool > , # [doc = "Domains or subdomains of websites allowed to use the key. All subdomains of an allowed domain are automatically allowed. A valid domain requires a host and must not include any path, port, query or fragment. Examples: ‘example.com’ or ‘subdomain.example.com’"] # [serde (rename = "allowedDomains" , default , skip_serializing_if = "std::option::Option::is_none")] pub allowed_domains : :: std :: option :: Option < Vec < String > > , # [doc = "Settings for the frequency and difficulty at which this key triggers captcha challenges. This should only be specified for IntegrationTypes CHECKBOX and INVISIBLE."] # [serde (rename = "challengeSecurityPreference" , default , skip_serializing_if = "std::option::Option::is_none")] pub challenge_security_preference : :: std :: option :: Option < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1WebKeySettingsChallengeSecurityPreference > , # [doc = "Required. Describes how this key is integrated with the website."] # [serde (rename = "integrationType" , default , skip_serializing_if = "std::option::Option::is_none")] pub integration_type : :: std :: option :: Option < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1WebKeySettingsIntegrationType > , }
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
        #[doc = "Default type that indicates this enum hasn’t been specified."]
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
        #[doc = "Displays the “I’m not a robot” checkbox and may show captcha challenges after it is checked."]
        Checkbox,
        #[doc = "Default type that indicates this enum hasn’t been specified. This is not a valid IntegrationType, one of the other types must be specified instead."]
        IntegrationTypeUnspecified,
        #[doc = "Doesn’t display the “I’m not a robot” checkbox, but may show captcha challenges after risk analysis."]
        Invisible,
        #[doc = "Only used to produce scores. It doesn’t display the “I’m not a robot” checkbox and never shows captcha challenges."]
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
                pub(crate) reqwest: &'a reqwest::Client,
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
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentRequest,
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
                ) -> Result<
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1AnnotateAssessmentResponse,
                    crate::Error,
                > {
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
            #[doc = "Created via [AssessmentsActions::create()](struct.AssessmentsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudRecaptchaenterpriseV1Assessment,
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
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Assessment, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Assessment, crate::Error>
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
        pub mod keys {
            pub mod params {}
            pub struct KeysActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
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
                #[doc = "Returns the secret key related to the specified public key. You must use the legacy secret key only in a 3rd party integration with legacy reCAPTCHA."]
                pub fn retrieve_legacy_secret_key(
                    &self,
                    key: impl Into<String>,
                ) -> RetrieveLegacySecretKeyRequestBuilder {
                    RetrieveLegacySecretKeyRequestBuilder {
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
                        key: key.into(),
                    }
                }
            }
            #[doc = "Created via [KeysActions::create()](struct.KeysActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudRecaptchaenterpriseV1Key,
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
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
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
            #[doc = "Created via [KeysActions::delete()](struct.KeysActions.html#method.delete)"]
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
                ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error> {
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
            #[doc = "Created via [KeysActions::get()](struct.KeysActions.html#method.get)"]
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
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
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
            #[doc = "Created via [KeysActions::get_metrics()](struct.KeysActions.html#method.get_metrics)"]
            #[derive(Debug, Clone)]
            pub struct GetMetricsRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Metrics, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Metrics, crate::Error>
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
            #[doc = "Created via [KeysActions::list()](struct.KeysActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
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
                #[doc = "\nExecute the request and yield each item in the `keys` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_keys<T>(
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
                    self.stream_keys_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `keys` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_keys_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleCloudRecaptchaenterpriseV1Key,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_keys_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `keys` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_keys_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleCloudRecaptchaenterpriseV1Key,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_keys_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `keys` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_keys_with_fields<T, F>(
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
                        #[serde(rename = "keys")]
                        pub items: Vec<T>,
                    }
                    impl<T> crate::GetNextPageToken<String> for Page<T> {
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
                        let mut selector = concat!("nextPageToken,", "keys").to_owned();
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
                    T: crate::GetNextPageToken<String>
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
                    Item = Result<
                        crate::schemas::GoogleCloudRecaptchaenterpriseV1ListKeysResponse,
                        crate::Error,
                    >,
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
                    Item = Result<
                        crate::schemas::GoogleCloudRecaptchaenterpriseV1ListKeysResponse,
                        crate::Error,
                    >,
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
                    T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned + 'a,
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
                ) -> Result<
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1ListKeysResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1ListKeysResponse,
                    crate::Error,
                > {
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
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                type PageToken = String;
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                async fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned,
                {
                    self._execute().await
                }
            }
            #[doc = "Created via [KeysActions::migrate()](struct.KeysActions.html#method.migrate)"]
            #[derive(Debug, Clone)]
            pub struct MigrateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudRecaptchaenterpriseV1MigrateKeyRequest,
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
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
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
            #[doc = "Created via [KeysActions::patch()](struct.KeysActions.html#method.patch)"]
            #[derive(Debug, Clone)]
            pub struct PatchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudRecaptchaenterpriseV1Key,
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
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GoogleCloudRecaptchaenterpriseV1Key, crate::Error>
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
            #[doc = "Created via [KeysActions::retrieve_legacy_secret_key()](struct.KeysActions.html#method.retrieve_legacy_secret_key)"]
            #[derive(Debug, Clone)]
            pub struct RetrieveLegacySecretKeyRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                key: String,
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
            impl<'a> RetrieveLegacySecretKeyRequestBuilder<'a> {
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
                ) -> Result<
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1RetrieveLegacySecretKeyResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudRecaptchaenterpriseV1RetrieveLegacySecretKeyResponse,
                    crate::Error,
                > {
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
                    let mut output = "https://recaptchaenterprise.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.key;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":retrieveLegacySecretKey");
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
        }
        pub mod relatedaccountgroupmemberships {
            pub mod params {}
            pub struct RelatedaccountgroupmembershipsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
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
            pub struct SearchRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsRequest , project : String , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
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
                #[doc = r" the response resource."]                pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsResponse , crate :: Error >{
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]                pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1SearchRelatedAccountGroupMembershipsResponse , crate :: Error >{
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
        pub mod relatedaccountgroups {
            pub mod params {}
            pub struct RelatedaccountgroupsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
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
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
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
                #[doc = "Optional. The maximum number of groups to return. The service might return fewer than this value. If unspecified, at most 50 groups are returned. The maximum value is 1000; values above 1000 are coerced to 1000."]
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
                #[doc = "\nExecute the request and yield each item in the `relatedAccountGroups` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_related_account_groups<T>(
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
                    self.stream_related_account_groups_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `relatedAccountGroups` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_related_account_groups_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleCloudRecaptchaenterpriseV1RelatedAccountGroup,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_related_account_groups_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `relatedAccountGroups` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_related_account_groups_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleCloudRecaptchaenterpriseV1RelatedAccountGroup,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_related_account_groups_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `relatedAccountGroups` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_related_account_groups_with_fields<T, F>(
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
                        #[serde(rename = "relatedAccountGroups")]
                        pub items: Vec<T>,
                    }
                    impl<T> crate::GetNextPageToken<String> for Page<T> {
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
                    T: crate::GetNextPageToken<String>
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
                #[doc = r" Requests the default set of fields from the server."]                pub fn stream_with_default_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse , crate :: Error >> + 'a{
                    self.stream_with_fields(None::<&str>)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests all fields from the server."]                pub fn stream_with_all_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse , crate :: Error >> + 'a{
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
                    T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned + 'a,
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
                #[doc = r" the response resource."]                pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse , crate :: Error >{
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]                pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupsResponse , crate :: Error >{
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
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                type PageToken = String;
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                async fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned,
                {
                    self._execute().await
                }
            }
            pub mod memberships {
                pub mod params {}
                pub struct MembershipsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> MembershipsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Get memberships in a group of related accounts."]
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
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
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
                    #[doc = "Optional. The maximum number of accounts to return. The service might return fewer than this value. If unspecified, at most 50 accounts are returned. The maximum value is 1000; values above 1000 are coerced to 1000."]
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
                    #[doc = "\nExecute the request and yield each item in the `relatedAccountGroupMemberships` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_related_account_group_memberships<T>(
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
                        self.stream_related_account_group_memberships_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `relatedAccountGroupMemberships` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]                    pub fn stream_related_account_group_memberships_with_default_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership , crate :: Error >> + 'a{
                        self.stream_related_account_group_memberships_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `relatedAccountGroupMemberships` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]                    pub fn stream_related_account_group_memberships_with_all_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1RelatedAccountGroupMembership , crate :: Error >> + 'a{
                        self.stream_related_account_group_memberships_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `relatedAccountGroupMemberships` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_related_account_group_memberships_with_fields<T, F>(
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
                            #[serde(rename = "relatedAccountGroupMemberships")]
                            pub items: Vec<T>,
                        }
                        impl<T> crate::GetNextPageToken<String> for Page<T> {
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
                        T: crate::GetNextPageToken<String>
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
                    #[doc = r" Requests the default set of fields from the server."]                    pub fn stream_with_default_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse , crate :: Error >> + 'a{
                        self.stream_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                    #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                    #[doc = r" repeated until no page token is returned."]
                    #[doc = r""]
                    #[doc = r" Requests all fields from the server."]                    pub fn stream_with_all_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse , crate :: Error >> + 'a{
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
                        T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned + 'a,
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
                    #[doc = r" the response resource."]                    pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse , crate :: Error >{
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]                    pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudRecaptchaenterpriseV1ListRelatedAccountGroupMembershipsResponse , crate :: Error >{
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
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                    type PageToken = String;
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    async fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned,
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
        /// Type of the `pageToken` and `nextPageToken` fields.
        type PageToken;

        /// Update the current page token of the request.
        fn set_page_token(&mut self, value: Self::PageToken);

        /// Execute the request.
        async fn execute<T>(&mut self) -> Result<T, crate::Error>
        where
            T: GetNextPageToken<Self::PageToken> + ::serde::de::DeserializeOwned;
    }

    /// Return a [`Stream`](::futures::Stream) over all pages of the given API
    /// method.
    pub fn page_stream<M, T>(method: M) -> impl ::futures::Stream<Item = Result<T, crate::Error>>
    where
        M: StreamableMethod,
        T: GetNextPageToken<M::PageToken> + ::serde::de::DeserializeOwned,
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
        T: GetNextPageToken<M::PageToken> + ::serde::de::DeserializeOwned + IntoPageItems,
    {
        use ::futures::StreamExt;
        use ::futures::TryStreamExt;

        page_stream::<M, T>(method)
            .map_ok(|page| ::futures::stream::iter(page.into_page_items()).map(Ok))
            .try_flatten()
    }
}
