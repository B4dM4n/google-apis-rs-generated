#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [videos](resources/videos/struct.VideosActions.html)\n  * [*annotate*](resources/videos/struct.AnnotateRequestBuilder.html)\n"]
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
    pub struct GoogleCloudVideointelligenceV1AnnotateVideoProgress {
        #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
        #[serde(
            rename = "annotationProgress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotation_progress: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1VideoAnnotationProgress>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1AnnotateVideoProgress
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1AnnotateVideoProgress {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVideointelligenceV1AnnotateVideoResponse {
        #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
        #[serde(
            rename = "annotationResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotation_results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1VideoAnnotationResults>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1AnnotateVideoResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1AnnotateVideoResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2AnnotateVideoProgress {
        #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
        #[serde(
            rename = "annotationProgress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotation_progress: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgress>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2AnnotateVideoProgress
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2AnnotateVideoProgress
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVideointelligenceV1Beta2AnnotateVideoResponse {
        #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
        #[serde(
            rename = "annotationResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotation_results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoAnnotationResults>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2AnnotateVideoResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2AnnotateVideoResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2DetectedAttribute {
        #[doc = "Detected attribute confidence. Range \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The name of the attribute, for example, glasses, dark_glasses, mouth_open. A full list of supported type names will be provided in the document."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Text value of the detection result. For example, the value for “HairColor” can be “black”, “blonde”, etc."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2DetectedAttribute
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2DetectedAttribute {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2DetectedLandmark {
        #[doc = "The confidence score of the detected landmark. Range \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The name of this landmark, for example, left_hand, right_shoulder."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The 2D point of the detected landmark using the normalized image coordindate system. The normalized coordinates have the range from 0 to 1."]
        #[serde(
            rename = "point",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub point: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1Beta2NormalizedVertex,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2DetectedLandmark
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2DetectedLandmark {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2Entity {
        #[doc = "Textual description, e.g., `Fixed-gear bicycle`."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
        #[serde(
            rename = "entityId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_id: ::std::option::Option<String>,
        #[doc = "Language code for `description` in BCP-47 format."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2Entity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2Entity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2ExplicitContentAnnotation {
        #[doc = "All video frames where explicit content was detected."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2ExplicitContentFrame>,
        >,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2ExplicitContentAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2ExplicitContentAnnotation
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
    pub struct GoogleCloudVideointelligenceV1Beta2ExplicitContentFrame { # [doc = "Likelihood of the pornography content.."] # [serde (rename = "pornographyLikelihood" , default , skip_serializing_if = "std::option::Option::is_none")] pub pornography_likelihood : :: std :: option :: Option < crate :: schemas :: GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood > , # [doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."] # [serde (rename = "timeOffset" , default , skip_serializing_if = "std::option::Option::is_none")] pub time_offset : :: std :: option :: Option < String > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2ExplicitContentFrame
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2ExplicitContentFrame
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood {
        #[doc = "Unspecified likelihood."]
        LikelihoodUnspecified,
        #[doc = "Likely."]
        Likely,
        #[doc = "Possible."]
        Possible,
        #[doc = "Unlikely."]
        Unlikely,
        #[doc = "Very likely."]
        VeryLikely,
        #[doc = "Very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified => "LIKELIHOOD_UNSPECIFIED" , GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: Likely => "LIKELY" , GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: Possible => "POSSIBLE" , GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: Unlikely => "UNLIKELY" , GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: VeryLikely => "VERY_LIKELY" , GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: VeryUnlikely => "VERY_UNLIKELY" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood,
            (),
        > {
            Ok (match s { "LIKELIHOOD_UNSPECIFIED" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified , "LIKELY" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: Likely , "POSSIBLE" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: Possible , "UNLIKELY" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: Unlikely , "VERY_LIKELY" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: VeryLikely , "VERY_UNLIKELY" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: VeryUnlikely , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "LIKELIHOOD_UNSPECIFIED" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified , "LIKELY" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: Likely , "POSSIBLE" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: Possible , "UNLIKELY" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: Unlikely , "VERY_LIKELY" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: VeryLikely , "VERY_UNLIKELY" => GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood :: VeryUnlikely , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2ExplicitContentFramePornographyLikelihood
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2FaceAnnotation {
        #[doc = "All video frames where a face was detected."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2FaceFrame>,
        >,
        #[doc = "All video segments where a face was detected."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2FaceSegment>,
        >,
        #[doc = "Thumbnail of a representative face view (in JPEG format)."]
        #[serde(
            rename = "thumbnail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2FaceAnnotation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2FaceAnnotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2FaceDetectionAnnotation {
        #[doc = "The thumbnail of a person’s face."]
        #[serde(
            rename = "thumbnail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The face tracks with attributes."]
        #[serde(
            rename = "tracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracks:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2Track>>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2FaceDetectionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2FaceDetectionAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2FaceFrame {
        #[doc = "Normalized Bounding boxes in a frame. There can be more than one boxes if the same face is detected in multiple locations within the current frame."]
        #[serde(
            rename = "normalizedBoundingBoxes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_bounding_boxes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2NormalizedBoundingBox>,
        >,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2FaceFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2FaceFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2FaceSegment {
        #[doc = "Video segment where a face was detected."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoSegment>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2FaceSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2FaceSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2LabelAnnotation {
        #[doc = "Common categories for the detected entity. For example, when the label is `Terrier`, the category is likely `dog`. And in some cases there might be more than one categories e.g., `Terrier` could also be a `pet`."]
        #[serde(
            rename = "categoryEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category_entities:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2Entity>>,
        #[doc = "Detected entity."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2Entity>,
        #[doc = "All video frames where a label was detected."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2LabelFrame>,
        >,
        #[doc = "All video segments where a label was detected."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2LabelSegment>,
        >,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2LabelAnnotation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2LabelAnnotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2LabelFrame {
        #[doc = "Confidence that the label is accurate. Range: \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2LabelFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2LabelFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2LabelSegment {
        #[doc = "Confidence that the label is accurate. Range: \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Video segment where a label was detected."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoSegment>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2LabelSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2LabelSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2LogoRecognitionAnnotation {
        #[doc = "Entity category information to specify the logo class that all the logo tracks within this LogoRecognitionAnnotation are recognized as."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2Entity>,
        #[doc = "All video segments where the recognized logo appears. There might be multiple instances of the same logo class appearing in one VideoSegment."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoSegment>,
        >,
        #[doc = "All logo tracks where the recognized logo appears. Each track corresponds to one logo instance appearing in consecutive frames."]
        #[serde(
            rename = "tracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracks:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2Track>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2LogoRecognitionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2LogoRecognitionAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2NormalizedBoundingBox {
        #[doc = "Bottom Y coordinate."]
        #[serde(
            rename = "bottom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bottom: ::std::option::Option<f32>,
        #[doc = "Left X coordinate."]
        #[serde(
            rename = "left",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left: ::std::option::Option<f32>,
        #[doc = "Right X coordinate."]
        #[serde(
            rename = "right",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub right: ::std::option::Option<f32>,
        #[doc = "Top Y coordinate."]
        #[serde(
            rename = "top",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2NormalizedBoundingBox
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2NormalizedBoundingBox
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2NormalizedBoundingPoly {
        #[doc = "Normalized vertices of the bounding polygon."]
        #[serde(
            rename = "vertices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertices: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2NormalizedVertex>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2NormalizedBoundingPoly
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2NormalizedBoundingPoly
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(
            rename = "x",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(
            rename = "y",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2NormalizedVertex
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2NormalizedVertex {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2ObjectTrackingAnnotation {
        #[doc = "Object category’s labeling confidence of this track."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Entity to specify the object category that this track is labeled as."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2Entity>,
        #[doc = "Information corresponding to all frames where this object track appears. Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame messages in frames. Streaming mode: it can only be one ObjectTrackingFrame message in frames."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2ObjectTrackingFrame>,
        >,
        #[doc = "Non-streaming batch mode ONLY. Each object track corresponds to one video segment where it appears."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoSegment>,
        #[doc = "Streaming mode ONLY. In streaming mode, we do not know the end time of a tracked object before it is completed. Hence, there is no VideoSegment info returned. Instead, we provide a unique identifiable integer track_id so that the customers can correlate the results of the ongoing ObjectTrackAnnotation of the same track_id over time."]
        #[serde(
            rename = "trackId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub track_id: ::std::option::Option<i64>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2ObjectTrackingAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2ObjectTrackingAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2ObjectTrackingFrame {
        #[doc = "The normalized bounding box location of this object track for the frame."]
        #[serde(
            rename = "normalizedBoundingBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_bounding_box: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1Beta2NormalizedBoundingBox,
        >,
        #[doc = "The timestamp of the frame in microseconds."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2ObjectTrackingFrame
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2ObjectTrackingFrame
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2PersonDetectionAnnotation {
        #[doc = "The detected tracks of a person."]
        #[serde(
            rename = "tracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracks:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2Track>>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2PersonDetectionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2PersonDetectionAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2SpeechRecognitionAlternative {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Transcript text representing the words that the user spoke."]
        #[serde(
            rename = "transcript",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transcript: ::std::option::Option<String>,
        #[doc = "Output only. A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is set to true, you will see all the words from the beginning of the audio."]
        #[serde(
            rename = "words",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub words:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2WordInfo>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2SpeechRecognitionAlternative
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2SpeechRecognitionAlternative
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2SpeechTranscription {
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
        #[serde(
            rename = "alternatives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alternatives: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2SpeechRecognitionAlternative>,
        >,
        #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2SpeechTranscription
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2SpeechTranscription
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2TextAnnotation {
        #[doc = "All video segments where OCR detected text appears."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2TextSegment>,
        >,
        #[doc = "The detected text."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2TextAnnotation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2TextAnnotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2TextFrame {
        #[doc = "Bounding polygon of the detected text for this frame."]
        #[serde(
            rename = "rotatedBoundingBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rotated_bounding_box: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1Beta2NormalizedBoundingPoly,
        >,
        #[doc = "Timestamp of this frame."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2TextFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2TextFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2TextSegment {
        #[doc = "Confidence for the track of detected text. It is calculated as the highest over all frames where OCR detected text appears."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Information related to the frames where OCR detected text appears."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2TextFrame>,
        >,
        #[doc = "Video segment where a text snippet was detected."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoSegment>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2TextSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2TextSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2TimestampedObject {
        #[doc = "Optional. The attributes of the object in the bounding box."]
        #[serde(
            rename = "attributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2DetectedAttribute>,
        >,
        #[doc = "Optional. The detected landmarks."]
        #[serde(
            rename = "landmarks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub landmarks: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2DetectedLandmark>,
        >,
        #[doc = "Normalized Bounding box in a frame, where the object is located."]
        #[serde(
            rename = "normalizedBoundingBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_bounding_box: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1Beta2NormalizedBoundingBox,
        >,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this object."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2TimestampedObject
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2TimestampedObject {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2Track {
        #[doc = "Optional. Attributes in the track level."]
        #[serde(
            rename = "attributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2DetectedAttribute>,
        >,
        #[doc = "Optional. The confidence score of the tracked object."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Video segment of a track."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoSegment>,
        #[doc = "The object with timestamp and attributes per frame in the track."]
        #[serde(
            rename = "timestampedObjects",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamped_objects: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2TimestampedObject>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2Track {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2Track {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgress {
        #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
        #[serde(
            rename = "feature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feature: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature,
        >,
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(
            rename = "inputUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_uri: ::std::option::Option<String>,
        #[doc = "Approximate percentage processed thus far. Guaranteed to be 100 when fully processed."]
        #[serde(
            rename = "progressPercent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_percent: ::std::option::Option<i32>,
        #[doc = "Specifies which segment is being tracked if the request contains more than one segment."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoSegment>,
        #[doc = "Time when the request was received."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Time of the most recent update."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgress
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgress
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature {
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[doc = "Human face detection."]
        FaceDetection,
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[doc = "Logo detection, tracking, and recognition."]
        LogoRecognition,
        #[doc = "Object detection and tracking."]
        ObjectTracking,
        #[doc = "Person detection."]
        PersonDetection,
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[doc = "OCR text detection and tracking."]
        TextDetection,
    }
    impl GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: ExplicitContentDetection => "EXPLICIT_CONTENT_DETECTION" , GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: FaceDetection => "FACE_DETECTION" , GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: FeatureUnspecified => "FEATURE_UNSPECIFIED" , GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: LabelDetection => "LABEL_DETECTION" , GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: LogoRecognition => "LOGO_RECOGNITION" , GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: ObjectTracking => "OBJECT_TRACKING" , GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: PersonDetection => "PERSON_DETECTION" , GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: ShotChangeDetection => "SHOT_CHANGE_DETECTION" , GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: SpeechTranscription => "SPEECH_TRANSCRIPTION" , GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: TextDetection => "TEXT_DETECTION" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature,
            (),
        > {
            Ok (match s { "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: ExplicitContentDetection , "FACE_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: FaceDetection , "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: LabelDetection , "LOGO_RECOGNITION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: LogoRecognition , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: ObjectTracking , "PERSON_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: PersonDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: ShotChangeDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: TextDetection , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: ExplicitContentDetection , "FACE_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: FaceDetection , "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: LabelDetection , "LOGO_RECOGNITION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: LogoRecognition , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: ObjectTracking , "PERSON_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: PersonDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: ShotChangeDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature :: TextDetection , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2VideoAnnotationProgressFeature
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVideointelligenceV1Beta2VideoAnnotationResults {
        #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest` some videos may succeed and some may fail."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Explicit content annotation."]
        #[serde(
            rename = "explicitAnnotation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub explicit_annotation: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1Beta2ExplicitContentAnnotation,
        >,
        #[doc = "Deprecated. Please use `face_detection_annotations` instead."]
        #[serde(
            rename = "faceAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub face_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2FaceAnnotation>,
        >,
        #[doc = "Face detection annotations."]
        #[serde(
            rename = "faceDetectionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub face_detection_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2FaceDetectionAnnotation>,
        >,
        #[doc = "Label annotations on frame level. There is exactly one element for each unique label."]
        #[serde(
            rename = "frameLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frame_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2LabelAnnotation>,
        >,
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(
            rename = "inputUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_uri: ::std::option::Option<String>,
        #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
        #[serde(
            rename = "logoRecognitionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logo_recognition_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2LogoRecognitionAnnotation>,
        >,
        #[doc = "Annotations for list of objects detected and tracked in video."]
        #[serde(
            rename = "objectAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2ObjectTrackingAnnotation>,
        >,
        #[doc = "Person detection annotations."]
        #[serde(
            rename = "personDetectionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person_detection_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2PersonDetectionAnnotation>,
        >,
        #[doc = "Video segment on which the annotation is run."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoSegment>,
        #[doc = "Topical label annotations on video level or user-specified segment level. There is exactly one element for each unique label."]
        #[serde(
            rename = "segmentLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2LabelAnnotation>,
        >,
        #[doc = "Presence label annotations on video level or user-specified segment level. There is exactly one element for each unique label. Compared to the existing topical `segment_label_annotations`, this field presents more fine-grained, segment-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to “builtin/latest” in the request."]
        #[serde(
            rename = "segmentPresenceLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_presence_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2LabelAnnotation>,
        >,
        #[doc = "Shot annotations. Each shot is represented as a video segment."]
        #[serde(
            rename = "shotAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2VideoSegment>,
        >,
        #[doc = "Topical label annotations on shot level. There is exactly one element for each unique label."]
        #[serde(
            rename = "shotLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2LabelAnnotation>,
        >,
        #[doc = "Presence label annotations on shot level. There is exactly one element for each unique label. Compared to the existing topical `shot_label_annotations`, this field presents more fine-grained, shot-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to “builtin/latest” in the request."]
        #[serde(
            rename = "shotPresenceLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_presence_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2LabelAnnotation>,
        >,
        #[doc = "Speech transcription."]
        #[serde(
            rename = "speechTranscriptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub speech_transcriptions: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2SpeechTranscription>,
        >,
        #[doc = "OCR text detection and tracking. Annotations for list of detected text snippets. Each will have list of frame information associated with it."]
        #[serde(
            rename = "textAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1Beta2TextAnnotation>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1Beta2VideoAnnotationResults
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1Beta2VideoAnnotationResults
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
    pub struct GoogleCloudVideointelligenceV1Beta2VideoSegment {
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive)."]
        #[serde(
            rename = "endTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time_offset: ::std::option::Option<String>,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive)."]
        #[serde(
            rename = "startTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2VideoSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2VideoSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Beta2WordInfo {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from 1 up to diarization_speaker_count, and is only set if speaker diarization is enabled."]
        #[serde(
            rename = "speakerTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub speaker_tag: ::std::option::Option<i32>,
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the start of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "The word corresponding to this set of information."]
        #[serde(
            rename = "word",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub word: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1Beta2WordInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Beta2WordInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1DetectedAttribute {
        #[doc = "Detected attribute confidence. Range \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The name of the attribute, for example, glasses, dark_glasses, mouth_open. A full list of supported type names will be provided in the document."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Text value of the detection result. For example, the value for “HairColor” can be “black”, “blonde”, etc."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1DetectedAttribute {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1DetectedAttribute {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1DetectedLandmark {
        #[doc = "The confidence score of the detected landmark. Range \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The name of this landmark, for example, left_hand, right_shoulder."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The 2D point of the detected landmark using the normalized image coordindate system. The normalized coordinates have the range from 0 to 1."]
        #[serde(
            rename = "point",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub point:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1NormalizedVertex>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1DetectedLandmark {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1DetectedLandmark {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Entity {
        #[doc = "Textual description, e.g., `Fixed-gear bicycle`."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
        #[serde(
            rename = "entityId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_id: ::std::option::Option<String>,
        #[doc = "Language code for `description` in BCP-47 format."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1Entity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Entity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1ExplicitContentAnnotation {
        #[doc = "All video frames where explicit content was detected."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1ExplicitContentFrame>,
        >,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1ExplicitContentAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1ExplicitContentAnnotation
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
    pub struct GoogleCloudVideointelligenceV1ExplicitContentFrame {
        #[doc = "Likelihood of the pornography content.."]
        #[serde(
            rename = "pornographyLikelihood",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pornography_likelihood: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood,
        >,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1ExplicitContentFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1ExplicitContentFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood {
        #[doc = "Unspecified likelihood."]
        LikelihoodUnspecified,
        #[doc = "Likely."]
        Likely,
        #[doc = "Possible."]
        Possible,
        #[doc = "Unlikely."]
        Unlikely,
        #[doc = "Very likely."]
        VeryLikely,
        #[doc = "Very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified => "LIKELIHOOD_UNSPECIFIED" , GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: Likely => "LIKELY" , GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: Possible => "POSSIBLE" , GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: Unlikely => "UNLIKELY" , GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: VeryLikely => "VERY_LIKELY" , GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: VeryUnlikely => "VERY_UNLIKELY" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood,
            (),
        > {
            Ok (match s { "LIKELIHOOD_UNSPECIFIED" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified , "LIKELY" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: Likely , "POSSIBLE" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: Possible , "UNLIKELY" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: Unlikely , "VERY_LIKELY" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: VeryLikely , "VERY_UNLIKELY" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: VeryUnlikely , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "LIKELIHOOD_UNSPECIFIED" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified , "LIKELY" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: Likely , "POSSIBLE" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: Possible , "UNLIKELY" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: Unlikely , "VERY_LIKELY" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: VeryLikely , "VERY_UNLIKELY" => GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood :: VeryUnlikely , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1ExplicitContentFramePornographyLikelihood
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1FaceAnnotation {
        #[doc = "All video frames where a face was detected."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1FaceFrame>>,
        #[doc = "All video segments where a face was detected."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1FaceSegment>>,
        #[doc = "Thumbnail of a representative face view (in JPEG format)."]
        #[serde(
            rename = "thumbnail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1FaceAnnotation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1FaceAnnotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1FaceDetectionAnnotation {
        #[doc = "The thumbnail of a person’s face."]
        #[serde(
            rename = "thumbnail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The face tracks with attributes."]
        #[serde(
            rename = "tracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracks: ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Track>>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1FaceDetectionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1FaceDetectionAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1FaceFrame {
        #[doc = "Normalized Bounding boxes in a frame. There can be more than one boxes if the same face is detected in multiple locations within the current frame."]
        #[serde(
            rename = "normalizedBoundingBoxes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_bounding_boxes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1NormalizedBoundingBox>,
        >,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1FaceFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1FaceFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1FaceSegment {
        #[doc = "Video segment where a face was detected."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1VideoSegment>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1FaceSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1FaceSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1LabelAnnotation {
        #[doc = "Common categories for the detected entity. For example, when the label is `Terrier`, the category is likely `dog`. And in some cases there might be more than one categories e.g., `Terrier` could also be a `pet`."]
        #[serde(
            rename = "categoryEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category_entities:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Entity>>,
        #[doc = "Detected entity."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity: ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1Entity>,
        #[doc = "All video frames where a label was detected."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1LabelFrame>>,
        #[doc = "All video segments where a label was detected."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1LabelSegment>>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1LabelAnnotation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1LabelAnnotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1LabelFrame {
        #[doc = "Confidence that the label is accurate. Range: \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1LabelFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1LabelFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1LabelSegment {
        #[doc = "Confidence that the label is accurate. Range: \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Video segment where a label was detected."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1VideoSegment>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1LabelSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1LabelSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1LogoRecognitionAnnotation {
        #[doc = "Entity category information to specify the logo class that all the logo tracks within this LogoRecognitionAnnotation are recognized as."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity: ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1Entity>,
        #[doc = "All video segments where the recognized logo appears. There might be multiple instances of the same logo class appearing in one VideoSegment."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1VideoSegment>>,
        #[doc = "All logo tracks where the recognized logo appears. Each track corresponds to one logo instance appearing in consecutive frames."]
        #[serde(
            rename = "tracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracks: ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Track>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1LogoRecognitionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1LogoRecognitionAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1NormalizedBoundingBox {
        #[doc = "Bottom Y coordinate."]
        #[serde(
            rename = "bottom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bottom: ::std::option::Option<f32>,
        #[doc = "Left X coordinate."]
        #[serde(
            rename = "left",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left: ::std::option::Option<f32>,
        #[doc = "Right X coordinate."]
        #[serde(
            rename = "right",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub right: ::std::option::Option<f32>,
        #[doc = "Top Y coordinate."]
        #[serde(
            rename = "top",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1NormalizedBoundingBox
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1NormalizedBoundingBox {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1NormalizedBoundingPoly {
        #[doc = "Normalized vertices of the bounding polygon."]
        #[serde(
            rename = "vertices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertices: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1NormalizedVertex>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1NormalizedBoundingPoly
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1NormalizedBoundingPoly {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(
            rename = "x",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(
            rename = "y",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1NormalizedVertex {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1NormalizedVertex {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1ObjectTrackingAnnotation {
        #[doc = "Object category’s labeling confidence of this track."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Entity to specify the object category that this track is labeled as."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity: ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1Entity>,
        #[doc = "Information corresponding to all frames where this object track appears. Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame messages in frames. Streaming mode: it can only be one ObjectTrackingFrame message in frames."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1ObjectTrackingFrame>,
        >,
        #[doc = "Non-streaming batch mode ONLY. Each object track corresponds to one video segment where it appears."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1VideoSegment>,
        #[doc = "Streaming mode ONLY. In streaming mode, we do not know the end time of a tracked object before it is completed. Hence, there is no VideoSegment info returned. Instead, we provide a unique identifiable integer track_id so that the customers can correlate the results of the ongoing ObjectTrackAnnotation of the same track_id over time."]
        #[serde(
            rename = "trackId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub track_id: ::std::option::Option<i64>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1ObjectTrackingAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1ObjectTrackingAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1ObjectTrackingFrame {
        #[doc = "The normalized bounding box location of this object track for the frame."]
        #[serde(
            rename = "normalizedBoundingBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_bounding_box: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1NormalizedBoundingBox,
        >,
        #[doc = "The timestamp of the frame in microseconds."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1ObjectTrackingFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1ObjectTrackingFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1AnnotateVideoProgress {
        #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
        #[serde(
            rename = "annotationProgress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotation_progress: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgress>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1AnnotateVideoProgress
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1AnnotateVideoProgress
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVideointelligenceV1P1Beta1AnnotateVideoResponse {
        #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
        #[serde(
            rename = "annotationResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotation_results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationResults>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1AnnotateVideoResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1AnnotateVideoResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1DetectedAttribute {
        #[doc = "Detected attribute confidence. Range \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The name of the attribute, for example, glasses, dark_glasses, mouth_open. A full list of supported type names will be provided in the document."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Text value of the detection result. For example, the value for “HairColor” can be “black”, “blonde”, etc."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1DetectedAttribute
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1DetectedAttribute
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1DetectedLandmark {
        #[doc = "The confidence score of the detected landmark. Range \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The name of this landmark, for example, left_hand, right_shoulder."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The 2D point of the detected landmark using the normalized image coordindate system. The normalized coordinates have the range from 0 to 1."]
        #[serde(
            rename = "point",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub point: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P1Beta1NormalizedVertex,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1DetectedLandmark
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1DetectedLandmark
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1Entity {
        #[doc = "Textual description, e.g., `Fixed-gear bicycle`."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
        #[serde(
            rename = "entityId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_id: ::std::option::Option<String>,
        #[doc = "Language code for `description` in BCP-47 format."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1Entity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P1Beta1Entity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1ExplicitContentAnnotation {
        #[doc = "All video frames where explicit content was detected."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFrame>,
        >,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentAnnotation
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFrame { # [doc = "Likelihood of the pornography content.."] # [serde (rename = "pornographyLikelihood" , default , skip_serializing_if = "std::option::Option::is_none")] pub pornography_likelihood : :: std :: option :: Option < crate :: schemas :: GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood > , # [doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."] # [serde (rename = "timeOffset" , default , skip_serializing_if = "std::option::Option::is_none")] pub time_offset : :: std :: option :: Option < String > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFrame
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFrame
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood {
        #[doc = "Unspecified likelihood."]
        LikelihoodUnspecified,
        #[doc = "Likely."]
        Likely,
        #[doc = "Possible."]
        Possible,
        #[doc = "Unlikely."]
        Unlikely,
        #[doc = "Very likely."]
        VeryLikely,
        #[doc = "Very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified => "LIKELIHOOD_UNSPECIFIED" , GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: Likely => "LIKELY" , GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: Possible => "POSSIBLE" , GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: Unlikely => "UNLIKELY" , GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: VeryLikely => "VERY_LIKELY" , GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: VeryUnlikely => "VERY_UNLIKELY" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood,
            (),
        > {
            Ok (match s { "LIKELIHOOD_UNSPECIFIED" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified , "LIKELY" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: Likely , "POSSIBLE" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: Possible , "UNLIKELY" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: Unlikely , "VERY_LIKELY" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: VeryLikely , "VERY_UNLIKELY" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: VeryUnlikely , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "LIKELIHOOD_UNSPECIFIED" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified , "LIKELY" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: Likely , "POSSIBLE" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: Possible , "UNLIKELY" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: Unlikely , "VERY_LIKELY" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: VeryLikely , "VERY_UNLIKELY" => GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood :: VeryUnlikely , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1ExplicitContentFramePornographyLikelihood
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1FaceAnnotation {
        #[doc = "All video frames where a face was detected."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1FaceFrame>,
        >,
        #[doc = "All video segments where a face was detected."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1FaceSegment>,
        >,
        #[doc = "Thumbnail of a representative face view (in JPEG format)."]
        #[serde(
            rename = "thumbnail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1FaceAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P1Beta1FaceAnnotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1FaceDetectionAnnotation {
        #[doc = "The thumbnail of a person’s face."]
        #[serde(
            rename = "thumbnail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The face tracks with attributes."]
        #[serde(
            rename = "tracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracks:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1Track>>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1FaceDetectionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1FaceDetectionAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1FaceFrame {
        #[doc = "Normalized Bounding boxes in a frame. There can be more than one boxes if the same face is detected in multiple locations within the current frame."]
        #[serde(
            rename = "normalizedBoundingBoxes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_bounding_boxes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingBox>,
        >,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1FaceFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P1Beta1FaceFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1FaceSegment {
        #[doc = "Video segment where a face was detected."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoSegment,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1FaceSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P1Beta1FaceSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1LabelAnnotation {
        #[doc = "Common categories for the detected entity. For example, when the label is `Terrier`, the category is likely `dog`. And in some cases there might be more than one categories e.g., `Terrier` could also be a `pet`."]
        #[serde(
            rename = "categoryEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category_entities:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1Entity>>,
        #[doc = "Detected entity."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1Entity>,
        #[doc = "All video frames where a label was detected."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1LabelFrame>,
        >,
        #[doc = "All video segments where a label was detected."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1LabelSegment>,
        >,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1LabelAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P1Beta1LabelAnnotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1LabelFrame {
        #[doc = "Confidence that the label is accurate. Range: \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1LabelFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P1Beta1LabelFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1LabelSegment {
        #[doc = "Confidence that the label is accurate. Range: \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Video segment where a label was detected."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoSegment,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1LabelSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P1Beta1LabelSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1LogoRecognitionAnnotation {
        #[doc = "Entity category information to specify the logo class that all the logo tracks within this LogoRecognitionAnnotation are recognized as."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1Entity>,
        #[doc = "All video segments where the recognized logo appears. There might be multiple instances of the same logo class appearing in one VideoSegment."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoSegment>,
        >,
        #[doc = "All logo tracks where the recognized logo appears. Each track corresponds to one logo instance appearing in consecutive frames."]
        #[serde(
            rename = "tracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracks:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1Track>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1LogoRecognitionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1LogoRecognitionAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingBox {
        #[doc = "Bottom Y coordinate."]
        #[serde(
            rename = "bottom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bottom: ::std::option::Option<f32>,
        #[doc = "Left X coordinate."]
        #[serde(
            rename = "left",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left: ::std::option::Option<f32>,
        #[doc = "Right X coordinate."]
        #[serde(
            rename = "right",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub right: ::std::option::Option<f32>,
        #[doc = "Top Y coordinate."]
        #[serde(
            rename = "top",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingBox
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingBox
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingPoly {
        #[doc = "Normalized vertices of the bounding polygon."]
        #[serde(
            rename = "vertices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertices: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1NormalizedVertex>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingPoly
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingPoly
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(
            rename = "x",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(
            rename = "y",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1NormalizedVertex
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1NormalizedVertex
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1ObjectTrackingAnnotation {
        #[doc = "Object category’s labeling confidence of this track."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Entity to specify the object category that this track is labeled as."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1Entity>,
        #[doc = "Information corresponding to all frames where this object track appears. Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame messages in frames. Streaming mode: it can only be one ObjectTrackingFrame message in frames."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1ObjectTrackingFrame>,
        >,
        #[doc = "Non-streaming batch mode ONLY. Each object track corresponds to one video segment where it appears."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoSegment,
        >,
        #[doc = "Streaming mode ONLY. In streaming mode, we do not know the end time of a tracked object before it is completed. Hence, there is no VideoSegment info returned. Instead, we provide a unique identifiable integer track_id so that the customers can correlate the results of the ongoing ObjectTrackAnnotation of the same track_id over time."]
        #[serde(
            rename = "trackId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub track_id: ::std::option::Option<i64>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1ObjectTrackingAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1ObjectTrackingAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1ObjectTrackingFrame {
        #[doc = "The normalized bounding box location of this object track for the frame."]
        #[serde(
            rename = "normalizedBoundingBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_bounding_box: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingBox,
        >,
        #[doc = "The timestamp of the frame in microseconds."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1ObjectTrackingFrame
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1ObjectTrackingFrame
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1PersonDetectionAnnotation {
        #[doc = "The detected tracks of a person."]
        #[serde(
            rename = "tracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracks:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1Track>>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1PersonDetectionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1PersonDetectionAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1SpeechRecognitionAlternative {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Transcript text representing the words that the user spoke."]
        #[serde(
            rename = "transcript",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transcript: ::std::option::Option<String>,
        #[doc = "Output only. A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is set to true, you will see all the words from the beginning of the audio."]
        #[serde(
            rename = "words",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub words: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1WordInfo>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1SpeechRecognitionAlternative
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1SpeechRecognitionAlternative
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1SpeechTranscription {
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
        #[serde(
            rename = "alternatives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alternatives: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1SpeechRecognitionAlternative>,
        >,
        #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1SpeechTranscription
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1SpeechTranscription
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1TextAnnotation {
        #[doc = "All video segments where OCR detected text appears."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1TextSegment>,
        >,
        #[doc = "The detected text."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1TextAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P1Beta1TextAnnotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1TextFrame {
        #[doc = "Bounding polygon of the detected text for this frame."]
        #[serde(
            rename = "rotatedBoundingBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rotated_bounding_box: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingPoly,
        >,
        #[doc = "Timestamp of this frame."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1TextFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P1Beta1TextFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1TextSegment {
        #[doc = "Confidence for the track of detected text. It is calculated as the highest over all frames where OCR detected text appears."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Information related to the frames where OCR detected text appears."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1TextFrame>,
        >,
        #[doc = "Video segment where a text snippet was detected."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoSegment,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1TextSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P1Beta1TextSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1TimestampedObject {
        #[doc = "Optional. The attributes of the object in the bounding box."]
        #[serde(
            rename = "attributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1DetectedAttribute>,
        >,
        #[doc = "Optional. The detected landmarks."]
        #[serde(
            rename = "landmarks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub landmarks: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1DetectedLandmark>,
        >,
        #[doc = "Normalized Bounding box in a frame, where the object is located."]
        #[serde(
            rename = "normalizedBoundingBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_bounding_box: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P1Beta1NormalizedBoundingBox,
        >,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this object."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1TimestampedObject
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1TimestampedObject
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1Track {
        #[doc = "Optional. Attributes in the track level."]
        #[serde(
            rename = "attributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1DetectedAttribute>,
        >,
        #[doc = "Optional. The confidence score of the tracked object."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Video segment of a track."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoSegment,
        >,
        #[doc = "The object with timestamp and attributes per frame in the track."]
        #[serde(
            rename = "timestampedObjects",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamped_objects: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1TimestampedObject>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1Track {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P1Beta1Track {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgress {
        #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
        #[serde(
            rename = "feature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feature: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature,
        >,
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(
            rename = "inputUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_uri: ::std::option::Option<String>,
        #[doc = "Approximate percentage processed thus far. Guaranteed to be 100 when fully processed."]
        #[serde(
            rename = "progressPercent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_percent: ::std::option::Option<i32>,
        #[doc = "Specifies which segment is being tracked if the request contains more than one segment."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoSegment,
        >,
        #[doc = "Time when the request was received."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Time of the most recent update."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgress
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgress
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature {
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[doc = "Human face detection."]
        FaceDetection,
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[doc = "Logo detection, tracking, and recognition."]
        LogoRecognition,
        #[doc = "Object detection and tracking."]
        ObjectTracking,
        #[doc = "Person detection."]
        PersonDetection,
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[doc = "OCR text detection and tracking."]
        TextDetection,
    }
    impl GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: ExplicitContentDetection => "EXPLICIT_CONTENT_DETECTION" , GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: FaceDetection => "FACE_DETECTION" , GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: FeatureUnspecified => "FEATURE_UNSPECIFIED" , GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: LabelDetection => "LABEL_DETECTION" , GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: LogoRecognition => "LOGO_RECOGNITION" , GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: ObjectTracking => "OBJECT_TRACKING" , GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: PersonDetection => "PERSON_DETECTION" , GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: ShotChangeDetection => "SHOT_CHANGE_DETECTION" , GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: SpeechTranscription => "SPEECH_TRANSCRIPTION" , GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: TextDetection => "TEXT_DETECTION" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature,
            (),
        > {
            Ok (match s { "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: ExplicitContentDetection , "FACE_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: FaceDetection , "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: LabelDetection , "LOGO_RECOGNITION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: LogoRecognition , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: ObjectTracking , "PERSON_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: PersonDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: ShotChangeDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: TextDetection , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: ExplicitContentDetection , "FACE_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: FaceDetection , "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: LabelDetection , "LOGO_RECOGNITION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: LogoRecognition , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: ObjectTracking , "PERSON_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: PersonDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: ShotChangeDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature :: TextDetection , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationProgressFeature
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationResults {
        #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest` some videos may succeed and some may fail."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Explicit content annotation."]
        #[serde(
            rename = "explicitAnnotation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub explicit_annotation: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P1Beta1ExplicitContentAnnotation,
        >,
        #[doc = "Deprecated. Please use `face_detection_annotations` instead."]
        #[serde(
            rename = "faceAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub face_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1FaceAnnotation>,
        >,
        #[doc = "Face detection annotations."]
        #[serde(
            rename = "faceDetectionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub face_detection_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1FaceDetectionAnnotation>,
        >,
        #[doc = "Label annotations on frame level. There is exactly one element for each unique label."]
        #[serde(
            rename = "frameLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frame_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1LabelAnnotation>,
        >,
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(
            rename = "inputUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_uri: ::std::option::Option<String>,
        #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
        #[serde(
            rename = "logoRecognitionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logo_recognition_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1LogoRecognitionAnnotation>,
        >,
        #[doc = "Annotations for list of objects detected and tracked in video."]
        #[serde(
            rename = "objectAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1ObjectTrackingAnnotation>,
        >,
        #[doc = "Person detection annotations."]
        #[serde(
            rename = "personDetectionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person_detection_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1PersonDetectionAnnotation>,
        >,
        #[doc = "Video segment on which the annotation is run."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoSegment,
        >,
        #[doc = "Topical label annotations on video level or user-specified segment level. There is exactly one element for each unique label."]
        #[serde(
            rename = "segmentLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1LabelAnnotation>,
        >,
        #[doc = "Presence label annotations on video level or user-specified segment level. There is exactly one element for each unique label. Compared to the existing topical `segment_label_annotations`, this field presents more fine-grained, segment-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to “builtin/latest” in the request."]
        #[serde(
            rename = "segmentPresenceLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_presence_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1LabelAnnotation>,
        >,
        #[doc = "Shot annotations. Each shot is represented as a video segment."]
        #[serde(
            rename = "shotAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1VideoSegment>,
        >,
        #[doc = "Topical label annotations on shot level. There is exactly one element for each unique label."]
        #[serde(
            rename = "shotLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1LabelAnnotation>,
        >,
        #[doc = "Presence label annotations on shot level. There is exactly one element for each unique label. Compared to the existing topical `shot_label_annotations`, this field presents more fine-grained, shot-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to “builtin/latest” in the request."]
        #[serde(
            rename = "shotPresenceLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_presence_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1LabelAnnotation>,
        >,
        #[doc = "Speech transcription."]
        #[serde(
            rename = "speechTranscriptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub speech_transcriptions: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1SpeechTranscription>,
        >,
        #[doc = "OCR text detection and tracking. Annotations for list of detected text snippets. Each will have list of frame information associated with it."]
        #[serde(
            rename = "textAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P1Beta1TextAnnotation>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationResults
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P1Beta1VideoAnnotationResults
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
    pub struct GoogleCloudVideointelligenceV1P1Beta1VideoSegment {
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive)."]
        #[serde(
            rename = "endTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time_offset: ::std::option::Option<String>,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive)."]
        #[serde(
            rename = "startTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1VideoSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P1Beta1VideoSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P1Beta1WordInfo {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from 1 up to diarization_speaker_count, and is only set if speaker diarization is enabled."]
        #[serde(
            rename = "speakerTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub speaker_tag: ::std::option::Option<i32>,
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the start of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "The word corresponding to this set of information."]
        #[serde(
            rename = "word",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub word: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P1Beta1WordInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P1Beta1WordInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoProgress {
        #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
        #[serde(
            rename = "annotationProgress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotation_progress: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgress>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoProgress
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoProgress
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequest { # [doc = "Required. Requested video annotation features."] # [serde (rename = "features" , default , skip_serializing_if = "std::option::Option::is_none")] pub features : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems > > , # [doc = "The video data bytes. If unset, the input video(s) should be specified via the `input_uri`. If set, `input_uri` must be unset."] # [serde (rename = "inputContent" , default , skip_serializing_if = "std::option::Option::is_none")] pub input_content : :: std :: option :: Option < :: google_api_bytes :: Bytes > , # [doc = "Input video location. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. URIs must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints). To identify multiple videos, a video URI may include wildcards in the `object-id`. Supported wildcards: ‘\\*’ to match 0 or more characters; ‘?’ to match 1 character. If unset, the input video should be embedded in the request as `input_content`. If set, `input_content` must be unset."] # [serde (rename = "inputUri" , default , skip_serializing_if = "std::option::Option::is_none")] pub input_uri : :: std :: option :: Option < String > , # [doc = "Optional. Cloud region where annotation should take place. Supported cloud regions are: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region is specified, the region will be determined based on video file location."] # [serde (rename = "locationId" , default , skip_serializing_if = "std::option::Option::is_none")] pub location_id : :: std :: option :: Option < String > , # [doc = "Optional. Location where the output (in JSON format) should be stored. Currently, only [Cloud Storage](https://cloud.google.com/storage/) URIs are supported. These must be specified in the following format: `gs://bucket-id/object-id` (other URI formats return google.rpc.Code.INVALID_ARGUMENT). For more information, see [Request URIs](https://cloud.google.com/storage/docs/request-endpoints)."] # [serde (rename = "outputUri" , default , skip_serializing_if = "std::option::Option::is_none")] pub output_uri : :: std :: option :: Option < String > , # [doc = "Additional video context and/or feature-specific parameters."] # [serde (rename = "videoContext" , default , skip_serializing_if = "std::option::Option::is_none")] pub video_context : :: std :: option :: Option < crate :: schemas :: GoogleCloudVideointelligenceV1P2Beta1VideoContext > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems {
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[doc = "Human face detection."]
        FaceDetection,
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[doc = "Logo detection, tracking, and recognition."]
        LogoRecognition,
        #[doc = "Object detection and tracking."]
        ObjectTracking,
        #[doc = "Person detection."]
        PersonDetection,
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[doc = "OCR text detection and tracking."]
        TextDetection,
    }
    impl GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: ExplicitContentDetection => "EXPLICIT_CONTENT_DETECTION" , GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: FaceDetection => "FACE_DETECTION" , GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: FeatureUnspecified => "FEATURE_UNSPECIFIED" , GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: LabelDetection => "LABEL_DETECTION" , GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: LogoRecognition => "LOGO_RECOGNITION" , GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: ObjectTracking => "OBJECT_TRACKING" , GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: PersonDetection => "PERSON_DETECTION" , GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: ShotChangeDetection => "SHOT_CHANGE_DETECTION" , GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: SpeechTranscription => "SPEECH_TRANSCRIPTION" , GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: TextDetection => "TEXT_DETECTION" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems,
            (),
        > {
            Ok (match s { "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: ExplicitContentDetection , "FACE_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: FaceDetection , "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: LabelDetection , "LOGO_RECOGNITION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: LogoRecognition , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: ObjectTracking , "PERSON_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: PersonDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: ShotChangeDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: TextDetection , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: ExplicitContentDetection , "FACE_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: FaceDetection , "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: LabelDetection , "LOGO_RECOGNITION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: LogoRecognition , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: ObjectTracking , "PERSON_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: PersonDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: ShotChangeDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems :: TextDetection , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequestFeaturesItems
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoResponse {
        #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
        #[serde(
            rename = "annotationResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotation_results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationResults>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1DetectedAttribute {
        #[doc = "Detected attribute confidence. Range \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The name of the attribute, for example, glasses, dark_glasses, mouth_open. A full list of supported type names will be provided in the document."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Text value of the detection result. For example, the value for “HairColor” can be “black”, “blonde”, etc."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1DetectedAttribute
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1DetectedAttribute
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1DetectedLandmark {
        #[doc = "The confidence score of the detected landmark. Range \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The name of this landmark, for example, left_hand, right_shoulder."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The 2D point of the detected landmark using the normalized image coordindate system. The normalized coordinates have the range from 0 to 1."]
        #[serde(
            rename = "point",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub point: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1NormalizedVertex,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1DetectedLandmark
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1DetectedLandmark
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1Entity {
        #[doc = "Textual description, e.g., `Fixed-gear bicycle`."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
        #[serde(
            rename = "entityId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_id: ::std::option::Option<String>,
        #[doc = "Language code for `description` in BCP-47 format."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1Entity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P2Beta1Entity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1ExplicitContentAnnotation {
        #[doc = "All video frames where explicit content was detected."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFrame>,
        >,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentAnnotation
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1ExplicitContentDetectionConfig {
        #[doc = "Model to use for explicit content detection. Supported values: “builtin/stable” (the default if unset) and “builtin/latest”."]
        #[serde(
            rename = "model",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentDetectionConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentDetectionConfig
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFrame { # [doc = "Likelihood of the pornography content.."] # [serde (rename = "pornographyLikelihood" , default , skip_serializing_if = "std::option::Option::is_none")] pub pornography_likelihood : :: std :: option :: Option < crate :: schemas :: GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood > , # [doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."] # [serde (rename = "timeOffset" , default , skip_serializing_if = "std::option::Option::is_none")] pub time_offset : :: std :: option :: Option < String > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFrame
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFrame
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood {
        #[doc = "Unspecified likelihood."]
        LikelihoodUnspecified,
        #[doc = "Likely."]
        Likely,
        #[doc = "Possible."]
        Possible,
        #[doc = "Unlikely."]
        Unlikely,
        #[doc = "Very likely."]
        VeryLikely,
        #[doc = "Very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified => "LIKELIHOOD_UNSPECIFIED" , GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: Likely => "LIKELY" , GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: Possible => "POSSIBLE" , GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: Unlikely => "UNLIKELY" , GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: VeryLikely => "VERY_LIKELY" , GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: VeryUnlikely => "VERY_UNLIKELY" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood,
            (),
        > {
            Ok (match s { "LIKELIHOOD_UNSPECIFIED" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified , "LIKELY" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: Likely , "POSSIBLE" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: Possible , "UNLIKELY" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: Unlikely , "VERY_LIKELY" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: VeryLikely , "VERY_UNLIKELY" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: VeryUnlikely , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "LIKELIHOOD_UNSPECIFIED" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified , "LIKELY" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: Likely , "POSSIBLE" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: Possible , "UNLIKELY" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: Unlikely , "VERY_LIKELY" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: VeryLikely , "VERY_UNLIKELY" => GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood :: VeryUnlikely , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1ExplicitContentFramePornographyLikelihood
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1FaceAnnotation {
        #[doc = "All video frames where a face was detected."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1FaceFrame>,
        >,
        #[doc = "All video segments where a face was detected."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1FaceSegment>,
        >,
        #[doc = "Thumbnail of a representative face view (in JPEG format)."]
        #[serde(
            rename = "thumbnail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1FaceAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P2Beta1FaceAnnotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1FaceDetectionAnnotation {
        #[doc = "The thumbnail of a person’s face."]
        #[serde(
            rename = "thumbnail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The face tracks with attributes."]
        #[serde(
            rename = "tracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracks:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1Track>>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1FaceDetectionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1FaceDetectionAnnotation
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1FaceDetectionConfig {
        #[doc = "Whether to enable face attributes detection, such as glasses, dark_glasses, mouth_open etc. Ignored if ‘include_bounding_boxes’ is set to false."]
        #[serde(
            rename = "includeAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_attributes: ::std::option::Option<bool>,
        #[doc = "Whether bounding boxes are included in the face annotation output."]
        #[serde(
            rename = "includeBoundingBoxes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_bounding_boxes: ::std::option::Option<bool>,
        #[doc = "Model to use for face detection. Supported values: “builtin/stable” (the default if unset) and “builtin/latest”."]
        #[serde(
            rename = "model",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1FaceDetectionConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1FaceDetectionConfig
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1FaceFrame {
        #[doc = "Normalized Bounding boxes in a frame. There can be more than one boxes if the same face is detected in multiple locations within the current frame."]
        #[serde(
            rename = "normalizedBoundingBoxes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_bounding_boxes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingBox>,
        >,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1FaceFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P2Beta1FaceFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1FaceSegment {
        #[doc = "Video segment where a face was detected."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1FaceSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P2Beta1FaceSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1LabelAnnotation {
        #[doc = "Common categories for the detected entity. For example, when the label is `Terrier`, the category is likely `dog`. And in some cases there might be more than one categories e.g., `Terrier` could also be a `pet`."]
        #[serde(
            rename = "categoryEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category_entities:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1Entity>>,
        #[doc = "Detected entity."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1Entity>,
        #[doc = "All video frames where a label was detected."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1LabelFrame>,
        >,
        #[doc = "All video segments where a label was detected."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1LabelSegment>,
        >,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1LabelAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P2Beta1LabelAnnotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfig { # [doc = "The confidence threshold we perform filtering on the labels from frame-level detection. If not set, it is set to 0.4 by default. The valid range for this threshold is \\[0.1, 0.9\\]. Any value set outside of this range will be clipped. Note: For best results, follow the default threshold. We will update the default threshold everytime when we release a new model."] # [serde (rename = "frameConfidenceThreshold" , default , skip_serializing_if = "std::option::Option::is_none")] pub frame_confidence_threshold : :: std :: option :: Option < f32 > , # [doc = "What labels should be detected with LABEL_DETECTION, in addition to video-level labels or segment-level labels. If unspecified, defaults to `SHOT_MODE`."] # [serde (rename = "labelDetectionMode" , default , skip_serializing_if = "std::option::Option::is_none")] pub label_detection_mode : :: std :: option :: Option < crate :: schemas :: GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode > , # [doc = "Model to use for label detection. Supported values: “builtin/stable” (the default if unset) and “builtin/latest”."] # [serde (rename = "model" , default , skip_serializing_if = "std::option::Option::is_none")] pub model : :: std :: option :: Option < String > , # [doc = "Whether the video has been shot from a stationary (i.e., non-moving) camera. When set to true, might improve detection accuracy for moving objects. Should be used with `SHOT_AND_FRAME_MODE` enabled."] # [serde (rename = "stationaryCamera" , default , skip_serializing_if = "std::option::Option::is_none")] pub stationary_camera : :: std :: option :: Option < bool > , # [doc = "The confidence threshold we perform filtering on the labels from video-level and shot-level detections. If not set, it’s set to 0.3 by default. The valid range for this threshold is \\[0.1, 0.9\\]. Any value set outside of this range will be clipped. Note: For best results, follow the default threshold. We will update the default threshold everytime when we release a new model."] # [serde (rename = "videoConfidenceThreshold" , default , skip_serializing_if = "std::option::Option::is_none")] pub video_confidence_threshold : :: std :: option :: Option < f32 > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfig
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode {
        #[doc = "Detect frame-level labels."]
        FrameMode,
        #[doc = "Unspecified."]
        LabelDetectionModeUnspecified,
        #[doc = "Detect both shot-level and frame-level labels."]
        ShotAndFrameMode,
        #[doc = "Detect shot-level labels."]
        ShotMode,
    }
    impl GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: FrameMode => "FRAME_MODE" , GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: LabelDetectionModeUnspecified => "LABEL_DETECTION_MODE_UNSPECIFIED" , GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: ShotAndFrameMode => "SHOT_AND_FRAME_MODE" , GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: ShotMode => "SHOT_MODE" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode,
            (),
        > {
            Ok (match s { "FRAME_MODE" => GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: FrameMode , "LABEL_DETECTION_MODE_UNSPECIFIED" => GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: LabelDetectionModeUnspecified , "SHOT_AND_FRAME_MODE" => GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: ShotAndFrameMode , "SHOT_MODE" => GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: ShotMode , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "FRAME_MODE" => GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: FrameMode , "LABEL_DETECTION_MODE_UNSPECIFIED" => GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: LabelDetectionModeUnspecified , "SHOT_AND_FRAME_MODE" => GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: ShotAndFrameMode , "SHOT_MODE" => GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode :: ShotMode , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfigLabelDetectionMode
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1LabelFrame {
        #[doc = "Confidence that the label is accurate. Range: \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1LabelFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P2Beta1LabelFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1LabelSegment {
        #[doc = "Confidence that the label is accurate. Range: \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Video segment where a label was detected."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1LabelSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P2Beta1LabelSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1LogoRecognitionAnnotation {
        #[doc = "Entity category information to specify the logo class that all the logo tracks within this LogoRecognitionAnnotation are recognized as."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1Entity>,
        #[doc = "All video segments where the recognized logo appears. There might be multiple instances of the same logo class appearing in one VideoSegment."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment>,
        >,
        #[doc = "All logo tracks where the recognized logo appears. Each track corresponds to one logo instance appearing in consecutive frames."]
        #[serde(
            rename = "tracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracks:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1Track>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1LogoRecognitionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1LogoRecognitionAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingBox {
        #[doc = "Bottom Y coordinate."]
        #[serde(
            rename = "bottom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bottom: ::std::option::Option<f32>,
        #[doc = "Left X coordinate."]
        #[serde(
            rename = "left",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left: ::std::option::Option<f32>,
        #[doc = "Right X coordinate."]
        #[serde(
            rename = "right",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub right: ::std::option::Option<f32>,
        #[doc = "Top Y coordinate."]
        #[serde(
            rename = "top",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingBox
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingBox
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingPoly {
        #[doc = "Normalized vertices of the bounding polygon."]
        #[serde(
            rename = "vertices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertices: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1NormalizedVertex>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingPoly
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingPoly
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(
            rename = "x",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(
            rename = "y",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1NormalizedVertex
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1NormalizedVertex
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingAnnotation {
        #[doc = "Object category’s labeling confidence of this track."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Entity to specify the object category that this track is labeled as."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1Entity>,
        #[doc = "Information corresponding to all frames where this object track appears. Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame messages in frames. Streaming mode: it can only be one ObjectTrackingFrame message in frames."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingFrame>,
        >,
        #[doc = "Non-streaming batch mode ONLY. Each object track corresponds to one video segment where it appears."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment,
        >,
        #[doc = "Streaming mode ONLY. In streaming mode, we do not know the end time of a tracked object before it is completed. Hence, there is no VideoSegment info returned. Instead, we provide a unique identifiable integer track_id so that the customers can correlate the results of the ongoing ObjectTrackAnnotation of the same track_id over time."]
        #[serde(
            rename = "trackId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub track_id: ::std::option::Option<i64>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingAnnotation
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingConfig {
        #[doc = "Model to use for object tracking. Supported values: “builtin/stable” (the default if unset) and “builtin/latest”."]
        #[serde(
            rename = "model",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingConfig
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingFrame {
        #[doc = "The normalized bounding box location of this object track for the frame."]
        #[serde(
            rename = "normalizedBoundingBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_bounding_box: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingBox,
        >,
        #[doc = "The timestamp of the frame in microseconds."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingFrame
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingFrame
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1PersonDetectionAnnotation {
        #[doc = "The detected tracks of a person."]
        #[serde(
            rename = "tracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracks:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1Track>>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1PersonDetectionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1PersonDetectionAnnotation
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1PersonDetectionConfig {
        #[doc = "Whether to enable person attributes detection, such as cloth color (black, blue, etc), type (coat, dress, etc), pattern (plain, floral, etc), hair, etc. Ignored if ‘include_bounding_boxes’ is set to false."]
        #[serde(
            rename = "includeAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_attributes: ::std::option::Option<bool>,
        #[doc = "Whether bounding boxes are included in the person detection annotation output."]
        #[serde(
            rename = "includeBoundingBoxes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_bounding_boxes: ::std::option::Option<bool>,
        #[doc = "Whether to enable pose landmarks detection. Ignored if ‘include_bounding_boxes’ is set to false."]
        #[serde(
            rename = "includePoseLandmarks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_pose_landmarks: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1PersonDetectionConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1PersonDetectionConfig
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1ShotChangeDetectionConfig {
        #[doc = "Model to use for shot change detection. Supported values: “builtin/stable” (the default if unset), “builtin/latest”, and “builtin/legacy”."]
        #[serde(
            rename = "model",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1ShotChangeDetectionConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1ShotChangeDetectionConfig
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1SpeechContext {
        #[doc = "Optional. A list of strings containing words and phrases “hints” so that the speech recognition is more likely to recognize them. This can be used to improve the accuracy for specific words and phrases, for example, if specific commands are typically spoken by the user. This can also be used to add additional words to the vocabulary of the recognizer. See [usage limits](https://cloud.google.com/speech/limits#content)."]
        #[serde(
            rename = "phrases",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phrases: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1SpeechContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P2Beta1SpeechContext {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1SpeechRecognitionAlternative {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Transcript text representing the words that the user spoke."]
        #[serde(
            rename = "transcript",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transcript: ::std::option::Option<String>,
        #[doc = "Output only. A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is set to true, you will see all the words from the beginning of the audio."]
        #[serde(
            rename = "words",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub words: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1WordInfo>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1SpeechRecognitionAlternative
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1SpeechRecognitionAlternative
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1SpeechTranscription {
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
        #[serde(
            rename = "alternatives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alternatives: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1SpeechRecognitionAlternative>,
        >,
        #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1SpeechTranscription
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1SpeechTranscription
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
    pub struct GoogleCloudVideointelligenceV1P2Beta1SpeechTranscriptionConfig {
        #[doc = "Optional. For file formats, such as MXF or MKV, supporting multiple audio tracks, specify up to two tracks. Default: track 0."]
        #[serde(
            rename = "audioTracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audio_tracks: ::std::option::Option<Vec<i32>>,
        #[doc = "Optional. If set, specifies the estimated number of speakers in the conversation. If not set, defaults to ‘2’. Ignored unless enable_speaker_diarization is set to true."]
        #[serde(
            rename = "diarizationSpeakerCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub diarization_speaker_count: ::std::option::Option<i32>,
        #[doc = "Optional. If ‘true’, adds punctuation to recognition result hypotheses. This feature is only available in select languages. Setting this for requests in other languages has no effect at all. The default ‘false’ value does not add punctuation to result hypotheses. NOTE: “This is currently offered as an experimental service, complimentary to all users. In the future this may be exclusively available as a premium feature.”"]
        #[serde(
            rename = "enableAutomaticPunctuation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_automatic_punctuation: ::std::option::Option<bool>,
        #[doc = "Optional. If ‘true’, enables speaker detection for each recognized word in the top alternative of the recognition result using a speaker_tag provided in the WordInfo. Note: When this is true, we send all the words from the beginning of the audio for the top alternative in every consecutive response. This is done in order to improve our speaker tags as our models learn to identify the speakers in the conversation over time."]
        #[serde(
            rename = "enableSpeakerDiarization",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_speaker_diarization: ::std::option::Option<bool>,
        #[doc = "Optional. If `true`, the top result includes a list of words and the confidence for those words. If `false`, no word-level confidence information is returned. The default is `false`."]
        #[serde(
            rename = "enableWordConfidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_word_confidence: ::std::option::Option<bool>,
        #[doc = "Optional. If set to `true`, the server will attempt to filter out profanities, replacing all but the initial character in each filtered word with asterisks, e.g. “f\\*\\*\\*”. If set to `false` or omitted, profanities won’t be filtered out."]
        #[serde(
            rename = "filterProfanity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter_profanity: ::std::option::Option<bool>,
        #[doc = "Required. *Required* The language of the supplied audio as a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: “en-US”. See [Language Support](https://cloud.google.com/speech/docs/languages) for a list of the currently supported language codes."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Optional. Maximum number of recognition hypotheses to be returned. Specifically, the maximum number of `SpeechRecognitionAlternative` messages within each `SpeechTranscription`. The server may return fewer than `max_alternatives`. Valid values are `0`-`30`. A value of `0` or `1` will return a maximum of one. If omitted, will return a maximum of one."]
        #[serde(
            rename = "maxAlternatives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_alternatives: ::std::option::Option<i32>,
        #[doc = "Optional. A means to provide context to assist the speech recognition."]
        #[serde(
            rename = "speechContexts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub speech_contexts: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1SpeechContext>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1SpeechTranscriptionConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1SpeechTranscriptionConfig
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1TextAnnotation {
        #[doc = "All video segments where OCR detected text appears."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1TextSegment>,
        >,
        #[doc = "The detected text."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1TextAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P2Beta1TextAnnotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1TextDetectionConfig {
        #[doc = "Language hint can be specified if the language to be detected is known a priori. It can increase the accuracy of the detection. Language hint must be language code in BCP-47 format. Automatic language detection is performed if no hint is provided."]
        #[serde(
            rename = "languageHints",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_hints: ::std::option::Option<Vec<String>>,
        #[doc = "Model to use for text detection. Supported values: “builtin/stable” (the default if unset) and “builtin/latest”."]
        #[serde(
            rename = "model",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1TextDetectionConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1TextDetectionConfig
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1TextFrame {
        #[doc = "Bounding polygon of the detected text for this frame."]
        #[serde(
            rename = "rotatedBoundingBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rotated_bounding_box: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingPoly,
        >,
        #[doc = "Timestamp of this frame."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1TextFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P2Beta1TextFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1TextSegment {
        #[doc = "Confidence for the track of detected text. It is calculated as the highest over all frames where OCR detected text appears."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Information related to the frames where OCR detected text appears."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1TextFrame>,
        >,
        #[doc = "Video segment where a text snippet was detected."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1TextSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P2Beta1TextSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1TimestampedObject {
        #[doc = "Optional. The attributes of the object in the bounding box."]
        #[serde(
            rename = "attributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1DetectedAttribute>,
        >,
        #[doc = "Optional. The detected landmarks."]
        #[serde(
            rename = "landmarks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub landmarks: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1DetectedLandmark>,
        >,
        #[doc = "Normalized Bounding box in a frame, where the object is located."]
        #[serde(
            rename = "normalizedBoundingBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_bounding_box: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1NormalizedBoundingBox,
        >,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this object."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1TimestampedObject
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1TimestampedObject
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1Track {
        #[doc = "Optional. Attributes in the track level."]
        #[serde(
            rename = "attributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1DetectedAttribute>,
        >,
        #[doc = "Optional. The confidence score of the tracked object."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Video segment of a track."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment,
        >,
        #[doc = "The object with timestamp and attributes per frame in the track."]
        #[serde(
            rename = "timestampedObjects",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamped_objects: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1TimestampedObject>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1Track {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P2Beta1Track {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgress {
        #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
        #[serde(
            rename = "feature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feature: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature,
        >,
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(
            rename = "inputUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_uri: ::std::option::Option<String>,
        #[doc = "Approximate percentage processed thus far. Guaranteed to be 100 when fully processed."]
        #[serde(
            rename = "progressPercent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_percent: ::std::option::Option<i32>,
        #[doc = "Specifies which segment is being tracked if the request contains more than one segment."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment,
        >,
        #[doc = "Time when the request was received."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Time of the most recent update."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgress
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgress
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature {
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[doc = "Human face detection."]
        FaceDetection,
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[doc = "Logo detection, tracking, and recognition."]
        LogoRecognition,
        #[doc = "Object detection and tracking."]
        ObjectTracking,
        #[doc = "Person detection."]
        PersonDetection,
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[doc = "OCR text detection and tracking."]
        TextDetection,
    }
    impl GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: ExplicitContentDetection => "EXPLICIT_CONTENT_DETECTION" , GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: FaceDetection => "FACE_DETECTION" , GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: FeatureUnspecified => "FEATURE_UNSPECIFIED" , GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: LabelDetection => "LABEL_DETECTION" , GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: LogoRecognition => "LOGO_RECOGNITION" , GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: ObjectTracking => "OBJECT_TRACKING" , GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: PersonDetection => "PERSON_DETECTION" , GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: ShotChangeDetection => "SHOT_CHANGE_DETECTION" , GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: SpeechTranscription => "SPEECH_TRANSCRIPTION" , GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: TextDetection => "TEXT_DETECTION" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature,
            (),
        > {
            Ok (match s { "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: ExplicitContentDetection , "FACE_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: FaceDetection , "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: LabelDetection , "LOGO_RECOGNITION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: LogoRecognition , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: ObjectTracking , "PERSON_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: PersonDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: ShotChangeDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: TextDetection , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: ExplicitContentDetection , "FACE_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: FaceDetection , "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: LabelDetection , "LOGO_RECOGNITION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: LogoRecognition , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: ObjectTracking , "PERSON_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: PersonDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: ShotChangeDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature :: TextDetection , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationProgressFeature
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationResults {
        #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest` some videos may succeed and some may fail."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Explicit content annotation."]
        #[serde(
            rename = "explicitAnnotation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub explicit_annotation: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1ExplicitContentAnnotation,
        >,
        #[doc = "Deprecated. Please use `face_detection_annotations` instead."]
        #[serde(
            rename = "faceAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub face_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1FaceAnnotation>,
        >,
        #[doc = "Face detection annotations."]
        #[serde(
            rename = "faceDetectionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub face_detection_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1FaceDetectionAnnotation>,
        >,
        #[doc = "Label annotations on frame level. There is exactly one element for each unique label."]
        #[serde(
            rename = "frameLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frame_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1LabelAnnotation>,
        >,
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(
            rename = "inputUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_uri: ::std::option::Option<String>,
        #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
        #[serde(
            rename = "logoRecognitionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logo_recognition_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1LogoRecognitionAnnotation>,
        >,
        #[doc = "Annotations for list of objects detected and tracked in video."]
        #[serde(
            rename = "objectAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingAnnotation>,
        >,
        #[doc = "Person detection annotations."]
        #[serde(
            rename = "personDetectionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person_detection_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1PersonDetectionAnnotation>,
        >,
        #[doc = "Video segment on which the annotation is run."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment,
        >,
        #[doc = "Topical label annotations on video level or user-specified segment level. There is exactly one element for each unique label."]
        #[serde(
            rename = "segmentLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1LabelAnnotation>,
        >,
        #[doc = "Presence label annotations on video level or user-specified segment level. There is exactly one element for each unique label. Compared to the existing topical `segment_label_annotations`, this field presents more fine-grained, segment-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to “builtin/latest” in the request."]
        #[serde(
            rename = "segmentPresenceLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_presence_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1LabelAnnotation>,
        >,
        #[doc = "Shot annotations. Each shot is represented as a video segment."]
        #[serde(
            rename = "shotAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment>,
        >,
        #[doc = "Topical label annotations on shot level. There is exactly one element for each unique label."]
        #[serde(
            rename = "shotLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1LabelAnnotation>,
        >,
        #[doc = "Presence label annotations on shot level. There is exactly one element for each unique label. Compared to the existing topical `shot_label_annotations`, this field presents more fine-grained, shot-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to “builtin/latest” in the request."]
        #[serde(
            rename = "shotPresenceLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_presence_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1LabelAnnotation>,
        >,
        #[doc = "Speech transcription."]
        #[serde(
            rename = "speechTranscriptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub speech_transcriptions: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1SpeechTranscription>,
        >,
        #[doc = "OCR text detection and tracking. Annotations for list of detected text snippets. Each will have list of frame information associated with it."]
        #[serde(
            rename = "textAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1TextAnnotation>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationResults
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P2Beta1VideoAnnotationResults
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1VideoContext {
        #[doc = "Config for EXPLICIT_CONTENT_DETECTION."]
        #[serde(
            rename = "explicitContentDetectionConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub explicit_content_detection_config: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1ExplicitContentDetectionConfig,
        >,
        #[doc = "Config for FACE_DETECTION."]
        #[serde(
            rename = "faceDetectionConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub face_detection_config: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1FaceDetectionConfig,
        >,
        #[doc = "Config for LABEL_DETECTION."]
        #[serde(
            rename = "labelDetectionConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label_detection_config: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1LabelDetectionConfig,
        >,
        #[doc = "Config for OBJECT_TRACKING."]
        #[serde(
            rename = "objectTrackingConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_tracking_config: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1ObjectTrackingConfig,
        >,
        #[doc = "Config for PERSON_DETECTION."]
        #[serde(
            rename = "personDetectionConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person_detection_config: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1PersonDetectionConfig,
        >,
        #[doc = "Video segments to annotate. The segments may overlap and are not required to be contiguous or span the whole video. If unspecified, each video is treated as a single segment."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P2Beta1VideoSegment>,
        >,
        #[doc = "Config for SHOT_CHANGE_DETECTION."]
        #[serde(
            rename = "shotChangeDetectionConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_change_detection_config: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1ShotChangeDetectionConfig,
        >,
        #[doc = "Config for SPEECH_TRANSCRIPTION."]
        #[serde(
            rename = "speechTranscriptionConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub speech_transcription_config: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1SpeechTranscriptionConfig,
        >,
        #[doc = "Config for TEXT_DETECTION."]
        #[serde(
            rename = "textDetectionConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_detection_config: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P2Beta1TextDetectionConfig,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1VideoContext {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P2Beta1VideoContext {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1VideoSegment {
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive)."]
        #[serde(
            rename = "endTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time_offset: ::std::option::Option<String>,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive)."]
        #[serde(
            rename = "startTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1VideoSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P2Beta1VideoSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P2Beta1WordInfo {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from 1 up to diarization_speaker_count, and is only set if speaker diarization is enabled."]
        #[serde(
            rename = "speakerTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub speaker_tag: ::std::option::Option<i32>,
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the start of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "The word corresponding to this set of information."]
        #[serde(
            rename = "word",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub word: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P2Beta1WordInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P2Beta1WordInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1AnnotateVideoProgress {
        #[doc = "Progress metadata for all videos specified in `AnnotateVideoRequest`."]
        #[serde(
            rename = "annotationProgress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotation_progress: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgress>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1AnnotateVideoProgress
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1AnnotateVideoProgress
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVideointelligenceV1P3Beta1AnnotateVideoResponse {
        #[doc = "Annotation results for all videos specified in `AnnotateVideoRequest`."]
        #[serde(
            rename = "annotationResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotation_results: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationResults>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1AnnotateVideoResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1AnnotateVideoResponse
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1Celebrity {
        #[doc = "Textual description of additional information about the celebrity, if applicable."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The celebrity name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The resource name of the celebrity. Have the format `video-intelligence/kg-mid` indicates a celebrity from preloaded gallery. kg-mid is the id in Google knowledge graph, which is unique for the celebrity."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1Celebrity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P3Beta1Celebrity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1CelebrityRecognitionAnnotation {
        #[doc = "The tracks detected from the input video, including recognized celebrities and other detected faces in the video."]
        #[serde(
            rename = "celebrityTracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub celebrity_tracks: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1CelebrityTrack>,
        >,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1CelebrityRecognitionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1CelebrityRecognitionAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1CelebrityTrack {
        #[doc = "Top N match of the celebrities for the face in this track."]
        #[serde(
            rename = "celebrities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub celebrities: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1RecognizedCelebrity>,
        >,
        #[doc = "A track of a person’s face."]
        #[serde(
            rename = "faceTrack",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub face_track:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1Track>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1CelebrityTrack
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P3Beta1CelebrityTrack {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1DetectedAttribute {
        #[doc = "Detected attribute confidence. Range \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The name of the attribute, for example, glasses, dark_glasses, mouth_open. A full list of supported type names will be provided in the document."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Text value of the detection result. For example, the value for “HairColor” can be “black”, “blonde”, etc."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1DetectedAttribute
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1DetectedAttribute
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1DetectedLandmark {
        #[doc = "The confidence score of the detected landmark. Range \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "The name of this landmark, for example, left_hand, right_shoulder."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The 2D point of the detected landmark using the normalized image coordindate system. The normalized coordinates have the range from 0 to 1."]
        #[serde(
            rename = "point",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub point: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1NormalizedVertex,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1DetectedLandmark
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1DetectedLandmark
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1Entity {
        #[doc = "Textual description, e.g., `Fixed-gear bicycle`."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Opaque entity ID. Some IDs may be available in [Google Knowledge Graph Search API](https://developers.google.com/knowledge-graph/)."]
        #[serde(
            rename = "entityId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity_id: ::std::option::Option<String>,
        #[doc = "Language code for `description` in BCP-47 format."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1Entity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P3Beta1Entity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1ExplicitContentAnnotation {
        #[doc = "All video frames where explicit content was detected."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFrame>,
        >,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentAnnotation
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFrame { # [doc = "Likelihood of the pornography content.."] # [serde (rename = "pornographyLikelihood" , default , skip_serializing_if = "std::option::Option::is_none")] pub pornography_likelihood : :: std :: option :: Option < crate :: schemas :: GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood > , # [doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."] # [serde (rename = "timeOffset" , default , skip_serializing_if = "std::option::Option::is_none")] pub time_offset : :: std :: option :: Option < String > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFrame
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFrame
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood {
        #[doc = "Unspecified likelihood."]
        LikelihoodUnspecified,
        #[doc = "Likely."]
        Likely,
        #[doc = "Possible."]
        Possible,
        #[doc = "Unlikely."]
        Unlikely,
        #[doc = "Very likely."]
        VeryLikely,
        #[doc = "Very unlikely."]
        VeryUnlikely,
    }
    impl GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified => "LIKELIHOOD_UNSPECIFIED" , GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: Likely => "LIKELY" , GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: Possible => "POSSIBLE" , GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: Unlikely => "UNLIKELY" , GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: VeryLikely => "VERY_LIKELY" , GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: VeryUnlikely => "VERY_UNLIKELY" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood,
            (),
        > {
            Ok (match s { "LIKELIHOOD_UNSPECIFIED" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified , "LIKELY" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: Likely , "POSSIBLE" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: Possible , "UNLIKELY" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: Unlikely , "VERY_LIKELY" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: VeryLikely , "VERY_UNLIKELY" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: VeryUnlikely , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "LIKELIHOOD_UNSPECIFIED" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: LikelihoodUnspecified , "LIKELY" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: Likely , "POSSIBLE" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: Possible , "UNLIKELY" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: Unlikely , "VERY_LIKELY" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: VeryLikely , "VERY_UNLIKELY" => GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood :: VeryUnlikely , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1ExplicitContentFramePornographyLikelihood
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1FaceAnnotation {
        #[doc = "All video frames where a face was detected."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1FaceFrame>,
        >,
        #[doc = "All video segments where a face was detected."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1FaceSegment>,
        >,
        #[doc = "Thumbnail of a representative face view (in JPEG format)."]
        #[serde(
            rename = "thumbnail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1FaceAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P3Beta1FaceAnnotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1FaceDetectionAnnotation {
        #[doc = "The thumbnail of a person’s face."]
        #[serde(
            rename = "thumbnail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "The face tracks with attributes."]
        #[serde(
            rename = "tracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracks:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1Track>>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1FaceDetectionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1FaceDetectionAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1FaceFrame {
        #[doc = "Normalized Bounding boxes in a frame. There can be more than one boxes if the same face is detected in multiple locations within the current frame."]
        #[serde(
            rename = "normalizedBoundingBoxes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_bounding_boxes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingBox>,
        >,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1FaceFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P3Beta1FaceFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1FaceSegment {
        #[doc = "Video segment where a face was detected."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1FaceSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P3Beta1FaceSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1LabelAnnotation {
        #[doc = "Common categories for the detected entity. For example, when the label is `Terrier`, the category is likely `dog`. And in some cases there might be more than one categories e.g., `Terrier` could also be a `pet`."]
        #[serde(
            rename = "categoryEntities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category_entities:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1Entity>>,
        #[doc = "Detected entity."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1Entity>,
        #[doc = "All video frames where a label was detected."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LabelFrame>,
        >,
        #[doc = "All video segments where a label was detected."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LabelSegment>,
        >,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1LabelAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P3Beta1LabelAnnotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1LabelFrame {
        #[doc = "Confidence that the label is accurate. Range: \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this location."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1LabelFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P3Beta1LabelFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1LabelSegment {
        #[doc = "Confidence that the label is accurate. Range: \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Video segment where a label was detected."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1LabelSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P3Beta1LabelSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1LogoRecognitionAnnotation {
        #[doc = "Entity category information to specify the logo class that all the logo tracks within this LogoRecognitionAnnotation are recognized as."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1Entity>,
        #[doc = "All video segments where the recognized logo appears. There might be multiple instances of the same logo class appearing in one VideoSegment."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment>,
        >,
        #[doc = "All logo tracks where the recognized logo appears. Each track corresponds to one logo instance appearing in consecutive frames."]
        #[serde(
            rename = "tracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracks:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1Track>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1LogoRecognitionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1LogoRecognitionAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingBox {
        #[doc = "Bottom Y coordinate."]
        #[serde(
            rename = "bottom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bottom: ::std::option::Option<f32>,
        #[doc = "Left X coordinate."]
        #[serde(
            rename = "left",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left: ::std::option::Option<f32>,
        #[doc = "Right X coordinate."]
        #[serde(
            rename = "right",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub right: ::std::option::Option<f32>,
        #[doc = "Top Y coordinate."]
        #[serde(
            rename = "top",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingBox
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingBox
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingPoly {
        #[doc = "Normalized vertices of the bounding polygon."]
        #[serde(
            rename = "vertices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vertices: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1NormalizedVertex>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingPoly
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingPoly
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1NormalizedVertex {
        #[doc = "X coordinate."]
        #[serde(
            rename = "x",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x: ::std::option::Option<f32>,
        #[doc = "Y coordinate."]
        #[serde(
            rename = "y",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1NormalizedVertex
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1NormalizedVertex
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingAnnotation {
        #[doc = "Object category’s labeling confidence of this track."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Entity to specify the object category that this track is labeled as."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1Entity>,
        #[doc = "Information corresponding to all frames where this object track appears. Non-streaming batch mode: it may be one or multiple ObjectTrackingFrame messages in frames. Streaming mode: it can only be one ObjectTrackingFrame message in frames."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingFrame>,
        >,
        #[doc = "Non-streaming batch mode ONLY. Each object track corresponds to one video segment where it appears."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment,
        >,
        #[doc = "Streaming mode ONLY. In streaming mode, we do not know the end time of a tracked object before it is completed. Hence, there is no VideoSegment info returned. Instead, we provide a unique identifiable integer track_id so that the customers can correlate the results of the ongoing ObjectTrackAnnotation of the same track_id over time."]
        #[serde(
            rename = "trackId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub track_id: ::std::option::Option<i64>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingFrame {
        #[doc = "The normalized bounding box location of this object track for the frame."]
        #[serde(
            rename = "normalizedBoundingBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_bounding_box: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingBox,
        >,
        #[doc = "The timestamp of the frame in microseconds."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingFrame
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingFrame
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1PersonDetectionAnnotation {
        #[doc = "The detected tracks of a person."]
        #[serde(
            rename = "tracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracks:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1Track>>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1PersonDetectionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1PersonDetectionAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1RecognizedCelebrity {
        #[doc = "The recognized celebrity."]
        #[serde(
            rename = "celebrity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub celebrity:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1Celebrity>,
        #[doc = "Recognition confidence. Range \\[0, 1\\]."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1RecognizedCelebrity
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1RecognizedCelebrity
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1SpeechRecognitionAlternative {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Transcript text representing the words that the user spoke."]
        #[serde(
            rename = "transcript",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transcript: ::std::option::Option<String>,
        #[doc = "Output only. A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is set to true, you will see all the words from the beginning of the audio."]
        #[serde(
            rename = "words",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub words: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1WordInfo>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1SpeechRecognitionAlternative
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1SpeechRecognitionAlternative
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1SpeechTranscription {
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
        #[serde(
            rename = "alternatives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alternatives: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1SpeechRecognitionAlternative>,
        >,
        #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1SpeechTranscription
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1SpeechTranscription
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVideointelligenceV1P3Beta1StreamingAnnotateVideoResponse {
        #[doc = "Streaming annotation results."]
        #[serde(
            rename = "annotationResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotation_results: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1StreamingVideoAnnotationResults,
        >,
        #[doc = "Google Cloud Storage URI that stores annotation results of one streaming session in JSON format. It is the annotation_result_storage_directory from the request followed by ‘/cloud_project_number-session_id’."]
        #[serde(
            rename = "annotationResultsUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub annotation_results_uri: ::std::option::Option<String>,
        #[doc = "If set, returns a google.rpc.Status message that specifies the error for the operation."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1StreamingAnnotateVideoResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1StreamingAnnotateVideoResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1StreamingVideoAnnotationResults {
        #[doc = "Explicit content annotation results."]
        #[serde(
            rename = "explicitAnnotation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub explicit_annotation: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1ExplicitContentAnnotation,
        >,
        #[doc = "Timestamp of the processed frame in microseconds."]
        #[serde(
            rename = "frameTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frame_timestamp: ::std::option::Option<String>,
        #[doc = "Label annotation results."]
        #[serde(
            rename = "labelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LabelAnnotation>,
        >,
        #[doc = "Object tracking results."]
        #[serde(
            rename = "objectAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingAnnotation>,
        >,
        #[doc = "Shot annotation results. Each shot is represented as a video segment."]
        #[serde(
            rename = "shotAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1StreamingVideoAnnotationResults
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1StreamingVideoAnnotationResults
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1TextAnnotation {
        #[doc = "All video segments where OCR detected text appears."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1TextSegment>,
        >,
        #[doc = "The detected text."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1TextAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P3Beta1TextAnnotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1TextFrame {
        #[doc = "Bounding polygon of the detected text for this frame."]
        #[serde(
            rename = "rotatedBoundingBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rotated_bounding_box: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingPoly,
        >,
        #[doc = "Timestamp of this frame."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1TextFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P3Beta1TextFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1TextSegment {
        #[doc = "Confidence for the track of detected text. It is calculated as the highest over all frames where OCR detected text appears."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Information related to the frames where OCR detected text appears."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1TextFrame>,
        >,
        #[doc = "Video segment where a text snippet was detected."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1TextSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P3Beta1TextSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1TimestampedObject {
        #[doc = "Optional. The attributes of the object in the bounding box."]
        #[serde(
            rename = "attributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1DetectedAttribute>,
        >,
        #[doc = "Optional. The detected landmarks."]
        #[serde(
            rename = "landmarks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub landmarks: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1DetectedLandmark>,
        >,
        #[doc = "Normalized Bounding box in a frame, where the object is located."]
        #[serde(
            rename = "normalizedBoundingBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_bounding_box: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1NormalizedBoundingBox,
        >,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this object."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1TimestampedObject
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1TimestampedObject
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1Track {
        #[doc = "Optional. Attributes in the track level."]
        #[serde(
            rename = "attributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1DetectedAttribute>,
        >,
        #[doc = "Optional. The confidence score of the tracked object."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Video segment of a track."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment,
        >,
        #[doc = "The object with timestamp and attributes per frame in the track."]
        #[serde(
            rename = "timestampedObjects",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamped_objects: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1TimestampedObject>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1Track {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P3Beta1Track {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgress {
        #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
        #[serde(
            rename = "feature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feature: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature,
        >,
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(
            rename = "inputUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_uri: ::std::option::Option<String>,
        #[doc = "Approximate percentage processed thus far. Guaranteed to be 100 when fully processed."]
        #[serde(
            rename = "progressPercent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_percent: ::std::option::Option<i32>,
        #[doc = "Specifies which segment is being tracked if the request contains more than one segment."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment,
        >,
        #[doc = "Time when the request was received."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Time of the most recent update."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgress
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgress
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature {
        #[doc = "Celebrity recognition."]
        CelebrityRecognition,
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[doc = "Human face detection."]
        FaceDetection,
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[doc = "Logo detection, tracking, and recognition."]
        LogoRecognition,
        #[doc = "Object detection and tracking."]
        ObjectTracking,
        #[doc = "Person detection."]
        PersonDetection,
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[doc = "OCR text detection and tracking."]
        TextDetection,
    }
    impl GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: CelebrityRecognition => "CELEBRITY_RECOGNITION" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: ExplicitContentDetection => "EXPLICIT_CONTENT_DETECTION" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: FaceDetection => "FACE_DETECTION" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: FeatureUnspecified => "FEATURE_UNSPECIFIED" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: LabelDetection => "LABEL_DETECTION" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: LogoRecognition => "LOGO_RECOGNITION" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: ObjectTracking => "OBJECT_TRACKING" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: PersonDetection => "PERSON_DETECTION" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: ShotChangeDetection => "SHOT_CHANGE_DETECTION" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: SpeechTranscription => "SPEECH_TRANSCRIPTION" , GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: TextDetection => "TEXT_DETECTION" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature,
            (),
        > {
            Ok (match s { "CELEBRITY_RECOGNITION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: CelebrityRecognition , "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: ExplicitContentDetection , "FACE_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: FaceDetection , "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: LabelDetection , "LOGO_RECOGNITION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: LogoRecognition , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: ObjectTracking , "PERSON_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: PersonDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: ShotChangeDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: TextDetection , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CELEBRITY_RECOGNITION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: CelebrityRecognition , "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: ExplicitContentDetection , "FACE_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: FaceDetection , "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: LabelDetection , "LOGO_RECOGNITION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: LogoRecognition , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: ObjectTracking , "PERSON_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: PersonDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: ShotChangeDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature :: TextDetection , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationProgressFeature
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationResults {
        #[doc = "Celebrity recognition annotations."]
        #[serde(
            rename = "celebrityRecognitionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub celebrity_recognition_annotations: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1CelebrityRecognitionAnnotation,
        >,
        #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest` some videos may succeed and some may fail."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Explicit content annotation."]
        #[serde(
            rename = "explicitAnnotation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub explicit_annotation: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1ExplicitContentAnnotation,
        >,
        #[doc = "Deprecated. Please use `face_detection_annotations` instead."]
        #[serde(
            rename = "faceAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub face_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1FaceAnnotation>,
        >,
        #[doc = "Face detection annotations."]
        #[serde(
            rename = "faceDetectionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub face_detection_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1FaceDetectionAnnotation>,
        >,
        #[doc = "Label annotations on frame level. There is exactly one element for each unique label."]
        #[serde(
            rename = "frameLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frame_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LabelAnnotation>,
        >,
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(
            rename = "inputUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_uri: ::std::option::Option<String>,
        #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
        #[serde(
            rename = "logoRecognitionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logo_recognition_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LogoRecognitionAnnotation>,
        >,
        #[doc = "Annotations for list of objects detected and tracked in video."]
        #[serde(
            rename = "objectAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1ObjectTrackingAnnotation>,
        >,
        #[doc = "Person detection annotations."]
        #[serde(
            rename = "personDetectionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person_detection_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1PersonDetectionAnnotation>,
        >,
        #[doc = "Video segment on which the annotation is run."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment,
        >,
        #[doc = "Topical label annotations on video level or user-specified segment level. There is exactly one element for each unique label."]
        #[serde(
            rename = "segmentLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LabelAnnotation>,
        >,
        #[doc = "Presence label annotations on video level or user-specified segment level. There is exactly one element for each unique label. Compared to the existing topical `segment_label_annotations`, this field presents more fine-grained, segment-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to “builtin/latest” in the request."]
        #[serde(
            rename = "segmentPresenceLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_presence_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LabelAnnotation>,
        >,
        #[doc = "Shot annotations. Each shot is represented as a video segment."]
        #[serde(
            rename = "shotAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1VideoSegment>,
        >,
        #[doc = "Topical label annotations on shot level. There is exactly one element for each unique label."]
        #[serde(
            rename = "shotLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LabelAnnotation>,
        >,
        #[doc = "Presence label annotations on shot level. There is exactly one element for each unique label. Compared to the existing topical `shot_label_annotations`, this field presents more fine-grained, shot-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to “builtin/latest” in the request."]
        #[serde(
            rename = "shotPresenceLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_presence_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1LabelAnnotation>,
        >,
        #[doc = "Speech transcription."]
        #[serde(
            rename = "speechTranscriptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub speech_transcriptions: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1SpeechTranscription>,
        >,
        #[doc = "OCR text detection and tracking. Annotations for list of detected text snippets. Each will have list of frame information associated with it."]
        #[serde(
            rename = "textAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1P3Beta1TextAnnotation>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationResults
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1P3Beta1VideoAnnotationResults
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
    pub struct GoogleCloudVideointelligenceV1P3Beta1VideoSegment {
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive)."]
        #[serde(
            rename = "endTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time_offset: ::std::option::Option<String>,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive)."]
        #[serde(
            rename = "startTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1VideoSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P3Beta1VideoSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1P3Beta1WordInfo {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from 1 up to diarization_speaker_count, and is only set if speaker diarization is enabled."]
        #[serde(
            rename = "speakerTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub speaker_tag: ::std::option::Option<i32>,
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the start of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "The word corresponding to this set of information."]
        #[serde(
            rename = "word",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub word: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1P3Beta1WordInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1P3Beta1WordInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1PersonDetectionAnnotation {
        #[doc = "The detected tracks of a person."]
        #[serde(
            rename = "tracks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tracks: ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1Track>>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1PersonDetectionAnnotation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1PersonDetectionAnnotation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1SpeechRecognitionAlternative {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Transcript text representing the words that the user spoke."]
        #[serde(
            rename = "transcript",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub transcript: ::std::option::Option<String>,
        #[doc = "Output only. A list of word-specific information for each recognized word. Note: When `enable_speaker_diarization` is set to true, you will see all the words from the beginning of the audio."]
        #[serde(
            rename = "words",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub words:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1WordInfo>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1SpeechRecognitionAlternative
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1SpeechRecognitionAlternative
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1SpeechTranscription {
        #[doc = "May contain one or more recognition hypotheses (up to the maximum specified in `max_alternatives`). These alternatives are ordered in terms of accuracy, with the top (first) alternative being the most probable, as ranked by the recognizer."]
        #[serde(
            rename = "alternatives",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alternatives: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1SpeechRecognitionAlternative>,
        >,
        #[doc = "Output only. The [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag of the language in this result. This language code was detected to have the most likelihood of being spoken in the audio."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1SpeechTranscription {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1SpeechTranscription {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1TextAnnotation {
        #[doc = "All video segments where OCR detected text appears."]
        #[serde(
            rename = "segments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segments:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1TextSegment>>,
        #[doc = "The detected text."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
        #[doc = "Feature version."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1TextAnnotation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1TextAnnotation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1TextFrame {
        #[doc = "Bounding polygon of the detected text for this frame."]
        #[serde(
            rename = "rotatedBoundingBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rotated_bounding_box: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1NormalizedBoundingPoly,
        >,
        #[doc = "Timestamp of this frame."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1TextFrame {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1TextFrame {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1TextSegment {
        #[doc = "Confidence for the track of detected text. It is calculated as the highest over all frames where OCR detected text appears."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Information related to the frames where OCR detected text appears."]
        #[serde(
            rename = "frames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frames:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1TextFrame>>,
        #[doc = "Video segment where a text snippet was detected."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1VideoSegment>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1TextSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1TextSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1TimestampedObject {
        #[doc = "Optional. The attributes of the object in the bounding box."]
        #[serde(
            rename = "attributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1DetectedAttribute>,
        >,
        #[doc = "Optional. The detected landmarks."]
        #[serde(
            rename = "landmarks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub landmarks: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1DetectedLandmark>,
        >,
        #[doc = "Normalized Bounding box in a frame, where the object is located."]
        #[serde(
            rename = "normalizedBoundingBox",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_bounding_box: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1NormalizedBoundingBox,
        >,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the video frame for this object."]
        #[serde(
            rename = "timeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1TimestampedObject {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1TimestampedObject {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1Track {
        #[doc = "Optional. Attributes in the track level."]
        #[serde(
            rename = "attributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributes: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1DetectedAttribute>,
        >,
        #[doc = "Optional. The confidence score of the tracked object."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Video segment of a track."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1VideoSegment>,
        #[doc = "The object with timestamp and attributes per frame in the track."]
        #[serde(
            rename = "timestampedObjects",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamped_objects: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1TimestampedObject>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1Track {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1Track {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1VideoAnnotationProgress {
        #[doc = "Specifies which feature is being tracked if the request contains more than one feature."]
        #[serde(
            rename = "feature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feature: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature,
        >,
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(
            rename = "inputUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_uri: ::std::option::Option<String>,
        #[doc = "Approximate percentage processed thus far. Guaranteed to be 100 when fully processed."]
        #[serde(
            rename = "progressPercent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub progress_percent: ::std::option::Option<i32>,
        #[doc = "Specifies which segment is being tracked if the request contains more than one segment."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1VideoSegment>,
        #[doc = "Time when the request was received."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Time of the most recent update."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1VideoAnnotationProgress
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1VideoAnnotationProgress
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature {
        #[doc = "Explicit content detection."]
        ExplicitContentDetection,
        #[doc = "Human face detection."]
        FaceDetection,
        #[doc = "Unspecified."]
        FeatureUnspecified,
        #[doc = "Label detection. Detect objects, such as dog or flower."]
        LabelDetection,
        #[doc = "Logo detection, tracking, and recognition."]
        LogoRecognition,
        #[doc = "Object detection and tracking."]
        ObjectTracking,
        #[doc = "Person detection."]
        PersonDetection,
        #[doc = "Shot change detection."]
        ShotChangeDetection,
        #[doc = "Speech transcription."]
        SpeechTranscription,
        #[doc = "OCR text detection and tracking."]
        TextDetection,
    }
    impl GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: ExplicitContentDetection => "EXPLICIT_CONTENT_DETECTION" , GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: FaceDetection => "FACE_DETECTION" , GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: FeatureUnspecified => "FEATURE_UNSPECIFIED" , GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: LabelDetection => "LABEL_DETECTION" , GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: LogoRecognition => "LOGO_RECOGNITION" , GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: ObjectTracking => "OBJECT_TRACKING" , GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: PersonDetection => "PERSON_DETECTION" , GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: ShotChangeDetection => "SHOT_CHANGE_DETECTION" , GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: SpeechTranscription => "SPEECH_TRANSCRIPTION" , GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: TextDetection => "TEXT_DETECTION" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature, ()>
        {
            Ok (match s { "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: ExplicitContentDetection , "FACE_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: FaceDetection , "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: LabelDetection , "LOGO_RECOGNITION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: LogoRecognition , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: ObjectTracking , "PERSON_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: PersonDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: ShotChangeDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: TextDetection , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "EXPLICIT_CONTENT_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: ExplicitContentDetection , "FACE_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: FaceDetection , "FEATURE_UNSPECIFIED" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: FeatureUnspecified , "LABEL_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: LabelDetection , "LOGO_RECOGNITION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: LogoRecognition , "OBJECT_TRACKING" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: ObjectTracking , "PERSON_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: PersonDetection , "SHOT_CHANGE_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: ShotChangeDetection , "SPEECH_TRANSCRIPTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: SpeechTranscription , "TEXT_DETECTION" => GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature :: TextDetection , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudVideointelligenceV1VideoAnnotationProgressFeature
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudVideointelligenceV1VideoAnnotationResults {
        #[doc = "If set, indicates an error. Note that for a single `AnnotateVideoRequest` some videos may succeed and some may fail."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Explicit content annotation."]
        #[serde(
            rename = "explicitAnnotation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub explicit_annotation: ::std::option::Option<
            crate::schemas::GoogleCloudVideointelligenceV1ExplicitContentAnnotation,
        >,
        #[doc = "Deprecated. Please use `face_detection_annotations` instead."]
        #[serde(
            rename = "faceAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub face_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1FaceAnnotation>,
        >,
        #[doc = "Face detection annotations."]
        #[serde(
            rename = "faceDetectionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub face_detection_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1FaceDetectionAnnotation>,
        >,
        #[doc = "Label annotations on frame level. There is exactly one element for each unique label."]
        #[serde(
            rename = "frameLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frame_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1LabelAnnotation>,
        >,
        #[doc = "Video file location in [Cloud Storage](https://cloud.google.com/storage/)."]
        #[serde(
            rename = "inputUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_uri: ::std::option::Option<String>,
        #[doc = "Annotations for list of logos detected, tracked and recognized in video."]
        #[serde(
            rename = "logoRecognitionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logo_recognition_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1LogoRecognitionAnnotation>,
        >,
        #[doc = "Annotations for list of objects detected and tracked in video."]
        #[serde(
            rename = "objectAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1ObjectTrackingAnnotation>,
        >,
        #[doc = "Person detection annotations."]
        #[serde(
            rename = "personDetectionAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person_detection_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1PersonDetectionAnnotation>,
        >,
        #[doc = "Video segment on which the annotation is run."]
        #[serde(
            rename = "segment",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment:
            ::std::option::Option<crate::schemas::GoogleCloudVideointelligenceV1VideoSegment>,
        #[doc = "Topical label annotations on video level or user-specified segment level. There is exactly one element for each unique label."]
        #[serde(
            rename = "segmentLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1LabelAnnotation>,
        >,
        #[doc = "Presence label annotations on video level or user-specified segment level. There is exactly one element for each unique label. Compared to the existing topical `segment_label_annotations`, this field presents more fine-grained, segment-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to “builtin/latest” in the request."]
        #[serde(
            rename = "segmentPresenceLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_presence_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1LabelAnnotation>,
        >,
        #[doc = "Shot annotations. Each shot is represented as a video segment."]
        #[serde(
            rename = "shotAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_annotations:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudVideointelligenceV1VideoSegment>>,
        #[doc = "Topical label annotations on shot level. There is exactly one element for each unique label."]
        #[serde(
            rename = "shotLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1LabelAnnotation>,
        >,
        #[doc = "Presence label annotations on shot level. There is exactly one element for each unique label. Compared to the existing topical `shot_label_annotations`, this field presents more fine-grained, shot-level labels detected in video content and is made available only when the client sets `LabelDetectionConfig.model` to “builtin/latest” in the request."]
        #[serde(
            rename = "shotPresenceLabelAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shot_presence_label_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1LabelAnnotation>,
        >,
        #[doc = "Speech transcription."]
        #[serde(
            rename = "speechTranscriptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub speech_transcriptions: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1SpeechTranscription>,
        >,
        #[doc = "OCR text detection and tracking. Annotations for list of detected text snippets. Each will have list of frame information associated with it."]
        #[serde(
            rename = "textAnnotations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_annotations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudVideointelligenceV1TextAnnotation>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudVideointelligenceV1VideoAnnotationResults
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1VideoAnnotationResults {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1VideoSegment {
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the end of the segment (inclusive)."]
        #[serde(
            rename = "endTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time_offset: ::std::option::Option<String>,
        #[doc = "Time-offset, relative to the beginning of the video, corresponding to the start of the segment (inclusive)."]
        #[serde(
            rename = "startTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1VideoSegment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1VideoSegment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudVideointelligenceV1WordInfo {
        #[doc = "Output only. The confidence estimate between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. This field is set only for the top alternative. This field is not guaranteed to be accurate and users should not rely on it to be always provided. The default of 0.0 is a sentinel value indicating `confidence` was not set."]
        #[serde(
            rename = "confidence",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub confidence: ::std::option::Option<f32>,
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the end of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Output only. A distinct integer value is assigned for every speaker within the audio. This field specifies which one of those speakers was detected to have spoken this word. Value ranges from 1 up to diarization_speaker_count, and is only set if speaker diarization is enabled."]
        #[serde(
            rename = "speakerTag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub speaker_tag: ::std::option::Option<i32>,
        #[doc = "Time offset relative to the beginning of the audio, and corresponding to the start of the spoken word. This field is only set if `enable_word_time_offsets=true` and only in the top hypothesis. This is an experimental feature and the accuracy of the time offset can vary."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "The word corresponding to this set of information."]
        #[serde(
            rename = "word",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub word: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudVideointelligenceV1WordInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudVideointelligenceV1WordInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleLongrunningOperation {
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
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
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
    impl ::google_field_selector::FieldSelector for GoogleLongrunningOperation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleLongrunningOperation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleRpcStatus {
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
    impl ::google_field_selector::FieldSelector for GoogleRpcStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleRpcStatus {
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
    #[doc = "Actions that can be performed on the videos resource"]
    pub fn videos(&self) -> crate::resources::videos::VideosActions {
        crate::resources::videos::VideosActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod videos {
        pub mod params {}
        pub struct VideosActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> VideosActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Performs asynchronous video annotation. Progress and results can be retrieved through the `google.longrunning.Operations` interface. `Operation.metadata` contains `AnnotateVideoProgress` (progress). `Operation.response` contains `AnnotateVideoResponse` (results)."]
            pub fn annotate(
                &self,
                request: crate::schemas::GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequest,
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
                }
            }
        }
        #[doc = "Created via [VideosActions::annotate()](struct.VideosActions.html#method.annotate)"]
        #[derive(Debug, Clone)]
        pub struct AnnotateRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::GoogleCloudVideointelligenceV1P2Beta1AnnotateVideoRequest,
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
            ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error> {
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
                let mut output = "https://videointelligence.googleapis.com/".to_owned();
                output.push_str("v1p2beta1/videos:annotate");
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
