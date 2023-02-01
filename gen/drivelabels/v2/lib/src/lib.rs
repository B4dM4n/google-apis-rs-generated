#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [labels](resources/labels/struct.LabelsActions.html)\n  * [*get*](resources/labels/struct.GetRequestBuilder.html), [*list*](resources/labels/struct.ListRequestBuilder.html)\n"]
pub mod scopes {}
pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2BadgeColors {
        #[doc = "Output only. Badge background that pairs with the foreground."]
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub background_color: ::std::option::Option<crate::schemas::GoogleTypeColor>,
        #[doc = "Output only. Badge foreground that pairs with the background."]
        #[serde(
            rename = "foregroundColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub foreground_color: ::std::option::Option<crate::schemas::GoogleTypeColor>,
        #[doc = "Output only. Color that can be used for text without a background."]
        #[serde(
            rename = "soloColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub solo_color: ::std::option::Option<crate::schemas::GoogleTypeColor>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2BadgeColors {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2BadgeColors {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2BadgeConfig {
        #[doc = "The color of the badge. When not specified, no badge is rendered. The background, foreground, and solo (light and dark mode) colors set here are changed in the Drive UI into the closest recommended supported color."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::GoogleTypeColor>,
        #[doc = "Override the default global priority of this badge. When set to 0, the default priority heuristic is used."]
        #[serde(
            rename = "priorityOverride",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub priority_override: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2BadgeConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2BadgeConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2Field {
        #[doc = "Output only. The capabilities this user has on this field and its value when the label is applied on Drive items."]
        #[serde(
            rename = "appliedCapabilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub applied_capabilities:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2FieldAppliedCapabilities>,
        #[doc = "Output only. The time this field was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. The user who created this field."]
        #[serde(
            rename = "creator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creator: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2UserInfo>,
        #[doc = "Date field options."]
        #[serde(
            rename = "dateOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_options:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2FieldDateOptions>,
        #[doc = "Output only. The time this field was disabled. This value has no meaning when the field is not disabled."]
        #[serde(
            rename = "disableTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_time: ::std::option::Option<String>,
        #[doc = "Output only. The user who disabled this field. This value has no meaning when the field is not disabled."]
        #[serde(
            rename = "disabler",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabler: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2UserInfo>,
        #[doc = "Output only. UI display hints for rendering a field."]
        #[serde(
            rename = "displayHints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_hints:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2FieldDisplayHints>,
        #[doc = "Output only. The key of a field, unique within a label or library. This value is autogenerated. Matches the regex: `([a-zA-Z0-9])+`"]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Integer field options."]
        #[serde(
            rename = "integerOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub integer_options:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2FieldIntegerOptions>,
        #[doc = "Output only. The lifecycle of this field."]
        #[serde(
            rename = "lifecycle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lifecycle: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2Lifecycle>,
        #[doc = "Output only. The LockStatus of this field."]
        #[serde(
            rename = "lockStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lock_status: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2LockStatus>,
        #[doc = "The basic properties of the field."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2FieldProperties>,
        #[doc = "Output only. The user who published this field. This value has no meaning when the field is not published."]
        #[serde(
            rename = "publisher",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publisher: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2UserInfo>,
        #[doc = "Output only. The key to use when constructing Drive search queries to find files based on values defined for this field on files. For example, “`{query_key}` > 2001-01-01”."]
        #[serde(
            rename = "queryKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query_key: ::std::option::Option<String>,
        #[doc = "Output only. The capabilities this user has when editing this field."]
        #[serde(
            rename = "schemaCapabilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema_capabilities:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2FieldSchemaCapabilities>,
        #[doc = "Selection field options."]
        #[serde(
            rename = "selectionOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selection_options:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2FieldSelectionOptions>,
        #[doc = "Text field options."]
        #[serde(
            rename = "textOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_options:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2FieldTextOptions>,
        #[doc = "Output only. The time this field was updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
        #[doc = "Output only. The user who modified this field."]
        #[serde(
            rename = "updater",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updater: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2UserInfo>,
        #[doc = "User field options."]
        #[serde(
            rename = "userOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_options:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2FieldUserOptions>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2Field {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2Field {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2FieldAppliedCapabilities {
        #[doc = "Whether the user can read related applied metadata on items."]
        #[serde(
            rename = "canRead",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_read: ::std::option::Option<bool>,
        #[doc = "Whether the user can search for Drive items referencing this field."]
        #[serde(
            rename = "canSearch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_search: ::std::option::Option<bool>,
        #[doc = "Whether the user can set this field on Drive items."]
        #[serde(
            rename = "canWrite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_write: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2FieldAppliedCapabilities {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2FieldAppliedCapabilities {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2FieldDateOptions {
        #[doc = "Output only. ICU date format."]
        #[serde(
            rename = "dateFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_format: ::std::option::Option<String>,
        #[doc = "Localized date formatting option. Field values are rendered in this format according to their locale."]
        #[serde(
            rename = "dateFormatType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_format_type: ::std::option::Option<
            crate::schemas::GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType,
        >,
        #[doc = "Output only. Maximum valid value (year, month, day)."]
        #[serde(
            rename = "maxValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_value: ::std::option::Option<crate::schemas::GoogleTypeDate>,
        #[doc = "Output only. Minimum valid value (year, month, day)."]
        #[serde(
            rename = "minValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_value: ::std::option::Option<crate::schemas::GoogleTypeDate>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2FieldDateOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2FieldDateOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType {
        #[doc = "Date format unspecified."]
        DateFormatUnspecified,
        #[doc = "Includes full month name. For example, January 12, 1999 (MMMM d, y)"]
        LongDate,
        #[doc = "Short, numeric, representation. For example, 12/13/99 (M/d/yy)"]
        ShortDate,
    }
    impl GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType::DateFormatUnspecified => {
                    "DATE_FORMAT_UNSPECIFIED"
                }
                GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType::LongDate => "LONG_DATE",
                GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType::ShortDate => "SHORT_DATE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType, ()>
        {
            Ok(match s {
                "DATE_FORMAT_UNSPECIFIED" => {
                    GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType::DateFormatUnspecified
                }
                "LONG_DATE" => GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType::LongDate,
                "SHORT_DATE" => GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType::ShortDate,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DATE_FORMAT_UNSPECIFIED" => {
                    GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType::DateFormatUnspecified
                }
                "LONG_DATE" => GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType::LongDate,
                "SHORT_DATE" => GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType::ShortDate,
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
        for GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAppsDriveLabelsV2FieldDateOptionsDateFormatType
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
    pub struct GoogleAppsDriveLabelsV2FieldDisplayHints {
        #[doc = "Whether the field should be shown in the UI as disabled."]
        #[serde(
            rename = "disabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "This field should be hidden in the search menu when searching for Drive items."]
        #[serde(
            rename = "hiddenInSearch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hidden_in_search: ::std::option::Option<bool>,
        #[doc = "Whether the field should be shown as required in the UI."]
        #[serde(
            rename = "required",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub required: ::std::option::Option<bool>,
        #[doc = "This field should be shown in the apply menu when applying values to a Drive item."]
        #[serde(
            rename = "shownInApply",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shown_in_apply: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2FieldDisplayHints {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2FieldDisplayHints {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2FieldIntegerOptions {
        #[doc = "Output only. The maximum valid value for the integer field."]
        #[serde(
            rename = "maxValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub max_value: ::std::option::Option<i64>,
        #[doc = "Output only. The minimum valid value for the integer field."]
        #[serde(
            rename = "minValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub min_value: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2FieldIntegerOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2FieldIntegerOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2FieldListOptions {
        #[doc = "Maximum number of entries permitted."]
        #[serde(
            rename = "maxEntries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_entries: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2FieldListOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2FieldListOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2FieldProperties {
        #[doc = "Required. The display text to show in the UI identifying this field."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Input only. Insert or move this field before the indicated field. If empty, the field is placed at the end of the list."]
        #[serde(
            rename = "insertBeforeField",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_before_field: ::std::option::Option<String>,
        #[doc = "Whether the field should be marked as required."]
        #[serde(
            rename = "required",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub required: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2FieldProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2FieldProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2FieldSchemaCapabilities {
        #[doc = "Whether the user can delete this field. The user must have permission and the field must be deprecated."]
        #[serde(
            rename = "canDelete",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_delete: ::std::option::Option<bool>,
        #[doc = "Whether the user can disable this field. The user must have permission and this field must not already be disabled."]
        #[serde(
            rename = "canDisable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_disable: ::std::option::Option<bool>,
        #[doc = "Whether the user can enable this field. The user must have permission and this field must be disabled."]
        #[serde(
            rename = "canEnable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_enable: ::std::option::Option<bool>,
        #[doc = "Whether the user can change this field."]
        #[serde(
            rename = "canUpdate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_update: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2FieldSchemaCapabilities {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2FieldSchemaCapabilities {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2FieldSelectionOptions {
        #[doc = "The options available for this selection field. The list order is consistent, and modified with `insert_before_choice`."]
        #[serde(
            rename = "choices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub choices: ::std::option::Option<
            Vec<crate::schemas::GoogleAppsDriveLabelsV2FieldSelectionOptionsChoice>,
        >,
        #[doc = "When specified, indicates this field supports a list of values. Once the field is published, this cannot be changed."]
        #[serde(
            rename = "listOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list_options:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2FieldListOptions>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2FieldSelectionOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2FieldSelectionOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2FieldSelectionOptionsChoice {
        #[doc = "Output only. The capabilities related to this choice on applied metadata."]
        #[serde(
            rename = "appliedCapabilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub applied_capabilities: ::std::option::Option<
            crate::schemas::GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceAppliedCapabilities,
        >,
        #[doc = "Output only. The time this choice was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. The user who created this choice."]
        #[serde(
            rename = "creator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creator: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2UserInfo>,
        #[doc = "Output only. The time this choice was disabled. This value has no meaning when the choice is not disabled."]
        #[serde(
            rename = "disableTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_time: ::std::option::Option<String>,
        #[doc = "Output only. The user who disabled this choice. This value has no meaning when the option is not disabled."]
        #[serde(
            rename = "disabler",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabler: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2UserInfo>,
        #[doc = "Output only. UI display hints for rendering a choice."]
        #[serde(
            rename = "displayHints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_hints: ::std::option::Option<
            crate::schemas::GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceDisplayHints,
        >,
        #[doc = "The unique value of the choice. This ID is autogenerated. Matches the regex: `([a-zA-Z0-9_])+`."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Output only. Lifecycle of the choice."]
        #[serde(
            rename = "lifecycle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lifecycle: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2Lifecycle>,
        #[doc = "Output only. The LockStatus of this choice."]
        #[serde(
            rename = "lockStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lock_status: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2LockStatus>,
        #[doc = "Basic properties of the choice."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties: ::std::option::Option<
            crate::schemas::GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceProperties,
        >,
        #[doc = "Output only. The time this choice was published. This value has no meaning when the choice is not published."]
        #[serde(
            rename = "publishTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publish_time: ::std::option::Option<String>,
        #[doc = "Output only. The user who published this choice. This value has no meaning when the choice is not published."]
        #[serde(
            rename = "publisher",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publisher: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2UserInfo>,
        #[doc = "Output only. The capabilities related to this option when editing the option."]
        #[serde(
            rename = "schemaCapabilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema_capabilities: ::std::option::Option<
            crate::schemas::GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceSchemaCapabilities,
        >,
        #[doc = "Output only. The time this choice was updated last."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
        #[doc = "Output only. The user who updated this choice last."]
        #[serde(
            rename = "updater",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub updater: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2UserInfo>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2FieldSelectionOptionsChoice {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2FieldSelectionOptionsChoice {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceAppliedCapabilities {
        #[doc = "Whether the user can read related applied metadata on items."]
        #[serde(
            rename = "canRead",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_read: ::std::option::Option<bool>,
        #[doc = "Whether the user can use this choice in search queries."]
        #[serde(
            rename = "canSearch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_search: ::std::option::Option<bool>,
        #[doc = "Whether the user can select this choice on an item."]
        #[serde(
            rename = "canSelect",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_select: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceAppliedCapabilities
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceAppliedCapabilities
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceDisplayHints {
        #[doc = "The colors to use for the badge. Changed to Google Material colors based on the chosen `properties.badge_config.color`."]
        #[serde(
            rename = "badgeColors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub badge_colors: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2BadgeColors>,
        #[doc = "The priority of this badge. Used to compare and sort between multiple badges. A lower number means the badge should be shown first. When a badging configuration is not present, this will be 0. Otherwise, this will be set to `BadgeConfig.priority_override` or the default heuristic which prefers creation date of the label, and field and option priority."]
        #[serde(
            rename = "badgePriority",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub badge_priority: ::std::option::Option<i64>,
        #[doc = "The dark-mode color to use for the badge. Changed to Google Material colors based on the chosen `properties.badge_config.color`."]
        #[serde(
            rename = "darkBadgeColors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dark_badge_colors:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2BadgeColors>,
        #[doc = "Whether the option should be shown in the UI as disabled."]
        #[serde(
            rename = "disabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "This option should be hidden in the search menu when searching for Drive items."]
        #[serde(
            rename = "hiddenInSearch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hidden_in_search: ::std::option::Option<bool>,
        #[doc = "This option should be shown in the apply menu when applying values to a Drive item."]
        #[serde(
            rename = "shownInApply",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shown_in_apply: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceDisplayHints
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceDisplayHints
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceProperties {
        #[doc = "The badge configuration for this choice. When set, the label that owns this choice is considered a “badged label”."]
        #[serde(
            rename = "badgeConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub badge_config: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2BadgeConfig>,
        #[doc = "The description of this label."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Required. The display text to show in the UI identifying this field."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Input only. Insert or move this choice before the indicated choice. If empty, the choice is placed at the end of the list."]
        #[serde(
            rename = "insertBeforeChoice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insert_before_choice: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceProperties
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceProperties
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
    pub struct GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceSchemaCapabilities {
        #[doc = "Whether the user can delete this choice."]
        #[serde(
            rename = "canDelete",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_delete: ::std::option::Option<bool>,
        #[doc = "Whether the user can disable this choice."]
        #[serde(
            rename = "canDisable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_disable: ::std::option::Option<bool>,
        #[doc = "Whether the user can enable this choice."]
        #[serde(
            rename = "canEnable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_enable: ::std::option::Option<bool>,
        #[doc = "Whether the user can update this choice."]
        #[serde(
            rename = "canUpdate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_update: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceSchemaCapabilities
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAppsDriveLabelsV2FieldSelectionOptionsChoiceSchemaCapabilities
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
    pub struct GoogleAppsDriveLabelsV2FieldTextOptions {
        #[doc = "Output only. The maximum valid length of values for the text field."]
        #[serde(
            rename = "maxLength",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_length: ::std::option::Option<i32>,
        #[doc = "Output only. The minimum valid length of values for the text field."]
        #[serde(
            rename = "minLength",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_length: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2FieldTextOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2FieldTextOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2FieldUserOptions {
        #[doc = "When specified, indicates that this field supports a list of values. Once the field is published, this cannot be changed."]
        #[serde(
            rename = "listOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list_options:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2FieldListOptions>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2FieldUserOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2FieldUserOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2Label {
        #[doc = "Output only. The capabilities related to this label on applied metadata."]
        #[serde(
            rename = "appliedCapabilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub applied_capabilities:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2LabelAppliedCapabilities>,
        #[doc = "Output only. Behavior of this label when it’s applied to Drive items."]
        #[serde(
            rename = "appliedLabelPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub applied_label_policy:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2LabelAppliedLabelPolicy>,
        #[doc = "Output only. The time this label was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. The user who created this label."]
        #[serde(
            rename = "creator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creator: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2UserInfo>,
        #[doc = "Output only. The time this label was disabled. This value has no meaning when the label is not disabled."]
        #[serde(
            rename = "disableTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_time: ::std::option::Option<String>,
        #[doc = "Output only. The user who disabled this label. This value has no meaning when the label is not disabled."]
        #[serde(
            rename = "disabler",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabler: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2UserInfo>,
        #[doc = "Output only. UI display hints for rendering the label."]
        #[serde(
            rename = "displayHints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_hints:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2LabelDisplayHints>,
        #[doc = "List of fields in descending priority order."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<Vec<crate::schemas::GoogleAppsDriveLabelsV2Field>>,
        #[doc = "Output only. Globally unique identifier of this label. ID makes up part of the label `name`, but unlike `name`, ID is consistent between revisions. Matches the regex: `([a-zA-Z0-9])+`"]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Required. The type of label."]
        #[serde(
            rename = "labelType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label_type:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2LabelLabelType>,
        #[doc = "Custom URL to present to users to allow them to learn more about this label and how it should be used."]
        #[serde(
            rename = "learnMoreUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub learn_more_uri: ::std::option::Option<String>,
        #[doc = "Output only. The lifecycle state of the label including whether it’s published, deprecated, and has draft changes."]
        #[serde(
            rename = "lifecycle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lifecycle: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2Lifecycle>,
        #[doc = "Output only. The LockStatus of this label."]
        #[serde(
            rename = "lockStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lock_status: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2LockStatus>,
        #[doc = "Output only. Resource name of the label. Will be in the form of either: `labels/{id}` or `labels/{id}@{revision_id}` depending on the request. See `id` and `revision_id` below."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Required. The basic properties of the label."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2LabelProperties>,
        #[doc = "Output only. The time this label was published. This value has no meaning when the label is not published."]
        #[serde(
            rename = "publishTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publish_time: ::std::option::Option<String>,
        #[doc = "Output only. The user who published this label. This value has no meaning when the label is not published."]
        #[serde(
            rename = "publisher",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publisher: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2UserInfo>,
        #[doc = "Output only. The time this label revision was created."]
        #[serde(
            rename = "revisionCreateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_create_time: ::std::option::Option<String>,
        #[doc = "Output only. The user who created this label revision."]
        #[serde(
            rename = "revisionCreator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_creator:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2UserInfo>,
        #[doc = "Output only. Revision ID of the label. Revision ID might be part of the label `name` depending on the request issued. A new revision is created whenever revisioned properties of a label are changed. Matches the regex: `([a-zA-Z0-9])+`"]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
        #[doc = "Output only. The capabilities the user has on this label."]
        #[serde(
            rename = "schemaCapabilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema_capabilities:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2LabelSchemaCapabilities>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2Label {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2Label {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsDriveLabelsV2LabelLabelType {
        #[doc = "Admin-owned label. Only creatable and editable by admins. Supports some additional admin-only features."]
        Admin,
        #[doc = "Unknown label type."]
        LabelTypeUnspecified,
        #[doc = "Shared labels may be shared with users to apply to Drive items."]
        Shared,
    }
    impl GoogleAppsDriveLabelsV2LabelLabelType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsDriveLabelsV2LabelLabelType::Admin => "ADMIN",
                GoogleAppsDriveLabelsV2LabelLabelType::LabelTypeUnspecified => {
                    "LABEL_TYPE_UNSPECIFIED"
                }
                GoogleAppsDriveLabelsV2LabelLabelType::Shared => "SHARED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsDriveLabelsV2LabelLabelType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsDriveLabelsV2LabelLabelType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsDriveLabelsV2LabelLabelType, ()> {
            Ok(match s {
                "ADMIN" => GoogleAppsDriveLabelsV2LabelLabelType::Admin,
                "LABEL_TYPE_UNSPECIFIED" => {
                    GoogleAppsDriveLabelsV2LabelLabelType::LabelTypeUnspecified
                }
                "SHARED" => GoogleAppsDriveLabelsV2LabelLabelType::Shared,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsDriveLabelsV2LabelLabelType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsDriveLabelsV2LabelLabelType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsDriveLabelsV2LabelLabelType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMIN" => GoogleAppsDriveLabelsV2LabelLabelType::Admin,
                "LABEL_TYPE_UNSPECIFIED" => {
                    GoogleAppsDriveLabelsV2LabelLabelType::LabelTypeUnspecified
                }
                "SHARED" => GoogleAppsDriveLabelsV2LabelLabelType::Shared,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2LabelLabelType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2LabelLabelType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2LabelAppliedCapabilities {
        #[doc = "Whether the user can apply this label to items."]
        #[serde(
            rename = "canApply",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_apply: ::std::option::Option<bool>,
        #[doc = "Whether the user can read applied metadata related to this label."]
        #[serde(
            rename = "canRead",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_read: ::std::option::Option<bool>,
        #[doc = "Whether the user can remove this label from items."]
        #[serde(
            rename = "canRemove",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_remove: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2LabelAppliedCapabilities {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2LabelAppliedCapabilities {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2LabelAppliedLabelPolicy {
        #[doc = "Indicates how the applied label and field values should be copied when a Drive item is copied."]
        #[serde(
            rename = "copyMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub copy_mode: ::std::option::Option<
            crate::schemas::GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2LabelAppliedLabelPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2LabelAppliedLabelPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode {
        #[doc = "The applied label and field values are always copied when the Drive item it’s applied to is copied. Only admins can use this mode."]
        AlwaysCopy,
        #[doc = "The applied label and field values are copied if the label is appliable by the user making the copy."]
        CopyAppliable,
        #[doc = "Copy mode unspecified."]
        CopyModeUnspecified,
        #[doc = "The applied label and field values are not copied by default when the Drive item it’s applied to is copied."]
        DoNotCopy,
    }
    impl GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode::AlwaysCopy => "ALWAYS_COPY",
                GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode::CopyAppliable => {
                    "COPY_APPLIABLE"
                }
                GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode::CopyModeUnspecified => {
                    "COPY_MODE_UNSPECIFIED"
                }
                GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode::DoNotCopy => "DO_NOT_COPY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode, ()>
        {
            Ok(match s {
                "ALWAYS_COPY" => GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode::AlwaysCopy,
                "COPY_APPLIABLE" => {
                    GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode::CopyAppliable
                }
                "COPY_MODE_UNSPECIFIED" => {
                    GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode::CopyModeUnspecified
                }
                "DO_NOT_COPY" => GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode::DoNotCopy,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALWAYS_COPY" => GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode::AlwaysCopy,
                "COPY_APPLIABLE" => {
                    GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode::CopyAppliable
                }
                "COPY_MODE_UNSPECIFIED" => {
                    GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode::CopyModeUnspecified
                }
                "DO_NOT_COPY" => GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode::DoNotCopy,
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
        for GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAppsDriveLabelsV2LabelAppliedLabelPolicyCopyMode
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
    pub struct GoogleAppsDriveLabelsV2LabelDisplayHints {
        #[doc = "Whether the label should be shown in the UI as disabled."]
        #[serde(
            rename = "disabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "This label should be hidden in the search menu when searching for Drive items."]
        #[serde(
            rename = "hiddenInSearch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hidden_in_search: ::std::option::Option<bool>,
        #[doc = "Order to display label in a list."]
        #[serde(
            rename = "priority",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub priority: ::std::option::Option<i64>,
        #[doc = "This label should be shown in the apply menu when applying values to a Drive item."]
        #[serde(
            rename = "shownInApply",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shown_in_apply: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2LabelDisplayHints {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2LabelDisplayHints {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2LabelProperties {
        #[doc = "The description of the label."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Required. Title of the label."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2LabelProperties {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2LabelProperties {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2LabelSchemaCapabilities {
        #[doc = "Whether the user can delete this label. The user must have permission and the label must be disabled."]
        #[serde(
            rename = "canDelete",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_delete: ::std::option::Option<bool>,
        #[doc = "Whether the user can disable this label. The user must have permission and this label must not already be disabled."]
        #[serde(
            rename = "canDisable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_disable: ::std::option::Option<bool>,
        #[doc = "Whether the user can enable this label. The user must have permission and this label must be disabled."]
        #[serde(
            rename = "canEnable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_enable: ::std::option::Option<bool>,
        #[doc = "Whether the user can change this label."]
        #[serde(
            rename = "canUpdate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub can_update: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2LabelSchemaCapabilities {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2LabelSchemaCapabilities {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2Lifecycle {
        #[doc = "The policy that governs how to show a disabled label, field, or selection choice."]
        #[serde(
            rename = "disabledPolicy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabled_policy:
            ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2LifecycleDisabledPolicy>,
        #[doc = "Output only. Whether the object associated with this lifecycle has unpublished changes."]
        #[serde(
            rename = "hasUnpublishedChanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_unpublished_changes: ::std::option::Option<bool>,
        #[doc = "Output only. The state of the object associated with this lifecycle."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::GoogleAppsDriveLabelsV2LifecycleState>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2Lifecycle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2Lifecycle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsDriveLabelsV2LifecycleState {
        #[doc = "The object has been deleted."]
        Deleted,
        #[doc = "The object has been published and has since been disabled. The object might have unpublished draft changes as indicated by `has_unpublished_changes`."]
        Disabled,
        #[doc = "The object has been published. The object might have unpublished draft changes as indicated by `has_unpublished_changes`."]
        Published,
        #[doc = "Unknown State."]
        StateUnspecified,
        #[doc = "The initial state of an object. Once published, the object can never return to this state. Once an object is published, certain kinds of changes are no longer permitted."]
        UnpublishedDraft,
    }
    impl GoogleAppsDriveLabelsV2LifecycleState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsDriveLabelsV2LifecycleState::Deleted => "DELETED",
                GoogleAppsDriveLabelsV2LifecycleState::Disabled => "DISABLED",
                GoogleAppsDriveLabelsV2LifecycleState::Published => "PUBLISHED",
                GoogleAppsDriveLabelsV2LifecycleState::StateUnspecified => "STATE_UNSPECIFIED",
                GoogleAppsDriveLabelsV2LifecycleState::UnpublishedDraft => "UNPUBLISHED_DRAFT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsDriveLabelsV2LifecycleState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsDriveLabelsV2LifecycleState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsDriveLabelsV2LifecycleState, ()> {
            Ok(match s {
                "DELETED" => GoogleAppsDriveLabelsV2LifecycleState::Deleted,
                "DISABLED" => GoogleAppsDriveLabelsV2LifecycleState::Disabled,
                "PUBLISHED" => GoogleAppsDriveLabelsV2LifecycleState::Published,
                "STATE_UNSPECIFIED" => GoogleAppsDriveLabelsV2LifecycleState::StateUnspecified,
                "UNPUBLISHED_DRAFT" => GoogleAppsDriveLabelsV2LifecycleState::UnpublishedDraft,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsDriveLabelsV2LifecycleState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsDriveLabelsV2LifecycleState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsDriveLabelsV2LifecycleState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DELETED" => GoogleAppsDriveLabelsV2LifecycleState::Deleted,
                "DISABLED" => GoogleAppsDriveLabelsV2LifecycleState::Disabled,
                "PUBLISHED" => GoogleAppsDriveLabelsV2LifecycleState::Published,
                "STATE_UNSPECIFIED" => GoogleAppsDriveLabelsV2LifecycleState::StateUnspecified,
                "UNPUBLISHED_DRAFT" => GoogleAppsDriveLabelsV2LifecycleState::UnpublishedDraft,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2LifecycleState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2LifecycleState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2LifecycleDisabledPolicy {
        #[doc = "Whether to hide this disabled object in the search menu for Drive items. * When `false`, the object is generally shown in the UI as disabled but it appears in the search results when searching for Drive items. * When `true`, the object is generally hidden in the UI when searching for Drive items."]
        #[serde(
            rename = "hideInSearch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hide_in_search: ::std::option::Option<bool>,
        #[doc = "Whether to show this disabled object in the apply menu on Drive items. * When `true`, the object is generally shown in the UI as disabled and is unselectable. * When `false`, the object is generally hidden in the UI."]
        #[serde(
            rename = "showInApply",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub show_in_apply: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2LifecycleDisabledPolicy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2LifecycleDisabledPolicy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2ListLabelsResponse {
        #[doc = "Labels."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<crate::schemas::GoogleAppsDriveLabelsV2Label>>,
        #[doc = "The token of the next page in the response."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2ListLabelsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2ListLabelsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for GoogleAppsDriveLabelsV2ListLabelsResponse {
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
    pub struct GoogleAppsDriveLabelsV2LockStatus {
        #[doc = "Output only. Indicates whether this label component is the (direct) target of a LabelLock. A label component can be implicitly locked even if it’s not the direct target of a LabelLock, in which case this field is set to false."]
        #[serde(
            rename = "locked",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locked: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2LockStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2LockStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsDriveLabelsV2UserInfo {
        #[doc = "The identifier for this user that can be used with the People API to get more information. For example, people/12345678."]
        #[serde(
            rename = "person",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsDriveLabelsV2UserInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsDriveLabelsV2UserInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleTypeColor {
        #[doc = "The fraction of this color that should be applied to the pixel. That is, the final pixel color is defined by the equation: `pixel color = alpha * (this color) + (1.0 - alpha) * (background color)` This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color. This uses a wrapper message rather than a simple float scalar so that it is possible to distinguish between a default value and the value being unset. If omitted, this color object is rendered as a solid color (as if the alpha value had been explicitly given a value of 1.0)."]
        #[serde(
            rename = "alpha",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alpha: ::std::option::Option<f32>,
        #[doc = "The amount of blue in the color as a value in the interval \\[0, 1\\]."]
        #[serde(
            rename = "blue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blue: ::std::option::Option<f32>,
        #[doc = "The amount of green in the color as a value in the interval \\[0, 1\\]."]
        #[serde(
            rename = "green",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub green: ::std::option::Option<f32>,
        #[doc = "The amount of red in the color as a value in the interval \\[0, 1\\]."]
        #[serde(
            rename = "red",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub red: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleTypeColor {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeColor {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleTypeDate {
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn’t significant."]
        #[serde(
            rename = "day",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day: ::std::option::Option<i32>,
        #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
        #[serde(
            rename = "month",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub month: ::std::option::Option<i32>,
        #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
        #[serde(
            rename = "year",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub year: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleTypeDate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeDate {
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
    #[doc = "Actions that can be performed on the labels resource"]
    pub fn labels(&self) -> crate::resources::labels::LabelsActions {
        crate::resources::labels::LabelsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod labels {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetView {
                #[doc = "Implies the field mask: `name,id,revision_id,label_type,properties.*`"]
                LabelViewBasic,
                #[doc = "All possible fields."]
                LabelViewFull,
            }
            impl GetView {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetView::LabelViewBasic => "LABEL_VIEW_BASIC",
                        GetView::LabelViewFull => "LABEL_VIEW_FULL",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for GetView {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetView {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<GetView, ()> {
                    Ok(match s {
                        "LABEL_VIEW_BASIC" => GetView::LabelViewBasic,
                        "LABEL_VIEW_FULL" => GetView::LabelViewFull,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for GetView {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetView {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetView {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "LABEL_VIEW_BASIC" => GetView::LabelViewBasic,
                        "LABEL_VIEW_FULL" => GetView::LabelViewFull,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetView {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetView {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListMinimumRole {
                #[doc = "An applier can write associated metadata on Drive items in which they also have write access to. Implies `READER`."]
                Applier,
                #[doc = "Editors can make any update including deleting the label which also deletes the associated Drive item metadata. Implies `APPLIER`."]
                Editor,
                #[doc = "Unknown role."]
                LabelRoleUnspecified,
                #[doc = "An organizer can pin this label in shared drives they manage and add new appliers to the label."]
                Organizer,
                #[doc = "A reader can read the label and associated metadata applied to Drive items."]
                Reader,
            }
            impl ListMinimumRole {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListMinimumRole::Applier => "APPLIER",
                        ListMinimumRole::Editor => "EDITOR",
                        ListMinimumRole::LabelRoleUnspecified => "LABEL_ROLE_UNSPECIFIED",
                        ListMinimumRole::Organizer => "ORGANIZER",
                        ListMinimumRole::Reader => "READER",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListMinimumRole {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListMinimumRole {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListMinimumRole, ()> {
                    Ok(match s {
                        "APPLIER" => ListMinimumRole::Applier,
                        "EDITOR" => ListMinimumRole::Editor,
                        "LABEL_ROLE_UNSPECIFIED" => ListMinimumRole::LabelRoleUnspecified,
                        "ORGANIZER" => ListMinimumRole::Organizer,
                        "READER" => ListMinimumRole::Reader,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListMinimumRole {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListMinimumRole {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListMinimumRole {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "APPLIER" => ListMinimumRole::Applier,
                        "EDITOR" => ListMinimumRole::Editor,
                        "LABEL_ROLE_UNSPECIFIED" => ListMinimumRole::LabelRoleUnspecified,
                        "ORGANIZER" => ListMinimumRole::Organizer,
                        "READER" => ListMinimumRole::Reader,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListMinimumRole {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListMinimumRole {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListView {
                #[doc = "Implies the field mask: `name,id,revision_id,label_type,properties.*`"]
                LabelViewBasic,
                #[doc = "All possible fields."]
                LabelViewFull,
            }
            impl ListView {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListView::LabelViewBasic => "LABEL_VIEW_BASIC",
                        ListView::LabelViewFull => "LABEL_VIEW_FULL",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListView {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListView {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListView, ()> {
                    Ok(match s {
                        "LABEL_VIEW_BASIC" => ListView::LabelViewBasic,
                        "LABEL_VIEW_FULL" => ListView::LabelViewFull,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListView {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListView {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListView {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "LABEL_VIEW_BASIC" => ListView::LabelViewBasic,
                        "LABEL_VIEW_FULL" => ListView::LabelViewFull,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListView {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListView {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct LabelsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> LabelsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Get a label by its resource name. Resource name may be any of: * `labels/{id}` - See `labels/{id}@latest` * `labels/{id}@latest` - Gets the latest revision of the label. * `labels/{id}@published` - Gets the current published revision of the label. * `labels/{id}@{revision_id}` - Gets the label at the specified revision ID."]
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
                    language_code: None,
                    use_admin_access: None,
                    view: None,
                }
            }
            #[doc = "List labels."]
            pub fn list(&self) -> ListRequestBuilder {
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
                    language_code: None,
                    minimum_role: None,
                    page_size: None,
                    page_token: None,
                    published_only: None,
                    use_admin_access: None,
                    view: None,
                }
            }
        }
        #[doc = "Created via [LabelsActions::get()](struct.LabelsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            name: String,
            language_code: ::std::option::Option<String>,
            use_admin_access: ::std::option::Option<bool>,
            view: ::std::option::Option<crate::resources::labels::params::GetView>,
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
            #[doc = "The BCP-47 language code to use for evaluating localized field labels. When not specified, values in the default configured language are used."]
            pub fn language_code(mut self, value: impl Into<String>) -> Self {
                self.language_code = Some(value.into());
                self
            }
            #[doc = "Set to `true` in order to use the user’s admin credentials. The server verifies that the user is an admin for the label before allowing access."]
            pub fn use_admin_access(mut self, value: bool) -> Self {
                self.use_admin_access = Some(value);
                self
            }
            #[doc = "When specified, only certain fields belonging to the indicated view are returned."]
            pub fn view(mut self, value: crate::resources::labels::params::GetView) -> Self {
                self.view = Some(value);
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
            ) -> Result<crate::schemas::GoogleAppsDriveLabelsV2Label, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleAppsDriveLabelsV2Label, crate::Error> {
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
                let mut output = "https://drivelabels.googleapis.com/".to_owned();
                output.push_str("v2/");
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
                req = req.query(&[("languageCode", &self.language_code)]);
                req = req.query(&[("useAdminAccess", &self.use_admin_access)]);
                req = req.query(&[("view", &self.view)]);
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
        #[doc = "Created via [LabelsActions::list()](struct.LabelsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            language_code: ::std::option::Option<String>,
            minimum_role: ::std::option::Option<crate::resources::labels::params::ListMinimumRole>,
            page_size: ::std::option::Option<i32>,
            page_token: ::std::option::Option<String>,
            published_only: ::std::option::Option<bool>,
            use_admin_access: ::std::option::Option<bool>,
            view: ::std::option::Option<crate::resources::labels::params::ListView>,
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
            #[doc = "The BCP-47 language code to use for evaluating localized field labels. When not specified, values in the default configured language are used."]
            pub fn language_code(mut self, value: impl Into<String>) -> Self {
                self.language_code = Some(value.into());
                self
            }
            #[doc = "Specifies the level of access the user must have on the returned Labels. The minimum role a user must have on a label. Defaults to `READER`."]
            pub fn minimum_role(
                mut self,
                value: crate::resources::labels::params::ListMinimumRole,
            ) -> Self {
                self.minimum_role = Some(value);
                self
            }
            #[doc = "Maximum number of labels to return per page. Default: 50. Max: 200."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The token of the page to return."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "Whether to include only published labels in the results. * When `true`, only the current published label revisions are returned. Disabled labels are included. Returned label resource names reference the published revision (`labels/{id}/{revision_id}`). * When `false`, the current label revisions are returned, which might not be published. Returned label resource names don’t reference a specific revision (`labels/{id}`)."]
            pub fn published_only(mut self, value: bool) -> Self {
                self.published_only = Some(value);
                self
            }
            #[doc = "Set to `true` in order to use the user’s admin credentials. This will return all Labels within the customer."]
            pub fn use_admin_access(mut self, value: bool) -> Self {
                self.use_admin_access = Some(value);
                self
            }
            #[doc = "When specified, only certain fields belonging to the indicated view are returned."]
            pub fn view(mut self, value: crate::resources::labels::params::ListView) -> Self {
                self.view = Some(value);
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
            #[doc = "\nExecute the request and yield each item in the `labels` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
            pub fn stream_labels<T>(
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
                self.stream_labels_with_fields(fields)
            }
            #[doc = "\nExecute the request and yield each item in the `labels` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
            pub fn stream_labels_with_default_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::GoogleAppsDriveLabelsV2Label, crate::Error>,
            > + 'a {
                self.stream_labels_with_fields(None::<String>)
            }
            #[doc = "\nExecute the request and yield each item in the `labels` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
            pub fn stream_labels_with_all_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::GoogleAppsDriveLabelsV2Label, crate::Error>,
            > + 'a {
                self.stream_labels_with_fields(Some("*"))
            }
            #[doc = "\nExecute the request and yield each item in the `labels` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
            pub fn stream_labels_with_fields<T, F>(
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
                    #[serde(rename = "labels")]
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
                    let mut selector = concat!("nextPageToken,", "labels").to_owned();
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
            pub fn stream<T>(self) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
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
                    crate::schemas::GoogleAppsDriveLabelsV2ListLabelsResponse,
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
                    crate::schemas::GoogleAppsDriveLabelsV2ListLabelsResponse,
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
            ) -> Result<crate::schemas::GoogleAppsDriveLabelsV2ListLabelsResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleAppsDriveLabelsV2ListLabelsResponse, crate::Error>
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
                let mut output = "https://drivelabels.googleapis.com/".to_owned();
                output.push_str("v2/labels");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("languageCode", &self.language_code)]);
                req = req.query(&[("minimumRole", &self.minimum_role)]);
                req = req.query(&[("pageSize", &self.page_size)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("publishedOnly", &self.published_only)]);
                req = req.query(&[("useAdminAccess", &self.use_admin_access)]);
                req = req.query(&[("view", &self.view)]);
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
