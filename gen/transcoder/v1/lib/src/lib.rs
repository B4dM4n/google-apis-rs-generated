#![doc = "# Resources and Methods\n* [projects](resources/projects/struct.ProjectsActions.html)\n  * [locations](resources/projects/locations/struct.LocationsActions.html)\n    * [job_templates](resources/projects/locations/job_templates/struct.JobTemplatesActions.html)\n      * [*create*](resources/projects/locations/job_templates/struct.CreateRequestBuilder.html), [*delete*](resources/projects/locations/job_templates/struct.DeleteRequestBuilder.html), [*get*](resources/projects/locations/job_templates/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/job_templates/struct.ListRequestBuilder.html)\n    * [jobs](resources/projects/locations/jobs/struct.JobsActions.html)\n      * [*create*](resources/projects/locations/jobs/struct.CreateRequestBuilder.html), [*delete*](resources/projects/locations/jobs/struct.DeleteRequestBuilder.html), [*get*](resources/projects/locations/jobs/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/jobs/struct.ListRequestBuilder.html)\n"]
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
    pub struct AdBreak {
        #[doc = "Start time in seconds for the ad break, relative to the output file timeline. The default is `0s`."]
        #[serde(
            rename = "startTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AdBreak {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdBreak {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Animation {
        #[doc = "End previous animation."]
        #[serde(
            rename = "animationEnd",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub animation_end: ::std::option::Option<crate::schemas::AnimationEnd>,
        #[doc = "Display overlay object with fade animation."]
        #[serde(
            rename = "animationFade",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub animation_fade: ::std::option::Option<crate::schemas::AnimationFade>,
        #[doc = "Display static overlay object."]
        #[serde(
            rename = "animationStatic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub animation_static: ::std::option::Option<crate::schemas::AnimationStatic>,
    }
    impl ::google_field_selector::FieldSelector for Animation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Animation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AnimationEnd {
        #[doc = "The time to end overlay object, in seconds. Default: 0"]
        #[serde(
            rename = "startTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AnimationEnd {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnimationEnd {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AnimationFade {
        #[doc = "The time to end the fade animation, in seconds. Default: `start_time_offset` + 1s"]
        #[serde(
            rename = "endTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time_offset: ::std::option::Option<String>,
        #[doc = "Required. Type of fade animation: `FADE_IN` or `FADE_OUT`."]
        #[serde(
            rename = "fadeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fade_type: ::std::option::Option<crate::schemas::AnimationFadeFadeType>,
        #[doc = "The time to start the fade animation, in seconds. Default: 0"]
        #[serde(
            rename = "startTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time_offset: ::std::option::Option<String>,
        #[doc = "Normalized coordinates based on output video resolution. Valid values: `0.0`–`1.0`. `xy` is the upper-left coordinate of the overlay object. For example, use the x and y coordinates {0,0} to position the top-left corner of the overlay animation in the top-left corner of the output video."]
        #[serde(
            rename = "xy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub xy: ::std::option::Option<crate::schemas::NormalizedCoordinate>,
    }
    impl ::google_field_selector::FieldSelector for AnimationFade {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnimationFade {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AnimationFadeFadeType {
        #[doc = "Fade the overlay object into view."]
        FadeIn,
        #[doc = "Fade the overlay object out of view."]
        FadeOut,
        #[doc = "The fade type is not specified."]
        FadeTypeUnspecified,
    }
    impl AnimationFadeFadeType {
        pub fn as_str(self) -> &'static str {
            match self {
                AnimationFadeFadeType::FadeIn => "FADE_IN",
                AnimationFadeFadeType::FadeOut => "FADE_OUT",
                AnimationFadeFadeType::FadeTypeUnspecified => "FADE_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AnimationFadeFadeType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AnimationFadeFadeType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AnimationFadeFadeType, ()> {
            Ok(match s {
                "FADE_IN" => AnimationFadeFadeType::FadeIn,
                "FADE_OUT" => AnimationFadeFadeType::FadeOut,
                "FADE_TYPE_UNSPECIFIED" => AnimationFadeFadeType::FadeTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AnimationFadeFadeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AnimationFadeFadeType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AnimationFadeFadeType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FADE_IN" => AnimationFadeFadeType::FadeIn,
                "FADE_OUT" => AnimationFadeFadeType::FadeOut,
                "FADE_TYPE_UNSPECIFIED" => AnimationFadeFadeType::FadeTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AnimationFadeFadeType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnimationFadeFadeType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AnimationStatic {
        #[doc = "The time to start displaying the overlay object, in seconds. Default: 0"]
        #[serde(
            rename = "startTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time_offset: ::std::option::Option<String>,
        #[doc = "Normalized coordinates based on output video resolution. Valid values: `0.0`–`1.0`. `xy` is the upper-left coordinate of the overlay object. For example, use the x and y coordinates {0,0} to position the top-left corner of the overlay animation in the top-left corner of the output video."]
        #[serde(
            rename = "xy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub xy: ::std::option::Option<crate::schemas::NormalizedCoordinate>,
    }
    impl ::google_field_selector::FieldSelector for AnimationStatic {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AnimationStatic {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Audio {
        #[doc = "Enable boosting high frequency components. The default is `false`. **Note:** This field is not supported."]
        #[serde(
            rename = "highBoost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub high_boost: ::std::option::Option<bool>,
        #[doc = "Enable boosting low frequency components. The default is `false`. **Note:** This field is not supported."]
        #[serde(
            rename = "lowBoost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub low_boost: ::std::option::Option<bool>,
        #[doc = "Specify audio loudness normalization in loudness units relative to full scale (LUFS). Enter a value between -24 and 0 (the default), where: * -24 is the Advanced Television Systems Committee (ATSC A/85) standard * -23 is the EU R128 broadcast standard * -19 is the prior standard for online mono audio * -18 is the ReplayGain standard * -16 is the prior standard for stereo audio * -14 is the new online audio standard recommended by Spotify, as well as Amazon Echo * 0 disables normalization"]
        #[serde(
            rename = "lufs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lufs: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for Audio {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Audio {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AudioMapping {
        #[doc = "Required. The `EditAtom.key` that references the atom with audio inputs in the `Job.edit_list`."]
        #[serde(
            rename = "atomKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub atom_key: ::std::option::Option<String>,
        #[doc = "Audio volume control in dB. Negative values decrease volume, positive values increase. The default is 0."]
        #[serde(
            rename = "gainDb",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gain_db: ::std::option::Option<f64>,
        #[doc = "Required. The zero-based index of the channel in the input audio stream."]
        #[serde(
            rename = "inputChannel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_channel: ::std::option::Option<i32>,
        #[doc = "Required. The `Input.key` that identifies the input file."]
        #[serde(
            rename = "inputKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_key: ::std::option::Option<String>,
        #[doc = "Required. The zero-based index of the track in the input file."]
        #[serde(
            rename = "inputTrack",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_track: ::std::option::Option<i32>,
        #[doc = "Required. The zero-based index of the channel in the output audio stream."]
        #[serde(
            rename = "outputChannel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_channel: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for AudioMapping {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AudioMapping {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AudioStream {
        #[doc = "Required. Audio bitrate in bits per second. Must be between 1 and 10,000,000."]
        #[serde(
            rename = "bitrateBps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bitrate_bps: ::std::option::Option<i32>,
        #[doc = "Number of audio channels. Must be between 1 and 6. The default is 2."]
        #[serde(
            rename = "channelCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub channel_count: ::std::option::Option<i32>,
        #[doc = "A list of channel names specifying layout of the audio channels. This only affects the metadata embedded in the container headers, if supported by the specified format. The default is `[\"fl\", \"fr\"]`. Supported channel names: - `fl` - Front left channel - `fr` - Front right channel - `sl` - Side left channel - `sr` - Side right channel - `fc` - Front center channel - `lfe` - Low frequency"]
        #[serde(
            rename = "channelLayout",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub channel_layout: ::std::option::Option<Vec<String>>,
        #[doc = "The codec for this audio stream. The default is `aac`. Supported audio codecs: - `aac` - `aac-he` - `aac-he-v2` - `mp3` - `ac3` - `eac3`"]
        #[serde(
            rename = "codec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub codec: ::std::option::Option<String>,
        #[doc = "The mapping for the `Job.edit_list` atoms with audio `EditAtom.inputs`."]
        #[serde(
            rename = "mapping",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mapping: ::std::option::Option<Vec<crate::schemas::AudioMapping>>,
        #[doc = "The audio sample rate in Hertz. The default is 48000 Hertz."]
        #[serde(
            rename = "sampleRateHertz",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_rate_hertz: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for AudioStream {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AudioStream {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Color {
        #[doc = "Control brightness of the video. Enter a value between -1 and 1, where -1 is minimum brightness and 1 is maximum brightness. 0 is no change. The default is 0."]
        #[serde(
            rename = "brightness",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub brightness: ::std::option::Option<f64>,
        #[doc = "Control black and white contrast of the video. Enter a value between -1 and 1, where -1 is minimum contrast and 1 is maximum contrast. 0 is no change. The default is 0."]
        #[serde(
            rename = "contrast",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contrast: ::std::option::Option<f64>,
        #[doc = "Control color saturation of the video. Enter a value between -1 and 1, where -1 is fully desaturated and 1 is maximum saturation. 0 is no change. The default is 0."]
        #[serde(
            rename = "saturation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub saturation: ::std::option::Option<f64>,
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
    pub struct Crop {
        #[doc = "The number of pixels to crop from the bottom. The default is 0."]
        #[serde(
            rename = "bottomPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bottom_pixels: ::std::option::Option<i32>,
        #[doc = "The number of pixels to crop from the left. The default is 0."]
        #[serde(
            rename = "leftPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left_pixels: ::std::option::Option<i32>,
        #[doc = "The number of pixels to crop from the right. The default is 0."]
        #[serde(
            rename = "rightPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub right_pixels: ::std::option::Option<i32>,
        #[doc = "The number of pixels to crop from the top. The default is 0."]
        #[serde(
            rename = "topPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top_pixels: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Crop {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Crop {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Deblock {
        #[doc = "Enable deblocker. The default is `false`."]
        #[serde(
            rename = "enabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enabled: ::std::option::Option<bool>,
        #[doc = "Set strength of the deblocker. Enter a value between 0 and 1. The higher the value, the stronger the block removal. 0 is no deblocking. The default is 0."]
        #[serde(
            rename = "strength",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub strength: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for Deblock {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Deblock {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Denoise {
        #[doc = "Set strength of the denoise. Enter a value between 0 and 1. The higher the value, the smoother the image. 0 is no denoising. The default is 0."]
        #[serde(
            rename = "strength",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub strength: ::std::option::Option<f64>,
        #[doc = "Set the denoiser mode. The default is `standard`. Supported denoiser modes: - `standard` - `grain`"]
        #[serde(
            rename = "tune",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tune: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Denoise {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Denoise {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EditAtom {
        #[doc = "End time in seconds for the atom, relative to the input file timeline. When `end_time_offset` is not specified, the `inputs` are used until the end of the atom."]
        #[serde(
            rename = "endTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time_offset: ::std::option::Option<String>,
        #[doc = "List of `Input.key`s identifying files that should be used in this atom. The listed `inputs` must have the same timeline."]
        #[serde(
            rename = "inputs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inputs: ::std::option::Option<Vec<String>>,
        #[doc = "A unique key for this atom. Must be specified when using advanced mapping."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "Start time in seconds for the atom, relative to the input file timeline. The default is `0s`."]
        #[serde(
            rename = "startTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EditAtom {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EditAtom {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ElementaryStream {
        #[doc = "Encoding of an audio stream."]
        #[serde(
            rename = "audioStream",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audio_stream: ::std::option::Option<crate::schemas::AudioStream>,
        #[doc = "A unique key for this elementary stream."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "Encoding of a text stream. For example, closed captions or subtitles."]
        #[serde(
            rename = "textStream",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_stream: ::std::option::Option<crate::schemas::TextStream>,
        #[doc = "Encoding of a video stream."]
        #[serde(
            rename = "videoStream",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub video_stream: ::std::option::Option<crate::schemas::VideoStream>,
    }
    impl ::google_field_selector::FieldSelector for ElementaryStream {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ElementaryStream {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct H264CodecSettings {
        #[doc = "Specifies whether an open Group of Pictures (GOP) structure should be allowed or not. The default is `false`."]
        #[serde(
            rename = "allowOpenGop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_open_gop: ::std::option::Option<bool>,
        #[doc = "Specify the intensity of the adaptive quantizer (AQ). Must be between 0 and 1, where 0 disables the quantizer and 1 maximizes the quantizer. A higher value equals a lower bitrate but smoother image. The default is 0."]
        #[serde(
            rename = "aqStrength",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aq_strength: ::std::option::Option<f64>,
        #[doc = "The number of consecutive B-frames. Must be greater than or equal to zero. Must be less than `VideoStream.gop_frame_count` if set. The default is 0."]
        #[serde(
            rename = "bFrameCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub b_frame_count: ::std::option::Option<i32>,
        #[doc = "Allow B-pyramid for reference frame selection. This may not be supported on all decoders. The default is `false`."]
        #[serde(
            rename = "bPyramid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub b_pyramid: ::std::option::Option<bool>,
        #[doc = "Required. The video bitrate in bits per second. The minimum value is 1,000. The maximum value is 800,000,000."]
        #[serde(
            rename = "bitrateBps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bitrate_bps: ::std::option::Option<i32>,
        #[doc = "Target CRF level. Must be between 10 and 36, where 10 is the highest quality and 36 is the most efficient compression. The default is 21."]
        #[serde(
            rename = "crfLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crf_level: ::std::option::Option<i32>,
        #[doc = "Use two-pass encoding strategy to achieve better video quality. `VideoStream.rate_control_mode` must be `vbr`. The default is `false`."]
        #[serde(
            rename = "enableTwoPass",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_two_pass: ::std::option::Option<bool>,
        #[doc = "The entropy coder to use. The default is `cabac`. Supported entropy coders: - `cavlc` - `cabac`"]
        #[serde(
            rename = "entropyCoder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entropy_coder: ::std::option::Option<String>,
        #[doc = "Required. The target video frame rate in frames per second (FPS). Must be less than or equal to 120. Will default to the input frame rate if larger than the input frame rate. The API will generate an output FPS that is divisible by the input FPS, and smaller or equal to the target FPS. See [Calculating frame rate](https://cloud.google.com/transcoder/docs/concepts/frame-rate) for more information."]
        #[serde(
            rename = "frameRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frame_rate: ::std::option::Option<f64>,
        #[doc = "Select the GOP size based on the specified duration. The default is `3s`. Note that `gopDuration` must be less than or equal to [`segmentDuration`](#SegmentSettings), and [`segmentDuration`](#SegmentSettings) must be divisible by `gopDuration`."]
        #[serde(
            rename = "gopDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gop_duration: ::std::option::Option<String>,
        #[doc = "Select the GOP size based on the specified frame count. Must be greater than zero."]
        #[serde(
            rename = "gopFrameCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gop_frame_count: ::std::option::Option<i32>,
        #[doc = "The height of the video in pixels. Must be an even integer. When not specified, the height is adjusted to match the specified width and input aspect ratio. If both are omitted, the input height is used."]
        #[serde(
            rename = "heightPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height_pixels: ::std::option::Option<i32>,
        #[doc = "Pixel format to use. The default is `yuv420p`. Supported pixel formats: - `yuv420p` pixel format - `yuv422p` pixel format - `yuv444p` pixel format - `yuv420p10` 10-bit HDR pixel format - `yuv422p10` 10-bit HDR pixel format - `yuv444p10` 10-bit HDR pixel format - `yuv420p12` 12-bit HDR pixel format - `yuv422p12` 12-bit HDR pixel format - `yuv444p12` 12-bit HDR pixel format"]
        #[serde(
            rename = "pixelFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pixel_format: ::std::option::Option<String>,
        #[doc = "Enforces the specified codec preset. The default is `veryfast`. The available options are [FFmpeg-compatible](https://trac.ffmpeg.org/wiki/Encode/H.264#Preset). Note that certain values for this field may cause the transcoder to override other fields you set in the `H264CodecSettings` message."]
        #[serde(
            rename = "preset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preset: ::std::option::Option<String>,
        #[doc = "Enforces the specified codec profile. The following profiles are supported: * `baseline` * `main` * `high` (default) The available options are [FFmpeg-compatible](https://trac.ffmpeg.org/wiki/Encode/H.264#Tune). Note that certain values for this field may cause the transcoder to override other fields you set in the `H264CodecSettings` message."]
        #[serde(
            rename = "profile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub profile: ::std::option::Option<String>,
        #[doc = "Specify the `rate_control_mode`. The default is `vbr`. Supported rate control modes: - `vbr` - variable bitrate - `crf` - constant rate factor"]
        #[serde(
            rename = "rateControlMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rate_control_mode: ::std::option::Option<String>,
        #[doc = "Enforces the specified codec tune. The available options are [FFmpeg-compatible](https://trac.ffmpeg.org/wiki/Encode/H.264#Tune). Note that certain values for this field may cause the transcoder to override other fields you set in the `H264CodecSettings` message."]
        #[serde(
            rename = "tune",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tune: ::std::option::Option<String>,
        #[doc = "Initial fullness of the Video Buffering Verifier (VBV) buffer in bits. Must be greater than zero. The default is equal to 90% of `VideoStream.vbv_size_bits`."]
        #[serde(
            rename = "vbvFullnessBits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vbv_fullness_bits: ::std::option::Option<i32>,
        #[doc = "Size of the Video Buffering Verifier (VBV) buffer in bits. Must be greater than zero. The default is equal to `VideoStream.bitrate_bps`."]
        #[serde(
            rename = "vbvSizeBits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vbv_size_bits: ::std::option::Option<i32>,
        #[doc = "The width of the video in pixels. Must be an even integer. When not specified, the width is adjusted to match the specified height and input aspect ratio. If both are omitted, the input width is used."]
        #[serde(
            rename = "widthPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width_pixels: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for H264CodecSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for H264CodecSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct H265CodecSettings {
        #[doc = "Specifies whether an open Group of Pictures (GOP) structure should be allowed or not. The default is `false`."]
        #[serde(
            rename = "allowOpenGop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_open_gop: ::std::option::Option<bool>,
        #[doc = "Specify the intensity of the adaptive quantizer (AQ). Must be between 0 and 1, where 0 disables the quantizer and 1 maximizes the quantizer. A higher value equals a lower bitrate but smoother image. The default is 0."]
        #[serde(
            rename = "aqStrength",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aq_strength: ::std::option::Option<f64>,
        #[doc = "The number of consecutive B-frames. Must be greater than or equal to zero. Must be less than `VideoStream.gop_frame_count` if set. The default is 0."]
        #[serde(
            rename = "bFrameCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub b_frame_count: ::std::option::Option<i32>,
        #[doc = "Allow B-pyramid for reference frame selection. This may not be supported on all decoders. The default is `false`."]
        #[serde(
            rename = "bPyramid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub b_pyramid: ::std::option::Option<bool>,
        #[doc = "Required. The video bitrate in bits per second. The minimum value is 1,000. The maximum value is 800,000,000."]
        #[serde(
            rename = "bitrateBps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bitrate_bps: ::std::option::Option<i32>,
        #[doc = "Target CRF level. Must be between 10 and 36, where 10 is the highest quality and 36 is the most efficient compression. The default is 21."]
        #[serde(
            rename = "crfLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crf_level: ::std::option::Option<i32>,
        #[doc = "Use two-pass encoding strategy to achieve better video quality. `VideoStream.rate_control_mode` must be `vbr`. The default is `false`."]
        #[serde(
            rename = "enableTwoPass",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_two_pass: ::std::option::Option<bool>,
        #[doc = "Required. The target video frame rate in frames per second (FPS). Must be less than or equal to 120. Will default to the input frame rate if larger than the input frame rate. The API will generate an output FPS that is divisible by the input FPS, and smaller or equal to the target FPS. See [Calculating frame rate](https://cloud.google.com/transcoder/docs/concepts/frame-rate) for more information."]
        #[serde(
            rename = "frameRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frame_rate: ::std::option::Option<f64>,
        #[doc = "Select the GOP size based on the specified duration. The default is `3s`. Note that `gopDuration` must be less than or equal to [`segmentDuration`](#SegmentSettings), and [`segmentDuration`](#SegmentSettings) must be divisible by `gopDuration`."]
        #[serde(
            rename = "gopDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gop_duration: ::std::option::Option<String>,
        #[doc = "Select the GOP size based on the specified frame count. Must be greater than zero."]
        #[serde(
            rename = "gopFrameCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gop_frame_count: ::std::option::Option<i32>,
        #[doc = "The height of the video in pixels. Must be an even integer. When not specified, the height is adjusted to match the specified width and input aspect ratio. If both are omitted, the input height is used."]
        #[serde(
            rename = "heightPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height_pixels: ::std::option::Option<i32>,
        #[doc = "Pixel format to use. The default is `yuv420p`. Supported pixel formats: - `yuv420p` pixel format - `yuv422p` pixel format - `yuv444p` pixel format - `yuv420p10` 10-bit HDR pixel format - `yuv422p10` 10-bit HDR pixel format - `yuv444p10` 10-bit HDR pixel format - `yuv420p12` 12-bit HDR pixel format - `yuv422p12` 12-bit HDR pixel format - `yuv444p12` 12-bit HDR pixel format"]
        #[serde(
            rename = "pixelFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pixel_format: ::std::option::Option<String>,
        #[doc = "Enforces the specified codec preset. The default is `veryfast`. The available options are [FFmpeg-compatible](https://trac.ffmpeg.org/wiki/Encode/H.265). Note that certain values for this field may cause the transcoder to override other fields you set in the `H265CodecSettings` message."]
        #[serde(
            rename = "preset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preset: ::std::option::Option<String>,
        #[doc = "Enforces the specified codec profile. The following profiles are supported: * 8-bit profiles * `main` (default) * `main-intra` * `mainstillpicture` * 10-bit profiles * `main10` (default) * `main10-intra` * `main422-10` * `main422-10-intra` * `main444-10` * `main444-10-intra` * 12-bit profiles * `main12` (default) * `main12-intra` * `main422-12` * `main422-12-intra` * `main444-12` * `main444-12-intra` The available options are [FFmpeg-compatible](https://x265.readthedocs.io/). Note that certain values for this field may cause the transcoder to override other fields you set in the `H265CodecSettings` message."]
        #[serde(
            rename = "profile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub profile: ::std::option::Option<String>,
        #[doc = "Specify the `rate_control_mode`. The default is `vbr`. Supported rate control modes: - `vbr` - variable bitrate - `crf` - constant rate factor"]
        #[serde(
            rename = "rateControlMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rate_control_mode: ::std::option::Option<String>,
        #[doc = "Enforces the specified codec tune. The available options are [FFmpeg-compatible](https://trac.ffmpeg.org/wiki/Encode/H.265). Note that certain values for this field may cause the transcoder to override other fields you set in the `H265CodecSettings` message."]
        #[serde(
            rename = "tune",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tune: ::std::option::Option<String>,
        #[doc = "Initial fullness of the Video Buffering Verifier (VBV) buffer in bits. Must be greater than zero. The default is equal to 90% of `VideoStream.vbv_size_bits`."]
        #[serde(
            rename = "vbvFullnessBits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vbv_fullness_bits: ::std::option::Option<i32>,
        #[doc = "Size of the Video Buffering Verifier (VBV) buffer in bits. Must be greater than zero. The default is equal to `VideoStream.bitrate_bps`."]
        #[serde(
            rename = "vbvSizeBits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vbv_size_bits: ::std::option::Option<i32>,
        #[doc = "The width of the video in pixels. Must be an even integer. When not specified, the width is adjusted to match the specified height and input aspect ratio. If both are omitted, the input width is used."]
        #[serde(
            rename = "widthPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width_pixels: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for H265CodecSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for H265CodecSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Image {
        #[doc = "Target image opacity. Valid values are from `1.0` (solid, default) to `0.0` (transparent), exclusive. Set this to a value greater than `0.0`."]
        #[serde(
            rename = "alpha",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alpha: ::std::option::Option<f64>,
        #[doc = "Normalized image resolution, based on output video resolution. Valid values: `0.0`–`1.0`. To respect the original image aspect ratio, set either `x` or `y` to `0.0`. To use the original image resolution, set both `x` and `y` to `0.0`."]
        #[serde(
            rename = "resolution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolution: ::std::option::Option<crate::schemas::NormalizedCoordinate>,
        #[doc = "Required. URI of the JPEG image in Cloud Storage. For example, `gs://bucket/inputs/image.jpeg`. JPEG is the only supported image type."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Input {
        #[doc = "A unique key for this input. Must be specified when using advanced mapping and edit lists."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "Preprocessing configurations."]
        #[serde(
            rename = "preprocessingConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preprocessing_config: ::std::option::Option<crate::schemas::PreprocessingConfig>,
        #[doc = "URI of the media. Input files must be at least 5 seconds in duration and stored in Cloud Storage (for example, `gs://bucket/inputs/file.mp4`). If empty, the value is populated from `Job.input_uri`. See [Supported input and output formats](https://cloud.google.com/transcoder/docs/concepts/supported-input-and-output-formats)."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Input {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Input {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Job {
        #[doc = "The configuration for this job."]
        #[serde(
            rename = "config",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config: ::std::option::Option<crate::schemas::JobConfig>,
        #[doc = "Output only. The time the job was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. The time the transcoding finished."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Output only. An error object that describes the reason for the failure. This property is always present when `state` is `FAILED`."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::Status>,
        #[doc = "Input only. Specify the `input_uri` to populate empty `uri` fields in each element of `Job.config.inputs` or `JobTemplate.config.inputs` when using template. URI of the media. Input files must be at least 5 seconds in duration and stored in Cloud Storage (for example, `gs://bucket/inputs/file.mp4`). See [Supported input and output formats](https://cloud.google.com/transcoder/docs/concepts/supported-input-and-output-formats)."]
        #[serde(
            rename = "inputUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_uri: ::std::option::Option<String>,
        #[doc = "The resource name of the job. Format: `projects/{project_number}/locations/{location}/jobs/{job}`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Input only. Specify the `output_uri` to populate an empty `Job.config.output.uri` or `JobTemplate.config.output.uri` when using template. URI for the output file(s). For example, `gs://my-bucket/outputs/`. See [Supported input and output formats](https://cloud.google.com/transcoder/docs/concepts/supported-input-and-output-formats)."]
        #[serde(
            rename = "outputUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_uri: ::std::option::Option<String>,
        #[doc = "Output only. The time the transcoding started."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Output only. The current state of the job."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::JobState>,
        #[doc = "Input only. Specify the `template_id` to use for populating `Job.config`. The default is `preset/web-hd`. Preset Transcoder templates: - `preset/{preset_id}` - User defined JobTemplate: `{job_template_id}`"]
        #[serde(
            rename = "templateId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub template_id: ::std::option::Option<String>,
        #[doc = "Job time to live value in days, which will be effective after job completion. Job should be deleted automatically after the given TTL. Enter a value between 1 and 90. The default is 30."]
        #[serde(
            rename = "ttlAfterCompletionDays",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ttl_after_completion_days: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Job {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Job {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobState {
        #[doc = "The job has failed. For additional information, see `failure_reason` and `failure_details`"]
        Failed,
        #[doc = "The job is enqueued and will be picked up for processing soon."]
        Pending,
        #[doc = "The processing state is not specified."]
        ProcessingStateUnspecified,
        #[doc = "The job is being processed."]
        Running,
        #[doc = "The job has been completed successfully."]
        Succeeded,
    }
    impl JobState {
        pub fn as_str(self) -> &'static str {
            match self {
                JobState::Failed => "FAILED",
                JobState::Pending => "PENDING",
                JobState::ProcessingStateUnspecified => "PROCESSING_STATE_UNSPECIFIED",
                JobState::Running => "RUNNING",
                JobState::Succeeded => "SUCCEEDED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for JobState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for JobState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<JobState, ()> {
            Ok(match s {
                "FAILED" => JobState::Failed,
                "PENDING" => JobState::Pending,
                "PROCESSING_STATE_UNSPECIFIED" => JobState::ProcessingStateUnspecified,
                "RUNNING" => JobState::Running,
                "SUCCEEDED" => JobState::Succeeded,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for JobState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FAILED" => JobState::Failed,
                "PENDING" => JobState::Pending,
                "PROCESSING_STATE_UNSPECIFIED" => JobState::ProcessingStateUnspecified,
                "RUNNING" => JobState::Running,
                "SUCCEEDED" => JobState::Succeeded,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for JobState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JobConfig {
        #[doc = "List of ad breaks. Specifies where to insert ad break tags in the output manifests."]
        #[serde(
            rename = "adBreaks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_breaks: ::std::option::Option<Vec<crate::schemas::AdBreak>>,
        #[doc = "List of `Edit atom`s. Defines the ultimate timeline of the resulting file or manifest."]
        #[serde(
            rename = "editList",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub edit_list: ::std::option::Option<Vec<crate::schemas::EditAtom>>,
        #[doc = "List of elementary streams."]
        #[serde(
            rename = "elementaryStreams",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub elementary_streams: ::std::option::Option<Vec<crate::schemas::ElementaryStream>>,
        #[doc = "List of input assets stored in Cloud Storage."]
        #[serde(
            rename = "inputs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inputs: ::std::option::Option<Vec<crate::schemas::Input>>,
        #[doc = "List of output manifests."]
        #[serde(
            rename = "manifests",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manifests: ::std::option::Option<Vec<crate::schemas::Manifest>>,
        #[doc = "List of multiplexing settings for output streams."]
        #[serde(
            rename = "muxStreams",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mux_streams: ::std::option::Option<Vec<crate::schemas::MuxStream>>,
        #[doc = "Output configuration."]
        #[serde(
            rename = "output",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output: ::std::option::Option<crate::schemas::Output>,
        #[doc = "List of overlays on the output video, in descending Z-order."]
        #[serde(
            rename = "overlays",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub overlays: ::std::option::Option<Vec<crate::schemas::Overlay>>,
        #[doc = "Destination on Pub/Sub."]
        #[serde(
            rename = "pubsubDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pubsub_destination: ::std::option::Option<crate::schemas::PubsubDestination>,
        #[doc = "List of output sprite sheets."]
        #[serde(
            rename = "spriteSheets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sprite_sheets: ::std::option::Option<Vec<crate::schemas::SpriteSheet>>,
    }
    impl ::google_field_selector::FieldSelector for JobConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct JobTemplate {
        #[doc = "The configuration for this template."]
        #[serde(
            rename = "config",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub config: ::std::option::Option<crate::schemas::JobConfig>,
        #[doc = "The resource name of the job template. Format: `projects/{project_number}/locations/{location}/jobTemplates/{job_template}`"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for JobTemplate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for JobTemplate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListJobTemplatesResponse {
        #[doc = "List of job templates in the specified region."]
        #[serde(
            rename = "jobTemplates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_templates: ::std::option::Option<Vec<crate::schemas::JobTemplate>>,
        #[doc = "The pagination token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of regions that could not be reached."]
        #[serde(
            rename = "unreachable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unreachable: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ListJobTemplatesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListJobTemplatesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListJobTemplatesResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListJobsResponse {
        #[doc = "List of jobs in the specified region."]
        #[serde(
            rename = "jobs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub jobs: ::std::option::Option<Vec<crate::schemas::Job>>,
        #[doc = "The pagination token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of regions that could not be reached."]
        #[serde(
            rename = "unreachable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unreachable: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ListJobsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListJobsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListJobsResponse {
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
    pub struct Manifest {
        #[doc = "The name of the generated file. The default is `manifest` with the extension suffix corresponding to the `Manifest.type`."]
        #[serde(
            rename = "fileName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_name: ::std::option::Option<String>,
        #[doc = "Required. List of user given `MuxStream.key`s that should appear in this manifest. When `Manifest.type` is `HLS`, a media manifest with name `MuxStream.key` and `.m3u8` extension is generated for each element of the `Manifest.mux_streams`."]
        #[serde(
            rename = "muxStreams",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mux_streams: ::std::option::Option<Vec<String>>,
        #[doc = "Required. Type of the manifest, can be `HLS` or `DASH`."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::ManifestType>,
    }
    impl ::google_field_selector::FieldSelector for Manifest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Manifest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ManifestType {
        #[doc = "Create `DASH` manifest. The corresponding file extension is `.mpd`."]
        Dash,
        #[doc = "Create `HLS` manifest. The corresponding file extension is `.m3u8`."]
        Hls,
        #[doc = "The manifest type is not specified."]
        ManifestTypeUnspecified,
    }
    impl ManifestType {
        pub fn as_str(self) -> &'static str {
            match self {
                ManifestType::Dash => "DASH",
                ManifestType::Hls => "HLS",
                ManifestType::ManifestTypeUnspecified => "MANIFEST_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ManifestType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ManifestType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ManifestType, ()> {
            Ok(match s {
                "DASH" => ManifestType::Dash,
                "HLS" => ManifestType::Hls,
                "MANIFEST_TYPE_UNSPECIFIED" => ManifestType::ManifestTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ManifestType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ManifestType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ManifestType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DASH" => ManifestType::Dash,
                "HLS" => ManifestType::Hls,
                "MANIFEST_TYPE_UNSPECIFIED" => ManifestType::ManifestTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ManifestType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ManifestType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MuxStream {
        #[doc = "The container format. The default is `mp4` Supported container formats: - `ts` - `fmp4`- the corresponding file extension is `.m4s` - `mp4` - `vtt` See also: [Supported input and output formats](https://cloud.google.com/transcoder/docs/concepts/supported-input-and-output-formats)"]
        #[serde(
            rename = "container",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub container: ::std::option::Option<String>,
        #[doc = "List of `ElementaryStream.key`s multiplexed in this stream."]
        #[serde(
            rename = "elementaryStreams",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub elementary_streams: ::std::option::Option<Vec<String>>,
        #[doc = "The name of the generated file. The default is `MuxStream.key` with the extension suffix corresponding to the `MuxStream.container`. Individual segments also have an incremental 10-digit zero-padded suffix starting from 0 before the extension, such as `mux_stream0000000123.ts`."]
        #[serde(
            rename = "fileName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_name: ::std::option::Option<String>,
        #[doc = "A unique key for this multiplexed stream. HLS media manifests will be named `MuxStream.key` with the `.m3u8` extension suffix."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<String>,
        #[doc = "Segment settings for `ts`, `fmp4` and `vtt`."]
        #[serde(
            rename = "segmentSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_settings: ::std::option::Option<crate::schemas::SegmentSettings>,
    }
    impl ::google_field_selector::FieldSelector for MuxStream {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MuxStream {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NormalizedCoordinate {
        #[doc = "Normalized x coordinate."]
        #[serde(
            rename = "x",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub x: ::std::option::Option<f64>,
        #[doc = "Normalized y coordinate."]
        #[serde(
            rename = "y",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub y: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for NormalizedCoordinate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NormalizedCoordinate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Output {
        #[doc = "URI for the output file(s). For example, `gs://my-bucket/outputs/`. If empty, the value is populated from `Job.output_uri`. See [Supported input and output formats](https://cloud.google.com/transcoder/docs/concepts/supported-input-and-output-formats)."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Output {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Output {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Overlay {
        #[doc = "List of Animations. The list should be chronological, without any time overlap."]
        #[serde(
            rename = "animations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub animations: ::std::option::Option<Vec<crate::schemas::Animation>>,
        #[doc = "Image overlay."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::Image>,
    }
    impl ::google_field_selector::FieldSelector for Overlay {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Overlay {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Pad {
        #[doc = "The number of pixels to add to the bottom. The default is 0."]
        #[serde(
            rename = "bottomPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bottom_pixels: ::std::option::Option<i32>,
        #[doc = "The number of pixels to add to the left. The default is 0."]
        #[serde(
            rename = "leftPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub left_pixels: ::std::option::Option<i32>,
        #[doc = "The number of pixels to add to the right. The default is 0."]
        #[serde(
            rename = "rightPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub right_pixels: ::std::option::Option<i32>,
        #[doc = "The number of pixels to add to the top. The default is 0."]
        #[serde(
            rename = "topPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub top_pixels: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Pad {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Pad {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PreprocessingConfig {
        #[doc = "Audio preprocessing configuration."]
        #[serde(
            rename = "audio",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audio: ::std::option::Option<crate::schemas::Audio>,
        #[doc = "Color preprocessing configuration."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<crate::schemas::Color>,
        #[doc = "Specify the video cropping configuration."]
        #[serde(
            rename = "crop",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crop: ::std::option::Option<crate::schemas::Crop>,
        #[doc = "Deblock preprocessing configuration."]
        #[serde(
            rename = "deblock",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deblock: ::std::option::Option<crate::schemas::Deblock>,
        #[doc = "Denoise preprocessing configuration."]
        #[serde(
            rename = "denoise",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub denoise: ::std::option::Option<crate::schemas::Denoise>,
        #[doc = "Specify the video pad filter configuration."]
        #[serde(
            rename = "pad",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pad: ::std::option::Option<crate::schemas::Pad>,
    }
    impl ::google_field_selector::FieldSelector for PreprocessingConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PreprocessingConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PubsubDestination {
        #[doc = "The name of the Pub/Sub topic to publish job completion notification to. For example: `projects/{project}/topics/{topic}`."]
        #[serde(
            rename = "topic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub topic: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PubsubDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PubsubDestination {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SegmentSettings {
        #[doc = "Required. Create an individual segment file. The default is `false`."]
        #[serde(
            rename = "individualSegments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub individual_segments: ::std::option::Option<bool>,
        #[doc = "Duration of the segments in seconds. The default is `6.0s`. Note that `segmentDuration` must be greater than or equal to [`gopDuration`](#videostream), and `segmentDuration` must be divisible by [`gopDuration`](#videostream)."]
        #[serde(
            rename = "segmentDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_duration: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SegmentSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SegmentSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SpriteSheet {
        #[doc = "The maximum number of sprites per row in a sprite sheet. The default is 0, which indicates no maximum limit."]
        #[serde(
            rename = "columnCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub column_count: ::std::option::Option<i32>,
        #[doc = "End time in seconds, relative to the output file timeline. When `end_time_offset` is not specified, the sprites are generated until the end of the output file."]
        #[serde(
            rename = "endTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time_offset: ::std::option::Option<String>,
        #[doc = "Required. File name prefix for the generated sprite sheets. Each sprite sheet has an incremental 10-digit zero-padded suffix starting from 0 before the extension, such as `sprite_sheet0000000123.jpeg`."]
        #[serde(
            rename = "filePrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_prefix: ::std::option::Option<String>,
        #[doc = "Format type. The default is `jpeg`. Supported formats: - `jpeg`"]
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format: ::std::option::Option<String>,
        #[doc = "Starting from `0s`, create sprites at regular intervals. Specify the interval value in seconds."]
        #[serde(
            rename = "interval",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interval: ::std::option::Option<String>,
        #[doc = "The quality of the generated sprite sheet. Enter a value between 1 and 100, where 1 is the lowest quality and 100 is the highest quality. The default is 100. A high quality value corresponds to a low image data compression ratio."]
        #[serde(
            rename = "quality",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quality: ::std::option::Option<i32>,
        #[doc = "The maximum number of rows per sprite sheet. When the sprite sheet is full, a new sprite sheet is created. The default is 0, which indicates no maximum limit."]
        #[serde(
            rename = "rowCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_count: ::std::option::Option<i32>,
        #[doc = "Required. The height of sprite in pixels. Must be an even integer. To preserve the source aspect ratio, set the SpriteSheet.sprite_height_pixels field or the SpriteSheet.sprite_width_pixels field, but not both (the API will automatically calculate the missing field)."]
        #[serde(
            rename = "spriteHeightPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sprite_height_pixels: ::std::option::Option<i32>,
        #[doc = "Required. The width of sprite in pixels. Must be an even integer. To preserve the source aspect ratio, set the SpriteSheet.sprite_width_pixels field or the SpriteSheet.sprite_height_pixels field, but not both (the API will automatically calculate the missing field)."]
        #[serde(
            rename = "spriteWidthPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sprite_width_pixels: ::std::option::Option<i32>,
        #[doc = "Start time in seconds, relative to the output file timeline. Determines the first sprite to pick. The default is `0s`."]
        #[serde(
            rename = "startTimeOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time_offset: ::std::option::Option<String>,
        #[doc = "Total number of sprites. Create the specified number of sprites distributed evenly across the timeline of the output media. The default is 100."]
        #[serde(
            rename = "totalCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for SpriteSheet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpriteSheet {
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
    pub struct TextMapping {
        #[doc = "Required. The `EditAtom.key` that references atom with text inputs in the `Job.edit_list`."]
        #[serde(
            rename = "atomKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub atom_key: ::std::option::Option<String>,
        #[doc = "Required. The `Input.key` that identifies the input file."]
        #[serde(
            rename = "inputKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_key: ::std::option::Option<String>,
        #[doc = "Required. The zero-based index of the track in the input file."]
        #[serde(
            rename = "inputTrack",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_track: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for TextMapping {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextMapping {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TextStream {
        #[doc = "The codec for this text stream. The default is `webvtt`. Supported text codecs: - `srt` - `ttml` - `cea608` - `cea708` - `webvtt`"]
        #[serde(
            rename = "codec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub codec: ::std::option::Option<String>,
        #[doc = "The mapping for the `Job.edit_list` atoms with text `EditAtom.inputs`."]
        #[serde(
            rename = "mapping",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mapping: ::std::option::Option<Vec<crate::schemas::TextMapping>>,
    }
    impl ::google_field_selector::FieldSelector for TextStream {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextStream {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VideoStream {
        #[doc = "H264 codec settings."]
        #[serde(
            rename = "h264",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub h_264: ::std::option::Option<crate::schemas::H264CodecSettings>,
        #[doc = "H265 codec settings."]
        #[serde(
            rename = "h265",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub h_265: ::std::option::Option<crate::schemas::H265CodecSettings>,
        #[doc = "VP9 codec settings."]
        #[serde(
            rename = "vp9",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vp_9: ::std::option::Option<crate::schemas::Vp9CodecSettings>,
    }
    impl ::google_field_selector::FieldSelector for VideoStream {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VideoStream {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Vp9CodecSettings {
        #[doc = "Required. The video bitrate in bits per second. The minimum value is 1,000. The maximum value is 480,000,000."]
        #[serde(
            rename = "bitrateBps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bitrate_bps: ::std::option::Option<i32>,
        #[doc = "Target CRF level. Must be between 10 and 36, where 10 is the highest quality and 36 is the most efficient compression. The default is 21. **Note:** This field is not supported."]
        #[serde(
            rename = "crfLevel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crf_level: ::std::option::Option<i32>,
        #[doc = "Required. The target video frame rate in frames per second (FPS). Must be less than or equal to 120. Will default to the input frame rate if larger than the input frame rate. The API will generate an output FPS that is divisible by the input FPS, and smaller or equal to the target FPS. See [Calculating frame rate](https://cloud.google.com/transcoder/docs/concepts/frame-rate) for more information."]
        #[serde(
            rename = "frameRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frame_rate: ::std::option::Option<f64>,
        #[doc = "Select the GOP size based on the specified duration. The default is `3s`. Note that `gopDuration` must be less than or equal to [`segmentDuration`](#SegmentSettings), and [`segmentDuration`](#SegmentSettings) must be divisible by `gopDuration`."]
        #[serde(
            rename = "gopDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gop_duration: ::std::option::Option<String>,
        #[doc = "Select the GOP size based on the specified frame count. Must be greater than zero."]
        #[serde(
            rename = "gopFrameCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gop_frame_count: ::std::option::Option<i32>,
        #[doc = "The height of the video in pixels. Must be an even integer. When not specified, the height is adjusted to match the specified width and input aspect ratio. If both are omitted, the input height is used."]
        #[serde(
            rename = "heightPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height_pixels: ::std::option::Option<i32>,
        #[doc = "Pixel format to use. The default is `yuv420p`. Supported pixel formats: - `yuv420p` pixel format - `yuv422p` pixel format - `yuv444p` pixel format - `yuv420p10` 10-bit HDR pixel format - `yuv422p10` 10-bit HDR pixel format - `yuv444p10` 10-bit HDR pixel format - `yuv420p12` 12-bit HDR pixel format - `yuv422p12` 12-bit HDR pixel format - `yuv444p12` 12-bit HDR pixel format"]
        #[serde(
            rename = "pixelFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pixel_format: ::std::option::Option<String>,
        #[doc = "Enforces the specified codec profile. The following profiles are supported: * `profile0` (default) * `profile1` * `profile2` * `profile3` The available options are [WebM-compatible](https://www.webmproject.org/vp9/profiles/). Note that certain values for this field may cause the transcoder to override other fields you set in the `Vp9CodecSettings` message."]
        #[serde(
            rename = "profile",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub profile: ::std::option::Option<String>,
        #[doc = "Specify the `rate_control_mode`. The default is `vbr`. Supported rate control modes: - `vbr` - variable bitrate"]
        #[serde(
            rename = "rateControlMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rate_control_mode: ::std::option::Option<String>,
        #[doc = "The width of the video in pixels. Must be an even integer. When not specified, the width is adjusted to match the specified height and input aspect ratio. If both are omitted, the input width is used."]
        #[serde(
            rename = "widthPixels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width_pixels: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Vp9CodecSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Vp9CodecSettings {
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
            #[doc = "Actions that can be performed on the locations resource"]
            pub fn locations(&self) -> crate::resources::projects::locations::LocationsActions {
                crate::resources::projects::locations::LocationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod locations {
            pub mod params {}
            pub struct LocationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> LocationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the job_templates resource"]
                pub fn job_templates(
                    &self,
                ) -> crate::resources::projects::locations::job_templates::JobTemplatesActions
                {
                    crate::resources::projects::locations::job_templates::JobTemplatesActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the jobs resource"]
                pub fn jobs(&self) -> crate::resources::projects::locations::jobs::JobsActions {
                    crate::resources::projects::locations::jobs::JobsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod job_templates {
                pub mod params {}
                pub struct JobTemplatesActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> JobTemplatesActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Creates a job template in the specified region."]
                    pub fn create(
                        &self,
                        request: crate::schemas::JobTemplate,
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
                            job_template_id: None,
                        }
                    }
                    #[doc = "Deletes a job template."]
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
                            allow_missing: None,
                        }
                    }
                    #[doc = "Returns the job template data."]
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
                    #[doc = "Lists job templates in the specified region."]
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
                            order_by: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[doc = "Created via [JobTemplatesActions::create()](struct.JobTemplatesActions.html#method.create)"]
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::JobTemplate,
                    parent: String,
                    job_template_id: ::std::option::Option<String>,
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
                    #[doc = "Required. The ID to use for the job template, which will become the final component of the job template’s resource name. This value should be 4-63 characters, and valid characters must match the regular expression `a-zA-Z*`."]
                    pub fn job_template_id(mut self, value: impl Into<String>) -> Self {
                        self.job_template_id = Some(value.into());
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
                    ) -> Result<crate::schemas::JobTemplate, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::JobTemplate, crate::Error> {
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
                        let mut output = "https://transcoder.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/jobTemplates");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                        req = req.query(&[("jobTemplateId", &self.job_template_id)]);
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
                #[doc = "Created via [JobTemplatesActions::delete()](struct.JobTemplatesActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    allow_missing: ::std::option::Option<bool>,
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
                    #[doc = "If set to true, and the job template is not found, the request will succeed but no action will be taken on the server."]
                    pub fn allow_missing(mut self, value: bool) -> Self {
                        self.allow_missing = Some(value);
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
                        let mut output = "https://transcoder.googleapis.com/".to_owned();
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
                        req = req.query(&[("allowMissing", &self.allow_missing)]);
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
                #[doc = "Created via [JobTemplatesActions::get()](struct.JobTemplatesActions.html#method.get)"]
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
                    ) -> Result<crate::schemas::JobTemplate, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::JobTemplate, crate::Error> {
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
                        let mut output = "https://transcoder.googleapis.com/".to_owned();
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
                #[doc = "Created via [JobTemplatesActions::list()](struct.JobTemplatesActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    filter: ::std::option::Option<String>,
                    order_by: ::std::option::Option<String>,
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
                    #[doc = "The filter expression, following the syntax outlined in https://google.aip.dev/160."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "One or more fields to compare and use to sort the output. See https://google.aip.dev/132#ordering."]
                    pub fn order_by(mut self, value: impl Into<String>) -> Self {
                        self.order_by = Some(value.into());
                        self
                    }
                    #[doc = "The maximum number of items to return."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "The `next_page_token` value returned from a previous List request, if any."]
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
                    #[doc = "\nExecute the request and yield each item in the `jobTemplates` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_job_templates<T>(
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
                        self.stream_job_templates_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `jobTemplates` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_job_templates_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::JobTemplate, crate::Error>,
                    > + 'a {
                        self.stream_job_templates_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `jobTemplates` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_job_templates_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<
                        Item = Result<crate::schemas::JobTemplate, crate::Error>,
                    > + 'a {
                        self.stream_job_templates_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `jobTemplates` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_job_templates_with_fields<T, F>(
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
                            #[serde(rename = "jobTemplates")]
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
                            let mut selector = concat!("nextPageToken,", "jobTemplates").to_owned();
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
                    #[doc = "\nExecute the request and yield each item in the `unreachable` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_unreachable<T>(
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
                        self.stream_unreachable_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `unreachable` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_unreachable_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<String, crate::Error>> + 'a
                    {
                        self.stream_unreachable_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `unreachable` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_unreachable_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<String, crate::Error>> + 'a
                    {
                        self.stream_unreachable_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `unreachable` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_unreachable_with_fields<T, F>(
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
                            #[serde(rename = "unreachable")]
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
                            let mut selector = concat!("nextPageToken,", "unreachable").to_owned();
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
                        Item = Result<crate::schemas::ListJobTemplatesResponse, crate::Error>,
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
                        Item = Result<crate::schemas::ListJobTemplatesResponse, crate::Error>,
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
                    ) -> Result<crate::schemas::ListJobTemplatesResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListJobTemplatesResponse, crate::Error>
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
                        let mut output = "https://transcoder.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/jobTemplates");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("filter", &self.filter)]);
                        req = req.query(&[("orderBy", &self.order_by)]);
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
            pub mod jobs {
                pub mod params {}
                pub struct JobsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> JobsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Creates a job in the specified region."]
                    pub fn create(
                        &self,
                        request: crate::schemas::Job,
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
                    #[doc = "Deletes a job."]
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
                            allow_missing: None,
                        }
                    }
                    #[doc = "Returns the job data."]
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
                    #[doc = "Lists jobs in the specified region."]
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
                            order_by: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                }
                #[doc = "Created via [JobsActions::create()](struct.JobsActions.html#method.create)"]
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::Job,
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
                    ) -> Result<crate::schemas::Job, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Job, crate::Error> {
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
                        let mut output = "https://transcoder.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/jobs");
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
                #[doc = "Created via [JobsActions::delete()](struct.JobsActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    allow_missing: ::std::option::Option<bool>,
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
                    #[doc = "If set to true, and the job is not found, the request will succeed but no action will be taken on the server."]
                    pub fn allow_missing(mut self, value: bool) -> Self {
                        self.allow_missing = Some(value);
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
                        let mut output = "https://transcoder.googleapis.com/".to_owned();
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
                        req = req.query(&[("allowMissing", &self.allow_missing)]);
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
                #[doc = "Created via [JobsActions::get()](struct.JobsActions.html#method.get)"]
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
                    ) -> Result<crate::schemas::Job, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Job, crate::Error> {
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
                        let mut output = "https://transcoder.googleapis.com/".to_owned();
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
                #[doc = "Created via [JobsActions::list()](struct.JobsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    filter: ::std::option::Option<String>,
                    order_by: ::std::option::Option<String>,
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
                    #[doc = "The filter expression, following the syntax outlined in https://google.aip.dev/160."]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "One or more fields to compare and use to sort the output. See https://google.aip.dev/132#ordering."]
                    pub fn order_by(mut self, value: impl Into<String>) -> Self {
                        self.order_by = Some(value.into());
                        self
                    }
                    #[doc = "The maximum number of items to return."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "The `next_page_token` value returned from a previous List request, if any."]
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
                    #[doc = "\nExecute the request and yield each item in the `jobs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_jobs<T>(
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
                        self.stream_jobs_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `jobs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_jobs_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<crate::schemas::Job, crate::Error>> + 'a
                    {
                        self.stream_jobs_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `jobs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_jobs_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<crate::schemas::Job, crate::Error>> + 'a
                    {
                        self.stream_jobs_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `jobs` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_jobs_with_fields<T, F>(
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
                            #[serde(rename = "jobs")]
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
                            let mut selector = concat!("nextPageToken,", "jobs").to_owned();
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
                    #[doc = "\nExecute the request and yield each item in the `unreachable` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                    pub fn stream_unreachable<T>(
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
                        self.stream_unreachable_with_fields(fields)
                    }
                    #[doc = "\nExecute the request and yield each item in the `unreachable` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                    pub fn stream_unreachable_with_default_fields(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<String, crate::Error>> + 'a
                    {
                        self.stream_unreachable_with_fields(None::<String>)
                    }
                    #[doc = "\nExecute the request and yield each item in the `unreachable` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                    pub fn stream_unreachable_with_all_fields(
                        self,
                    ) -> impl ::futures::Stream<Item = Result<String, crate::Error>> + 'a
                    {
                        self.stream_unreachable_with_fields(Some("*"))
                    }
                    #[doc = "\nExecute the request and yield each item in the `unreachable` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                    pub fn stream_unreachable_with_fields<T, F>(
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
                            #[serde(rename = "unreachable")]
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
                            let mut selector = concat!("nextPageToken,", "unreachable").to_owned();
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
                        Item = Result<crate::schemas::ListJobsResponse, crate::Error>,
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
                        Item = Result<crate::schemas::ListJobsResponse, crate::Error>,
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
                    ) -> Result<crate::schemas::ListJobsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListJobsResponse, crate::Error>
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
                        let mut output = "https://transcoder.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/jobs");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("filter", &self.filter)]);
                        req = req.query(&[("orderBy", &self.order_by)]);
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
