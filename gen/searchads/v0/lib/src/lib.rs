#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [customers](resources/customers/struct.CustomersActions.html)\n  * [custom_columns](resources/customers/custom_columns/struct.CustomColumnsActions.html)\n    * [*get*](resources/customers/custom_columns/struct.GetRequestBuilder.html), [*list*](resources/customers/custom_columns/struct.ListRequestBuilder.html)\n  * [search_ads_360](resources/customers/search_ads_360/struct.SearchAds360Actions.html)\n    * [*search*](resources/customers/search_ads_360/struct.SearchRequestBuilder.html), [*searchStream*](resources/customers/search_ads_360/struct.SearchStreamRequestBuilder.html)\n* [search_ads_360_fields](resources/search_ads_360_fields/struct.SearchAds360FieldsActions.html)\n  * [*get*](resources/search_ads_360_fields/struct.GetRequestBuilder.html), [*search*](resources/search_ads_360_fields/struct.SearchRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your advertising data in DoubleClick Search\n\n`https://www.googleapis.com/auth/doubleclicksearch`"]
    pub const DOUBLECLICKSEARCH: &str = "https://www.googleapis.com/auth/doubleclicksearch";
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
    pub struct GoogleAdsSearchads360V0CommonAgeRangeInfo {
        #[doc = "Type of the age range."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonAgeRangeInfoType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonAgeRangeInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonAgeRangeInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonAgeRangeInfoType {
        #[doc = "Between 18 and 24 years old."]
        AgeRange1824,
        #[doc = "Between 25 and 34 years old."]
        AgeRange2534,
        #[doc = "Between 35 and 44 years old."]
        AgeRange3544,
        #[doc = "Between 45 and 54 years old."]
        AgeRange4554,
        #[doc = "Between 55 and 64 years old."]
        AgeRange5564,
        #[doc = "65 years old and beyond."]
        AgeRange65Up,
        #[doc = "Undetermined age range."]
        AgeRangeUndetermined,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0CommonAgeRangeInfoType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange1824 => "AGE_RANGE_18_24",
                GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange2534 => "AGE_RANGE_25_34",
                GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange3544 => "AGE_RANGE_35_44",
                GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange4554 => "AGE_RANGE_45_54",
                GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange5564 => "AGE_RANGE_55_64",
                GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange65Up => "AGE_RANGE_65_UP",
                GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRangeUndetermined => {
                    "AGE_RANGE_UNDETERMINED"
                }
                GoogleAdsSearchads360V0CommonAgeRangeInfoType::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0CommonAgeRangeInfoType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0CommonAgeRangeInfoType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonAgeRangeInfoType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0CommonAgeRangeInfoType, ()> {
            Ok(match s {
                "AGE_RANGE_18_24" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange1824,
                "AGE_RANGE_25_34" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange2534,
                "AGE_RANGE_35_44" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange3544,
                "AGE_RANGE_45_54" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange4554,
                "AGE_RANGE_55_64" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange5564,
                "AGE_RANGE_65_UP" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange65Up,
                "AGE_RANGE_UNDETERMINED" => {
                    GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRangeUndetermined
                }
                "UNKNOWN" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonAgeRangeInfoType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonAgeRangeInfoType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0CommonAgeRangeInfoType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AGE_RANGE_18_24" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange1824,
                "AGE_RANGE_25_34" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange2534,
                "AGE_RANGE_35_44" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange3544,
                "AGE_RANGE_45_54" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange4554,
                "AGE_RANGE_55_64" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange5564,
                "AGE_RANGE_65_UP" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRange65Up,
                "AGE_RANGE_UNDETERMINED" => {
                    GoogleAdsSearchads360V0CommonAgeRangeInfoType::AgeRangeUndetermined
                }
                "UNKNOWN" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0CommonAgeRangeInfoType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonAgeRangeInfoType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonAgeRangeInfoType {
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
    pub struct GoogleAdsSearchads360V0CommonCustomParameter {
        #[doc = "The key matching the parameter tag name."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "The value to be substituted."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonCustomParameter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonCustomParameter {
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
    pub struct GoogleAdsSearchads360V0CommonDeviceInfo {
        #[doc = "Type of the device."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonDeviceInfoType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonDeviceInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonDeviceInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonDeviceInfoType {
        #[doc = "Smart TVs and game consoles."]
        ConnectedTv,
        #[doc = "Computers."]
        Desktop,
        #[doc = "Mobile devices with full browsers."]
        Mobile,
        #[doc = "Other device types."]
        Other,
        #[doc = "Tablets with full browsers."]
        Tablet,
        #[doc = "The value is unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0CommonDeviceInfoType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0CommonDeviceInfoType::ConnectedTv => "CONNECTED_TV",
                GoogleAdsSearchads360V0CommonDeviceInfoType::Desktop => "DESKTOP",
                GoogleAdsSearchads360V0CommonDeviceInfoType::Mobile => "MOBILE",
                GoogleAdsSearchads360V0CommonDeviceInfoType::Other => "OTHER",
                GoogleAdsSearchads360V0CommonDeviceInfoType::Tablet => "TABLET",
                GoogleAdsSearchads360V0CommonDeviceInfoType::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0CommonDeviceInfoType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0CommonDeviceInfoType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonDeviceInfoType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0CommonDeviceInfoType, ()> {
            Ok(match s {
                "CONNECTED_TV" => GoogleAdsSearchads360V0CommonDeviceInfoType::ConnectedTv,
                "DESKTOP" => GoogleAdsSearchads360V0CommonDeviceInfoType::Desktop,
                "MOBILE" => GoogleAdsSearchads360V0CommonDeviceInfoType::Mobile,
                "OTHER" => GoogleAdsSearchads360V0CommonDeviceInfoType::Other,
                "TABLET" => GoogleAdsSearchads360V0CommonDeviceInfoType::Tablet,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonDeviceInfoType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0CommonDeviceInfoType::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonDeviceInfoType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonDeviceInfoType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0CommonDeviceInfoType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONNECTED_TV" => GoogleAdsSearchads360V0CommonDeviceInfoType::ConnectedTv,
                "DESKTOP" => GoogleAdsSearchads360V0CommonDeviceInfoType::Desktop,
                "MOBILE" => GoogleAdsSearchads360V0CommonDeviceInfoType::Mobile,
                "OTHER" => GoogleAdsSearchads360V0CommonDeviceInfoType::Other,
                "TABLET" => GoogleAdsSearchads360V0CommonDeviceInfoType::Tablet,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonDeviceInfoType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0CommonDeviceInfoType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonDeviceInfoType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonDeviceInfoType {
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
    pub struct GoogleAdsSearchads360V0CommonEnhancedCpc {}
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonEnhancedCpc {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonEnhancedCpc {
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
    pub struct GoogleAdsSearchads360V0CommonFrequencyCapEntry {}
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonFrequencyCapEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonFrequencyCapEntry {
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
    pub struct GoogleAdsSearchads360V0CommonGenderInfo {
        #[doc = "Type of the gender."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonGenderInfoType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonGenderInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonGenderInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonGenderInfoType {
        #[doc = "Female."]
        Female,
        #[doc = "Male."]
        Male,
        #[doc = "Undetermined gender."]
        Undetermined,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0CommonGenderInfoType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0CommonGenderInfoType::Female => "FEMALE",
                GoogleAdsSearchads360V0CommonGenderInfoType::Male => "MALE",
                GoogleAdsSearchads360V0CommonGenderInfoType::Undetermined => "UNDETERMINED",
                GoogleAdsSearchads360V0CommonGenderInfoType::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0CommonGenderInfoType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0CommonGenderInfoType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonGenderInfoType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0CommonGenderInfoType, ()> {
            Ok(match s {
                "FEMALE" => GoogleAdsSearchads360V0CommonGenderInfoType::Female,
                "MALE" => GoogleAdsSearchads360V0CommonGenderInfoType::Male,
                "UNDETERMINED" => GoogleAdsSearchads360V0CommonGenderInfoType::Undetermined,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonGenderInfoType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0CommonGenderInfoType::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonGenderInfoType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonGenderInfoType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0CommonGenderInfoType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FEMALE" => GoogleAdsSearchads360V0CommonGenderInfoType::Female,
                "MALE" => GoogleAdsSearchads360V0CommonGenderInfoType::Male,
                "UNDETERMINED" => GoogleAdsSearchads360V0CommonGenderInfoType::Undetermined,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonGenderInfoType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0CommonGenderInfoType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonGenderInfoType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonGenderInfoType {
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
    pub struct GoogleAdsSearchads360V0CommonKeywordInfo {
        #[doc = "The match type of the keyword."]
        #[serde(
            rename = "matchType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub match_type: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0CommonKeywordInfoMatchType,
        >,
        #[doc = "The text of the keyword (at most 80 characters and 10 words)."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonKeywordInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonKeywordInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonKeywordInfoMatchType {
        #[doc = "Broad match."]
        Broad,
        #[doc = "Exact match."]
        Exact,
        #[doc = "Phrase match."]
        Phrase,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0CommonKeywordInfoMatchType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0CommonKeywordInfoMatchType::Broad => "BROAD",
                GoogleAdsSearchads360V0CommonKeywordInfoMatchType::Exact => "EXACT",
                GoogleAdsSearchads360V0CommonKeywordInfoMatchType::Phrase => "PHRASE",
                GoogleAdsSearchads360V0CommonKeywordInfoMatchType::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0CommonKeywordInfoMatchType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0CommonKeywordInfoMatchType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonKeywordInfoMatchType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0CommonKeywordInfoMatchType, ()> {
            Ok(match s {
                "BROAD" => GoogleAdsSearchads360V0CommonKeywordInfoMatchType::Broad,
                "EXACT" => GoogleAdsSearchads360V0CommonKeywordInfoMatchType::Exact,
                "PHRASE" => GoogleAdsSearchads360V0CommonKeywordInfoMatchType::Phrase,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonKeywordInfoMatchType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0CommonKeywordInfoMatchType::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonKeywordInfoMatchType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonKeywordInfoMatchType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0CommonKeywordInfoMatchType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BROAD" => GoogleAdsSearchads360V0CommonKeywordInfoMatchType::Broad,
                "EXACT" => GoogleAdsSearchads360V0CommonKeywordInfoMatchType::Exact,
                "PHRASE" => GoogleAdsSearchads360V0CommonKeywordInfoMatchType::Phrase,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonKeywordInfoMatchType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0CommonKeywordInfoMatchType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonKeywordInfoMatchType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonKeywordInfoMatchType {
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
    pub struct GoogleAdsSearchads360V0CommonLanguageInfo {
        #[doc = "The language constant resource name."]
        #[serde(
            rename = "languageConstant",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_constant: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonLanguageInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonLanguageInfo {
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
    pub struct GoogleAdsSearchads360V0CommonListingGroupInfo {
        #[doc = "Type of the listing group."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0CommonListingGroupInfoType,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonListingGroupInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonListingGroupInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonListingGroupInfoType {
        #[doc = "Subdivision of products along some listing dimension. These nodes are not used by serving to target listing entries, but is purely to define the structure of the tree."]
        Subdivision,
        #[doc = "Listing group unit that defines a bid."]
        Unit,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0CommonListingGroupInfoType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0CommonListingGroupInfoType::Subdivision => "SUBDIVISION",
                GoogleAdsSearchads360V0CommonListingGroupInfoType::Unit => "UNIT",
                GoogleAdsSearchads360V0CommonListingGroupInfoType::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0CommonListingGroupInfoType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0CommonListingGroupInfoType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonListingGroupInfoType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0CommonListingGroupInfoType, ()> {
            Ok(match s {
                "SUBDIVISION" => GoogleAdsSearchads360V0CommonListingGroupInfoType::Subdivision,
                "UNIT" => GoogleAdsSearchads360V0CommonListingGroupInfoType::Unit,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonListingGroupInfoType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0CommonListingGroupInfoType::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonListingGroupInfoType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonListingGroupInfoType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0CommonListingGroupInfoType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SUBDIVISION" => GoogleAdsSearchads360V0CommonListingGroupInfoType::Subdivision,
                "UNIT" => GoogleAdsSearchads360V0CommonListingGroupInfoType::Unit,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonListingGroupInfoType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0CommonListingGroupInfoType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonListingGroupInfoType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonListingGroupInfoType {
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
    pub struct GoogleAdsSearchads360V0CommonLocationGroupInfo {
        #[doc = "FeedItemSets whose FeedItems are targeted. If multiple IDs are specified, then all items that appear in at least one set are targeted. This field cannot be used with geo_target_constants. This is optional and can only be set in CREATE operations."]
        #[serde(
            rename = "feedItemSets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feed_item_sets: ::std::option::Option<Vec<String>>,
        #[doc = "Geo target constant(s) restricting the scope of the geographic area within the feed. Currently only one geo target constant is allowed."]
        #[serde(
            rename = "geoTargetConstants",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub geo_target_constants: ::std::option::Option<Vec<String>>,
        #[doc = "Distance in units specifying the radius around targeted locations. This is required and must be set in CREATE operations."]
        #[serde(
            rename = "radius",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub radius: ::std::option::Option<i64>,
        #[doc = "Unit of the radius. Miles and meters are supported for geo target constants. Milli miles and meters are supported for feed item sets. This is required and must be set in CREATE operations."]
        #[serde(
            rename = "radiusUnits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub radius_units: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonLocationGroupInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonLocationGroupInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits {
        #[doc = "Meters"]
        Meters,
        #[doc = "Miles"]
        Miles,
        #[doc = "Milli Miles"]
        MilliMiles,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits::Meters => "METERS",
                GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits::Miles => "MILES",
                GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits::MilliMiles => {
                    "MILLI_MILES"
                }
                GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits::Unspecified => {
                    "UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits, ()>
        {
            Ok(match s {
                "METERS" => GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits::Meters,
                "MILES" => GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits::Miles,
                "MILLI_MILES" => {
                    GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits::MilliMiles
                }
                "UNKNOWN" => GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits::Unspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "METERS" => GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits::Meters,
                "MILES" => GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits::Miles,
                "MILLI_MILES" => {
                    GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits::MilliMiles
                }
                "UNKNOWN" => GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits::Unspecified
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
        for GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0CommonLocationGroupInfoRadiusUnits
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
    pub struct GoogleAdsSearchads360V0CommonLocationInfo {
        #[doc = "The geo target constant resource name."]
        #[serde(
            rename = "geoTargetConstant",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub geo_target_constant: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonLocationInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonLocationInfo {
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
    pub struct GoogleAdsSearchads360V0CommonManualCpa {}
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonManualCpa {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonManualCpa {
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
    pub struct GoogleAdsSearchads360V0CommonManualCpc {
        #[doc = "Whether bids are to be enhanced based on conversion optimizer data."]
        #[serde(
            rename = "enhancedCpcEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enhanced_cpc_enabled: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonManualCpc {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonManualCpc {
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
    pub struct GoogleAdsSearchads360V0CommonManualCpm {}
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonManualCpm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonManualCpm {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0CommonMaximizeConversionValue {
        #[doc = "Maximum bid limit that can be set by the bid strategy. The limit applies to all keywords managed by the strategy. Mutable for portfolio bidding strategies only."]
        #[serde(
            rename = "cpcBidCeilingMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cpc_bid_ceiling_micros: ::std::option::Option<i64>,
        #[doc = "Minimum bid limit that can be set by the bid strategy. The limit applies to all keywords managed by the strategy. Mutable for portfolio bidding strategies only."]
        #[serde(
            rename = "cpcBidFloorMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cpc_bid_floor_micros: ::std::option::Option<i64>,
        #[doc = "The target return on ad spend (ROAS) option. If set, the bid strategy will maximize revenue while averaging the target return on ad spend. If the target ROAS is high, the bid strategy may not be able to spend the full budget. If the target ROAS is not set, the bid strategy will aim to achieve the highest possible ROAS for the budget."]
        #[serde(
            rename = "targetRoas",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_roas: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0CommonMaximizeConversionValue
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonMaximizeConversionValue {
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
    pub struct GoogleAdsSearchads360V0CommonMaximizeConversions {
        #[doc = "Maximum bid limit that can be set by the bid strategy. The limit applies to all keywords managed by the strategy. Mutable for portfolio bidding strategies only."]
        #[serde(
            rename = "cpcBidCeilingMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cpc_bid_ceiling_micros: ::std::option::Option<i64>,
        #[doc = "Minimum bid limit that can be set by the bid strategy. The limit applies to all keywords managed by the strategy. Mutable for portfolio bidding strategies only."]
        #[serde(
            rename = "cpcBidFloorMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cpc_bid_floor_micros: ::std::option::Option<i64>,
        #[doc = "The target cost-per-action (CPA) option. This is the average amount that you would like to spend per conversion action specified in micro units of the bidding strategy’s currency. If set, the bid strategy will get as many conversions as possible at or below the target cost-per-action. If the target CPA is not set, the bid strategy will aim to achieve the lowest possible CPA given the budget."]
        #[serde(
            rename = "targetCpaMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub target_cpa_micros: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonMaximizeConversions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonMaximizeConversions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0CommonMetrics {
        #[doc = "The percent of your ad impressions that are shown as the very first ad above the organic search results."]
        #[serde(
            rename = "absoluteTopImpressionPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub absolute_top_impression_percentage: ::std::option::Option<f64>,
        #[doc = "The total number of conversions. This includes all conversions regardless of the value of include_in_conversions_metric."]
        #[serde(
            rename = "allConversions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_conversions: ::std::option::Option<f64>,
        #[doc = "The total number of conversions. This includes all conversions regardless of the value of include_in_conversions_metric. When this column is selected with date, the values in date column means the conversion date. Details for the by_conversion_date columns are available at https://support.google.com/sa360/answer/9250611."]
        #[serde(
            rename = "allConversionsByConversionDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_conversions_by_conversion_date: ::std::option::Option<f64>,
        #[doc = "The number of times people clicked the “Call” button to call a store during or after clicking an ad. This number doesn’t include whether or not calls were connected, or the duration of any calls. This metric applies to feed items only."]
        #[serde(
            rename = "allConversionsFromClickToCall",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_conversions_from_click_to_call: ::std::option::Option<f64>,
        #[doc = "The number of times people clicked a “Get directions” button to navigate to a store after clicking an ad. This metric applies to feed items only."]
        #[serde(
            rename = "allConversionsFromDirections",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_conversions_from_directions: ::std::option::Option<f64>,
        #[doc = "All conversions from interactions (as oppose to view through conversions) divided by the number of ad interactions."]
        #[serde(
            rename = "allConversionsFromInteractionsRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_conversions_from_interactions_rate: ::std::option::Option<f64>,
        #[doc = "The value of all conversions from interactions divided by the total number of interactions."]
        #[serde(
            rename = "allConversionsFromInteractionsValuePerInteraction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_conversions_from_interactions_value_per_interaction: ::std::option::Option<f64>,
        #[doc = "The number of times people clicked a link to view a store’s menu after clicking an ad. This metric applies to feed items only."]
        #[serde(
            rename = "allConversionsFromMenu",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_conversions_from_menu: ::std::option::Option<f64>,
        #[doc = "The number of times people placed an order at a store after clicking an ad. This metric applies to feed items only."]
        #[serde(
            rename = "allConversionsFromOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_conversions_from_order: ::std::option::Option<f64>,
        #[doc = "The number of other conversions (for example, posting a review or saving a location for a store) that occurred after people clicked an ad. This metric applies to feed items only."]
        #[serde(
            rename = "allConversionsFromOtherEngagement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_conversions_from_other_engagement: ::std::option::Option<f64>,
        #[doc = "Estimated number of times people visited a store after clicking an ad. This metric applies to feed items only."]
        #[serde(
            rename = "allConversionsFromStoreVisit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_conversions_from_store_visit: ::std::option::Option<f64>,
        #[doc = "The number of times that people were taken to a store’s URL after clicking an ad. This metric applies to feed items only."]
        #[serde(
            rename = "allConversionsFromStoreWebsite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_conversions_from_store_website: ::std::option::Option<f64>,
        #[doc = "The value of all conversions."]
        #[serde(
            rename = "allConversionsValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_conversions_value: ::std::option::Option<f64>,
        #[doc = "The value of all conversions. When this column is selected with date, the values in date column means the conversion date. Details for the by_conversion_date columns are available at https://support.google.com/sa360/answer/9250611."]
        #[serde(
            rename = "allConversionsValueByConversionDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_conversions_value_by_conversion_date: ::std::option::Option<f64>,
        #[doc = "The value of all conversions divided by the total cost of ad interactions (such as clicks for text ads or views for video ads)."]
        #[serde(
            rename = "allConversionsValuePerCost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub all_conversions_value_per_cost: ::std::option::Option<f64>,
        #[doc = "The average amount you pay per interaction. This amount is the total cost of your ads divided by the total number of interactions."]
        #[serde(
            rename = "averageCost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub average_cost: ::std::option::Option<f64>,
        #[doc = "The total cost of all clicks divided by the total number of clicks received."]
        #[serde(
            rename = "averageCpc",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub average_cpc: ::std::option::Option<f64>,
        #[doc = "Average cost-per-thousand impressions (CPM)."]
        #[serde(
            rename = "averageCpm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub average_cpm: ::std::option::Option<f64>,
        #[doc = "The number of clicks."]
        #[serde(
            rename = "clicks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub clicks: ::std::option::Option<i64>,
        #[doc = "The number of client account conversions. This only includes conversion actions which include_in_client_account_conversions_metric attribute is set to true. If you use conversion-based bidding, your bid strategies will optimize for these conversions."]
        #[serde(
            rename = "clientAccountConversions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_account_conversions: ::std::option::Option<f64>,
        #[doc = "The value of client account conversions. This only includes conversion actions which include_in_client_account_conversions_metric attribute is set to true. If you use conversion-based bidding, your bid strategies will optimize for these conversions."]
        #[serde(
            rename = "clientAccountConversionsValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_account_conversions_value: ::std::option::Option<f64>,
        #[doc = "The total number of view-through conversions. These happen when a customer sees an image or rich media ad, then later completes a conversion on your site without interacting with (for example, clicking on) another ad."]
        #[serde(
            rename = "clientAccountViewThroughConversions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub client_account_view_through_conversions: ::std::option::Option<i64>,
        #[doc = "The estimated percent of times that your ad was eligible to show on the Display Network but didn’t because your budget was too low. Note: Content budget lost impression share is reported in the range of 0 to 0.9. Any value above 0.9 is reported as 0.9001."]
        #[serde(
            rename = "contentBudgetLostImpressionShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_budget_lost_impression_share: ::std::option::Option<f64>,
        #[doc = "The impressions you’ve received on the Display Network divided by the estimated number of impressions you were eligible to receive. Note: Content impression share is reported in the range of 0.1 to 1. Any value below 0.1 is reported as 0.0999."]
        #[serde(
            rename = "contentImpressionShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_impression_share: ::std::option::Option<f64>,
        #[doc = "The estimated percentage of impressions on the Display Network that your ads didn’t receive due to poor Ad Rank. Note: Content rank lost impression share is reported in the range of 0 to 0.9. Any value above 0.9 is reported as 0.9001."]
        #[serde(
            rename = "contentRankLostImpressionShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_rank_lost_impression_share: ::std::option::Option<f64>,
        #[doc = "The number of conversions. This only includes conversion actions which include_in_conversions_metric attribute is set to true. If you use conversion-based bidding, your bid strategies will optimize for these conversions."]
        #[serde(
            rename = "conversions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversions: ::std::option::Option<f64>,
        #[doc = "The sum of conversions by conversion date for biddable conversion types. Can be fractional due to attribution modeling. When this column is selected with date, the values in date column means the conversion date."]
        #[serde(
            rename = "conversionsByConversionDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversions_by_conversion_date: ::std::option::Option<f64>,
        #[doc = "Average biddable conversions (from interaction) per conversion eligible interaction. Shows how often, on average, an ad interaction leads to a biddable conversion."]
        #[serde(
            rename = "conversionsFromInteractionsRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversions_from_interactions_rate: ::std::option::Option<f64>,
        #[doc = "The value of conversions from interactions divided by the number of ad interactions. This only includes conversion actions which include_in_conversions_metric attribute is set to true. If you use conversion-based bidding, your bid strategies will optimize for these conversions."]
        #[serde(
            rename = "conversionsFromInteractionsValuePerInteraction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversions_from_interactions_value_per_interaction: ::std::option::Option<f64>,
        #[doc = "The sum of conversion values for the conversions included in the “conversions” field. This metric is useful only if you entered a value for your conversion actions."]
        #[serde(
            rename = "conversionsValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversions_value: ::std::option::Option<f64>,
        #[doc = "The sum of biddable conversions value by conversion date. When this column is selected with date, the values in date column means the conversion date."]
        #[serde(
            rename = "conversionsValueByConversionDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversions_value_by_conversion_date: ::std::option::Option<f64>,
        #[doc = "The value of biddable conversion divided by the total cost of conversion eligible interactions."]
        #[serde(
            rename = "conversionsValuePerCost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversions_value_per_cost: ::std::option::Option<f64>,
        #[doc = "The sum of your cost-per-click (CPC) and cost-per-thousand impressions (CPM) costs during this period."]
        #[serde(
            rename = "costMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cost_micros: ::std::option::Option<i64>,
        #[doc = "The cost of ad interactions divided by all conversions."]
        #[serde(
            rename = "costPerAllConversions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cost_per_all_conversions: ::std::option::Option<f64>,
        #[doc = "Average conversion eligible cost per biddable conversion."]
        #[serde(
            rename = "costPerConversion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cost_per_conversion: ::std::option::Option<f64>,
        #[doc = "The cost of ad interactions divided by current model attributed conversions. This only includes conversion actions which include_in_conversions_metric attribute is set to true. If you use conversion-based bidding, your bid strategies will optimize for these conversions."]
        #[serde(
            rename = "costPerCurrentModelAttributedConversion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cost_per_current_model_attributed_conversion: ::std::option::Option<f64>,
        #[doc = "Conversions from when a customer clicks on an ad on one device, then converts on a different device or browser. Cross-device conversions are already included in all_conversions."]
        #[serde(
            rename = "crossDeviceConversions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cross_device_conversions: ::std::option::Option<f64>,
        #[doc = "The sum of the value of cross-device conversions."]
        #[serde(
            rename = "crossDeviceConversionsValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cross_device_conversions_value: ::std::option::Option<f64>,
        #[doc = "The number of clicks your ad receives (Clicks) divided by the number of times your ad is shown (Impressions)."]
        #[serde(
            rename = "ctr",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ctr: ::std::option::Option<f64>,
        #[doc = "The creative historical quality score."]
        #[serde(
            rename = "historicalCreativeQualityScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub historical_creative_quality_score: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore,
        >,
        #[doc = "The quality of historical landing page experience."]
        #[serde(
            rename = "historicalLandingPageQualityScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub historical_landing_page_quality_score: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore,
        >,
        #[doc = "The historical quality score."]
        #[serde(
            rename = "historicalQualityScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub historical_quality_score: ::std::option::Option<i64>,
        #[doc = "The historical search predicted click through rate (CTR)."]
        #[serde(
            rename = "historicalSearchPredictedCtr",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub historical_search_predicted_ctr: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr,
        >,
        #[doc = "Count of how often your ad has appeared on a search results page or website on the Google Network."]
        #[serde(
            rename = "impressions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub impressions: ::std::option::Option<i64>,
        #[doc = "The types of payable and free interactions."]
        #[serde(
            rename = "interactionEventTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interaction_event_types: ::std::option::Option<
            Vec<crate::schemas::GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems>,
        >,
        #[doc = "How often people interact with your ad after it is shown to them. This is the number of interactions divided by the number of times your ad is shown."]
        #[serde(
            rename = "interactionRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interaction_rate: ::std::option::Option<f64>,
        #[doc = "The number of interactions. An interaction is the main user action associated with an ad format-clicks for text and shopping ads, views for video ads, and so on."]
        #[serde(
            rename = "interactions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub interactions: ::std::option::Option<i64>,
        #[doc = "The percentage of clicks filtered out of your total number of clicks (filtered + non-filtered clicks) during the reporting period."]
        #[serde(
            rename = "invalidClickRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub invalid_click_rate: ::std::option::Option<f64>,
        #[doc = "Number of clicks Google considers illegitimate and doesn’t charge you for."]
        #[serde(
            rename = "invalidClicks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub invalid_clicks: ::std::option::Option<i64>,
        #[doc = "The percentage of mobile clicks that go to a mobile-friendly page."]
        #[serde(
            rename = "mobileFriendlyClicksPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mobile_friendly_clicks_percentage: ::std::option::Option<f64>,
        #[doc = "The percentage of the customer’s Shopping or Search ad impressions that are shown in the most prominent Shopping position. See https://support.google.com/sa360/answer/9566729 for details. Any value below 0.1 is reported as 0.0999."]
        #[serde(
            rename = "searchAbsoluteTopImpressionShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_absolute_top_impression_share: ::std::option::Option<f64>,
        #[doc = "The number estimating how often your ad wasn’t the very first ad above the organic search results due to a low budget. Note: Search budget lost absolute top impression share is reported in the range of 0 to 0.9. Any value above 0.9 is reported as 0.9001."]
        #[serde(
            rename = "searchBudgetLostAbsoluteTopImpressionShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_budget_lost_absolute_top_impression_share: ::std::option::Option<f64>,
        #[doc = "The estimated percent of times that your ad was eligible to show on the Search Network but didn’t because your budget was too low. Note: Search budget lost impression share is reported in the range of 0 to 0.9. Any value above 0.9 is reported as 0.9001."]
        #[serde(
            rename = "searchBudgetLostImpressionShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_budget_lost_impression_share: ::std::option::Option<f64>,
        #[doc = "The number estimating how often your ad didn’t show anywhere above the organic search results due to a low budget. Note: Search budget lost top impression share is reported in the range of 0 to 0.9. Any value above 0.9 is reported as 0.9001."]
        #[serde(
            rename = "searchBudgetLostTopImpressionShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_budget_lost_top_impression_share: ::std::option::Option<f64>,
        #[doc = "The number of clicks you’ve received on the Search Network divided by the estimated number of clicks you were eligible to receive. Note: Search click share is reported in the range of 0.1 to 1. Any value below 0.1 is reported as 0.0999."]
        #[serde(
            rename = "searchClickShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_click_share: ::std::option::Option<f64>,
        #[doc = "The impressions you’ve received divided by the estimated number of impressions you were eligible to receive on the Search Network for search terms that matched your keywords exactly (or were close variants of your keyword), regardless of your keyword match types. Note: Search exact match impression share is reported in the range of 0.1 to 1. Any value below 0.1 is reported as 0.0999."]
        #[serde(
            rename = "searchExactMatchImpressionShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_exact_match_impression_share: ::std::option::Option<f64>,
        #[doc = "The impressions you’ve received on the Search Network divided by the estimated number of impressions you were eligible to receive. Note: Search impression share is reported in the range of 0.1 to 1. Any value below 0.1 is reported as 0.0999."]
        #[serde(
            rename = "searchImpressionShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_impression_share: ::std::option::Option<f64>,
        #[doc = "The number estimating how often your ad wasn’t the very first ad above the organic search results due to poor Ad Rank. Note: Search rank lost absolute top impression share is reported in the range of 0 to 0.9. Any value above 0.9 is reported as 0.9001."]
        #[serde(
            rename = "searchRankLostAbsoluteTopImpressionShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_rank_lost_absolute_top_impression_share: ::std::option::Option<f64>,
        #[doc = "The estimated percentage of impressions on the Search Network that your ads didn’t receive due to poor Ad Rank. Note: Search rank lost impression share is reported in the range of 0 to 0.9. Any value above 0.9 is reported as 0.9001."]
        #[serde(
            rename = "searchRankLostImpressionShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_rank_lost_impression_share: ::std::option::Option<f64>,
        #[doc = "The number estimating how often your ad didn’t show anywhere above the organic search results due to poor Ad Rank. Note: Search rank lost top impression share is reported in the range of 0 to 0.9. Any value above 0.9 is reported as 0.9001."]
        #[serde(
            rename = "searchRankLostTopImpressionShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_rank_lost_top_impression_share: ::std::option::Option<f64>,
        #[doc = "The impressions you’ve received in the top location (anywhere above the organic search results) compared to the estimated number of impressions you were eligible to receive in the top location. Note: Search top impression share is reported in the range of 0.1 to 1. Any value below 0.1 is reported as 0.0999."]
        #[serde(
            rename = "searchTopImpressionShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_top_impression_share: ::std::option::Option<f64>,
        #[doc = "The percent of your ad impressions that are shown anywhere above the organic search results."]
        #[serde(
            rename = "topImpressionPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top_impression_percentage: ::std::option::Option<f64>,
        #[doc = "The value of all conversions divided by the number of all conversions."]
        #[serde(
            rename = "valuePerAllConversions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_per_all_conversions: ::std::option::Option<f64>,
        #[doc = "The value of all conversions divided by the number of all conversions. When this column is selected with date, the values in date column means the conversion date. Details for the by_conversion_date columns are available at https://support.google.com/sa360/answer/9250611."]
        #[serde(
            rename = "valuePerAllConversionsByConversionDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_per_all_conversions_by_conversion_date: ::std::option::Option<f64>,
        #[doc = "The value of biddable conversion divided by the number of biddable conversions. Shows how much, on average, each of the biddable conversions is worth."]
        #[serde(
            rename = "valuePerConversion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_per_conversion: ::std::option::Option<f64>,
        #[doc = "Biddable conversions value by conversion date divided by biddable conversions by conversion date. Shows how much, on average, each of the biddable conversions is worth (by conversion date). When this column is selected with date, the values in date column means the conversion date."]
        #[serde(
            rename = "valuePerConversionsByConversionDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_per_conversions_by_conversion_date: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonMetrics {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonMetrics {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore {
        #[doc = "Quality of the creative is above average."]
        AboveAverage,
        #[doc = "Quality of the creative is average."]
        Average,
        #[doc = "Quality of the creative is below average."]
        BelowAverage,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore :: AboveAverage => "ABOVE_AVERAGE" , GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore :: Average => "AVERAGE" , GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore :: BelowAverage => "BELOW_AVERAGE" , GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore,
            (),
        > {
            Ok(match s {
                "ABOVE_AVERAGE" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore::AboveAverage
                }
                "AVERAGE" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore::Average
                }
                "BELOW_AVERAGE" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore::BelowAverage
                }
                "UNKNOWN" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore::Unknown
                }
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore::Unspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ABOVE_AVERAGE" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore::AboveAverage
                }
                "AVERAGE" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore::Average
                }
                "BELOW_AVERAGE" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore::BelowAverage
                }
                "UNKNOWN" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore::Unknown
                }
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore::Unspecified
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
        for GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0CommonMetricsHistoricalCreativeQualityScore
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore {
        #[doc = "Quality of the creative is above average."]
        AboveAverage,
        #[doc = "Quality of the creative is average."]
        Average,
        #[doc = "Quality of the creative is below average."]
        BelowAverage,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore :: AboveAverage => "ABOVE_AVERAGE" , GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore :: Average => "AVERAGE" , GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore :: BelowAverage => "BELOW_AVERAGE" , GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore,
            (),
        > {
            Ok (match s { "ABOVE_AVERAGE" => GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore :: AboveAverage , "AVERAGE" => GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore :: Average , "BELOW_AVERAGE" => GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore :: BelowAverage , "UNKNOWN" => GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore :: Unspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ABOVE_AVERAGE" => GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore :: AboveAverage , "AVERAGE" => GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore :: Average , "BELOW_AVERAGE" => GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore :: BelowAverage , "UNKNOWN" => GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0CommonMetricsHistoricalLandingPageQualityScore
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr {
        #[doc = "Quality of the creative is above average."]
        AboveAverage,
        #[doc = "Quality of the creative is average."]
        Average,
        #[doc = "Quality of the creative is below average."]
        BelowAverage,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr::AboveAverage => {
                    "ABOVE_AVERAGE"
                }
                GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr::Average => {
                    "AVERAGE"
                }
                GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr::BelowAverage => {
                    "BELOW_AVERAGE"
                }
                GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr::Unknown => {
                    "UNKNOWN"
                }
                GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr::Unspecified => {
                    "UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr,
            (),
        > {
            Ok(match s {
                "ABOVE_AVERAGE" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr::AboveAverage
                }
                "AVERAGE" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr::Average
                }
                "BELOW_AVERAGE" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr::BelowAverage
                }
                "UNKNOWN" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr::Unknown
                }
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr::Unspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ABOVE_AVERAGE" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr::AboveAverage
                }
                "AVERAGE" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr::Average
                }
                "BELOW_AVERAGE" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr::BelowAverage
                }
                "UNKNOWN" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr::Unknown
                }
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr::Unspecified
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
        for GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0CommonMetricsHistoricalSearchPredictedCtr
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems {
        #[doc = "Click to site. In most cases, this interaction navigates to an external location, usually the advertiser’s landing page. This is also the default InteractionEventType for click events."]
        Click,
        #[doc = "The user’s expressed intent to engage with the ad in-place."]
        Engagement,
        #[doc = "The default InteractionEventType for ad conversion events. This is used when an ad conversion row does NOT indicate that the free interactions (for example, the ad conversions) should be ‘promoted’ and reported as part of the core metrics. These are simply other (ad) conversions."]
        None,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
        #[doc = "User viewed a video ad."]
        VideoView,
    }
    impl GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::Click => "CLICK",
                GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::Engagement => {
                    "ENGAGEMENT"
                }
                GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::None => "NONE",
                GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::Unknown => {
                    "UNKNOWN"
                }
                GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::Unspecified => {
                    "UNSPECIFIED"
                }
                GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::VideoView => {
                    "VIDEO_VIEW"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems, ()>
        {
            Ok(match s {
                "CLICK" => GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::Click,
                "ENGAGEMENT" => {
                    GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::Engagement
                }
                "NONE" => GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::None,
                "UNKNOWN" => {
                    GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::Unknown
                }
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::Unspecified
                }
                "VIDEO_VIEW" => {
                    GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::VideoView
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CLICK" => GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::Click,
                "ENGAGEMENT" => {
                    GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::Engagement
                }
                "NONE" => GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::None,
                "UNKNOWN" => {
                    GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::Unknown
                }
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::Unspecified
                }
                "VIDEO_VIEW" => {
                    GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems::VideoView
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
        for GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0CommonMetricsInteractionEventTypesItems
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
    pub struct GoogleAdsSearchads360V0CommonPercentCpc {
        #[doc = "Maximum bid limit that can be set by the bid strategy. This is an optional field entered by the advertiser and specified in local micros. Note: A zero value is interpreted in the same way as having bid_ceiling undefined."]
        #[serde(
            rename = "cpcBidCeilingMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cpc_bid_ceiling_micros: ::std::option::Option<i64>,
        #[doc = "Adjusts the bid for each auction upward or downward, depending on the likelihood of a conversion. Individual bids may exceed cpc_bid_ceiling_micros, but the average bid amount for a campaign should not."]
        #[serde(
            rename = "enhancedCpcEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enhanced_cpc_enabled: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonPercentCpc {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonPercentCpc {
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
    pub struct GoogleAdsSearchads360V0CommonRealTimeBiddingSetting {
        #[doc = "Whether the campaign is opted in to real-time bidding."]
        #[serde(
            rename = "optIn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub opt_in: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0CommonRealTimeBiddingSetting
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonRealTimeBiddingSetting {
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
    pub struct GoogleAdsSearchads360V0CommonSegments {
        #[doc = "Resource name of the conversion action."]
        #[serde(
            rename = "conversionAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversion_action: ::std::option::Option<String>,
        #[doc = "Conversion action category."]
        #[serde(
            rename = "conversionActionCategory",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversion_action_category: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory,
        >,
        #[doc = "Conversion action name."]
        #[serde(
            rename = "conversionActionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversion_action_name: ::std::option::Option<String>,
        #[doc = "Date to which metrics apply. yyyy-MM-dd format, for example, 2018-04-17."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<String>,
        #[doc = "Day of the week, for example, MONDAY."]
        #[serde(
            rename = "dayOfWeek",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day_of_week:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonSegmentsDayOfWeek>,
        #[doc = "Device to which metrics apply."]
        #[serde(
            rename = "device",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonSegmentsDevice>,
        #[doc = "Month as represented by the date of the first day of a month. Formatted as yyyy-MM-dd."]
        #[serde(
            rename = "month",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub month: ::std::option::Option<String>,
        #[doc = "Quarter as represented by the date of the first day of a quarter. Uses the calendar year for quarters, for example, the second quarter of 2018 starts on 2018-04-01. Formatted as yyyy-MM-dd."]
        #[serde(
            rename = "quarter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quarter: ::std::option::Option<String>,
        #[doc = "Week as defined as Monday through Sunday, and represented by the date of Monday. Formatted as yyyy-MM-dd."]
        #[serde(
            rename = "week",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub week: ::std::option::Option<String>,
        #[doc = "Year, formatted as yyyy."]
        #[serde(
            rename = "year",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub year: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonSegments {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonSegments {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory {
        #[doc = "The addition of items to a shopping cart or bag on an advertiser site."]
        AddToCart,
        #[doc = "When someone enters the checkout flow on an advertiser site."]
        BeginCheckout,
        #[doc = "A booking of an appointment with an advertiser’s business."]
        BookAppointment,
        #[doc = "A call, SMS, email, chat or other type of contact to an advertiser."]
        Contact,
        #[doc = "A lead conversion imported from an external source into Google Ads, that has further completed a chosen stage as defined by the lead gen advertiser."]
        ConvertedLead,
        #[doc = "Default category."]
        Default,
        #[doc = "Software download action (as for an app)."]
        Download,
        #[doc = "A website engagement event such as long site time or a Google Analytics (GA) Smart Goal. Intended to be used for GA, Firebase, GA Gold goal imports."]
        Engagement,
        #[doc = "A search for an advertiser’s business location with intention to visit."]
        GetDirections,
        #[doc = "A lead conversion imported from an external source into Google Ads."]
        ImportedLead,
        #[doc = "Lead-generating action."]
        Lead,
        #[doc = "A click to an advertiser’s partner’s site."]
        OutboundClick,
        #[doc = "User visiting a page."]
        PageView,
        #[doc = "A call to indicate interest in an advertiser’s offering."]
        PhoneCallLead,
        #[doc = "Purchase, sales, or “order placed” event."]
        Purchase,
        #[doc = "A lead conversion imported from an external source into Google Ads, that has been further qualified by the advertiser (marketing/sales team). In the lead-to-sale journey, advertisers get leads, then act on them by reaching out to the consumer. If the consumer is interested and may end up buying their product, the advertiser marks such leads as “qualified leads”."]
        QualifiedLead,
        #[doc = "A quote or price estimate request."]
        RequestQuote,
        #[doc = "Signup user action."]
        Signup,
        #[doc = "A sale occurring in a physical store."]
        StoreSale,
        #[doc = "A visit to a physical store location."]
        StoreVisit,
        #[doc = "A submission of a form on an advertiser site indicating business interest."]
        SubmitLeadForm,
        #[doc = "The start of a paid subscription for a product or service."]
        SubscribePaid,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::AddToCart => {
                    "ADD_TO_CART"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::BeginCheckout => {
                    "BEGIN_CHECKOUT"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::BookAppointment => {
                    "BOOK_APPOINTMENT"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Contact => "CONTACT",
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::ConvertedLead => {
                    "CONVERTED_LEAD"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Default => "DEFAULT",
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Download => {
                    "DOWNLOAD"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Engagement => {
                    "ENGAGEMENT"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::GetDirections => {
                    "GET_DIRECTIONS"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::ImportedLead => {
                    "IMPORTED_LEAD"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Lead => "LEAD",
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::OutboundClick => {
                    "OUTBOUND_CLICK"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::PageView => {
                    "PAGE_VIEW"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::PhoneCallLead => {
                    "PHONE_CALL_LEAD"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Purchase => {
                    "PURCHASE"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::QualifiedLead => {
                    "QUALIFIED_LEAD"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::RequestQuote => {
                    "REQUEST_QUOTE"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Signup => "SIGNUP",
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::StoreSale => {
                    "STORE_SALE"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::StoreVisit => {
                    "STORE_VISIT"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::SubmitLeadForm => {
                    "SUBMIT_LEAD_FORM"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::SubscribePaid => {
                    "SUBSCRIBE_PAID"
                }
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Unspecified => {
                    "UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory, ()>
        {
            Ok(match s {
                "ADD_TO_CART" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::AddToCart
                }
                "BEGIN_CHECKOUT" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::BeginCheckout
                }
                "BOOK_APPOINTMENT" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::BookAppointment
                }
                "CONTACT" => GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Contact,
                "CONVERTED_LEAD" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::ConvertedLead
                }
                "DEFAULT" => GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Default,
                "DOWNLOAD" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Download
                }
                "ENGAGEMENT" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Engagement
                }
                "GET_DIRECTIONS" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::GetDirections
                }
                "IMPORTED_LEAD" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::ImportedLead
                }
                "LEAD" => GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Lead,
                "OUTBOUND_CLICK" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::OutboundClick
                }
                "PAGE_VIEW" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::PageView
                }
                "PHONE_CALL_LEAD" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::PhoneCallLead
                }
                "PURCHASE" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Purchase
                }
                "QUALIFIED_LEAD" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::QualifiedLead
                }
                "REQUEST_QUOTE" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::RequestQuote
                }
                "SIGNUP" => GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Signup,
                "STORE_SALE" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::StoreSale
                }
                "STORE_VISIT" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::StoreVisit
                }
                "SUBMIT_LEAD_FORM" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::SubmitLeadForm
                }
                "SUBSCRIBE_PAID" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::SubscribePaid
                }
                "UNKNOWN" => GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Unspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADD_TO_CART" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::AddToCart
                }
                "BEGIN_CHECKOUT" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::BeginCheckout
                }
                "BOOK_APPOINTMENT" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::BookAppointment
                }
                "CONTACT" => GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Contact,
                "CONVERTED_LEAD" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::ConvertedLead
                }
                "DEFAULT" => GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Default,
                "DOWNLOAD" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Download
                }
                "ENGAGEMENT" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Engagement
                }
                "GET_DIRECTIONS" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::GetDirections
                }
                "IMPORTED_LEAD" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::ImportedLead
                }
                "LEAD" => GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Lead,
                "OUTBOUND_CLICK" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::OutboundClick
                }
                "PAGE_VIEW" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::PageView
                }
                "PHONE_CALL_LEAD" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::PhoneCallLead
                }
                "PURCHASE" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Purchase
                }
                "QUALIFIED_LEAD" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::QualifiedLead
                }
                "REQUEST_QUOTE" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::RequestQuote
                }
                "SIGNUP" => GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Signup,
                "STORE_SALE" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::StoreSale
                }
                "STORE_VISIT" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::StoreVisit
                }
                "SUBMIT_LEAD_FORM" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::SubmitLeadForm
                }
                "SUBSCRIBE_PAID" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::SubscribePaid
                }
                "UNKNOWN" => GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory::Unspecified
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
        for GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0CommonSegmentsConversionActionCategory
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonSegmentsDayOfWeek {
        #[doc = "Friday."]
        Friday,
        #[doc = "Monday."]
        Monday,
        #[doc = "Saturday."]
        Saturday,
        #[doc = "Sunday."]
        Sunday,
        #[doc = "Thursday."]
        Thursday,
        #[doc = "Tuesday."]
        Tuesday,
        #[doc = "The value is unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
        #[doc = "Wednesday."]
        Wednesday,
    }
    impl GoogleAdsSearchads360V0CommonSegmentsDayOfWeek {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Friday => "FRIDAY",
                GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Monday => "MONDAY",
                GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Saturday => "SATURDAY",
                GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Sunday => "SUNDAY",
                GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Thursday => "THURSDAY",
                GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Tuesday => "TUESDAY",
                GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Unspecified => "UNSPECIFIED",
                GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Wednesday => "WEDNESDAY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0CommonSegmentsDayOfWeek {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonSegmentsDayOfWeek {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0CommonSegmentsDayOfWeek, ()> {
            Ok(match s {
                "FRIDAY" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Friday,
                "MONDAY" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Monday,
                "SATURDAY" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Saturday,
                "SUNDAY" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Sunday,
                "THURSDAY" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Thursday,
                "TUESDAY" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Tuesday,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Unspecified,
                "WEDNESDAY" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Wednesday,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonSegmentsDayOfWeek {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonSegmentsDayOfWeek {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0CommonSegmentsDayOfWeek {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FRIDAY" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Friday,
                "MONDAY" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Monday,
                "SATURDAY" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Saturday,
                "SUNDAY" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Sunday,
                "THURSDAY" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Thursday,
                "TUESDAY" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Tuesday,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Unspecified,
                "WEDNESDAY" => GoogleAdsSearchads360V0CommonSegmentsDayOfWeek::Wednesday,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonSegmentsDayOfWeek {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonSegmentsDayOfWeek {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonSegmentsDevice {
        #[doc = "Smart TVs and game consoles."]
        ConnectedTv,
        #[doc = "Computers."]
        Desktop,
        #[doc = "Mobile devices with full browsers."]
        Mobile,
        #[doc = "Other device types."]
        Other,
        #[doc = "Tablets with full browsers."]
        Tablet,
        #[doc = "The value is unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0CommonSegmentsDevice {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0CommonSegmentsDevice::ConnectedTv => "CONNECTED_TV",
                GoogleAdsSearchads360V0CommonSegmentsDevice::Desktop => "DESKTOP",
                GoogleAdsSearchads360V0CommonSegmentsDevice::Mobile => "MOBILE",
                GoogleAdsSearchads360V0CommonSegmentsDevice::Other => "OTHER",
                GoogleAdsSearchads360V0CommonSegmentsDevice::Tablet => "TABLET",
                GoogleAdsSearchads360V0CommonSegmentsDevice::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0CommonSegmentsDevice::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0CommonSegmentsDevice {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonSegmentsDevice {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0CommonSegmentsDevice, ()> {
            Ok(match s {
                "CONNECTED_TV" => GoogleAdsSearchads360V0CommonSegmentsDevice::ConnectedTv,
                "DESKTOP" => GoogleAdsSearchads360V0CommonSegmentsDevice::Desktop,
                "MOBILE" => GoogleAdsSearchads360V0CommonSegmentsDevice::Mobile,
                "OTHER" => GoogleAdsSearchads360V0CommonSegmentsDevice::Other,
                "TABLET" => GoogleAdsSearchads360V0CommonSegmentsDevice::Tablet,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonSegmentsDevice::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0CommonSegmentsDevice::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonSegmentsDevice {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonSegmentsDevice {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0CommonSegmentsDevice {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONNECTED_TV" => GoogleAdsSearchads360V0CommonSegmentsDevice::ConnectedTv,
                "DESKTOP" => GoogleAdsSearchads360V0CommonSegmentsDevice::Desktop,
                "MOBILE" => GoogleAdsSearchads360V0CommonSegmentsDevice::Mobile,
                "OTHER" => GoogleAdsSearchads360V0CommonSegmentsDevice::Other,
                "TABLET" => GoogleAdsSearchads360V0CommonSegmentsDevice::Tablet,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonSegmentsDevice::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0CommonSegmentsDevice::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonSegmentsDevice {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonSegmentsDevice {
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
    pub struct GoogleAdsSearchads360V0CommonTargetCpa {
        #[doc = "Maximum bid limit that can be set by the bid strategy. The limit applies to all keywords managed by the strategy. This should only be set for portfolio bid strategies."]
        #[serde(
            rename = "cpcBidCeilingMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cpc_bid_ceiling_micros: ::std::option::Option<i64>,
        #[doc = "Minimum bid limit that can be set by the bid strategy. The limit applies to all keywords managed by the strategy. This should only be set for portfolio bid strategies."]
        #[serde(
            rename = "cpcBidFloorMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cpc_bid_floor_micros: ::std::option::Option<i64>,
        #[doc = "Average CPA target. This target should be greater than or equal to minimum billable unit based on the currency for the account."]
        #[serde(
            rename = "targetCpaMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub target_cpa_micros: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonTargetCpa {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonTargetCpa {
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
    pub struct GoogleAdsSearchads360V0CommonTargetCpm {}
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonTargetCpm {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonTargetCpm {
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
    pub struct GoogleAdsSearchads360V0CommonTargetImpressionShare {
        #[doc = "The highest CPC bid the automated bidding system is permitted to specify. This is a required field entered by the advertiser that sets the ceiling and specified in local micros."]
        #[serde(
            rename = "cpcBidCeilingMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cpc_bid_ceiling_micros: ::std::option::Option<i64>,
        #[doc = "The targeted location on the search results page."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0CommonTargetImpressionShareLocation,
        >,
        #[doc = "The chosen fraction of ads to be shown in the targeted location in micros. For example, 1% equals 10,000."]
        #[serde(
            rename = "locationFractionMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub location_fraction_micros: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonTargetImpressionShare {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonTargetImpressionShare {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonTargetImpressionShareLocation {
        #[doc = "Top slot in the top box of ads."]
        AbsoluteTopOfPage,
        #[doc = "Any location on the web page."]
        AnywhereOnPage,
        #[doc = "Top box of ads."]
        TopOfPage,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0CommonTargetImpressionShareLocation {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0CommonTargetImpressionShareLocation::AbsoluteTopOfPage => {
                    "ABSOLUTE_TOP_OF_PAGE"
                }
                GoogleAdsSearchads360V0CommonTargetImpressionShareLocation::AnywhereOnPage => {
                    "ANYWHERE_ON_PAGE"
                }
                GoogleAdsSearchads360V0CommonTargetImpressionShareLocation::TopOfPage => {
                    "TOP_OF_PAGE"
                }
                GoogleAdsSearchads360V0CommonTargetImpressionShareLocation::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0CommonTargetImpressionShareLocation::Unspecified => {
                    "UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0CommonTargetImpressionShareLocation {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonTargetImpressionShareLocation {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0CommonTargetImpressionShareLocation, ()>
        {
            Ok(match s {
                "ABSOLUTE_TOP_OF_PAGE" => {
                    GoogleAdsSearchads360V0CommonTargetImpressionShareLocation::AbsoluteTopOfPage
                }
                "ANYWHERE_ON_PAGE" => {
                    GoogleAdsSearchads360V0CommonTargetImpressionShareLocation::AnywhereOnPage
                }
                "TOP_OF_PAGE" => {
                    GoogleAdsSearchads360V0CommonTargetImpressionShareLocation::TopOfPage
                }
                "UNKNOWN" => GoogleAdsSearchads360V0CommonTargetImpressionShareLocation::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonTargetImpressionShareLocation::Unspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonTargetImpressionShareLocation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonTargetImpressionShareLocation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0CommonTargetImpressionShareLocation {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ABSOLUTE_TOP_OF_PAGE" => {
                    GoogleAdsSearchads360V0CommonTargetImpressionShareLocation::AbsoluteTopOfPage
                }
                "ANYWHERE_ON_PAGE" => {
                    GoogleAdsSearchads360V0CommonTargetImpressionShareLocation::AnywhereOnPage
                }
                "TOP_OF_PAGE" => {
                    GoogleAdsSearchads360V0CommonTargetImpressionShareLocation::TopOfPage
                }
                "UNKNOWN" => GoogleAdsSearchads360V0CommonTargetImpressionShareLocation::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonTargetImpressionShareLocation::Unspecified
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
        for GoogleAdsSearchads360V0CommonTargetImpressionShareLocation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0CommonTargetImpressionShareLocation
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
    pub struct GoogleAdsSearchads360V0CommonTargetOutrankShare {
        #[doc = "Maximum bid limit that can be set by the bid strategy. The limit applies to all keywords managed by the strategy."]
        #[serde(
            rename = "cpcBidCeilingMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cpc_bid_ceiling_micros: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonTargetOutrankShare {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonTargetOutrankShare {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0CommonTargetRoas {
        #[doc = "Maximum bid limit that can be set by the bid strategy. The limit applies to all keywords managed by the strategy. This should only be set for portfolio bid strategies."]
        #[serde(
            rename = "cpcBidCeilingMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cpc_bid_ceiling_micros: ::std::option::Option<i64>,
        #[doc = "Minimum bid limit that can be set by the bid strategy. The limit applies to all keywords managed by the strategy. This should only be set for portfolio bid strategies."]
        #[serde(
            rename = "cpcBidFloorMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cpc_bid_floor_micros: ::std::option::Option<i64>,
        #[doc = "Required. The chosen revenue (based on conversion data) per unit of spend. Value must be between 0.01 and 1000.0, inclusive."]
        #[serde(
            rename = "targetRoas",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_roas: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonTargetRoas {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonTargetRoas {
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
    pub struct GoogleAdsSearchads360V0CommonTargetSpend {
        #[doc = "Maximum bid limit that can be set by the bid strategy. The limit applies to all keywords managed by the strategy."]
        #[serde(
            rename = "cpcBidCeilingMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cpc_bid_ceiling_micros: ::std::option::Option<i64>,
        #[doc = "The spend target under which to maximize clicks. A TargetSpend bidder will attempt to spend the smaller of this value or the natural throttling spend amount. If not specified, the budget is used as the spend target. This field is deprecated and should no longer be used. See https://ads-developers.googleblog.com/2020/05/reminder-about-sunset-creation-of.html for details."]
        #[serde(
            rename = "targetSpendMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub target_spend_micros: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonTargetSpend {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonTargetSpend {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0CommonValue {
        #[doc = "A boolean."]
        #[serde(
            rename = "booleanValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boolean_value: ::std::option::Option<bool>,
        #[doc = "A double."]
        #[serde(
            rename = "doubleValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub double_value: ::std::option::Option<f64>,
        #[doc = "A float."]
        #[serde(
            rename = "floatValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub float_value: ::std::option::Option<f32>,
        #[doc = "An int64."]
        #[serde(
            rename = "int64Value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub int_64_value: ::std::option::Option<i64>,
        #[doc = "A string."]
        #[serde(
            rename = "stringValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonValue {
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
    pub struct GoogleAdsSearchads360V0CommonWebpageConditionInfo {
        #[doc = "Argument of webpage targeting condition."]
        #[serde(
            rename = "argument",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub argument: ::std::option::Option<String>,
        #[doc = "Operand of webpage targeting condition."]
        #[serde(
            rename = "operand",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operand: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand,
        >,
        #[doc = "Operator of webpage targeting condition."]
        #[serde(
            rename = "operator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonWebpageConditionInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonWebpageConditionInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand {
        #[doc = "Operand denoting a webpage category targeting condition."]
        Category,
        #[doc = "Operand denoting a webpage custom label targeting condition."]
        CustomLabel,
        #[doc = "Operand denoting a webpage content targeting condition."]
        PageContent,
        #[doc = "Operand denoting a webpage title targeting condition."]
        PageTitle,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
        #[doc = "Operand denoting a webpage URL targeting condition."]
        Url,
    }
    impl GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::Category => "CATEGORY",
                GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::CustomLabel => {
                    "CUSTOM_LABEL"
                }
                GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::PageContent => {
                    "PAGE_CONTENT"
                }
                GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::PageTitle => "PAGE_TITLE",
                GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::Unspecified => {
                    "UNSPECIFIED"
                }
                GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::Url => "URL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand, ()>
        {
            Ok(match s {
                "CATEGORY" => GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::Category,
                "CUSTOM_LABEL" => {
                    GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::CustomLabel
                }
                "PAGE_CONTENT" => {
                    GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::PageContent
                }
                "PAGE_TITLE" => GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::PageTitle,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::Unspecified
                }
                "URL" => GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::Url,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CATEGORY" => GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::Category,
                "CUSTOM_LABEL" => {
                    GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::CustomLabel
                }
                "PAGE_CONTENT" => {
                    GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::PageContent
                }
                "PAGE_TITLE" => GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::PageTitle,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::Unspecified
                }
                "URL" => GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand::Url,
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
        for GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0CommonWebpageConditionInfoOperand
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator {
        #[doc = "The argument web condition is part of the compared web condition."]
        Contains,
        #[doc = "The argument web condition is equal to the compared web condition."]
        Equals,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator::Contains => "CONTAINS",
                GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator::Equals => "EQUALS",
                GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator::Unspecified => {
                    "UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator, ()>
        {
            Ok(match s {
                "CONTAINS" => GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator::Contains,
                "EQUALS" => GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator::Equals,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator::Unspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTAINS" => GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator::Contains,
                "EQUALS" => GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator::Equals,
                "UNKNOWN" => GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator::Unspecified
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
        for GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0CommonWebpageConditionInfoOperator
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0CommonWebpageInfo {
        #[doc = "Conditions, or logical expressions, for webpage targeting. The list of webpage targeting conditions are and-ed together when evaluated for targeting. An empty list of conditions indicates all pages of the campaign’s website are targeted. This field is required for CREATE operations and is prohibited on UPDATE operations."]
        #[serde(
            rename = "conditions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conditions: ::std::option::Option<
            Vec<crate::schemas::GoogleAdsSearchads360V0CommonWebpageConditionInfo>,
        >,
        #[doc = "Website criteria coverage percentage. This is the computed percentage of website coverage based on the website target, negative website target and negative keywords in the ad group and campaign. For instance, when coverage returns as 1, it indicates it has 100% coverage. This field is read-only."]
        #[serde(
            rename = "coveragePercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub coverage_percentage: ::std::option::Option<f64>,
        #[doc = "The name of the criterion that is defined by this parameter. The name value will be used for identifying, sorting and filtering criteria with this type of parameters. This field is required for CREATE operations and is prohibited on UPDATE operations."]
        #[serde(
            rename = "criterionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub criterion_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0CommonWebpageInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0CommonWebpageInfo {
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
    pub struct GoogleAdsSearchads360V0ErrorsErrorCode {
        #[doc = "Indicates failure to properly authenticate user."]
        #[serde(
            rename = "authenticationError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authentication_error: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError,
        >,
        #[doc = "An error encountered when trying to authorize a user."]
        #[serde(
            rename = "authorizationError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub authorization_error: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError,
        >,
        #[doc = "The reasons for the date error"]
        #[serde(
            rename = "dateError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_error:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ErrorsErrorCodeDateError>,
        #[doc = "The reasons for the date range error"]
        #[serde(
            rename = "dateRangeError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_range_error: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError,
        >,
        #[doc = "The reasons for the distinct error"]
        #[serde(
            rename = "distinctError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub distinct_error: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError,
        >,
        #[doc = "The reasons for the header error."]
        #[serde(
            rename = "headerError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header_error: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError,
        >,
        #[doc = "An unexpected server-side error."]
        #[serde(
            rename = "internalError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub internal_error: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ErrorsErrorCodeInternalError,
        >,
        #[doc = "An error with the query"]
        #[serde(
            rename = "queryError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query_error:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ErrorsErrorCodeQueryError>,
        #[doc = "An error with the amonut of quota remaining."]
        #[serde(
            rename = "quotaError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quota_error:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError>,
        #[doc = "An error caused by the request"]
        #[serde(
            rename = "requestError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_error: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ErrorsErrorCodeRequestError,
        >,
        #[doc = "The reasons for the size limit error"]
        #[serde(
            rename = "sizeLimitError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size_limit_error: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ErrorsErrorCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ErrorsErrorCode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError {
        #[doc = "An account administrator changed this account’s authentication settings. To access this account, enable Advanced Protection in your Google account at https://landing.google.com/advancedprotection."]
        AdvancedProtectionNotEnrolled,
        #[doc = "Authentication of the request failed."]
        AuthenticationError,
        #[doc = "Client customer ID is not a number."]
        ClientCustomerIdInvalid,
        #[doc = "No customer found for the provided customer ID."]
        CustomerNotFound,
        #[doc = "A problem occurred during Google account authentication."]
        GoogleAccountAuthenticationFailed,
        #[doc = "Account login token in the cookie is invalid."]
        GoogleAccountCookieInvalid,
        #[doc = "Client’s Google account is deleted."]
        GoogleAccountDeleted,
        #[doc = "The user in the Google account login token does not match the user ID in the cookie."]
        GoogleAccountUserAndAdsUserMismatch,
        #[doc = "Login cookie is not valid."]
        LoginCookieInvalid,
        #[doc = "Login cookie is required for authentication."]
        LoginCookieRequired,
        #[doc = "User in the cookie is not a valid Ads user."]
        NotAdsUser,
        #[doc = "OAuth token in the header has been disabled."]
        OauthTokenDisabled,
        #[doc = "OAuth token in the header has expired."]
        OauthTokenExpired,
        #[doc = "OAuth token HTTP header is malformed."]
        OauthTokenHeaderInvalid,
        #[doc = "OAuth token in the header is not valid."]
        OauthTokenInvalid,
        #[doc = "OAuth token in the header has been revoked."]
        OauthTokenRevoked,
        #[doc = "An account administrator changed this account’s authentication settings. To access this account, enable 2-Step Verification in your Google account at https://www.google.com/landing/2step."]
        TwoStepVerificationNotEnrolled,
        #[doc = "The received error code is not known in this version."]
        Unknown,
        #[doc = "Enum unspecified."]
        Unspecified,
        #[doc = "User ID in the header is not a valid ID."]
        UserIdInvalid,
    }
    impl GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: AdvancedProtectionNotEnrolled => "ADVANCED_PROTECTION_NOT_ENROLLED" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: AuthenticationError => "AUTHENTICATION_ERROR" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: ClientCustomerIdInvalid => "CLIENT_CUSTOMER_ID_INVALID" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: CustomerNotFound => "CUSTOMER_NOT_FOUND" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: GoogleAccountAuthenticationFailed => "GOOGLE_ACCOUNT_AUTHENTICATION_FAILED" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: GoogleAccountCookieInvalid => "GOOGLE_ACCOUNT_COOKIE_INVALID" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: GoogleAccountDeleted => "GOOGLE_ACCOUNT_DELETED" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: GoogleAccountUserAndAdsUserMismatch => "GOOGLE_ACCOUNT_USER_AND_ADS_USER_MISMATCH" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: LoginCookieInvalid => "LOGIN_COOKIE_INVALID" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: LoginCookieRequired => "LOGIN_COOKIE_REQUIRED" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: NotAdsUser => "NOT_ADS_USER" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: OauthTokenDisabled => "OAUTH_TOKEN_DISABLED" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: OauthTokenExpired => "OAUTH_TOKEN_EXPIRED" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: OauthTokenHeaderInvalid => "OAUTH_TOKEN_HEADER_INVALID" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: OauthTokenInvalid => "OAUTH_TOKEN_INVALID" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: OauthTokenRevoked => "OAUTH_TOKEN_REVOKED" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: TwoStepVerificationNotEnrolled => "TWO_STEP_VERIFICATION_NOT_ENROLLED" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: Unspecified => "UNSPECIFIED" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: UserIdInvalid => "USER_ID_INVALID" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError, ()>
        {
            Ok (match s { "ADVANCED_PROTECTION_NOT_ENROLLED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: AdvancedProtectionNotEnrolled , "AUTHENTICATION_ERROR" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: AuthenticationError , "CLIENT_CUSTOMER_ID_INVALID" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: ClientCustomerIdInvalid , "CUSTOMER_NOT_FOUND" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: CustomerNotFound , "GOOGLE_ACCOUNT_AUTHENTICATION_FAILED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: GoogleAccountAuthenticationFailed , "GOOGLE_ACCOUNT_COOKIE_INVALID" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: GoogleAccountCookieInvalid , "GOOGLE_ACCOUNT_DELETED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: GoogleAccountDeleted , "GOOGLE_ACCOUNT_USER_AND_ADS_USER_MISMATCH" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: GoogleAccountUserAndAdsUserMismatch , "LOGIN_COOKIE_INVALID" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: LoginCookieInvalid , "LOGIN_COOKIE_REQUIRED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: LoginCookieRequired , "NOT_ADS_USER" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: NotAdsUser , "OAUTH_TOKEN_DISABLED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: OauthTokenDisabled , "OAUTH_TOKEN_EXPIRED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: OauthTokenExpired , "OAUTH_TOKEN_HEADER_INVALID" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: OauthTokenHeaderInvalid , "OAUTH_TOKEN_INVALID" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: OauthTokenInvalid , "OAUTH_TOKEN_REVOKED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: OauthTokenRevoked , "TWO_STEP_VERIFICATION_NOT_ENROLLED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: TwoStepVerificationNotEnrolled , "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: Unspecified , "USER_ID_INVALID" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: UserIdInvalid , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ADVANCED_PROTECTION_NOT_ENROLLED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: AdvancedProtectionNotEnrolled , "AUTHENTICATION_ERROR" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: AuthenticationError , "CLIENT_CUSTOMER_ID_INVALID" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: ClientCustomerIdInvalid , "CUSTOMER_NOT_FOUND" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: CustomerNotFound , "GOOGLE_ACCOUNT_AUTHENTICATION_FAILED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: GoogleAccountAuthenticationFailed , "GOOGLE_ACCOUNT_COOKIE_INVALID" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: GoogleAccountCookieInvalid , "GOOGLE_ACCOUNT_DELETED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: GoogleAccountDeleted , "GOOGLE_ACCOUNT_USER_AND_ADS_USER_MISMATCH" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: GoogleAccountUserAndAdsUserMismatch , "LOGIN_COOKIE_INVALID" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: LoginCookieInvalid , "LOGIN_COOKIE_REQUIRED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: LoginCookieRequired , "NOT_ADS_USER" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: NotAdsUser , "OAUTH_TOKEN_DISABLED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: OauthTokenDisabled , "OAUTH_TOKEN_EXPIRED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: OauthTokenExpired , "OAUTH_TOKEN_HEADER_INVALID" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: OauthTokenHeaderInvalid , "OAUTH_TOKEN_INVALID" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: OauthTokenInvalid , "OAUTH_TOKEN_REVOKED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: OauthTokenRevoked , "TWO_STEP_VERIFICATION_NOT_ENROLLED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: TwoStepVerificationNotEnrolled , "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: Unspecified , "USER_ID_INVALID" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError :: UserIdInvalid , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ErrorsErrorCodeAuthenticationError
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError {
        #[doc = "The customer (or login customer) isn’t allowed in Search Ads 360 API. It belongs to another ads system."]
        AccessDeniedForAccountType,
        #[doc = "The user does not have permission to perform this action (for example, ADD, UPDATE, REMOVE) on the resource or call a method."]
        ActionNotPermitted,
        #[doc = "Authorization of the client failed."]
        AuthorizationError,
        #[doc = "The customer account can’t be accessed because it is not yet enabled or has been deactivated."]
        CustomerNotEnabled,
        #[doc = "Signup not complete."]
        IncompleteSignup,
        #[doc = "The login customer specified does not have access to the account specified, so the request is invalid."]
        InvalidLoginCustomerIdServingCustomerIdCombination,
        #[doc = "The developer does not have access to the metrics queried."]
        MetricAccessDenied,
        #[doc = "The developer must sign the terms of service. They can be found here: https://developers.google.com/terms"]
        MissingTos,
        #[doc = "The Google Cloud project sent in the request does not have permission to access the api."]
        ProjectDisabled,
        #[doc = "The developer specified does not have access to the service."]
        ServiceAccessDenied,
        #[doc = "The received error code is not known in this version."]
        Unknown,
        #[doc = "Enum unspecified."]
        Unspecified,
        #[doc = "User doesn’t have permission to access customer. Note: If you’re accessing a client customer, the manager’s customer ID must be set in the `login-customer-id` header. Learn more at https://developers.google.com/search-ads/reporting/concepts/call-structure#login_customer_id_header"]
        UserPermissionDenied,
    }
    impl GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: AccessDeniedForAccountType => "ACCESS_DENIED_FOR_ACCOUNT_TYPE" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: ActionNotPermitted => "ACTION_NOT_PERMITTED" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: AuthorizationError => "AUTHORIZATION_ERROR" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: CustomerNotEnabled => "CUSTOMER_NOT_ENABLED" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: IncompleteSignup => "INCOMPLETE_SIGNUP" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: InvalidLoginCustomerIdServingCustomerIdCombination => "INVALID_LOGIN_CUSTOMER_ID_SERVING_CUSTOMER_ID_COMBINATION" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: MetricAccessDenied => "METRIC_ACCESS_DENIED" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: MissingTos => "MISSING_TOS" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: ProjectDisabled => "PROJECT_DISABLED" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: ServiceAccessDenied => "SERVICE_ACCESS_DENIED" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: Unspecified => "UNSPECIFIED" , GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: UserPermissionDenied => "USER_PERMISSION_DENIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError, ()>
        {
            Ok (match s { "ACCESS_DENIED_FOR_ACCOUNT_TYPE" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: AccessDeniedForAccountType , "ACTION_NOT_PERMITTED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: ActionNotPermitted , "AUTHORIZATION_ERROR" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: AuthorizationError , "CUSTOMER_NOT_ENABLED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: CustomerNotEnabled , "INCOMPLETE_SIGNUP" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: IncompleteSignup , "INVALID_LOGIN_CUSTOMER_ID_SERVING_CUSTOMER_ID_COMBINATION" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: InvalidLoginCustomerIdServingCustomerIdCombination , "METRIC_ACCESS_DENIED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: MetricAccessDenied , "MISSING_TOS" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: MissingTos , "PROJECT_DISABLED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: ProjectDisabled , "SERVICE_ACCESS_DENIED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: ServiceAccessDenied , "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: Unspecified , "USER_PERMISSION_DENIED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: UserPermissionDenied , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ACCESS_DENIED_FOR_ACCOUNT_TYPE" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: AccessDeniedForAccountType , "ACTION_NOT_PERMITTED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: ActionNotPermitted , "AUTHORIZATION_ERROR" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: AuthorizationError , "CUSTOMER_NOT_ENABLED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: CustomerNotEnabled , "INCOMPLETE_SIGNUP" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: IncompleteSignup , "INVALID_LOGIN_CUSTOMER_ID_SERVING_CUSTOMER_ID_COMBINATION" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: InvalidLoginCustomerIdServingCustomerIdCombination , "METRIC_ACCESS_DENIED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: MetricAccessDenied , "MISSING_TOS" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: MissingTos , "PROJECT_DISABLED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: ProjectDisabled , "SERVICE_ACCESS_DENIED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: ServiceAccessDenied , "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: Unspecified , "USER_PERMISSION_DENIED" => GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError :: UserPermissionDenied , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ErrorsErrorCodeAuthorizationError
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ErrorsErrorCodeDateError {
        #[doc = "Both dates in range are null."]
        DateRangeMinimumAndMaximumDatesBothNull,
        #[doc = "Date range bounds are not in order."]
        DateRangeMinimumDateLaterThanMaximumDate,
        #[doc = "Date is before allowed minimum."]
        EarlierThanMinimumDate,
        #[doc = "Given field values do not correspond to a valid date."]
        InvalidFieldValuesInDate,
        #[doc = "Given field values do not correspond to a valid date time."]
        InvalidFieldValuesInDateTime,
        #[doc = "The string date’s format should be yyyy-mm-dd."]
        InvalidStringDate,
        #[doc = "The string date time’s format should be yyyy-mm-dd hh:mm:ss.ssssss."]
        InvalidStringDateTimeMicros,
        #[doc = "The string date time’s format should be yyyy-mm-dd hh:mm:ss."]
        InvalidStringDateTimeSeconds,
        #[doc = "The string date time’s format should be yyyy-mm-dd hh:mm:ss+|-hh:mm."]
        InvalidStringDateTimeSecondsWithOffset,
        #[doc = "Date is after allowed maximum."]
        LaterThanMaximumDate,
        #[doc = "The received error code is not known in this version."]
        Unknown,
        #[doc = "Enum unspecified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ErrorsErrorCodeDateError {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: DateRangeMinimumAndMaximumDatesBothNull => "DATE_RANGE_MINIMUM_AND_MAXIMUM_DATES_BOTH_NULL" , GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: DateRangeMinimumDateLaterThanMaximumDate => "DATE_RANGE_MINIMUM_DATE_LATER_THAN_MAXIMUM_DATE" , GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: EarlierThanMinimumDate => "EARLIER_THAN_MINIMUM_DATE" , GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidFieldValuesInDate => "INVALID_FIELD_VALUES_IN_DATE" , GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidFieldValuesInDateTime => "INVALID_FIELD_VALUES_IN_DATE_TIME" , GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidStringDate => "INVALID_STRING_DATE" , GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidStringDateTimeMicros => "INVALID_STRING_DATE_TIME_MICROS" , GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidStringDateTimeSeconds => "INVALID_STRING_DATE_TIME_SECONDS" , GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidStringDateTimeSecondsWithOffset => "INVALID_STRING_DATE_TIME_SECONDS_WITH_OFFSET" , GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: LaterThanMaximumDate => "LATER_THAN_MAXIMUM_DATE" , GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ErrorsErrorCodeDateError {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ErrorsErrorCodeDateError {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ErrorsErrorCodeDateError, ()> {
            Ok (match s { "DATE_RANGE_MINIMUM_AND_MAXIMUM_DATES_BOTH_NULL" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: DateRangeMinimumAndMaximumDatesBothNull , "DATE_RANGE_MINIMUM_DATE_LATER_THAN_MAXIMUM_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: DateRangeMinimumDateLaterThanMaximumDate , "EARLIER_THAN_MINIMUM_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: EarlierThanMinimumDate , "INVALID_FIELD_VALUES_IN_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidFieldValuesInDate , "INVALID_FIELD_VALUES_IN_DATE_TIME" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidFieldValuesInDateTime , "INVALID_STRING_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidStringDate , "INVALID_STRING_DATE_TIME_MICROS" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidStringDateTimeMicros , "INVALID_STRING_DATE_TIME_SECONDS" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidStringDateTimeSeconds , "INVALID_STRING_DATE_TIME_SECONDS_WITH_OFFSET" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidStringDateTimeSecondsWithOffset , "LATER_THAN_MAXIMUM_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: LaterThanMaximumDate , "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: Unspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ErrorsErrorCodeDateError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ErrorsErrorCodeDateError {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ErrorsErrorCodeDateError {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "DATE_RANGE_MINIMUM_AND_MAXIMUM_DATES_BOTH_NULL" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: DateRangeMinimumAndMaximumDatesBothNull , "DATE_RANGE_MINIMUM_DATE_LATER_THAN_MAXIMUM_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: DateRangeMinimumDateLaterThanMaximumDate , "EARLIER_THAN_MINIMUM_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: EarlierThanMinimumDate , "INVALID_FIELD_VALUES_IN_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidFieldValuesInDate , "INVALID_FIELD_VALUES_IN_DATE_TIME" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidFieldValuesInDateTime , "INVALID_STRING_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidStringDate , "INVALID_STRING_DATE_TIME_MICROS" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidStringDateTimeMicros , "INVALID_STRING_DATE_TIME_SECONDS" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidStringDateTimeSeconds , "INVALID_STRING_DATE_TIME_SECONDS_WITH_OFFSET" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: InvalidStringDateTimeSecondsWithOffset , "LATER_THAN_MAXIMUM_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: LaterThanMaximumDate , "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeDateError :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ErrorsErrorCodeDateError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ErrorsErrorCodeDateError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError {
        #[doc = "A date was used that is past the system “last” date."]
        AfterMaximumAllowableDate,
        #[doc = "Trying to change start date on a resource that has started."]
        CannotModifyStartDateIfAlreadyStarted,
        #[doc = "Cannot set date to past time"]
        CannotSetDateToPast,
        #[doc = "Invalid date."]
        InvalidDate,
        #[doc = "The start date was after the end date."]
        StartDateAfterEndDate,
        #[doc = "The received error code is not known in this version."]
        Unknown,
        #[doc = "Enum unspecified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: AfterMaximumAllowableDate => "AFTER_MAXIMUM_ALLOWABLE_DATE" , GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: CannotModifyStartDateIfAlreadyStarted => "CANNOT_MODIFY_START_DATE_IF_ALREADY_STARTED" , GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: CannotSetDateToPast => "CANNOT_SET_DATE_TO_PAST" , GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: InvalidDate => "INVALID_DATE" , GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: StartDateAfterEndDate => "START_DATE_AFTER_END_DATE" , GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError, ()>
        {
            Ok (match s { "AFTER_MAXIMUM_ALLOWABLE_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: AfterMaximumAllowableDate , "CANNOT_MODIFY_START_DATE_IF_ALREADY_STARTED" => GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: CannotModifyStartDateIfAlreadyStarted , "CANNOT_SET_DATE_TO_PAST" => GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: CannotSetDateToPast , "INVALID_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: InvalidDate , "START_DATE_AFTER_END_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: StartDateAfterEndDate , "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: Unspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "AFTER_MAXIMUM_ALLOWABLE_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: AfterMaximumAllowableDate , "CANNOT_MODIFY_START_DATE_IF_ALREADY_STARTED" => GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: CannotModifyStartDateIfAlreadyStarted , "CANNOT_SET_DATE_TO_PAST" => GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: CannotSetDateToPast , "INVALID_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: InvalidDate , "START_DATE_AFTER_END_DATE" => GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: StartDateAfterEndDate , "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ErrorsErrorCodeDateRangeError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError {
        #[doc = "Duplicate element."]
        DuplicateElement,
        #[doc = "Duplicate type."]
        DuplicateType,
        #[doc = "The received error code is not known in this version."]
        Unknown,
        #[doc = "Enum unspecified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError::DuplicateElement => {
                    "DUPLICATE_ELEMENT"
                }
                GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError::DuplicateType => {
                    "DUPLICATE_TYPE"
                }
                GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError, ()>
        {
            Ok(match s {
                "DUPLICATE_ELEMENT" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError::DuplicateElement
                }
                "DUPLICATE_TYPE" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError::DuplicateType
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DUPLICATE_ELEMENT" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError::DuplicateElement
                }
                "DUPLICATE_TYPE" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError::DuplicateType
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError::Unspecified,
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
        for GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ErrorsErrorCodeDistinctError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError {
        #[doc = "The login customer ID could not be validated."]
        InvalidLoginCustomerId,
        #[doc = "The user selected customer ID could not be validated."]
        InvalidUserSelectedCustomerId,
        #[doc = "The received error code is not known in this version."]
        Unknown,
        #[doc = "Enum unspecified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError :: InvalidLoginCustomerId => "INVALID_LOGIN_CUSTOMER_ID" , GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError :: InvalidUserSelectedCustomerId => "INVALID_USER_SELECTED_CUSTOMER_ID" , GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError, ()> {
            Ok(match s {
                "INVALID_LOGIN_CUSTOMER_ID" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError::InvalidLoginCustomerId
                }
                "INVALID_USER_SELECTED_CUSTOMER_ID" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError::InvalidUserSelectedCustomerId
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INVALID_LOGIN_CUSTOMER_ID" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError::InvalidLoginCustomerId
                }
                "INVALID_USER_SELECTED_CUSTOMER_ID" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError::InvalidUserSelectedCustomerId
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ErrorsErrorCodeHeaderError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ErrorsErrorCodeInternalError {
        #[doc = "The request took longer than a deadline."]
        DeadlineExceeded,
        #[doc = "The intended error code doesn’t exist in specified API version. It will be released in a future API version."]
        ErrorCodeNotPublished,
        #[doc = "API encountered unexpected internal error."]
        InternalError,
        #[doc = "API encountered an unexpected transient error. The user should retry their request in these cases."]
        TransientError,
        #[doc = "The received error code is not known in this version."]
        Unknown,
        #[doc = "Enum unspecified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ErrorsErrorCodeInternalError {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::DeadlineExceeded => {
                    "DEADLINE_EXCEEDED"
                }
                GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::ErrorCodeNotPublished => {
                    "ERROR_CODE_NOT_PUBLISHED"
                }
                GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::InternalError => {
                    "INTERNAL_ERROR"
                }
                GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::TransientError => {
                    "TRANSIENT_ERROR"
                }
                GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ErrorsErrorCodeInternalError {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ErrorsErrorCodeInternalError {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ErrorsErrorCodeInternalError, ()>
        {
            Ok(match s {
                "DEADLINE_EXCEEDED" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::DeadlineExceeded
                }
                "ERROR_CODE_NOT_PUBLISHED" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::ErrorCodeNotPublished
                }
                "INTERNAL_ERROR" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::InternalError
                }
                "TRANSIENT_ERROR" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::TransientError
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ErrorsErrorCodeInternalError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ErrorsErrorCodeInternalError {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ErrorsErrorCodeInternalError {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEADLINE_EXCEEDED" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::DeadlineExceeded
                }
                "ERROR_CODE_NOT_PUBLISHED" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::ErrorCodeNotPublished
                }
                "INTERNAL_ERROR" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::InternalError
                }
                "TRANSIENT_ERROR" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::TransientError
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeInternalError::Unspecified,
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
        for GoogleAdsSearchads360V0ErrorsErrorCodeInternalError
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ErrorsErrorCodeInternalError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ErrorsErrorCodeQueryError {
        #[doc = "A condition used in the query references an invalid enum constant."]
        BadEnumConstant,
        #[doc = "Query contains an invalid escape sequence."]
        BadEscapeSequence,
        #[doc = "Field name is invalid."]
        BadFieldName,
        #[doc = "Limit value is invalid (for example, not a number)"]
        BadLimitValue,
        #[doc = "Encountered number can not be parsed."]
        BadNumber,
        #[doc = "Invalid operator encountered."]
        BadOperator,
        #[doc = "Parameter unknown or not supported."]
        BadParameterName,
        #[doc = "Parameter have invalid value."]
        BadParameterValue,
        #[doc = "Invalid resource type was specified in the FROM clause."]
        BadResourceTypeInFromClause,
        #[doc = "Non-ASCII symbol encountered outside of strings."]
        BadSymbol,
        #[doc = "Value is invalid."]
        BadValue,
        #[doc = "Filters on date/week/month/quarter have a start date after end date."]
        DateRangeTooNarrow,
        #[doc = "Date filters fail to restrict date to a range smaller than 31 days. Applicable if the query is segmented by date."]
        DateRangeTooWide,
        #[doc = "Expected AND between values with BETWEEN operator."]
        ExpectedAnd,
        #[doc = "Expecting ORDER BY to have BY."]
        ExpectedBy,
        #[doc = "There was no dimension field selected."]
        ExpectedDimensionFieldInSelectClause,
        #[doc = "Missing filters on date related fields."]
        ExpectedFiltersOnDateRange,
        #[doc = "Missing FROM clause."]
        ExpectedFrom,
        #[doc = "The operator used in the conditions requires the value to be a list."]
        ExpectedList,
        #[doc = "Fields used in WHERE or ORDER BY clauses are missing from the SELECT clause."]
        ExpectedReferencedFieldInSelectClause,
        #[doc = "SELECT is missing at the beginning of query."]
        ExpectedSelect,
        #[doc = "A list was passed as a value to a condition whose operator expects a single value."]
        ExpectedSingleValue,
        #[doc = "Missing one or both values with BETWEEN operator."]
        ExpectedValueWithBetweenOperator,
        #[doc = "The number of values (right-hand-side operands) in a filter exceeds the limit."]
        FilterHasTooManyValues,
        #[doc = "Invalid date format. Expected ‘YYYY-MM-DD’."]
        InvalidDateFormat,
        #[doc = "Value passed was not a string when it should have been. For example, it was a number or unquoted literal."]
        InvalidStringValue,
        #[doc = "A String value passed to the BETWEEN operator does not parse as a date."]
        InvalidValueWithBetweenOperator,
        #[doc = "The value passed to the DURING operator is not a Date range literal"]
        InvalidValueWithDuringOperator,
        #[doc = "A value was passed to the LIKE operator."]
        InvalidValueWithLikeOperator,
        #[doc = "The value passed to the limit clause is too low."]
        LimitValueTooLow,
        #[doc = "Misaligned date value for the filter. The date should be the start of a week/month/quarter if the filtered field is segments.week/segments.month/segments.quarter."]
        MisalignedDateForFilter,
        #[doc = "An operator was provided that is inapplicable to the field being filtered."]
        OperatorFieldMismatch,
        #[doc = "A Condition was found with an empty list."]
        ProhibitedEmptyListInCondition,
        #[doc = "A condition used in the query references an unsupported enum constant."]
        ProhibitedEnumConstant,
        #[doc = "Fields that are not allowed to be selected together were included in the SELECT clause."]
        ProhibitedFieldCombinationInSelectClause,
        #[doc = "A field that is not orderable was included in the ORDER BY clause."]
        ProhibitedFieldInOrderByClause,
        #[doc = "A field that is not selectable was included in the SELECT clause."]
        ProhibitedFieldInSelectClause,
        #[doc = "A field that is not filterable was included in the WHERE clause."]
        ProhibitedFieldInWhereClause,
        #[doc = "A metric incompatible with the main resource or other selected segmenting resources was included in the SELECT or WHERE clause."]
        ProhibitedMetricInSelectOrWhereClause,
        #[doc = "Query has a string containing a newline character."]
        ProhibitedNewlineInString,
        #[doc = "Resource type specified in the FROM clause is not supported by this service."]
        ProhibitedResourceTypeInFromClause,
        #[doc = "A field that comes from an incompatible resource was included in the SELECT clause."]
        ProhibitedResourceTypeInSelectClause,
        #[doc = "A field that comes from an incompatible resource was included in the WHERE clause."]
        ProhibitedResourceTypeInWhereClause,
        #[doc = "A segment incompatible with the main resource or other selected segmenting resources was included in the SELECT or WHERE clause."]
        ProhibitedSegmentInSelectOrWhereClause,
        #[doc = "A segment in the SELECT clause is incompatible with a metric in the SELECT or WHERE clause."]
        ProhibitedSegmentWithMetricInSelectOrWhereClause,
        #[doc = "List contains values of different types."]
        ProhibitedValueCombinationInList,
        #[doc = "The values passed to the BETWEEN operator are not of the same type."]
        ProhibitedValueCombinationWithBetweenOperator,
        #[doc = "Returned if all other query error reasons are not applicable."]
        QueryError,
        #[doc = "Metrics cannot be requested for a manager account. To retrieve metrics, issue separate requests against each client account under the manager account."]
        RequestedMetricsForManager,
        #[doc = "Query contains unterminated string."]
        StringNotTerminated,
        #[doc = "Too many segments are specified in SELECT clause."]
        TooManySegments,
        #[doc = "Query is incomplete and cannot be parsed."]
        UnexpectedEndOfQuery,
        #[doc = "FROM clause cannot be specified in this query."]
        UnexpectedFromClause,
        #[doc = "Query has an unexpected extra part."]
        UnexpectedInput,
        #[doc = "The received error code is not known in this version."]
        Unknown,
        #[doc = "Query contains one or more unrecognized fields."]
        UnrecognizedField,
        #[doc = "Name unspecified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ErrorsErrorCodeQueryError {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadEnumConstant => "BAD_ENUM_CONSTANT" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadEscapeSequence => "BAD_ESCAPE_SEQUENCE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadFieldName => "BAD_FIELD_NAME" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadLimitValue => "BAD_LIMIT_VALUE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadNumber => "BAD_NUMBER" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadOperator => "BAD_OPERATOR" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadParameterName => "BAD_PARAMETER_NAME" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadParameterValue => "BAD_PARAMETER_VALUE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadResourceTypeInFromClause => "BAD_RESOURCE_TYPE_IN_FROM_CLAUSE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadSymbol => "BAD_SYMBOL" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadValue => "BAD_VALUE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: DateRangeTooNarrow => "DATE_RANGE_TOO_NARROW" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: DateRangeTooWide => "DATE_RANGE_TOO_WIDE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedAnd => "EXPECTED_AND" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedBy => "EXPECTED_BY" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedDimensionFieldInSelectClause => "EXPECTED_DIMENSION_FIELD_IN_SELECT_CLAUSE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedFiltersOnDateRange => "EXPECTED_FILTERS_ON_DATE_RANGE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedFrom => "EXPECTED_FROM" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedList => "EXPECTED_LIST" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedReferencedFieldInSelectClause => "EXPECTED_REFERENCED_FIELD_IN_SELECT_CLAUSE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedSelect => "EXPECTED_SELECT" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedSingleValue => "EXPECTED_SINGLE_VALUE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedValueWithBetweenOperator => "EXPECTED_VALUE_WITH_BETWEEN_OPERATOR" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: FilterHasTooManyValues => "FILTER_HAS_TOO_MANY_VALUES" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: InvalidDateFormat => "INVALID_DATE_FORMAT" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: InvalidStringValue => "INVALID_STRING_VALUE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: InvalidValueWithBetweenOperator => "INVALID_VALUE_WITH_BETWEEN_OPERATOR" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: InvalidValueWithDuringOperator => "INVALID_VALUE_WITH_DURING_OPERATOR" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: InvalidValueWithLikeOperator => "INVALID_VALUE_WITH_LIKE_OPERATOR" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: LimitValueTooLow => "LIMIT_VALUE_TOO_LOW" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: MisalignedDateForFilter => "MISALIGNED_DATE_FOR_FILTER" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: OperatorFieldMismatch => "OPERATOR_FIELD_MISMATCH" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedEmptyListInCondition => "PROHIBITED_EMPTY_LIST_IN_CONDITION" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedEnumConstant => "PROHIBITED_ENUM_CONSTANT" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedFieldCombinationInSelectClause => "PROHIBITED_FIELD_COMBINATION_IN_SELECT_CLAUSE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedFieldInOrderByClause => "PROHIBITED_FIELD_IN_ORDER_BY_CLAUSE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedFieldInSelectClause => "PROHIBITED_FIELD_IN_SELECT_CLAUSE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedFieldInWhereClause => "PROHIBITED_FIELD_IN_WHERE_CLAUSE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedMetricInSelectOrWhereClause => "PROHIBITED_METRIC_IN_SELECT_OR_WHERE_CLAUSE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedNewlineInString => "PROHIBITED_NEWLINE_IN_STRING" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedResourceTypeInFromClause => "PROHIBITED_RESOURCE_TYPE_IN_FROM_CLAUSE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedResourceTypeInSelectClause => "PROHIBITED_RESOURCE_TYPE_IN_SELECT_CLAUSE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedResourceTypeInWhereClause => "PROHIBITED_RESOURCE_TYPE_IN_WHERE_CLAUSE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedSegmentInSelectOrWhereClause => "PROHIBITED_SEGMENT_IN_SELECT_OR_WHERE_CLAUSE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedSegmentWithMetricInSelectOrWhereClause => "PROHIBITED_SEGMENT_WITH_METRIC_IN_SELECT_OR_WHERE_CLAUSE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedValueCombinationInList => "PROHIBITED_VALUE_COMBINATION_IN_LIST" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedValueCombinationWithBetweenOperator => "PROHIBITED_VALUE_COMBINATION_WITH_BETWEEN_OPERATOR" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: QueryError => "QUERY_ERROR" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: RequestedMetricsForManager => "REQUESTED_METRICS_FOR_MANAGER" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: StringNotTerminated => "STRING_NOT_TERMINATED" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: TooManySegments => "TOO_MANY_SEGMENTS" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: UnexpectedEndOfQuery => "UNEXPECTED_END_OF_QUERY" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: UnexpectedFromClause => "UNEXPECTED_FROM_CLAUSE" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: UnexpectedInput => "UNEXPECTED_INPUT" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: UnrecognizedField => "UNRECOGNIZED_FIELD" , GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ErrorsErrorCodeQueryError {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ErrorsErrorCodeQueryError {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ErrorsErrorCodeQueryError, ()> {
            Ok (match s { "BAD_ENUM_CONSTANT" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadEnumConstant , "BAD_ESCAPE_SEQUENCE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadEscapeSequence , "BAD_FIELD_NAME" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadFieldName , "BAD_LIMIT_VALUE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadLimitValue , "BAD_NUMBER" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadNumber , "BAD_OPERATOR" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadOperator , "BAD_PARAMETER_NAME" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadParameterName , "BAD_PARAMETER_VALUE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadParameterValue , "BAD_RESOURCE_TYPE_IN_FROM_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadResourceTypeInFromClause , "BAD_SYMBOL" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadSymbol , "BAD_VALUE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadValue , "DATE_RANGE_TOO_NARROW" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: DateRangeTooNarrow , "DATE_RANGE_TOO_WIDE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: DateRangeTooWide , "EXPECTED_AND" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedAnd , "EXPECTED_BY" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedBy , "EXPECTED_DIMENSION_FIELD_IN_SELECT_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedDimensionFieldInSelectClause , "EXPECTED_FILTERS_ON_DATE_RANGE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedFiltersOnDateRange , "EXPECTED_FROM" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedFrom , "EXPECTED_LIST" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedList , "EXPECTED_REFERENCED_FIELD_IN_SELECT_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedReferencedFieldInSelectClause , "EXPECTED_SELECT" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedSelect , "EXPECTED_SINGLE_VALUE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedSingleValue , "EXPECTED_VALUE_WITH_BETWEEN_OPERATOR" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedValueWithBetweenOperator , "FILTER_HAS_TOO_MANY_VALUES" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: FilterHasTooManyValues , "INVALID_DATE_FORMAT" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: InvalidDateFormat , "INVALID_STRING_VALUE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: InvalidStringValue , "INVALID_VALUE_WITH_BETWEEN_OPERATOR" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: InvalidValueWithBetweenOperator , "INVALID_VALUE_WITH_DURING_OPERATOR" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: InvalidValueWithDuringOperator , "INVALID_VALUE_WITH_LIKE_OPERATOR" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: InvalidValueWithLikeOperator , "LIMIT_VALUE_TOO_LOW" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: LimitValueTooLow , "MISALIGNED_DATE_FOR_FILTER" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: MisalignedDateForFilter , "OPERATOR_FIELD_MISMATCH" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: OperatorFieldMismatch , "PROHIBITED_EMPTY_LIST_IN_CONDITION" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedEmptyListInCondition , "PROHIBITED_ENUM_CONSTANT" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedEnumConstant , "PROHIBITED_FIELD_COMBINATION_IN_SELECT_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedFieldCombinationInSelectClause , "PROHIBITED_FIELD_IN_ORDER_BY_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedFieldInOrderByClause , "PROHIBITED_FIELD_IN_SELECT_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedFieldInSelectClause , "PROHIBITED_FIELD_IN_WHERE_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedFieldInWhereClause , "PROHIBITED_METRIC_IN_SELECT_OR_WHERE_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedMetricInSelectOrWhereClause , "PROHIBITED_NEWLINE_IN_STRING" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedNewlineInString , "PROHIBITED_RESOURCE_TYPE_IN_FROM_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedResourceTypeInFromClause , "PROHIBITED_RESOURCE_TYPE_IN_SELECT_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedResourceTypeInSelectClause , "PROHIBITED_RESOURCE_TYPE_IN_WHERE_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedResourceTypeInWhereClause , "PROHIBITED_SEGMENT_IN_SELECT_OR_WHERE_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedSegmentInSelectOrWhereClause , "PROHIBITED_SEGMENT_WITH_METRIC_IN_SELECT_OR_WHERE_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedSegmentWithMetricInSelectOrWhereClause , "PROHIBITED_VALUE_COMBINATION_IN_LIST" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedValueCombinationInList , "PROHIBITED_VALUE_COMBINATION_WITH_BETWEEN_OPERATOR" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedValueCombinationWithBetweenOperator , "QUERY_ERROR" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: QueryError , "REQUESTED_METRICS_FOR_MANAGER" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: RequestedMetricsForManager , "STRING_NOT_TERMINATED" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: StringNotTerminated , "TOO_MANY_SEGMENTS" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: TooManySegments , "UNEXPECTED_END_OF_QUERY" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: UnexpectedEndOfQuery , "UNEXPECTED_FROM_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: UnexpectedFromClause , "UNEXPECTED_INPUT" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: UnexpectedInput , "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: Unknown , "UNRECOGNIZED_FIELD" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: UnrecognizedField , "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: Unspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ErrorsErrorCodeQueryError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ErrorsErrorCodeQueryError {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ErrorsErrorCodeQueryError {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "BAD_ENUM_CONSTANT" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadEnumConstant , "BAD_ESCAPE_SEQUENCE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadEscapeSequence , "BAD_FIELD_NAME" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadFieldName , "BAD_LIMIT_VALUE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadLimitValue , "BAD_NUMBER" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadNumber , "BAD_OPERATOR" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadOperator , "BAD_PARAMETER_NAME" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadParameterName , "BAD_PARAMETER_VALUE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadParameterValue , "BAD_RESOURCE_TYPE_IN_FROM_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadResourceTypeInFromClause , "BAD_SYMBOL" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadSymbol , "BAD_VALUE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: BadValue , "DATE_RANGE_TOO_NARROW" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: DateRangeTooNarrow , "DATE_RANGE_TOO_WIDE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: DateRangeTooWide , "EXPECTED_AND" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedAnd , "EXPECTED_BY" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedBy , "EXPECTED_DIMENSION_FIELD_IN_SELECT_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedDimensionFieldInSelectClause , "EXPECTED_FILTERS_ON_DATE_RANGE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedFiltersOnDateRange , "EXPECTED_FROM" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedFrom , "EXPECTED_LIST" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedList , "EXPECTED_REFERENCED_FIELD_IN_SELECT_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedReferencedFieldInSelectClause , "EXPECTED_SELECT" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedSelect , "EXPECTED_SINGLE_VALUE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedSingleValue , "EXPECTED_VALUE_WITH_BETWEEN_OPERATOR" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ExpectedValueWithBetweenOperator , "FILTER_HAS_TOO_MANY_VALUES" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: FilterHasTooManyValues , "INVALID_DATE_FORMAT" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: InvalidDateFormat , "INVALID_STRING_VALUE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: InvalidStringValue , "INVALID_VALUE_WITH_BETWEEN_OPERATOR" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: InvalidValueWithBetweenOperator , "INVALID_VALUE_WITH_DURING_OPERATOR" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: InvalidValueWithDuringOperator , "INVALID_VALUE_WITH_LIKE_OPERATOR" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: InvalidValueWithLikeOperator , "LIMIT_VALUE_TOO_LOW" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: LimitValueTooLow , "MISALIGNED_DATE_FOR_FILTER" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: MisalignedDateForFilter , "OPERATOR_FIELD_MISMATCH" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: OperatorFieldMismatch , "PROHIBITED_EMPTY_LIST_IN_CONDITION" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedEmptyListInCondition , "PROHIBITED_ENUM_CONSTANT" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedEnumConstant , "PROHIBITED_FIELD_COMBINATION_IN_SELECT_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedFieldCombinationInSelectClause , "PROHIBITED_FIELD_IN_ORDER_BY_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedFieldInOrderByClause , "PROHIBITED_FIELD_IN_SELECT_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedFieldInSelectClause , "PROHIBITED_FIELD_IN_WHERE_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedFieldInWhereClause , "PROHIBITED_METRIC_IN_SELECT_OR_WHERE_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedMetricInSelectOrWhereClause , "PROHIBITED_NEWLINE_IN_STRING" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedNewlineInString , "PROHIBITED_RESOURCE_TYPE_IN_FROM_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedResourceTypeInFromClause , "PROHIBITED_RESOURCE_TYPE_IN_SELECT_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedResourceTypeInSelectClause , "PROHIBITED_RESOURCE_TYPE_IN_WHERE_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedResourceTypeInWhereClause , "PROHIBITED_SEGMENT_IN_SELECT_OR_WHERE_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedSegmentInSelectOrWhereClause , "PROHIBITED_SEGMENT_WITH_METRIC_IN_SELECT_OR_WHERE_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedSegmentWithMetricInSelectOrWhereClause , "PROHIBITED_VALUE_COMBINATION_IN_LIST" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedValueCombinationInList , "PROHIBITED_VALUE_COMBINATION_WITH_BETWEEN_OPERATOR" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: ProhibitedValueCombinationWithBetweenOperator , "QUERY_ERROR" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: QueryError , "REQUESTED_METRICS_FOR_MANAGER" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: RequestedMetricsForManager , "STRING_NOT_TERMINATED" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: StringNotTerminated , "TOO_MANY_SEGMENTS" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: TooManySegments , "UNEXPECTED_END_OF_QUERY" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: UnexpectedEndOfQuery , "UNEXPECTED_FROM_CLAUSE" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: UnexpectedFromClause , "UNEXPECTED_INPUT" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: UnexpectedInput , "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: Unknown , "UNRECOGNIZED_FIELD" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: UnrecognizedField , "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeQueryError :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ErrorsErrorCodeQueryError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ErrorsErrorCodeQueryError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError {
        #[doc = "Access is prohibited."]
        AccessProhibited,
        #[doc = "Too many requests."]
        ResourceExhausted,
        #[doc = "Too many requests in a short amount of time."]
        ResourceTemporarilyExhausted,
        #[doc = "The received error code is not known in this version."]
        Unknown,
        #[doc = "Enum unspecified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError::AccessProhibited => {
                    "ACCESS_PROHIBITED"
                }
                GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError::ResourceExhausted => {
                    "RESOURCE_EXHAUSTED"
                }
                GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError::ResourceTemporarilyExhausted => {
                    "RESOURCE_TEMPORARILY_EXHAUSTED"
                }
                GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError, ()> {
            Ok(match s {
                "ACCESS_PROHIBITED" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError::AccessProhibited
                }
                "RESOURCE_EXHAUSTED" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError::ResourceExhausted
                }
                "RESOURCE_TEMPORARILY_EXHAUSTED" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError::ResourceTemporarilyExhausted
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCESS_PROHIBITED" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError::AccessProhibited
                }
                "RESOURCE_EXHAUSTED" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError::ResourceExhausted
                }
                "RESOURCE_TEMPORARILY_EXHAUSTED" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError::ResourceTemporarilyExhausted
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ErrorsErrorCodeQuotaError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ErrorsErrorCodeRequestError {
        #[doc = "Resource name provided is malformed."]
        BadResourceId,
        #[doc = "Request cannot be executed by a manager account."]
        CannotBeExecutedByManagerAccount,
        #[doc = "Mutate request was attempting to modify a readonly field. For instance, Budget fields can be requested for Ad Group, but are read-only for adGroups:mutate."]
        CannotModifyForeignField,
        #[doc = "return_summary_row cannot be enabled if request did not select any metrics field."]
        CannotReturnSummaryRowForRequestWithoutMetrics,
        #[doc = "return_summary_row should not be enabled for validate only requests."]
        CannotReturnSummaryRowForValidateOnlyRequests,
        #[doc = "Next page token specified in user request has expired."]
        ExpiredPageToken,
        #[doc = "The field cannot be modified because it’s immutable. It’s also possible that the field can be modified using ‘create’ operation but not ‘update’."]
        ImmutableField,
        #[doc = "return_summary_row parameter value should be the same between requests with page_token field set and their original request."]
        InconsistentReturnSummaryRowValue,
        #[doc = "Customer ID is invalid."]
        InvalidCustomerId,
        #[doc = "Enum value is not permitted."]
        InvalidEnumValue,
        #[doc = "Page size specified in user request is invalid."]
        InvalidPageSize,
        #[doc = "Next page token specified in user request is invalid."]
        InvalidPageToken,
        #[doc = "Product name is invalid."]
        InvalidProductName,
        #[doc = "The login-customer-id parameter is required for this request."]
        LoginCustomerIdParameterMissing,
        #[doc = "Either login-customer-id or linked-customer-id parameter is required for this request."]
        LoginOrLinkedCustomerIdParameterRequired,
        #[doc = "Mutate operation should have either create, update, or remove specified."]
        OperationRequired,
        #[doc = "The product associated with the request is not supported for the current request."]
        ProductNotSupported,
        #[doc = "Required field is missing."]
        RequiredFieldMissing,
        #[doc = "Resource name provided is malformed."]
        ResourceNameMalformed,
        #[doc = "Resource name is required for this request."]
        ResourceNameMissing,
        #[doc = "Requested resource not found."]
        ResourceNotFound,
        #[doc = "Deadline specified by the client was too short."]
        RpcDeadlineTooShort,
        #[doc = "Received too many entries in request."]
        TooManyMutateOperations,
        #[doc = "The total results count cannot be returned if it was not requested in the original request."]
        TotalResultsCountNotOriginallyRequested,
        #[doc = "The received error code is not known in this version."]
        Unknown,
        #[doc = "Enum unspecified."]
        Unspecified,
        #[doc = "page_token is set in the validate only request"]
        ValidateOnlyRequestHasPageToken,
    }
    impl GoogleAdsSearchads360V0ErrorsErrorCodeRequestError {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: BadResourceId => "BAD_RESOURCE_ID" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: CannotBeExecutedByManagerAccount => "CANNOT_BE_EXECUTED_BY_MANAGER_ACCOUNT" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: CannotModifyForeignField => "CANNOT_MODIFY_FOREIGN_FIELD" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: CannotReturnSummaryRowForRequestWithoutMetrics => "CANNOT_RETURN_SUMMARY_ROW_FOR_REQUEST_WITHOUT_METRICS" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: CannotReturnSummaryRowForValidateOnlyRequests => "CANNOT_RETURN_SUMMARY_ROW_FOR_VALIDATE_ONLY_REQUESTS" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ExpiredPageToken => "EXPIRED_PAGE_TOKEN" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ImmutableField => "IMMUTABLE_FIELD" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InconsistentReturnSummaryRowValue => "INCONSISTENT_RETURN_SUMMARY_ROW_VALUE" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InvalidCustomerId => "INVALID_CUSTOMER_ID" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InvalidEnumValue => "INVALID_ENUM_VALUE" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InvalidPageSize => "INVALID_PAGE_SIZE" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InvalidPageToken => "INVALID_PAGE_TOKEN" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InvalidProductName => "INVALID_PRODUCT_NAME" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: LoginCustomerIdParameterMissing => "LOGIN_CUSTOMER_ID_PARAMETER_MISSING" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: LoginOrLinkedCustomerIdParameterRequired => "LOGIN_OR_LINKED_CUSTOMER_ID_PARAMETER_REQUIRED" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: OperationRequired => "OPERATION_REQUIRED" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ProductNotSupported => "PRODUCT_NOT_SUPPORTED" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: RequiredFieldMissing => "REQUIRED_FIELD_MISSING" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ResourceNameMalformed => "RESOURCE_NAME_MALFORMED" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ResourceNameMissing => "RESOURCE_NAME_MISSING" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ResourceNotFound => "RESOURCE_NOT_FOUND" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: RpcDeadlineTooShort => "RPC_DEADLINE_TOO_SHORT" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: TooManyMutateOperations => "TOO_MANY_MUTATE_OPERATIONS" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: TotalResultsCountNotOriginallyRequested => "TOTAL_RESULTS_COUNT_NOT_ORIGINALLY_REQUESTED" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: Unspecified => "UNSPECIFIED" , GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ValidateOnlyRequestHasPageToken => "VALIDATE_ONLY_REQUEST_HAS_PAGE_TOKEN" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ErrorsErrorCodeRequestError {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ErrorsErrorCodeRequestError {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ErrorsErrorCodeRequestError, ()> {
            Ok (match s { "BAD_RESOURCE_ID" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: BadResourceId , "CANNOT_BE_EXECUTED_BY_MANAGER_ACCOUNT" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: CannotBeExecutedByManagerAccount , "CANNOT_MODIFY_FOREIGN_FIELD" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: CannotModifyForeignField , "CANNOT_RETURN_SUMMARY_ROW_FOR_REQUEST_WITHOUT_METRICS" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: CannotReturnSummaryRowForRequestWithoutMetrics , "CANNOT_RETURN_SUMMARY_ROW_FOR_VALIDATE_ONLY_REQUESTS" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: CannotReturnSummaryRowForValidateOnlyRequests , "EXPIRED_PAGE_TOKEN" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ExpiredPageToken , "IMMUTABLE_FIELD" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ImmutableField , "INCONSISTENT_RETURN_SUMMARY_ROW_VALUE" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InconsistentReturnSummaryRowValue , "INVALID_CUSTOMER_ID" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InvalidCustomerId , "INVALID_ENUM_VALUE" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InvalidEnumValue , "INVALID_PAGE_SIZE" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InvalidPageSize , "INVALID_PAGE_TOKEN" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InvalidPageToken , "INVALID_PRODUCT_NAME" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InvalidProductName , "LOGIN_CUSTOMER_ID_PARAMETER_MISSING" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: LoginCustomerIdParameterMissing , "LOGIN_OR_LINKED_CUSTOMER_ID_PARAMETER_REQUIRED" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: LoginOrLinkedCustomerIdParameterRequired , "OPERATION_REQUIRED" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: OperationRequired , "PRODUCT_NOT_SUPPORTED" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ProductNotSupported , "REQUIRED_FIELD_MISSING" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: RequiredFieldMissing , "RESOURCE_NAME_MALFORMED" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ResourceNameMalformed , "RESOURCE_NAME_MISSING" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ResourceNameMissing , "RESOURCE_NOT_FOUND" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ResourceNotFound , "RPC_DEADLINE_TOO_SHORT" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: RpcDeadlineTooShort , "TOO_MANY_MUTATE_OPERATIONS" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: TooManyMutateOperations , "TOTAL_RESULTS_COUNT_NOT_ORIGINALLY_REQUESTED" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: TotalResultsCountNotOriginallyRequested , "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: Unspecified , "VALIDATE_ONLY_REQUEST_HAS_PAGE_TOKEN" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ValidateOnlyRequestHasPageToken , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ErrorsErrorCodeRequestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ErrorsErrorCodeRequestError {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ErrorsErrorCodeRequestError {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "BAD_RESOURCE_ID" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: BadResourceId , "CANNOT_BE_EXECUTED_BY_MANAGER_ACCOUNT" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: CannotBeExecutedByManagerAccount , "CANNOT_MODIFY_FOREIGN_FIELD" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: CannotModifyForeignField , "CANNOT_RETURN_SUMMARY_ROW_FOR_REQUEST_WITHOUT_METRICS" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: CannotReturnSummaryRowForRequestWithoutMetrics , "CANNOT_RETURN_SUMMARY_ROW_FOR_VALIDATE_ONLY_REQUESTS" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: CannotReturnSummaryRowForValidateOnlyRequests , "EXPIRED_PAGE_TOKEN" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ExpiredPageToken , "IMMUTABLE_FIELD" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ImmutableField , "INCONSISTENT_RETURN_SUMMARY_ROW_VALUE" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InconsistentReturnSummaryRowValue , "INVALID_CUSTOMER_ID" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InvalidCustomerId , "INVALID_ENUM_VALUE" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InvalidEnumValue , "INVALID_PAGE_SIZE" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InvalidPageSize , "INVALID_PAGE_TOKEN" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InvalidPageToken , "INVALID_PRODUCT_NAME" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: InvalidProductName , "LOGIN_CUSTOMER_ID_PARAMETER_MISSING" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: LoginCustomerIdParameterMissing , "LOGIN_OR_LINKED_CUSTOMER_ID_PARAMETER_REQUIRED" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: LoginOrLinkedCustomerIdParameterRequired , "OPERATION_REQUIRED" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: OperationRequired , "PRODUCT_NOT_SUPPORTED" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ProductNotSupported , "REQUIRED_FIELD_MISSING" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: RequiredFieldMissing , "RESOURCE_NAME_MALFORMED" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ResourceNameMalformed , "RESOURCE_NAME_MISSING" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ResourceNameMissing , "RESOURCE_NOT_FOUND" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ResourceNotFound , "RPC_DEADLINE_TOO_SHORT" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: RpcDeadlineTooShort , "TOO_MANY_MUTATE_OPERATIONS" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: TooManyMutateOperations , "TOTAL_RESULTS_COUNT_NOT_ORIGINALLY_REQUESTED" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: TotalResultsCountNotOriginallyRequested , "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: Unspecified , "VALIDATE_ONLY_REQUEST_HAS_PAGE_TOKEN" => GoogleAdsSearchads360V0ErrorsErrorCodeRequestError :: ValidateOnlyRequestHasPageToken , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ErrorsErrorCodeRequestError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ErrorsErrorCodeRequestError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError {
        #[doc = "The number of entries in the request exceeds the system limit, or the contents of the operations exceed transaction limits due to their size or complexity. Try reducing the number of entries per request."]
        RequestSizeLimitExceeded,
        #[doc = "The number of entries in the response exceeds the system limit."]
        ResponseSizeLimitExceeded,
        #[doc = "The received error code is not known in this version."]
        Unknown,
        #[doc = "Enum unspecified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError::RequestSizeLimitExceeded => {
                    "REQUEST_SIZE_LIMIT_EXCEEDED"
                }
                GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError::ResponseSizeLimitExceeded => {
                    "RESPONSE_SIZE_LIMIT_EXCEEDED"
                }
                GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError, ()>
        {
            Ok(match s {
                "REQUEST_SIZE_LIMIT_EXCEEDED" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError::RequestSizeLimitExceeded
                }
                "RESPONSE_SIZE_LIMIT_EXCEEDED" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError::ResponseSizeLimitExceeded
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "REQUEST_SIZE_LIMIT_EXCEEDED" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError::RequestSizeLimitExceeded
                }
                "RESPONSE_SIZE_LIMIT_EXCEEDED" => {
                    GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError::ResponseSizeLimitExceeded
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError::Unspecified,
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
        for GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ErrorsErrorCodeSizeLimitError {
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
    pub struct GoogleAdsSearchads360V0ErrorsErrorDetails {
        #[doc = "Details on the quota error, including the scope (account or developer), the rate bucket name and the retry delay."]
        #[serde(
            rename = "quotaErrorDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quota_error_details:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ErrorsQuotaErrorDetails>,
        #[doc = "The error code that should have been returned, but wasn’t. This is used when the error code is not published in the client specified version."]
        #[serde(
            rename = "unpublishedErrorCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unpublished_error_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ErrorsErrorDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ErrorsErrorDetails {
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
    pub struct GoogleAdsSearchads360V0ErrorsErrorLocation {
        #[doc = "A field path that indicates which field was invalid in the request."]
        #[serde(
            rename = "fieldPathElements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_path_elements: ::std::option::Option<
            Vec<crate::schemas::GoogleAdsSearchads360V0ErrorsErrorLocationFieldPathElement>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ErrorsErrorLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ErrorsErrorLocation {
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
    pub struct GoogleAdsSearchads360V0ErrorsErrorLocationFieldPathElement {
        #[doc = "The name of a field or a oneof"]
        #[serde(
            rename = "fieldName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_name: ::std::option::Option<String>,
        #[doc = "If field_name is a repeated field, this is the element that failed"]
        #[serde(
            rename = "index",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ErrorsErrorLocationFieldPathElement
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ErrorsErrorLocationFieldPathElement
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
    pub struct GoogleAdsSearchads360V0ErrorsQuotaErrorDetails {
        #[doc = "The high level description of the quota bucket. Examples are “Get requests for standard access” or “Requests per account”."]
        #[serde(
            rename = "rateName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rate_name: ::std::option::Option<String>,
        #[doc = "The rate scope of the quota limit."]
        #[serde(
            rename = "rateScope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rate_scope: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope,
        >,
        #[doc = "Backoff period that customers should wait before sending next request."]
        #[serde(
            rename = "retryDelay",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub retry_delay: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ErrorsQuotaErrorDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ErrorsQuotaErrorDetails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope {
        #[doc = "Per customer account quota"]
        Account,
        #[doc = "Per project quota"]
        Developer,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Unspecified enum"]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope::Account => "ACCOUNT",
                GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope::Developer => "DEVELOPER",
                GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope::Unspecified => {
                    "UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope, ()>
        {
            Ok(match s {
                "ACCOUNT" => GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope::Account,
                "DEVELOPER" => GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope::Developer,
                "UNKNOWN" => GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope::Unspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCOUNT" => GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope::Account,
                "DEVELOPER" => GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope::Developer,
                "UNKNOWN" => GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope::Unspecified
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
        for GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ErrorsQuotaErrorDetailsRateScope
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0ErrorsSearchAds360Error {
        #[doc = "Additional error details, which are returned by certain error codes. Most error codes do not include details."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ErrorsErrorDetails>,
        #[doc = "An enum value that indicates which error occurred."]
        #[serde(
            rename = "errorCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_code:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ErrorsErrorCode>,
        #[doc = "Describes the part of the request proto that caused the error."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ErrorsErrorLocation>,
        #[doc = "A human-readable description of the error."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
        #[doc = "The value that triggered the error."]
        #[serde(
            rename = "trigger",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trigger: ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonValue>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ErrorsSearchAds360Error {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ErrorsSearchAds360Error {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0ErrorsSearchAds360Failure {
        #[doc = "The list of errors that occurred."]
        #[serde(
            rename = "errors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors: ::std::option::Option<
            Vec<crate::schemas::GoogleAdsSearchads360V0ErrorsSearchAds360Error>,
        >,
        #[doc = "The unique ID of the request that is used for debugging purposes."]
        #[serde(
            rename = "requestId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ErrorsSearchAds360Failure {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ErrorsSearchAds360Failure {
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
    pub struct GoogleAdsSearchads360V0ResourcesAdGroup {
        #[doc = "The ad rotation mode of the ad group."]
        #[serde(
            rename = "adRotationMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_rotation_mode: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode,
        >,
        #[doc = "The maximum CPC (cost-per-click) bid."]
        #[serde(
            rename = "cpcBidMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cpc_bid_micros: ::std::option::Option<i64>,
        #[doc = "Output only. The ID of the ad group."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<i64>,
        #[doc = "The name of the ad group. This field is required and should not be empty when creating new ad groups. It must contain fewer than 255 UTF-8 full-width characters. It must not contain any null (code point 0x0), NL line feed (code point 0xA) or carriage return (code point 0xD) characters."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Immutable. The type of the ad group."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ResourcesAdGroupType>,
        #[doc = "Immutable. The resource name of the ad group. Ad group resource names have the form: `customers/{customer_id}/adGroups/{ad_group_id}`"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "The status of the ad group."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ResourcesAdGroupStatus>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesAdGroup {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesAdGroup {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode {
        #[doc = "Optimize ad group ads based on clicks or conversions."]
        Optimize,
        #[doc = "Rotate evenly forever."]
        RotateForever,
        #[doc = "The received value is not known in this version. This is a response-only value."]
        Unknown,
        #[doc = "The ad rotation mode has not been specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode::Optimize => "OPTIMIZE",
                GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode::RotateForever => {
                    "ROTATE_FOREVER"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode, ()>
        {
            Ok(match s {
                "OPTIMIZE" => GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode::Optimize,
                "ROTATE_FOREVER" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode::RotateForever
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OPTIMIZE" => GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode::Optimize,
                "ROTATE_FOREVER" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode::RotateForever
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode::Unspecified,
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
        for GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesAdGroupAdRotationMode
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesAdGroupType {
        #[doc = "The default ad group type for Display campaigns."]
        DisplayStandard,
        #[doc = "The default ad group type for Hotel campaigns."]
        HotelAds,
        #[doc = "The ad group type for Promoted Hotel ad groups."]
        PromotedHotelAds,
        #[doc = "Ad group type for Dynamic Search Ads ad groups."]
        SearchDynamicAds,
        #[doc = "The default ad group type for Search campaigns."]
        SearchStandard,
        #[doc = "The type for ad groups in Shopping Comparison Listing campaigns."]
        ShoppingComparisonListingAds,
        #[doc = "The ad group type for Shopping campaigns serving standard product ads."]
        ShoppingProductAds,
        #[doc = "The type for ad groups that are limited to serving Showcase or Merchant ads in Shopping results."]
        ShoppingShowcaseAds,
        #[doc = "The type for ad groups in Smart Shopping campaigns."]
        ShoppingSmartAds,
        #[doc = "Ad group type for Smart campaigns."]
        SmartCampaignAds,
        #[doc = "The received value is not known in this version. This is a response-only value."]
        Unknown,
        #[doc = "The type has not been specified."]
        Unspecified,
        #[doc = "Short unskippable in-stream video ads."]
        VideoBumper,
        #[doc = "Video efficient reach ad groups."]
        VideoEfficientReach,
        #[doc = "Unskippable in-stream video ads."]
        VideoNonSkippableInStream,
        #[doc = "Outstream video ads."]
        VideoOutstream,
        #[doc = "Video responsive ad groups."]
        VideoResponsive,
        #[doc = "TrueView in-display video ads."]
        VideoTrueViewInDisplay,
        #[doc = "TrueView (skippable) in-stream video ads."]
        VideoTrueViewInStream,
    }
    impl GoogleAdsSearchads360V0ResourcesAdGroupType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesAdGroupType::DisplayStandard => "DISPLAY_STANDARD",
                GoogleAdsSearchads360V0ResourcesAdGroupType::HotelAds => "HOTEL_ADS",
                GoogleAdsSearchads360V0ResourcesAdGroupType::PromotedHotelAds => {
                    "PROMOTED_HOTEL_ADS"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupType::SearchDynamicAds => {
                    "SEARCH_DYNAMIC_ADS"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupType::SearchStandard => "SEARCH_STANDARD",
                GoogleAdsSearchads360V0ResourcesAdGroupType::ShoppingComparisonListingAds => {
                    "SHOPPING_COMPARISON_LISTING_ADS"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupType::ShoppingProductAds => {
                    "SHOPPING_PRODUCT_ADS"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupType::ShoppingShowcaseAds => {
                    "SHOPPING_SHOWCASE_ADS"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupType::ShoppingSmartAds => {
                    "SHOPPING_SMART_ADS"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupType::SmartCampaignAds => {
                    "SMART_CAMPAIGN_ADS"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupType::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesAdGroupType::Unspecified => "UNSPECIFIED",
                GoogleAdsSearchads360V0ResourcesAdGroupType::VideoBumper => "VIDEO_BUMPER",
                GoogleAdsSearchads360V0ResourcesAdGroupType::VideoEfficientReach => {
                    "VIDEO_EFFICIENT_REACH"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupType::VideoNonSkippableInStream => {
                    "VIDEO_NON_SKIPPABLE_IN_STREAM"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupType::VideoOutstream => "VIDEO_OUTSTREAM",
                GoogleAdsSearchads360V0ResourcesAdGroupType::VideoResponsive => "VIDEO_RESPONSIVE",
                GoogleAdsSearchads360V0ResourcesAdGroupType::VideoTrueViewInDisplay => {
                    "VIDEO_TRUE_VIEW_IN_DISPLAY"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupType::VideoTrueViewInStream => {
                    "VIDEO_TRUE_VIEW_IN_STREAM"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesAdGroupType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesAdGroupType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesAdGroupType, ()> {
            Ok(match s {
                "DISPLAY_STANDARD" => GoogleAdsSearchads360V0ResourcesAdGroupType::DisplayStandard,
                "HOTEL_ADS" => GoogleAdsSearchads360V0ResourcesAdGroupType::HotelAds,
                "PROMOTED_HOTEL_ADS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::PromotedHotelAds
                }
                "SEARCH_DYNAMIC_ADS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::SearchDynamicAds
                }
                "SEARCH_STANDARD" => GoogleAdsSearchads360V0ResourcesAdGroupType::SearchStandard,
                "SHOPPING_COMPARISON_LISTING_ADS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::ShoppingComparisonListingAds
                }
                "SHOPPING_PRODUCT_ADS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::ShoppingProductAds
                }
                "SHOPPING_SHOWCASE_ADS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::ShoppingShowcaseAds
                }
                "SHOPPING_SMART_ADS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::ShoppingSmartAds
                }
                "SMART_CAMPAIGN_ADS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::SmartCampaignAds
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesAdGroupType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesAdGroupType::Unspecified,
                "VIDEO_BUMPER" => GoogleAdsSearchads360V0ResourcesAdGroupType::VideoBumper,
                "VIDEO_EFFICIENT_REACH" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::VideoEfficientReach
                }
                "VIDEO_NON_SKIPPABLE_IN_STREAM" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::VideoNonSkippableInStream
                }
                "VIDEO_OUTSTREAM" => GoogleAdsSearchads360V0ResourcesAdGroupType::VideoOutstream,
                "VIDEO_RESPONSIVE" => GoogleAdsSearchads360V0ResourcesAdGroupType::VideoResponsive,
                "VIDEO_TRUE_VIEW_IN_DISPLAY" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::VideoTrueViewInDisplay
                }
                "VIDEO_TRUE_VIEW_IN_STREAM" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::VideoTrueViewInStream
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesAdGroupType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesAdGroupType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesAdGroupType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DISPLAY_STANDARD" => GoogleAdsSearchads360V0ResourcesAdGroupType::DisplayStandard,
                "HOTEL_ADS" => GoogleAdsSearchads360V0ResourcesAdGroupType::HotelAds,
                "PROMOTED_HOTEL_ADS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::PromotedHotelAds
                }
                "SEARCH_DYNAMIC_ADS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::SearchDynamicAds
                }
                "SEARCH_STANDARD" => GoogleAdsSearchads360V0ResourcesAdGroupType::SearchStandard,
                "SHOPPING_COMPARISON_LISTING_ADS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::ShoppingComparisonListingAds
                }
                "SHOPPING_PRODUCT_ADS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::ShoppingProductAds
                }
                "SHOPPING_SHOWCASE_ADS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::ShoppingShowcaseAds
                }
                "SHOPPING_SMART_ADS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::ShoppingSmartAds
                }
                "SMART_CAMPAIGN_ADS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::SmartCampaignAds
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesAdGroupType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesAdGroupType::Unspecified,
                "VIDEO_BUMPER" => GoogleAdsSearchads360V0ResourcesAdGroupType::VideoBumper,
                "VIDEO_EFFICIENT_REACH" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::VideoEfficientReach
                }
                "VIDEO_NON_SKIPPABLE_IN_STREAM" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::VideoNonSkippableInStream
                }
                "VIDEO_OUTSTREAM" => GoogleAdsSearchads360V0ResourcesAdGroupType::VideoOutstream,
                "VIDEO_RESPONSIVE" => GoogleAdsSearchads360V0ResourcesAdGroupType::VideoResponsive,
                "VIDEO_TRUE_VIEW_IN_DISPLAY" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::VideoTrueViewInDisplay
                }
                "VIDEO_TRUE_VIEW_IN_STREAM" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupType::VideoTrueViewInStream
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
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesAdGroupType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesAdGroupType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesAdGroupStatus {
        #[doc = "The ad group is enabled."]
        Enabled,
        #[doc = "The ad group is paused."]
        Paused,
        #[doc = "The ad group is removed."]
        Removed,
        #[doc = "The received value is not known in this version. This is a response-only value."]
        Unknown,
        #[doc = "The status has not been specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesAdGroupStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesAdGroupStatus::Enabled => "ENABLED",
                GoogleAdsSearchads360V0ResourcesAdGroupStatus::Paused => "PAUSED",
                GoogleAdsSearchads360V0ResourcesAdGroupStatus::Removed => "REMOVED",
                GoogleAdsSearchads360V0ResourcesAdGroupStatus::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesAdGroupStatus::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesAdGroupStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesAdGroupStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesAdGroupStatus, ()> {
            Ok(match s {
                "ENABLED" => GoogleAdsSearchads360V0ResourcesAdGroupStatus::Enabled,
                "PAUSED" => GoogleAdsSearchads360V0ResourcesAdGroupStatus::Paused,
                "REMOVED" => GoogleAdsSearchads360V0ResourcesAdGroupStatus::Removed,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesAdGroupStatus::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesAdGroupStatus::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesAdGroupStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesAdGroupStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesAdGroupStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ENABLED" => GoogleAdsSearchads360V0ResourcesAdGroupStatus::Enabled,
                "PAUSED" => GoogleAdsSearchads360V0ResourcesAdGroupStatus::Paused,
                "REMOVED" => GoogleAdsSearchads360V0ResourcesAdGroupStatus::Removed,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesAdGroupStatus::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesAdGroupStatus::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesAdGroupStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesAdGroupStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0ResourcesAdGroupBidModifier {
        #[doc = "The modifier for the bid when the criterion matches. The modifier must be in the range: 0.1 - 10.0. The range is 1.0 - 6.0 for PreferredContent. Use 0 to opt out of a Device type."]
        #[serde(
            rename = "bidModifier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bid_modifier: ::std::option::Option<f64>,
        #[doc = "Immutable. The resource name of the ad group bid modifier. Ad group bid modifier resource names have the form: `customers/{customer_id}/adGroupBidModifiers/{ad_group_id}~{criterion_id}`"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesAdGroupBidModifier {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesAdGroupBidModifier {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0ResourcesAdGroupCriterion {
        #[doc = "Immutable. The ad group to which the criterion belongs."]
        #[serde(
            rename = "adGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_group: ::std::option::Option<String>,
        #[doc = "Immutable. Age range."]
        #[serde(
            rename = "ageRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub age_range:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonAgeRangeInfo>,
        #[doc = "The modifier for the bid when the criterion matches. The modifier must be in the range: 0.1 - 10.0. Most targetable criteria types support modifiers."]
        #[serde(
            rename = "bidModifier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bid_modifier: ::std::option::Option<f64>,
        #[doc = "The CPC (cost-per-click) bid."]
        #[serde(
            rename = "cpcBidMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cpc_bid_micros: ::std::option::Option<i64>,
        #[doc = "Output only. The ID of the criterion."]
        #[serde(
            rename = "criterionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub criterion_id: ::std::option::Option<i64>,
        #[doc = "Output only. The effective CPC (cost-per-click) bid."]
        #[serde(
            rename = "effectiveCpcBidMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub effective_cpc_bid_micros: ::std::option::Option<i64>,
        #[doc = "Output only. The Engine Status for ad group criterion."]
        #[serde(
            rename = "engineStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub engine_status: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus,
        >,
        #[doc = "URL template for appending params to final URL."]
        #[serde(
            rename = "finalUrlSuffix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub final_url_suffix: ::std::option::Option<String>,
        #[doc = "The list of possible final URLs after all cross-domain redirects for the ad."]
        #[serde(
            rename = "finalUrls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub final_urls: ::std::option::Option<Vec<String>>,
        #[doc = "Immutable. Gender."]
        #[serde(
            rename = "gender",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gender: ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonGenderInfo>,
        #[doc = "Immutable. Keyword."]
        #[serde(
            rename = "keyword",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keyword:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonKeywordInfo>,
        #[doc = "Output only. The datetime when this ad group criterion was last modified. The datetime is in the customer’s time zone and in “yyyy-MM-dd HH:mm:ss.ssssss” format."]
        #[serde(
            rename = "lastModifiedTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_modified_time: ::std::option::Option<String>,
        #[doc = "Immutable. Listing group."]
        #[serde(
            rename = "listingGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub listing_group:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonListingGroupInfo>,
        #[doc = "Output only. Information regarding the quality of the criterion."]
        #[serde(
            rename = "qualityInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quality_info: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesAdGroupCriterionQualityInfo,
        >,
        #[doc = "Output only. The type of the criterion."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesAdGroupCriterionType,
        >,
        #[doc = "Immutable. The resource name of the ad group criterion. Ad group criterion resource names have the form: `customers/{customer_id}/adGroupCriteria/{ad_group_id}~{criterion_id}`"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "The status of the criterion. This is the status of the ad group criterion entity, set by the client. Note: UI reports may incorporate additional information that affects whether a criterion is eligible to run. In some cases a criterion that’s REMOVED in the API can still show as enabled in the UI. For example, campaigns by default show to users of all age ranges unless excluded. The UI will show each age range as “enabled”, since they’re eligible to see the ads; but AdGroupCriterion.status will show “removed”, since no positive criterion was added."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus,
        >,
        #[doc = "The URL template for constructing a tracking URL."]
        #[serde(
            rename = "trackingUrlTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracking_url_template: ::std::option::Option<String>,
        #[doc = "Immutable. Webpage"]
        #[serde(
            rename = "webpage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub webpage:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonWebpageInfo>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesAdGroupCriterion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesAdGroupCriterion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus {
        #[doc = "Criterion has been paused since the account is paused."]
        AdGroupCriterionAccountPaused,
        #[doc = "Criterion has been approved."]
        AdGroupCriterionApproved,
        #[doc = "Criterion has been disapproved."]
        AdGroupCriterionDisapproved,
        #[doc = "Deprecated. Do not use."]
        AdGroupCriterionEligible,
        #[doc = "Baidu: Bid or quality too low to be displayed."]
        AdGroupCriterionInappropriateForCampaign,
        #[doc = "Baidu: Bid or quality too low for mobile, but eligible to display for desktop."]
        AdGroupCriterionInvalidMobileSearch,
        #[doc = "Baidu: Bid or quality too low for desktop, but eligible to display for mobile."]
        AdGroupCriterionInvalidPcSearch,
        #[doc = "Baidu: Bid or quality too low to be displayed."]
        AdGroupCriterionInvalidSearch,
        #[doc = "Baidu: Paused by Baidu due to low search volume."]
        AdGroupCriterionLowSearchVolume,
        #[doc = "Baidu: Mobile URL in process to be reviewed."]
        AdGroupCriterionMobileUrlUnderReview,
        #[doc = "Baidu: Criterion to be reviewed."]
        AdGroupCriterionNotReviewed,
        #[doc = "Deprecated. Do not use. Previously used by Gemini"]
        AdGroupCriterionOnHold,
        #[doc = "Baidu: The landing page for one device is invalid, while the landing page for the other device is valid."]
        AdGroupCriterionPartiallyInvalid,
        #[doc = "Criterion has been paused."]
        AdGroupCriterionPaused,
        #[doc = "Y!J : Criterion pending review"]
        AdGroupCriterionPendingReview,
        #[doc = "Criterion has been removed."]
        AdGroupCriterionRemoved,
        #[doc = "Criterion is active and serving."]
        AdGroupCriterionServing,
        #[doc = "Baidu: Keyword has been created and paused by Baidu account management, and is now ready for you to activate it."]
        AdGroupCriterionToBeActivated,
        #[doc = "Baidu: In process to be reviewed by Baidu. Gemini: Criterion under review."]
        AdGroupCriterionUnderReview,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionAccountPaused => "AD_GROUP_CRITERION_ACCOUNT_PAUSED" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionApproved => "AD_GROUP_CRITERION_APPROVED" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionDisapproved => "AD_GROUP_CRITERION_DISAPPROVED" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionEligible => "AD_GROUP_CRITERION_ELIGIBLE" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionInappropriateForCampaign => "AD_GROUP_CRITERION_INAPPROPRIATE_FOR_CAMPAIGN" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionInvalidMobileSearch => "AD_GROUP_CRITERION_INVALID_MOBILE_SEARCH" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionInvalidPcSearch => "AD_GROUP_CRITERION_INVALID_PC_SEARCH" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionInvalidSearch => "AD_GROUP_CRITERION_INVALID_SEARCH" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionLowSearchVolume => "AD_GROUP_CRITERION_LOW_SEARCH_VOLUME" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionMobileUrlUnderReview => "AD_GROUP_CRITERION_MOBILE_URL_UNDER_REVIEW" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionNotReviewed => "AD_GROUP_CRITERION_NOT_REVIEWED" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionOnHold => "AD_GROUP_CRITERION_ON_HOLD" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionPartiallyInvalid => "AD_GROUP_CRITERION_PARTIALLY_INVALID" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionPaused => "AD_GROUP_CRITERION_PAUSED" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionPendingReview => "AD_GROUP_CRITERION_PENDING_REVIEW" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionRemoved => "AD_GROUP_CRITERION_REMOVED" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionServing => "AD_GROUP_CRITERION_SERVING" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionToBeActivated => "AD_GROUP_CRITERION_TO_BE_ACTIVATED" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionUnderReview => "AD_GROUP_CRITERION_UNDER_REVIEW" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus, ()>
        {
            Ok (match s { "AD_GROUP_CRITERION_ACCOUNT_PAUSED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionAccountPaused , "AD_GROUP_CRITERION_APPROVED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionApproved , "AD_GROUP_CRITERION_DISAPPROVED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionDisapproved , "AD_GROUP_CRITERION_ELIGIBLE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionEligible , "AD_GROUP_CRITERION_INAPPROPRIATE_FOR_CAMPAIGN" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionInappropriateForCampaign , "AD_GROUP_CRITERION_INVALID_MOBILE_SEARCH" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionInvalidMobileSearch , "AD_GROUP_CRITERION_INVALID_PC_SEARCH" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionInvalidPcSearch , "AD_GROUP_CRITERION_INVALID_SEARCH" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionInvalidSearch , "AD_GROUP_CRITERION_LOW_SEARCH_VOLUME" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionLowSearchVolume , "AD_GROUP_CRITERION_MOBILE_URL_UNDER_REVIEW" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionMobileUrlUnderReview , "AD_GROUP_CRITERION_NOT_REVIEWED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionNotReviewed , "AD_GROUP_CRITERION_ON_HOLD" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionOnHold , "AD_GROUP_CRITERION_PARTIALLY_INVALID" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionPartiallyInvalid , "AD_GROUP_CRITERION_PAUSED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionPaused , "AD_GROUP_CRITERION_PENDING_REVIEW" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionPendingReview , "AD_GROUP_CRITERION_REMOVED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionRemoved , "AD_GROUP_CRITERION_SERVING" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionServing , "AD_GROUP_CRITERION_TO_BE_ACTIVATED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionToBeActivated , "AD_GROUP_CRITERION_UNDER_REVIEW" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionUnderReview , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: Unspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "AD_GROUP_CRITERION_ACCOUNT_PAUSED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionAccountPaused , "AD_GROUP_CRITERION_APPROVED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionApproved , "AD_GROUP_CRITERION_DISAPPROVED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionDisapproved , "AD_GROUP_CRITERION_ELIGIBLE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionEligible , "AD_GROUP_CRITERION_INAPPROPRIATE_FOR_CAMPAIGN" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionInappropriateForCampaign , "AD_GROUP_CRITERION_INVALID_MOBILE_SEARCH" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionInvalidMobileSearch , "AD_GROUP_CRITERION_INVALID_PC_SEARCH" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionInvalidPcSearch , "AD_GROUP_CRITERION_INVALID_SEARCH" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionInvalidSearch , "AD_GROUP_CRITERION_LOW_SEARCH_VOLUME" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionLowSearchVolume , "AD_GROUP_CRITERION_MOBILE_URL_UNDER_REVIEW" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionMobileUrlUnderReview , "AD_GROUP_CRITERION_NOT_REVIEWED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionNotReviewed , "AD_GROUP_CRITERION_ON_HOLD" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionOnHold , "AD_GROUP_CRITERION_PARTIALLY_INVALID" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionPartiallyInvalid , "AD_GROUP_CRITERION_PAUSED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionPaused , "AD_GROUP_CRITERION_PENDING_REVIEW" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionPendingReview , "AD_GROUP_CRITERION_REMOVED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionRemoved , "AD_GROUP_CRITERION_SERVING" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionServing , "AD_GROUP_CRITERION_TO_BE_ACTIVATED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionToBeActivated , "AD_GROUP_CRITERION_UNDER_REVIEW" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: AdGroupCriterionUnderReview , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesAdGroupCriterionEngineStatus
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesAdGroupCriterionType {
        #[doc = "Ad Schedule."]
        AdSchedule,
        #[doc = "Age range."]
        AgeRange,
        #[doc = "App payment model."]
        AppPaymentModel,
        #[doc = "Audience"]
        Audience,
        #[doc = "Carrier."]
        Carrier,
        #[doc = "Combined audience"]
        CombinedAudience,
        #[doc = "Content Label for category exclusion."]
        ContentLabel,
        #[doc = "Custom affinity."]
        CustomAffinity,
        #[doc = "Custom audience"]
        CustomAudience,
        #[doc = "Custom intent."]
        CustomIntent,
        #[doc = "Devices to target."]
        Device,
        #[doc = "Gender."]
        Gender,
        #[doc = "Income Range."]
        IncomeRange,
        #[doc = "IpBlock."]
        IpBlock,
        #[doc = "Keyword, for example, ‘mars cruise’."]
        Keyword,
        #[doc = "Smart Campaign keyword theme"]
        KeywordTheme,
        #[doc = "Language."]
        Language,
        #[doc = "Listing groups to target."]
        ListingGroup,
        #[doc = "Listing scope to target."]
        ListingScope,
        #[doc = "Google Local Services (GLS) Service ID."]
        LocalServiceId,
        #[doc = "Locations to target."]
        Location,
        #[doc = "Location group."]
        LocationGroup,
        #[doc = "Mobile application categories to target."]
        MobileAppCategory,
        #[doc = "Mobile applications to target."]
        MobileApplication,
        #[doc = "Mobile device."]
        MobileDevice,
        #[doc = "Operating system version."]
        OperatingSystemVersion,
        #[doc = "Parental status."]
        ParentalStatus,
        #[doc = "Placement, also known as Website, for example, ‘www.flowers4sale.com’"]
        Placement,
        #[doc = "Proximity."]
        Proximity,
        #[doc = "A topic target on the display network (for example, “Pets & Animals”)."]
        Topic,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
        #[doc = "A category the user is interested in."]
        UserInterest,
        #[doc = "User list."]
        UserList,
        #[doc = "Webpage criterion for dynamic search ads."]
        Webpage,
        #[doc = "YouTube Channel."]
        YoutubeChannel,
        #[doc = "YouTube Video."]
        YoutubeVideo,
    }
    impl GoogleAdsSearchads360V0ResourcesAdGroupCriterionType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::AdSchedule => "AD_SCHEDULE",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::AgeRange => "AGE_RANGE",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::AppPaymentModel => {
                    "APP_PAYMENT_MODEL"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Audience => "AUDIENCE",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Carrier => "CARRIER",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::CombinedAudience => {
                    "COMBINED_AUDIENCE"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::ContentLabel => {
                    "CONTENT_LABEL"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::CustomAffinity => {
                    "CUSTOM_AFFINITY"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::CustomAudience => {
                    "CUSTOM_AUDIENCE"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::CustomIntent => {
                    "CUSTOM_INTENT"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Device => "DEVICE",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Gender => "GENDER",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::IncomeRange => "INCOME_RANGE",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::IpBlock => "IP_BLOCK",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Keyword => "KEYWORD",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::KeywordTheme => {
                    "KEYWORD_THEME"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Language => "LANGUAGE",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::ListingGroup => {
                    "LISTING_GROUP"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::ListingScope => {
                    "LISTING_SCOPE"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::LocalServiceId => {
                    "LOCAL_SERVICE_ID"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Location => "LOCATION",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::LocationGroup => {
                    "LOCATION_GROUP"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::MobileAppCategory => {
                    "MOBILE_APP_CATEGORY"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::MobileApplication => {
                    "MOBILE_APPLICATION"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::MobileDevice => {
                    "MOBILE_DEVICE"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::OperatingSystemVersion => {
                    "OPERATING_SYSTEM_VERSION"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::ParentalStatus => {
                    "PARENTAL_STATUS"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Placement => "PLACEMENT",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Proximity => "PROXIMITY",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Topic => "TOPIC",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Unspecified => "UNSPECIFIED",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::UserInterest => {
                    "USER_INTEREST"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::UserList => "USER_LIST",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Webpage => "WEBPAGE",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::YoutubeChannel => {
                    "YOUTUBE_CHANNEL"
                }
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::YoutubeVideo => {
                    "YOUTUBE_VIDEO"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesAdGroupCriterionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesAdGroupCriterionType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesAdGroupCriterionType, ()>
        {
            Ok(match s {
                "AD_SCHEDULE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::AdSchedule,
                "AGE_RANGE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::AgeRange,
                "APP_PAYMENT_MODEL" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::AppPaymentModel
                }
                "AUDIENCE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Audience,
                "CARRIER" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Carrier,
                "COMBINED_AUDIENCE" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::CombinedAudience
                }
                "CONTENT_LABEL" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::ContentLabel
                }
                "CUSTOM_AFFINITY" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::CustomAffinity
                }
                "CUSTOM_AUDIENCE" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::CustomAudience
                }
                "CUSTOM_INTENT" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::CustomIntent
                }
                "DEVICE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Device,
                "GENDER" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Gender,
                "INCOME_RANGE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::IncomeRange,
                "IP_BLOCK" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::IpBlock,
                "KEYWORD" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Keyword,
                "KEYWORD_THEME" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::KeywordTheme
                }
                "LANGUAGE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Language,
                "LISTING_GROUP" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::ListingGroup
                }
                "LISTING_SCOPE" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::ListingScope
                }
                "LOCAL_SERVICE_ID" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::LocalServiceId
                }
                "LOCATION" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Location,
                "LOCATION_GROUP" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::LocationGroup
                }
                "MOBILE_APP_CATEGORY" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::MobileAppCategory
                }
                "MOBILE_APPLICATION" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::MobileApplication
                }
                "MOBILE_DEVICE" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::MobileDevice
                }
                "OPERATING_SYSTEM_VERSION" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::OperatingSystemVersion
                }
                "PARENTAL_STATUS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::ParentalStatus
                }
                "PLACEMENT" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Placement,
                "PROXIMITY" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Proximity,
                "TOPIC" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Topic,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Unspecified,
                "USER_INTEREST" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::UserInterest
                }
                "USER_LIST" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::UserList,
                "WEBPAGE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Webpage,
                "YOUTUBE_CHANNEL" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::YoutubeChannel
                }
                "YOUTUBE_VIDEO" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::YoutubeVideo
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesAdGroupCriterionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesAdGroupCriterionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesAdGroupCriterionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AD_SCHEDULE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::AdSchedule,
                "AGE_RANGE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::AgeRange,
                "APP_PAYMENT_MODEL" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::AppPaymentModel
                }
                "AUDIENCE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Audience,
                "CARRIER" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Carrier,
                "COMBINED_AUDIENCE" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::CombinedAudience
                }
                "CONTENT_LABEL" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::ContentLabel
                }
                "CUSTOM_AFFINITY" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::CustomAffinity
                }
                "CUSTOM_AUDIENCE" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::CustomAudience
                }
                "CUSTOM_INTENT" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::CustomIntent
                }
                "DEVICE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Device,
                "GENDER" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Gender,
                "INCOME_RANGE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::IncomeRange,
                "IP_BLOCK" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::IpBlock,
                "KEYWORD" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Keyword,
                "KEYWORD_THEME" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::KeywordTheme
                }
                "LANGUAGE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Language,
                "LISTING_GROUP" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::ListingGroup
                }
                "LISTING_SCOPE" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::ListingScope
                }
                "LOCAL_SERVICE_ID" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::LocalServiceId
                }
                "LOCATION" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Location,
                "LOCATION_GROUP" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::LocationGroup
                }
                "MOBILE_APP_CATEGORY" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::MobileAppCategory
                }
                "MOBILE_APPLICATION" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::MobileApplication
                }
                "MOBILE_DEVICE" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::MobileDevice
                }
                "OPERATING_SYSTEM_VERSION" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::OperatingSystemVersion
                }
                "PARENTAL_STATUS" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::ParentalStatus
                }
                "PLACEMENT" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Placement,
                "PROXIMITY" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Proximity,
                "TOPIC" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Topic,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Unspecified,
                "USER_INTEREST" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::UserInterest
                }
                "USER_LIST" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::UserList,
                "WEBPAGE" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::Webpage,
                "YOUTUBE_CHANNEL" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::YoutubeChannel
                }
                "YOUTUBE_VIDEO" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionType::YoutubeVideo
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
        for GoogleAdsSearchads360V0ResourcesAdGroupCriterionType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesAdGroupCriterionType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus {
        #[doc = "The ad group criterion is enabled."]
        Enabled,
        #[doc = "The ad group criterion is paused."]
        Paused,
        #[doc = "The ad group criterion is removed."]
        Removed,
        #[doc = "The received value is not known in this version. This is a response-only value."]
        Unknown,
        #[doc = "No value has been specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus::Enabled => "ENABLED",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus::Paused => "PAUSED",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus::Removed => "REMOVED",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus::Unspecified => {
                    "UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus, ()>
        {
            Ok(match s {
                "ENABLED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus::Enabled,
                "PAUSED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus::Paused,
                "REMOVED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus::Removed,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus::Unspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ENABLED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus::Enabled,
                "PAUSED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus::Paused,
                "REMOVED" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus::Removed,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus::Unspecified
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
        for GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesAdGroupCriterionStatus
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
    pub struct GoogleAdsSearchads360V0ResourcesAdGroupCriterionQualityInfo {
        #[doc = "Output only. The quality score. This field may not be populated if Google does not have enough information to determine a value."]
        #[serde(
            rename = "qualityScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quality_score: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesAdGroupCriterionQualityInfo
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesAdGroupCriterionQualityInfo
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0ResourcesBiddingStrategy {
        #[doc = "Output only. The number of campaigns attached to this bidding strategy. This field is read-only."]
        #[serde(
            rename = "campaignCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub campaign_count: ::std::option::Option<i64>,
        #[doc = "Immutable. The currency used by the bidding strategy (ISO 4217 three-letter code). For bidding strategies in manager customers, this currency can be set on creation and defaults to the manager customer’s currency. For serving customers, this field cannot be set; all strategies in a serving customer implicitly use the serving customer’s currency. In all cases the effective_currency_code field returns the currency used by the strategy."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Output only. The currency used by the bidding strategy (ISO 4217 three-letter code). For bidding strategies in manager customers, this is the currency set by the advertiser when creating the strategy. For serving customers, this is the customer’s currency_code. Bidding strategy metrics are reported in this currency. This field is read-only."]
        #[serde(
            rename = "effectiveCurrencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub effective_currency_code: ::std::option::Option<String>,
        #[doc = "A bidding strategy that raises bids for clicks that seem more likely to lead to a conversion and lowers them for clicks where they seem less likely."]
        #[serde(
            rename = "enhancedCpc",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enhanced_cpc:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonEnhancedCpc>,
        #[doc = "Output only. The ID of the bidding strategy."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<i64>,
        #[doc = "An automated bidding strategy to help get the most conversion value for your campaigns while spending your budget."]
        #[serde(
            rename = "maximizeConversionValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maximize_conversion_value: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0CommonMaximizeConversionValue,
        >,
        #[doc = "An automated bidding strategy to help get the most conversions for your campaigns while spending your budget."]
        #[serde(
            rename = "maximizeConversions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maximize_conversions:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonMaximizeConversions>,
        #[doc = "The name of the bidding strategy. All bidding strategies within an account must be named distinctly. The length of this string should be between 1 and 255, inclusive, in UTF-8 bytes, (trimmed)."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. The number of non-removed campaigns attached to this bidding strategy. This field is read-only."]
        #[serde(
            rename = "nonRemovedCampaignCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub non_removed_campaign_count: ::std::option::Option<i64>,
        #[doc = "Output only. The type of the bidding strategy. Create a bidding strategy by setting the bidding scheme. This field is read-only."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesBiddingStrategyType,
        >,
        #[doc = "Immutable. The resource name of the bidding strategy. Bidding strategy resource names have the form: `customers/{customer_id}/biddingStrategies/{bidding_strategy_id}`"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "Output only. The status of the bidding strategy. This field is read-only."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus,
        >,
        #[doc = "A bidding strategy that sets bids to help get as many conversions as possible at the target cost-per-acquisition (CPA) you set."]
        #[serde(
            rename = "targetCpa",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_cpa:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonTargetCpa>,
        #[doc = "A bidding strategy that automatically optimizes towards a chosen percentage of impressions."]
        #[serde(
            rename = "targetImpressionShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_impression_share: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0CommonTargetImpressionShare,
        >,
        #[doc = "A bidding strategy that sets bids based on the target fraction of auctions where the advertiser should outrank a specific competitor. This field is deprecated. Creating a new bidding strategy with this field or attaching bidding strategies with this field to a campaign will fail. Mutates to strategies that already have this scheme populated are allowed."]
        #[serde(
            rename = "targetOutrankShare",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_outrank_share:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonTargetOutrankShare>,
        #[doc = "A bidding strategy that helps you maximize revenue while averaging a specific target Return On Ad Spend (ROAS)."]
        #[serde(
            rename = "targetRoas",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_roas:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonTargetRoas>,
        #[doc = "A bid strategy that sets your bids to help get as many clicks as possible within your budget."]
        #[serde(
            rename = "targetSpend",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_spend:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonTargetSpend>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesBiddingStrategy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesBiddingStrategy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesBiddingStrategyType {
        #[doc = "Commission is an automatic bidding strategy in which the advertiser pays a certain portion of the conversion value."]
        Commission,
        #[doc = "Enhanced CPC is a bidding strategy that raises bids for clicks that seem more likely to lead to a conversion and lowers them for clicks where they seem less likely."]
        EnhancedCpc,
        #[doc = "Used for return value only. Indicates that a campaign does not have a bidding strategy. This prevents the campaign from serving. For example, a campaign may be attached to a manager bidding strategy and the serving account is subsequently unlinked from the manager account. In this case the campaign will automatically be detached from the now inaccessible manager bidding strategy and transition to the INVALID bidding strategy type."]
        Invalid,
        #[doc = "Manual bidding strategy that allows advertiser to set the bid per advertiser-specified action."]
        ManualCpa,
        #[doc = "Manual click based bidding where user pays per click."]
        ManualCpc,
        #[doc = "Manual impression based bidding where user pays per thousand impressions."]
        ManualCpm,
        #[doc = "A bidding strategy that pays a configurable amount per video view."]
        ManualCpv,
        #[doc = "An automated bidding strategy that automatically sets bids to maximize revenue while spending your budget."]
        MaximizeConversionValue,
        #[doc = "A bidding strategy that automatically maximizes number of conversions given a daily budget."]
        MaximizeConversions,
        #[doc = "Page-One Promoted bidding scheme, which sets max cpc bids to target impressions on page one or page one promoted slots on google.com. This enum value is deprecated."]
        PageOnePromoted,
        #[doc = "Percent Cpc is bidding strategy where bids are a fraction of the advertised price for some good or service."]
        PercentCpc,
        #[doc = "Target CPA is an automated bid strategy that sets bids to help get as many conversions as possible at the target cost-per-acquisition (CPA) you set."]
        TargetCpa,
        #[doc = "Target CPM is an automated bid strategy that sets bids to help get as many impressions as possible at the target cost per one thousand impressions (CPM) you set."]
        TargetCpm,
        #[doc = "An automated bidding strategy that sets bids so that a certain percentage of search ads are shown at the top of the first page (or other targeted location)."]
        TargetImpressionShare,
        #[doc = "Target Outrank Share is an automated bidding strategy that sets bids based on the target fraction of auctions where the advertiser should outrank a specific competitor. This enum value is deprecated."]
        TargetOutrankShare,
        #[doc = "Target ROAS is an automated bidding strategy that helps you maximize revenue while averaging a specific target Return On Average Spend (ROAS)."]
        TargetRoas,
        #[doc = "Target Spend is an automated bid strategy that sets your bids to help get as many clicks as possible within your budget."]
        TargetSpend,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesBiddingStrategyType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::Commission => "COMMISSION",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::EnhancedCpc => "ENHANCED_CPC",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::Invalid => "INVALID",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::ManualCpa => "MANUAL_CPA",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::ManualCpc => "MANUAL_CPC",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::ManualCpm => "MANUAL_CPM",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::ManualCpv => "MANUAL_CPV",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::MaximizeConversionValue => {
                    "MAXIMIZE_CONVERSION_VALUE"
                }
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::MaximizeConversions => {
                    "MAXIMIZE_CONVERSIONS"
                }
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::PageOnePromoted => {
                    "PAGE_ONE_PROMOTED"
                }
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::PercentCpc => "PERCENT_CPC",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetCpa => "TARGET_CPA",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetCpm => "TARGET_CPM",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetImpressionShare => {
                    "TARGET_IMPRESSION_SHARE"
                }
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetOutrankShare => {
                    "TARGET_OUTRANK_SHARE"
                }
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetRoas => "TARGET_ROAS",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetSpend => "TARGET_SPEND",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesBiddingStrategyType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesBiddingStrategyType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesBiddingStrategyType, ()>
        {
            Ok(match s {
                "COMMISSION" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::Commission,
                "ENHANCED_CPC" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::EnhancedCpc,
                "INVALID" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::Invalid,
                "MANUAL_CPA" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::ManualCpa,
                "MANUAL_CPC" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::ManualCpc,
                "MANUAL_CPM" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::ManualCpm,
                "MANUAL_CPV" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::ManualCpv,
                "MAXIMIZE_CONVERSION_VALUE" => {
                    GoogleAdsSearchads360V0ResourcesBiddingStrategyType::MaximizeConversionValue
                }
                "MAXIMIZE_CONVERSIONS" => {
                    GoogleAdsSearchads360V0ResourcesBiddingStrategyType::MaximizeConversions
                }
                "PAGE_ONE_PROMOTED" => {
                    GoogleAdsSearchads360V0ResourcesBiddingStrategyType::PageOnePromoted
                }
                "PERCENT_CPC" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::PercentCpc,
                "TARGET_CPA" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetCpa,
                "TARGET_CPM" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetCpm,
                "TARGET_IMPRESSION_SHARE" => {
                    GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetImpressionShare
                }
                "TARGET_OUTRANK_SHARE" => {
                    GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetOutrankShare
                }
                "TARGET_ROAS" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetRoas,
                "TARGET_SPEND" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetSpend,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesBiddingStrategyType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesBiddingStrategyType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesBiddingStrategyType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMMISSION" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::Commission,
                "ENHANCED_CPC" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::EnhancedCpc,
                "INVALID" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::Invalid,
                "MANUAL_CPA" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::ManualCpa,
                "MANUAL_CPC" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::ManualCpc,
                "MANUAL_CPM" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::ManualCpm,
                "MANUAL_CPV" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::ManualCpv,
                "MAXIMIZE_CONVERSION_VALUE" => {
                    GoogleAdsSearchads360V0ResourcesBiddingStrategyType::MaximizeConversionValue
                }
                "MAXIMIZE_CONVERSIONS" => {
                    GoogleAdsSearchads360V0ResourcesBiddingStrategyType::MaximizeConversions
                }
                "PAGE_ONE_PROMOTED" => {
                    GoogleAdsSearchads360V0ResourcesBiddingStrategyType::PageOnePromoted
                }
                "PERCENT_CPC" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::PercentCpc,
                "TARGET_CPA" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetCpa,
                "TARGET_CPM" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetCpm,
                "TARGET_IMPRESSION_SHARE" => {
                    GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetImpressionShare
                }
                "TARGET_OUTRANK_SHARE" => {
                    GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetOutrankShare
                }
                "TARGET_ROAS" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetRoas,
                "TARGET_SPEND" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::TargetSpend,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesBiddingStrategyType::Unspecified,
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
        for GoogleAdsSearchads360V0ResourcesBiddingStrategyType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesBiddingStrategyType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus {
        #[doc = "The bidding strategy is enabled."]
        Enabled,
        #[doc = "The bidding strategy is removed."]
        Removed,
        #[doc = "The received value is not known in this version. This is a response-only value."]
        Unknown,
        #[doc = "No value has been specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus::Enabled => "ENABLED",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus::Removed => "REMOVED",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus, ()>
        {
            Ok(match s {
                "ENABLED" => GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus::Enabled,
                "REMOVED" => GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus::Removed,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ENABLED" => GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus::Enabled,
                "REMOVED" => GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus::Removed,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus::Unspecified,
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
        for GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesBiddingStrategyStatus
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0ResourcesCampaign { # [doc = "The ad serving optimization status of the campaign."] # [serde (rename = "adServingOptimizationStatus" , default , skip_serializing_if = "std::option::Option::is_none")] pub ad_serving_optimization_status : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus > , # [doc = "Immutable. Optional refinement to `advertising_channel_type`. Must be a valid sub-type of the parent channel type. Can be set only when creating campaigns. After campaign is created, the field can not be changed."] # [serde (rename = "advertisingChannelSubType" , default , skip_serializing_if = "std::option::Option::is_none")] pub advertising_channel_sub_type : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType > , # [doc = "Immutable. The primary serving target for ads within the campaign. The targeting options can be refined in `network_settings`. This field is required and should not be empty when creating new campaigns. Can be set only when creating campaigns. After the campaign is created, the field can not be changed."] # [serde (rename = "advertisingChannelType" , default , skip_serializing_if = "std::option::Option::is_none")] pub advertising_channel_type : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType > , # [doc = "Portfolio bidding strategy used by campaign."] # [serde (rename = "biddingStrategy" , default , skip_serializing_if = "std::option::Option::is_none")] pub bidding_strategy : :: std :: option :: Option < String > , # [doc = "Output only. The system status of the campaign’s bidding strategy."] # [serde (rename = "biddingStrategySystemStatus" , default , skip_serializing_if = "std::option::Option::is_none")] pub bidding_strategy_system_status : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus > , # [doc = "Output only. The type of bidding strategy. A bidding strategy can be created by setting either the bidding scheme to create a standard bidding strategy or the `bidding_strategy` field to create a portfolio bidding strategy. This field is read-only."] # [serde (rename = "biddingStrategyType" , default , skip_serializing_if = "std::option::Option::is_none")] pub bidding_strategy_type : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType > , # [doc = "The budget of the campaign."] # [serde (rename = "campaignBudget" , default , skip_serializing_if = "std::option::Option::is_none")] pub campaign_budget : :: std :: option :: Option < String > , # [doc = "Output only. Timestamp of the campaign’s creation time, formatted in ISO 8601."] # [serde (rename = "createTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub create_time : :: std :: option :: Option < String > , # [doc = "The setting for controlling Dynamic Search Ads (DSA)."] # [serde (rename = "dynamicSearchAdsSetting" , default , skip_serializing_if = "std::option::Option::is_none")] pub dynamic_search_ads_setting : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignDynamicSearchAdsSetting > , # [doc = "The last day of the campaign in serving customer’s timezone in YYYY-MM-DD format. On create, defaults to 2037-12-30, which means the campaign will run indefinitely. To set an existing campaign to run indefinitely, set this field to 2037-12-30."] # [serde (rename = "endDate" , default , skip_serializing_if = "std::option::Option::is_none")] pub end_date : :: std :: option :: Option < String > , # [doc = "Output only. ID of the campaign in the external engine account. This field is for non-Google Ads account only, for example, Yahoo Japan, Microsoft, Baidu etc. For Google Ads entity, use “campaign.id” instead."] # [serde (rename = "engineId" , default , skip_serializing_if = "std::option::Option::is_none")] pub engine_id : :: std :: option :: Option < String > , # [doc = "The asset field types that should be excluded from this campaign. Asset links with these field types will not be inherited by this campaign from the upper level."] # [serde (rename = "excludedParentAssetFieldTypes" , default , skip_serializing_if = "std::option::Option::is_none")] pub excluded_parent_asset_field_types : :: std :: option :: Option < Vec < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems > > , # [doc = "Suffix used to append query parameters to landing pages that are served with parallel tracking."] # [serde (rename = "finalUrlSuffix" , default , skip_serializing_if = "std::option::Option::is_none")] pub final_url_suffix : :: std :: option :: Option < String > , # [doc = "A list that limits how often each user will see this campaign’s ads."] # [serde (rename = "frequencyCaps" , default , skip_serializing_if = "std::option::Option::is_none")] pub frequency_caps : :: std :: option :: Option < Vec < crate :: schemas :: GoogleAdsSearchads360V0CommonFrequencyCapEntry > > , # [doc = "The setting for ads geotargeting."] # [serde (rename = "geoTargetTypeSetting" , default , skip_serializing_if = "std::option::Option::is_none")] pub geo_target_type_setting : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSetting > , # [doc = "Output only. The ID of the campaign."] # [serde (rename = "id" , default , skip_serializing_if = "std::option::Option::is_none")] # [serde (with = "crate::parsed_string")] pub id : :: std :: option :: Option < i64 > , # [doc = "Output only. The resource names of labels attached to this campaign."] # [serde (rename = "labels" , default , skip_serializing_if = "std::option::Option::is_none")] pub labels : :: std :: option :: Option < Vec < String > > , # [doc = "Output only. The datetime when this campaign was last modified. The datetime is in the customer’s time zone and in “yyyy-MM-dd HH:mm:ss.ssssss” format."] # [serde (rename = "lastModifiedTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub last_modified_time : :: std :: option :: Option < String > , # [doc = "Standard Manual CPA bidding strategy. Manual bidding strategy that allows advertiser to set the bid per advertiser-specified action. Supported only for Local Services campaigns."] # [serde (rename = "manualCpa" , default , skip_serializing_if = "std::option::Option::is_none")] pub manual_cpa : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0CommonManualCpa > , # [doc = "Standard Manual CPC bidding strategy. Manual click-based bidding where user pays per click."] # [serde (rename = "manualCpc" , default , skip_serializing_if = "std::option::Option::is_none")] pub manual_cpc : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0CommonManualCpc > , # [doc = "Standard Manual CPM bidding strategy. Manual impression-based bidding where user pays per thousand impressions."] # [serde (rename = "manualCpm" , default , skip_serializing_if = "std::option::Option::is_none")] pub manual_cpm : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0CommonManualCpm > , # [doc = "Standard Maximize Conversion Value bidding strategy that automatically sets bids to maximize revenue while spending your budget."] # [serde (rename = "maximizeConversionValue" , default , skip_serializing_if = "std::option::Option::is_none")] pub maximize_conversion_value : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0CommonMaximizeConversionValue > , # [doc = "Standard Maximize Conversions bidding strategy that automatically maximizes number of conversions while spending your budget."] # [serde (rename = "maximizeConversions" , default , skip_serializing_if = "std::option::Option::is_none")] pub maximize_conversions : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0CommonMaximizeConversions > , # [doc = "The name of the campaign. This field is required and should not be empty when creating new campaigns. It must not contain any null (code point 0x0), NL line feed (code point 0xA) or carriage return (code point 0xD) characters."] # [serde (rename = "name" , default , skip_serializing_if = "std::option::Option::is_none")] pub name : :: std :: option :: Option < String > , # [doc = "The network settings for the campaign."] # [serde (rename = "networkSettings" , default , skip_serializing_if = "std::option::Option::is_none")] pub network_settings : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignNetworkSettings > , # [doc = "Optimization goal setting for this campaign, which includes a set of optimization goal types."] # [serde (rename = "optimizationGoalSetting" , default , skip_serializing_if = "std::option::Option::is_none")] pub optimization_goal_setting : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSetting > , # [doc = "Standard Percent Cpc bidding strategy where bids are a fraction of the advertised price for some good or service."] # [serde (rename = "percentCpc" , default , skip_serializing_if = "std::option::Option::is_none")] pub percent_cpc : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0CommonPercentCpc > , # [doc = "Settings for Real-Time Bidding, a feature only available for campaigns targeting the Ad Exchange network."] # [serde (rename = "realTimeBiddingSetting" , default , skip_serializing_if = "std::option::Option::is_none")] pub real_time_bidding_setting : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0CommonRealTimeBiddingSetting > , # [doc = "Immutable. The resource name of the campaign. Campaign resource names have the form: `customers/{customer_id}/campaigns/{campaign_id}`"] # [serde (rename = "resourceName" , default , skip_serializing_if = "std::option::Option::is_none")] pub resource_name : :: std :: option :: Option < String > , # [doc = "Selective optimization setting for this campaign, which includes a set of conversion actions to optimize this campaign towards."] # [serde (rename = "selectiveOptimization" , default , skip_serializing_if = "std::option::Option::is_none")] pub selective_optimization : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignSelectiveOptimization > , # [doc = "Output only. The ad serving status of the campaign."] # [serde (rename = "servingStatus" , default , skip_serializing_if = "std::option::Option::is_none")] pub serving_status : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignServingStatus > , # [doc = "The setting for controlling Shopping campaigns."] # [serde (rename = "shoppingSetting" , default , skip_serializing_if = "std::option::Option::is_none")] pub shopping_setting : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignShoppingSetting > , # [doc = "The date when campaign started in serving customer’s timezone in YYYY-MM-DD format."] # [serde (rename = "startDate" , default , skip_serializing_if = "std::option::Option::is_none")] pub start_date : :: std :: option :: Option < String > , # [doc = "The status of the campaign. When a new campaign is added, the status defaults to ENABLED."] # [serde (rename = "status" , default , skip_serializing_if = "std::option::Option::is_none")] pub status : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignStatus > , # [doc = "Standard Target CPA bidding strategy that automatically sets bids to help get as many conversions as possible at the target cost-per-acquisition (CPA) you set."] # [serde (rename = "targetCpa" , default , skip_serializing_if = "std::option::Option::is_none")] pub target_cpa : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0CommonTargetCpa > , # [doc = "A bidding strategy that automatically optimizes cost per thousand impressions."] # [serde (rename = "targetCpm" , default , skip_serializing_if = "std::option::Option::is_none")] pub target_cpm : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0CommonTargetCpm > , # [doc = "Target Impression Share bidding strategy. An automated bidding strategy that sets bids to achieve a chosen percentage of impressions."] # [serde (rename = "targetImpressionShare" , default , skip_serializing_if = "std::option::Option::is_none")] pub target_impression_share : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0CommonTargetImpressionShare > , # [doc = "Standard Target ROAS bidding strategy that automatically maximizes revenue while averaging a specific target return on ad spend (ROAS)."] # [serde (rename = "targetRoas" , default , skip_serializing_if = "std::option::Option::is_none")] pub target_roas : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0CommonTargetRoas > , # [doc = "Standard Target Spend bidding strategy that automatically sets your bids to help get as many clicks as possible within your budget."] # [serde (rename = "targetSpend" , default , skip_serializing_if = "std::option::Option::is_none")] pub target_spend : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0CommonTargetSpend > , # [doc = "Output only. Campaign-level settings for tracking information."] # [serde (rename = "trackingSetting" , default , skip_serializing_if = "std::option::Option::is_none")] pub tracking_setting : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignTrackingSetting > , # [doc = "The URL template for constructing a tracking URL."] # [serde (rename = "trackingUrlTemplate" , default , skip_serializing_if = "std::option::Option::is_none")] pub tracking_url_template : :: std :: option :: Option < String > , # [doc = "The list of mappings used to substitute custom parameter tags in a `tracking_url_template`, `final_urls`, or `mobile_final_urls`."] # [serde (rename = "urlCustomParameters" , default , skip_serializing_if = "std::option::Option::is_none")] pub url_custom_parameters : :: std :: option :: Option < Vec < crate :: schemas :: GoogleAdsSearchads360V0CommonCustomParameter > > , # [doc = "Represents opting out of URL expansion to more targeted URLs. If opted out (true), only the final URLs in the asset group or URLs specified in the advertiser’s Google Merchant Center or business data feeds are targeted. If opted in (false), the entire domain will be targeted. This field can only be set for Performance Max campaigns, where the default value is false."] # [serde (rename = "urlExpansionOptOut" , default , skip_serializing_if = "std::option::Option::is_none")] pub url_expansion_opt_out : :: std :: option :: Option < bool > , }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesCampaign {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesCampaign {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus {
        #[doc = "Ad serving is optimized based on CTR * Conversion for the campaign. If the campaign is not in the conversion optimizer bidding strategy, it will default to OPTIMIZED."]
        ConversionOptimize,
        #[doc = "Ad serving is optimized based on CTR for the campaign."]
        Optimize,
        #[doc = "Ads are rotated evenly for 90 days, then optimized for clicks."]
        Rotate,
        #[doc = "Show lower performing ads more evenly with higher performing ads, and do not optimize."]
        RotateIndefinitely,
        #[doc = "Ad serving optimization status is not available."]
        Unavailable,
        #[doc = "The received value is not known in this version. This is a response-only value."]
        Unknown,
        #[doc = "No value has been specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: ConversionOptimize => "CONVERSION_OPTIMIZE" , GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: Optimize => "OPTIMIZE" , GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: Rotate => "ROTATE" , GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: RotateIndefinitely => "ROTATE_INDEFINITELY" , GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: Unavailable => "UNAVAILABLE" , GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus,
            (),
        > {
            Ok (match s { "CONVERSION_OPTIMIZE" => GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: ConversionOptimize , "OPTIMIZE" => GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: Optimize , "ROTATE" => GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: Rotate , "ROTATE_INDEFINITELY" => GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: RotateIndefinitely , "UNAVAILABLE" => GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: Unavailable , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: Unspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CONVERSION_OPTIMIZE" => GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: ConversionOptimize , "OPTIMIZE" => GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: Optimize , "ROTATE" => GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: Rotate , "ROTATE_INDEFINITELY" => GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: RotateIndefinitely , "UNAVAILABLE" => GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: Unavailable , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignAdServingOptimizationStatus
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType {
        #[doc = "App Campaign that lets you easily promote your Android or iOS app across Google’s top properties including Search, Play, YouTube, and the Google Display Network."]
        AppCampaign,
        #[doc = "App Campaign for engagement, focused on driving re-engagement with the app across several of Google’s top properties including Search, YouTube, and the Google Display Network."]
        AppCampaignForEngagement,
        #[doc = "App Campaign for pre registration, specialized for advertising mobile app pre-registration, that targets multiple advertising channels across Google Play, YouTube and Display Network."]
        AppCampaignForPreRegistration,
        #[doc = "AdWords Express campaigns for display."]
        DisplayExpress,
        #[doc = "Gmail Ad campaigns."]
        DisplayGmailAd,
        #[doc = "Mobile app campaigns for Display."]
        DisplayMobileApp,
        #[doc = "Smart display campaigns."]
        DisplaySmartCampaign,
        #[doc = "Campaigns specialized for local advertising."]
        LocalCampaign,
        #[doc = "AdWords express campaigns for search."]
        SearchExpress,
        #[doc = "Mobile app campaigns for Search."]
        SearchMobileApp,
        #[doc = "Shopping Comparison Listing campaigns."]
        ShoppingComparisonListingAds,
        #[doc = "Smart Shopping campaigns."]
        ShoppingSmartAds,
        #[doc = "Standard Smart campaigns."]
        SmartCampaign,
        #[doc = "Used as a return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
        #[doc = "Video TrueView for Action campaigns."]
        VideoAction,
        #[doc = "Video campaigns with non-skippable video ads."]
        VideoNonSkippable,
        #[doc = "Video Outstream campaigns."]
        VideoOutstream,
        #[doc = "Video reach campaign with Target Frequency bidding strategy."]
        VideoReachTargetFrequency,
        #[doc = "Video campaigns with sequence video ads."]
        VideoSequence,
    }
    impl GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: AppCampaign => "APP_CAMPAIGN" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: AppCampaignForEngagement => "APP_CAMPAIGN_FOR_ENGAGEMENT" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: AppCampaignForPreRegistration => "APP_CAMPAIGN_FOR_PRE_REGISTRATION" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: DisplayExpress => "DISPLAY_EXPRESS" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: DisplayGmailAd => "DISPLAY_GMAIL_AD" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: DisplayMobileApp => "DISPLAY_MOBILE_APP" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: DisplaySmartCampaign => "DISPLAY_SMART_CAMPAIGN" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: LocalCampaign => "LOCAL_CAMPAIGN" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: SearchExpress => "SEARCH_EXPRESS" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: SearchMobileApp => "SEARCH_MOBILE_APP" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: ShoppingComparisonListingAds => "SHOPPING_COMPARISON_LISTING_ADS" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: ShoppingSmartAds => "SHOPPING_SMART_ADS" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: SmartCampaign => "SMART_CAMPAIGN" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: Unspecified => "UNSPECIFIED" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: VideoAction => "VIDEO_ACTION" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: VideoNonSkippable => "VIDEO_NON_SKIPPABLE" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: VideoOutstream => "VIDEO_OUTSTREAM" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: VideoReachTargetFrequency => "VIDEO_REACH_TARGET_FREQUENCY" , GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: VideoSequence => "VIDEO_SEQUENCE" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType,
            (),
        > {
            Ok (match s { "APP_CAMPAIGN" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: AppCampaign , "APP_CAMPAIGN_FOR_ENGAGEMENT" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: AppCampaignForEngagement , "APP_CAMPAIGN_FOR_PRE_REGISTRATION" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: AppCampaignForPreRegistration , "DISPLAY_EXPRESS" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: DisplayExpress , "DISPLAY_GMAIL_AD" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: DisplayGmailAd , "DISPLAY_MOBILE_APP" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: DisplayMobileApp , "DISPLAY_SMART_CAMPAIGN" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: DisplaySmartCampaign , "LOCAL_CAMPAIGN" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: LocalCampaign , "SEARCH_EXPRESS" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: SearchExpress , "SEARCH_MOBILE_APP" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: SearchMobileApp , "SHOPPING_COMPARISON_LISTING_ADS" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: ShoppingComparisonListingAds , "SHOPPING_SMART_ADS" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: ShoppingSmartAds , "SMART_CAMPAIGN" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: SmartCampaign , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: Unspecified , "VIDEO_ACTION" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: VideoAction , "VIDEO_NON_SKIPPABLE" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: VideoNonSkippable , "VIDEO_OUTSTREAM" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: VideoOutstream , "VIDEO_REACH_TARGET_FREQUENCY" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: VideoReachTargetFrequency , "VIDEO_SEQUENCE" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: VideoSequence , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "APP_CAMPAIGN" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: AppCampaign , "APP_CAMPAIGN_FOR_ENGAGEMENT" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: AppCampaignForEngagement , "APP_CAMPAIGN_FOR_PRE_REGISTRATION" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: AppCampaignForPreRegistration , "DISPLAY_EXPRESS" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: DisplayExpress , "DISPLAY_GMAIL_AD" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: DisplayGmailAd , "DISPLAY_MOBILE_APP" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: DisplayMobileApp , "DISPLAY_SMART_CAMPAIGN" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: DisplaySmartCampaign , "LOCAL_CAMPAIGN" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: LocalCampaign , "SEARCH_EXPRESS" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: SearchExpress , "SEARCH_MOBILE_APP" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: SearchMobileApp , "SHOPPING_COMPARISON_LISTING_ADS" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: ShoppingComparisonListingAds , "SHOPPING_SMART_ADS" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: ShoppingSmartAds , "SMART_CAMPAIGN" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: SmartCampaign , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: Unspecified , "VIDEO_ACTION" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: VideoAction , "VIDEO_NON_SKIPPABLE" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: VideoNonSkippable , "VIDEO_OUTSTREAM" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: VideoOutstream , "VIDEO_REACH_TARGET_FREQUENCY" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: VideoReachTargetFrequency , "VIDEO_SEQUENCE" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType :: VideoSequence , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelSubType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType {
        #[doc = "Discovery campaigns."]
        Discovery,
        #[doc = "Google Display Network only."]
        Display,
        #[doc = "Hotel Ads campaigns."]
        Hotel,
        #[doc = "Local ads campaigns."]
        Local,
        #[doc = "Local services campaigns."]
        LocalServices,
        #[doc = "App Campaigns, and App Campaigns for Engagement, that run across multiple channels."]
        MultiChannel,
        #[doc = "Performance Max campaigns."]
        PerformanceMax,
        #[doc = "Search Network. Includes display bundled, and Search+ campaigns."]
        Search,
        #[doc = "Shopping campaigns serve on the shopping property and on google.com search results."]
        Shopping,
        #[doc = "Smart campaigns."]
        Smart,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
        #[doc = "Video campaigns."]
        Video,
    }
    impl GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Discovery => {
                    "DISCOVERY"
                }
                GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Display => {
                    "DISPLAY"
                }
                GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Hotel => "HOTEL",
                GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Local => "LOCAL",
                GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::LocalServices => {
                    "LOCAL_SERVICES"
                }
                GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::MultiChannel => {
                    "MULTI_CHANNEL"
                }
                GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::PerformanceMax => {
                    "PERFORMANCE_MAX"
                }
                GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Search => "SEARCH",
                GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Shopping => {
                    "SHOPPING"
                }
                GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Smart => "SMART",
                GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Unknown => {
                    "UNKNOWN"
                }
                GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Unspecified => {
                    "UNSPECIFIED"
                }
                GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Video => "VIDEO",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType, ()>
        {
            Ok(match s {
                "DISCOVERY" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Discovery
                }
                "DISPLAY" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Display
                }
                "HOTEL" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Hotel,
                "LOCAL" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Local,
                "LOCAL_SERVICES" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::LocalServices
                }
                "MULTI_CHANNEL" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::MultiChannel
                }
                "PERFORMANCE_MAX" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::PerformanceMax
                }
                "SEARCH" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Search,
                "SHOPPING" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Shopping
                }
                "SMART" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Smart,
                "UNKNOWN" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Unknown
                }
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Unspecified
                }
                "VIDEO" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Video,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DISCOVERY" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Discovery
                }
                "DISPLAY" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Display
                }
                "HOTEL" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Hotel,
                "LOCAL" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Local,
                "LOCAL_SERVICES" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::LocalServices
                }
                "MULTI_CHANNEL" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::MultiChannel
                }
                "PERFORMANCE_MAX" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::PerformanceMax
                }
                "SEARCH" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Search,
                "SHOPPING" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Shopping
                }
                "SMART" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Smart,
                "UNKNOWN" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Unknown
                }
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Unspecified
                }
                "VIDEO" => GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType::Video,
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
        for GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignAdvertisingChannelType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus {
        #[doc = "The bid strategy is active, and AdWords cannot find any specific issues with the strategy."]
        Enabled,
        #[doc = "The bid strategy is learning because of a recent budget change."]
        LearningBudgetChange,
        #[doc = "The bid strategy is learning because of recent change in number of campaigns, ad groups or keywords attached to it."]
        LearningCompositionChange,
        #[doc = "The bid strategy depends on conversion reporting and the customer recently changed their conversion settings."]
        LearningConversionSettingChange,
        #[doc = "The bid strategy depends on conversion reporting and the customer recently modified conversion types that were relevant to the bid strategy."]
        LearningConversionTypeChange,
        #[doc = "The bid strategy is learning because it has been recently created or recently reactivated."]
        LearningNew,
        #[doc = "The bid strategy is learning because of a recent setting change."]
        LearningSettingChange,
        #[doc = "A significant fraction of keywords in this bid strategy are limited by budget."]
        LimitedByBudget,
        #[doc = "The bid strategy is limited by its bid ceiling."]
        LimitedByCpcBidCeiling,
        #[doc = "The bid strategy is limited by its bid floor."]
        LimitedByCpcBidFloor,
        #[doc = "The bid strategy is limited because there was not enough conversion traffic over the past weeks."]
        LimitedByData,
        #[doc = "The bid strategy cannot fully spend its budget because of narrow targeting."]
        LimitedByInventory,
        #[doc = "The bid strategy cannot reach its target spend because its spend has been de-prioritized."]
        LimitedByLowPrioritySpend,
        #[doc = "A significant fraction of keywords in this bid strategy have a low Quality Score."]
        LimitedByLowQuality,
        #[doc = "The bid strategy depends on conversion reporting and the customer’s conversion settings are misconfigured."]
        MisconfiguredConversionSettings,
        #[doc = "The bid strategy depends on conversion reporting and the customer is lacking conversion types that might be reported against this strategy."]
        MisconfiguredConversionTypes,
        #[doc = "There are campaigns outside the bid strategy that share budgets with campaigns included in the strategy."]
        MisconfiguredSharedBudget,
        #[doc = "The campaign has an invalid strategy type and is not serving."]
        MisconfiguredStrategyType,
        #[doc = "Missing conversion tracking (no pings present) and/or remarketing lists for SSC."]
        MisconfiguredZeroEligibility,
        #[doc = "There were multiple system statuses for this bid strategy during the time in question."]
        Multiple,
        #[doc = "There were multiple LEARNING\\_\\* system statuses for this bid strategy during the time in question."]
        MultipleLearning,
        #[doc = "There were multiple LIMITED\\_\\* system statuses for this bid strategy during the time in question."]
        MultipleLimited,
        #[doc = "There were multiple MISCONFIGURED\\_\\* system statuses for this bid strategy during the time in question."]
        MultipleMisconfigured,
        #[doc = "The bid strategy is not active. Either there are no active campaigns, ad groups or keywords attached to the bid strategy. Or there are no active budgets connected to the bid strategy."]
        Paused,
        #[doc = "This bid strategy currently does not support status reporting."]
        Unavailable,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Signals that an unexpected error occurred, for example, no bidding strategy type was found, or no status information was found."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Enabled => "ENABLED" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningBudgetChange => "LEARNING_BUDGET_CHANGE" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningCompositionChange => "LEARNING_COMPOSITION_CHANGE" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningConversionSettingChange => "LEARNING_CONVERSION_SETTING_CHANGE" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningConversionTypeChange => "LEARNING_CONVERSION_TYPE_CHANGE" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningNew => "LEARNING_NEW" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningSettingChange => "LEARNING_SETTING_CHANGE" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByBudget => "LIMITED_BY_BUDGET" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByCpcBidCeiling => "LIMITED_BY_CPC_BID_CEILING" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByCpcBidFloor => "LIMITED_BY_CPC_BID_FLOOR" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByData => "LIMITED_BY_DATA" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByInventory => "LIMITED_BY_INVENTORY" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByLowPrioritySpend => "LIMITED_BY_LOW_PRIORITY_SPEND" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByLowQuality => "LIMITED_BY_LOW_QUALITY" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MisconfiguredConversionSettings => "MISCONFIGURED_CONVERSION_SETTINGS" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MisconfiguredConversionTypes => "MISCONFIGURED_CONVERSION_TYPES" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MisconfiguredSharedBudget => "MISCONFIGURED_SHARED_BUDGET" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MisconfiguredStrategyType => "MISCONFIGURED_STRATEGY_TYPE" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MisconfiguredZeroEligibility => "MISCONFIGURED_ZERO_ELIGIBILITY" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Multiple => "MULTIPLE" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MultipleLearning => "MULTIPLE_LEARNING" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MultipleLimited => "MULTIPLE_LIMITED" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MultipleMisconfigured => "MULTIPLE_MISCONFIGURED" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Paused => "PAUSED" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Unavailable => "UNAVAILABLE" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus,
            (),
        > {
            Ok (match s { "ENABLED" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Enabled , "LEARNING_BUDGET_CHANGE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningBudgetChange , "LEARNING_COMPOSITION_CHANGE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningCompositionChange , "LEARNING_CONVERSION_SETTING_CHANGE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningConversionSettingChange , "LEARNING_CONVERSION_TYPE_CHANGE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningConversionTypeChange , "LEARNING_NEW" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningNew , "LEARNING_SETTING_CHANGE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningSettingChange , "LIMITED_BY_BUDGET" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByBudget , "LIMITED_BY_CPC_BID_CEILING" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByCpcBidCeiling , "LIMITED_BY_CPC_BID_FLOOR" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByCpcBidFloor , "LIMITED_BY_DATA" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByData , "LIMITED_BY_INVENTORY" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByInventory , "LIMITED_BY_LOW_PRIORITY_SPEND" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByLowPrioritySpend , "LIMITED_BY_LOW_QUALITY" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByLowQuality , "MISCONFIGURED_CONVERSION_SETTINGS" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MisconfiguredConversionSettings , "MISCONFIGURED_CONVERSION_TYPES" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MisconfiguredConversionTypes , "MISCONFIGURED_SHARED_BUDGET" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MisconfiguredSharedBudget , "MISCONFIGURED_STRATEGY_TYPE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MisconfiguredStrategyType , "MISCONFIGURED_ZERO_ELIGIBILITY" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MisconfiguredZeroEligibility , "MULTIPLE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Multiple , "MULTIPLE_LEARNING" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MultipleLearning , "MULTIPLE_LIMITED" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MultipleLimited , "MULTIPLE_MISCONFIGURED" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MultipleMisconfigured , "PAUSED" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Paused , "UNAVAILABLE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Unavailable , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Unspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "ENABLED" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Enabled , "LEARNING_BUDGET_CHANGE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningBudgetChange , "LEARNING_COMPOSITION_CHANGE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningCompositionChange , "LEARNING_CONVERSION_SETTING_CHANGE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningConversionSettingChange , "LEARNING_CONVERSION_TYPE_CHANGE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningConversionTypeChange , "LEARNING_NEW" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningNew , "LEARNING_SETTING_CHANGE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LearningSettingChange , "LIMITED_BY_BUDGET" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByBudget , "LIMITED_BY_CPC_BID_CEILING" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByCpcBidCeiling , "LIMITED_BY_CPC_BID_FLOOR" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByCpcBidFloor , "LIMITED_BY_DATA" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByData , "LIMITED_BY_INVENTORY" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByInventory , "LIMITED_BY_LOW_PRIORITY_SPEND" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByLowPrioritySpend , "LIMITED_BY_LOW_QUALITY" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: LimitedByLowQuality , "MISCONFIGURED_CONVERSION_SETTINGS" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MisconfiguredConversionSettings , "MISCONFIGURED_CONVERSION_TYPES" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MisconfiguredConversionTypes , "MISCONFIGURED_SHARED_BUDGET" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MisconfiguredSharedBudget , "MISCONFIGURED_STRATEGY_TYPE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MisconfiguredStrategyType , "MISCONFIGURED_ZERO_ELIGIBILITY" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MisconfiguredZeroEligibility , "MULTIPLE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Multiple , "MULTIPLE_LEARNING" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MultipleLearning , "MULTIPLE_LIMITED" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MultipleLimited , "MULTIPLE_MISCONFIGURED" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: MultipleMisconfigured , "PAUSED" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Paused , "UNAVAILABLE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Unavailable , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategySystemStatus
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType {
        #[doc = "Commission is an automatic bidding strategy in which the advertiser pays a certain portion of the conversion value."]
        Commission,
        #[doc = "Enhanced CPC is a bidding strategy that raises bids for clicks that seem more likely to lead to a conversion and lowers them for clicks where they seem less likely."]
        EnhancedCpc,
        #[doc = "Used for return value only. Indicates that a campaign does not have a bidding strategy. This prevents the campaign from serving. For example, a campaign may be attached to a manager bidding strategy and the serving account is subsequently unlinked from the manager account. In this case the campaign will automatically be detached from the now inaccessible manager bidding strategy and transition to the INVALID bidding strategy type."]
        Invalid,
        #[doc = "Manual bidding strategy that allows advertiser to set the bid per advertiser-specified action."]
        ManualCpa,
        #[doc = "Manual click based bidding where user pays per click."]
        ManualCpc,
        #[doc = "Manual impression based bidding where user pays per thousand impressions."]
        ManualCpm,
        #[doc = "A bidding strategy that pays a configurable amount per video view."]
        ManualCpv,
        #[doc = "An automated bidding strategy that automatically sets bids to maximize revenue while spending your budget."]
        MaximizeConversionValue,
        #[doc = "A bidding strategy that automatically maximizes number of conversions given a daily budget."]
        MaximizeConversions,
        #[doc = "Page-One Promoted bidding scheme, which sets max cpc bids to target impressions on page one or page one promoted slots on google.com. This enum value is deprecated."]
        PageOnePromoted,
        #[doc = "Percent Cpc is bidding strategy where bids are a fraction of the advertised price for some good or service."]
        PercentCpc,
        #[doc = "Target CPA is an automated bid strategy that sets bids to help get as many conversions as possible at the target cost-per-acquisition (CPA) you set."]
        TargetCpa,
        #[doc = "Target CPM is an automated bid strategy that sets bids to help get as many impressions as possible at the target cost per one thousand impressions (CPM) you set."]
        TargetCpm,
        #[doc = "An automated bidding strategy that sets bids so that a certain percentage of search ads are shown at the top of the first page (or other targeted location)."]
        TargetImpressionShare,
        #[doc = "Target Outrank Share is an automated bidding strategy that sets bids based on the target fraction of auctions where the advertiser should outrank a specific competitor. This enum value is deprecated."]
        TargetOutrankShare,
        #[doc = "Target ROAS is an automated bidding strategy that helps you maximize revenue while averaging a specific target Return On Average Spend (ROAS)."]
        TargetRoas,
        #[doc = "Target Spend is an automated bid strategy that sets your bids to help get as many clicks as possible within your budget."]
        TargetSpend,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: Commission => "COMMISSION" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: EnhancedCpc => "ENHANCED_CPC" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: Invalid => "INVALID" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: ManualCpa => "MANUAL_CPA" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: ManualCpc => "MANUAL_CPC" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: ManualCpm => "MANUAL_CPM" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: ManualCpv => "MANUAL_CPV" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: MaximizeConversionValue => "MAXIMIZE_CONVERSION_VALUE" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: MaximizeConversions => "MAXIMIZE_CONVERSIONS" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: PageOnePromoted => "PAGE_ONE_PROMOTED" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: PercentCpc => "PERCENT_CPC" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetCpa => "TARGET_CPA" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetCpm => "TARGET_CPM" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetImpressionShare => "TARGET_IMPRESSION_SHARE" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetOutrankShare => "TARGET_OUTRANK_SHARE" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetRoas => "TARGET_ROAS" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetSpend => "TARGET_SPEND" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType, ()>
        {
            Ok (match s { "COMMISSION" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: Commission , "ENHANCED_CPC" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: EnhancedCpc , "INVALID" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: Invalid , "MANUAL_CPA" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: ManualCpa , "MANUAL_CPC" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: ManualCpc , "MANUAL_CPM" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: ManualCpm , "MANUAL_CPV" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: ManualCpv , "MAXIMIZE_CONVERSION_VALUE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: MaximizeConversionValue , "MAXIMIZE_CONVERSIONS" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: MaximizeConversions , "PAGE_ONE_PROMOTED" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: PageOnePromoted , "PERCENT_CPC" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: PercentCpc , "TARGET_CPA" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetCpa , "TARGET_CPM" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetCpm , "TARGET_IMPRESSION_SHARE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetImpressionShare , "TARGET_OUTRANK_SHARE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetOutrankShare , "TARGET_ROAS" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetRoas , "TARGET_SPEND" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetSpend , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: Unspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "COMMISSION" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: Commission , "ENHANCED_CPC" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: EnhancedCpc , "INVALID" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: Invalid , "MANUAL_CPA" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: ManualCpa , "MANUAL_CPC" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: ManualCpc , "MANUAL_CPM" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: ManualCpm , "MANUAL_CPV" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: ManualCpv , "MAXIMIZE_CONVERSION_VALUE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: MaximizeConversionValue , "MAXIMIZE_CONVERSIONS" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: MaximizeConversions , "PAGE_ONE_PROMOTED" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: PageOnePromoted , "PERCENT_CPC" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: PercentCpc , "TARGET_CPA" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetCpa , "TARGET_CPM" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetCpm , "TARGET_IMPRESSION_SHARE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetImpressionShare , "TARGET_OUTRANK_SHARE" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetOutrankShare , "TARGET_ROAS" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetRoas , "TARGET_SPEND" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: TargetSpend , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignBiddingStrategyType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems {
        #[doc = "The asset is linked for use to select an ad image."]
        AdImage,
        #[doc = "The asset is linked to indicate that a hotels campaign is “Book on Google” enabled."]
        BookOnGoogle,
        #[doc = "The asset is linked for use as a business name."]
        BusinessName,
        #[doc = "The asset is linked for use as a Call extension."]
        Call,
        #[doc = "The asset is linked for use to select a call-to-action."]
        CallToActionSelection,
        #[doc = "The asset is linked for use as a Callout extension."]
        Callout,
        #[doc = "The asset is linked for use as a description."]
        Description,
        #[doc = "The asset is linked for use as a headline."]
        Headline,
        #[doc = "The asset is linked for use as a Hotel Callout extension."]
        HotelCallout,
        #[doc = "The asset is linked for use as a landscape logo."]
        LandscapeLogo,
        #[doc = "The asset is linked for use as a Lead Form extension."]
        LeadForm,
        #[doc = "The asset is linked for use as a logo."]
        Logo,
        #[doc = "The asset is linked for use as a long headline."]
        LongHeadline,
        #[doc = "The asset is linked for use as mandatory ad text."]
        MandatoryAdText,
        #[doc = "The asset is linked for use as a marketing image."]
        MarketingImage,
        #[doc = "The asset is linked for use as a media bundle."]
        MediaBundle,
        #[doc = "The asset is linked for use as a Mobile App extension."]
        MobileApp,
        #[doc = "The asset is linked for use as a portrait marketing image."]
        PortraitMarketingImage,
        #[doc = "The asset is linked for use as a Price extension."]
        Price,
        #[doc = "The asset is linked for use as a Promotion extension."]
        Promotion,
        #[doc = "The asset is linked for use as a Sitelink extension."]
        Sitelink,
        #[doc = "The asset is linked for use as a square marketing image."]
        SquareMarketingImage,
        #[doc = "The asset is linked for use as a Structured Snippet extension."]
        StructuredSnippet,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
        #[doc = "The asset is linked for use as a non YouTube logo."]
        Video,
        #[doc = "The asset is linked for use as a YouTube video."]
        YoutubeVideo,
    }
    impl GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: AdImage => "AD_IMAGE" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: BookOnGoogle => "BOOK_ON_GOOGLE" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: BusinessName => "BUSINESS_NAME" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Call => "CALL" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: CallToActionSelection => "CALL_TO_ACTION_SELECTION" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Callout => "CALLOUT" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Description => "DESCRIPTION" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Headline => "HEADLINE" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: HotelCallout => "HOTEL_CALLOUT" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: LandscapeLogo => "LANDSCAPE_LOGO" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: LeadForm => "LEAD_FORM" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Logo => "LOGO" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: LongHeadline => "LONG_HEADLINE" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: MandatoryAdText => "MANDATORY_AD_TEXT" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: MarketingImage => "MARKETING_IMAGE" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: MediaBundle => "MEDIA_BUNDLE" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: MobileApp => "MOBILE_APP" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: PortraitMarketingImage => "PORTRAIT_MARKETING_IMAGE" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Price => "PRICE" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Promotion => "PROMOTION" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Sitelink => "SITELINK" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: SquareMarketingImage => "SQUARE_MARKETING_IMAGE" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: StructuredSnippet => "STRUCTURED_SNIPPET" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Unspecified => "UNSPECIFIED" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Video => "VIDEO" , GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: YoutubeVideo => "YOUTUBE_VIDEO" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems,
            (),
        > {
            Ok (match s { "AD_IMAGE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: AdImage , "BOOK_ON_GOOGLE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: BookOnGoogle , "BUSINESS_NAME" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: BusinessName , "CALL" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Call , "CALL_TO_ACTION_SELECTION" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: CallToActionSelection , "CALLOUT" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Callout , "DESCRIPTION" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Description , "HEADLINE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Headline , "HOTEL_CALLOUT" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: HotelCallout , "LANDSCAPE_LOGO" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: LandscapeLogo , "LEAD_FORM" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: LeadForm , "LOGO" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Logo , "LONG_HEADLINE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: LongHeadline , "MANDATORY_AD_TEXT" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: MandatoryAdText , "MARKETING_IMAGE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: MarketingImage , "MEDIA_BUNDLE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: MediaBundle , "MOBILE_APP" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: MobileApp , "PORTRAIT_MARKETING_IMAGE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: PortraitMarketingImage , "PRICE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Price , "PROMOTION" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Promotion , "SITELINK" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Sitelink , "SQUARE_MARKETING_IMAGE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: SquareMarketingImage , "STRUCTURED_SNIPPET" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: StructuredSnippet , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Unspecified , "VIDEO" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Video , "YOUTUBE_VIDEO" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: YoutubeVideo , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "AD_IMAGE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: AdImage , "BOOK_ON_GOOGLE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: BookOnGoogle , "BUSINESS_NAME" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: BusinessName , "CALL" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Call , "CALL_TO_ACTION_SELECTION" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: CallToActionSelection , "CALLOUT" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Callout , "DESCRIPTION" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Description , "HEADLINE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Headline , "HOTEL_CALLOUT" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: HotelCallout , "LANDSCAPE_LOGO" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: LandscapeLogo , "LEAD_FORM" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: LeadForm , "LOGO" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Logo , "LONG_HEADLINE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: LongHeadline , "MANDATORY_AD_TEXT" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: MandatoryAdText , "MARKETING_IMAGE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: MarketingImage , "MEDIA_BUNDLE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: MediaBundle , "MOBILE_APP" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: MobileApp , "PORTRAIT_MARKETING_IMAGE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: PortraitMarketingImage , "PRICE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Price , "PROMOTION" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Promotion , "SITELINK" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Sitelink , "SQUARE_MARKETING_IMAGE" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: SquareMarketingImage , "STRUCTURED_SNIPPET" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: StructuredSnippet , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Unspecified , "VIDEO" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: Video , "YOUTUBE_VIDEO" => GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems :: YoutubeVideo , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignExcludedParentAssetFieldTypesItems
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCampaignServingStatus {
        #[doc = "Ended."]
        Ended,
        #[doc = "None."]
        None,
        #[doc = "Pending."]
        Pending,
        #[doc = "Serving."]
        Serving,
        #[doc = "Suspended."]
        Suspended,
        #[doc = "The received value is not known in this version. This is a response-only value."]
        Unknown,
        #[doc = "No value has been specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesCampaignServingStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Ended => "ENDED",
                GoogleAdsSearchads360V0ResourcesCampaignServingStatus::None => "NONE",
                GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Pending => "PENDING",
                GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Serving => "SERVING",
                GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Suspended => "SUSPENDED",
                GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesCampaignServingStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCampaignServingStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesCampaignServingStatus, ()>
        {
            Ok(match s {
                "ENDED" => GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Ended,
                "NONE" => GoogleAdsSearchads360V0ResourcesCampaignServingStatus::None,
                "PENDING" => GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Pending,
                "SERVING" => GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Serving,
                "SUSPENDED" => GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Suspended,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCampaignServingStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCampaignServingStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesCampaignServingStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ENDED" => GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Ended,
                "NONE" => GoogleAdsSearchads360V0ResourcesCampaignServingStatus::None,
                "PENDING" => GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Pending,
                "SERVING" => GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Serving,
                "SUSPENDED" => GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Suspended,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignServingStatus::Unspecified,
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
        for GoogleAdsSearchads360V0ResourcesCampaignServingStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignServingStatus
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCampaignStatus {
        #[doc = "Campaign is active and can show ads."]
        Enabled,
        #[doc = "Campaign has been paused by the user."]
        Paused,
        #[doc = "Campaign has been removed."]
        Removed,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesCampaignStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesCampaignStatus::Enabled => "ENABLED",
                GoogleAdsSearchads360V0ResourcesCampaignStatus::Paused => "PAUSED",
                GoogleAdsSearchads360V0ResourcesCampaignStatus::Removed => "REMOVED",
                GoogleAdsSearchads360V0ResourcesCampaignStatus::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesCampaignStatus::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesCampaignStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCampaignStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesCampaignStatus, ()> {
            Ok(match s {
                "ENABLED" => GoogleAdsSearchads360V0ResourcesCampaignStatus::Enabled,
                "PAUSED" => GoogleAdsSearchads360V0ResourcesCampaignStatus::Paused,
                "REMOVED" => GoogleAdsSearchads360V0ResourcesCampaignStatus::Removed,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignStatus::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignStatus::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCampaignStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCampaignStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesCampaignStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ENABLED" => GoogleAdsSearchads360V0ResourcesCampaignStatus::Enabled,
                "PAUSED" => GoogleAdsSearchads360V0ResourcesCampaignStatus::Paused,
                "REMOVED" => GoogleAdsSearchads360V0ResourcesCampaignStatus::Removed,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignStatus::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignStatus::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesCampaignStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesCampaignStatus {
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
    pub struct GoogleAdsSearchads360V0ResourcesCampaignBudget {
        #[doc = "The amount of the budget, in the local currency for the account. Amount is specified in micros, where one million is equivalent to one currency unit. Monthly spend is capped at 30.4 times this amount."]
        #[serde(
            rename = "amountMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub amount_micros: ::std::option::Option<i64>,
        #[doc = "The delivery method that determines the rate at which the campaign budget is spent. Defaults to STANDARD if unspecified in a create operation."]
        #[serde(
            rename = "deliveryMethod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delivery_method: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod,
        >,
        #[doc = "Immutable. Period over which to spend the budget. Defaults to DAILY if not specified."]
        #[serde(
            rename = "period",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub period: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod,
        >,
        #[doc = "Immutable. The resource name of the campaign budget. Campaign budget resource names have the form: `customers/{customer_id}/campaignBudgets/{campaign_budget_id}`"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesCampaignBudget {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesCampaignBudget {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod {
        #[doc = "The budget server will not throttle serving, and ads will serve as fast as possible."]
        Accelerated,
        #[doc = "The budget server will throttle serving evenly across the entire time period."]
        Standard,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod::Accelerated => {
                    "ACCELERATED"
                }
                GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod::Standard => {
                    "STANDARD"
                }
                GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod::Unspecified => {
                    "UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod, ()>
        {
            Ok(match s {
                "ACCELERATED" => {
                    GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod::Accelerated
                }
                "STANDARD" => {
                    GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod::Standard
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod::Unspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCELERATED" => {
                    GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod::Accelerated
                }
                "STANDARD" => {
                    GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod::Standard
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod::Unspecified
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
        for GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignBudgetDeliveryMethod
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod {
        #[doc = "Custom budget, added back in V5. Custom bugdet can be used with total_amount to specify lifetime budget limit."]
        CustomPeriod,
        #[doc = "Daily budget."]
        Daily,
        #[doc = "Fixed daily budget."]
        FixedDaily,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod::CustomPeriod => {
                    "CUSTOM_PERIOD"
                }
                GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod::Daily => "DAILY",
                GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod::FixedDaily => "FIXED_DAILY",
                GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod, ()>
        {
            Ok(match s {
                "CUSTOM_PERIOD" => {
                    GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod::CustomPeriod
                }
                "DAILY" => GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod::Daily,
                "FIXED_DAILY" => GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod::FixedDaily,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CUSTOM_PERIOD" => {
                    GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod::CustomPeriod
                }
                "DAILY" => GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod::Daily,
                "FIXED_DAILY" => GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod::FixedDaily,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod::Unspecified,
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
        for GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesCampaignBudgetPeriod {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0ResourcesCampaignCriterion {
        #[doc = "The modifier for the bids when the criterion matches. The modifier must be in the range: 0.1 - 10.0. Most targetable criteria types support modifiers. Use 0 to opt out of a Device type."]
        #[serde(
            rename = "bidModifier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bid_modifier: ::std::option::Option<f32>,
        #[doc = "Output only. The ID of the criterion. This field is ignored during mutate."]
        #[serde(
            rename = "criterionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub criterion_id: ::std::option::Option<i64>,
        #[doc = "Immutable. Device."]
        #[serde(
            rename = "device",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device: ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonDeviceInfo>,
        #[doc = "Output only. The display name of the criterion. This field is ignored for mutates."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Immutable. Language."]
        #[serde(
            rename = "language",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonLanguageInfo>,
        #[doc = "Immutable. Location."]
        #[serde(
            rename = "location",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonLocationInfo>,
        #[doc = "Immutable. Location Group"]
        #[serde(
            rename = "locationGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_group:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonLocationGroupInfo>,
        #[doc = "Immutable. Whether to target (`false`) or exclude (`true`) the criterion."]
        #[serde(
            rename = "negative",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub negative: ::std::option::Option<bool>,
        #[doc = "Output only. The type of the criterion."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesCampaignCriterionType,
        >,
        #[doc = "Immutable. The resource name of the campaign criterion. Campaign criterion resource names have the form: `customers/{customer_id}/campaignCriteria/{campaign_id}~{criterion_id}`"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesCampaignCriterion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesCampaignCriterion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCampaignCriterionType {
        #[doc = "Ad Schedule."]
        AdSchedule,
        #[doc = "Age range."]
        AgeRange,
        #[doc = "App payment model."]
        AppPaymentModel,
        #[doc = "Audience"]
        Audience,
        #[doc = "Carrier."]
        Carrier,
        #[doc = "Combined audience"]
        CombinedAudience,
        #[doc = "Content Label for category exclusion."]
        ContentLabel,
        #[doc = "Custom affinity."]
        CustomAffinity,
        #[doc = "Custom audience"]
        CustomAudience,
        #[doc = "Custom intent."]
        CustomIntent,
        #[doc = "Devices to target."]
        Device,
        #[doc = "Gender."]
        Gender,
        #[doc = "Income Range."]
        IncomeRange,
        #[doc = "IpBlock."]
        IpBlock,
        #[doc = "Keyword, for example, ‘mars cruise’."]
        Keyword,
        #[doc = "Smart Campaign keyword theme"]
        KeywordTheme,
        #[doc = "Language."]
        Language,
        #[doc = "Listing groups to target."]
        ListingGroup,
        #[doc = "Listing scope to target."]
        ListingScope,
        #[doc = "Google Local Services (GLS) Service ID."]
        LocalServiceId,
        #[doc = "Locations to target."]
        Location,
        #[doc = "Location group."]
        LocationGroup,
        #[doc = "Mobile application categories to target."]
        MobileAppCategory,
        #[doc = "Mobile applications to target."]
        MobileApplication,
        #[doc = "Mobile device."]
        MobileDevice,
        #[doc = "Operating system version."]
        OperatingSystemVersion,
        #[doc = "Parental status."]
        ParentalStatus,
        #[doc = "Placement, also known as Website, for example, ‘www.flowers4sale.com’"]
        Placement,
        #[doc = "Proximity."]
        Proximity,
        #[doc = "A topic target on the display network (for example, “Pets & Animals”)."]
        Topic,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
        #[doc = "A category the user is interested in."]
        UserInterest,
        #[doc = "User list."]
        UserList,
        #[doc = "Webpage criterion for dynamic search ads."]
        Webpage,
        #[doc = "YouTube Channel."]
        YoutubeChannel,
        #[doc = "YouTube Video."]
        YoutubeVideo,
    }
    impl GoogleAdsSearchads360V0ResourcesCampaignCriterionType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::AdSchedule => "AD_SCHEDULE",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::AgeRange => "AGE_RANGE",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::AppPaymentModel => {
                    "APP_PAYMENT_MODEL"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Audience => "AUDIENCE",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Carrier => "CARRIER",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::CombinedAudience => {
                    "COMBINED_AUDIENCE"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::ContentLabel => {
                    "CONTENT_LABEL"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::CustomAffinity => {
                    "CUSTOM_AFFINITY"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::CustomAudience => {
                    "CUSTOM_AUDIENCE"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::CustomIntent => {
                    "CUSTOM_INTENT"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Device => "DEVICE",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Gender => "GENDER",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::IncomeRange => {
                    "INCOME_RANGE"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::IpBlock => "IP_BLOCK",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Keyword => "KEYWORD",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::KeywordTheme => {
                    "KEYWORD_THEME"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Language => "LANGUAGE",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::ListingGroup => {
                    "LISTING_GROUP"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::ListingScope => {
                    "LISTING_SCOPE"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::LocalServiceId => {
                    "LOCAL_SERVICE_ID"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Location => "LOCATION",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::LocationGroup => {
                    "LOCATION_GROUP"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::MobileAppCategory => {
                    "MOBILE_APP_CATEGORY"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::MobileApplication => {
                    "MOBILE_APPLICATION"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::MobileDevice => {
                    "MOBILE_DEVICE"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::OperatingSystemVersion => {
                    "OPERATING_SYSTEM_VERSION"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::ParentalStatus => {
                    "PARENTAL_STATUS"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Placement => "PLACEMENT",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Proximity => "PROXIMITY",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Topic => "TOPIC",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Unspecified => "UNSPECIFIED",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::UserInterest => {
                    "USER_INTEREST"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::UserList => "USER_LIST",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Webpage => "WEBPAGE",
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::YoutubeChannel => {
                    "YOUTUBE_CHANNEL"
                }
                GoogleAdsSearchads360V0ResourcesCampaignCriterionType::YoutubeVideo => {
                    "YOUTUBE_VIDEO"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesCampaignCriterionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCampaignCriterionType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesCampaignCriterionType, ()>
        {
            Ok(match s {
                "AD_SCHEDULE" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::AdSchedule,
                "AGE_RANGE" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::AgeRange,
                "APP_PAYMENT_MODEL" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::AppPaymentModel
                }
                "AUDIENCE" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Audience,
                "CARRIER" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Carrier,
                "COMBINED_AUDIENCE" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::CombinedAudience
                }
                "CONTENT_LABEL" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::ContentLabel
                }
                "CUSTOM_AFFINITY" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::CustomAffinity
                }
                "CUSTOM_AUDIENCE" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::CustomAudience
                }
                "CUSTOM_INTENT" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::CustomIntent
                }
                "DEVICE" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Device,
                "GENDER" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Gender,
                "INCOME_RANGE" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::IncomeRange
                }
                "IP_BLOCK" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::IpBlock,
                "KEYWORD" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Keyword,
                "KEYWORD_THEME" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::KeywordTheme
                }
                "LANGUAGE" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Language,
                "LISTING_GROUP" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::ListingGroup
                }
                "LISTING_SCOPE" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::ListingScope
                }
                "LOCAL_SERVICE_ID" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::LocalServiceId
                }
                "LOCATION" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Location,
                "LOCATION_GROUP" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::LocationGroup
                }
                "MOBILE_APP_CATEGORY" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::MobileAppCategory
                }
                "MOBILE_APPLICATION" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::MobileApplication
                }
                "MOBILE_DEVICE" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::MobileDevice
                }
                "OPERATING_SYSTEM_VERSION" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::OperatingSystemVersion
                }
                "PARENTAL_STATUS" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::ParentalStatus
                }
                "PLACEMENT" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Placement,
                "PROXIMITY" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Proximity,
                "TOPIC" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Topic,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Unspecified,
                "USER_INTEREST" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::UserInterest
                }
                "USER_LIST" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::UserList,
                "WEBPAGE" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Webpage,
                "YOUTUBE_CHANNEL" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::YoutubeChannel
                }
                "YOUTUBE_VIDEO" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::YoutubeVideo
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCampaignCriterionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCampaignCriterionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesCampaignCriterionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AD_SCHEDULE" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::AdSchedule,
                "AGE_RANGE" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::AgeRange,
                "APP_PAYMENT_MODEL" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::AppPaymentModel
                }
                "AUDIENCE" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Audience,
                "CARRIER" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Carrier,
                "COMBINED_AUDIENCE" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::CombinedAudience
                }
                "CONTENT_LABEL" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::ContentLabel
                }
                "CUSTOM_AFFINITY" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::CustomAffinity
                }
                "CUSTOM_AUDIENCE" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::CustomAudience
                }
                "CUSTOM_INTENT" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::CustomIntent
                }
                "DEVICE" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Device,
                "GENDER" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Gender,
                "INCOME_RANGE" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::IncomeRange
                }
                "IP_BLOCK" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::IpBlock,
                "KEYWORD" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Keyword,
                "KEYWORD_THEME" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::KeywordTheme
                }
                "LANGUAGE" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Language,
                "LISTING_GROUP" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::ListingGroup
                }
                "LISTING_SCOPE" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::ListingScope
                }
                "LOCAL_SERVICE_ID" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::LocalServiceId
                }
                "LOCATION" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Location,
                "LOCATION_GROUP" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::LocationGroup
                }
                "MOBILE_APP_CATEGORY" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::MobileAppCategory
                }
                "MOBILE_APPLICATION" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::MobileApplication
                }
                "MOBILE_DEVICE" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::MobileDevice
                }
                "OPERATING_SYSTEM_VERSION" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::OperatingSystemVersion
                }
                "PARENTAL_STATUS" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::ParentalStatus
                }
                "PLACEMENT" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Placement,
                "PROXIMITY" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Proximity,
                "TOPIC" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Topic,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Unspecified,
                "USER_INTEREST" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::UserInterest
                }
                "USER_LIST" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::UserList,
                "WEBPAGE" => GoogleAdsSearchads360V0ResourcesCampaignCriterionType::Webpage,
                "YOUTUBE_CHANNEL" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::YoutubeChannel
                }
                "YOUTUBE_VIDEO" => {
                    GoogleAdsSearchads360V0ResourcesCampaignCriterionType::YoutubeVideo
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
        for GoogleAdsSearchads360V0ResourcesCampaignCriterionType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignCriterionType
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
    pub struct GoogleAdsSearchads360V0ResourcesCampaignDynamicSearchAdsSetting {
        #[doc = "Required. The Internet domain name that this setting represents, for example, “google.com” or “www.google.com”."]
        #[serde(
            rename = "domainName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain_name: ::std::option::Option<String>,
        #[doc = "Required. The language code specifying the language of the domain, for example, “en”."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Whether the campaign uses advertiser supplied URLs exclusively."]
        #[serde(
            rename = "useSuppliedUrlsOnly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_supplied_urls_only: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesCampaignDynamicSearchAdsSetting
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignDynamicSearchAdsSetting
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
    pub struct GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSetting { # [doc = "The setting used for negative geotargeting in this particular campaign."] # [serde (rename = "negativeGeoTargetType" , default , skip_serializing_if = "std::option::Option::is_none")] pub negative_geo_target_type : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType > , # [doc = "The setting used for positive geotargeting in this particular campaign."] # [serde (rename = "positiveGeoTargetType" , default , skip_serializing_if = "std::option::Option::is_none")] pub positive_geo_target_type : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType > , }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSetting
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSetting
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType {
        #[doc = "Specifies that a user is excluded from seeing the ad if they are in advertiser’s excluded locations."]
        Presence,
        #[doc = "Specifies that a user is excluded from seeing the ad if they are in, or show interest in, advertiser’s excluded locations."]
        PresenceOrInterest,
        #[doc = "The value is unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType :: Presence => "PRESENCE" , GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType :: PresenceOrInterest => "PRESENCE_OR_INTEREST" , GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType,
            (),
        > {
            Ok (match s { "PRESENCE" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType :: Presence , "PRESENCE_OR_INTEREST" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType :: PresenceOrInterest , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType :: Unspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "PRESENCE" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType :: Presence , "PRESENCE_OR_INTEREST" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType :: PresenceOrInterest , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingNegativeGeoTargetType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType {
        #[doc = "Specifies that an ad is triggered if the user is in or regularly in advertiser’s targeted locations."]
        Presence,
        #[doc = "Specifies that an ad is triggered if the user is in, or shows interest in, advertiser’s targeted locations."]
        PresenceOrInterest,
        #[doc = "Specifies that an ad is triggered if the user searches for advertiser’s targeted locations. This can only be used with Search and standard Shopping campaigns."]
        SearchInterest,
        #[doc = "The value is unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType :: Presence => "PRESENCE" , GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType :: PresenceOrInterest => "PRESENCE_OR_INTEREST" , GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType :: SearchInterest => "SEARCH_INTEREST" , GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType,
            (),
        > {
            Ok (match s { "PRESENCE" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType :: Presence , "PRESENCE_OR_INTEREST" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType :: PresenceOrInterest , "SEARCH_INTEREST" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType :: SearchInterest , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType :: Unspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "PRESENCE" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType :: Presence , "PRESENCE_OR_INTEREST" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType :: PresenceOrInterest , "SEARCH_INTEREST" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType :: SearchInterest , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignGeoTargetTypeSettingPositiveGeoTargetType
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
    pub struct GoogleAdsSearchads360V0ResourcesCampaignNetworkSettings {
        #[doc = "Whether ads will be served on specified placements in the Google Display Network. Placements are specified using the Placement criterion."]
        #[serde(
            rename = "targetContentNetwork",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_content_network: ::std::option::Option<bool>,
        #[doc = "Whether ads will be served with google.com search results."]
        #[serde(
            rename = "targetGoogleSearch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_google_search: ::std::option::Option<bool>,
        #[doc = "Whether ads will be served on the Google Partner Network. This is available only to some select Google partner accounts."]
        #[serde(
            rename = "targetPartnerSearchNetwork",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_partner_search_network: ::std::option::Option<bool>,
        #[doc = "Whether ads will be served on partner sites in the Google Search Network (requires `target_google_search` to also be `true`)."]
        #[serde(
            rename = "targetSearchNetwork",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_search_network: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesCampaignNetworkSettings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignNetworkSettings
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
    pub struct GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSetting { # [doc = "The list of optimization goal types."] # [serde (rename = "optimizationGoalTypes" , default , skip_serializing_if = "std::option::Option::is_none")] pub optimization_goal_types : :: std :: option :: Option < Vec < crate :: schemas :: GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems > > , }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSetting
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSetting
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems {
        #[doc = "Optimize for pre-registration. Pre-registration conversions are the number of pre-registration signups to receive a notification when the app is released."]
        AppPreRegistration,
        #[doc = "Optimize for call clicks. Call click conversions are times people selected ‘Call’ to contact a store after viewing an ad."]
        CallClicks,
        #[doc = "Optimize for driving directions. Driving directions conversions are times people selected ‘Get directions’ to navigate to a store after viewing an ad."]
        DrivingDirections,
        #[doc = "Used as a return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems :: AppPreRegistration => "APP_PRE_REGISTRATION" , GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems :: CallClicks => "CALL_CLICKS" , GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems :: DrivingDirections => "DRIVING_DIRECTIONS" , GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl :: std :: convert :: AsRef < str > for GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems { fn as_ref (& self) -> & str { self . as_str () } }
    impl :: std :: str :: FromStr for GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems { type Err = () ; fn from_str (s : & str) -> :: std :: result :: Result < GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems , () > { Ok (match s { "APP_PRE_REGISTRATION" => GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems :: AppPreRegistration , "CALL_CLICKS" => GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems :: CallClicks , "DRIVING_DIRECTIONS" => GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems :: DrivingDirections , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems :: Unspecified , _ => return Err (()) , }) } }
    impl :: std :: fmt :: Display for GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> :: std :: fmt :: Result { f . write_str (self . as_str ()) } }
    impl :: serde :: Serialize for GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems { fn serialize < S > (& self , serializer : S) -> :: std :: result :: Result < S :: Ok , S :: Error > where S : :: serde :: ser :: Serializer { serializer . serialize_str (self . as_str ()) } }
    impl < 'de > :: serde :: Deserialize < 'de > for GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems { fn deserialize < D > (deserializer : D) -> :: std :: result :: Result < Self , D :: Error > where D : :: serde :: de :: Deserializer < 'de > , { let value : & 'de str = < & str > :: deserialize (deserializer) ? ; Ok (match value { "APP_PRE_REGISTRATION" => GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems :: AppPreRegistration , "CALL_CLICKS" => GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems :: CallClicks , "DRIVING_DIRECTIONS" => GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems :: DrivingDirections , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , }) } }
    impl :: google_field_selector :: FieldSelector for GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems { fn fields () -> Vec < :: google_field_selector :: Field > { Vec :: new () } }
    impl :: google_field_selector :: ToFieldType for GoogleAdsSearchads360V0ResourcesCampaignOptimizationGoalSettingOptimizationGoalTypesItems { fn field_type () -> :: google_field_selector :: FieldType { :: google_field_selector :: FieldType :: Leaf } }
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
    pub struct GoogleAdsSearchads360V0ResourcesCampaignSelectiveOptimization {
        #[doc = "The selected set of conversion actions for optimizing this campaign."]
        #[serde(
            rename = "conversionActions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversion_actions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesCampaignSelectiveOptimization
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignSelectiveOptimization
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
    pub struct GoogleAdsSearchads360V0ResourcesCampaignShoppingSetting {
        #[doc = "Priority of the campaign. Campaigns with numerically higher priorities take precedence over those with lower priorities. This field is required for Shopping campaigns, with values between 0 and 2, inclusive. This field is optional for Smart Shopping campaigns, but must be equal to 3 if set."]
        #[serde(
            rename = "campaignPriority",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub campaign_priority: ::std::option::Option<i32>,
        #[doc = "Whether to include local products."]
        #[serde(
            rename = "enableLocal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_local: ::std::option::Option<bool>,
        #[doc = "Feed label of products to include in the campaign. Only one of feed_label or sales_country can be set. If used instead of sales_country, the feed_label field accepts country codes in the same format for example: ‘XX’. Otherwise can be any string used for feed label in Google Merchant Center."]
        #[serde(
            rename = "feedLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feed_label: ::std::option::Option<String>,
        #[doc = "Immutable. ID of the Merchant Center account. This field is required for create operations. This field is immutable for Shopping campaigns."]
        #[serde(
            rename = "merchantId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub merchant_id: ::std::option::Option<i64>,
        #[doc = "Sales country of products to include in the campaign. "]
        #[serde(
            rename = "salesCountry",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sales_country: ::std::option::Option<String>,
        #[doc = "Immutable. Whether to target Vehicle Listing inventory."]
        #[serde(
            rename = "useVehicleInventory",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub use_vehicle_inventory: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesCampaignShoppingSetting
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignShoppingSetting
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
    pub struct GoogleAdsSearchads360V0ResourcesCampaignTrackingSetting {
        #[doc = "Output only. The url used for dynamic tracking."]
        #[serde(
            rename = "trackingUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracking_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesCampaignTrackingSetting
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCampaignTrackingSetting
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0ResourcesConversionAction { # [doc = "App ID for an app conversion action."] # [serde (rename = "appId" , default , skip_serializing_if = "std::option::Option::is_none")] pub app_id : :: std :: option :: Option < String > , # [doc = "Settings related to this conversion action’s attribution model."] # [serde (rename = "attributionModelSettings" , default , skip_serializing_if = "std::option::Option::is_none")] pub attribution_model_settings : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettings > , # [doc = "The category of conversions reported for this conversion action."] # [serde (rename = "category" , default , skip_serializing_if = "std::option::Option::is_none")] pub category : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesConversionActionCategory > , # [doc = "The maximum number of days that may elapse between an interaction (for example, a click) and a conversion event."] # [serde (rename = "clickThroughLookbackWindowDays" , default , skip_serializing_if = "std::option::Option::is_none")] # [serde (with = "crate::parsed_string")] pub click_through_lookback_window_days : :: std :: option :: Option < i64 > , # [doc = "Output only. Timestamp of the Floodlight activity’s creation, formatted in ISO 8601."] # [serde (rename = "creationTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub creation_time : :: std :: option :: Option < String > , # [doc = "Output only. Floodlight settings for Floodlight conversion types."] # [serde (rename = "floodlightSettings" , default , skip_serializing_if = "std::option::Option::is_none")] pub floodlight_settings : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesConversionActionFloodlightSettings > , # [doc = "Output only. The ID of the conversion action."] # [serde (rename = "id" , default , skip_serializing_if = "std::option::Option::is_none")] # [serde (with = "crate::parsed_string")] pub id : :: std :: option :: Option < i64 > , # [doc = "Whether this conversion action should be included in the “client_account_conversions” metric."] # [serde (rename = "includeInClientAccountConversionsMetric" , default , skip_serializing_if = "std::option::Option::is_none")] pub include_in_client_account_conversions_metric : :: std :: option :: Option < bool > , # [doc = "Output only. Whether this conversion action should be included in the “conversions” metric."] # [serde (rename = "includeInConversionsMetric" , default , skip_serializing_if = "std::option::Option::is_none")] pub include_in_conversions_metric : :: std :: option :: Option < bool > , # [doc = "The name of the conversion action. This field is required and should not be empty when creating new conversion actions."] # [serde (rename = "name" , default , skip_serializing_if = "std::option::Option::is_none")] pub name : :: std :: option :: Option < String > , # [doc = "Output only. The resource name of the conversion action owner customer, or null if this is a system-defined conversion action."] # [serde (rename = "ownerCustomer" , default , skip_serializing_if = "std::option::Option::is_none")] pub owner_customer : :: std :: option :: Option < String > , # [doc = "If a conversion action’s primary_for_goal bit is false, the conversion action is non-biddable for all campaigns regardless of their customer conversion goal or campaign conversion goal. However, custom conversion goals do not respect primary_for_goal, so if a campaign has a custom conversion goal configured with a primary_for_goal = false conversion action, that conversion action is still biddable. By default, primary_for_goal will be true if not set. In V9, primary_for_goal can only be set to false after creation through an ‘update’ operation because it’s not declared as optional."] # [serde (rename = "primaryForGoal" , default , skip_serializing_if = "std::option::Option::is_none")] pub primary_for_goal : :: std :: option :: Option < bool > , # [doc = "Immutable. The type of this conversion action."] # [serde (rename = "type" , default , skip_serializing_if = "std::option::Option::is_none")] pub r#type : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesConversionActionType > , # [doc = "Immutable. The resource name of the conversion action. Conversion action resource names have the form: `customers/{customer_id}/conversionActions/{conversion_action_id}`"] # [serde (rename = "resourceName" , default , skip_serializing_if = "std::option::Option::is_none")] pub resource_name : :: std :: option :: Option < String > , # [doc = "The status of this conversion action for conversion event accrual."] # [serde (rename = "status" , default , skip_serializing_if = "std::option::Option::is_none")] pub status : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesConversionActionStatus > , # [doc = "Settings related to the value for conversion events associated with this conversion action."] # [serde (rename = "valueSettings" , default , skip_serializing_if = "std::option::Option::is_none")] pub value_settings : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesConversionActionValueSettings > , }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesConversionAction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesConversionAction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesConversionActionCategory {
        #[doc = "The addition of items to a shopping cart or bag on an advertiser site."]
        AddToCart,
        #[doc = "When someone enters the checkout flow on an advertiser site."]
        BeginCheckout,
        #[doc = "A booking of an appointment with an advertiser’s business."]
        BookAppointment,
        #[doc = "A call, SMS, email, chat or other type of contact to an advertiser."]
        Contact,
        #[doc = "A lead conversion imported from an external source into Google Ads, that has further completed a chosen stage as defined by the lead gen advertiser."]
        ConvertedLead,
        #[doc = "Default category."]
        Default,
        #[doc = "Software download action (as for an app)."]
        Download,
        #[doc = "A website engagement event such as long site time or a Google Analytics (GA) Smart Goal. Intended to be used for GA, Firebase, GA Gold goal imports."]
        Engagement,
        #[doc = "A search for an advertiser’s business location with intention to visit."]
        GetDirections,
        #[doc = "A lead conversion imported from an external source into Google Ads."]
        ImportedLead,
        #[doc = "Lead-generating action."]
        Lead,
        #[doc = "A click to an advertiser’s partner’s site."]
        OutboundClick,
        #[doc = "User visiting a page."]
        PageView,
        #[doc = "A call to indicate interest in an advertiser’s offering."]
        PhoneCallLead,
        #[doc = "Purchase, sales, or “order placed” event."]
        Purchase,
        #[doc = "A lead conversion imported from an external source into Google Ads, that has been further qualified by the advertiser (marketing/sales team). In the lead-to-sale journey, advertisers get leads, then act on them by reaching out to the consumer. If the consumer is interested and may end up buying their product, the advertiser marks such leads as “qualified leads”."]
        QualifiedLead,
        #[doc = "A quote or price estimate request."]
        RequestQuote,
        #[doc = "Signup user action."]
        Signup,
        #[doc = "A sale occurring in a physical store."]
        StoreSale,
        #[doc = "A visit to a physical store location."]
        StoreVisit,
        #[doc = "A submission of a form on an advertiser site indicating business interest."]
        SubmitLeadForm,
        #[doc = "The start of a paid subscription for a product or service."]
        SubscribePaid,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesConversionActionCategory {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::AddToCart => {
                    "ADD_TO_CART"
                }
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::BeginCheckout => {
                    "BEGIN_CHECKOUT"
                }
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::BookAppointment => {
                    "BOOK_APPOINTMENT"
                }
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::Contact => "CONTACT",
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::ConvertedLead => {
                    "CONVERTED_LEAD"
                }
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::Default => "DEFAULT",
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::Download => "DOWNLOAD",
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::Engagement => {
                    "ENGAGEMENT"
                }
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::GetDirections => {
                    "GET_DIRECTIONS"
                }
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::ImportedLead => {
                    "IMPORTED_LEAD"
                }
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::Lead => "LEAD",
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::OutboundClick => {
                    "OUTBOUND_CLICK"
                }
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::PageView => "PAGE_VIEW",
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::PhoneCallLead => {
                    "PHONE_CALL_LEAD"
                }
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::Purchase => "PURCHASE",
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::QualifiedLead => {
                    "QUALIFIED_LEAD"
                }
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::RequestQuote => {
                    "REQUEST_QUOTE"
                }
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::Signup => "SIGNUP",
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::StoreSale => "STORE_SALE",
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::StoreVisit => {
                    "STORE_VISIT"
                }
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::SubmitLeadForm => {
                    "SUBMIT_LEAD_FORM"
                }
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::SubscribePaid => {
                    "SUBSCRIBE_PAID"
                }
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesConversionActionCategory::Unspecified => {
                    "UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesConversionActionCategory {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesConversionActionCategory {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesConversionActionCategory, ()>
        {
            Ok(match s {
                "ADD_TO_CART" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::AddToCart
                }
                "BEGIN_CHECKOUT" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::BeginCheckout
                }
                "BOOK_APPOINTMENT" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::BookAppointment
                }
                "CONTACT" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::Contact,
                "CONVERTED_LEAD" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::ConvertedLead
                }
                "DEFAULT" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::Default,
                "DOWNLOAD" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::Download,
                "ENGAGEMENT" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::Engagement
                }
                "GET_DIRECTIONS" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::GetDirections
                }
                "IMPORTED_LEAD" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::ImportedLead
                }
                "LEAD" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::Lead,
                "OUTBOUND_CLICK" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::OutboundClick
                }
                "PAGE_VIEW" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::PageView,
                "PHONE_CALL_LEAD" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::PhoneCallLead
                }
                "PURCHASE" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::Purchase,
                "QUALIFIED_LEAD" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::QualifiedLead
                }
                "REQUEST_QUOTE" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::RequestQuote
                }
                "SIGNUP" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::Signup,
                "STORE_SALE" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::StoreSale,
                "STORE_VISIT" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::StoreVisit
                }
                "SUBMIT_LEAD_FORM" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::SubmitLeadForm
                }
                "SUBSCRIBE_PAID" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::SubscribePaid
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::Unspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesConversionActionCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesConversionActionCategory {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesConversionActionCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADD_TO_CART" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::AddToCart
                }
                "BEGIN_CHECKOUT" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::BeginCheckout
                }
                "BOOK_APPOINTMENT" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::BookAppointment
                }
                "CONTACT" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::Contact,
                "CONVERTED_LEAD" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::ConvertedLead
                }
                "DEFAULT" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::Default,
                "DOWNLOAD" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::Download,
                "ENGAGEMENT" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::Engagement
                }
                "GET_DIRECTIONS" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::GetDirections
                }
                "IMPORTED_LEAD" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::ImportedLead
                }
                "LEAD" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::Lead,
                "OUTBOUND_CLICK" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::OutboundClick
                }
                "PAGE_VIEW" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::PageView,
                "PHONE_CALL_LEAD" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::PhoneCallLead
                }
                "PURCHASE" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::Purchase,
                "QUALIFIED_LEAD" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::QualifiedLead
                }
                "REQUEST_QUOTE" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::RequestQuote
                }
                "SIGNUP" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::Signup,
                "STORE_SALE" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::StoreSale,
                "STORE_VISIT" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::StoreVisit
                }
                "SUBMIT_LEAD_FORM" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::SubmitLeadForm
                }
                "SUBSCRIBE_PAID" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::SubscribePaid
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesConversionActionCategory::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionCategory::Unspecified
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
        for GoogleAdsSearchads360V0ResourcesConversionActionCategory
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesConversionActionCategory
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesConversionActionType {
        #[doc = "Conversions that occur when a user clicks on an ad’s call extension."]
        AdCall,
        #[doc = "Conversions that occur when a user pre-registers a mobile app from the Google Play Store. Read only."]
        AndroidAppPreRegistration,
        #[doc = "Conversions that track all Google Play downloads which aren’t tracked by an app-specific type. Read only."]
        AndroidInstallsAllOtherApps,
        #[doc = "Conversions that occur when a user on a mobile device clicks a phone number."]
        ClickToCall,
        #[doc = "Android app custom conversions tracked through Firebase."]
        FirebaseAndroidCustom,
        #[doc = "Android app first open conversions tracked through Firebase."]
        FirebaseAndroidFirstOpen,
        #[doc = "Android app in app purchase conversions tracked through Firebase."]
        FirebaseAndroidInAppPurchase,
        #[doc = "iOS app custom conversions tracked through Firebase."]
        FirebaseIosCustom,
        #[doc = "iOS app first open conversions tracked through Firebase."]
        FirebaseIosFirstOpen,
        #[doc = "iOS app in app purchase conversions tracked through Firebase."]
        FirebaseIosInAppPurchase,
        #[doc = "Floodlight activity that counts the number of times that users have visited a particular webpage after seeing or clicking on one of an advertiser’s ads. Read only."]
        FloodlightAction,
        #[doc = "Floodlight activity that tracks the number of sales made or the number of items purchased. Can also capture the total value of each sale. Read only."]
        FloodlightTransaction,
        #[doc = "Conversions that track local actions from Google’s products and services after interacting with an ad. Read only."]
        GoogleHosted,
        #[doc = "Conversions that occur when a user downloads a mobile app from the Google Play Store."]
        GooglePlayDownload,
        #[doc = "Conversions that occur when a user makes a purchase in an app through Android billing."]
        GooglePlayInAppPurchase,
        #[doc = "Conversions reported when a user submits a lead form. Read only."]
        LeadFormSubmit,
        #[doc = "Conversions that come from Salesforce. Read only."]
        Salesforce,
        #[doc = "Conversions imported from Search Ads 360 Floodlight data. Read only."]
        SearchAds360,
        #[doc = "Call conversions that occur on Smart campaign Ads without call tracking setup, using Smart campaign custom criteria. Read only."]
        SmartCampaignAdClicksToCall,
        #[doc = "The user clicks on a call element within Google Maps. Smart campaign only. Read only."]
        SmartCampaignMapClicksToCall,
        #[doc = "The user requests directions to a business location within Google Maps. Smart campaign only. Read only."]
        SmartCampaignMapDirections,
        #[doc = "Call conversions that occur on Smart campaign Ads with call tracking setup, using Smart campaign custom criteria. Read only."]
        SmartCampaignTrackedCalls,
        #[doc = "Store Sales conversion based on first-party or third-party merchant data uploads and/or from in-store purchases using cards from payment networks. Only customers on the allowlist can use store sales types. Read only."]
        StoreSales,
        #[doc = "Store Sales conversion based on first-party or third-party merchant data uploads. Only customers on the allowlist can use store sales direct upload types."]
        StoreSalesDirectUpload,
        #[doc = "Conversions that occur when a user visits an advertiser’s retail store. Read only."]
        StoreVisits,
        #[doc = "Android app custom conversions tracked through Third Party App Analytics."]
        ThirdPartyAppAnalyticsAndroidCustom,
        #[doc = "Android app first open conversions tracked through Third Party App Analytics."]
        ThirdPartyAppAnalyticsAndroidFirstOpen,
        #[doc = "Android app in app purchase conversions tracked through Third Party App Analytics."]
        ThirdPartyAppAnalyticsAndroidInAppPurchase,
        #[doc = "iOS app custom conversions tracked through Third Party App Analytics."]
        ThirdPartyAppAnalyticsIosCustom,
        #[doc = "iOS app first open conversions tracked through Third Party App Analytics."]
        ThirdPartyAppAnalyticsIosFirstOpen,
        #[doc = "iOS app in app purchase conversions tracked through Third Party App Analytics."]
        ThirdPartyAppAnalyticsIosInAppPurchase,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
        #[doc = "Call conversions that are tracked by the advertiser and uploaded."]
        UploadCalls,
        #[doc = "Conversions that are tracked by the advertiser and uploaded with attributed clicks."]
        UploadClicks,
        #[doc = "Conversions that occur on a webpage."]
        Webpage,
        #[doc = "Conversions that occur when a user calls a dynamically-generated phone number from an advertiser’s website."]
        WebsiteCall,
    }
    impl GoogleAdsSearchads360V0ResourcesConversionActionType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ResourcesConversionActionType :: AdCall => "AD_CALL" , GoogleAdsSearchads360V0ResourcesConversionActionType :: AndroidAppPreRegistration => "ANDROID_APP_PRE_REGISTRATION" , GoogleAdsSearchads360V0ResourcesConversionActionType :: AndroidInstallsAllOtherApps => "ANDROID_INSTALLS_ALL_OTHER_APPS" , GoogleAdsSearchads360V0ResourcesConversionActionType :: ClickToCall => "CLICK_TO_CALL" , GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseAndroidCustom => "FIREBASE_ANDROID_CUSTOM" , GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseAndroidFirstOpen => "FIREBASE_ANDROID_FIRST_OPEN" , GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseAndroidInAppPurchase => "FIREBASE_ANDROID_IN_APP_PURCHASE" , GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseIosCustom => "FIREBASE_IOS_CUSTOM" , GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseIosFirstOpen => "FIREBASE_IOS_FIRST_OPEN" , GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseIosInAppPurchase => "FIREBASE_IOS_IN_APP_PURCHASE" , GoogleAdsSearchads360V0ResourcesConversionActionType :: FloodlightAction => "FLOODLIGHT_ACTION" , GoogleAdsSearchads360V0ResourcesConversionActionType :: FloodlightTransaction => "FLOODLIGHT_TRANSACTION" , GoogleAdsSearchads360V0ResourcesConversionActionType :: GoogleHosted => "GOOGLE_HOSTED" , GoogleAdsSearchads360V0ResourcesConversionActionType :: GooglePlayDownload => "GOOGLE_PLAY_DOWNLOAD" , GoogleAdsSearchads360V0ResourcesConversionActionType :: GooglePlayInAppPurchase => "GOOGLE_PLAY_IN_APP_PURCHASE" , GoogleAdsSearchads360V0ResourcesConversionActionType :: LeadFormSubmit => "LEAD_FORM_SUBMIT" , GoogleAdsSearchads360V0ResourcesConversionActionType :: Salesforce => "SALESFORCE" , GoogleAdsSearchads360V0ResourcesConversionActionType :: SearchAds360 => "SEARCH_ADS_360" , GoogleAdsSearchads360V0ResourcesConversionActionType :: SmartCampaignAdClicksToCall => "SMART_CAMPAIGN_AD_CLICKS_TO_CALL" , GoogleAdsSearchads360V0ResourcesConversionActionType :: SmartCampaignMapClicksToCall => "SMART_CAMPAIGN_MAP_CLICKS_TO_CALL" , GoogleAdsSearchads360V0ResourcesConversionActionType :: SmartCampaignMapDirections => "SMART_CAMPAIGN_MAP_DIRECTIONS" , GoogleAdsSearchads360V0ResourcesConversionActionType :: SmartCampaignTrackedCalls => "SMART_CAMPAIGN_TRACKED_CALLS" , GoogleAdsSearchads360V0ResourcesConversionActionType :: StoreSales => "STORE_SALES" , GoogleAdsSearchads360V0ResourcesConversionActionType :: StoreSalesDirectUpload => "STORE_SALES_DIRECT_UPLOAD" , GoogleAdsSearchads360V0ResourcesConversionActionType :: StoreVisits => "STORE_VISITS" , GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsAndroidCustom => "THIRD_PARTY_APP_ANALYTICS_ANDROID_CUSTOM" , GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsAndroidFirstOpen => "THIRD_PARTY_APP_ANALYTICS_ANDROID_FIRST_OPEN" , GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsAndroidInAppPurchase => "THIRD_PARTY_APP_ANALYTICS_ANDROID_IN_APP_PURCHASE" , GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsIosCustom => "THIRD_PARTY_APP_ANALYTICS_IOS_CUSTOM" , GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsIosFirstOpen => "THIRD_PARTY_APP_ANALYTICS_IOS_FIRST_OPEN" , GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsIosInAppPurchase => "THIRD_PARTY_APP_ANALYTICS_IOS_IN_APP_PURCHASE" , GoogleAdsSearchads360V0ResourcesConversionActionType :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ResourcesConversionActionType :: Unspecified => "UNSPECIFIED" , GoogleAdsSearchads360V0ResourcesConversionActionType :: UploadCalls => "UPLOAD_CALLS" , GoogleAdsSearchads360V0ResourcesConversionActionType :: UploadClicks => "UPLOAD_CLICKS" , GoogleAdsSearchads360V0ResourcesConversionActionType :: Webpage => "WEBPAGE" , GoogleAdsSearchads360V0ResourcesConversionActionType :: WebsiteCall => "WEBSITE_CALL" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesConversionActionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesConversionActionType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesConversionActionType, ()>
        {
            Ok (match s { "AD_CALL" => GoogleAdsSearchads360V0ResourcesConversionActionType :: AdCall , "ANDROID_APP_PRE_REGISTRATION" => GoogleAdsSearchads360V0ResourcesConversionActionType :: AndroidAppPreRegistration , "ANDROID_INSTALLS_ALL_OTHER_APPS" => GoogleAdsSearchads360V0ResourcesConversionActionType :: AndroidInstallsAllOtherApps , "CLICK_TO_CALL" => GoogleAdsSearchads360V0ResourcesConversionActionType :: ClickToCall , "FIREBASE_ANDROID_CUSTOM" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseAndroidCustom , "FIREBASE_ANDROID_FIRST_OPEN" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseAndroidFirstOpen , "FIREBASE_ANDROID_IN_APP_PURCHASE" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseAndroidInAppPurchase , "FIREBASE_IOS_CUSTOM" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseIosCustom , "FIREBASE_IOS_FIRST_OPEN" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseIosFirstOpen , "FIREBASE_IOS_IN_APP_PURCHASE" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseIosInAppPurchase , "FLOODLIGHT_ACTION" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FloodlightAction , "FLOODLIGHT_TRANSACTION" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FloodlightTransaction , "GOOGLE_HOSTED" => GoogleAdsSearchads360V0ResourcesConversionActionType :: GoogleHosted , "GOOGLE_PLAY_DOWNLOAD" => GoogleAdsSearchads360V0ResourcesConversionActionType :: GooglePlayDownload , "GOOGLE_PLAY_IN_APP_PURCHASE" => GoogleAdsSearchads360V0ResourcesConversionActionType :: GooglePlayInAppPurchase , "LEAD_FORM_SUBMIT" => GoogleAdsSearchads360V0ResourcesConversionActionType :: LeadFormSubmit , "SALESFORCE" => GoogleAdsSearchads360V0ResourcesConversionActionType :: Salesforce , "SEARCH_ADS_360" => GoogleAdsSearchads360V0ResourcesConversionActionType :: SearchAds360 , "SMART_CAMPAIGN_AD_CLICKS_TO_CALL" => GoogleAdsSearchads360V0ResourcesConversionActionType :: SmartCampaignAdClicksToCall , "SMART_CAMPAIGN_MAP_CLICKS_TO_CALL" => GoogleAdsSearchads360V0ResourcesConversionActionType :: SmartCampaignMapClicksToCall , "SMART_CAMPAIGN_MAP_DIRECTIONS" => GoogleAdsSearchads360V0ResourcesConversionActionType :: SmartCampaignMapDirections , "SMART_CAMPAIGN_TRACKED_CALLS" => GoogleAdsSearchads360V0ResourcesConversionActionType :: SmartCampaignTrackedCalls , "STORE_SALES" => GoogleAdsSearchads360V0ResourcesConversionActionType :: StoreSales , "STORE_SALES_DIRECT_UPLOAD" => GoogleAdsSearchads360V0ResourcesConversionActionType :: StoreSalesDirectUpload , "STORE_VISITS" => GoogleAdsSearchads360V0ResourcesConversionActionType :: StoreVisits , "THIRD_PARTY_APP_ANALYTICS_ANDROID_CUSTOM" => GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsAndroidCustom , "THIRD_PARTY_APP_ANALYTICS_ANDROID_FIRST_OPEN" => GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsAndroidFirstOpen , "THIRD_PARTY_APP_ANALYTICS_ANDROID_IN_APP_PURCHASE" => GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsAndroidInAppPurchase , "THIRD_PARTY_APP_ANALYTICS_IOS_CUSTOM" => GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsIosCustom , "THIRD_PARTY_APP_ANALYTICS_IOS_FIRST_OPEN" => GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsIosFirstOpen , "THIRD_PARTY_APP_ANALYTICS_IOS_IN_APP_PURCHASE" => GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsIosInAppPurchase , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesConversionActionType :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesConversionActionType :: Unspecified , "UPLOAD_CALLS" => GoogleAdsSearchads360V0ResourcesConversionActionType :: UploadCalls , "UPLOAD_CLICKS" => GoogleAdsSearchads360V0ResourcesConversionActionType :: UploadClicks , "WEBPAGE" => GoogleAdsSearchads360V0ResourcesConversionActionType :: Webpage , "WEBSITE_CALL" => GoogleAdsSearchads360V0ResourcesConversionActionType :: WebsiteCall , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesConversionActionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesConversionActionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesConversionActionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "AD_CALL" => GoogleAdsSearchads360V0ResourcesConversionActionType :: AdCall , "ANDROID_APP_PRE_REGISTRATION" => GoogleAdsSearchads360V0ResourcesConversionActionType :: AndroidAppPreRegistration , "ANDROID_INSTALLS_ALL_OTHER_APPS" => GoogleAdsSearchads360V0ResourcesConversionActionType :: AndroidInstallsAllOtherApps , "CLICK_TO_CALL" => GoogleAdsSearchads360V0ResourcesConversionActionType :: ClickToCall , "FIREBASE_ANDROID_CUSTOM" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseAndroidCustom , "FIREBASE_ANDROID_FIRST_OPEN" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseAndroidFirstOpen , "FIREBASE_ANDROID_IN_APP_PURCHASE" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseAndroidInAppPurchase , "FIREBASE_IOS_CUSTOM" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseIosCustom , "FIREBASE_IOS_FIRST_OPEN" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseIosFirstOpen , "FIREBASE_IOS_IN_APP_PURCHASE" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FirebaseIosInAppPurchase , "FLOODLIGHT_ACTION" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FloodlightAction , "FLOODLIGHT_TRANSACTION" => GoogleAdsSearchads360V0ResourcesConversionActionType :: FloodlightTransaction , "GOOGLE_HOSTED" => GoogleAdsSearchads360V0ResourcesConversionActionType :: GoogleHosted , "GOOGLE_PLAY_DOWNLOAD" => GoogleAdsSearchads360V0ResourcesConversionActionType :: GooglePlayDownload , "GOOGLE_PLAY_IN_APP_PURCHASE" => GoogleAdsSearchads360V0ResourcesConversionActionType :: GooglePlayInAppPurchase , "LEAD_FORM_SUBMIT" => GoogleAdsSearchads360V0ResourcesConversionActionType :: LeadFormSubmit , "SALESFORCE" => GoogleAdsSearchads360V0ResourcesConversionActionType :: Salesforce , "SEARCH_ADS_360" => GoogleAdsSearchads360V0ResourcesConversionActionType :: SearchAds360 , "SMART_CAMPAIGN_AD_CLICKS_TO_CALL" => GoogleAdsSearchads360V0ResourcesConversionActionType :: SmartCampaignAdClicksToCall , "SMART_CAMPAIGN_MAP_CLICKS_TO_CALL" => GoogleAdsSearchads360V0ResourcesConversionActionType :: SmartCampaignMapClicksToCall , "SMART_CAMPAIGN_MAP_DIRECTIONS" => GoogleAdsSearchads360V0ResourcesConversionActionType :: SmartCampaignMapDirections , "SMART_CAMPAIGN_TRACKED_CALLS" => GoogleAdsSearchads360V0ResourcesConversionActionType :: SmartCampaignTrackedCalls , "STORE_SALES" => GoogleAdsSearchads360V0ResourcesConversionActionType :: StoreSales , "STORE_SALES_DIRECT_UPLOAD" => GoogleAdsSearchads360V0ResourcesConversionActionType :: StoreSalesDirectUpload , "STORE_VISITS" => GoogleAdsSearchads360V0ResourcesConversionActionType :: StoreVisits , "THIRD_PARTY_APP_ANALYTICS_ANDROID_CUSTOM" => GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsAndroidCustom , "THIRD_PARTY_APP_ANALYTICS_ANDROID_FIRST_OPEN" => GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsAndroidFirstOpen , "THIRD_PARTY_APP_ANALYTICS_ANDROID_IN_APP_PURCHASE" => GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsAndroidInAppPurchase , "THIRD_PARTY_APP_ANALYTICS_IOS_CUSTOM" => GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsIosCustom , "THIRD_PARTY_APP_ANALYTICS_IOS_FIRST_OPEN" => GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsIosFirstOpen , "THIRD_PARTY_APP_ANALYTICS_IOS_IN_APP_PURCHASE" => GoogleAdsSearchads360V0ResourcesConversionActionType :: ThirdPartyAppAnalyticsIosInAppPurchase , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesConversionActionType :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesConversionActionType :: Unspecified , "UPLOAD_CALLS" => GoogleAdsSearchads360V0ResourcesConversionActionType :: UploadCalls , "UPLOAD_CLICKS" => GoogleAdsSearchads360V0ResourcesConversionActionType :: UploadClicks , "WEBPAGE" => GoogleAdsSearchads360V0ResourcesConversionActionType :: Webpage , "WEBSITE_CALL" => GoogleAdsSearchads360V0ResourcesConversionActionType :: WebsiteCall , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesConversionActionType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesConversionActionType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesConversionActionStatus {
        #[doc = "Conversions will be recorded."]
        Enabled,
        #[doc = "Conversions will not be recorded and the conversion action will not appear in the UI."]
        Hidden,
        #[doc = "Conversions will not be recorded."]
        Removed,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesConversionActionStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesConversionActionStatus::Enabled => "ENABLED",
                GoogleAdsSearchads360V0ResourcesConversionActionStatus::Hidden => "HIDDEN",
                GoogleAdsSearchads360V0ResourcesConversionActionStatus::Removed => "REMOVED",
                GoogleAdsSearchads360V0ResourcesConversionActionStatus::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesConversionActionStatus::Unspecified => {
                    "UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesConversionActionStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesConversionActionStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesConversionActionStatus, ()>
        {
            Ok(match s {
                "ENABLED" => GoogleAdsSearchads360V0ResourcesConversionActionStatus::Enabled,
                "HIDDEN" => GoogleAdsSearchads360V0ResourcesConversionActionStatus::Hidden,
                "REMOVED" => GoogleAdsSearchads360V0ResourcesConversionActionStatus::Removed,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesConversionActionStatus::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionStatus::Unspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesConversionActionStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesConversionActionStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesConversionActionStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ENABLED" => GoogleAdsSearchads360V0ResourcesConversionActionStatus::Enabled,
                "HIDDEN" => GoogleAdsSearchads360V0ResourcesConversionActionStatus::Hidden,
                "REMOVED" => GoogleAdsSearchads360V0ResourcesConversionActionStatus::Removed,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesConversionActionStatus::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesConversionActionStatus::Unspecified
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
        for GoogleAdsSearchads360V0ResourcesConversionActionStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesConversionActionStatus
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
    pub struct GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettings { # [doc = "The attribution model type of this conversion action."] # [serde (rename = "attributionModel" , default , skip_serializing_if = "std::option::Option::is_none")] pub attribution_model : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel > , # [doc = "Output only. The status of the data-driven attribution model for the conversion action."] # [serde (rename = "dataDrivenModelStatus" , default , skip_serializing_if = "std::option::Option::is_none")] pub data_driven_model_status : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus > , }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettings
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel {
        #[doc = "Uses external attribution."]
        External,
        #[doc = "Attributes all credit for a conversion to its last click."]
        GoogleAdsLastClick,
        #[doc = "Flexible model that uses machine learning to determine the appropriate distribution of credit among clicks using Google Search attribution."]
        GoogleSearchAttributionDataDriven,
        #[doc = "Attributes all credit for a conversion to its first click using Google Search attribution."]
        GoogleSearchAttributionFirstClick,
        #[doc = "Attributes credit for a conversion equally across all of its clicks using Google Search attribution."]
        GoogleSearchAttributionLinear,
        #[doc = "Attributes 40% of the credit for a conversion to its first and last clicks. Remaining 20% is evenly distributed across all other clicks. This uses Google Search attribution."]
        GoogleSearchAttributionPositionBased,
        #[doc = "Attributes exponentially more credit for a conversion to its more recent clicks using Google Search attribution (half-life is 1 week)."]
        GoogleSearchAttributionTimeDecay,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: External => "EXTERNAL" , GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleAdsLastClick => "GOOGLE_ADS_LAST_CLICK" , GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleSearchAttributionDataDriven => "GOOGLE_SEARCH_ATTRIBUTION_DATA_DRIVEN" , GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleSearchAttributionFirstClick => "GOOGLE_SEARCH_ATTRIBUTION_FIRST_CLICK" , GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleSearchAttributionLinear => "GOOGLE_SEARCH_ATTRIBUTION_LINEAR" , GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleSearchAttributionPositionBased => "GOOGLE_SEARCH_ATTRIBUTION_POSITION_BASED" , GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleSearchAttributionTimeDecay => "GOOGLE_SEARCH_ATTRIBUTION_TIME_DECAY" , GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel
    {
        type Err = ();        fn from_str (s : & str) -> :: std :: result :: Result < GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel , () >{
            Ok (match s { "EXTERNAL" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: External , "GOOGLE_ADS_LAST_CLICK" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleAdsLastClick , "GOOGLE_SEARCH_ATTRIBUTION_DATA_DRIVEN" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleSearchAttributionDataDriven , "GOOGLE_SEARCH_ATTRIBUTION_FIRST_CLICK" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleSearchAttributionFirstClick , "GOOGLE_SEARCH_ATTRIBUTION_LINEAR" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleSearchAttributionLinear , "GOOGLE_SEARCH_ATTRIBUTION_POSITION_BASED" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleSearchAttributionPositionBased , "GOOGLE_SEARCH_ATTRIBUTION_TIME_DECAY" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleSearchAttributionTimeDecay , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: Unspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "EXTERNAL" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: External , "GOOGLE_ADS_LAST_CLICK" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleAdsLastClick , "GOOGLE_SEARCH_ATTRIBUTION_DATA_DRIVEN" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleSearchAttributionDataDriven , "GOOGLE_SEARCH_ATTRIBUTION_FIRST_CLICK" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleSearchAttributionFirstClick , "GOOGLE_SEARCH_ATTRIBUTION_LINEAR" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleSearchAttributionLinear , "GOOGLE_SEARCH_ATTRIBUTION_POSITION_BASED" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleSearchAttributionPositionBased , "GOOGLE_SEARCH_ATTRIBUTION_TIME_DECAY" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: GoogleSearchAttributionTimeDecay , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsAttributionModel
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus
    {
        #[doc = "The data driven model is available."]
        Available,
        #[doc = "The data driven model expired. It hasn’t been updated for at least 30 days and cannot be used. Most commonly this is because there hasn’t been the required number of events in a recent 30-day period."]
        Expired,
        #[doc = "The data driven model has never been generated. Most commonly this is because there has never been the required number of events in any 30-day period."]
        NeverGenerated,
        #[doc = "The data driven model is stale. It hasn’t been updated for at least 7 days. It is still being used, but will become expired if it does not get updated for 30 days."]
        Stale,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: Available => "AVAILABLE" , GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: Expired => "EXPIRED" , GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: NeverGenerated => "NEVER_GENERATED" , GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: Stale => "STALE" , GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl :: std :: convert :: AsRef < str > for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus { fn as_ref (& self) -> & str { self . as_str () } }
    impl :: std :: str :: FromStr for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus { type Err = () ; fn from_str (s : & str) -> :: std :: result :: Result < GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus , () > { Ok (match s { "AVAILABLE" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: Available , "EXPIRED" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: Expired , "NEVER_GENERATED" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: NeverGenerated , "STALE" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: Stale , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: Unspecified , _ => return Err (()) , }) } }
    impl :: std :: fmt :: Display for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> :: std :: fmt :: Result { f . write_str (self . as_str ()) } }
    impl :: serde :: Serialize for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus { fn serialize < S > (& self , serializer : S) -> :: std :: result :: Result < S :: Ok , S :: Error > where S : :: serde :: ser :: Serializer { serializer . serialize_str (self . as_str ()) } }
    impl < 'de > :: serde :: Deserialize < 'de > for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus { fn deserialize < D > (deserializer : D) -> :: std :: result :: Result < Self , D :: Error > where D : :: serde :: de :: Deserializer < 'de > , { let value : & 'de str = < & str > :: deserialize (deserializer) ? ; Ok (match value { "AVAILABLE" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: Available , "EXPIRED" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: Expired , "NEVER_GENERATED" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: NeverGenerated , "STALE" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: Stale , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , }) } }
    impl :: google_field_selector :: FieldSelector for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus { fn fields () -> Vec < :: google_field_selector :: Field > { Vec :: new () } }
    impl :: google_field_selector :: ToFieldType for GoogleAdsSearchads360V0ResourcesConversionActionAttributionModelSettingsDataDrivenModelStatus { fn field_type () -> :: google_field_selector :: FieldType { :: google_field_selector :: FieldType :: Leaf } }
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
    pub struct GoogleAdsSearchads360V0ResourcesConversionActionFloodlightSettings {
        #[doc = "Output only. String used to identify a Floodlight activity group when reporting conversions."]
        #[serde(
            rename = "activityGroupTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub activity_group_tag: ::std::option::Option<String>,
        #[doc = "Output only. ID of the Floodlight activity in DoubleClick Campaign Manager (DCM)."]
        #[serde(
            rename = "activityId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub activity_id: ::std::option::Option<i64>,
        #[doc = "Output only. String used to identify a Floodlight activity when reporting conversions."]
        #[serde(
            rename = "activityTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub activity_tag: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesConversionActionFloodlightSettings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesConversionActionFloodlightSettings
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0ResourcesConversionActionValueSettings {
        #[doc = "Controls whether the default value and default currency code are used in place of the value and currency code specified in conversion events for this conversion action."]
        #[serde(
            rename = "alwaysUseDefaultValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub always_use_default_value: ::std::option::Option<bool>,
        #[doc = "The currency code to use when conversion events for this conversion action are sent with an invalid or missing currency code, or when this conversion action is configured to always use the default value."]
        #[serde(
            rename = "defaultCurrencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_currency_code: ::std::option::Option<String>,
        #[doc = "The value to use when conversion events for this conversion action are sent with an invalid, disallowed or missing value, or when this conversion action is configured to always use the default value."]
        #[serde(
            rename = "defaultValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_value: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesConversionActionValueSettings
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesConversionActionValueSettings
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
    pub struct GoogleAdsSearchads360V0ResourcesConversionTrackingSetting { # [doc = "Output only. Whether the customer has accepted customer data terms. If using cross-account conversion tracking, this value is inherited from the manager. This field is read-only. For more information, see https://support.google.com/adspolicy/answer/7475709."] # [serde (rename = "acceptedCustomerDataTerms" , default , skip_serializing_if = "std::option::Option::is_none")] pub accepted_customer_data_terms : :: std :: option :: Option < bool > , # [doc = "Output only. The conversion tracking id used for this account. This id doesn’t indicate whether the customer uses conversion tracking (conversion_tracking_status does). This field is read-only."] # [serde (rename = "conversionTrackingId" , default , skip_serializing_if = "std::option::Option::is_none")] # [serde (with = "crate::parsed_string")] pub conversion_tracking_id : :: std :: option :: Option < i64 > , # [doc = "Output only. Conversion tracking status. It indicates whether the customer is using conversion tracking, and who is the conversion tracking owner of this customer. If this customer is using cross-account conversion tracking, the value returned will differ based on the `login-customer-id` of the request."] # [serde (rename = "conversionTrackingStatus" , default , skip_serializing_if = "std::option::Option::is_none")] pub conversion_tracking_status : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus > , # [doc = "Output only. The conversion tracking id of the customer’s manager. This is set when the customer is opted into cross-account conversion tracking, and it overrides conversion_tracking_id."] # [serde (rename = "crossAccountConversionTrackingId" , default , skip_serializing_if = "std::option::Option::is_none")] # [serde (with = "crate::parsed_string")] pub cross_account_conversion_tracking_id : :: std :: option :: Option < i64 > , # [doc = "Output only. Whether the customer is opted-in for enhanced conversions for leads. If using cross-account conversion tracking, this value is inherited from the manager. This field is read-only."] # [serde (rename = "enhancedConversionsForLeadsEnabled" , default , skip_serializing_if = "std::option::Option::is_none")] pub enhanced_conversions_for_leads_enabled : :: std :: option :: Option < bool > , # [doc = "Output only. The resource name of the customer where conversions are created and managed. This field is read-only."] # [serde (rename = "googleAdsConversionCustomer" , default , skip_serializing_if = "std::option::Option::is_none")] pub google_ads_conversion_customer : :: std :: option :: Option < String > , # [doc = "Output only. The conversion tracking id of the customer’s manager. This is set when the customer is opted into conversion tracking, and it overrides conversion_tracking_id. This field can only be managed through the Google Ads UI. This field is read-only."] # [serde (rename = "googleAdsCrossAccountConversionTrackingId" , default , skip_serializing_if = "std::option::Option::is_none")] # [serde (with = "crate::parsed_string")] pub google_ads_cross_account_conversion_tracking_id : :: std :: option :: Option < i64 > , }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesConversionTrackingSetting
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesConversionTrackingSetting
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus {
        #[doc = "The conversion actions are created and managed by a manager different from the customer or manager specified in the request’s `login-customer-id`."]
        ConversionTrackingManagedByAnotherManager,
        #[doc = "The conversion actions are created and managed by this customer."]
        ConversionTrackingManagedBySelf,
        #[doc = "The conversion actions are created and managed by the manager specified in the request’s `login-customer-id`."]
        ConversionTrackingManagedByThisManager,
        #[doc = "Customer does not use any conversion tracking."]
        NotConversionTracked,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: ConversionTrackingManagedByAnotherManager => "CONVERSION_TRACKING_MANAGED_BY_ANOTHER_MANAGER" , GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: ConversionTrackingManagedBySelf => "CONVERSION_TRACKING_MANAGED_BY_SELF" , GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: ConversionTrackingManagedByThisManager => "CONVERSION_TRACKING_MANAGED_BY_THIS_MANAGER" , GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: NotConversionTracked => "NOT_CONVERSION_TRACKED" , GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus,
            (),
        > {
            Ok (match s { "CONVERSION_TRACKING_MANAGED_BY_ANOTHER_MANAGER" => GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: ConversionTrackingManagedByAnotherManager , "CONVERSION_TRACKING_MANAGED_BY_SELF" => GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: ConversionTrackingManagedBySelf , "CONVERSION_TRACKING_MANAGED_BY_THIS_MANAGER" => GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: ConversionTrackingManagedByThisManager , "NOT_CONVERSION_TRACKED" => GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: NotConversionTracked , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: Unspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CONVERSION_TRACKING_MANAGED_BY_ANOTHER_MANAGER" => GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: ConversionTrackingManagedByAnotherManager , "CONVERSION_TRACKING_MANAGED_BY_SELF" => GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: ConversionTrackingManagedBySelf , "CONVERSION_TRACKING_MANAGED_BY_THIS_MANAGER" => GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: ConversionTrackingManagedByThisManager , "NOT_CONVERSION_TRACKED" => GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: NotConversionTracked , "UNKNOWN" => GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesConversionTrackingSettingConversionTrackingStatus
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
    pub struct GoogleAdsSearchads360V0ResourcesCustomColumn {
        #[doc = "Output only. User-defined description of the custom column."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Output only. ID of the custom column."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<i64>,
        #[doc = "Output only. User-defined name of the custom column."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. True when the custom column is available to be used in the query of SearchAds360Service.Search and SearchAds360Service.SearchStream."]
        #[serde(
            rename = "queryable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub queryable: ::std::option::Option<bool>,
        #[doc = "Output only. The list of the referenced system columns of this custom column. For example, A custom column “sum of impressions and clicks” has referenced system columns of {“metrics.clicks”, “metrics.impressions”}."]
        #[serde(
            rename = "referencedSystemColumns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referenced_system_columns: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. True when the custom column is referring to one or more attributes."]
        #[serde(
            rename = "referencesAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub references_attributes: ::std::option::Option<bool>,
        #[doc = "Output only. True when the custom column is referring to one or more metrics."]
        #[serde(
            rename = "referencesMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub references_metrics: ::std::option::Option<bool>,
        #[doc = "Immutable. The resource name of the custom column. Custom column resource names have the form: `customers/{customer_id}/customColumns/{custom_column_id}`"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "Output only. The type of the result value of the custom column."]
        #[serde(
            rename = "valueType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_type: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesCustomColumnValueType,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesCustomColumn {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesCustomColumn {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCustomColumnValueType {
        #[doc = "The custom column value is a boolean."]
        Boolean,
        #[doc = "The custom column value is a double number."]
        Double,
        #[doc = "The custom column value is an int64 number."]
        Int64,
        #[doc = "The custom column value is a string."]
        String,
        #[doc = "Unknown."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesCustomColumnValueType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesCustomColumnValueType::Boolean => "BOOLEAN",
                GoogleAdsSearchads360V0ResourcesCustomColumnValueType::Double => "DOUBLE",
                GoogleAdsSearchads360V0ResourcesCustomColumnValueType::Int64 => "INT64",
                GoogleAdsSearchads360V0ResourcesCustomColumnValueType::String => "STRING",
                GoogleAdsSearchads360V0ResourcesCustomColumnValueType::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesCustomColumnValueType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesCustomColumnValueType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCustomColumnValueType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesCustomColumnValueType, ()>
        {
            Ok(match s {
                "BOOLEAN" => GoogleAdsSearchads360V0ResourcesCustomColumnValueType::Boolean,
                "DOUBLE" => GoogleAdsSearchads360V0ResourcesCustomColumnValueType::Double,
                "INT64" => GoogleAdsSearchads360V0ResourcesCustomColumnValueType::Int64,
                "STRING" => GoogleAdsSearchads360V0ResourcesCustomColumnValueType::String,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCustomColumnValueType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCustomColumnValueType::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCustomColumnValueType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCustomColumnValueType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesCustomColumnValueType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOOLEAN" => GoogleAdsSearchads360V0ResourcesCustomColumnValueType::Boolean,
                "DOUBLE" => GoogleAdsSearchads360V0ResourcesCustomColumnValueType::Double,
                "INT64" => GoogleAdsSearchads360V0ResourcesCustomColumnValueType::Int64,
                "STRING" => GoogleAdsSearchads360V0ResourcesCustomColumnValueType::String,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCustomColumnValueType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCustomColumnValueType::Unspecified,
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
        for GoogleAdsSearchads360V0ResourcesCustomColumnValueType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCustomColumnValueType
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
    pub struct GoogleAdsSearchads360V0ResourcesCustomer {
        #[doc = "Output only. Account status, for example, Enabled, Paused, Removed, etc."]
        #[serde(
            rename = "accountStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_status: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesCustomerAccountStatus,
        >,
        #[doc = "Output only. Engine account type, for example, Google Ads, Microsoft Advertising, Yahoo Japan, Baidu, Facebook, Engine Track, etc."]
        #[serde(
            rename = "accountType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_type: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesCustomerAccountType,
        >,
        #[doc = "Whether auto-tagging is enabled for the customer."]
        #[serde(
            rename = "autoTaggingEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_tagging_enabled: ::std::option::Option<bool>,
        #[doc = "Output only. Conversion tracking setting for a customer."]
        #[serde(
            rename = "conversionTrackingSetting",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversion_tracking_setting: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesConversionTrackingSetting,
        >,
        #[doc = "Immutable. The currency in which the account operates. A subset of the currency codes from the ISO 4217 standard is supported."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Optional, non-unique descriptive name of the customer."]
        #[serde(
            rename = "descriptiveName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub descriptive_name: ::std::option::Option<String>,
        #[doc = "Output only. DoubleClick Campaign Manager (DCM) setting for a manager customer."]
        #[serde(
            rename = "doubleClickCampaignManagerSetting",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub double_click_campaign_manager_setting: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesDoubleClickCampaignManagerSetting,
        >,
        #[doc = "Output only. ID of the account in the external engine account."]
        #[serde(
            rename = "engineId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub engine_id: ::std::option::Option<String>,
        #[doc = "The URL template for appending params to the final URL"]
        #[serde(
            rename = "finalUrlSuffix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub final_url_suffix: ::std::option::Option<String>,
        #[doc = "Output only. The ID of the customer."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<i64>,
        #[doc = "Output only. The datetime when this customer was last modified. The datetime is in the customer’s time zone and in “yyyy-MM-dd HH:mm:ss.ssssss” format."]
        #[serde(
            rename = "lastModifiedTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_modified_time: ::std::option::Option<String>,
        #[doc = "Output only. Whether the customer is a manager."]
        #[serde(
            rename = "manager",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manager: ::std::option::Option<bool>,
        #[doc = "Immutable. The resource name of the customer. Customer resource names have the form: `customers/{customer_id}`"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "Output only. The status of the customer."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ResourcesCustomerStatus>,
        #[doc = "Immutable. The local timezone ID of the customer."]
        #[serde(
            rename = "timeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone: ::std::option::Option<String>,
        #[doc = "The URL template for constructing a tracking URL out of parameters."]
        #[serde(
            rename = "trackingUrlTemplate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracking_url_template: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesCustomer {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesCustomer {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCustomerAccountStatus {
        #[doc = "Account is still in the process of setup, not ENABLED yet."]
        Draft,
        #[doc = "Account is able to serve ads."]
        Enabled,
        #[doc = "Account is deactivated by the user."]
        Paused,
        #[doc = "Account is irrevocably deactivated."]
        Removed,
        #[doc = "Account is deactivated by an internal process."]
        Suspended,
        #[doc = "Unknown value."]
        Unknown,
        #[doc = "Default value."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesCustomerAccountStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Draft => "DRAFT",
                GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Enabled => "ENABLED",
                GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Paused => "PAUSED",
                GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Removed => "REMOVED",
                GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Suspended => "SUSPENDED",
                GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesCustomerAccountStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCustomerAccountStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesCustomerAccountStatus, ()>
        {
            Ok(match s {
                "DRAFT" => GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Draft,
                "ENABLED" => GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Enabled,
                "PAUSED" => GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Paused,
                "REMOVED" => GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Removed,
                "SUSPENDED" => GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Suspended,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCustomerAccountStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCustomerAccountStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesCustomerAccountStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DRAFT" => GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Draft,
                "ENABLED" => GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Enabled,
                "PAUSED" => GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Paused,
                "REMOVED" => GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Removed,
                "SUSPENDED" => GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Suspended,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCustomerAccountStatus::Unspecified,
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
        for GoogleAdsSearchads360V0ResourcesCustomerAccountStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCustomerAccountStatus
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCustomerAccountType {
        #[doc = "Baidu account."]
        Baidu,
        #[doc = "Engine track account."]
        EngineTrack,
        #[doc = "Facebook account."]
        Facebook,
        #[doc = "Facebook account managed through gateway."]
        FacebookGateway,
        #[doc = "Google Ads account."]
        GoogleAds,
        #[doc = "Microsoft Advertising account."]
        Microsoft,
        #[doc = "Search Ads 360 manager account."]
        SearchAds360,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
        #[doc = "Yahoo Japan account."]
        YahooJapan,
    }
    impl GoogleAdsSearchads360V0ResourcesCustomerAccountType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesCustomerAccountType::Baidu => "BAIDU",
                GoogleAdsSearchads360V0ResourcesCustomerAccountType::EngineTrack => "ENGINE_TRACK",
                GoogleAdsSearchads360V0ResourcesCustomerAccountType::Facebook => "FACEBOOK",
                GoogleAdsSearchads360V0ResourcesCustomerAccountType::FacebookGateway => {
                    "FACEBOOK_GATEWAY"
                }
                GoogleAdsSearchads360V0ResourcesCustomerAccountType::GoogleAds => "GOOGLE_ADS",
                GoogleAdsSearchads360V0ResourcesCustomerAccountType::Microsoft => "MICROSOFT",
                GoogleAdsSearchads360V0ResourcesCustomerAccountType::SearchAds360 => {
                    "SEARCH_ADS_360"
                }
                GoogleAdsSearchads360V0ResourcesCustomerAccountType::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesCustomerAccountType::Unspecified => "UNSPECIFIED",
                GoogleAdsSearchads360V0ResourcesCustomerAccountType::YahooJapan => "YAHOO_JAPAN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesCustomerAccountType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCustomerAccountType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesCustomerAccountType, ()>
        {
            Ok(match s {
                "BAIDU" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::Baidu,
                "ENGINE_TRACK" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::EngineTrack,
                "FACEBOOK" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::Facebook,
                "FACEBOOK_GATEWAY" => {
                    GoogleAdsSearchads360V0ResourcesCustomerAccountType::FacebookGateway
                }
                "GOOGLE_ADS" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::GoogleAds,
                "MICROSOFT" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::Microsoft,
                "SEARCH_ADS_360" => {
                    GoogleAdsSearchads360V0ResourcesCustomerAccountType::SearchAds360
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::Unspecified,
                "YAHOO_JAPAN" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::YahooJapan,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCustomerAccountType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCustomerAccountType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesCustomerAccountType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BAIDU" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::Baidu,
                "ENGINE_TRACK" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::EngineTrack,
                "FACEBOOK" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::Facebook,
                "FACEBOOK_GATEWAY" => {
                    GoogleAdsSearchads360V0ResourcesCustomerAccountType::FacebookGateway
                }
                "GOOGLE_ADS" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::GoogleAds,
                "MICROSOFT" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::Microsoft,
                "SEARCH_ADS_360" => {
                    GoogleAdsSearchads360V0ResourcesCustomerAccountType::SearchAds360
                }
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::Unspecified,
                "YAHOO_JAPAN" => GoogleAdsSearchads360V0ResourcesCustomerAccountType::YahooJapan,
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
        for GoogleAdsSearchads360V0ResourcesCustomerAccountType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesCustomerAccountType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCustomerStatus {
        #[doc = "Indicates a canceled account unable to serve ads. Can be reactivated by an admin user."]
        Canceled,
        #[doc = "Indicates a closed account unable to serve ads. Test account will also have CLOSED status. Status is permanent and may not be reopened."]
        Closed,
        #[doc = "Indicates an active account able to serve ads."]
        Enabled,
        #[doc = "Indicates a suspended account unable to serve ads. May only be activated by Google support."]
        Suspended,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesCustomerStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesCustomerStatus::Canceled => "CANCELED",
                GoogleAdsSearchads360V0ResourcesCustomerStatus::Closed => "CLOSED",
                GoogleAdsSearchads360V0ResourcesCustomerStatus::Enabled => "ENABLED",
                GoogleAdsSearchads360V0ResourcesCustomerStatus::Suspended => "SUSPENDED",
                GoogleAdsSearchads360V0ResourcesCustomerStatus::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesCustomerStatus::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesCustomerStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCustomerStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesCustomerStatus, ()> {
            Ok(match s {
                "CANCELED" => GoogleAdsSearchads360V0ResourcesCustomerStatus::Canceled,
                "CLOSED" => GoogleAdsSearchads360V0ResourcesCustomerStatus::Closed,
                "ENABLED" => GoogleAdsSearchads360V0ResourcesCustomerStatus::Enabled,
                "SUSPENDED" => GoogleAdsSearchads360V0ResourcesCustomerStatus::Suspended,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCustomerStatus::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCustomerStatus::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCustomerStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCustomerStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesCustomerStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELED" => GoogleAdsSearchads360V0ResourcesCustomerStatus::Canceled,
                "CLOSED" => GoogleAdsSearchads360V0ResourcesCustomerStatus::Closed,
                "ENABLED" => GoogleAdsSearchads360V0ResourcesCustomerStatus::Enabled,
                "SUSPENDED" => GoogleAdsSearchads360V0ResourcesCustomerStatus::Suspended,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCustomerStatus::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCustomerStatus::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesCustomerStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesCustomerStatus {
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
    pub struct GoogleAdsSearchads360V0ResourcesCustomerClient {
        #[doc = "Output only. The resource names of the labels owned by the requesting customer that are applied to the client customer. Label resource names have the form: `customers/{customer_id}/labels/{label_id}`"]
        #[serde(
            rename = "appliedLabels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub applied_labels: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. The resource name of the client-customer which is linked to the given customer. Read only."]
        #[serde(
            rename = "clientCustomer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub client_customer: ::std::option::Option<String>,
        #[doc = "Output only. Currency code (for example, ‘USD’, ‘EUR’) for the client. Read only."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Output only. Descriptive name for the client. Read only."]
        #[serde(
            rename = "descriptiveName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub descriptive_name: ::std::option::Option<String>,
        #[doc = "Output only. Specifies whether this is a hidden account. Read only."]
        #[serde(
            rename = "hidden",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hidden: ::std::option::Option<bool>,
        #[doc = "Output only. The ID of the client customer. Read only."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<i64>,
        #[doc = "Output only. Distance between given customer and client. For self link, the level value will be 0. Read only."]
        #[serde(
            rename = "level",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub level: ::std::option::Option<i64>,
        #[doc = "Output only. Identifies if the client is a manager. Read only."]
        #[serde(
            rename = "manager",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manager: ::std::option::Option<bool>,
        #[doc = "Output only. The resource name of the customer client. CustomerClient resource names have the form: `customers/{customer_id}/customerClients/{client_customer_id}`"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "Output only. The status of the client customer. Read only."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesCustomerClientStatus,
        >,
        #[doc = "Output only. Identifies if the client is a test account. Read only."]
        #[serde(
            rename = "testAccount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub test_account: ::std::option::Option<bool>,
        #[doc = "Output only. Common Locale Data Repository (CLDR) string representation of the time zone of the client, for example, America/Los_Angeles. Read only."]
        #[serde(
            rename = "timeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesCustomerClient {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesCustomerClient {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCustomerClientStatus {
        #[doc = "Indicates a canceled account unable to serve ads. Can be reactivated by an admin user."]
        Canceled,
        #[doc = "Indicates a closed account unable to serve ads. Test account will also have CLOSED status. Status is permanent and may not be reopened."]
        Closed,
        #[doc = "Indicates an active account able to serve ads."]
        Enabled,
        #[doc = "Indicates a suspended account unable to serve ads. May only be activated by Google support."]
        Suspended,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesCustomerClientStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Canceled => "CANCELED",
                GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Closed => "CLOSED",
                GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Enabled => "ENABLED",
                GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Suspended => "SUSPENDED",
                GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesCustomerClientStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCustomerClientStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesCustomerClientStatus, ()>
        {
            Ok(match s {
                "CANCELED" => GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Canceled,
                "CLOSED" => GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Closed,
                "ENABLED" => GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Enabled,
                "SUSPENDED" => GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Suspended,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCustomerClientStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCustomerClientStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesCustomerClientStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCELED" => GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Canceled,
                "CLOSED" => GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Closed,
                "ENABLED" => GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Enabled,
                "SUSPENDED" => GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Suspended,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Unknown,
                "UNSPECIFIED" => GoogleAdsSearchads360V0ResourcesCustomerClientStatus::Unspecified,
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
        for GoogleAdsSearchads360V0ResourcesCustomerClientStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesCustomerClientStatus {
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
    pub struct GoogleAdsSearchads360V0ResourcesCustomerManagerLink {
        #[doc = "Output only. The manager customer linked to the customer."]
        #[serde(
            rename = "managerCustomer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manager_customer: ::std::option::Option<String>,
        #[doc = "Output only. ID of the customer-manager link. This field is read only."]
        #[serde(
            rename = "managerLinkId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub manager_link_id: ::std::option::Option<i64>,
        #[doc = "Immutable. Name of the resource. CustomerManagerLink resource names have the form: `customers/{customer_id}/customerManagerLinks/{manager_customer_id}~{manager_link_id}`"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "Status of the link between the customer and the manager."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesCustomerManagerLink
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesCustomerManagerLink {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus {
        #[doc = "Indicates current in-effect relationship"]
        Active,
        #[doc = "Indicates relationship has been requested by manager, but manager canceled it."]
        Canceled,
        #[doc = "Indicates terminated relationship"]
        Inactive,
        #[doc = "Indicates relationship has been requested by manager, but the client hasn’t accepted yet."]
        Pending,
        #[doc = "Relationship was requested by the manager, but the client has refused."]
        Refused,
        #[doc = "Used for return value only. Represents value unknown in this version."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Active => "ACTIVE",
                GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Canceled => "CANCELED",
                GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Inactive => "INACTIVE",
                GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Pending => "PENDING",
                GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Refused => "REFUSED",
                GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Unspecified => {
                    "UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus, ()>
        {
            Ok(match s {
                "ACTIVE" => GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Active,
                "CANCELED" => GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Canceled,
                "INACTIVE" => GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Inactive,
                "PENDING" => GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Pending,
                "REFUSED" => GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Refused,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Unspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Active,
                "CANCELED" => GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Canceled,
                "INACTIVE" => GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Inactive,
                "PENDING" => GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Pending,
                "REFUSED" => GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Refused,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus::Unspecified
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
        for GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesCustomerManagerLinkStatus
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
    pub struct GoogleAdsSearchads360V0ResourcesDoubleClickCampaignManagerSetting {
        #[doc = "Output only. ID of the Campaign Manager advertiser associated with this customer."]
        #[serde(
            rename = "advertiserId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub advertiser_id: ::std::option::Option<i64>,
        #[doc = "Output only. ID of the Campaign Manager network associated with this customer."]
        #[serde(
            rename = "networkId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub network_id: ::std::option::Option<i64>,
        #[doc = "Output only. Time zone of the Campaign Manager network associated with this customer in IANA Time Zone Database format, such as America/New_York."]
        #[serde(
            rename = "timeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ResourcesDoubleClickCampaignManagerSetting
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesDoubleClickCampaignManagerSetting
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
    pub struct GoogleAdsSearchads360V0ResourcesKeywordView {
        #[doc = "Output only. The resource name of the keyword view. Keyword view resource names have the form: `customers/{customer_id}/keywordViews/{ad_group_id}~{criterion_id}`"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesKeywordView {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesKeywordView {
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
    pub struct GoogleAdsSearchads360V0ResourcesProductGroupView {
        #[doc = "Output only. The resource name of the product group view. Product group view resource names have the form: `customers/{customer_id}/productGroupViews/{ad_group_id}~{criterion_id}`"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesProductGroupView {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesProductGroupView {
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
    pub struct GoogleAdsSearchads360V0ResourcesSearchAds360Field {
        #[doc = "Output only. The names of all resources that are selectable with the described artifact. Fields from these resources do not segment metrics when included in search queries. This field is only set for artifacts whose category is RESOURCE."]
        #[serde(
            rename = "attributeResources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attribute_resources: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. The category of the artifact."]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory,
        >,
        #[doc = "Output only. This field determines the operators that can be used with the artifact in WHERE clauses."]
        #[serde(
            rename = "dataType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_type: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType,
        >,
        #[doc = "Output only. Values the artifact can assume if it is a field of type ENUM. This field is only set for artifacts of category SEGMENT or ATTRIBUTE."]
        #[serde(
            rename = "enumValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enum_values: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. Whether the artifact can be used in a WHERE clause in search queries."]
        #[serde(
            rename = "filterable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filterable: ::std::option::Option<bool>,
        #[doc = "Output only. Whether the field artifact is repeated."]
        #[serde(
            rename = "isRepeated",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_repeated: ::std::option::Option<bool>,
        #[doc = "Output only. This field lists the names of all metrics that are selectable with the described artifact when it is used in the FROM clause. It is only set for artifacts whose category is RESOURCE."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. The name of the artifact."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. The resource name of the artifact. Artifact resource names have the form: `SearchAds360Fields/{name}`"]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
        #[doc = "Output only. This field lists the names of all artifacts, whether a segment or another resource, that segment metrics when included in search queries and when the described artifact is used in the FROM clause. It is only set for artifacts whose category is RESOURCE."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. Whether the artifact can be used in a SELECT clause in search queries."]
        #[serde(
            rename = "selectable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selectable: ::std::option::Option<bool>,
        #[doc = "Output only. The names of all resources, segments, and metrics that are selectable with the described artifact."]
        #[serde(
            rename = "selectableWith",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selectable_with: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. Whether the artifact can be used in a ORDER BY clause in search queries."]
        #[serde(
            rename = "sortable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sortable: ::std::option::Option<bool>,
        #[doc = "Output only. The URL of proto describing the artifact’s data type."]
        #[serde(
            rename = "typeUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub type_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ResourcesSearchAds360Field {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ResourcesSearchAds360Field {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory {
        #[doc = "The described artifact is a field and is an attribute of a resource. Including a resource attribute field in a query may segment the query if the resource to which it is attributed segments the resource found in the FROM clause."]
        Attribute,
        #[doc = "The described artifact is a field and is a metric. It never segments search queries."]
        Metric,
        #[doc = "The described artifact is a resource."]
        Resource,
        #[doc = "The described artifact is a field and always segments search queries."]
        Segment,
        #[doc = "Unknown"]
        Unknown,
        #[doc = "Unspecified"]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Attribute => "ATTRIBUTE",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Metric => "METRIC",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Resource => "RESOURCE",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Segment => "SEGMENT",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Unspecified => {
                    "UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory, ()>
        {
            Ok(match s {
                "ATTRIBUTE" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Attribute,
                "METRIC" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Metric,
                "RESOURCE" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Resource,
                "SEGMENT" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Segment,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Unspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ATTRIBUTE" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Attribute,
                "METRIC" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Metric,
                "RESOURCE" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Resource,
                "SEGMENT" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Segment,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory::Unspecified
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
        for GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesSearchAds360FieldCategory
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType {
        #[doc = "Maps to google.protobuf.BoolValue Applicable operators: =, !="]
        Boolean,
        #[doc = "Maps to google.protobuf.StringValue. It can be compared using the set of operators specific to dates however. Applicable operators: =, \\<, >, \\<=, >=, BETWEEN, DURING, and IN"]
        Date,
        #[doc = "Maps to google.protobuf.DoubleValue Applicable operators: =, !=, \\<, >, IN, NOT IN"]
        Double,
        #[doc = "Maps to an enum. It’s specific definition can be found at type_url. Applicable operators: =, !=, IN, NOT IN"]
        Enum,
        #[doc = "Maps to google.protobuf.FloatValue Applicable operators: =, !=, \\<, >, IN, NOT IN"]
        Float,
        #[doc = "Maps to google.protobuf.Int32Value Applicable operators: =, !=, \\<, >, \\<=, >=, BETWEEN, IN, NOT IN"]
        Int32,
        #[doc = "Maps to google.protobuf.Int64Value Applicable operators: =, !=, \\<, >, \\<=, >=, BETWEEN, IN, NOT IN"]
        Int64,
        #[doc = "Maps to a protocol buffer message type. The data type’s details can be found in type_url. No operators work with MESSAGE fields."]
        Message,
        #[doc = "Maps to google.protobuf.StringValue. Represents the resource name (unique id) of a resource or one of its foreign keys. No operators work with RESOURCE_NAME fields."]
        ResourceName,
        #[doc = "Maps to google.protobuf.StringValue. Applicable operators: =, !=, LIKE, NOT LIKE, IN, NOT IN"]
        String,
        #[doc = "Maps to google.protobuf.UInt64Value Applicable operators: =, !=, \\<, >, \\<=, >=, BETWEEN, IN, NOT IN"]
        Uint64,
        #[doc = "Unknown"]
        Unknown,
        #[doc = "Unspecified"]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Boolean => "BOOLEAN",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Date => "DATE",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Double => "DOUBLE",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Enum => "ENUM",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Float => "FLOAT",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Int32 => "INT32",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Int64 => "INT64",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Message => "MESSAGE",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::ResourceName => {
                    "RESOURCE_NAME"
                }
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::String => "STRING",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Uint64 => "UINT64",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Unknown => "UNKNOWN",
                GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Unspecified => {
                    "UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType, ()>
        {
            Ok(match s {
                "BOOLEAN" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Boolean,
                "DATE" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Date,
                "DOUBLE" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Double,
                "ENUM" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Enum,
                "FLOAT" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Float,
                "INT32" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Int32,
                "INT64" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Int64,
                "MESSAGE" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Message,
                "RESOURCE_NAME" => {
                    GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::ResourceName
                }
                "STRING" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::String,
                "UINT64" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Uint64,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Unspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOOLEAN" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Boolean,
                "DATE" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Date,
                "DOUBLE" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Double,
                "ENUM" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Enum,
                "FLOAT" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Float,
                "INT32" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Int32,
                "INT64" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Int64,
                "MESSAGE" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Message,
                "RESOURCE_NAME" => {
                    GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::ResourceName
                }
                "STRING" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::String,
                "UINT64" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Uint64,
                "UNKNOWN" => GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Unknown,
                "UNSPECIFIED" => {
                    GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType::Unspecified
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
        for GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ResourcesSearchAds360FieldDataType
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
    pub struct GoogleAdsSearchads360V0ServicesCustomColumnHeader {
        #[doc = "The custom column ID."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<i64>,
        #[doc = "The user defined name of the custom column."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "True when the custom column references metrics."]
        #[serde(
            rename = "referencesMetrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub references_metrics: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ServicesCustomColumnHeader {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ServicesCustomColumnHeader {
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
    pub struct GoogleAdsSearchads360V0ServicesListCustomColumnsResponse {
        #[doc = "The CustomColumns owned by the provided customer."]
        #[serde(
            rename = "customColumns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_columns: ::std::option::Option<
            Vec<crate::schemas::GoogleAdsSearchads360V0ResourcesCustomColumn>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ServicesListCustomColumnsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ServicesListCustomColumnsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0ServicesSearchAds360Row {
        #[doc = "The ad group referenced in the query."]
        #[serde(
            rename = "adGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_group:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ResourcesAdGroup>,
        #[doc = "The bid modifier referenced in the query."]
        #[serde(
            rename = "adGroupBidModifier",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_group_bid_modifier: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesAdGroupBidModifier,
        >,
        #[doc = "The criterion referenced in the query."]
        #[serde(
            rename = "adGroupCriterion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_group_criterion:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ResourcesAdGroupCriterion>,
        #[doc = "The bidding strategy referenced in the query."]
        #[serde(
            rename = "biddingStrategy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bidding_strategy:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ResourcesBiddingStrategy>,
        #[doc = "The campaign referenced in the query."]
        #[serde(
            rename = "campaign",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub campaign:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ResourcesCampaign>,
        #[doc = "The campaign budget referenced in the query."]
        #[serde(
            rename = "campaignBudget",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub campaign_budget:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ResourcesCampaignBudget>,
        #[doc = "The campaign criterion referenced in the query."]
        #[serde(
            rename = "campaignCriterion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub campaign_criterion: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesCampaignCriterion,
        >,
        #[doc = "The conversion action referenced in the query."]
        #[serde(
            rename = "conversionAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub conversion_action:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ResourcesConversionAction>,
        #[doc = "The custom columns."]
        #[serde(
            rename = "customColumns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_columns:
            ::std::option::Option<Vec<crate::schemas::GoogleAdsSearchads360V0CommonValue>>,
        #[doc = "The customer referenced in the query."]
        #[serde(
            rename = "customer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ResourcesCustomer>,
        #[doc = "The CustomerClient referenced in the query."]
        #[serde(
            rename = "customerClient",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer_client:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ResourcesCustomerClient>,
        #[doc = "The CustomerManagerLink referenced in the query."]
        #[serde(
            rename = "customerManagerLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer_manager_link: ::std::option::Option<
            crate::schemas::GoogleAdsSearchads360V0ResourcesCustomerManagerLink,
        >,
        #[doc = "The keyword view referenced in the query."]
        #[serde(
            rename = "keywordView",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keyword_view:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ResourcesKeywordView>,
        #[doc = "The metrics."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonMetrics>,
        #[doc = "The product group view referenced in the query."]
        #[serde(
            rename = "productGroupView",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_group_view:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ResourcesProductGroupView>,
        #[doc = "The segments."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0CommonSegments>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsSearchads360V0ServicesSearchAds360Row {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsSearchads360V0ServicesSearchAds360Row {
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
    pub struct GoogleAdsSearchads360V0ServicesSearchSearchAds360FieldsRequest {
        #[doc = "Number of elements to retrieve in a single page. When too large a page is requested, the server may decide to further limit the number of returned resources."]
        #[serde(
            rename = "pageSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_size: ::std::option::Option<i32>,
        #[doc = "Token of the page to retrieve. If not specified, the first page of results will be returned. Use the value obtained from `next_page_token` in the previous response in order to request the next page of results."]
        #[serde(
            rename = "pageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_token: ::std::option::Option<String>,
        #[doc = "Required. The query string."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360FieldsRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360FieldsRequest
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
    pub struct GoogleAdsSearchads360V0ServicesSearchSearchAds360FieldsResponse {
        #[doc = "Pagination token used to retrieve the next page of results. Pass the content of this string as the `page_token` attribute of the next request. `next_page_token` is not returned for the last page."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of fields that matched the query."]
        #[serde(
            rename = "results",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub results: ::std::option::Option<
            Vec<crate::schemas::GoogleAdsSearchads360V0ResourcesSearchAds360Field>,
        >,
        #[doc = "Total number of results that match the query ignoring the LIMIT clause."]
        #[serde(
            rename = "totalResultsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_results_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360FieldsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360FieldsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String>
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360FieldsResponse
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
    pub struct GoogleAdsSearchads360V0ServicesSearchSearchAds360Request { # [doc = "Number of elements to retrieve in a single page. When too large a page is requested, the server may decide to further limit the number of returned resources."] # [serde (rename = "pageSize" , default , skip_serializing_if = "std::option::Option::is_none")] pub page_size : :: std :: option :: Option < i32 > , # [doc = "Token of the page to retrieve. If not specified, the first page of results will be returned. Use the value obtained from `next_page_token` in the previous response in order to request the next page of results."] # [serde (rename = "pageToken" , default , skip_serializing_if = "std::option::Option::is_none")] pub page_token : :: std :: option :: Option < String > , # [doc = "Required. The query string."] # [serde (rename = "query" , default , skip_serializing_if = "std::option::Option::is_none")] pub query : :: std :: option :: Option < String > , # [doc = "If true, the total number of results that match the query ignoring the LIMIT clause will be included in the response. Default is false."] # [serde (rename = "returnTotalResultsCount" , default , skip_serializing_if = "std::option::Option::is_none")] pub return_total_results_count : :: std :: option :: Option < bool > , # [doc = "Determines whether a summary row will be returned. By default, summary row is not returned. If requested, the summary row will be sent in a response by itself after all other query results are returned."] # [serde (rename = "summaryRowSetting" , default , skip_serializing_if = "std::option::Option::is_none")] pub summary_row_setting : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting > , # [doc = "If true, the request is validated but not executed."] # [serde (rename = "validateOnly" , default , skip_serializing_if = "std::option::Option::is_none")] pub validate_only : :: std :: option :: Option < bool > , }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360Request
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360Request
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting {
        #[doc = "Do not return summary row."]
        NoSummaryRow,
        #[doc = "Return summary row only and return no results."]
        SummaryRowOnly,
        #[doc = "Return summary row along with results. The summary row will be returned in the last batch alone (last batch will contain no results)."]
        SummaryRowWithResults,
        #[doc = "Represent unknown values of return summary row."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting :: NoSummaryRow => "NO_SUMMARY_ROW" , GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting :: SummaryRowOnly => "SUMMARY_ROW_ONLY" , GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting :: SummaryRowWithResults => "SUMMARY_ROW_WITH_RESULTS" , GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting,
            (),
        > {
            Ok (match s { "NO_SUMMARY_ROW" => GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting :: NoSummaryRow , "SUMMARY_ROW_ONLY" => GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting :: SummaryRowOnly , "SUMMARY_ROW_WITH_RESULTS" => GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting :: SummaryRowWithResults , "UNKNOWN" => GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting :: Unspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "NO_SUMMARY_ROW" => GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting :: NoSummaryRow , "SUMMARY_ROW_ONLY" => GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting :: SummaryRowOnly , "SUMMARY_ROW_WITH_RESULTS" => GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting :: SummaryRowWithResults , "UNKNOWN" => GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360RequestSummaryRowSetting
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0ServicesSearchSearchAds360Response {
        #[doc = "The headers of the custom columns in the results."]
        #[serde(
            rename = "customColumnHeaders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_column_headers: ::std::option::Option<
            Vec<crate::schemas::GoogleAdsSearchads360V0ServicesCustomColumnHeader>,
        >,
        #[doc = "FieldMask that represents what fields were requested by the user."]
        #[serde(
            rename = "fieldMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_mask: ::std::option::Option<String>,
        #[doc = "Pagination token used to retrieve the next page of results. Pass the content of this string as the `page_token` attribute of the next request. `next_page_token` is not returned for the last page."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of rows that matched the query."]
        #[serde(
            rename = "results",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub results: ::std::option::Option<
            Vec<crate::schemas::GoogleAdsSearchads360V0ServicesSearchAds360Row>,
        >,
        #[doc = "Summary row that contains summary of metrics in results. Summary of metrics means aggregation of metrics across all results, here aggregation could be sum, average, rate, etc."]
        #[serde(
            rename = "summaryRow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub summary_row:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ServicesSearchAds360Row>,
        #[doc = "Total number of results that match the query ignoring the LIMIT clause."]
        #[serde(
            rename = "totalResultsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_results_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360Response
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360Response
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for GoogleAdsSearchads360V0ServicesSearchSearchAds360Response {
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
    pub struct GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequest { # [doc = "The number of rows that are returned in each stream response batch. When too large batch is requested, the server may decide to further limit the number of returned rows."] # [serde (rename = "batchSize" , default , skip_serializing_if = "std::option::Option::is_none")] pub batch_size : :: std :: option :: Option < i32 > , # [doc = "Required. The query string."] # [serde (rename = "query" , default , skip_serializing_if = "std::option::Option::is_none")] pub query : :: std :: option :: Option < String > , # [doc = "Determines whether a summary row will be returned. By default, summary row is not returned. If requested, the summary row will be sent in a response by itself after all other query results are returned."] # [serde (rename = "summaryRowSetting" , default , skip_serializing_if = "std::option::Option::is_none")] pub summary_row_setting : :: std :: option :: Option < crate :: schemas :: GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting > , }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting {
        #[doc = "Do not return summary row."]
        NoSummaryRow,
        #[doc = "Return summary row only and return no results."]
        SummaryRowOnly,
        #[doc = "Return summary row along with results. The summary row will be returned in the last batch alone (last batch will contain no results)."]
        SummaryRowWithResults,
        #[doc = "Represent unknown values of return summary row."]
        Unknown,
        #[doc = "Not specified."]
        Unspecified,
    }
    impl GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting :: NoSummaryRow => "NO_SUMMARY_ROW" , GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting :: SummaryRowOnly => "SUMMARY_ROW_ONLY" , GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting :: SummaryRowWithResults => "SUMMARY_ROW_WITH_RESULTS" , GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting :: Unknown => "UNKNOWN" , GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting :: Unspecified => "UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting,
            (),
        > {
            Ok (match s { "NO_SUMMARY_ROW" => GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting :: NoSummaryRow , "SUMMARY_ROW_ONLY" => GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting :: SummaryRowOnly , "SUMMARY_ROW_WITH_RESULTS" => GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting :: SummaryRowWithResults , "UNKNOWN" => GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting :: Unspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "NO_SUMMARY_ROW" => GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting :: NoSummaryRow , "SUMMARY_ROW_ONLY" => GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting :: SummaryRowOnly , "SUMMARY_ROW_WITH_RESULTS" => GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting :: SummaryRowWithResults , "UNKNOWN" => GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting :: Unknown , "UNSPECIFIED" => GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting :: Unspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequestSummaryRowSetting
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamResponse {
        #[doc = "The headers of the custom columns in the results."]
        #[serde(
            rename = "customColumnHeaders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_column_headers: ::std::option::Option<
            Vec<crate::schemas::GoogleAdsSearchads360V0ServicesCustomColumnHeader>,
        >,
        #[doc = "FieldMask that represents what fields were requested by the user."]
        #[serde(
            rename = "fieldMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_mask: ::std::option::Option<String>,
        #[doc = "The unique id of the request that is used for debugging purposes."]
        #[serde(
            rename = "requestId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_id: ::std::option::Option<String>,
        #[doc = "The list of rows that matched the query."]
        #[serde(
            rename = "results",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub results: ::std::option::Option<
            Vec<crate::schemas::GoogleAdsSearchads360V0ServicesSearchAds360Row>,
        >,
        #[doc = "Summary row that contains summary of metrics in results. Summary of metrics means aggregation of metrics across all results, here aggregation could be sum, average, rate, etc."]
        #[serde(
            rename = "summaryRow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub summary_row:
            ::std::option::Option<crate::schemas::GoogleAdsSearchads360V0ServicesSearchAds360Row>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamResponse
    {
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
    #[doc = "Actions that can be performed on the customers resource"]
    pub fn customers(&self) -> crate::resources::customers::CustomersActions {
        crate::resources::customers::CustomersActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the search_ads_360_fields resource"]
    pub fn search_ads_360_fields(
        &self,
    ) -> crate::resources::search_ads_360_fields::SearchAds360FieldsActions {
        crate::resources::search_ads_360_fields::SearchAds360FieldsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod customers {
        pub mod params {}
        pub struct CustomersActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> CustomersActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the custom_columns resource"]
            pub fn custom_columns(
                &self,
            ) -> crate::resources::customers::custom_columns::CustomColumnsActions {
                crate::resources::customers::custom_columns::CustomColumnsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the search_ads_360 resource"]
            pub fn search_ads_360(
                &self,
            ) -> crate::resources::customers::search_ads_360::SearchAds360Actions {
                crate::resources::customers::search_ads_360::SearchAds360Actions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod custom_columns {
            pub mod params {}
            pub struct CustomColumnsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> CustomColumnsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Returns the requested custom column in full detail."]
                pub fn get(&self, resource_name: impl Into<String>) -> GetRequestBuilder {
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
                        resource_name: resource_name.into(),
                    }
                }
                #[doc = "Returns all the custom columns associated with the customer in full detail."]
                pub fn list(&self, customer_id: impl Into<String>) -> ListRequestBuilder {
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
                        customer_id: customer_id.into(),
                    }
                }
            }
            #[doc = "Created via [CustomColumnsActions::get()](struct.CustomColumnsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                resource_name: String,
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
                ) -> Result<
                    crate::schemas::GoogleAdsSearchads360V0ResourcesCustomColumn,
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
                    crate::schemas::GoogleAdsSearchads360V0ResourcesCustomColumn,
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
                    let mut output = "https://searchads360.googleapis.com/".to_owned();
                    output.push_str("v0/");
                    {
                        let var_as_str = &self.resource_name;
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
            #[doc = "Created via [CustomColumnsActions::list()](struct.CustomColumnsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                customer_id: String,
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
                    crate::schemas::GoogleAdsSearchads360V0ServicesListCustomColumnsResponse,
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
                    crate::schemas::GoogleAdsSearchads360V0ServicesListCustomColumnsResponse,
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
                    let mut output = "https://searchads360.googleapis.com/".to_owned();
                    output.push_str("v0/customers/");
                    {
                        let var_as_str = &self.customer_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/customColumns");
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
        pub mod search_ads_360 {
            pub mod params {}
            pub struct SearchAds360Actions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> SearchAds360Actions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Returns all rows that match the search query. List of thrown errors: [AuthenticationError]() [AuthorizationError]() [HeaderError]() [InternalError]() [QueryError]() [QuotaError]() [RequestError]()"]
                pub fn search(
                    &self,
                    request : crate :: schemas :: GoogleAdsSearchads360V0ServicesSearchSearchAds360Request,
                    customer_id: impl Into<String>,
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
                        customer_id: customer_id.into(),
                    }
                }
                #[doc = "Returns all rows that match the search stream query. List of thrown errors: [AuthenticationError]() [AuthorizationError]() [HeaderError]() [InternalError]() [QueryError]() [QuotaError]() [RequestError]()"]
                pub fn search_stream(
                    &self,
                    request : crate :: schemas :: GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequest,
                    customer_id: impl Into<String>,
                ) -> SearchStreamRequestBuilder {
                    SearchStreamRequestBuilder {
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
                        customer_id: customer_id.into(),
                    }
                }
            }
            #[doc = "Created via [SearchAds360Actions::search()](struct.SearchAds360Actions.html#method.search)"]
            #[derive(Debug, Clone)]
            pub struct SearchRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleAdsSearchads360V0ServicesSearchSearchAds360Request,
                customer_id: String,
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
                #[doc = r" the response resource."]
                pub async fn execute_with_default_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleAdsSearchads360V0ServicesSearchSearchAds360Response,
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
                    crate::schemas::GoogleAdsSearchads360V0ServicesSearchSearchAds360Response,
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
                    let mut output = "https://searchads360.googleapis.com/".to_owned();
                    output.push_str("v0/customers/");
                    {
                        let var_as_str = &self.customer_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/searchAds360:search");
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
            #[doc = "Created via [SearchAds360Actions::search_stream()](struct.SearchAds360Actions.html#method.search_stream)"]
            #[derive(Debug, Clone)]
            pub struct SearchStreamRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request:
                    crate::schemas::GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamRequest,
                customer_id: String,
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
            impl<'a> SearchStreamRequestBuilder<'a> {
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
                    crate::schemas::GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamResponse,
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
                    crate::schemas::GoogleAdsSearchads360V0ServicesSearchSearchAds360StreamResponse,
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
                    let mut output = "https://searchads360.googleapis.com/".to_owned();
                    output.push_str("v0/customers/");
                    {
                        let var_as_str = &self.customer_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/searchAds360:searchStream");
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
    pub mod search_ads_360_fields {
        pub mod params {}
        pub struct SearchAds360FieldsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> SearchAds360FieldsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns just the requested field. List of thrown errors: [AuthenticationError]() [AuthorizationError]() [HeaderError]() [InternalError]() [QuotaError]() [RequestError]()"]
            pub fn get(&self, resource_name: impl Into<String>) -> GetRequestBuilder {
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
                    resource_name: resource_name.into(),
                }
            }
            #[doc = "Returns all fields that match the search query. List of thrown errors: [AuthenticationError]() [AuthorizationError]() [HeaderError]() [InternalError]() [QueryError]() [QuotaError]() [RequestError]()"]
            pub fn search(
                &self,
                request : crate :: schemas :: GoogleAdsSearchads360V0ServicesSearchSearchAds360FieldsRequest,
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
                }
            }
        }
        #[doc = "Created via [SearchAds360FieldsActions::get()](struct.SearchAds360FieldsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            resource_name: String,
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
            ) -> Result<
                crate::schemas::GoogleAdsSearchads360V0ResourcesSearchAds360Field,
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
                crate::schemas::GoogleAdsSearchads360V0ResourcesSearchAds360Field,
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
                let mut output = "https://searchads360.googleapis.com/".to_owned();
                output.push_str("v0/");
                {
                    let var_as_str = &self.resource_name;
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
        #[doc = "Created via [SearchAds360FieldsActions::search()](struct.SearchAds360FieldsActions.html#method.search)"]
        #[derive(Debug, Clone)]
        pub struct SearchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleAdsSearchads360V0ServicesSearchSearchAds360FieldsRequest,
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
            #[doc = r" the response resource."]
            pub async fn execute_with_default_fields(
                self,
            ) -> Result<
                crate::schemas::GoogleAdsSearchads360V0ServicesSearchSearchAds360FieldsResponse,
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
                crate::schemas::GoogleAdsSearchads360V0ServicesSearchSearchAds360FieldsResponse,
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
                let mut output = "https://searchads360.googleapis.com/".to_owned();
                output.push_str("v0/searchAds360Fields:search");
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
