#![doc = "# Resources and Methods\n* [dms](resources/dms/struct.DmsActions.html)\n  * [*messages*](resources/dms/struct.MessagesRequestBuilder.html), [*webhooks*](resources/dms/struct.WebhooksRequestBuilder.html)\n  * [conversations](resources/dms/conversations/struct.ConversationsActions.html)\n    * [*messages*](resources/dms/conversations/struct.MessagesRequestBuilder.html)\n* [media](resources/media/struct.MediaActions.html)\n  * [*download*](resources/media/struct.DownloadRequestBuilder.html)\n* [rooms](resources/rooms/struct.RoomsActions.html)\n  * [*messages*](resources/rooms/struct.MessagesRequestBuilder.html), [*webhooks*](resources/rooms/struct.WebhooksRequestBuilder.html)\n  * [conversations](resources/rooms/conversations/struct.ConversationsActions.html)\n    * [*messages*](resources/rooms/conversations/struct.MessagesRequestBuilder.html)\n* [spaces](resources/spaces/struct.SpacesActions.html)\n  * [*get*](resources/spaces/struct.GetRequestBuilder.html), [*list*](resources/spaces/struct.ListRequestBuilder.html), [*webhooks*](resources/spaces/struct.WebhooksRequestBuilder.html)\n  * [members](resources/spaces/members/struct.MembersActions.html)\n    * [*get*](resources/spaces/members/struct.GetRequestBuilder.html), [*list*](resources/spaces/members/struct.ListRequestBuilder.html)\n  * [messages](resources/spaces/messages/struct.MessagesActions.html)\n    * [*create*](resources/spaces/messages/struct.CreateRequestBuilder.html), [*delete*](resources/spaces/messages/struct.DeleteRequestBuilder.html), [*get*](resources/spaces/messages/struct.GetRequestBuilder.html), [*update*](resources/spaces/messages/struct.UpdateRequestBuilder.html)\n    * [attachments](resources/spaces/messages/attachments/struct.AttachmentsActions.html)\n      * [*get*](resources/spaces/messages/attachments/struct.GetRequestBuilder.html)\n"]
pub mod scopes {}
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
    pub struct ActionParameter {
        #[doc = "The name of the parameter for the action script."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "The value of the parameter."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ActionParameter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActionParameter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ActionResponse {
        #[doc = "Input only. A response to an event related to a [dialog](https://developers.google.com/chat/how-tos/dialogs). Must be accompanied by `ResponseType.Dialog`."]
        #[serde(
            rename = "dialogAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dialog_action: ::std::option::Option<crate::schemas::DialogAction>,
        #[doc = "Input only. The type of Chat app response."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::ActionResponseType>,
        #[doc = "Input only. URL for users to auth or config. (Only for REQUEST_CONFIG response types.)"]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ActionResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActionResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ActionResponseType {
        #[doc = "Presents a [dialog](https://developers.google.com/chat/how-tos/dialogs)."]
        Dialog,
        #[doc = "Post as a new message in the topic."]
        NewMessage,
        #[doc = "Privately ask the user for additional auth or config."]
        RequestConfig,
        #[doc = "Default type; will be handled as NEW_MESSAGE."]
        TypeUnspecified,
        #[doc = "Update the Chat app's message. This is only permitted on a CARD_CLICKED event where the message sender type is BOT."]
        UpdateMessage,
        #[doc = "Update the cards on a user's message. This is only permitted as a response to a MESSAGE event with a matched url, or a CARD_CLICKED event where the message sender type is HUMAN. Text will be ignored."]
        UpdateUserMessageCards,
    }
    impl ActionResponseType {
        pub fn as_str(self) -> &'static str {
            match self {
                ActionResponseType::Dialog => "DIALOG",
                ActionResponseType::NewMessage => "NEW_MESSAGE",
                ActionResponseType::RequestConfig => "REQUEST_CONFIG",
                ActionResponseType::TypeUnspecified => "TYPE_UNSPECIFIED",
                ActionResponseType::UpdateMessage => "UPDATE_MESSAGE",
                ActionResponseType::UpdateUserMessageCards => "UPDATE_USER_MESSAGE_CARDS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ActionResponseType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ActionResponseType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ActionResponseType, ()> {
            Ok(match s {
                "DIALOG" => ActionResponseType::Dialog,
                "NEW_MESSAGE" => ActionResponseType::NewMessage,
                "REQUEST_CONFIG" => ActionResponseType::RequestConfig,
                "TYPE_UNSPECIFIED" => ActionResponseType::TypeUnspecified,
                "UPDATE_MESSAGE" => ActionResponseType::UpdateMessage,
                "UPDATE_USER_MESSAGE_CARDS" => ActionResponseType::UpdateUserMessageCards,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ActionResponseType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ActionResponseType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ActionResponseType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DIALOG" => ActionResponseType::Dialog,
                "NEW_MESSAGE" => ActionResponseType::NewMessage,
                "REQUEST_CONFIG" => ActionResponseType::RequestConfig,
                "TYPE_UNSPECIFIED" => ActionResponseType::TypeUnspecified,
                "UPDATE_MESSAGE" => ActionResponseType::UpdateMessage,
                "UPDATE_USER_MESSAGE_CARDS" => ActionResponseType::UpdateUserMessageCards,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ActionResponseType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActionResponseType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ActionStatus {
        #[doc = "The status code."]
        #[serde(
            rename = "statusCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status_code: ::std::option::Option<crate::schemas::ActionStatusStatusCode>,
        #[doc = "The message to send users about the status of their request. If unset, a generic message based on the `status_code` is sent."]
        #[serde(
            rename = "userFacingMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_facing_message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ActionStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActionStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ActionStatusStatusCode {
        #[doc = "The operation was aborted, typically due to a concurrency issue such as a sequencer check failure or transaction abort. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 409 Conflict"]
        Aborted,
        #[doc = "The entity that a client attempted to create (e.g., file or directory) already exists. HTTP Mapping: 409 Conflict"]
        AlreadyExists,
        #[doc = "The operation was cancelled, typically by the caller. HTTP Mapping: 499 Client Closed Request"]
        Cancelled,
        #[doc = "Unrecoverable data loss or corruption. HTTP Mapping: 500 Internal Server Error"]
        DataLoss,
        #[doc = "The deadline expired before the operation could complete. For operations that change the state of the system, this error may be returned even if the operation has completed successfully. For example, a successful response from a server could have been delayed long enough for the deadline to expire. HTTP Mapping: 504 Gateway Timeout"]
        DeadlineExceeded,
        #[doc = "The operation was rejected because the system is not in a state required for the operation's execution. For example, the directory to be deleted is non-empty, an rmdir operation is applied to a non-directory, etc. Service implementors can use the following guidelines to decide between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`: (a) Use `UNAVAILABLE` if the client can retry just the failing call. (b) Use `ABORTED` if the client should retry at a higher level. For example, when a client-specified test-and-set fails, indicating the client should restart a read-modify-write sequence. (c) Use `FAILED_PRECONDITION` if the client should not retry until the system state has been explicitly fixed. For example, if an \"rmdir\" fails because the directory is non-empty, `FAILED_PRECONDITION` should be returned since the client should not retry unless the files are deleted from the directory. HTTP Mapping: 400 Bad Request"]
        FailedPrecondition,
        #[doc = "Internal errors. This means that some invariants expected by the underlying system have been broken. This error code is reserved for serious errors. HTTP Mapping: 500 Internal Server Error"]
        Internal,
        #[doc = "The client specified an invalid argument. Note that this differs from `FAILED_PRECONDITION`. `INVALID_ARGUMENT` indicates arguments that are problematic regardless of the state of the system (e.g., a malformed file name). HTTP Mapping: 400 Bad Request"]
        InvalidArgument,
        #[doc = "Some requested entity (e.g., file or directory) was not found. Note to server developers: if a request is denied for an entire class of users, such as gradual feature rollout or undocumented allowlist, `NOT_FOUND` may be used. If a request is denied for some users within a class of users, such as user-based access control, `PERMISSION_DENIED` must be used. HTTP Mapping: 404 Not Found"]
        NotFound,
        #[doc = "Not an error; returned on success HTTP Mapping: 200 OK"]
        Ok,
        #[doc = "The operation was attempted past the valid range. E.g., seeking or reading past end-of-file. Unlike `INVALID_ARGUMENT`, this error indicates a problem that may be fixed if the system state changes. For example, a 32-bit file system will generate `INVALID_ARGUMENT` if asked to read at an offset that is not in the range [0,2^32-1], but it will generate `OUT_OF_RANGE` if asked to read from an offset past the current file size. There is a fair bit of overlap between `FAILED_PRECONDITION` and `OUT_OF_RANGE`. We recommend using `OUT_OF_RANGE` (the more specific error) when it applies so that callers who are iterating through a space can easily look for an `OUT_OF_RANGE` error to detect when they are done. HTTP Mapping: 400 Bad Request"]
        OutOfRange,
        #[doc = "The caller does not have permission to execute the specified operation. `PERMISSION_DENIED` must not be used for rejections caused by exhausting some resource (use `RESOURCE_EXHAUSTED` instead for those errors). `PERMISSION_DENIED` must not be used if the caller can not be identified (use `UNAUTHENTICATED` instead for those errors). This error code does not imply the request is valid or the requested entity exists or satisfies other pre-conditions. HTTP Mapping: 403 Forbidden"]
        PermissionDenied,
        #[doc = "Some resource has been exhausted, perhaps a per-user quota, or perhaps the entire file system is out of space. HTTP Mapping: 429 Too Many Requests"]
        ResourceExhausted,
        #[doc = "The request does not have valid authentication credentials for the operation. HTTP Mapping: 401 Unauthorized"]
        Unauthenticated,
        #[doc = "The service is currently unavailable. This is most likely a transient condition, which can be corrected by retrying with a backoff. Note that it is not always safe to retry non-idempotent operations. See the guidelines above for deciding between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`. HTTP Mapping: 503 Service Unavailable"]
        Unavailable,
        #[doc = "The operation is not implemented or is not supported/enabled in this service. HTTP Mapping: 501 Not Implemented"]
        Unimplemented,
        #[doc = "Unknown error. For example, this error may be returned when a `Status` value received from another address space belongs to an error space that is not known in this address space. Also errors raised by APIs that do not return enough error information may be converted to this error. HTTP Mapping: 500 Internal Server Error"]
        Unknown,
    }
    impl ActionStatusStatusCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ActionStatusStatusCode::Aborted => "ABORTED",
                ActionStatusStatusCode::AlreadyExists => "ALREADY_EXISTS",
                ActionStatusStatusCode::Cancelled => "CANCELLED",
                ActionStatusStatusCode::DataLoss => "DATA_LOSS",
                ActionStatusStatusCode::DeadlineExceeded => "DEADLINE_EXCEEDED",
                ActionStatusStatusCode::FailedPrecondition => "FAILED_PRECONDITION",
                ActionStatusStatusCode::Internal => "INTERNAL",
                ActionStatusStatusCode::InvalidArgument => "INVALID_ARGUMENT",
                ActionStatusStatusCode::NotFound => "NOT_FOUND",
                ActionStatusStatusCode::Ok => "OK",
                ActionStatusStatusCode::OutOfRange => "OUT_OF_RANGE",
                ActionStatusStatusCode::PermissionDenied => "PERMISSION_DENIED",
                ActionStatusStatusCode::ResourceExhausted => "RESOURCE_EXHAUSTED",
                ActionStatusStatusCode::Unauthenticated => "UNAUTHENTICATED",
                ActionStatusStatusCode::Unavailable => "UNAVAILABLE",
                ActionStatusStatusCode::Unimplemented => "UNIMPLEMENTED",
                ActionStatusStatusCode::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ActionStatusStatusCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ActionStatusStatusCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ActionStatusStatusCode, ()> {
            Ok(match s {
                "ABORTED" => ActionStatusStatusCode::Aborted,
                "ALREADY_EXISTS" => ActionStatusStatusCode::AlreadyExists,
                "CANCELLED" => ActionStatusStatusCode::Cancelled,
                "DATA_LOSS" => ActionStatusStatusCode::DataLoss,
                "DEADLINE_EXCEEDED" => ActionStatusStatusCode::DeadlineExceeded,
                "FAILED_PRECONDITION" => ActionStatusStatusCode::FailedPrecondition,
                "INTERNAL" => ActionStatusStatusCode::Internal,
                "INVALID_ARGUMENT" => ActionStatusStatusCode::InvalidArgument,
                "NOT_FOUND" => ActionStatusStatusCode::NotFound,
                "OK" => ActionStatusStatusCode::Ok,
                "OUT_OF_RANGE" => ActionStatusStatusCode::OutOfRange,
                "PERMISSION_DENIED" => ActionStatusStatusCode::PermissionDenied,
                "RESOURCE_EXHAUSTED" => ActionStatusStatusCode::ResourceExhausted,
                "UNAUTHENTICATED" => ActionStatusStatusCode::Unauthenticated,
                "UNAVAILABLE" => ActionStatusStatusCode::Unavailable,
                "UNIMPLEMENTED" => ActionStatusStatusCode::Unimplemented,
                "UNKNOWN" => ActionStatusStatusCode::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ActionStatusStatusCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ActionStatusStatusCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ActionStatusStatusCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ABORTED" => ActionStatusStatusCode::Aborted,
                "ALREADY_EXISTS" => ActionStatusStatusCode::AlreadyExists,
                "CANCELLED" => ActionStatusStatusCode::Cancelled,
                "DATA_LOSS" => ActionStatusStatusCode::DataLoss,
                "DEADLINE_EXCEEDED" => ActionStatusStatusCode::DeadlineExceeded,
                "FAILED_PRECONDITION" => ActionStatusStatusCode::FailedPrecondition,
                "INTERNAL" => ActionStatusStatusCode::Internal,
                "INVALID_ARGUMENT" => ActionStatusStatusCode::InvalidArgument,
                "NOT_FOUND" => ActionStatusStatusCode::NotFound,
                "OK" => ActionStatusStatusCode::Ok,
                "OUT_OF_RANGE" => ActionStatusStatusCode::OutOfRange,
                "PERMISSION_DENIED" => ActionStatusStatusCode::PermissionDenied,
                "RESOURCE_EXHAUSTED" => ActionStatusStatusCode::ResourceExhausted,
                "UNAUTHENTICATED" => ActionStatusStatusCode::Unauthenticated,
                "UNAVAILABLE" => ActionStatusStatusCode::Unavailable,
                "UNIMPLEMENTED" => ActionStatusStatusCode::Unimplemented,
                "UNKNOWN" => ActionStatusStatusCode::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ActionStatusStatusCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActionStatusStatusCode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Annotation {
        #[doc = "Length of the substring in the plain-text message body this annotation corresponds to."]
        #[serde(
            rename = "length",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub length: ::std::option::Option<i32>,
        #[doc = "The type of this annotation."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::AnnotationType>,
        #[doc = "The metadata for a slash command."]
        #[serde(
            rename = "slashCommand",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slash_command: ::std::option::Option<crate::schemas::SlashCommandMetadata>,
        #[doc = "Start index (0-based, inclusive) in the plain-text message body this annotation corresponds to."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
        #[doc = "The metadata of user mention."]
        #[serde(
            rename = "userMention",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_mention: ::std::option::Option<crate::schemas::UserMentionMetadata>,
    }
    impl ::google_field_selector::FieldSelector for Annotation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Annotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AnnotationType {
        #[doc = "Default value for the enum. DO NOT USE."]
        AnnotationTypeUnspecified,
        #[doc = "A slash command is invoked."]
        SlashCommand,
        #[doc = "A user is mentioned."]
        UserMention,
    }
    impl AnnotationType {
        pub fn as_str(self) -> &'static str {
            match self {
                AnnotationType::AnnotationTypeUnspecified => "ANNOTATION_TYPE_UNSPECIFIED",
                AnnotationType::SlashCommand => "SLASH_COMMAND",
                AnnotationType::UserMention => "USER_MENTION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AnnotationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AnnotationType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AnnotationType, ()> {
            Ok(match s {
                "ANNOTATION_TYPE_UNSPECIFIED" => AnnotationType::AnnotationTypeUnspecified,
                "SLASH_COMMAND" => AnnotationType::SlashCommand,
                "USER_MENTION" => AnnotationType::UserMention,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AnnotationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AnnotationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AnnotationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANNOTATION_TYPE_UNSPECIFIED" => AnnotationType::AnnotationTypeUnspecified,
                "SLASH_COMMAND" => AnnotationType::SlashCommand,
                "USER_MENTION" => AnnotationType::UserMention,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AnnotationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnnotationType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Attachment {
        #[doc = "A reference to the attachment data. This is used with the media API to download the attachment data."]
        #[serde(
            rename = "attachmentDataRef",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attachment_data_ref: ::std::option::Option<crate::schemas::AttachmentDataRef>,
        #[doc = "The original file name for the content, not the full path."]
        #[serde(
            rename = "contentName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_name: ::std::option::Option<String>,
        #[doc = "The content type (MIME type) of the file."]
        #[serde(
            rename = "contentType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_type: ::std::option::Option<String>,
        #[doc = "Output only. The download URL which should be used to allow a human user to download the attachment. Chat apps should not use this URL to download attachment content."]
        #[serde(
            rename = "downloadUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub download_uri: ::std::option::Option<String>,
        #[doc = "A reference to the drive attachment. This is used with the Drive API."]
        #[serde(
            rename = "driveDataRef",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive_data_ref: ::std::option::Option<crate::schemas::DriveDataRef>,
        #[doc = "Resource name of the attachment, in the form \"spaces/*/messages/*/attachments/*\"."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The source of the attachment."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::AttachmentSource>,
        #[doc = "Output only. The thumbnail URL which should be used to preview the attachment to a human user. Chat apps should not use this URL to download attachment content."]
        #[serde(
            rename = "thumbnailUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Attachment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Attachment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AttachmentSource {
        DriveFile,
        SourceUnspecified,
        UploadedContent,
    }
    impl AttachmentSource {
        pub fn as_str(self) -> &'static str {
            match self {
                AttachmentSource::DriveFile => "DRIVE_FILE",
                AttachmentSource::SourceUnspecified => "SOURCE_UNSPECIFIED",
                AttachmentSource::UploadedContent => "UPLOADED_CONTENT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AttachmentSource {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AttachmentSource {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AttachmentSource, ()> {
            Ok(match s {
                "DRIVE_FILE" => AttachmentSource::DriveFile,
                "SOURCE_UNSPECIFIED" => AttachmentSource::SourceUnspecified,
                "UPLOADED_CONTENT" => AttachmentSource::UploadedContent,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AttachmentSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AttachmentSource {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AttachmentSource {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DRIVE_FILE" => AttachmentSource::DriveFile,
                "SOURCE_UNSPECIFIED" => AttachmentSource::SourceUnspecified,
                "UPLOADED_CONTENT" => AttachmentSource::UploadedContent,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AttachmentSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AttachmentSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AttachmentDataRef {
        #[doc = "The resource name of the attachment data. This is used with the media API to download the attachment data."]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AttachmentDataRef {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AttachmentDataRef {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Button {
        #[doc = "A button with image and onclick action."]
        #[serde(
            rename = "imageButton",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_button: ::std::option::Option<crate::schemas::ImageButton>,
        #[doc = "A button with text and onclick action."]
        #[serde(
            rename = "textButton",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_button: ::std::option::Option<crate::schemas::TextButton>,
    }
    impl ::google_field_selector::FieldSelector for Button {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Button {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Card {
        #[doc = "The actions of this card."]
        #[serde(
            rename = "cardActions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub card_actions: ::std::option::Option<Vec<crate::schemas::CardAction>>,
        #[doc = "The header of the card. A header usually contains a title and an image."]
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<crate::schemas::CardHeader>,
        #[doc = "Name of the card."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Sections are separated by a line divider."]
        #[serde(
            rename = "sections",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sections: ::std::option::Option<Vec<crate::schemas::Section>>,
    }
    impl ::google_field_selector::FieldSelector for Card {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Card {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CardAction {
        #[doc = "The label used to be displayed in the action menu item."]
        #[serde(
            rename = "actionLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action_label: ::std::option::Option<String>,
        #[doc = "The onclick action for this action item."]
        #[serde(
            rename = "onClick",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_click: ::std::option::Option<crate::schemas::OnClick>,
    }
    impl ::google_field_selector::FieldSelector for CardAction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CardAction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CardHeader {
        #[doc = "The image's type (e.g. square border or circular border)."]
        #[serde(
            rename = "imageStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_style: ::std::option::Option<crate::schemas::CardHeaderImageStyle>,
        #[doc = "The URL of the image in the card header."]
        #[serde(
            rename = "imageUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_url: ::std::option::Option<String>,
        #[doc = "The subtitle of the card header."]
        #[serde(
            rename = "subtitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subtitle: ::std::option::Option<String>,
        #[doc = "The title must be specified. The header has a fixed height: if both a title and subtitle is specified, each will take up 1 line. If only the title is specified, it will take up both lines."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CardHeader {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CardHeader {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CardHeaderImageStyle {
        #[doc = "Circular border."]
        Avatar,
        #[doc = "Square border."]
        Image,
        ImageStyleUnspecified,
    }
    impl CardHeaderImageStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                CardHeaderImageStyle::Avatar => "AVATAR",
                CardHeaderImageStyle::Image => "IMAGE",
                CardHeaderImageStyle::ImageStyleUnspecified => "IMAGE_STYLE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CardHeaderImageStyle {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CardHeaderImageStyle {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CardHeaderImageStyle, ()> {
            Ok(match s {
                "AVATAR" => CardHeaderImageStyle::Avatar,
                "IMAGE" => CardHeaderImageStyle::Image,
                "IMAGE_STYLE_UNSPECIFIED" => CardHeaderImageStyle::ImageStyleUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CardHeaderImageStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CardHeaderImageStyle {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CardHeaderImageStyle {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AVATAR" => CardHeaderImageStyle::Avatar,
                "IMAGE" => CardHeaderImageStyle::Image,
                "IMAGE_STYLE_UNSPECIFIED" => CardHeaderImageStyle::ImageStyleUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CardHeaderImageStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CardHeaderImageStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ChatAppLogEntry {
        #[doc = "The deployment that caused the error. For Chat bots built in Apps Script, this is the deployment ID defined by Apps Script."]
        #[serde(
            rename = "deployment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deployment: ::std::option::Option<String>,
        #[doc = "The unencrypted `callback_method` name that was running when the error was encountered."]
        #[serde(
            rename = "deploymentFunction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deployment_function: ::std::option::Option<String>,
        #[doc = "The error code and message."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::Status>,
    }
    impl ::google_field_selector::FieldSelector for ChatAppLogEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ChatAppLogEntry {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Color {
        #[doc = "The fraction of this color that should be applied to the pixel. That is, the final pixel color is defined by the equation: `pixel color = alpha * (this color) + (1.0 - alpha) * (background color)` This means that a value of 1.0 corresponds to a solid color, whereas a value of 0.0 corresponds to a completely transparent color. This uses a wrapper message rather than a simple float scalar so that it is possible to distinguish between a default value and the value being unset. If omitted, this color object is rendered as a solid color (as if the alpha value had been explicitly given a value of 1.0)."]
        #[serde(
            rename = "alpha",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alpha: ::std::option::Option<f32>,
        #[doc = "The amount of blue in the color as a value in the interval [0, 1]."]
        #[serde(
            rename = "blue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blue: ::std::option::Option<f32>,
        #[doc = "The amount of green in the color as a value in the interval [0, 1]."]
        #[serde(
            rename = "green",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub green: ::std::option::Option<f32>,
        #[doc = "The amount of red in the color as a value in the interval [0, 1]."]
        #[serde(
            rename = "red",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub red: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for Color {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Color {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CommonEventObject {
        #[doc = "A map containing the current values of the widgets in a card. The map keys are the string IDs assigned to each widget, and the values represent inputs to the widget. Depending on the input data type, a different object represents each input: For single-value widgets, `StringInput`. For multi-value widgets, an array of `StringInput` objects. For a date-time picker, a `DateTimeInput`. For a date-only picker, a `DateInput`. For a time-only picker, a `TimeInput`. Corresponds with the data entered by a user on a card in a [dialog](https://developers.google.com/chat/how-tos/dialogs)."]
        #[serde(
            rename = "formInputs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub form_inputs:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::Inputs>>,
        #[doc = "The hostApp enum which indicates the app the add-on is invoked from. Always `CHAT` for Chat apps."]
        #[serde(
            rename = "hostApp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub host_app: ::std::option::Option<crate::schemas::CommonEventObjectHostApp>,
        #[doc = "Name of the invoked function associated with the widget. Only set for Chat apps."]
        #[serde(
            rename = "invokedFunction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub invoked_function: ::std::option::Option<String>,
        #[doc = "Custom [parameters](/chat/api/reference/rest/v1/cards#ActionParameter) passed to the invoked function. Both keys and values must be strings."]
        #[serde(
            rename = "parameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameters: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The platform enum which indicates the platform where the event originates (`WEB`, `IOS`, or `ANDROID`). Not supported by Chat apps."]
        #[serde(
            rename = "platform",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform: ::std::option::Option<crate::schemas::CommonEventObjectPlatform>,
        #[doc = "The timezone ID and offset from Coordinated Universal Time (UTC)."]
        #[serde(
            rename = "timeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone: ::std::option::Option<crate::schemas::TimeZone>,
        #[doc = "The full `locale.displayName` in the format of [ISO 639 language code]-[ISO 3166 country/region code] such as \"en-US\". Not supported by Chat apps."]
        #[serde(
            rename = "userLocale",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_locale: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CommonEventObject {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommonEventObject {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CommonEventObjectHostApp {
        #[doc = "The add-on launches from Google Calendar."]
        Calendar,
        #[doc = "A Google Chat app. Not used for Google Workspace Add-ons."]
        Chat,
        #[doc = "Not used."]
        Demo,
        #[doc = "The add-on launches from Google Docs."]
        Docs,
        #[doc = "The add-on launches from Google Drawings."]
        Drawings,
        #[doc = "The add-on launches from Google Drive."]
        Drive,
        #[doc = "The add-on launches from Gmail."]
        Gmail,
        #[doc = "The add-on launches from Google Sheets."]
        Sheets,
        #[doc = "The add-on launches from Google Slides."]
        Slides,
        #[doc = "Google can't identify a host app."]
        UnspecifiedHostApp,
    }
    impl CommonEventObjectHostApp {
        pub fn as_str(self) -> &'static str {
            match self {
                CommonEventObjectHostApp::Calendar => "CALENDAR",
                CommonEventObjectHostApp::Chat => "CHAT",
                CommonEventObjectHostApp::Demo => "DEMO",
                CommonEventObjectHostApp::Docs => "DOCS",
                CommonEventObjectHostApp::Drawings => "DRAWINGS",
                CommonEventObjectHostApp::Drive => "DRIVE",
                CommonEventObjectHostApp::Gmail => "GMAIL",
                CommonEventObjectHostApp::Sheets => "SHEETS",
                CommonEventObjectHostApp::Slides => "SLIDES",
                CommonEventObjectHostApp::UnspecifiedHostApp => "UNSPECIFIED_HOST_APP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CommonEventObjectHostApp {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CommonEventObjectHostApp {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CommonEventObjectHostApp, ()> {
            Ok(match s {
                "CALENDAR" => CommonEventObjectHostApp::Calendar,
                "CHAT" => CommonEventObjectHostApp::Chat,
                "DEMO" => CommonEventObjectHostApp::Demo,
                "DOCS" => CommonEventObjectHostApp::Docs,
                "DRAWINGS" => CommonEventObjectHostApp::Drawings,
                "DRIVE" => CommonEventObjectHostApp::Drive,
                "GMAIL" => CommonEventObjectHostApp::Gmail,
                "SHEETS" => CommonEventObjectHostApp::Sheets,
                "SLIDES" => CommonEventObjectHostApp::Slides,
                "UNSPECIFIED_HOST_APP" => CommonEventObjectHostApp::UnspecifiedHostApp,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CommonEventObjectHostApp {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CommonEventObjectHostApp {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CommonEventObjectHostApp {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CALENDAR" => CommonEventObjectHostApp::Calendar,
                "CHAT" => CommonEventObjectHostApp::Chat,
                "DEMO" => CommonEventObjectHostApp::Demo,
                "DOCS" => CommonEventObjectHostApp::Docs,
                "DRAWINGS" => CommonEventObjectHostApp::Drawings,
                "DRIVE" => CommonEventObjectHostApp::Drive,
                "GMAIL" => CommonEventObjectHostApp::Gmail,
                "SHEETS" => CommonEventObjectHostApp::Sheets,
                "SLIDES" => CommonEventObjectHostApp::Slides,
                "UNSPECIFIED_HOST_APP" => CommonEventObjectHostApp::UnspecifiedHostApp,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CommonEventObjectHostApp {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommonEventObjectHostApp {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CommonEventObjectPlatform {
        Android,
        Ios,
        UnknownPlatform,
        Web,
    }
    impl CommonEventObjectPlatform {
        pub fn as_str(self) -> &'static str {
            match self {
                CommonEventObjectPlatform::Android => "ANDROID",
                CommonEventObjectPlatform::Ios => "IOS",
                CommonEventObjectPlatform::UnknownPlatform => "UNKNOWN_PLATFORM",
                CommonEventObjectPlatform::Web => "WEB",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CommonEventObjectPlatform {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CommonEventObjectPlatform {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CommonEventObjectPlatform, ()> {
            Ok(match s {
                "ANDROID" => CommonEventObjectPlatform::Android,
                "IOS" => CommonEventObjectPlatform::Ios,
                "UNKNOWN_PLATFORM" => CommonEventObjectPlatform::UnknownPlatform,
                "WEB" => CommonEventObjectPlatform::Web,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CommonEventObjectPlatform {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CommonEventObjectPlatform {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CommonEventObjectPlatform {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANDROID" => CommonEventObjectPlatform::Android,
                "IOS" => CommonEventObjectPlatform::Ios,
                "UNKNOWN_PLATFORM" => CommonEventObjectPlatform::UnknownPlatform,
                "WEB" => CommonEventObjectPlatform::Web,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CommonEventObjectPlatform {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommonEventObjectPlatform {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DateInput {
        #[doc = "Time since epoch time, in milliseconds."]
        #[serde(
            rename = "msSinceEpoch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub ms_since_epoch: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for DateInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DateInput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DateTimeInput {
        #[doc = "Whether the `datetime` input includes a calendar date."]
        #[serde(
            rename = "hasDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_date: ::std::option::Option<bool>,
        #[doc = "Whether the `datetime` input includes a timestamp."]
        #[serde(
            rename = "hasTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_time: ::std::option::Option<bool>,
        #[doc = "Time since epoch time, in milliseconds."]
        #[serde(
            rename = "msSinceEpoch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub ms_since_epoch: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for DateTimeInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DateTimeInput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DeprecatedEvent {
        #[doc = "The form action data associated with an interactive card that was clicked. Only populated for CARD_CLICKED events. See the [Interactive Cards guide](/chat/how-tos/cards-onclick) for more information."]
        #[serde(
            rename = "action",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action: ::std::option::Option<crate::schemas::FormAction>,
        #[doc = "Represents information about the user's client, such as locale, host app, and platform. For Chat apps, `CommonEventObject` includes information submitted by users interacting with [dialogs](https://developers.google.com/chat/how-tos/dialogs), like data entered on a card."]
        #[serde(
            rename = "common",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common: ::std::option::Option<crate::schemas::CommonEventObject>,
        #[doc = "The URL the Chat app should redirect the user to after they have completed an authorization or configuration flow outside of Google Chat. See the [Authorizing access to 3p services guide](/chat/how-tos/auth-3p) for more information."]
        #[serde(
            rename = "configCompleteRedirectUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config_complete_redirect_url: ::std::option::Option<String>,
        #[doc = "The type of [dialog](https://developers.google.com/chat/how-tos/dialogs) event received."]
        #[serde(
            rename = "dialogEventType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dialog_event_type:
            ::std::option::Option<crate::schemas::DeprecatedEventDialogEventType>,
        #[doc = "The timestamp indicating when the event occurred."]
        #[serde(
            rename = "eventTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_time: ::std::option::Option<String>,
        #[doc = "True when the event is related to [dialogs](https://developers.google.com/chat/how-tos/dialogs)."]
        #[serde(
            rename = "isDialogEvent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_dialog_event: ::std::option::Option<bool>,
        #[doc = "The message that triggered the event, if applicable."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<crate::schemas::Message>,
        #[doc = "The type of the event."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::DeprecatedEventType>,
        #[doc = "The space in which the event occurred."]
        #[serde(
            rename = "space",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub space: ::std::option::Option<crate::schemas::Space>,
        #[doc = "The Chat app-defined key for the thread related to the event. See the thread_key field of the `spaces.message.create` request for more information."]
        #[serde(
            rename = "threadKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thread_key: ::std::option::Option<String>,
        #[doc = "A secret value that legacy Chat apps can use to verify if a request is from Google. Google randomly generates the token, and its value remains static. You can obtain, revoke, or regenerate the token from the [Chat API configuration page](https://console.cloud.google.com/apis/api/chat.googleapis.com/hangouts-chat) in the Google Cloud Console. Modern Chat apps don't use this field. It is absent from API responses and the [Chat API configuration page](https://console.cloud.google.com/apis/api/chat.googleapis.com/hangouts-chat)."]
        #[serde(
            rename = "token",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub token: ::std::option::Option<String>,
        #[doc = "The user that triggered the event."]
        #[serde(
            rename = "user",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user: ::std::option::Option<crate::schemas::User>,
    }
    impl ::google_field_selector::FieldSelector for DeprecatedEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeprecatedEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeprecatedEventDialogEventType {
        #[doc = "The [dialog](https://developers.google.com/chat/how-tos/dialogs) was cancelled."]
        CancelDialog,
        #[doc = "Any user action that opens a [dialog](https://developers.google.com/chat/how-tos/dialogs)."]
        RequestDialog,
        #[doc = "A card click event from a [dialog](https://developers.google.com/chat/how-tos/dialogs)."]
        SubmitDialog,
        #[doc = "This could be used when the corresponding event is not dialog related. For example an @mention."]
        TypeUnspecified,
    }
    impl DeprecatedEventDialogEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                DeprecatedEventDialogEventType::CancelDialog => "CANCEL_DIALOG",
                DeprecatedEventDialogEventType::RequestDialog => "REQUEST_DIALOG",
                DeprecatedEventDialogEventType::SubmitDialog => "SUBMIT_DIALOG",
                DeprecatedEventDialogEventType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeprecatedEventDialogEventType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeprecatedEventDialogEventType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeprecatedEventDialogEventType, ()> {
            Ok(match s {
                "CANCEL_DIALOG" => DeprecatedEventDialogEventType::CancelDialog,
                "REQUEST_DIALOG" => DeprecatedEventDialogEventType::RequestDialog,
                "SUBMIT_DIALOG" => DeprecatedEventDialogEventType::SubmitDialog,
                "TYPE_UNSPECIFIED" => DeprecatedEventDialogEventType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeprecatedEventDialogEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeprecatedEventDialogEventType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeprecatedEventDialogEventType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CANCEL_DIALOG" => DeprecatedEventDialogEventType::CancelDialog,
                "REQUEST_DIALOG" => DeprecatedEventDialogEventType::RequestDialog,
                "SUBMIT_DIALOG" => DeprecatedEventDialogEventType::SubmitDialog,
                "TYPE_UNSPECIFIED" => DeprecatedEventDialogEventType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeprecatedEventDialogEventType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeprecatedEventDialogEventType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeprecatedEventType {
        #[doc = "The Chat app was added to a space."]
        AddedToSpace,
        #[doc = "The Chat app's interactive card was clicked."]
        CardClicked,
        #[doc = "A message was sent in a space."]
        Message,
        #[doc = "The Chat app was removed from a space."]
        RemovedFromSpace,
        #[doc = "Default value for the enum. DO NOT USE."]
        Unspecified,
    }
    impl DeprecatedEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                DeprecatedEventType::AddedToSpace => "ADDED_TO_SPACE",
                DeprecatedEventType::CardClicked => "CARD_CLICKED",
                DeprecatedEventType::Message => "MESSAGE",
                DeprecatedEventType::RemovedFromSpace => "REMOVED_FROM_SPACE",
                DeprecatedEventType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DeprecatedEventType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DeprecatedEventType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DeprecatedEventType, ()> {
            Ok(match s {
                "ADDED_TO_SPACE" => DeprecatedEventType::AddedToSpace,
                "CARD_CLICKED" => DeprecatedEventType::CardClicked,
                "MESSAGE" => DeprecatedEventType::Message,
                "REMOVED_FROM_SPACE" => DeprecatedEventType::RemovedFromSpace,
                "UNSPECIFIED" => DeprecatedEventType::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DeprecatedEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeprecatedEventType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeprecatedEventType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADDED_TO_SPACE" => DeprecatedEventType::AddedToSpace,
                "CARD_CLICKED" => DeprecatedEventType::CardClicked,
                "MESSAGE" => DeprecatedEventType::Message,
                "REMOVED_FROM_SPACE" => DeprecatedEventType::RemovedFromSpace,
                "UNSPECIFIED" => DeprecatedEventType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DeprecatedEventType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeprecatedEventType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Dialog {
        #[doc = "Input only. Body of the dialog, which is rendered in a modal. Google Chat apps do not support the following card entities: `DateTimePicker`, `OnChangeAction`."]
        #[serde(
            rename = "body",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub body: ::std::option::Option<crate::schemas::GoogleAppsCardV1Card>,
    }
    impl ::google_field_selector::FieldSelector for Dialog {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Dialog {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DialogAction {
        #[doc = "Input only. Status for a request to either invoke or submit a [dialog](https://developers.google.com/chat/how-tos/dialogs). Displays a status and message to users, if necessary. For example, in case of an error or success."]
        #[serde(
            rename = "actionStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action_status: ::std::option::Option<crate::schemas::ActionStatus>,
        #[doc = "Input only. [Dialog](https://developers.google.com/chat/how-tos/dialogs) for the request."]
        #[serde(
            rename = "dialog",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dialog: ::std::option::Option<crate::schemas::Dialog>,
    }
    impl ::google_field_selector::FieldSelector for DialogAction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DialogAction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DriveDataRef {
        #[doc = "The id for the drive file, for use with the Drive API."]
        #[serde(
            rename = "driveFileId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive_file_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DriveDataRef {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveDataRef {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct FormAction {
        #[doc = "The method name is used to identify which part of the form triggered the form submission. This information is echoed back to the Chat app as part of the card click event. The same method name can be used for several elements that trigger a common behavior if desired."]
        #[serde(
            rename = "actionMethodName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action_method_name: ::std::option::Option<String>,
        #[doc = "List of action parameters."]
        #[serde(
            rename = "parameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameters: ::std::option::Option<Vec<crate::schemas::ActionParameter>>,
    }
    impl ::google_field_selector::FieldSelector for FormAction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FormAction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1Action {
        #[doc = "Apps Script function to invoke when the containing element is clicked/activated."]
        #[serde(
            rename = "function",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub function: ::std::option::Option<String>,
        #[doc = "Specifies the loading indicator that the action displays while making the call to the action."]
        #[serde(
            rename = "loadIndicator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub load_indicator:
            ::std::option::Option<crate::schemas::GoogleAppsCardV1ActionLoadIndicator>,
        #[doc = "List of action parameters."]
        #[serde(
            rename = "parameters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parameters: ::std::option::Option<Vec<crate::schemas::GoogleAppsCardV1ActionParameter>>,
        #[doc = "Indicates whether form values persist after the action. The default value is `false`. If `true`, form values remain after the action is triggered. When using [LoadIndicator.NONE](workspace/add-ons/reference/rpc/google.apps.card.v1#loadindicator) for actions, `persist_values` = `true`is recommended, as it ensures that any changes made by the user after form or on change actions are sent to the server are not overwritten by the response. If `false`, the form values are cleared when the action is triggered. When `persist_values` is set to `false`, it is strongly recommended that the card use [LoadIndicator.SPINNER](workspace/add-ons/reference/rpc/google.apps.card.v1#loadindicator) for all actions, as this locks the UI to ensure no changes are made by the user while the action is being processed."]
        #[serde(
            rename = "persistValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub persist_values: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1Action {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1Action {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsCardV1ActionLoadIndicator {
        #[doc = "Nothing is displayed."]
        None,
        #[doc = "Displays a spinner to indicate that content is loading."]
        Spinner,
    }
    impl GoogleAppsCardV1ActionLoadIndicator {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsCardV1ActionLoadIndicator::None => "NONE",
                GoogleAppsCardV1ActionLoadIndicator::Spinner => "SPINNER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsCardV1ActionLoadIndicator {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsCardV1ActionLoadIndicator {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsCardV1ActionLoadIndicator, ()> {
            Ok(match s {
                "NONE" => GoogleAppsCardV1ActionLoadIndicator::None,
                "SPINNER" => GoogleAppsCardV1ActionLoadIndicator::Spinner,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsCardV1ActionLoadIndicator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsCardV1ActionLoadIndicator {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsCardV1ActionLoadIndicator {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => GoogleAppsCardV1ActionLoadIndicator::None,
                "SPINNER" => GoogleAppsCardV1ActionLoadIndicator::Spinner,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1ActionLoadIndicator {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1ActionLoadIndicator {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1ActionParameter {
        #[doc = "The name of the parameter for the action script."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "The value of the parameter."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1ActionParameter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1ActionParameter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1BorderStyle {
        #[doc = "The corner radius for the border."]
        #[serde(
            rename = "cornerRadius",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub corner_radius: ::std::option::Option<i32>,
        #[doc = "The border type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::GoogleAppsCardV1BorderStyleType>,
        #[doc = "The colors to use when the type is `BORDER_TYPE_STROKE`."]
        #[serde(
            rename = "strokeColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stroke_color: ::std::option::Option<crate::schemas::Color>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1BorderStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1BorderStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsCardV1BorderStyleType {
        #[doc = "No value specified."]
        BorderTypeUnspecified,
        #[doc = "No border."]
        NoBorder,
        #[doc = "Outline."]
        Stroke,
    }
    impl GoogleAppsCardV1BorderStyleType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsCardV1BorderStyleType::BorderTypeUnspecified => "BORDER_TYPE_UNSPECIFIED",
                GoogleAppsCardV1BorderStyleType::NoBorder => "NO_BORDER",
                GoogleAppsCardV1BorderStyleType::Stroke => "STROKE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsCardV1BorderStyleType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsCardV1BorderStyleType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsCardV1BorderStyleType, ()> {
            Ok(match s {
                "BORDER_TYPE_UNSPECIFIED" => GoogleAppsCardV1BorderStyleType::BorderTypeUnspecified,
                "NO_BORDER" => GoogleAppsCardV1BorderStyleType::NoBorder,
                "STROKE" => GoogleAppsCardV1BorderStyleType::Stroke,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsCardV1BorderStyleType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsCardV1BorderStyleType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsCardV1BorderStyleType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BORDER_TYPE_UNSPECIFIED" => GoogleAppsCardV1BorderStyleType::BorderTypeUnspecified,
                "NO_BORDER" => GoogleAppsCardV1BorderStyleType::NoBorder,
                "STROKE" => GoogleAppsCardV1BorderStyleType::Stroke,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1BorderStyleType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1BorderStyleType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1Button {
        #[doc = "The alternative text used for accessibility. Has no effect when an icon is set; use `icon.alt_text` instead."]
        #[serde(
            rename = "altText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alt_text: ::std::option::Option<String>,
        #[doc = "If set, the button is filled with a solid background."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "If `true`, the button is displayed in a disabled state and doesn't respond to user actions."]
        #[serde(
            rename = "disabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "The icon image."]
        #[serde(
            rename = "icon",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon: ::std::option::Option<crate::schemas::GoogleAppsCardV1Icon>,
        #[doc = "The action to perform when the button is clicked."]
        #[serde(
            rename = "onClick",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_click: ::std::option::Option<Box<crate::schemas::GoogleAppsCardV1OnClick>>,
        #[doc = "The text of the button."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1Button {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1Button {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1ButtonList {
        #[doc = "An array of buttons."]
        #[serde(
            rename = "buttons",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buttons: ::std::option::Option<Vec<crate::schemas::GoogleAppsCardV1Button>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1ButtonList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1ButtonList {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1Card {
        #[doc = "The card's actions. Actions are added to the card's generated toolbar menu. For example, the following JSON constructs a card action menu with Settings and Send Feedback options: `\"card_actions\": [ { \"actionLabel\": \"Settings\", \"onClick\": { \"action\": { \"functionName\": \"goToView\", \"parameters\": [ { \"key\": \"viewType\", \"value\": \"SETTING\" } ], \"loadIndicator\": \"LoadIndicator.SPINNER\" } } }, { \"actionLabel\": \"Send Feedback\", \"onClick\": { \"openLink\": { \"url\": \"https://example.com/feedback\" } } } ]`"]
        #[serde(
            rename = "cardActions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub card_actions: ::std::option::Option<Vec<crate::schemas::GoogleAppsCardV1CardAction>>,
        #[doc = "The display style for `peekCardHeader`."]
        #[serde(
            rename = "displayStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_style: ::std::option::Option<crate::schemas::GoogleAppsCardV1CardDisplayStyle>,
        #[doc = "The fixed footer shown at the bottom of this card."]
        #[serde(
            rename = "fixedFooter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fixed_footer:
            ::std::option::Option<Box<crate::schemas::GoogleAppsCardV1CardFixedFooter>>,
        #[doc = "The header of the card. A header usually contains a title and an image."]
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<crate::schemas::GoogleAppsCardV1CardHeader>,
        #[doc = "Name of the card. Used as a card identifier in card navigation."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "When displaying contextual content, the peek card header acts as a placeholder so that the user can navigate forward between the homepage cards and the contextual cards."]
        #[serde(
            rename = "peekCardHeader",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub peek_card_header: ::std::option::Option<crate::schemas::GoogleAppsCardV1CardHeader>,
        #[doc = "Sections are separated by a line divider."]
        #[serde(
            rename = "sections",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sections: ::std::option::Option<Vec<crate::schemas::GoogleAppsCardV1Section>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1Card {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1Card {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsCardV1CardDisplayStyle {
        #[doc = "Default value. Do not use."]
        DisplayStyleUnspecified,
        #[doc = "The header of the card appears at the bottom of the sidebar, partially covering the current top card of the stack. Clicking the header pops the card into the card stack. If the card has no header, a generated header is used instead."]
        Peek,
        #[doc = "The card is shown by replacing the view of the top card in the card stack."]
        Replace,
    }
    impl GoogleAppsCardV1CardDisplayStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsCardV1CardDisplayStyle::DisplayStyleUnspecified => {
                    "DISPLAY_STYLE_UNSPECIFIED"
                }
                GoogleAppsCardV1CardDisplayStyle::Peek => "PEEK",
                GoogleAppsCardV1CardDisplayStyle::Replace => "REPLACE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsCardV1CardDisplayStyle {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsCardV1CardDisplayStyle {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsCardV1CardDisplayStyle, ()> {
            Ok(match s {
                "DISPLAY_STYLE_UNSPECIFIED" => {
                    GoogleAppsCardV1CardDisplayStyle::DisplayStyleUnspecified
                }
                "PEEK" => GoogleAppsCardV1CardDisplayStyle::Peek,
                "REPLACE" => GoogleAppsCardV1CardDisplayStyle::Replace,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsCardV1CardDisplayStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsCardV1CardDisplayStyle {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsCardV1CardDisplayStyle {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DISPLAY_STYLE_UNSPECIFIED" => {
                    GoogleAppsCardV1CardDisplayStyle::DisplayStyleUnspecified
                }
                "PEEK" => GoogleAppsCardV1CardDisplayStyle::Peek,
                "REPLACE" => GoogleAppsCardV1CardDisplayStyle::Replace,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1CardDisplayStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1CardDisplayStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1CardAction {
        #[doc = "The label that displays as the action menu item."]
        #[serde(
            rename = "actionLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action_label: ::std::option::Option<String>,
        #[doc = "The `onClick` action for this action item."]
        #[serde(
            rename = "onClick",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_click: ::std::option::Option<crate::schemas::GoogleAppsCardV1OnClick>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1CardAction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1CardAction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1CardFixedFooter {
        #[doc = "The primary button of the fixed footer. The button must be a text button with text and color set."]
        #[serde(
            rename = "primaryButton",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primary_button: ::std::option::Option<Box<crate::schemas::GoogleAppsCardV1Button>>,
        #[doc = "The secondary button of the fixed footer. The button must be a text button with text and color set. `primaryButton` must be set if `secondaryButton` is set."]
        #[serde(
            rename = "secondaryButton",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub secondary_button: ::std::option::Option<Box<crate::schemas::GoogleAppsCardV1Button>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1CardFixedFooter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1CardFixedFooter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1CardHeader {
        #[doc = "The alternative text of this image which is used for accessibility."]
        #[serde(
            rename = "imageAltText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_alt_text: ::std::option::Option<String>,
        #[doc = "The image's type."]
        #[serde(
            rename = "imageType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_type: ::std::option::Option<crate::schemas::GoogleAppsCardV1CardHeaderImageType>,
        #[doc = "The URL of the image in the card header."]
        #[serde(
            rename = "imageUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_url: ::std::option::Option<String>,
        #[doc = "The subtitle of the card header."]
        #[serde(
            rename = "subtitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subtitle: ::std::option::Option<String>,
        #[doc = "Required. The title of the card header. The header has a fixed height: if both a title and subtitle are specified, each takes up one line. If only the title is specified, it takes up both lines."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1CardHeader {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1CardHeader {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsCardV1CardHeaderImageType {
        #[doc = "Applies a circular mask to the image."]
        Circle,
        #[doc = "Applies no cropping to the image."]
        Square,
    }
    impl GoogleAppsCardV1CardHeaderImageType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsCardV1CardHeaderImageType::Circle => "CIRCLE",
                GoogleAppsCardV1CardHeaderImageType::Square => "SQUARE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsCardV1CardHeaderImageType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsCardV1CardHeaderImageType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsCardV1CardHeaderImageType, ()> {
            Ok(match s {
                "CIRCLE" => GoogleAppsCardV1CardHeaderImageType::Circle,
                "SQUARE" => GoogleAppsCardV1CardHeaderImageType::Square,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsCardV1CardHeaderImageType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsCardV1CardHeaderImageType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsCardV1CardHeaderImageType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CIRCLE" => GoogleAppsCardV1CardHeaderImageType::Circle,
                "SQUARE" => GoogleAppsCardV1CardHeaderImageType::Square,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1CardHeaderImageType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1CardHeaderImageType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1DateTimePicker {
        #[doc = "The label for the field that displays to the user."]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
        #[doc = "The name of the text input that's used in `formInput`, and uniquely identifies this input."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Triggered when the user clicks Save or Clear from the date/time picker dialog. This is only triggered if the value changed as a result of the Save/Clear operation."]
        #[serde(
            rename = "onChangeAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_change_action: ::std::option::Option<crate::schemas::GoogleAppsCardV1Action>,
        #[doc = "The type of the date/time picker."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::GoogleAppsCardV1DateTimePickerType>,
        #[doc = "The number representing the time zone offset from UTC, in minutes. If set, the `value_ms_epoch` is displayed in the specified time zone. If not set, it uses the user's time zone setting on the client side."]
        #[serde(
            rename = "timezoneOffsetDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timezone_offset_date: ::std::option::Option<i32>,
        #[doc = "The value to display as the default value before user input or previous user input. It is represented in milliseconds (Epoch time). For `DATE_AND_TIME` type, the full epoch value is used. For `DATE_ONLY` type, only date of the epoch time is used. For `TIME_ONLY` type, only time of the epoch time is used. For example, you can set epoch time to `3 * 60 * 60 * 1000` to represent 3am."]
        #[serde(
            rename = "valueMsEpoch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub value_ms_epoch: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1DateTimePicker {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1DateTimePicker {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsCardV1DateTimePickerType {
        #[doc = "The user can select a date and time."]
        DateAndTime,
        #[doc = "The user can only select a date."]
        DateOnly,
        #[doc = "The user can only select a time."]
        TimeOnly,
    }
    impl GoogleAppsCardV1DateTimePickerType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsCardV1DateTimePickerType::DateAndTime => "DATE_AND_TIME",
                GoogleAppsCardV1DateTimePickerType::DateOnly => "DATE_ONLY",
                GoogleAppsCardV1DateTimePickerType::TimeOnly => "TIME_ONLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsCardV1DateTimePickerType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsCardV1DateTimePickerType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsCardV1DateTimePickerType, ()> {
            Ok(match s {
                "DATE_AND_TIME" => GoogleAppsCardV1DateTimePickerType::DateAndTime,
                "DATE_ONLY" => GoogleAppsCardV1DateTimePickerType::DateOnly,
                "TIME_ONLY" => GoogleAppsCardV1DateTimePickerType::TimeOnly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsCardV1DateTimePickerType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsCardV1DateTimePickerType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsCardV1DateTimePickerType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DATE_AND_TIME" => GoogleAppsCardV1DateTimePickerType::DateAndTime,
                "DATE_ONLY" => GoogleAppsCardV1DateTimePickerType::DateOnly,
                "TIME_ONLY" => GoogleAppsCardV1DateTimePickerType::TimeOnly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1DateTimePickerType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1DateTimePickerType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1DecoratedText {
        #[doc = "The formatted text label that shows below the main text."]
        #[serde(
            rename = "bottomLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bottom_label: ::std::option::Option<String>,
        #[doc = "A button that can be clicked to trigger an action."]
        #[serde(
            rename = "button",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub button: ::std::option::Option<crate::schemas::GoogleAppsCardV1Button>,
        #[doc = "An icon displayed after the text."]
        #[serde(
            rename = "endIcon",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_icon: ::std::option::Option<crate::schemas::GoogleAppsCardV1Icon>,
        #[doc = "Deprecated in favor of start_icon."]
        #[serde(
            rename = "icon",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon: ::std::option::Option<crate::schemas::GoogleAppsCardV1Icon>,
        #[doc = "Only the top and bottom label and content region are clickable."]
        #[serde(
            rename = "onClick",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_click: ::std::option::Option<crate::schemas::GoogleAppsCardV1OnClick>,
        #[doc = "The icon displayed in front of the text."]
        #[serde(
            rename = "startIcon",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_icon: ::std::option::Option<crate::schemas::GoogleAppsCardV1Icon>,
        #[doc = "A switch widget can be clicked to change its state or trigger an action."]
        #[serde(
            rename = "switchControl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub switch_control: ::std::option::Option<crate::schemas::GoogleAppsCardV1SwitchControl>,
        #[doc = "Required. The main widget formatted text. See Text formatting for details."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
        #[doc = "The formatted text label that shows above the main text."]
        #[serde(
            rename = "topLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top_label: ::std::option::Option<String>,
        #[doc = "The wrap text setting. If `true`, the text is wrapped and displayed in multiline. Otherwise, the text is truncated."]
        #[serde(
            rename = "wrapText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub wrap_text: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1DecoratedText {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1DecoratedText {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct GoogleAppsCardV1Divider {}
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1Divider {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1Divider {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1Grid {
        #[doc = "The border style to apply to each grid item."]
        #[serde(
            rename = "borderStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_style: ::std::option::Option<crate::schemas::GoogleAppsCardV1BorderStyle>,
        #[doc = "The number of columns to display in the grid. A default value is used if this field isn't specified, and that default value is different depending on where the grid is shown (dialog versus companion)."]
        #[serde(
            rename = "columnCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_count: ::std::option::Option<i32>,
        #[doc = "The items to display in the grid."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::GoogleAppsCardV1GridItem>>,
        #[doc = "This callback is reused by each individual grid item, but with the item's identifier and index in the items list added to the callback's parameters."]
        #[serde(
            rename = "onClick",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_click: ::std::option::Option<crate::schemas::GoogleAppsCardV1OnClick>,
        #[doc = "The text that displays in the grid header."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1Grid {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1Grid {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1GridItem {
        #[doc = "A user-specified identifier for this grid item. This identifier is returned in the parent Grid's onClick callback parameters."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The image that displays in the grid item."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::GoogleAppsCardV1ImageComponent>,
        #[doc = "The layout to use for the grid item."]
        #[serde(
            rename = "layout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub layout: ::std::option::Option<crate::schemas::GoogleAppsCardV1GridItemLayout>,
        #[doc = "The grid item's subtitle."]
        #[serde(
            rename = "subtitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subtitle: ::std::option::Option<String>,
        #[doc = "The horizontal alignment of the grid item's text."]
        #[serde(
            rename = "textAlignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_alignment:
            ::std::option::Option<crate::schemas::GoogleAppsCardV1GridItemTextAlignment>,
        #[doc = "The grid item's title."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1GridItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1GridItem {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsCardV1GridItemLayout {
        #[doc = "No layout specified."]
        GridItemLayoutUnspecified,
        #[doc = "The title and subtitle are shown above the grid item's image."]
        TextAbove,
        #[doc = "The title and subtitle are shown below the grid item's image."]
        TextBelow,
    }
    impl GoogleAppsCardV1GridItemLayout {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsCardV1GridItemLayout::GridItemLayoutUnspecified => {
                    "GRID_ITEM_LAYOUT_UNSPECIFIED"
                }
                GoogleAppsCardV1GridItemLayout::TextAbove => "TEXT_ABOVE",
                GoogleAppsCardV1GridItemLayout::TextBelow => "TEXT_BELOW",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsCardV1GridItemLayout {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsCardV1GridItemLayout {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsCardV1GridItemLayout, ()> {
            Ok(match s {
                "GRID_ITEM_LAYOUT_UNSPECIFIED" => {
                    GoogleAppsCardV1GridItemLayout::GridItemLayoutUnspecified
                }
                "TEXT_ABOVE" => GoogleAppsCardV1GridItemLayout::TextAbove,
                "TEXT_BELOW" => GoogleAppsCardV1GridItemLayout::TextBelow,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsCardV1GridItemLayout {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsCardV1GridItemLayout {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsCardV1GridItemLayout {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GRID_ITEM_LAYOUT_UNSPECIFIED" => {
                    GoogleAppsCardV1GridItemLayout::GridItemLayoutUnspecified
                }
                "TEXT_ABOVE" => GoogleAppsCardV1GridItemLayout::TextAbove,
                "TEXT_BELOW" => GoogleAppsCardV1GridItemLayout::TextBelow,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1GridItemLayout {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1GridItemLayout {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsCardV1GridItemTextAlignment {
        #[doc = "Alignment to the center position."]
        Center,
        #[doc = "Alignment to the end position."]
        End,
        #[doc = "Unspecified alignment."]
        HorizontalAlignmentUnspecified,
        #[doc = "Alignment to the start position."]
        Start,
    }
    impl GoogleAppsCardV1GridItemTextAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsCardV1GridItemTextAlignment::Center => "CENTER",
                GoogleAppsCardV1GridItemTextAlignment::End => "END",
                GoogleAppsCardV1GridItemTextAlignment::HorizontalAlignmentUnspecified => {
                    "HORIZONTAL_ALIGNMENT_UNSPECIFIED"
                }
                GoogleAppsCardV1GridItemTextAlignment::Start => "START",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsCardV1GridItemTextAlignment {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsCardV1GridItemTextAlignment {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsCardV1GridItemTextAlignment, ()> {
            Ok(match s {
                "CENTER" => GoogleAppsCardV1GridItemTextAlignment::Center,
                "END" => GoogleAppsCardV1GridItemTextAlignment::End,
                "HORIZONTAL_ALIGNMENT_UNSPECIFIED" => {
                    GoogleAppsCardV1GridItemTextAlignment::HorizontalAlignmentUnspecified
                }
                "START" => GoogleAppsCardV1GridItemTextAlignment::Start,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsCardV1GridItemTextAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsCardV1GridItemTextAlignment {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsCardV1GridItemTextAlignment {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CENTER" => GoogleAppsCardV1GridItemTextAlignment::Center,
                "END" => GoogleAppsCardV1GridItemTextAlignment::End,
                "HORIZONTAL_ALIGNMENT_UNSPECIFIED" => {
                    GoogleAppsCardV1GridItemTextAlignment::HorizontalAlignmentUnspecified
                }
                "START" => GoogleAppsCardV1GridItemTextAlignment::Start,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1GridItemTextAlignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1GridItemTextAlignment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1Icon {
        #[doc = "The description of the icon, used for accessibility. The default value is provided if you don't specify one."]
        #[serde(
            rename = "altText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alt_text: ::std::option::Option<String>,
        #[doc = "The icon specified by a URL."]
        #[serde(
            rename = "iconUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon_url: ::std::option::Option<String>,
        #[doc = "The crop style applied to the image. In some cases, applying a `CIRCLE` crop causes the image to be drawn larger than a standard icon."]
        #[serde(
            rename = "imageType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_type: ::std::option::Option<crate::schemas::GoogleAppsCardV1IconImageType>,
        #[doc = "The icon specified by the string name of a list of known icons."]
        #[serde(
            rename = "knownIcon",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub known_icon: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1Icon {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1Icon {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsCardV1IconImageType {
        #[doc = "Applies a circular mask to the image."]
        Circle,
        #[doc = "Applies no cropping to the image."]
        Square,
    }
    impl GoogleAppsCardV1IconImageType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsCardV1IconImageType::Circle => "CIRCLE",
                GoogleAppsCardV1IconImageType::Square => "SQUARE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsCardV1IconImageType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsCardV1IconImageType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsCardV1IconImageType, ()> {
            Ok(match s {
                "CIRCLE" => GoogleAppsCardV1IconImageType::Circle,
                "SQUARE" => GoogleAppsCardV1IconImageType::Square,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsCardV1IconImageType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsCardV1IconImageType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsCardV1IconImageType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CIRCLE" => GoogleAppsCardV1IconImageType::Circle,
                "SQUARE" => GoogleAppsCardV1IconImageType::Square,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1IconImageType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1IconImageType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1Image {
        #[doc = "The alternative text of this image, used for accessibility."]
        #[serde(
            rename = "altText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alt_text: ::std::option::Option<String>,
        #[doc = "An image URL."]
        #[serde(
            rename = "imageUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_url: ::std::option::Option<String>,
        #[doc = "The action triggered by an `onClick` event."]
        #[serde(
            rename = "onClick",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_click: ::std::option::Option<crate::schemas::GoogleAppsCardV1OnClick>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1Image {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1Image {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1ImageComponent {
        #[doc = "The accessibility label for the image."]
        #[serde(
            rename = "altText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alt_text: ::std::option::Option<String>,
        #[doc = "The border style to apply to the image."]
        #[serde(
            rename = "borderStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub border_style: ::std::option::Option<crate::schemas::GoogleAppsCardV1BorderStyle>,
        #[doc = "The crop style to apply to the image."]
        #[serde(
            rename = "cropStyle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crop_style: ::std::option::Option<crate::schemas::GoogleAppsCardV1ImageCropStyle>,
        #[doc = "The image URL."]
        #[serde(
            rename = "imageUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1ImageComponent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1ImageComponent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1ImageCropStyle {
        #[doc = "The aspect ratio to use if the crop type is `RECTANGLE_CUSTOM`."]
        #[serde(
            rename = "aspectRatio",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aspect_ratio: ::std::option::Option<f64>,
        #[doc = "The crop type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::GoogleAppsCardV1ImageCropStyleType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1ImageCropStyle {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1ImageCropStyle {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsCardV1ImageCropStyleType {
        #[doc = "Applies a circular crop."]
        Circle,
        #[doc = "No value specified."]
        ImageCropTypeUnspecified,
        #[doc = "Applies a rectangular crop with a 4:3 aspect ratio."]
        Rectangle43,
        #[doc = "Applies a rectangular crop with a custom aspect ratio."]
        RectangleCustom,
        #[doc = "Applies a square crop."]
        Square,
    }
    impl GoogleAppsCardV1ImageCropStyleType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsCardV1ImageCropStyleType::Circle => "CIRCLE",
                GoogleAppsCardV1ImageCropStyleType::ImageCropTypeUnspecified => {
                    "IMAGE_CROP_TYPE_UNSPECIFIED"
                }
                GoogleAppsCardV1ImageCropStyleType::Rectangle43 => "RECTANGLE_4_3",
                GoogleAppsCardV1ImageCropStyleType::RectangleCustom => "RECTANGLE_CUSTOM",
                GoogleAppsCardV1ImageCropStyleType::Square => "SQUARE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsCardV1ImageCropStyleType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsCardV1ImageCropStyleType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsCardV1ImageCropStyleType, ()> {
            Ok(match s {
                "CIRCLE" => GoogleAppsCardV1ImageCropStyleType::Circle,
                "IMAGE_CROP_TYPE_UNSPECIFIED" => {
                    GoogleAppsCardV1ImageCropStyleType::ImageCropTypeUnspecified
                }
                "RECTANGLE_4_3" => GoogleAppsCardV1ImageCropStyleType::Rectangle43,
                "RECTANGLE_CUSTOM" => GoogleAppsCardV1ImageCropStyleType::RectangleCustom,
                "SQUARE" => GoogleAppsCardV1ImageCropStyleType::Square,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsCardV1ImageCropStyleType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsCardV1ImageCropStyleType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsCardV1ImageCropStyleType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CIRCLE" => GoogleAppsCardV1ImageCropStyleType::Circle,
                "IMAGE_CROP_TYPE_UNSPECIFIED" => {
                    GoogleAppsCardV1ImageCropStyleType::ImageCropTypeUnspecified
                }
                "RECTANGLE_4_3" => GoogleAppsCardV1ImageCropStyleType::Rectangle43,
                "RECTANGLE_CUSTOM" => GoogleAppsCardV1ImageCropStyleType::RectangleCustom,
                "SQUARE" => GoogleAppsCardV1ImageCropStyleType::Square,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1ImageCropStyleType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1ImageCropStyleType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1OnClick {
        #[doc = "If specified, an action is triggered by this `onClick`."]
        #[serde(
            rename = "action",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action: ::std::option::Option<crate::schemas::GoogleAppsCardV1Action>,
        #[doc = "A new card is pushed to the card stack after clicking if specified."]
        #[serde(
            rename = "card",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub card: ::std::option::Option<Box<crate::schemas::GoogleAppsCardV1Card>>,
        #[doc = "An add-on triggers this action when the action needs to open a link. This differs from the `open_link` above in that this needs to talk to server to get the link. Thus some preparation work is required for web client to do before the open link action response comes back."]
        #[serde(
            rename = "openDynamicLinkAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub open_dynamic_link_action: ::std::option::Option<crate::schemas::GoogleAppsCardV1Action>,
        #[doc = "If specified, this `onClick` triggers an open link action."]
        #[serde(
            rename = "openLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub open_link: ::std::option::Option<crate::schemas::GoogleAppsCardV1OpenLink>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1OnClick {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1OnClick {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1OpenLink {
        #[doc = "Whether the client forgets about a link after opening it, or observes it until the window closes. Not supported by Chat apps."]
        #[serde(
            rename = "onClose",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_close: ::std::option::Option<crate::schemas::GoogleAppsCardV1OpenLinkOnClose>,
        #[doc = "How to open a link. Not supported by Chat apps."]
        #[serde(
            rename = "openAs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub open_as: ::std::option::Option<crate::schemas::GoogleAppsCardV1OpenLinkOpenAs>,
        #[doc = "The URL to open."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1OpenLink {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1OpenLink {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsCardV1OpenLinkOnClose {
        #[doc = "Doesn’t reload the card after the child window closes."]
        Nothing,
        #[doc = "Reloads the card after the child window closes. If used in conjunction with [OpenAs.OVERLAY](/workspace/add-ons/reference/rpc/google.apps.card.v1#openas), the child window acts as a modal dialog and the main card is blocked until the child window closes."]
        Reload,
    }
    impl GoogleAppsCardV1OpenLinkOnClose {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsCardV1OpenLinkOnClose::Nothing => "NOTHING",
                GoogleAppsCardV1OpenLinkOnClose::Reload => "RELOAD",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsCardV1OpenLinkOnClose {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsCardV1OpenLinkOnClose {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsCardV1OpenLinkOnClose, ()> {
            Ok(match s {
                "NOTHING" => GoogleAppsCardV1OpenLinkOnClose::Nothing,
                "RELOAD" => GoogleAppsCardV1OpenLinkOnClose::Reload,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsCardV1OpenLinkOnClose {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsCardV1OpenLinkOnClose {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsCardV1OpenLinkOnClose {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NOTHING" => GoogleAppsCardV1OpenLinkOnClose::Nothing,
                "RELOAD" => GoogleAppsCardV1OpenLinkOnClose::Reload,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1OpenLinkOnClose {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1OpenLinkOnClose {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsCardV1OpenLinkOpenAs {
        #[doc = "The link opens as a full size window (if that's the frame used by the client."]
        FullSize,
        #[doc = "The link opens as an overlay, such as a pop-up."]
        Overlay,
    }
    impl GoogleAppsCardV1OpenLinkOpenAs {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsCardV1OpenLinkOpenAs::FullSize => "FULL_SIZE",
                GoogleAppsCardV1OpenLinkOpenAs::Overlay => "OVERLAY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsCardV1OpenLinkOpenAs {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsCardV1OpenLinkOpenAs {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsCardV1OpenLinkOpenAs, ()> {
            Ok(match s {
                "FULL_SIZE" => GoogleAppsCardV1OpenLinkOpenAs::FullSize,
                "OVERLAY" => GoogleAppsCardV1OpenLinkOpenAs::Overlay,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsCardV1OpenLinkOpenAs {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsCardV1OpenLinkOpenAs {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsCardV1OpenLinkOpenAs {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FULL_SIZE" => GoogleAppsCardV1OpenLinkOpenAs::FullSize,
                "OVERLAY" => GoogleAppsCardV1OpenLinkOpenAs::Overlay,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1OpenLinkOpenAs {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1OpenLinkOpenAs {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1Section {
        #[doc = "Indicates whether this section is collapsible. If a section is collapsible, the description must be given."]
        #[serde(
            rename = "collapsible",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub collapsible: ::std::option::Option<bool>,
        #[doc = "The header of the section. Formatted text is supported."]
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<String>,
        #[doc = "The number of uncollapsible widgets. For example, when a section contains five widgets and the `uncollapsibleWidgetsCount` is set to `2`, the first two widgets are always shown and the last three are collapsed as default. The `uncollapsibleWidgetsCount` is taken into account only when `collapsible` is `true`."]
        #[serde(
            rename = "uncollapsibleWidgetsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uncollapsible_widgets_count: ::std::option::Option<i32>,
        #[doc = "A section must contain at least 1 widget."]
        #[serde(
            rename = "widgets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub widgets: ::std::option::Option<Vec<crate::schemas::GoogleAppsCardV1Widget>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1Section {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1Section {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1SelectionInput {
        #[doc = "An array of the selected items."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::GoogleAppsCardV1SelectionItem>>,
        #[doc = "The label displayed ahead of the switch control."]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
        #[doc = "The name of the text input which is used in `formInput`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "If specified, the form is submitted when the selection changes. If not specified, you must specify a separate button."]
        #[serde(
            rename = "onChangeAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_change_action: ::std::option::Option<crate::schemas::GoogleAppsCardV1Action>,
        #[doc = "The type of the selection."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::GoogleAppsCardV1SelectionInputType>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1SelectionInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1SelectionInput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsCardV1SelectionInputType {
        #[doc = "A checkbox."]
        CheckBox,
        #[doc = "A dropdown menu."]
        Dropdown,
        #[doc = "A radio button."]
        RadioButton,
        #[doc = "A switch."]
        Switch,
    }
    impl GoogleAppsCardV1SelectionInputType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsCardV1SelectionInputType::CheckBox => "CHECK_BOX",
                GoogleAppsCardV1SelectionInputType::Dropdown => "DROPDOWN",
                GoogleAppsCardV1SelectionInputType::RadioButton => "RADIO_BUTTON",
                GoogleAppsCardV1SelectionInputType::Switch => "SWITCH",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsCardV1SelectionInputType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsCardV1SelectionInputType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsCardV1SelectionInputType, ()> {
            Ok(match s {
                "CHECK_BOX" => GoogleAppsCardV1SelectionInputType::CheckBox,
                "DROPDOWN" => GoogleAppsCardV1SelectionInputType::Dropdown,
                "RADIO_BUTTON" => GoogleAppsCardV1SelectionInputType::RadioButton,
                "SWITCH" => GoogleAppsCardV1SelectionInputType::Switch,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsCardV1SelectionInputType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsCardV1SelectionInputType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsCardV1SelectionInputType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHECK_BOX" => GoogleAppsCardV1SelectionInputType::CheckBox,
                "DROPDOWN" => GoogleAppsCardV1SelectionInputType::Dropdown,
                "RADIO_BUTTON" => GoogleAppsCardV1SelectionInputType::RadioButton,
                "SWITCH" => GoogleAppsCardV1SelectionInputType::Switch,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1SelectionInputType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1SelectionInputType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1SelectionItem {
        #[doc = "If more than one item is selected for `RADIO_BUTTON` and `DROPDOWN`, the first selected item is treated as selected and the ones after are ignored."]
        #[serde(
            rename = "selected",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selected: ::std::option::Option<bool>,
        #[doc = "The text to be displayed."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
        #[doc = "The value associated with this item. The client should use this as a form input value."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1SelectionItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1SelectionItem {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1SuggestionItem {
        #[doc = "The suggested autocomplete result."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1SuggestionItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1SuggestionItem {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1Suggestions {
        #[doc = "A list of suggestions used for autocomplete recommendations."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::GoogleAppsCardV1SuggestionItem>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1Suggestions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1Suggestions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1SwitchControl {
        #[doc = "The control type, either switch or checkbox."]
        #[serde(
            rename = "controlType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub control_type:
            ::std::option::Option<crate::schemas::GoogleAppsCardV1SwitchControlControlType>,
        #[doc = "The name of the switch widget that's used in `formInput`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The action when the switch state is changed."]
        #[serde(
            rename = "onChangeAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_change_action: ::std::option::Option<crate::schemas::GoogleAppsCardV1Action>,
        #[doc = "If the switch is selected."]
        #[serde(
            rename = "selected",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selected: ::std::option::Option<bool>,
        #[doc = "The value is what is passed back in the callback."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1SwitchControl {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1SwitchControl {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsCardV1SwitchControlControlType {
        #[doc = "A checkbox."]
        CheckBox,
        #[doc = "Deprecated in favor of `CHECK_BOX`."]
        Checkbox,
        #[doc = "A toggle-style switch."]
        Switch,
    }
    impl GoogleAppsCardV1SwitchControlControlType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsCardV1SwitchControlControlType::CheckBox => "CHECK_BOX",
                GoogleAppsCardV1SwitchControlControlType::Checkbox => "CHECKBOX",
                GoogleAppsCardV1SwitchControlControlType::Switch => "SWITCH",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsCardV1SwitchControlControlType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsCardV1SwitchControlControlType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAppsCardV1SwitchControlControlType, ()> {
            Ok(match s {
                "CHECK_BOX" => GoogleAppsCardV1SwitchControlControlType::CheckBox,
                "CHECKBOX" => GoogleAppsCardV1SwitchControlControlType::Checkbox,
                "SWITCH" => GoogleAppsCardV1SwitchControlControlType::Switch,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsCardV1SwitchControlControlType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsCardV1SwitchControlControlType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsCardV1SwitchControlControlType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CHECK_BOX" => GoogleAppsCardV1SwitchControlControlType::CheckBox,
                "CHECKBOX" => GoogleAppsCardV1SwitchControlControlType::Checkbox,
                "SWITCH" => GoogleAppsCardV1SwitchControlControlType::Switch,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1SwitchControlControlType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1SwitchControlControlType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1TextInput {
        #[doc = "The refresh function that returns suggestions based on the user's input text. If the callback is not specified, autocomplete is done in client side based on the initial suggestion items."]
        #[serde(
            rename = "autoCompleteAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_complete_action: ::std::option::Option<crate::schemas::GoogleAppsCardV1Action>,
        #[doc = "The hint text."]
        #[serde(
            rename = "hintText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hint_text: ::std::option::Option<String>,
        #[doc = "The initial suggestions made before any user input."]
        #[serde(
            rename = "initialSuggestions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub initial_suggestions: ::std::option::Option<crate::schemas::GoogleAppsCardV1Suggestions>,
        #[doc = "At least one of label and hintText must be specified."]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
        #[doc = "The name of the text input which is used in `formInput`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The onChange action, for example, invoke a function."]
        #[serde(
            rename = "onChangeAction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_change_action: ::std::option::Option<crate::schemas::GoogleAppsCardV1Action>,
        #[doc = "The style of the text, for example, a single line or multiple lines."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::GoogleAppsCardV1TextInputType>,
        #[doc = "The default value when there is no input from the user."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1TextInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1TextInput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsCardV1TextInputType {
        #[doc = "The text is put into multiple lines."]
        MultipleLine,
        #[doc = "The text is put into a single line."]
        SingleLine,
    }
    impl GoogleAppsCardV1TextInputType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsCardV1TextInputType::MultipleLine => "MULTIPLE_LINE",
                GoogleAppsCardV1TextInputType::SingleLine => "SINGLE_LINE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsCardV1TextInputType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsCardV1TextInputType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleAppsCardV1TextInputType, ()> {
            Ok(match s {
                "MULTIPLE_LINE" => GoogleAppsCardV1TextInputType::MultipleLine,
                "SINGLE_LINE" => GoogleAppsCardV1TextInputType::SingleLine,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsCardV1TextInputType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsCardV1TextInputType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsCardV1TextInputType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MULTIPLE_LINE" => GoogleAppsCardV1TextInputType::MultipleLine,
                "SINGLE_LINE" => GoogleAppsCardV1TextInputType::SingleLine,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1TextInputType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1TextInputType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1TextParagraph {
        #[doc = "The text that's shown in the widget."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1TextParagraph {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1TextParagraph {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAppsCardV1Widget {
        #[doc = "A list of buttons. For example, the following JSON creates two buttons. The first is a filled text button and the second is an image button that opens a link: `\"buttonList\": { \"buttons\": [ \"button\": { \"text\": \"Edit\", \"Color\": { \"Red\": 255 \"Green\": 255 \"Blue\": 255 } \"disabled\": true }, \"button\": { \"icon\": { \"knownIcon\": \"INVITE\" \"altText\": \"check calendar\" }, \"onClick\": { \"openLink\": { \"url\": \"https://example.com/calendar\" } } }, ] }`"]
        #[serde(
            rename = "buttonList",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub button_list: ::std::option::Option<crate::schemas::GoogleAppsCardV1ButtonList>,
        #[doc = "Displays a selection/input widget for date/time. For example, the following JSON creates a date/time picker for an appointment time: `\"date_time_picker\": { \"name\": \"appointment_time\", \"label\": \"Book your appointment at:\", \"type\": \"DateTimePickerType.DATE_AND_TIME\", \"valueMsEpoch\": \"796435200000\" }`"]
        #[serde(
            rename = "dateTimePicker",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_time_picker: ::std::option::Option<crate::schemas::GoogleAppsCardV1DateTimePicker>,
        #[doc = "Displays a decorated text item in this widget. For example, the following JSON creates a decorated text widget showing email address: `\"decoratedText\": { \"icon\": { \"knownIcon\": \"EMAIL\" }, \"topLabel\": \"Email Address\", \"content\": \"sasha@example.com\", \"bottomLabel\": \"This is a new Email address!\", \"switchWidget\": { \"name\": \"has_send_welcome_email_to_sasha\", \"selected\": false, \"controlType\": \"ControlType.CHECKBOX\" } }`"]
        #[serde(
            rename = "decoratedText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub decorated_text: ::std::option::Option<crate::schemas::GoogleAppsCardV1DecoratedText>,
        #[doc = "Displays a divider. For example, the following JSON creates a divider: `\"divider\": { }`"]
        #[serde(
            rename = "divider",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub divider: ::std::option::Option<crate::schemas::GoogleAppsCardV1Divider>,
        #[doc = "Displays a grid with a collection of items. For example, the following JSON creates a 2 column grid with a single item: `\"grid\": { \"title\": \"A fine collection of items\", \"numColumns\": 2, \"borderStyle\": { \"type\": \"STROKE\", \"cornerRadius\": 4.0 }, \"items\": [ \"image\": { \"imageUri\": \"https://www.example.com/image.png\", \"cropStyle\": { \"type\": \"SQUARE\" }, \"borderStyle\": { \"type\": \"STROKE\" } }, \"title\": \"An item\", \"textAlignment\": \"CENTER\" ], \"onClick\": { \"openLink\": { \"url\":\"https://www.example.com\" } } }`"]
        #[serde(
            rename = "grid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub grid: ::std::option::Option<crate::schemas::GoogleAppsCardV1Grid>,
        #[doc = "The horizontal alignment of this widget."]
        #[serde(
            rename = "horizontalAlignment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub horizontal_alignment:
            ::std::option::Option<crate::schemas::GoogleAppsCardV1WidgetHorizontalAlignment>,
        #[doc = "Displays an image in this widget. For example, the following JSON creates an image with alternative text: `\"image\": { \"imageUrl\": \"https://example.com/sasha.png\" \"altText\": \"Avatar for Sasha\" }`"]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::GoogleAppsCardV1Image>,
        #[doc = "Displays a switch control in this widget. For example, the following JSON creates a dropdown selection for size: `\"switchControl\": { \"name\": \"size\", \"label\": \"Size\" \"type\": \"SelectionType.DROPDOWN\", \"items\": [ { \"text\": \"S\", \"value\": \"small\", \"selected\": false }, { \"text\": \"M\", \"value\": \"medium\", \"selected\": true }, { \"text\": \"L\", \"value\": \"large\", \"selected\": false }, { \"text\": \"XL\", \"value\": \"extra_large\", \"selected\": false } ] }`"]
        #[serde(
            rename = "selectionInput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub selection_input: ::std::option::Option<crate::schemas::GoogleAppsCardV1SelectionInput>,
        #[doc = "Displays a text input in this widget. For example, the following JSON creates a text input for mail address: `\"textInput\": { \"name\": \"mailing_address\", \"label\": \"Mailing Address\" }` As another example, the following JSON creates a text input for programming language with static suggestions: `\"textInput\": { \"name\": \"preferred_programing_language\", \"label\": \"Preferred Language\", \"initialSuggestions\": { \"items\": [ { \"text\": \"C++\" }, { \"text\": \"Java\" }, { \"text\": \"JavaScript\" }, { \"text\": \"Python\" } ] } }`"]
        #[serde(
            rename = "textInput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_input: ::std::option::Option<crate::schemas::GoogleAppsCardV1TextInput>,
        #[doc = "Displays a text paragraph in this widget. For example, the following JSON creates a bolded text: `\"textParagraph\": { \"text\": \" *bold text*\" }`"]
        #[serde(
            rename = "textParagraph",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_paragraph: ::std::option::Option<crate::schemas::GoogleAppsCardV1TextParagraph>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1Widget {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1Widget {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAppsCardV1WidgetHorizontalAlignment {
        #[doc = "Alignment to the center position."]
        Center,
        #[doc = "Alignment to the end position."]
        End,
        #[doc = "Unspecified alignment."]
        HorizontalAlignmentUnspecified,
        #[doc = "Alignment to the start position."]
        Start,
    }
    impl GoogleAppsCardV1WidgetHorizontalAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleAppsCardV1WidgetHorizontalAlignment::Center => "CENTER",
                GoogleAppsCardV1WidgetHorizontalAlignment::End => "END",
                GoogleAppsCardV1WidgetHorizontalAlignment::HorizontalAlignmentUnspecified => {
                    "HORIZONTAL_ALIGNMENT_UNSPECIFIED"
                }
                GoogleAppsCardV1WidgetHorizontalAlignment::Start => "START",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAppsCardV1WidgetHorizontalAlignment {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAppsCardV1WidgetHorizontalAlignment {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAppsCardV1WidgetHorizontalAlignment, ()> {
            Ok(match s {
                "CENTER" => GoogleAppsCardV1WidgetHorizontalAlignment::Center,
                "END" => GoogleAppsCardV1WidgetHorizontalAlignment::End,
                "HORIZONTAL_ALIGNMENT_UNSPECIFIED" => {
                    GoogleAppsCardV1WidgetHorizontalAlignment::HorizontalAlignmentUnspecified
                }
                "START" => GoogleAppsCardV1WidgetHorizontalAlignment::Start,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleAppsCardV1WidgetHorizontalAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAppsCardV1WidgetHorizontalAlignment {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleAppsCardV1WidgetHorizontalAlignment {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CENTER" => GoogleAppsCardV1WidgetHorizontalAlignment::Center,
                "END" => GoogleAppsCardV1WidgetHorizontalAlignment::End,
                "HORIZONTAL_ALIGNMENT_UNSPECIFIED" => {
                    GoogleAppsCardV1WidgetHorizontalAlignment::HorizontalAlignmentUnspecified
                }
                "START" => GoogleAppsCardV1WidgetHorizontalAlignment::Start,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleAppsCardV1WidgetHorizontalAlignment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAppsCardV1WidgetHorizontalAlignment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Image {
        #[doc = "The aspect ratio of this image (width/height). This field allows clients to reserve the right height for the image while waiting for it to load. It's not meant to override the native aspect ratio of the image. If unset, the server fills it by prefetching the image."]
        #[serde(
            rename = "aspectRatio",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aspect_ratio: ::std::option::Option<f64>,
        #[doc = "The URL of the image."]
        #[serde(
            rename = "imageUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_url: ::std::option::Option<String>,
        #[doc = "The onclick action."]
        #[serde(
            rename = "onClick",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_click: ::std::option::Option<crate::schemas::OnClick>,
    }
    impl ::google_field_selector::FieldSelector for Image {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Image {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ImageButton {
        #[doc = "The icon specified by an enum that indices to an icon provided by Chat API."]
        #[serde(
            rename = "icon",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon: ::std::option::Option<crate::schemas::ImageButtonIcon>,
        #[doc = "The icon specified by a URL."]
        #[serde(
            rename = "iconUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon_url: ::std::option::Option<String>,
        #[doc = "The name of this image_button which will be used for accessibility. Default value will be provided if developers don't specify."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The onclick action."]
        #[serde(
            rename = "onClick",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_click: ::std::option::Option<crate::schemas::OnClick>,
    }
    impl ::google_field_selector::FieldSelector for ImageButton {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImageButton {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ImageButtonIcon {
        Airplane,
        Bookmark,
        Bus,
        Car,
        Clock,
        ConfirmationNumberIcon,
        Description,
        Dollar,
        Email,
        EventPerformer,
        EventSeat,
        FlightArrival,
        FlightDeparture,
        Hotel,
        HotelRoomType,
        IconUnspecified,
        Invite,
        MapPin,
        Membership,
        MultiplePeople,
        Offer,
        Person,
        Phone,
        RestaurantIcon,
        ShoppingCart,
        Star,
        Store,
        Ticket,
        Train,
        VideoCamera,
        VideoPlay,
    }
    impl ImageButtonIcon {
        pub fn as_str(self) -> &'static str {
            match self {
                ImageButtonIcon::Airplane => "AIRPLANE",
                ImageButtonIcon::Bookmark => "BOOKMARK",
                ImageButtonIcon::Bus => "BUS",
                ImageButtonIcon::Car => "CAR",
                ImageButtonIcon::Clock => "CLOCK",
                ImageButtonIcon::ConfirmationNumberIcon => "CONFIRMATION_NUMBER_ICON",
                ImageButtonIcon::Description => "DESCRIPTION",
                ImageButtonIcon::Dollar => "DOLLAR",
                ImageButtonIcon::Email => "EMAIL",
                ImageButtonIcon::EventPerformer => "EVENT_PERFORMER",
                ImageButtonIcon::EventSeat => "EVENT_SEAT",
                ImageButtonIcon::FlightArrival => "FLIGHT_ARRIVAL",
                ImageButtonIcon::FlightDeparture => "FLIGHT_DEPARTURE",
                ImageButtonIcon::Hotel => "HOTEL",
                ImageButtonIcon::HotelRoomType => "HOTEL_ROOM_TYPE",
                ImageButtonIcon::IconUnspecified => "ICON_UNSPECIFIED",
                ImageButtonIcon::Invite => "INVITE",
                ImageButtonIcon::MapPin => "MAP_PIN",
                ImageButtonIcon::Membership => "MEMBERSHIP",
                ImageButtonIcon::MultiplePeople => "MULTIPLE_PEOPLE",
                ImageButtonIcon::Offer => "OFFER",
                ImageButtonIcon::Person => "PERSON",
                ImageButtonIcon::Phone => "PHONE",
                ImageButtonIcon::RestaurantIcon => "RESTAURANT_ICON",
                ImageButtonIcon::ShoppingCart => "SHOPPING_CART",
                ImageButtonIcon::Star => "STAR",
                ImageButtonIcon::Store => "STORE",
                ImageButtonIcon::Ticket => "TICKET",
                ImageButtonIcon::Train => "TRAIN",
                ImageButtonIcon::VideoCamera => "VIDEO_CAMERA",
                ImageButtonIcon::VideoPlay => "VIDEO_PLAY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ImageButtonIcon {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ImageButtonIcon {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ImageButtonIcon, ()> {
            Ok(match s {
                "AIRPLANE" => ImageButtonIcon::Airplane,
                "BOOKMARK" => ImageButtonIcon::Bookmark,
                "BUS" => ImageButtonIcon::Bus,
                "CAR" => ImageButtonIcon::Car,
                "CLOCK" => ImageButtonIcon::Clock,
                "CONFIRMATION_NUMBER_ICON" => ImageButtonIcon::ConfirmationNumberIcon,
                "DESCRIPTION" => ImageButtonIcon::Description,
                "DOLLAR" => ImageButtonIcon::Dollar,
                "EMAIL" => ImageButtonIcon::Email,
                "EVENT_PERFORMER" => ImageButtonIcon::EventPerformer,
                "EVENT_SEAT" => ImageButtonIcon::EventSeat,
                "FLIGHT_ARRIVAL" => ImageButtonIcon::FlightArrival,
                "FLIGHT_DEPARTURE" => ImageButtonIcon::FlightDeparture,
                "HOTEL" => ImageButtonIcon::Hotel,
                "HOTEL_ROOM_TYPE" => ImageButtonIcon::HotelRoomType,
                "ICON_UNSPECIFIED" => ImageButtonIcon::IconUnspecified,
                "INVITE" => ImageButtonIcon::Invite,
                "MAP_PIN" => ImageButtonIcon::MapPin,
                "MEMBERSHIP" => ImageButtonIcon::Membership,
                "MULTIPLE_PEOPLE" => ImageButtonIcon::MultiplePeople,
                "OFFER" => ImageButtonIcon::Offer,
                "PERSON" => ImageButtonIcon::Person,
                "PHONE" => ImageButtonIcon::Phone,
                "RESTAURANT_ICON" => ImageButtonIcon::RestaurantIcon,
                "SHOPPING_CART" => ImageButtonIcon::ShoppingCart,
                "STAR" => ImageButtonIcon::Star,
                "STORE" => ImageButtonIcon::Store,
                "TICKET" => ImageButtonIcon::Ticket,
                "TRAIN" => ImageButtonIcon::Train,
                "VIDEO_CAMERA" => ImageButtonIcon::VideoCamera,
                "VIDEO_PLAY" => ImageButtonIcon::VideoPlay,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ImageButtonIcon {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ImageButtonIcon {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ImageButtonIcon {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AIRPLANE" => ImageButtonIcon::Airplane,
                "BOOKMARK" => ImageButtonIcon::Bookmark,
                "BUS" => ImageButtonIcon::Bus,
                "CAR" => ImageButtonIcon::Car,
                "CLOCK" => ImageButtonIcon::Clock,
                "CONFIRMATION_NUMBER_ICON" => ImageButtonIcon::ConfirmationNumberIcon,
                "DESCRIPTION" => ImageButtonIcon::Description,
                "DOLLAR" => ImageButtonIcon::Dollar,
                "EMAIL" => ImageButtonIcon::Email,
                "EVENT_PERFORMER" => ImageButtonIcon::EventPerformer,
                "EVENT_SEAT" => ImageButtonIcon::EventSeat,
                "FLIGHT_ARRIVAL" => ImageButtonIcon::FlightArrival,
                "FLIGHT_DEPARTURE" => ImageButtonIcon::FlightDeparture,
                "HOTEL" => ImageButtonIcon::Hotel,
                "HOTEL_ROOM_TYPE" => ImageButtonIcon::HotelRoomType,
                "ICON_UNSPECIFIED" => ImageButtonIcon::IconUnspecified,
                "INVITE" => ImageButtonIcon::Invite,
                "MAP_PIN" => ImageButtonIcon::MapPin,
                "MEMBERSHIP" => ImageButtonIcon::Membership,
                "MULTIPLE_PEOPLE" => ImageButtonIcon::MultiplePeople,
                "OFFER" => ImageButtonIcon::Offer,
                "PERSON" => ImageButtonIcon::Person,
                "PHONE" => ImageButtonIcon::Phone,
                "RESTAURANT_ICON" => ImageButtonIcon::RestaurantIcon,
                "SHOPPING_CART" => ImageButtonIcon::ShoppingCart,
                "STAR" => ImageButtonIcon::Star,
                "STORE" => ImageButtonIcon::Store,
                "TICKET" => ImageButtonIcon::Ticket,
                "TRAIN" => ImageButtonIcon::Train,
                "VIDEO_CAMERA" => ImageButtonIcon::VideoCamera,
                "VIDEO_PLAY" => ImageButtonIcon::VideoPlay,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ImageButtonIcon {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImageButtonIcon {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Inputs {
        #[doc = "Date input values. Not supported by Chat apps."]
        #[serde(
            rename = "dateInput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_input: ::std::option::Option<crate::schemas::DateInput>,
        #[doc = "Date and time input values. Not supported by Chat apps."]
        #[serde(
            rename = "dateTimeInput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_time_input: ::std::option::Option<crate::schemas::DateTimeInput>,
        #[doc = "Input parameter for regular widgets. For single-valued widgets, it is a single value list. For multi-valued widgets, such as checkbox, all the values are presented."]
        #[serde(
            rename = "stringInputs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_inputs: ::std::option::Option<crate::schemas::StringInputs>,
        #[doc = "Time input values. Not supported by Chat apps."]
        #[serde(
            rename = "timeInput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_input: ::std::option::Option<crate::schemas::TimeInput>,
    }
    impl ::google_field_selector::FieldSelector for Inputs {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Inputs {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct KeyValue {
        #[doc = "The text of the bottom label. Formatted text supported."]
        #[serde(
            rename = "bottomLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bottom_label: ::std::option::Option<String>,
        #[doc = "A button that can be clicked to trigger an action."]
        #[serde(
            rename = "button",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub button: ::std::option::Option<crate::schemas::Button>,
        #[doc = "The text of the content. Formatted text supported and always required."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<String>,
        #[doc = "If the content should be multiline."]
        #[serde(
            rename = "contentMultiline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_multiline: ::std::option::Option<bool>,
        #[doc = "An enum value that will be replaced by the Chat API with the corresponding icon image."]
        #[serde(
            rename = "icon",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon: ::std::option::Option<crate::schemas::KeyValueIcon>,
        #[doc = "The icon specified by a URL."]
        #[serde(
            rename = "iconUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon_url: ::std::option::Option<String>,
        #[doc = "The onclick action. Only the top label, bottom label and content region are clickable."]
        #[serde(
            rename = "onClick",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_click: ::std::option::Option<crate::schemas::OnClick>,
        #[doc = "The text of the top label. Formatted text supported."]
        #[serde(
            rename = "topLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top_label: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for KeyValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KeyValue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum KeyValueIcon {
        Airplane,
        Bookmark,
        Bus,
        Car,
        Clock,
        ConfirmationNumberIcon,
        Description,
        Dollar,
        Email,
        EventPerformer,
        EventSeat,
        FlightArrival,
        FlightDeparture,
        Hotel,
        HotelRoomType,
        IconUnspecified,
        Invite,
        MapPin,
        Membership,
        MultiplePeople,
        Offer,
        Person,
        Phone,
        RestaurantIcon,
        ShoppingCart,
        Star,
        Store,
        Ticket,
        Train,
        VideoCamera,
        VideoPlay,
    }
    impl KeyValueIcon {
        pub fn as_str(self) -> &'static str {
            match self {
                KeyValueIcon::Airplane => "AIRPLANE",
                KeyValueIcon::Bookmark => "BOOKMARK",
                KeyValueIcon::Bus => "BUS",
                KeyValueIcon::Car => "CAR",
                KeyValueIcon::Clock => "CLOCK",
                KeyValueIcon::ConfirmationNumberIcon => "CONFIRMATION_NUMBER_ICON",
                KeyValueIcon::Description => "DESCRIPTION",
                KeyValueIcon::Dollar => "DOLLAR",
                KeyValueIcon::Email => "EMAIL",
                KeyValueIcon::EventPerformer => "EVENT_PERFORMER",
                KeyValueIcon::EventSeat => "EVENT_SEAT",
                KeyValueIcon::FlightArrival => "FLIGHT_ARRIVAL",
                KeyValueIcon::FlightDeparture => "FLIGHT_DEPARTURE",
                KeyValueIcon::Hotel => "HOTEL",
                KeyValueIcon::HotelRoomType => "HOTEL_ROOM_TYPE",
                KeyValueIcon::IconUnspecified => "ICON_UNSPECIFIED",
                KeyValueIcon::Invite => "INVITE",
                KeyValueIcon::MapPin => "MAP_PIN",
                KeyValueIcon::Membership => "MEMBERSHIP",
                KeyValueIcon::MultiplePeople => "MULTIPLE_PEOPLE",
                KeyValueIcon::Offer => "OFFER",
                KeyValueIcon::Person => "PERSON",
                KeyValueIcon::Phone => "PHONE",
                KeyValueIcon::RestaurantIcon => "RESTAURANT_ICON",
                KeyValueIcon::ShoppingCart => "SHOPPING_CART",
                KeyValueIcon::Star => "STAR",
                KeyValueIcon::Store => "STORE",
                KeyValueIcon::Ticket => "TICKET",
                KeyValueIcon::Train => "TRAIN",
                KeyValueIcon::VideoCamera => "VIDEO_CAMERA",
                KeyValueIcon::VideoPlay => "VIDEO_PLAY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for KeyValueIcon {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for KeyValueIcon {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<KeyValueIcon, ()> {
            Ok(match s {
                "AIRPLANE" => KeyValueIcon::Airplane,
                "BOOKMARK" => KeyValueIcon::Bookmark,
                "BUS" => KeyValueIcon::Bus,
                "CAR" => KeyValueIcon::Car,
                "CLOCK" => KeyValueIcon::Clock,
                "CONFIRMATION_NUMBER_ICON" => KeyValueIcon::ConfirmationNumberIcon,
                "DESCRIPTION" => KeyValueIcon::Description,
                "DOLLAR" => KeyValueIcon::Dollar,
                "EMAIL" => KeyValueIcon::Email,
                "EVENT_PERFORMER" => KeyValueIcon::EventPerformer,
                "EVENT_SEAT" => KeyValueIcon::EventSeat,
                "FLIGHT_ARRIVAL" => KeyValueIcon::FlightArrival,
                "FLIGHT_DEPARTURE" => KeyValueIcon::FlightDeparture,
                "HOTEL" => KeyValueIcon::Hotel,
                "HOTEL_ROOM_TYPE" => KeyValueIcon::HotelRoomType,
                "ICON_UNSPECIFIED" => KeyValueIcon::IconUnspecified,
                "INVITE" => KeyValueIcon::Invite,
                "MAP_PIN" => KeyValueIcon::MapPin,
                "MEMBERSHIP" => KeyValueIcon::Membership,
                "MULTIPLE_PEOPLE" => KeyValueIcon::MultiplePeople,
                "OFFER" => KeyValueIcon::Offer,
                "PERSON" => KeyValueIcon::Person,
                "PHONE" => KeyValueIcon::Phone,
                "RESTAURANT_ICON" => KeyValueIcon::RestaurantIcon,
                "SHOPPING_CART" => KeyValueIcon::ShoppingCart,
                "STAR" => KeyValueIcon::Star,
                "STORE" => KeyValueIcon::Store,
                "TICKET" => KeyValueIcon::Ticket,
                "TRAIN" => KeyValueIcon::Train,
                "VIDEO_CAMERA" => KeyValueIcon::VideoCamera,
                "VIDEO_PLAY" => KeyValueIcon::VideoPlay,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for KeyValueIcon {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for KeyValueIcon {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for KeyValueIcon {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AIRPLANE" => KeyValueIcon::Airplane,
                "BOOKMARK" => KeyValueIcon::Bookmark,
                "BUS" => KeyValueIcon::Bus,
                "CAR" => KeyValueIcon::Car,
                "CLOCK" => KeyValueIcon::Clock,
                "CONFIRMATION_NUMBER_ICON" => KeyValueIcon::ConfirmationNumberIcon,
                "DESCRIPTION" => KeyValueIcon::Description,
                "DOLLAR" => KeyValueIcon::Dollar,
                "EMAIL" => KeyValueIcon::Email,
                "EVENT_PERFORMER" => KeyValueIcon::EventPerformer,
                "EVENT_SEAT" => KeyValueIcon::EventSeat,
                "FLIGHT_ARRIVAL" => KeyValueIcon::FlightArrival,
                "FLIGHT_DEPARTURE" => KeyValueIcon::FlightDeparture,
                "HOTEL" => KeyValueIcon::Hotel,
                "HOTEL_ROOM_TYPE" => KeyValueIcon::HotelRoomType,
                "ICON_UNSPECIFIED" => KeyValueIcon::IconUnspecified,
                "INVITE" => KeyValueIcon::Invite,
                "MAP_PIN" => KeyValueIcon::MapPin,
                "MEMBERSHIP" => KeyValueIcon::Membership,
                "MULTIPLE_PEOPLE" => KeyValueIcon::MultiplePeople,
                "OFFER" => KeyValueIcon::Offer,
                "PERSON" => KeyValueIcon::Person,
                "PHONE" => KeyValueIcon::Phone,
                "RESTAURANT_ICON" => KeyValueIcon::RestaurantIcon,
                "SHOPPING_CART" => KeyValueIcon::ShoppingCart,
                "STAR" => KeyValueIcon::Star,
                "STORE" => KeyValueIcon::Store,
                "TICKET" => KeyValueIcon::Ticket,
                "TRAIN" => KeyValueIcon::Train,
                "VIDEO_CAMERA" => KeyValueIcon::VideoCamera,
                "VIDEO_PLAY" => KeyValueIcon::VideoPlay,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for KeyValueIcon {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for KeyValueIcon {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListMembershipsResponse {
        #[doc = "List of memberships in the requested (or first) page."]
        #[serde(
            rename = "memberships",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub memberships: ::std::option::Option<Vec<crate::schemas::Membership>>,
        #[doc = "Continuation token to retrieve the next page of results. It will be empty for the last page of results."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListMembershipsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListMembershipsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListMembershipsResponse {
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
    pub struct ListSpacesResponse {
        #[doc = "Continuation token to retrieve the next page of results. It will be empty for the last page of results. Tokens expire in an hour. An error is thrown if an expired token is passed."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of spaces in the requested (or first) page."]
        #[serde(
            rename = "spaces",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spaces: ::std::option::Option<Vec<crate::schemas::Space>>,
    }
    impl ::google_field_selector::FieldSelector for ListSpacesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListSpacesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListSpacesResponse {
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
    pub struct MatchedUrl {
        #[doc = "Output only. The url that was matched."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MatchedUrl {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MatchedUrl {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Media {
        #[doc = "Name of the media resource."]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Media {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Media {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Membership {
        #[doc = "Output only. The creation time of the membership a.k.a. the time at which the member joined the space, if applicable."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "A Google Chat user or app. Format: `users/{person}` or `users/app` When `users/{person}`, represents a [person](https://developers.google.com/people/api/rest/v1/people) in the People API or a [user](https://developers.google.com/admin-sdk/directory/reference/rest/v1/users) in the Admin SDK Directory API. Format: `users/{user}` When `users/app`, represents a Chat app creating membership for itself. Creating membership is available as a [developer preview](https://developers.google.com/workspace/preview)."]
        #[serde(
            rename = "member",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub member: ::std::option::Option<crate::schemas::User>,
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. State of the membership."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::MembershipState>,
    }
    impl ::google_field_selector::FieldSelector for Membership {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Membership {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MembershipState {
        #[doc = "The user has been invited, is able to join the space, but currently has not joined."]
        Invited,
        #[doc = "The user has joined the space."]
        Joined,
        #[doc = "Default, do not use."]
        MembershipStateUnspecified,
        #[doc = "The user is not a member of the space, has not been invited and is not able to join the space."]
        NotAMember,
    }
    impl MembershipState {
        pub fn as_str(self) -> &'static str {
            match self {
                MembershipState::Invited => "INVITED",
                MembershipState::Joined => "JOINED",
                MembershipState::MembershipStateUnspecified => "MEMBERSHIP_STATE_UNSPECIFIED",
                MembershipState::NotAMember => "NOT_A_MEMBER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MembershipState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MembershipState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MembershipState, ()> {
            Ok(match s {
                "INVITED" => MembershipState::Invited,
                "JOINED" => MembershipState::Joined,
                "MEMBERSHIP_STATE_UNSPECIFIED" => MembershipState::MembershipStateUnspecified,
                "NOT_A_MEMBER" => MembershipState::NotAMember,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MembershipState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MembershipState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MembershipState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INVITED" => MembershipState::Invited,
                "JOINED" => MembershipState::Joined,
                "MEMBERSHIP_STATE_UNSPECIFIED" => MembershipState::MembershipStateUnspecified,
                "NOT_A_MEMBER" => MembershipState::NotAMember,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MembershipState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MembershipState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Message {
        #[doc = "Input only. Parameters that a Chat app can use to configure how its response is posted."]
        #[serde(
            rename = "actionResponse",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action_response: ::std::option::Option<crate::schemas::ActionResponse>,
        #[doc = "Output only. Annotations associated with the text in this message."]
        #[serde(
            rename = "annotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotations: ::std::option::Option<Vec<crate::schemas::Annotation>>,
        #[doc = "Plain-text body of the message with all Chat app mentions stripped out."]
        #[serde(
            rename = "argumentText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub argument_text: ::std::option::Option<String>,
        #[doc = "User uploaded attachment."]
        #[serde(
            rename = "attachment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attachment: ::std::option::Option<Vec<crate::schemas::Attachment>>,
        #[doc = "Rich, formatted and interactive cards that can be used to display UI elements such as: formatted texts, buttons, clickable images. Cards are normally displayed below the plain-text body of the message."]
        #[serde(
            rename = "cards",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cards: ::std::option::Option<Vec<crate::schemas::Card>>,
        #[doc = "Output only. The time at which the message was created in Google Chat server."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "A plain-text description of the message's cards, used when the actual cards cannot be displayed (e.g. mobile notifications)."]
        #[serde(
            rename = "fallbackText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fallback_text: ::std::option::Option<String>,
        #[doc = "Output only. The time at which the message was last updated in Google Chat server. If the message was never updated, this field will be same as create_time."]
        #[serde(
            rename = "lastUpdateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_update_time: ::std::option::Option<String>,
        #[doc = "Output only. A URL in `spaces.messages.text` that matches a link unfurling pattern. For more information, refer to [Unfurl links](https://developers.google.com/chat/how-tos/link-unfurling)."]
        #[serde(
            rename = "matchedUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub matched_url: ::std::option::Option<crate::schemas::MatchedUrl>,
        #[doc = "Resource name in the form `spaces/*/messages/*`. Example: `spaces/AAAAAAAAAAA/messages/BBBBBBBBBBB.BBBBBBBBBBB`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. The user who created the message."]
        #[serde(
            rename = "sender",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sender: ::std::option::Option<crate::schemas::User>,
        #[doc = "Output only. Slash command information, if applicable."]
        #[serde(
            rename = "slashCommand",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub slash_command: ::std::option::Option<crate::schemas::SlashCommand>,
        #[doc = "The space the message belongs to."]
        #[serde(
            rename = "space",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub space: ::std::option::Option<crate::schemas::Space>,
        #[doc = "Plain-text body of the message. The first link to an image, video, web page, or other preview-able item generates a preview chip."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
        #[doc = "The thread the message belongs to."]
        #[serde(
            rename = "thread",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thread: ::std::option::Option<crate::schemas::Thread>,
    }
    impl ::google_field_selector::FieldSelector for Message {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Message {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OnClick {
        #[doc = "A form action will be triggered by this onclick if specified."]
        #[serde(
            rename = "action",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action: ::std::option::Option<crate::schemas::FormAction>,
        #[doc = "This onclick triggers an open link action if specified."]
        #[serde(
            rename = "openLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub open_link: ::std::option::Option<crate::schemas::OpenLink>,
    }
    impl ::google_field_selector::FieldSelector for OnClick {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OnClick {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct OpenLink {
        #[doc = "The URL to open."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for OpenLink {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OpenLink {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Section {
        #[doc = "The header of the section, text formatted supported."]
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<String>,
        #[doc = "A section must contain at least 1 widget."]
        #[serde(
            rename = "widgets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub widgets: ::std::option::Option<Vec<crate::schemas::WidgetMarkup>>,
    }
    impl ::google_field_selector::FieldSelector for Section {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Section {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SlashCommand {
        #[doc = "The id of the slash command invoked."]
        #[serde(
            rename = "commandId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub command_id: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for SlashCommand {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlashCommand {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SlashCommandMetadata {
        #[doc = "The Chat app whose command was invoked."]
        #[serde(
            rename = "bot",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bot: ::std::option::Option<crate::schemas::User>,
        #[doc = "The command id of the invoked slash command."]
        #[serde(
            rename = "commandId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub command_id: ::std::option::Option<i64>,
        #[doc = "The name of the invoked slash command."]
        #[serde(
            rename = "commandName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub command_name: ::std::option::Option<String>,
        #[doc = "The type of slash command."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::SlashCommandMetadataType>,
        #[doc = "Indicating whether the slash command is for a dialog."]
        #[serde(
            rename = "triggersDialog",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub triggers_dialog: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for SlashCommandMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlashCommandMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SlashCommandMetadataType {
        #[doc = "Add Chat app to space."]
        Add,
        #[doc = "Invoke slash command in space."]
        Invoke,
        #[doc = "Default value for the enum. DO NOT USE."]
        TypeUnspecified,
    }
    impl SlashCommandMetadataType {
        pub fn as_str(self) -> &'static str {
            match self {
                SlashCommandMetadataType::Add => "ADD",
                SlashCommandMetadataType::Invoke => "INVOKE",
                SlashCommandMetadataType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SlashCommandMetadataType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SlashCommandMetadataType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SlashCommandMetadataType, ()> {
            Ok(match s {
                "ADD" => SlashCommandMetadataType::Add,
                "INVOKE" => SlashCommandMetadataType::Invoke,
                "TYPE_UNSPECIFIED" => SlashCommandMetadataType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SlashCommandMetadataType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SlashCommandMetadataType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SlashCommandMetadataType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADD" => SlashCommandMetadataType::Add,
                "INVOKE" => SlashCommandMetadataType::Invoke,
                "TYPE_UNSPECIFIED" => SlashCommandMetadataType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SlashCommandMetadataType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SlashCommandMetadataType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Space {
        #[doc = "The space's display name. For direct messages between humans, this field might be empty."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Resource name of the space, in the form \"spaces/*\". Example: spaces/AAAAAAAAAAAA"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Deprecated: Use `single_user_bot_dm` instead. Output only. The type of a space."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::SpaceType>,
        #[doc = "Output only. Whether the space is a DM between a Chat app and a single human."]
        #[serde(
            rename = "singleUserBotDm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub single_user_bot_dm: ::std::option::Option<bool>,
        #[doc = "Output only. Output only. Whether the messages are threaded in this space."]
        #[serde(
            rename = "threaded",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threaded: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for Space {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Space {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SpaceType {
        #[doc = "1:1 Direct Message between a human and a Chat app, where all messages are flat. Note that this does not include direct messages between two humans."]
        Dm,
        #[doc = "Conversations between two or more humans."]
        Room,
        TypeUnspecified,
    }
    impl SpaceType {
        pub fn as_str(self) -> &'static str {
            match self {
                SpaceType::Dm => "DM",
                SpaceType::Room => "ROOM",
                SpaceType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SpaceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SpaceType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SpaceType, ()> {
            Ok(match s {
                "DM" => SpaceType::Dm,
                "ROOM" => SpaceType::Room,
                "TYPE_UNSPECIFIED" => SpaceType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SpaceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SpaceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SpaceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DM" => SpaceType::Dm,
                "ROOM" => SpaceType::Room,
                "TYPE_UNSPECIFIED" => SpaceType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SpaceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpaceType {
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
    pub struct StringInputs {
        #[doc = "An array of strings entered by the user."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for StringInputs {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StringInputs {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TextButton {
        #[doc = "The onclick action of the button."]
        #[serde(
            rename = "onClick",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub on_click: ::std::option::Option<crate::schemas::OnClick>,
        #[doc = "The text of the button."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TextButton {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextButton {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TextParagraph {
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TextParagraph {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextParagraph {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Thread {
        #[doc = "Resource name, in the form \"spaces/*/threads/*\". Example: spaces/AAAAAAAAAAA/threads/TTTTTTTTTTT"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Thread {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Thread {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TimeInput {
        #[doc = "The hour on a 24-hour clock."]
        #[serde(
            rename = "hours",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hours: ::std::option::Option<i32>,
        #[doc = "The number of minutes past the hour. Valid values are 0 to 59."]
        #[serde(
            rename = "minutes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minutes: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for TimeInput {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimeInput {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TimeZone {
        #[doc = "The [IANA TZ](https://www.iana.org/time-zones) time zone database code, such as \"America/Toronto\"."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The user timezone offset, in milliseconds, from Coordinated Universal Time (UTC)."]
        #[serde(
            rename = "offset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offset: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for TimeZone {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimeZone {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct User {
        #[doc = "Output only. The user's display name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Unique identifier of the user's Google Workspace domain."]
        #[serde(
            rename = "domainId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain_id: ::std::option::Option<String>,
        #[doc = "Output only. When `true`, the user is deleted or their profile is not visible."]
        #[serde(
            rename = "isAnonymous",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_anonymous: ::std::option::Option<bool>,
        #[doc = "Resource name for a Google Chat user. Represents a [person](https://developers.google.com/people/api/rest/v1/people#Person) in the People API or a [user](https://developers.google.com/admin-sdk/directory/reference/rest/v1/users) in the Admin SDK Directory API. Formatted as: `users/{user}`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "User type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::UserType>,
    }
    impl ::google_field_selector::FieldSelector for User {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for User {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UserType {
        #[doc = "Chat app user."]
        Bot,
        #[doc = "Human user."]
        Human,
        #[doc = "Default value for the enum. DO NOT USE."]
        TypeUnspecified,
    }
    impl UserType {
        pub fn as_str(self) -> &'static str {
            match self {
                UserType::Bot => "BOT",
                UserType::Human => "HUMAN",
                UserType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for UserType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for UserType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<UserType, ()> {
            Ok(match s {
                "BOT" => UserType::Bot,
                "HUMAN" => UserType::Human,
                "TYPE_UNSPECIFIED" => UserType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for UserType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UserType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UserType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOT" => UserType::Bot,
                "HUMAN" => UserType::Human,
                "TYPE_UNSPECIFIED" => UserType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for UserType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UserMentionMetadata {
        #[doc = "The type of user mention."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::UserMentionMetadataType>,
        #[doc = "The user mentioned."]
        #[serde(
            rename = "user",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user: ::std::option::Option<crate::schemas::User>,
    }
    impl ::google_field_selector::FieldSelector for UserMentionMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserMentionMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UserMentionMetadataType {
        #[doc = "Add user to space."]
        Add,
        #[doc = "Mention user in space."]
        Mention,
        #[doc = "Default value for the enum. DO NOT USE."]
        TypeUnspecified,
    }
    impl UserMentionMetadataType {
        pub fn as_str(self) -> &'static str {
            match self {
                UserMentionMetadataType::Add => "ADD",
                UserMentionMetadataType::Mention => "MENTION",
                UserMentionMetadataType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for UserMentionMetadataType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for UserMentionMetadataType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<UserMentionMetadataType, ()> {
            Ok(match s {
                "ADD" => UserMentionMetadataType::Add,
                "MENTION" => UserMentionMetadataType::Mention,
                "TYPE_UNSPECIFIED" => UserMentionMetadataType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for UserMentionMetadataType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UserMentionMetadataType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UserMentionMetadataType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADD" => UserMentionMetadataType::Add,
                "MENTION" => UserMentionMetadataType::Mention,
                "TYPE_UNSPECIFIED" => UserMentionMetadataType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for UserMentionMetadataType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserMentionMetadataType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct WidgetMarkup {
        #[doc = "A list of buttons. Buttons is also oneof data and only one of these fields should be set."]
        #[serde(
            rename = "buttons",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buttons: ::std::option::Option<Vec<crate::schemas::Button>>,
        #[doc = "Display an image in this widget."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::Image>,
        #[doc = "Display a key value item in this widget."]
        #[serde(
            rename = "keyValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key_value: ::std::option::Option<crate::schemas::KeyValue>,
        #[doc = "Display a text paragraph in this widget."]
        #[serde(
            rename = "textParagraph",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_paragraph: ::std::option::Option<crate::schemas::TextParagraph>,
    }
    impl ::google_field_selector::FieldSelector for WidgetMarkup {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WidgetMarkup {
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
    #[doc = "Actions that can be performed on the dms resource"]
    pub fn dms(&self) -> crate::resources::dms::DmsActions {
        crate::resources::dms::DmsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the media resource"]
    pub fn media(&self) -> crate::resources::media::MediaActions {
        crate::resources::media::MediaActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the rooms resource"]
    pub fn rooms(&self) -> crate::resources::rooms::RoomsActions {
        crate::resources::rooms::RoomsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the spaces resource"]
    pub fn spaces(&self) -> crate::resources::spaces::SpacesActions {
        crate::resources::spaces::SpacesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod dms {
        pub mod params {}
        pub struct DmsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> DmsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Legacy path for creating message. Calling these will result in a BadRequest response."]
            pub fn messages(
                &self,
                request: crate::schemas::Message,
                parent: impl Into<String>,
            ) -> MessagesRequestBuilder {
                MessagesRequestBuilder {
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
                    request_id: None,
                    thread_key: None,
                }
            }
            #[doc = "Legacy path for creating message. Calling these will result in a BadRequest response."]
            pub fn webhooks(
                &self,
                request: crate::schemas::Message,
                parent: impl Into<String>,
            ) -> WebhooksRequestBuilder {
                WebhooksRequestBuilder {
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
                    request_id: None,
                    thread_key: None,
                }
            }
            #[doc = "Actions that can be performed on the conversations resource"]
            pub fn conversations(
                &self,
            ) -> crate::resources::dms::conversations::ConversationsActions {
                crate::resources::dms::conversations::ConversationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [DmsActions::messages()](struct.DmsActions.html#method.messages)"]
        #[derive(Debug, Clone)]
        pub struct MessagesRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Message,
            parent: String,
            request_id: ::std::option::Option<String>,
            thread_key: ::std::option::Option<String>,
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
        impl<'a> MessagesRequestBuilder<'a> {
            #[doc = "Optional. A unique request ID for this message. Specifying an existing request ID returns the message created with that ID instead of creating a new message."]
            pub fn request_id(mut self, value: impl Into<String>) -> Self {
                self.request_id = Some(value.into());
                self
            }
            #[doc = "Optional. Opaque thread identifier. To start or add to a thread, create a message and specify a `threadKey` instead of thread.name. (Setting thread.name has no effect.) The first message with a given `threadKey` starts a new thread. Subsequent messages with the same `threadKey` post into the same thread."]
            pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                self.thread_key = Some(value.into());
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
            ) -> Result<crate::schemas::Message, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Message, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/messages");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("requestId", &self.request_id)]);
                req = req.query(&[("threadKey", &self.thread_key)]);
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
        #[doc = "Created via [DmsActions::webhooks()](struct.DmsActions.html#method.webhooks)"]
        #[derive(Debug, Clone)]
        pub struct WebhooksRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Message,
            parent: String,
            request_id: ::std::option::Option<String>,
            thread_key: ::std::option::Option<String>,
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
        impl<'a> WebhooksRequestBuilder<'a> {
            #[doc = "Optional. A unique request ID for this message. Specifying an existing request ID returns the message created with that ID instead of creating a new message."]
            pub fn request_id(mut self, value: impl Into<String>) -> Self {
                self.request_id = Some(value.into());
                self
            }
            #[doc = "Optional. Opaque thread identifier. To start or add to a thread, create a message and specify a `threadKey` instead of thread.name. (Setting thread.name has no effect.) The first message with a given `threadKey` starts a new thread. Subsequent messages with the same `threadKey` post into the same thread."]
            pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                self.thread_key = Some(value.into());
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
            ) -> Result<crate::schemas::Message, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Message, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/webhooks");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("requestId", &self.request_id)]);
                req = req.query(&[("threadKey", &self.thread_key)]);
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
        pub mod conversations {
            pub mod params {}
            pub struct ConversationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ConversationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Legacy path for creating message. Calling these will result in a BadRequest response."]
                pub fn messages(
                    &self,
                    request: crate::schemas::Message,
                    parent: impl Into<String>,
                ) -> MessagesRequestBuilder {
                    MessagesRequestBuilder {
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
                        request_id: None,
                        thread_key: None,
                    }
                }
            }
            #[doc = "Created via [ConversationsActions::messages()](struct.ConversationsActions.html#method.messages)"]
            #[derive(Debug, Clone)]
            pub struct MessagesRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Message,
                parent: String,
                request_id: ::std::option::Option<String>,
                thread_key: ::std::option::Option<String>,
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
            impl<'a> MessagesRequestBuilder<'a> {
                #[doc = "Optional. A unique request ID for this message. Specifying an existing request ID returns the message created with that ID instead of creating a new message."]
                pub fn request_id(mut self, value: impl Into<String>) -> Self {
                    self.request_id = Some(value.into());
                    self
                }
                #[doc = "Optional. Opaque thread identifier. To start or add to a thread, create a message and specify a `threadKey` instead of thread.name. (Setting thread.name has no effect.) The first message with a given `threadKey` starts a new thread. Subsequent messages with the same `threadKey` post into the same thread."]
                pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                    self.thread_key = Some(value.into());
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
                ) -> Result<crate::schemas::Message, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Message, crate::Error> {
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/messages");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("requestId", &self.request_id)]);
                    req = req.query(&[("threadKey", &self.thread_key)]);
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
    pub mod media {
        pub mod params {}
        pub struct MediaActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> MediaActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Downloads media. Download is supported on the URI `/v1/media/{+name}?alt=media`."]
            pub fn download(&self, resource_name: impl Into<String>) -> DownloadRequestBuilder {
                DownloadRequestBuilder {
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
        }
        #[doc = "Created via [MediaActions::download()](struct.MediaActions.html#method.download)"]
        #[derive(Debug, Clone)]
        pub struct DownloadRequestBuilder<'a> {
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
        impl<'a> DownloadRequestBuilder<'a> {
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
            fn _download_path(&self) -> String {
                let mut output = "https://chat.googleapis.com/download/".to_owned();
                output.push_str("v1/media/");
                {
                    let var_as_str = &self.resource_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            pub async fn download<W>(mut self, output: &mut W) -> Result<u64, crate::Error>
            where
                W: futures::io::AsyncWrite + std::marker::Unpin + ?Sized,
            {
                use futures::io::AsyncWriteExt;
                self.alt = Some(crate::params::Alt::Media);
                let request = self._request(&self._path()).await?;
                let mut response = request.send().await?.error_for_status()?;
                let mut num_bytes_written: usize = 0;
                while let Some(chunk) = response.chunk().await? {
                    output.write(&chunk).await?;
                    num_bytes_written += chunk.len();
                }
                Ok(num_bytes_written as u64)
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
            ) -> Result<crate::schemas::Media, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Media, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
                output.push_str("v1/media/");
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
    }
    pub mod rooms {
        pub mod params {}
        pub struct RoomsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> RoomsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Legacy path for creating message. Calling these will result in a BadRequest response."]
            pub fn messages(
                &self,
                request: crate::schemas::Message,
                parent: impl Into<String>,
            ) -> MessagesRequestBuilder {
                MessagesRequestBuilder {
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
                    request_id: None,
                    thread_key: None,
                }
            }
            #[doc = "Legacy path for creating message. Calling these will result in a BadRequest response."]
            pub fn webhooks(
                &self,
                request: crate::schemas::Message,
                parent: impl Into<String>,
            ) -> WebhooksRequestBuilder {
                WebhooksRequestBuilder {
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
                    request_id: None,
                    thread_key: None,
                }
            }
            #[doc = "Actions that can be performed on the conversations resource"]
            pub fn conversations(
                &self,
            ) -> crate::resources::rooms::conversations::ConversationsActions {
                crate::resources::rooms::conversations::ConversationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [RoomsActions::messages()](struct.RoomsActions.html#method.messages)"]
        #[derive(Debug, Clone)]
        pub struct MessagesRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Message,
            parent: String,
            request_id: ::std::option::Option<String>,
            thread_key: ::std::option::Option<String>,
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
        impl<'a> MessagesRequestBuilder<'a> {
            #[doc = "Optional. A unique request ID for this message. Specifying an existing request ID returns the message created with that ID instead of creating a new message."]
            pub fn request_id(mut self, value: impl Into<String>) -> Self {
                self.request_id = Some(value.into());
                self
            }
            #[doc = "Optional. Opaque thread identifier. To start or add to a thread, create a message and specify a `threadKey` instead of thread.name. (Setting thread.name has no effect.) The first message with a given `threadKey` starts a new thread. Subsequent messages with the same `threadKey` post into the same thread."]
            pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                self.thread_key = Some(value.into());
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
            ) -> Result<crate::schemas::Message, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Message, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/messages");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("requestId", &self.request_id)]);
                req = req.query(&[("threadKey", &self.thread_key)]);
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
        #[doc = "Created via [RoomsActions::webhooks()](struct.RoomsActions.html#method.webhooks)"]
        #[derive(Debug, Clone)]
        pub struct WebhooksRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Message,
            parent: String,
            request_id: ::std::option::Option<String>,
            thread_key: ::std::option::Option<String>,
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
        impl<'a> WebhooksRequestBuilder<'a> {
            #[doc = "Optional. A unique request ID for this message. Specifying an existing request ID returns the message created with that ID instead of creating a new message."]
            pub fn request_id(mut self, value: impl Into<String>) -> Self {
                self.request_id = Some(value.into());
                self
            }
            #[doc = "Optional. Opaque thread identifier. To start or add to a thread, create a message and specify a `threadKey` instead of thread.name. (Setting thread.name has no effect.) The first message with a given `threadKey` starts a new thread. Subsequent messages with the same `threadKey` post into the same thread."]
            pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                self.thread_key = Some(value.into());
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
            ) -> Result<crate::schemas::Message, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Message, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/webhooks");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("requestId", &self.request_id)]);
                req = req.query(&[("threadKey", &self.thread_key)]);
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
        pub mod conversations {
            pub mod params {}
            pub struct ConversationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ConversationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Legacy path for creating message. Calling these will result in a BadRequest response."]
                pub fn messages(
                    &self,
                    request: crate::schemas::Message,
                    parent: impl Into<String>,
                ) -> MessagesRequestBuilder {
                    MessagesRequestBuilder {
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
                        request_id: None,
                        thread_key: None,
                    }
                }
            }
            #[doc = "Created via [ConversationsActions::messages()](struct.ConversationsActions.html#method.messages)"]
            #[derive(Debug, Clone)]
            pub struct MessagesRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Message,
                parent: String,
                request_id: ::std::option::Option<String>,
                thread_key: ::std::option::Option<String>,
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
            impl<'a> MessagesRequestBuilder<'a> {
                #[doc = "Optional. A unique request ID for this message. Specifying an existing request ID returns the message created with that ID instead of creating a new message."]
                pub fn request_id(mut self, value: impl Into<String>) -> Self {
                    self.request_id = Some(value.into());
                    self
                }
                #[doc = "Optional. Opaque thread identifier. To start or add to a thread, create a message and specify a `threadKey` instead of thread.name. (Setting thread.name has no effect.) The first message with a given `threadKey` starts a new thread. Subsequent messages with the same `threadKey` post into the same thread."]
                pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                    self.thread_key = Some(value.into());
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
                ) -> Result<crate::schemas::Message, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Message, crate::Error> {
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/messages");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("requestId", &self.request_id)]);
                    req = req.query(&[("threadKey", &self.thread_key)]);
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
    pub mod spaces {
        pub mod params {}
        pub struct SpacesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> SpacesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns a space. Requires [service account authentication](https://developers.google.com/chat/api/guides/auth/service-accounts)."]
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
            #[doc = "Lists spaces the caller is a member of. Requires [service account authentication](https://developers.google.com/chat/api/guides/auth/service-accounts)."]
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
                    page_size: None,
                    page_token: None,
                }
            }
            #[doc = "Legacy path for creating message. Calling these will result in a BadRequest response."]
            pub fn webhooks(
                &self,
                request: crate::schemas::Message,
                parent: impl Into<String>,
            ) -> WebhooksRequestBuilder {
                WebhooksRequestBuilder {
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
                    request_id: None,
                    thread_key: None,
                }
            }
            #[doc = "Actions that can be performed on the members resource"]
            pub fn members(&self) -> crate::resources::spaces::members::MembersActions {
                crate::resources::spaces::members::MembersActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the messages resource"]
            pub fn messages(&self) -> crate::resources::spaces::messages::MessagesActions {
                crate::resources::spaces::messages::MessagesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [SpacesActions::get()](struct.SpacesActions.html#method.get)"]
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
            ) -> Result<crate::schemas::Space, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Space, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
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
        #[doc = "Created via [SpacesActions::list()](struct.SpacesActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
            #[doc = "Requested page size. The value is capped at 1000. Server may return fewer results than requested. If unspecified, server will default to 100."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "A token identifying a page of results the server should return."]
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
            #[doc = "\nExecute the request and yield each item in the `spaces` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
            pub fn stream_spaces<T>(
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
                self.stream_spaces_with_fields(fields)
            }
            #[doc = "\nExecute the request and yield each item in the `spaces` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
            pub fn stream_spaces_with_default_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::Space, crate::Error>> + 'a
            {
                self.stream_spaces_with_fields(None::<String>)
            }
            #[doc = "\nExecute the request and yield each item in the `spaces` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
            pub fn stream_spaces_with_all_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::Space, crate::Error>> + 'a
            {
                self.stream_spaces_with_fields(Some("*"))
            }
            #[doc = "\nExecute the request and yield each item in the `spaces` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
            pub fn stream_spaces_with_fields<T, F>(
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
                    #[serde(rename = "spaces")]
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
                    let mut selector = concat!("nextPageToken,", "spaces").to_owned();
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
                Item = Result<crate::schemas::ListSpacesResponse, crate::Error>,
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
                Item = Result<crate::schemas::ListSpacesResponse, crate::Error>,
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
            ) -> Result<crate::schemas::ListSpacesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListSpacesResponse, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
                output.push_str("v1/spaces");
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
        #[doc = "Created via [SpacesActions::webhooks()](struct.SpacesActions.html#method.webhooks)"]
        #[derive(Debug, Clone)]
        pub struct WebhooksRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Message,
            parent: String,
            request_id: ::std::option::Option<String>,
            thread_key: ::std::option::Option<String>,
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
        impl<'a> WebhooksRequestBuilder<'a> {
            #[doc = "Optional. A unique request ID for this message. Specifying an existing request ID returns the message created with that ID instead of creating a new message."]
            pub fn request_id(mut self, value: impl Into<String>) -> Self {
                self.request_id = Some(value.into());
                self
            }
            #[doc = "Optional. Opaque thread identifier. To start or add to a thread, create a message and specify a `threadKey` instead of thread.name. (Setting thread.name has no effect.) The first message with a given `threadKey` starts a new thread. Subsequent messages with the same `threadKey` post into the same thread."]
            pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                self.thread_key = Some(value.into());
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
            ) -> Result<crate::schemas::Message, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Message, crate::Error> {
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
                let mut output = "https://chat.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str("/webhooks");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("requestId", &self.request_id)]);
                req = req.query(&[("threadKey", &self.thread_key)]);
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
        pub mod members {
            pub mod params {}
            pub struct MembersActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> MembersActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Returns a membership. Requires [service account authentication](https://developers.google.com/chat/api/guides/auth/service-accounts)."]
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
                #[doc = "Lists human memberships in a space. Requires [service account authentication](https://developers.google.com/chat/api/guides/auth/service-accounts)."]
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
            #[doc = "Created via [MembersActions::get()](struct.MembersActions.html#method.get)"]
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
                ) -> Result<crate::schemas::Membership, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Membership, crate::Error> {
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
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
            #[doc = "Created via [MembersActions::list()](struct.MembersActions.html#method.list)"]
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
                #[doc = "Requested page size. The value is capped at 1000. Server may return fewer results than requested. If unspecified, server will default to 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A token identifying a page of results the server should return."]
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
                #[doc = "\nExecute the request and yield each item in the `memberships` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_memberships<T>(
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
                    self.stream_memberships_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `memberships` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_memberships_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Membership, crate::Error>> + 'a
                {
                    self.stream_memberships_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `memberships` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_memberships_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Membership, crate::Error>> + 'a
                {
                    self.stream_memberships_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `memberships` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_memberships_with_fields<T, F>(
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
                        #[serde(rename = "memberships")]
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
                        let mut selector = concat!("nextPageToken,", "memberships").to_owned();
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
                    Item = Result<crate::schemas::ListMembershipsResponse, crate::Error>,
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
                    Item = Result<crate::schemas::ListMembershipsResponse, crate::Error>,
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
                ) -> Result<crate::schemas::ListMembershipsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListMembershipsResponse, crate::Error> {
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/members");
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
        pub mod messages {
            pub mod params {}
            pub struct MessagesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> MessagesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a message. Requires [service account authentication](https://developers.google.com/chat/api/guides/auth/service-accounts)."]
                pub fn create(
                    &self,
                    request: crate::schemas::Message,
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
                        request_id: None,
                        thread_key: None,
                    }
                }
                #[doc = "Deletes a message. Requires [service account authentication](https://developers.google.com/chat/api/guides/auth/service-accounts)."]
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
                #[doc = "Returns a message. Requires [service account authentication](https://developers.google.com/chat/api/guides/auth/service-accounts)."]
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
                #[doc = "Updates a message. Requires [service account authentication](https://developers.google.com/chat/api/guides/auth/service-accounts)."]
                pub fn update(
                    &self,
                    request: crate::schemas::Message,
                    name: impl Into<String>,
                ) -> UpdateRequestBuilder {
                    UpdateRequestBuilder {
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
                #[doc = "Actions that can be performed on the attachments resource"]
                pub fn attachments(
                    &self,
                ) -> crate::resources::spaces::messages::attachments::AttachmentsActions
                {
                    crate::resources::spaces::messages::attachments::AttachmentsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [MessagesActions::create()](struct.MessagesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Message,
                parent: String,
                request_id: ::std::option::Option<String>,
                thread_key: ::std::option::Option<String>,
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
                #[doc = "Optional. A unique request ID for this message. Specifying an existing request ID returns the message created with that ID instead of creating a new message."]
                pub fn request_id(mut self, value: impl Into<String>) -> Self {
                    self.request_id = Some(value.into());
                    self
                }
                #[doc = "Optional. Opaque thread identifier. To start or add to a thread, create a message and specify a `threadKey` instead of thread.name. (Setting thread.name has no effect.) The first message with a given `threadKey` starts a new thread. Subsequent messages with the same `threadKey` post into the same thread."]
                pub fn thread_key(mut self, value: impl Into<String>) -> Self {
                    self.thread_key = Some(value.into());
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
                ) -> Result<crate::schemas::Message, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Message, crate::Error> {
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/messages");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("requestId", &self.request_id)]);
                    req = req.query(&[("threadKey", &self.thread_key)]);
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
            #[doc = "Created via [MessagesActions::delete()](struct.MessagesActions.html#method.delete)"]
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
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
            #[doc = "Created via [MessagesActions::get()](struct.MessagesActions.html#method.get)"]
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
                ) -> Result<crate::schemas::Message, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Message, crate::Error> {
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
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
            #[doc = "Created via [MessagesActions::update()](struct.MessagesActions.html#method.update)"]
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Message,
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
            impl<'a> UpdateRequestBuilder<'a> {
                #[doc = "Required. The field paths to be updated, comma separated if there are multiple. Currently supported field paths: * text * cards * attachment"]
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
                ) -> Result<crate::schemas::Message, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Message, crate::Error> {
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
                    let mut output = "https://chat.googleapis.com/".to_owned();
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
                    let mut req = self.reqwest.request(::reqwest::Method::PUT, path);
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
            pub mod attachments {
                pub mod params {}
                pub struct AttachmentsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> AttachmentsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Gets the metadata of a message attachment. The attachment data is fetched using the media API. Requires [service account authentication](https://developers.google.com/chat/api/guides/auth/service-accounts)."]
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
                }
                #[doc = "Created via [AttachmentsActions::get()](struct.AttachmentsActions.html#method.get)"]
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
                    ) -> Result<crate::schemas::Attachment, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Attachment, crate::Error> {
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
                        let mut output = "https://chat.googleapis.com/".to_owned();
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
