pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AffineTransformUnit {
        #[doc = "The units are unknown."]
        UnitUnspecified,
        #[doc = "An English Metric Unit (EMU) is defined as 1/360,000 of a centimeter\nand thus there are 914,400 EMUs per inch, and 12,700 EMUs per point."]
        Emu,
        #[doc = "A point, 1/72 of an inch."]
        Pt,
    }
    impl AffineTransformUnit {
        pub fn as_str(self) -> &'static str {
            match self {
                AffineTransformUnit::UnitUnspecified => "UNIT_UNSPECIFIED",
                AffineTransformUnit::Emu => "EMU",
                AffineTransformUnit::Pt => "PT",
            }
        }
    }
    impl ::std::fmt::Display for AffineTransformUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AffineTransformUnit {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AffineTransformUnit {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNIT_UNSPECIFIED" => AffineTransformUnit::UnitUnspecified,
                "EMU" => AffineTransformUnit::Emu,
                "PT" => AffineTransformUnit::Pt,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AffineTransform {
        #[doc = "The X coordinate scaling element."]
        #[serde(rename = "scaleX", default)]
        pub scale_x: Option<f64>,
        #[doc = "The Y coordinate scaling element."]
        #[serde(rename = "scaleY", default)]
        pub scale_y: Option<f64>,
        #[doc = "The X coordinate shearing element."]
        #[serde(rename = "shearX", default)]
        pub shear_x: Option<f64>,
        #[doc = "The Y coordinate shearing element."]
        #[serde(rename = "shearY", default)]
        pub shear_y: Option<f64>,
        #[doc = "The X coordinate translation element."]
        #[serde(rename = "translateX", default)]
        pub translate_x: Option<f64>,
        #[doc = "The Y coordinate translation element."]
        #[serde(rename = "translateY", default)]
        pub translate_y: Option<f64>,
        #[doc = "The units for translate elements."]
        #[serde(rename = "unit", default)]
        pub unit: Option<crate::schemas::AffineTransformUnit>,
    }
    impl ::field_selector::FieldSelector for AffineTransform {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AutoTextType {
        #[doc = "An unspecified autotext type."]
        TypeUnspecified,
        #[doc = "Type for autotext that represents the current slide number."]
        SlideNumber,
    }
    impl AutoTextType {
        pub fn as_str(self) -> &'static str {
            match self {
                AutoTextType::TypeUnspecified => "TYPE_UNSPECIFIED",
                AutoTextType::SlideNumber => "SLIDE_NUMBER",
            }
        }
    }
    impl ::std::fmt::Display for AutoTextType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AutoTextType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AutoTextType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_UNSPECIFIED" => AutoTextType::TypeUnspecified,
                "SLIDE_NUMBER" => AutoTextType::SlideNumber,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct AutoText {
        #[doc = "The rendered content of this auto text, if available."]
        #[serde(rename = "content", default)]
        pub content: Option<String>,
        #[doc = "The type of this auto text."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::AutoTextType>,
        #[doc = "The styling applied to this auto text."]
        #[serde(rename = "style", default)]
        pub style: Option<crate::schemas::TextStyle>,
    }
    impl ::field_selector::FieldSelector for AutoText {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BatchUpdatePresentationRequest {
        #[doc = "A list of updates to apply to the presentation."]
        #[serde(rename = "requests", default)]
        pub requests: Option<Vec<crate::schemas::Request>>,
        #[doc = "Provides control over how write requests are executed."]
        #[serde(rename = "writeControl", default)]
        pub write_control: Option<crate::schemas::WriteControl>,
    }
    impl ::field_selector::FieldSelector for BatchUpdatePresentationRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct BatchUpdatePresentationResponse {
        #[doc = "The presentation the updates were applied to."]
        #[serde(rename = "presentationId", default)]
        pub presentation_id: Option<String>,
        #[doc = "The reply of the updates.  This maps 1:1 with the updates, although\nreplies to some requests may be empty."]
        #[serde(rename = "replies", default)]
        pub replies: Option<Vec<crate::schemas::Response>>,
        #[doc = "The updated write control after applying the request."]
        #[serde(rename = "writeControl", default)]
        pub write_control: Option<crate::schemas::WriteControl>,
    }
    impl ::field_selector::FieldSelector for BatchUpdatePresentationResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Bullet {
        #[doc = "The paragraph specific text style applied to this bullet."]
        #[serde(rename = "bulletStyle", default)]
        pub bullet_style: Option<crate::schemas::TextStyle>,
        #[doc = "The rendered bullet glyph for this paragraph."]
        #[serde(rename = "glyph", default)]
        pub glyph: Option<String>,
        #[doc = "The ID of the list this paragraph belongs to."]
        #[serde(rename = "listId", default)]
        pub list_id: Option<String>,
        #[doc = "The nesting level of this paragraph in the list."]
        #[serde(rename = "nestingLevel", default)]
        pub nesting_level: Option<i32>,
    }
    impl ::field_selector::FieldSelector for Bullet {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ColorScheme {
        #[doc = "The ThemeColorType and corresponding concrete color pairs."]
        #[serde(rename = "colors", default)]
        pub colors: Option<Vec<crate::schemas::ThemeColorPair>>,
    }
    impl ::field_selector::FieldSelector for ColorScheme {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ColorStop {
        #[doc = "The alpha value of this color in the gradient band. Defaults to 1.0,\nfully opaque."]
        #[serde(rename = "alpha", default)]
        pub alpha: Option<f32>,
        #[doc = "The color of the gradient stop."]
        #[serde(rename = "color", default)]
        pub color: Option<crate::schemas::OpaqueColor>,
        #[doc = "The relative position of the color stop in the gradient band measured\nin percentage. The value should be in the interval [0.0, 1.0]."]
        #[serde(rename = "position", default)]
        pub position: Option<f32>,
    }
    impl ::field_selector::FieldSelector for ColorStop {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateImageRequest {
        #[doc = "The element properties for the image.\n\nWhen the aspect ratio of the provided size does not match the image aspect\nratio, the image is scaled and centered with respect to the size in order\nto maintain aspect ratio. The provided transform is applied after this\noperation.\n\nThe PageElementProperties.size property is\noptional. If you don't specify the size, the default size of the image is\nused.\n\nThe PageElementProperties.transform property is\noptional. If you don't specify a transform, the image will be placed at the\ntop left corner of the page."]
        #[serde(rename = "elementProperties", default)]
        pub element_properties: Option<crate::schemas::PageElementProperties>,
        #[doc = "A user-supplied object ID.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\n\nIf you don't specify an ID, a unique one is generated."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The image URL.\n\nThe image is fetched once at insertion time and a copy is stored for\ndisplay inside the presentation. Images must be less than 50MB in size,\ncannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF\nformat.\n\nThe provided URL can be at most 2 kB in length. The URL itself is saved\nwith the image, and exposed via the Image.source_url field."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreateImageRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateImageResponse {
        #[doc = "The object ID of the created image."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreateImageResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreateLineRequestCategory {
        #[doc = "Unspecified line category."]
        LineCategoryUnspecified,
        #[doc = "Straight connectors, including straight connector 1."]
        Straight,
        #[doc = "Bent connectors, including bent connector 2 to 5."]
        Bent,
        #[doc = "Curved connectors, including curved connector 2 to 5."]
        Curved,
    }
    impl CreateLineRequestCategory {
        pub fn as_str(self) -> &'static str {
            match self {
                CreateLineRequestCategory::LineCategoryUnspecified => "LINE_CATEGORY_UNSPECIFIED",
                CreateLineRequestCategory::Straight => "STRAIGHT",
                CreateLineRequestCategory::Bent => "BENT",
                CreateLineRequestCategory::Curved => "CURVED",
            }
        }
    }
    impl ::std::fmt::Display for CreateLineRequestCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreateLineRequestCategory {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateLineRequestCategory {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LINE_CATEGORY_UNSPECIFIED" => CreateLineRequestCategory::LineCategoryUnspecified,
                "STRAIGHT" => CreateLineRequestCategory::Straight,
                "BENT" => CreateLineRequestCategory::Bent,
                "CURVED" => CreateLineRequestCategory::Curved,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreateLineRequestLineCategory {
        #[doc = "Straight connectors, including straight connector 1. The is the default\ncategory when one is not specified."]
        Straight,
        #[doc = "Bent connectors, including bent connector 2 to 5."]
        Bent,
        #[doc = "Curved connectors, including curved connector 2 to 5."]
        Curved,
    }
    impl CreateLineRequestLineCategory {
        pub fn as_str(self) -> &'static str {
            match self {
                CreateLineRequestLineCategory::Straight => "STRAIGHT",
                CreateLineRequestLineCategory::Bent => "BENT",
                CreateLineRequestLineCategory::Curved => "CURVED",
            }
        }
    }
    impl ::std::fmt::Display for CreateLineRequestLineCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreateLineRequestLineCategory {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateLineRequestLineCategory {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STRAIGHT" => CreateLineRequestLineCategory::Straight,
                "BENT" => CreateLineRequestLineCategory::Bent,
                "CURVED" => CreateLineRequestLineCategory::Curved,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateLineRequest {
        #[doc = "The category of the line to be created.\n\nThe exact line type created is\ndetermined based on the category and how it's routed to connect to other\npage elements.\n\nIf you specify both a `category` and a `line_category`, the `category`\ntakes precedence.\n\nIf you do not specify a value for `category`, but specify a value for\n`line_category`, then the specified `line_category` value is used.\n\nIf you do not specify either, then STRAIGHT is used."]
        #[serde(rename = "category", default)]
        pub category: Option<crate::schemas::CreateLineRequestCategory>,
        #[doc = "The element properties for the line."]
        #[serde(rename = "elementProperties", default)]
        pub element_properties: Option<crate::schemas::PageElementProperties>,
        #[doc = "The category of the line to be created.\n\n<b>Deprecated</b>: use `category` instead.\n\nThe exact line type created is\ndetermined based on the category and how it's routed to connect to other\npage elements.\n\nIf you specify both a `category` and a `line_category`, the `category`\ntakes precedence."]
        #[serde(rename = "lineCategory", default)]
        pub line_category: Option<crate::schemas::CreateLineRequestLineCategory>,
        #[doc = "A user-supplied object ID.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\n\nIf you don't specify an ID, a unique one is generated."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreateLineRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateLineResponse {
        #[doc = "The object ID of the created line."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreateLineResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreateParagraphBulletsRequestBulletPreset {
        #[doc = "A bulleted list with a `DISC`, `CIRCLE` and `SQUARE` bullet glyph for the\nfirst 3 list nesting levels."]
        BulletDiscCircleSquare,
        #[doc = "A bulleted list with a `DIAMONDX`, `ARROW3D` and `SQUARE` bullet glyph for\nthe first 3 list nesting levels."]
        BulletDiamondxArrow3DSquare,
        #[doc = "A bulleted list with `CHECKBOX` bullet glyphs for all list nesting levels."]
        BulletCheckbox,
        #[doc = "A bulleted list with a `ARROW`, `DIAMOND` and `DISC` bullet glyph for\nthe first 3 list nesting levels."]
        BulletArrowDiamondDisc,
        #[doc = "A bulleted list with a `STAR`, `CIRCLE` and `SQUARE` bullet glyph for\nthe first 3 list nesting levels."]
        BulletStarCircleSquare,
        #[doc = "A bulleted list with a `ARROW3D`, `CIRCLE` and `SQUARE` bullet glyph for\nthe first 3 list nesting levels."]
        BulletArrow3DCircleSquare,
        #[doc = "A bulleted list with a `LEFTTRIANGLE`, `DIAMOND` and `DISC` bullet glyph\nfor the first 3 list nesting levels."]
        BulletLefttriangleDiamondDisc,
        #[doc = "A bulleted list with a `DIAMONDX`, `HOLLOWDIAMOND` and `SQUARE` bullet\nglyph for the first 3 list nesting levels."]
        BulletDiamondxHollowdiamondSquare,
        #[doc = "A bulleted list with a `DIAMOND`, `CIRCLE` and `SQUARE` bullet glyph\nfor the first 3 list nesting levels."]
        BulletDiamondCircleSquare,
        #[doc = "A numbered list with `DIGIT`, `ALPHA` and `ROMAN` numeric glyphs for\nthe first 3 list nesting levels, followed by periods."]
        NumberedDigitAlphaRoman,
        #[doc = "A numbered list with `DIGIT`, `ALPHA` and `ROMAN` numeric glyphs for\nthe first 3 list nesting levels, followed by parenthesis."]
        NumberedDigitAlphaRomanParens,
        #[doc = "A numbered list with `DIGIT` numeric glyphs separated by periods, where\neach nesting level uses the previous nesting level's glyph as a prefix.\nFor example: '1.', '1.1.', '2.', '2.2.'."]
        NumberedDigitNested,
        #[doc = "A numbered list with `UPPERALPHA`, `ALPHA` and `ROMAN` numeric glyphs for\nthe first 3 list nesting levels, followed by periods."]
        NumberedUpperalphaAlphaRoman,
        #[doc = "A numbered list with `UPPERROMAN`, `UPPERALPHA` and `DIGIT` numeric glyphs\nfor the first 3 list nesting levels, followed by periods."]
        NumberedUpperromanUpperalphaDigit,
        #[doc = "A numbered list with `ZERODIGIT`, `ALPHA` and `ROMAN` numeric glyphs for\nthe first 3 list nesting levels, followed by periods."]
        NumberedZerodigitAlphaRoman,
    }
    impl CreateParagraphBulletsRequestBulletPreset {
        pub fn as_str(self) -> &'static str {
            match self {
                CreateParagraphBulletsRequestBulletPreset::BulletDiscCircleSquare => {
                    "BULLET_DISC_CIRCLE_SQUARE"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletDiamondxArrow3DSquare => {
                    "BULLET_DIAMONDX_ARROW3D_SQUARE"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletCheckbox => "BULLET_CHECKBOX",
                CreateParagraphBulletsRequestBulletPreset::BulletArrowDiamondDisc => {
                    "BULLET_ARROW_DIAMOND_DISC"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletStarCircleSquare => {
                    "BULLET_STAR_CIRCLE_SQUARE"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletArrow3DCircleSquare => {
                    "BULLET_ARROW3D_CIRCLE_SQUARE"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletLefttriangleDiamondDisc => {
                    "BULLET_LEFTTRIANGLE_DIAMOND_DISC"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletDiamondxHollowdiamondSquare => {
                    "BULLET_DIAMONDX_HOLLOWDIAMOND_SQUARE"
                }
                CreateParagraphBulletsRequestBulletPreset::BulletDiamondCircleSquare => {
                    "BULLET_DIAMOND_CIRCLE_SQUARE"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedDigitAlphaRoman => {
                    "NUMBERED_DIGIT_ALPHA_ROMAN"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedDigitAlphaRomanParens => {
                    "NUMBERED_DIGIT_ALPHA_ROMAN_PARENS"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedDigitNested => {
                    "NUMBERED_DIGIT_NESTED"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedUpperalphaAlphaRoman => {
                    "NUMBERED_UPPERALPHA_ALPHA_ROMAN"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedUpperromanUpperalphaDigit => {
                    "NUMBERED_UPPERROMAN_UPPERALPHA_DIGIT"
                }
                CreateParagraphBulletsRequestBulletPreset::NumberedZerodigitAlphaRoman => {
                    "NUMBERED_ZERODIGIT_ALPHA_ROMAN"
                }
            }
        }
    }
    impl ::std::fmt::Display for CreateParagraphBulletsRequestBulletPreset {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreateParagraphBulletsRequestBulletPreset {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateParagraphBulletsRequestBulletPreset {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BULLET_DISC_CIRCLE_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletDiscCircleSquare
                }
                "BULLET_DIAMONDX_ARROW3D_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletDiamondxArrow3DSquare
                }
                "BULLET_CHECKBOX" => CreateParagraphBulletsRequestBulletPreset::BulletCheckbox,
                "BULLET_ARROW_DIAMOND_DISC" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletArrowDiamondDisc
                }
                "BULLET_STAR_CIRCLE_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletStarCircleSquare
                }
                "BULLET_ARROW3D_CIRCLE_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletArrow3DCircleSquare
                }
                "BULLET_LEFTTRIANGLE_DIAMOND_DISC" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletLefttriangleDiamondDisc
                }
                "BULLET_DIAMONDX_HOLLOWDIAMOND_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletDiamondxHollowdiamondSquare
                }
                "BULLET_DIAMOND_CIRCLE_SQUARE" => {
                    CreateParagraphBulletsRequestBulletPreset::BulletDiamondCircleSquare
                }
                "NUMBERED_DIGIT_ALPHA_ROMAN" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedDigitAlphaRoman
                }
                "NUMBERED_DIGIT_ALPHA_ROMAN_PARENS" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedDigitAlphaRomanParens
                }
                "NUMBERED_DIGIT_NESTED" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedDigitNested
                }
                "NUMBERED_UPPERALPHA_ALPHA_ROMAN" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedUpperalphaAlphaRoman
                }
                "NUMBERED_UPPERROMAN_UPPERALPHA_DIGIT" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedUpperromanUpperalphaDigit
                }
                "NUMBERED_ZERODIGIT_ALPHA_ROMAN" => {
                    CreateParagraphBulletsRequestBulletPreset::NumberedZerodigitAlphaRoman
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateParagraphBulletsRequest {
        #[doc = "The kinds of bullet glyphs to be used. Defaults to the\n`BULLET_DISC_CIRCLE_SQUARE` preset."]
        #[serde(rename = "bulletPreset", default)]
        pub bullet_preset: Option<crate::schemas::CreateParagraphBulletsRequestBulletPreset>,
        #[doc = "The optional table cell location if the text to be modified is in a table\ncell. If present, the object_id must refer to a table."]
        #[serde(rename = "cellLocation", default)]
        pub cell_location: Option<crate::schemas::TableCellLocation>,
        #[doc = "The object ID of the shape or table containing the text to add bullets to."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The range of text to apply the bullet presets to, based on TextElement indexes."]
        #[serde(rename = "textRange", default)]
        pub text_range: Option<crate::schemas::Range>,
    }
    impl ::field_selector::FieldSelector for CreateParagraphBulletsRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreateShapeRequestShapeType {
        #[doc = "The shape type that is not predefined."]
        TypeUnspecified,
        #[doc = "Text box shape."]
        TextBox,
        #[doc = "Rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'rect'."]
        Rectangle,
        #[doc = "Round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'roundRect'"]
        RoundRectangle,
        #[doc = "Ellipse shape. Corresponds to ECMA-376 ST_ShapeType 'ellipse'"]
        Ellipse,
        #[doc = "Curved arc shape. Corresponds to ECMA-376 ST_ShapeType 'arc'"]
        Arc,
        #[doc = "Bent arrow shape. Corresponds to ECMA-376 ST_ShapeType 'bentArrow'"]
        BentArrow,
        #[doc = "Bent up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'bentUpArrow'"]
        BentUpArrow,
        #[doc = "Bevel shape. Corresponds to ECMA-376 ST_ShapeType 'bevel'"]
        Bevel,
        #[doc = "Block arc shape. Corresponds to ECMA-376 ST_ShapeType 'blockArc'"]
        BlockArc,
        #[doc = "Brace pair shape. Corresponds to ECMA-376 ST_ShapeType 'bracePair'"]
        BracePair,
        #[doc = "Bracket pair shape. Corresponds to ECMA-376 ST_ShapeType 'bracketPair'"]
        BracketPair,
        #[doc = "Can shape. Corresponds to ECMA-376 ST_ShapeType 'can'"]
        Can,
        #[doc = "Chevron shape. Corresponds to ECMA-376 ST_ShapeType 'chevron'"]
        Chevron,
        #[doc = "Chord shape. Corresponds to ECMA-376 ST_ShapeType 'chord'"]
        Chord,
        #[doc = "Cloud shape. Corresponds to ECMA-376 ST_ShapeType 'cloud'"]
        Cloud,
        #[doc = "Corner shape. Corresponds to ECMA-376 ST_ShapeType 'corner'"]
        Corner,
        #[doc = "Cube shape. Corresponds to ECMA-376 ST_ShapeType 'cube'"]
        Cube,
        #[doc = "Curved down arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedDownArrow'"]
        CurvedDownArrow,
        #[doc = "Curved left arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedLeftArrow'"]
        CurvedLeftArrow,
        #[doc = "Curved right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedRightArrow'"]
        CurvedRightArrow,
        #[doc = "Curved up arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedUpArrow'"]
        CurvedUpArrow,
        #[doc = "Decagon shape. Corresponds to ECMA-376 ST_ShapeType 'decagon'"]
        Decagon,
        #[doc = "Diagonal stripe shape. Corresponds to ECMA-376 ST_ShapeType 'diagStripe'"]
        DiagonalStripe,
        #[doc = "Diamond shape. Corresponds to ECMA-376 ST_ShapeType 'diamond'"]
        Diamond,
        #[doc = "Dodecagon shape. Corresponds to ECMA-376 ST_ShapeType 'dodecagon'"]
        Dodecagon,
        #[doc = "Donut shape. Corresponds to ECMA-376 ST_ShapeType 'donut'"]
        Donut,
        #[doc = "Double wave shape. Corresponds to ECMA-376 ST_ShapeType 'doubleWave'"]
        DoubleWave,
        #[doc = "Down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'downArrow'"]
        DownArrow,
        #[doc = "Callout down arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'downArrowCallout'"]
        DownArrowCallout,
        #[doc = "Folded corner shape. Corresponds to ECMA-376 ST_ShapeType 'foldedCorner'"]
        FoldedCorner,
        #[doc = "Frame shape. Corresponds to ECMA-376 ST_ShapeType 'frame'"]
        Frame,
        #[doc = "Half frame shape. Corresponds to ECMA-376 ST_ShapeType 'halfFrame'"]
        HalfFrame,
        #[doc = "Heart shape. Corresponds to ECMA-376 ST_ShapeType 'heart'"]
        Heart,
        #[doc = "Heptagon shape. Corresponds to ECMA-376 ST_ShapeType 'heptagon'"]
        Heptagon,
        #[doc = "Hexagon shape. Corresponds to ECMA-376 ST_ShapeType 'hexagon'"]
        Hexagon,
        #[doc = "Home plate shape. Corresponds to ECMA-376 ST_ShapeType 'homePlate'"]
        HomePlate,
        #[doc = "Horizontal scroll shape. Corresponds to ECMA-376 ST_ShapeType\n'horizontalScroll'"]
        HorizontalScroll,
        #[doc = "Irregular seal 1 shape. Corresponds to ECMA-376 ST_ShapeType\n'irregularSeal1'"]
        IrregularSeal1,
        #[doc = "Irregular seal 2 shape. Corresponds to ECMA-376 ST_ShapeType\n'irregularSeal2'"]
        IrregularSeal2,
        #[doc = "Left arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftArrow'"]
        LeftArrow,
        #[doc = "Callout left arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftArrowCallout'"]
        LeftArrowCallout,
        #[doc = "Left brace shape. Corresponds to ECMA-376 ST_ShapeType 'leftBrace'"]
        LeftBrace,
        #[doc = "Left bracket shape. Corresponds to ECMA-376 ST_ShapeType 'leftBracket'"]
        LeftBracket,
        #[doc = "Left right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftRightArrow'"]
        LeftRightArrow,
        #[doc = "Callout left right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftRightArrowCallout'"]
        LeftRightArrowCallout,
        #[doc = "Left right up arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftRightUpArrow'"]
        LeftRightUpArrow,
        #[doc = "Left up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftUpArrow'"]
        LeftUpArrow,
        #[doc = "Lightning bolt shape. Corresponds to ECMA-376 ST_ShapeType\n'lightningBolt'"]
        LightningBolt,
        #[doc = "Divide math shape. Corresponds to ECMA-376 ST_ShapeType 'mathDivide'"]
        MathDivide,
        #[doc = "Equal math shape. Corresponds to ECMA-376 ST_ShapeType 'mathEqual'"]
        MathEqual,
        #[doc = "Minus math shape. Corresponds to ECMA-376 ST_ShapeType 'mathMinus'"]
        MathMinus,
        #[doc = "Multiply math shape. Corresponds to ECMA-376 ST_ShapeType 'mathMultiply'"]
        MathMultiply,
        #[doc = "Not equal math shape. Corresponds to ECMA-376 ST_ShapeType 'mathNotEqual'"]
        MathNotEqual,
        #[doc = "Plus math shape. Corresponds to ECMA-376 ST_ShapeType 'mathPlus'"]
        MathPlus,
        #[doc = "Moon shape. Corresponds to ECMA-376 ST_ShapeType 'moon'"]
        Moon,
        #[doc = "No smoking shape. Corresponds to ECMA-376 ST_ShapeType 'noSmoking'"]
        NoSmoking,
        #[doc = "Notched right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'notchedRightArrow'"]
        NotchedRightArrow,
        #[doc = "Octagon shape. Corresponds to ECMA-376 ST_ShapeType 'octagon'"]
        Octagon,
        #[doc = "Parallelogram shape. Corresponds to ECMA-376 ST_ShapeType 'parallelogram'"]
        Parallelogram,
        #[doc = "Pentagon shape. Corresponds to ECMA-376 ST_ShapeType 'pentagon'"]
        Pentagon,
        #[doc = "Pie shape. Corresponds to ECMA-376 ST_ShapeType 'pie'"]
        Pie,
        #[doc = "Plaque shape. Corresponds to ECMA-376 ST_ShapeType 'plaque'"]
        Plaque,
        #[doc = "Plus shape. Corresponds to ECMA-376 ST_ShapeType 'plus'"]
        Plus,
        #[doc = "Quad-arrow shape. Corresponds to ECMA-376 ST_ShapeType 'quadArrow'"]
        QuadArrow,
        #[doc = "Callout quad-arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'quadArrowCallout'"]
        QuadArrowCallout,
        #[doc = "Ribbon shape. Corresponds to ECMA-376 ST_ShapeType 'ribbon'"]
        Ribbon,
        #[doc = "Ribbon 2 shape. Corresponds to ECMA-376 ST_ShapeType 'ribbon2'"]
        Ribbon2,
        #[doc = "Right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'rightArrow'"]
        RightArrow,
        #[doc = "Callout right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'rightArrowCallout'"]
        RightArrowCallout,
        #[doc = "Right brace shape. Corresponds to ECMA-376 ST_ShapeType 'rightBrace'"]
        RightBrace,
        #[doc = "Right bracket shape. Corresponds to ECMA-376 ST_ShapeType 'rightBracket'"]
        RightBracket,
        #[doc = "One round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'round1Rect'"]
        Round1Rectangle,
        #[doc = "Two diagonal round corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'round2DiagRect'"]
        Round2DiagonalRectangle,
        #[doc = "Two same-side round corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'round2SameRect'"]
        Round2SameRectangle,
        #[doc = "Right triangle shape. Corresponds to ECMA-376 ST_ShapeType 'rtTriangle'"]
        RightTriangle,
        #[doc = "Smiley face shape. Corresponds to ECMA-376 ST_ShapeType 'smileyFace'"]
        SmileyFace,
        #[doc = "One snip corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'snip1Rect'"]
        Snip1Rectangle,
        #[doc = "Two diagonal snip corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'snip2DiagRect'"]
        Snip2DiagonalRectangle,
        #[doc = "Two same-side snip corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'snip2SameRect'"]
        Snip2SameRectangle,
        #[doc = "One snip one round corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'snipRoundRect'"]
        SnipRoundRectangle,
        #[doc = "Ten pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star10'"]
        Star10,
        #[doc = "Twelve pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star12'"]
        Star12,
        #[doc = "Sixteen pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star16'"]
        Star16,
        #[doc = "Twenty four pointed star shape. Corresponds to ECMA-376 ST_ShapeType\n'star24'"]
        Star24,
        #[doc = "Thirty two pointed star shape. Corresponds to ECMA-376 ST_ShapeType\n'star32'"]
        Star32,
        #[doc = "Four pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star4'"]
        Star4,
        #[doc = "Five pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star5'"]
        Star5,
        #[doc = "Six pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star6'"]
        Star6,
        #[doc = "Seven pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star7'"]
        Star7,
        #[doc = "Eight pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star8'"]
        Star8,
        #[doc = "Striped right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'stripedRightArrow'"]
        StripedRightArrow,
        #[doc = "Sun shape. Corresponds to ECMA-376 ST_ShapeType 'sun'"]
        Sun,
        #[doc = "Trapezoid shape. Corresponds to ECMA-376 ST_ShapeType 'trapezoid'"]
        Trapezoid,
        #[doc = "Triangle shape. Corresponds to ECMA-376 ST_ShapeType 'triangle'"]
        Triangle,
        #[doc = "Up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'upArrow'"]
        UpArrow,
        #[doc = "Callout up arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'upArrowCallout'"]
        UpArrowCallout,
        #[doc = "Up down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'upDownArrow'"]
        UpDownArrow,
        #[doc = "U-turn arrow shape. Corresponds to ECMA-376 ST_ShapeType 'uturnArrow'"]
        UturnArrow,
        #[doc = "Vertical scroll shape. Corresponds to ECMA-376 ST_ShapeType\n'verticalScroll'"]
        VerticalScroll,
        #[doc = "Wave shape. Corresponds to ECMA-376 ST_ShapeType 'wave'"]
        Wave,
        #[doc = "Callout wedge ellipse shape. Corresponds to ECMA-376 ST_ShapeType\n'wedgeEllipseCallout'"]
        WedgeEllipseCallout,
        #[doc = "Callout wedge rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'wedgeRectCallout'"]
        WedgeRectangleCallout,
        #[doc = "Callout wedge round rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'wedgeRoundRectCallout'"]
        WedgeRoundRectangleCallout,
        #[doc = "Alternate process flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartAlternateProcess'"]
        FlowChartAlternateProcess,
        #[doc = "Collate flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartCollate'"]
        FlowChartCollate,
        #[doc = "Connector flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartConnector'"]
        FlowChartConnector,
        #[doc = "Decision flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartDecision'"]
        FlowChartDecision,
        #[doc = "Delay flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartDelay'"]
        FlowChartDelay,
        #[doc = "Display flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartDisplay'"]
        FlowChartDisplay,
        #[doc = "Document flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartDocument'"]
        FlowChartDocument,
        #[doc = "Extract flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartExtract'"]
        FlowChartExtract,
        #[doc = "Input output flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartInputOutput'"]
        FlowChartInputOutput,
        #[doc = "Internal storage flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartInternalStorage'"]
        FlowChartInternalStorage,
        #[doc = "Magnetic disk flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMagneticDisk'"]
        FlowChartMagneticDisk,
        #[doc = "Magnetic drum flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMagneticDrum'"]
        FlowChartMagneticDrum,
        #[doc = "Magnetic tape flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMagneticTape'"]
        FlowChartMagneticTape,
        #[doc = "Manual input flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartManualInput'"]
        FlowChartManualInput,
        #[doc = "Manual operation flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartManualOperation'"]
        FlowChartManualOperation,
        #[doc = "Merge flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartMerge'"]
        FlowChartMerge,
        #[doc = "Multi-document flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMultidocument'"]
        FlowChartMultidocument,
        #[doc = "Offline storage flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartOfflineStorage'"]
        FlowChartOfflineStorage,
        #[doc = "Off-page connector flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartOffpageConnector'"]
        FlowChartOffpageConnector,
        #[doc = "Online storage flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartOnlineStorage'"]
        FlowChartOnlineStorage,
        #[doc = "Or flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartOr'"]
        FlowChartOr,
        #[doc = "Predefined process flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPredefinedProcess'"]
        FlowChartPredefinedProcess,
        #[doc = "Preparation flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPreparation'"]
        FlowChartPreparation,
        #[doc = "Process flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartProcess'"]
        FlowChartProcess,
        #[doc = "Punched card flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPunchedCard'"]
        FlowChartPunchedCard,
        #[doc = "Punched tape flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPunchedTape'"]
        FlowChartPunchedTape,
        #[doc = "Sort flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartSort'"]
        FlowChartSort,
        #[doc = "Summing junction flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartSummingJunction'"]
        FlowChartSummingJunction,
        #[doc = "Terminator flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartTerminator'"]
        FlowChartTerminator,
        #[doc = "East arrow shape."]
        ArrowEast,
        #[doc = "Northeast arrow shape."]
        ArrowNorthEast,
        #[doc = "North arrow shape."]
        ArrowNorth,
        #[doc = "Speech shape."]
        Speech,
        #[doc = "Star burst shape."]
        Starburst,
        #[doc = "Teardrop shape. Corresponds to ECMA-376 ST_ShapeType 'teardrop'"]
        Teardrop,
        #[doc = "Ellipse ribbon shape. Corresponds to ECMA-376 ST_ShapeType\n'ellipseRibbon'"]
        EllipseRibbon,
        #[doc = "Ellipse ribbon 2 shape. Corresponds to ECMA-376 ST_ShapeType\n'ellipseRibbon2'"]
        EllipseRibbon2,
        #[doc = "Callout cloud shape. Corresponds to ECMA-376 ST_ShapeType 'cloudCallout'"]
        CloudCallout,
        #[doc = "Custom shape."]
        Custom,
    }
    impl CreateShapeRequestShapeType {
        pub fn as_str(self) -> &'static str {
            match self {
                CreateShapeRequestShapeType::TypeUnspecified => "TYPE_UNSPECIFIED",
                CreateShapeRequestShapeType::TextBox => "TEXT_BOX",
                CreateShapeRequestShapeType::Rectangle => "RECTANGLE",
                CreateShapeRequestShapeType::RoundRectangle => "ROUND_RECTANGLE",
                CreateShapeRequestShapeType::Ellipse => "ELLIPSE",
                CreateShapeRequestShapeType::Arc => "ARC",
                CreateShapeRequestShapeType::BentArrow => "BENT_ARROW",
                CreateShapeRequestShapeType::BentUpArrow => "BENT_UP_ARROW",
                CreateShapeRequestShapeType::Bevel => "BEVEL",
                CreateShapeRequestShapeType::BlockArc => "BLOCK_ARC",
                CreateShapeRequestShapeType::BracePair => "BRACE_PAIR",
                CreateShapeRequestShapeType::BracketPair => "BRACKET_PAIR",
                CreateShapeRequestShapeType::Can => "CAN",
                CreateShapeRequestShapeType::Chevron => "CHEVRON",
                CreateShapeRequestShapeType::Chord => "CHORD",
                CreateShapeRequestShapeType::Cloud => "CLOUD",
                CreateShapeRequestShapeType::Corner => "CORNER",
                CreateShapeRequestShapeType::Cube => "CUBE",
                CreateShapeRequestShapeType::CurvedDownArrow => "CURVED_DOWN_ARROW",
                CreateShapeRequestShapeType::CurvedLeftArrow => "CURVED_LEFT_ARROW",
                CreateShapeRequestShapeType::CurvedRightArrow => "CURVED_RIGHT_ARROW",
                CreateShapeRequestShapeType::CurvedUpArrow => "CURVED_UP_ARROW",
                CreateShapeRequestShapeType::Decagon => "DECAGON",
                CreateShapeRequestShapeType::DiagonalStripe => "DIAGONAL_STRIPE",
                CreateShapeRequestShapeType::Diamond => "DIAMOND",
                CreateShapeRequestShapeType::Dodecagon => "DODECAGON",
                CreateShapeRequestShapeType::Donut => "DONUT",
                CreateShapeRequestShapeType::DoubleWave => "DOUBLE_WAVE",
                CreateShapeRequestShapeType::DownArrow => "DOWN_ARROW",
                CreateShapeRequestShapeType::DownArrowCallout => "DOWN_ARROW_CALLOUT",
                CreateShapeRequestShapeType::FoldedCorner => "FOLDED_CORNER",
                CreateShapeRequestShapeType::Frame => "FRAME",
                CreateShapeRequestShapeType::HalfFrame => "HALF_FRAME",
                CreateShapeRequestShapeType::Heart => "HEART",
                CreateShapeRequestShapeType::Heptagon => "HEPTAGON",
                CreateShapeRequestShapeType::Hexagon => "HEXAGON",
                CreateShapeRequestShapeType::HomePlate => "HOME_PLATE",
                CreateShapeRequestShapeType::HorizontalScroll => "HORIZONTAL_SCROLL",
                CreateShapeRequestShapeType::IrregularSeal1 => "IRREGULAR_SEAL_1",
                CreateShapeRequestShapeType::IrregularSeal2 => "IRREGULAR_SEAL_2",
                CreateShapeRequestShapeType::LeftArrow => "LEFT_ARROW",
                CreateShapeRequestShapeType::LeftArrowCallout => "LEFT_ARROW_CALLOUT",
                CreateShapeRequestShapeType::LeftBrace => "LEFT_BRACE",
                CreateShapeRequestShapeType::LeftBracket => "LEFT_BRACKET",
                CreateShapeRequestShapeType::LeftRightArrow => "LEFT_RIGHT_ARROW",
                CreateShapeRequestShapeType::LeftRightArrowCallout => "LEFT_RIGHT_ARROW_CALLOUT",
                CreateShapeRequestShapeType::LeftRightUpArrow => "LEFT_RIGHT_UP_ARROW",
                CreateShapeRequestShapeType::LeftUpArrow => "LEFT_UP_ARROW",
                CreateShapeRequestShapeType::LightningBolt => "LIGHTNING_BOLT",
                CreateShapeRequestShapeType::MathDivide => "MATH_DIVIDE",
                CreateShapeRequestShapeType::MathEqual => "MATH_EQUAL",
                CreateShapeRequestShapeType::MathMinus => "MATH_MINUS",
                CreateShapeRequestShapeType::MathMultiply => "MATH_MULTIPLY",
                CreateShapeRequestShapeType::MathNotEqual => "MATH_NOT_EQUAL",
                CreateShapeRequestShapeType::MathPlus => "MATH_PLUS",
                CreateShapeRequestShapeType::Moon => "MOON",
                CreateShapeRequestShapeType::NoSmoking => "NO_SMOKING",
                CreateShapeRequestShapeType::NotchedRightArrow => "NOTCHED_RIGHT_ARROW",
                CreateShapeRequestShapeType::Octagon => "OCTAGON",
                CreateShapeRequestShapeType::Parallelogram => "PARALLELOGRAM",
                CreateShapeRequestShapeType::Pentagon => "PENTAGON",
                CreateShapeRequestShapeType::Pie => "PIE",
                CreateShapeRequestShapeType::Plaque => "PLAQUE",
                CreateShapeRequestShapeType::Plus => "PLUS",
                CreateShapeRequestShapeType::QuadArrow => "QUAD_ARROW",
                CreateShapeRequestShapeType::QuadArrowCallout => "QUAD_ARROW_CALLOUT",
                CreateShapeRequestShapeType::Ribbon => "RIBBON",
                CreateShapeRequestShapeType::Ribbon2 => "RIBBON_2",
                CreateShapeRequestShapeType::RightArrow => "RIGHT_ARROW",
                CreateShapeRequestShapeType::RightArrowCallout => "RIGHT_ARROW_CALLOUT",
                CreateShapeRequestShapeType::RightBrace => "RIGHT_BRACE",
                CreateShapeRequestShapeType::RightBracket => "RIGHT_BRACKET",
                CreateShapeRequestShapeType::Round1Rectangle => "ROUND_1_RECTANGLE",
                CreateShapeRequestShapeType::Round2DiagonalRectangle => {
                    "ROUND_2_DIAGONAL_RECTANGLE"
                }
                CreateShapeRequestShapeType::Round2SameRectangle => "ROUND_2_SAME_RECTANGLE",
                CreateShapeRequestShapeType::RightTriangle => "RIGHT_TRIANGLE",
                CreateShapeRequestShapeType::SmileyFace => "SMILEY_FACE",
                CreateShapeRequestShapeType::Snip1Rectangle => "SNIP_1_RECTANGLE",
                CreateShapeRequestShapeType::Snip2DiagonalRectangle => "SNIP_2_DIAGONAL_RECTANGLE",
                CreateShapeRequestShapeType::Snip2SameRectangle => "SNIP_2_SAME_RECTANGLE",
                CreateShapeRequestShapeType::SnipRoundRectangle => "SNIP_ROUND_RECTANGLE",
                CreateShapeRequestShapeType::Star10 => "STAR_10",
                CreateShapeRequestShapeType::Star12 => "STAR_12",
                CreateShapeRequestShapeType::Star16 => "STAR_16",
                CreateShapeRequestShapeType::Star24 => "STAR_24",
                CreateShapeRequestShapeType::Star32 => "STAR_32",
                CreateShapeRequestShapeType::Star4 => "STAR_4",
                CreateShapeRequestShapeType::Star5 => "STAR_5",
                CreateShapeRequestShapeType::Star6 => "STAR_6",
                CreateShapeRequestShapeType::Star7 => "STAR_7",
                CreateShapeRequestShapeType::Star8 => "STAR_8",
                CreateShapeRequestShapeType::StripedRightArrow => "STRIPED_RIGHT_ARROW",
                CreateShapeRequestShapeType::Sun => "SUN",
                CreateShapeRequestShapeType::Trapezoid => "TRAPEZOID",
                CreateShapeRequestShapeType::Triangle => "TRIANGLE",
                CreateShapeRequestShapeType::UpArrow => "UP_ARROW",
                CreateShapeRequestShapeType::UpArrowCallout => "UP_ARROW_CALLOUT",
                CreateShapeRequestShapeType::UpDownArrow => "UP_DOWN_ARROW",
                CreateShapeRequestShapeType::UturnArrow => "UTURN_ARROW",
                CreateShapeRequestShapeType::VerticalScroll => "VERTICAL_SCROLL",
                CreateShapeRequestShapeType::Wave => "WAVE",
                CreateShapeRequestShapeType::WedgeEllipseCallout => "WEDGE_ELLIPSE_CALLOUT",
                CreateShapeRequestShapeType::WedgeRectangleCallout => "WEDGE_RECTANGLE_CALLOUT",
                CreateShapeRequestShapeType::WedgeRoundRectangleCallout => {
                    "WEDGE_ROUND_RECTANGLE_CALLOUT"
                }
                CreateShapeRequestShapeType::FlowChartAlternateProcess => {
                    "FLOW_CHART_ALTERNATE_PROCESS"
                }
                CreateShapeRequestShapeType::FlowChartCollate => "FLOW_CHART_COLLATE",
                CreateShapeRequestShapeType::FlowChartConnector => "FLOW_CHART_CONNECTOR",
                CreateShapeRequestShapeType::FlowChartDecision => "FLOW_CHART_DECISION",
                CreateShapeRequestShapeType::FlowChartDelay => "FLOW_CHART_DELAY",
                CreateShapeRequestShapeType::FlowChartDisplay => "FLOW_CHART_DISPLAY",
                CreateShapeRequestShapeType::FlowChartDocument => "FLOW_CHART_DOCUMENT",
                CreateShapeRequestShapeType::FlowChartExtract => "FLOW_CHART_EXTRACT",
                CreateShapeRequestShapeType::FlowChartInputOutput => "FLOW_CHART_INPUT_OUTPUT",
                CreateShapeRequestShapeType::FlowChartInternalStorage => {
                    "FLOW_CHART_INTERNAL_STORAGE"
                }
                CreateShapeRequestShapeType::FlowChartMagneticDisk => "FLOW_CHART_MAGNETIC_DISK",
                CreateShapeRequestShapeType::FlowChartMagneticDrum => "FLOW_CHART_MAGNETIC_DRUM",
                CreateShapeRequestShapeType::FlowChartMagneticTape => "FLOW_CHART_MAGNETIC_TAPE",
                CreateShapeRequestShapeType::FlowChartManualInput => "FLOW_CHART_MANUAL_INPUT",
                CreateShapeRequestShapeType::FlowChartManualOperation => {
                    "FLOW_CHART_MANUAL_OPERATION"
                }
                CreateShapeRequestShapeType::FlowChartMerge => "FLOW_CHART_MERGE",
                CreateShapeRequestShapeType::FlowChartMultidocument => "FLOW_CHART_MULTIDOCUMENT",
                CreateShapeRequestShapeType::FlowChartOfflineStorage => {
                    "FLOW_CHART_OFFLINE_STORAGE"
                }
                CreateShapeRequestShapeType::FlowChartOffpageConnector => {
                    "FLOW_CHART_OFFPAGE_CONNECTOR"
                }
                CreateShapeRequestShapeType::FlowChartOnlineStorage => "FLOW_CHART_ONLINE_STORAGE",
                CreateShapeRequestShapeType::FlowChartOr => "FLOW_CHART_OR",
                CreateShapeRequestShapeType::FlowChartPredefinedProcess => {
                    "FLOW_CHART_PREDEFINED_PROCESS"
                }
                CreateShapeRequestShapeType::FlowChartPreparation => "FLOW_CHART_PREPARATION",
                CreateShapeRequestShapeType::FlowChartProcess => "FLOW_CHART_PROCESS",
                CreateShapeRequestShapeType::FlowChartPunchedCard => "FLOW_CHART_PUNCHED_CARD",
                CreateShapeRequestShapeType::FlowChartPunchedTape => "FLOW_CHART_PUNCHED_TAPE",
                CreateShapeRequestShapeType::FlowChartSort => "FLOW_CHART_SORT",
                CreateShapeRequestShapeType::FlowChartSummingJunction => {
                    "FLOW_CHART_SUMMING_JUNCTION"
                }
                CreateShapeRequestShapeType::FlowChartTerminator => "FLOW_CHART_TERMINATOR",
                CreateShapeRequestShapeType::ArrowEast => "ARROW_EAST",
                CreateShapeRequestShapeType::ArrowNorthEast => "ARROW_NORTH_EAST",
                CreateShapeRequestShapeType::ArrowNorth => "ARROW_NORTH",
                CreateShapeRequestShapeType::Speech => "SPEECH",
                CreateShapeRequestShapeType::Starburst => "STARBURST",
                CreateShapeRequestShapeType::Teardrop => "TEARDROP",
                CreateShapeRequestShapeType::EllipseRibbon => "ELLIPSE_RIBBON",
                CreateShapeRequestShapeType::EllipseRibbon2 => "ELLIPSE_RIBBON_2",
                CreateShapeRequestShapeType::CloudCallout => "CLOUD_CALLOUT",
                CreateShapeRequestShapeType::Custom => "CUSTOM",
            }
        }
    }
    impl ::std::fmt::Display for CreateShapeRequestShapeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreateShapeRequestShapeType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateShapeRequestShapeType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_UNSPECIFIED" => CreateShapeRequestShapeType::TypeUnspecified,
                "TEXT_BOX" => CreateShapeRequestShapeType::TextBox,
                "RECTANGLE" => CreateShapeRequestShapeType::Rectangle,
                "ROUND_RECTANGLE" => CreateShapeRequestShapeType::RoundRectangle,
                "ELLIPSE" => CreateShapeRequestShapeType::Ellipse,
                "ARC" => CreateShapeRequestShapeType::Arc,
                "BENT_ARROW" => CreateShapeRequestShapeType::BentArrow,
                "BENT_UP_ARROW" => CreateShapeRequestShapeType::BentUpArrow,
                "BEVEL" => CreateShapeRequestShapeType::Bevel,
                "BLOCK_ARC" => CreateShapeRequestShapeType::BlockArc,
                "BRACE_PAIR" => CreateShapeRequestShapeType::BracePair,
                "BRACKET_PAIR" => CreateShapeRequestShapeType::BracketPair,
                "CAN" => CreateShapeRequestShapeType::Can,
                "CHEVRON" => CreateShapeRequestShapeType::Chevron,
                "CHORD" => CreateShapeRequestShapeType::Chord,
                "CLOUD" => CreateShapeRequestShapeType::Cloud,
                "CORNER" => CreateShapeRequestShapeType::Corner,
                "CUBE" => CreateShapeRequestShapeType::Cube,
                "CURVED_DOWN_ARROW" => CreateShapeRequestShapeType::CurvedDownArrow,
                "CURVED_LEFT_ARROW" => CreateShapeRequestShapeType::CurvedLeftArrow,
                "CURVED_RIGHT_ARROW" => CreateShapeRequestShapeType::CurvedRightArrow,
                "CURVED_UP_ARROW" => CreateShapeRequestShapeType::CurvedUpArrow,
                "DECAGON" => CreateShapeRequestShapeType::Decagon,
                "DIAGONAL_STRIPE" => CreateShapeRequestShapeType::DiagonalStripe,
                "DIAMOND" => CreateShapeRequestShapeType::Diamond,
                "DODECAGON" => CreateShapeRequestShapeType::Dodecagon,
                "DONUT" => CreateShapeRequestShapeType::Donut,
                "DOUBLE_WAVE" => CreateShapeRequestShapeType::DoubleWave,
                "DOWN_ARROW" => CreateShapeRequestShapeType::DownArrow,
                "DOWN_ARROW_CALLOUT" => CreateShapeRequestShapeType::DownArrowCallout,
                "FOLDED_CORNER" => CreateShapeRequestShapeType::FoldedCorner,
                "FRAME" => CreateShapeRequestShapeType::Frame,
                "HALF_FRAME" => CreateShapeRequestShapeType::HalfFrame,
                "HEART" => CreateShapeRequestShapeType::Heart,
                "HEPTAGON" => CreateShapeRequestShapeType::Heptagon,
                "HEXAGON" => CreateShapeRequestShapeType::Hexagon,
                "HOME_PLATE" => CreateShapeRequestShapeType::HomePlate,
                "HORIZONTAL_SCROLL" => CreateShapeRequestShapeType::HorizontalScroll,
                "IRREGULAR_SEAL_1" => CreateShapeRequestShapeType::IrregularSeal1,
                "IRREGULAR_SEAL_2" => CreateShapeRequestShapeType::IrregularSeal2,
                "LEFT_ARROW" => CreateShapeRequestShapeType::LeftArrow,
                "LEFT_ARROW_CALLOUT" => CreateShapeRequestShapeType::LeftArrowCallout,
                "LEFT_BRACE" => CreateShapeRequestShapeType::LeftBrace,
                "LEFT_BRACKET" => CreateShapeRequestShapeType::LeftBracket,
                "LEFT_RIGHT_ARROW" => CreateShapeRequestShapeType::LeftRightArrow,
                "LEFT_RIGHT_ARROW_CALLOUT" => CreateShapeRequestShapeType::LeftRightArrowCallout,
                "LEFT_RIGHT_UP_ARROW" => CreateShapeRequestShapeType::LeftRightUpArrow,
                "LEFT_UP_ARROW" => CreateShapeRequestShapeType::LeftUpArrow,
                "LIGHTNING_BOLT" => CreateShapeRequestShapeType::LightningBolt,
                "MATH_DIVIDE" => CreateShapeRequestShapeType::MathDivide,
                "MATH_EQUAL" => CreateShapeRequestShapeType::MathEqual,
                "MATH_MINUS" => CreateShapeRequestShapeType::MathMinus,
                "MATH_MULTIPLY" => CreateShapeRequestShapeType::MathMultiply,
                "MATH_NOT_EQUAL" => CreateShapeRequestShapeType::MathNotEqual,
                "MATH_PLUS" => CreateShapeRequestShapeType::MathPlus,
                "MOON" => CreateShapeRequestShapeType::Moon,
                "NO_SMOKING" => CreateShapeRequestShapeType::NoSmoking,
                "NOTCHED_RIGHT_ARROW" => CreateShapeRequestShapeType::NotchedRightArrow,
                "OCTAGON" => CreateShapeRequestShapeType::Octagon,
                "PARALLELOGRAM" => CreateShapeRequestShapeType::Parallelogram,
                "PENTAGON" => CreateShapeRequestShapeType::Pentagon,
                "PIE" => CreateShapeRequestShapeType::Pie,
                "PLAQUE" => CreateShapeRequestShapeType::Plaque,
                "PLUS" => CreateShapeRequestShapeType::Plus,
                "QUAD_ARROW" => CreateShapeRequestShapeType::QuadArrow,
                "QUAD_ARROW_CALLOUT" => CreateShapeRequestShapeType::QuadArrowCallout,
                "RIBBON" => CreateShapeRequestShapeType::Ribbon,
                "RIBBON_2" => CreateShapeRequestShapeType::Ribbon2,
                "RIGHT_ARROW" => CreateShapeRequestShapeType::RightArrow,
                "RIGHT_ARROW_CALLOUT" => CreateShapeRequestShapeType::RightArrowCallout,
                "RIGHT_BRACE" => CreateShapeRequestShapeType::RightBrace,
                "RIGHT_BRACKET" => CreateShapeRequestShapeType::RightBracket,
                "ROUND_1_RECTANGLE" => CreateShapeRequestShapeType::Round1Rectangle,
                "ROUND_2_DIAGONAL_RECTANGLE" => {
                    CreateShapeRequestShapeType::Round2DiagonalRectangle
                }
                "ROUND_2_SAME_RECTANGLE" => CreateShapeRequestShapeType::Round2SameRectangle,
                "RIGHT_TRIANGLE" => CreateShapeRequestShapeType::RightTriangle,
                "SMILEY_FACE" => CreateShapeRequestShapeType::SmileyFace,
                "SNIP_1_RECTANGLE" => CreateShapeRequestShapeType::Snip1Rectangle,
                "SNIP_2_DIAGONAL_RECTANGLE" => CreateShapeRequestShapeType::Snip2DiagonalRectangle,
                "SNIP_2_SAME_RECTANGLE" => CreateShapeRequestShapeType::Snip2SameRectangle,
                "SNIP_ROUND_RECTANGLE" => CreateShapeRequestShapeType::SnipRoundRectangle,
                "STAR_10" => CreateShapeRequestShapeType::Star10,
                "STAR_12" => CreateShapeRequestShapeType::Star12,
                "STAR_16" => CreateShapeRequestShapeType::Star16,
                "STAR_24" => CreateShapeRequestShapeType::Star24,
                "STAR_32" => CreateShapeRequestShapeType::Star32,
                "STAR_4" => CreateShapeRequestShapeType::Star4,
                "STAR_5" => CreateShapeRequestShapeType::Star5,
                "STAR_6" => CreateShapeRequestShapeType::Star6,
                "STAR_7" => CreateShapeRequestShapeType::Star7,
                "STAR_8" => CreateShapeRequestShapeType::Star8,
                "STRIPED_RIGHT_ARROW" => CreateShapeRequestShapeType::StripedRightArrow,
                "SUN" => CreateShapeRequestShapeType::Sun,
                "TRAPEZOID" => CreateShapeRequestShapeType::Trapezoid,
                "TRIANGLE" => CreateShapeRequestShapeType::Triangle,
                "UP_ARROW" => CreateShapeRequestShapeType::UpArrow,
                "UP_ARROW_CALLOUT" => CreateShapeRequestShapeType::UpArrowCallout,
                "UP_DOWN_ARROW" => CreateShapeRequestShapeType::UpDownArrow,
                "UTURN_ARROW" => CreateShapeRequestShapeType::UturnArrow,
                "VERTICAL_SCROLL" => CreateShapeRequestShapeType::VerticalScroll,
                "WAVE" => CreateShapeRequestShapeType::Wave,
                "WEDGE_ELLIPSE_CALLOUT" => CreateShapeRequestShapeType::WedgeEllipseCallout,
                "WEDGE_RECTANGLE_CALLOUT" => CreateShapeRequestShapeType::WedgeRectangleCallout,
                "WEDGE_ROUND_RECTANGLE_CALLOUT" => {
                    CreateShapeRequestShapeType::WedgeRoundRectangleCallout
                }
                "FLOW_CHART_ALTERNATE_PROCESS" => {
                    CreateShapeRequestShapeType::FlowChartAlternateProcess
                }
                "FLOW_CHART_COLLATE" => CreateShapeRequestShapeType::FlowChartCollate,
                "FLOW_CHART_CONNECTOR" => CreateShapeRequestShapeType::FlowChartConnector,
                "FLOW_CHART_DECISION" => CreateShapeRequestShapeType::FlowChartDecision,
                "FLOW_CHART_DELAY" => CreateShapeRequestShapeType::FlowChartDelay,
                "FLOW_CHART_DISPLAY" => CreateShapeRequestShapeType::FlowChartDisplay,
                "FLOW_CHART_DOCUMENT" => CreateShapeRequestShapeType::FlowChartDocument,
                "FLOW_CHART_EXTRACT" => CreateShapeRequestShapeType::FlowChartExtract,
                "FLOW_CHART_INPUT_OUTPUT" => CreateShapeRequestShapeType::FlowChartInputOutput,
                "FLOW_CHART_INTERNAL_STORAGE" => {
                    CreateShapeRequestShapeType::FlowChartInternalStorage
                }
                "FLOW_CHART_MAGNETIC_DISK" => CreateShapeRequestShapeType::FlowChartMagneticDisk,
                "FLOW_CHART_MAGNETIC_DRUM" => CreateShapeRequestShapeType::FlowChartMagneticDrum,
                "FLOW_CHART_MAGNETIC_TAPE" => CreateShapeRequestShapeType::FlowChartMagneticTape,
                "FLOW_CHART_MANUAL_INPUT" => CreateShapeRequestShapeType::FlowChartManualInput,
                "FLOW_CHART_MANUAL_OPERATION" => {
                    CreateShapeRequestShapeType::FlowChartManualOperation
                }
                "FLOW_CHART_MERGE" => CreateShapeRequestShapeType::FlowChartMerge,
                "FLOW_CHART_MULTIDOCUMENT" => CreateShapeRequestShapeType::FlowChartMultidocument,
                "FLOW_CHART_OFFLINE_STORAGE" => {
                    CreateShapeRequestShapeType::FlowChartOfflineStorage
                }
                "FLOW_CHART_OFFPAGE_CONNECTOR" => {
                    CreateShapeRequestShapeType::FlowChartOffpageConnector
                }
                "FLOW_CHART_ONLINE_STORAGE" => CreateShapeRequestShapeType::FlowChartOnlineStorage,
                "FLOW_CHART_OR" => CreateShapeRequestShapeType::FlowChartOr,
                "FLOW_CHART_PREDEFINED_PROCESS" => {
                    CreateShapeRequestShapeType::FlowChartPredefinedProcess
                }
                "FLOW_CHART_PREPARATION" => CreateShapeRequestShapeType::FlowChartPreparation,
                "FLOW_CHART_PROCESS" => CreateShapeRequestShapeType::FlowChartProcess,
                "FLOW_CHART_PUNCHED_CARD" => CreateShapeRequestShapeType::FlowChartPunchedCard,
                "FLOW_CHART_PUNCHED_TAPE" => CreateShapeRequestShapeType::FlowChartPunchedTape,
                "FLOW_CHART_SORT" => CreateShapeRequestShapeType::FlowChartSort,
                "FLOW_CHART_SUMMING_JUNCTION" => {
                    CreateShapeRequestShapeType::FlowChartSummingJunction
                }
                "FLOW_CHART_TERMINATOR" => CreateShapeRequestShapeType::FlowChartTerminator,
                "ARROW_EAST" => CreateShapeRequestShapeType::ArrowEast,
                "ARROW_NORTH_EAST" => CreateShapeRequestShapeType::ArrowNorthEast,
                "ARROW_NORTH" => CreateShapeRequestShapeType::ArrowNorth,
                "SPEECH" => CreateShapeRequestShapeType::Speech,
                "STARBURST" => CreateShapeRequestShapeType::Starburst,
                "TEARDROP" => CreateShapeRequestShapeType::Teardrop,
                "ELLIPSE_RIBBON" => CreateShapeRequestShapeType::EllipseRibbon,
                "ELLIPSE_RIBBON_2" => CreateShapeRequestShapeType::EllipseRibbon2,
                "CLOUD_CALLOUT" => CreateShapeRequestShapeType::CloudCallout,
                "CUSTOM" => CreateShapeRequestShapeType::Custom,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateShapeRequest {
        #[doc = "The element properties for the shape."]
        #[serde(rename = "elementProperties", default)]
        pub element_properties: Option<crate::schemas::PageElementProperties>,
        #[doc = "A user-supplied object ID.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\nIf empty, a unique identifier will be generated."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The shape type."]
        #[serde(rename = "shapeType", default)]
        pub shape_type: Option<crate::schemas::CreateShapeRequestShapeType>,
    }
    impl ::field_selector::FieldSelector for CreateShapeRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateShapeResponse {
        #[doc = "The object ID of the created shape."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreateShapeResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreateSheetsChartRequestLinkingMode {
        #[doc = "The chart is not associated with the source spreadsheet and cannot be\nupdated. A chart that is not linked will be inserted as an image."]
        NotLinkedImage,
        #[doc = "Linking the chart allows it to be updated, and other collaborators will\nsee a link to the spreadsheet."]
        Linked,
    }
    impl CreateSheetsChartRequestLinkingMode {
        pub fn as_str(self) -> &'static str {
            match self {
                CreateSheetsChartRequestLinkingMode::NotLinkedImage => "NOT_LINKED_IMAGE",
                CreateSheetsChartRequestLinkingMode::Linked => "LINKED",
            }
        }
    }
    impl ::std::fmt::Display for CreateSheetsChartRequestLinkingMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreateSheetsChartRequestLinkingMode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateSheetsChartRequestLinkingMode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NOT_LINKED_IMAGE" => CreateSheetsChartRequestLinkingMode::NotLinkedImage,
                "LINKED" => CreateSheetsChartRequestLinkingMode::Linked,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateSheetsChartRequest {
        #[doc = "The ID of the specific chart in the Google Sheets spreadsheet."]
        #[serde(rename = "chartId", default)]
        pub chart_id: Option<i32>,
        #[doc = "The element properties for the chart.\n\nWhen the aspect ratio of the provided size does not match the chart aspect\nratio, the chart is scaled and centered with respect to the size in order\nto maintain aspect ratio. The provided transform is applied after this\noperation."]
        #[serde(rename = "elementProperties", default)]
        pub element_properties: Option<crate::schemas::PageElementProperties>,
        #[doc = "The mode with which the chart is linked to the source spreadsheet. When\nnot specified, the chart will be an image that is not linked."]
        #[serde(rename = "linkingMode", default)]
        pub linking_mode: Option<crate::schemas::CreateSheetsChartRequestLinkingMode>,
        #[doc = "A user-supplied object ID.\n\nIf specified, the ID must be unique among all pages and page elements in\nthe presentation. The ID should start with a word character [a-zA-Z0-9_]\nand then followed by any number of the following characters [a-zA-Z0-9_-:].\nThe length of the ID should not be less than 5 or greater than 50.\nIf empty, a unique identifier will be generated."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The ID of the Google Sheets spreadsheet that contains the chart."]
        #[serde(rename = "spreadsheetId", default)]
        pub spreadsheet_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreateSheetsChartRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateSheetsChartResponse {
        #[doc = "The object ID of the created chart."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreateSheetsChartResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateSlideRequest {
        #[doc = "The optional zero-based index indicating where to insert the slides.\n\nIf you don't specify an index, the new slide is created at the end."]
        #[serde(rename = "insertionIndex", default)]
        pub insertion_index: Option<i32>,
        #[doc = "A user-supplied object ID.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\n\nIf you don't specify an ID, a unique one is generated."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "An optional list of object ID mappings from the placeholder(s) on the layout to the placeholder(s)\nthat will be created on the new slide from that specified layout. Can only\nbe used when `slide_layout_reference` is specified."]
        #[serde(rename = "placeholderIdMappings", default)]
        pub placeholder_id_mappings: Option<Vec<crate::schemas::LayoutPlaceholderIdMapping>>,
        #[doc = "Layout reference of the slide to be inserted, based on the *current\nmaster*, which is one of the following:\n\n- The master of the previous slide index.\n- The master of the first slide, if the insertion_index is zero.\n- The first master in the presentation, if there are no slides.\n\nIf the LayoutReference is not found in the current master, a 400 bad\nrequest error is returned.\n\nIf you don't specify a layout reference, then the new slide will use the\npredefined layout `BLANK`."]
        #[serde(rename = "slideLayoutReference", default)]
        pub slide_layout_reference: Option<crate::schemas::LayoutReference>,
    }
    impl ::field_selector::FieldSelector for CreateSlideRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateSlideResponse {
        #[doc = "The object ID of the created slide."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreateSlideResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateTableRequest {
        #[doc = "Number of columns in the table."]
        #[serde(rename = "columns", default)]
        pub columns: Option<i32>,
        #[doc = "The element properties for the table.\n\nThe table will be created at the provided size, subject to a minimum size.\nIf no size is provided, the table will be automatically sized.\n\nTable transforms must have a scale of 1 and no shear components. If no\ntransform is provided, the table will be centered on the page."]
        #[serde(rename = "elementProperties", default)]
        pub element_properties: Option<crate::schemas::PageElementProperties>,
        #[doc = "A user-supplied object ID.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\n\nIf you don't specify an ID, a unique one is generated."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "Number of rows in the table."]
        #[serde(rename = "rows", default)]
        pub rows: Option<i32>,
    }
    impl ::field_selector::FieldSelector for CreateTableRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateTableResponse {
        #[doc = "The object ID of the created table."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreateTableResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CreateVideoRequestSource {
        #[doc = "The video source is unspecified."]
        SourceUnspecified,
        #[doc = "The video source is YouTube."]
        Youtube,
        #[doc = "The video source is Google Drive."]
        Drive,
    }
    impl CreateVideoRequestSource {
        pub fn as_str(self) -> &'static str {
            match self {
                CreateVideoRequestSource::SourceUnspecified => "SOURCE_UNSPECIFIED",
                CreateVideoRequestSource::Youtube => "YOUTUBE",
                CreateVideoRequestSource::Drive => "DRIVE",
            }
        }
    }
    impl ::std::fmt::Display for CreateVideoRequestSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CreateVideoRequestSource {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CreateVideoRequestSource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SOURCE_UNSPECIFIED" => CreateVideoRequestSource::SourceUnspecified,
                "YOUTUBE" => CreateVideoRequestSource::Youtube,
                "DRIVE" => CreateVideoRequestSource::Drive,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CreateVideoRequest {
        #[doc = "The element properties for the video.\n\nThe PageElementProperties.size property is\noptional. If you don't specify a size, a default size is chosen by the\nserver.\n\nThe PageElementProperties.transform property is\noptional. The transform must not have shear components.\nIf you don't specify a transform, the video will be placed at the top left\ncorner of the page."]
        #[serde(rename = "elementProperties", default)]
        pub element_properties: Option<crate::schemas::PageElementProperties>,
        #[doc = "The video source's unique identifier for this video.\n\ne.g. For YouTube video https://www.youtube.com/watch?v=7U3axjORYZ0,\nthe ID is 7U3axjORYZ0. For a Google Drive video\nhttps://drive.google.com/file/d/1xCgQLFTJi5_Xl8DgW_lcUYq5e-q6Hi5Q the ID\nis 1xCgQLFTJi5_Xl8DgW_lcUYq5e-q6Hi5Q."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "A user-supplied object ID.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\n\nIf you don't specify an ID, a unique one is generated."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The video source."]
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::CreateVideoRequestSource>,
    }
    impl ::field_selector::FieldSelector for CreateVideoRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreateVideoResponse {
        #[doc = "The object ID of the created video."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for CreateVideoResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CropProperties {
        #[doc = "The rotation angle of the crop window around its center, in radians.\nRotation angle is applied after the offset."]
        #[serde(rename = "angle", default)]
        pub angle: Option<f32>,
        #[doc = "The offset specifies the bottom edge of the crop rectangle that is located\nabove the original bounding rectangle bottom edge, relative to the object's\noriginal height."]
        #[serde(rename = "bottomOffset", default)]
        pub bottom_offset: Option<f32>,
        #[doc = "The offset specifies the left edge of the crop rectangle that is located to\nthe right of the original bounding rectangle left edge, relative to the\nobject's original width."]
        #[serde(rename = "leftOffset", default)]
        pub left_offset: Option<f32>,
        #[doc = "The offset specifies the right edge of the crop rectangle that is located\nto the left of the original bounding rectangle right edge, relative to the\nobject's original width."]
        #[serde(rename = "rightOffset", default)]
        pub right_offset: Option<f32>,
        #[doc = "The offset specifies the top edge of the crop rectangle that is located\nbelow the original bounding rectangle top edge, relative to the object's\noriginal height."]
        #[serde(rename = "topOffset", default)]
        pub top_offset: Option<f32>,
    }
    impl ::field_selector::FieldSelector for CropProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteObjectRequest {
        #[doc = "The object ID of the page or page element to delete.\n\nIf after a delete operation a group contains\nonly 1 or no page elements, the group is also deleted.\n\nIf a placeholder is deleted on a layout, any empty inheriting shapes are\nalso deleted."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for DeleteObjectRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteParagraphBulletsRequest {
        #[doc = "The optional table cell location if the text to be modified is in a table\ncell. If present, the object_id must refer to a table."]
        #[serde(rename = "cellLocation", default)]
        pub cell_location: Option<crate::schemas::TableCellLocation>,
        #[doc = "The object ID of the shape or table containing the text to delete bullets\nfrom."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The range of text to delete bullets from, based on TextElement indexes."]
        #[serde(rename = "textRange", default)]
        pub text_range: Option<crate::schemas::Range>,
    }
    impl ::field_selector::FieldSelector for DeleteParagraphBulletsRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteTableColumnRequest {
        #[doc = "The reference table cell location from which a column will be deleted.\n\nThe column this cell spans will be deleted. If this is a merged cell,\nmultiple columns will be deleted. If no columns remain in the table after\nthis deletion, the whole table is deleted."]
        #[serde(rename = "cellLocation", default)]
        pub cell_location: Option<crate::schemas::TableCellLocation>,
        #[doc = "The table to delete columns from."]
        #[serde(rename = "tableObjectId", default)]
        pub table_object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for DeleteTableColumnRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteTableRowRequest {
        #[doc = "The reference table cell location from which a row will be deleted.\n\nThe row this cell spans will be deleted. If this is a merged cell, multiple\nrows will be deleted. If no rows remain in the table after this deletion,\nthe whole table is deleted."]
        #[serde(rename = "cellLocation", default)]
        pub cell_location: Option<crate::schemas::TableCellLocation>,
        #[doc = "The table to delete rows from."]
        #[serde(rename = "tableObjectId", default)]
        pub table_object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for DeleteTableRowRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteTextRequest {
        #[doc = "The optional table cell location if the text is to be deleted from a table\ncell. If present, the object_id must refer to a table."]
        #[serde(rename = "cellLocation", default)]
        pub cell_location: Option<crate::schemas::TableCellLocation>,
        #[doc = "The object ID of the shape or table from which the text will be deleted."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The range of text to delete, based on TextElement indexes.\n\nThere is always an implicit newline character at the end of a shape's or\ntable cell's text that cannot be deleted. `Range.Type.ALL` will use the\ncorrect bounds, but care must be taken when specifying explicit bounds for\nrange types `FROM_START_INDEX` and `FIXED_RANGE`. For example, if the text\nis \"ABC\", followed by an implicit newline, then the maximum value is 2 for\n`text_range.start_index` and 3 for `text_range.end_index`.\n\nDeleting text that crosses a paragraph boundary may result in changes\nto paragraph styles and lists as the two paragraphs are merged.\n\nRanges that include only one code unit of a surrogate pair are expanded to\ninclude both code units."]
        #[serde(rename = "textRange", default)]
        pub text_range: Option<crate::schemas::Range>,
    }
    impl ::field_selector::FieldSelector for DeleteTextRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DimensionUnit {
        #[doc = "The units are unknown."]
        UnitUnspecified,
        #[doc = "An English Metric Unit (EMU) is defined as 1/360,000 of a centimeter\nand thus there are 914,400 EMUs per inch, and 12,700 EMUs per point."]
        Emu,
        #[doc = "A point, 1/72 of an inch."]
        Pt,
    }
    impl DimensionUnit {
        pub fn as_str(self) -> &'static str {
            match self {
                DimensionUnit::UnitUnspecified => "UNIT_UNSPECIFIED",
                DimensionUnit::Emu => "EMU",
                DimensionUnit::Pt => "PT",
            }
        }
    }
    impl ::std::fmt::Display for DimensionUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DimensionUnit {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DimensionUnit {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "UNIT_UNSPECIFIED" => DimensionUnit::UnitUnspecified,
                "EMU" => DimensionUnit::Emu,
                "PT" => DimensionUnit::Pt,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Dimension {
        #[doc = "The magnitude."]
        #[serde(rename = "magnitude", default)]
        pub magnitude: Option<f64>,
        #[doc = "The units for magnitude."]
        #[serde(rename = "unit", default)]
        pub unit: Option<crate::schemas::DimensionUnit>,
    }
    impl ::field_selector::FieldSelector for Dimension {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DuplicateObjectRequest {
        #[doc = "The ID of the object to duplicate."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The object being duplicated may contain other objects, for example when\nduplicating a slide or a group page element. This map defines how the IDs\nof duplicated objects are generated: the keys are the IDs of the original\nobjects and its values are the IDs that will be assigned to the\ncorresponding duplicate object. The ID of the source object's duplicate\nmay be specified in this map as well, using the same value of the\n`object_id` field as a key and the newly desired ID as the value.\n\nAll keys must correspond to existing IDs in the presentation. All values\nmust be unique in the presentation and must start with an alphanumeric\ncharacter or an underscore (matches regex `[a-zA-Z0-9_]`); remaining\ncharacters may include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`). The length of the new ID must not be less than 5 or\ngreater than 50.\n\nIf any IDs of source objects are omitted from the map, a new random ID will\nbe assigned. If the map is empty or unset, all duplicate objects will\nreceive a new random ID."]
        #[serde(rename = "objectIds", default)]
        pub object_ids: Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::field_selector::FieldSelector for DuplicateObjectRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DuplicateObjectResponse {
        #[doc = "The ID of the new duplicate object."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for DuplicateObjectResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Group {
        #[doc = "The collection of elements in the group. The minimum size of a group is 2."]
        #[serde(rename = "children", default)]
        pub children: Option<Vec<crate::schemas::PageElement>>,
    }
    impl ::field_selector::FieldSelector for Group {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GroupObjectsRequest {
        #[doc = "The object IDs of the objects to group.\n\nOnly page elements can be grouped. There should be at least two page\nelements on the same page that are not already in another group. Some page\nelements, such as videos, tables and placeholder shapes cannot be grouped."]
        #[serde(rename = "childrenObjectIds", default)]
        pub children_object_ids: Option<Vec<String>>,
        #[doc = "A user-supplied object ID for the group to be created.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\n\nIf you don't specify an ID, a unique one is generated."]
        #[serde(rename = "groupObjectId", default)]
        pub group_object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for GroupObjectsRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GroupObjectsResponse {
        #[doc = "The object ID of the created group."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for GroupObjectsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Image {
        #[doc = "An URL to an image with a default lifetime of 30 minutes.\nThis URL is tagged with the account of the requester. Anyone with the URL\neffectively accesses the image as the original requester. Access to the\nimage may be lost if the presentation's sharing settings change."]
        #[serde(rename = "contentUrl", default)]
        pub content_url: Option<String>,
        #[doc = "The properties of the image."]
        #[serde(rename = "imageProperties", default)]
        pub image_properties: Option<crate::schemas::ImageProperties>,
        #[doc = "The source URL is the URL used to insert the image. The source URL can be\nempty."]
        #[serde(rename = "sourceUrl", default)]
        pub source_url: Option<String>,
    }
    impl ::field_selector::FieldSelector for Image {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ImageProperties {
        #[doc = "The brightness effect of the image. The value should be in the interval\n[-1.0, 1.0], where 0 means no effect. This property is read-only."]
        #[serde(rename = "brightness", default)]
        pub brightness: Option<f32>,
        #[doc = "The contrast effect of the image. The value should be in the interval\n[-1.0, 1.0], where 0 means no effect. This property is read-only."]
        #[serde(rename = "contrast", default)]
        pub contrast: Option<f32>,
        #[doc = "The crop properties of the image. If not set, the image is not cropped.\nThis property is read-only."]
        #[serde(rename = "cropProperties", default)]
        pub crop_properties: Option<crate::schemas::CropProperties>,
        #[doc = "The hyperlink destination of the image. If unset, there is no link."]
        #[serde(rename = "link", default)]
        pub link: Option<crate::schemas::Link>,
        #[doc = "The outline of the image. If not set, the image has no outline."]
        #[serde(rename = "outline", default)]
        pub outline: Option<crate::schemas::Outline>,
        #[doc = "The recolor effect of the image. If not set, the image is not recolored.\nThis property is read-only."]
        #[serde(rename = "recolor", default)]
        pub recolor: Option<crate::schemas::Recolor>,
        #[doc = "The shadow of the image. If not set, the image has no shadow. This property\nis read-only."]
        #[serde(rename = "shadow", default)]
        pub shadow: Option<crate::schemas::Shadow>,
        #[doc = "The transparency effect of the image. The value should be in the interval\n[0.0, 1.0], where 0 means no effect and 1 means completely transparent.\nThis property is read-only."]
        #[serde(rename = "transparency", default)]
        pub transparency: Option<f32>,
    }
    impl ::field_selector::FieldSelector for ImageProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InsertTableColumnsRequest {
        #[doc = "The reference table cell location from which columns will be inserted.\n\nA new column will be inserted to the left (or right) of the column where\nthe reference cell is. If the reference cell is a merged cell, a new\ncolumn will be inserted to the left (or right) of the merged cell."]
        #[serde(rename = "cellLocation", default)]
        pub cell_location: Option<crate::schemas::TableCellLocation>,
        #[doc = "Whether to insert new columns to the right of the reference cell location.\n\n- `True`: insert to the right.\n- `False`: insert to the left."]
        #[serde(rename = "insertRight", default)]
        pub insert_right: Option<bool>,
        #[doc = "The number of columns to be inserted. Maximum 20 per request."]
        #[serde(rename = "number", default)]
        pub number: Option<i32>,
        #[doc = "The table to insert columns into."]
        #[serde(rename = "tableObjectId", default)]
        pub table_object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for InsertTableColumnsRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InsertTableRowsRequest {
        #[doc = "The reference table cell location from which rows will be inserted.\n\nA new row will be inserted above (or below) the row where the reference\ncell is. If the reference cell is a merged cell, a new row will be\ninserted above (or below) the merged cell."]
        #[serde(rename = "cellLocation", default)]
        pub cell_location: Option<crate::schemas::TableCellLocation>,
        #[doc = "Whether to insert new rows below the reference cell location.\n\n- `True`: insert below the cell.\n- `False`: insert above the cell."]
        #[serde(rename = "insertBelow", default)]
        pub insert_below: Option<bool>,
        #[doc = "The number of rows to be inserted. Maximum 20 per request."]
        #[serde(rename = "number", default)]
        pub number: Option<i32>,
        #[doc = "The table to insert rows into."]
        #[serde(rename = "tableObjectId", default)]
        pub table_object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for InsertTableRowsRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InsertTextRequest {
        #[doc = "The optional table cell location if the text is to be inserted into a table\ncell. If present, the object_id must refer to a table."]
        #[serde(rename = "cellLocation", default)]
        pub cell_location: Option<crate::schemas::TableCellLocation>,
        #[doc = "The index where the text will be inserted, in Unicode code units, based\non TextElement indexes.\n\nThe index is zero-based and is computed from the start of the string.\nThe index may be adjusted to prevent insertions inside Unicode grapheme\nclusters. In these cases, the text will be inserted immediately after the\ngrapheme cluster."]
        #[serde(rename = "insertionIndex", default)]
        pub insertion_index: Option<i32>,
        #[doc = "The object ID of the shape or table where the text will be inserted."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The text to be inserted.\n\nInserting a newline character will implicitly create a new\nParagraphMarker at that index.\nThe paragraph style of the new paragraph will be copied from the paragraph\nat the current insertion index, including lists and bullets.\n\nText styles for inserted text will be determined automatically, generally\npreserving the styling of neighboring text. In most cases, the text will be\nadded to the TextRun that exists at the\ninsertion index.\n\nSome control characters (U+0000-U+0008, U+000C-U+001F) and characters\nfrom the Unicode Basic Multilingual Plane Private Use Area (U+E000-U+F8FF)\nwill be stripped out of the inserted text."]
        #[serde(rename = "text", default)]
        pub text: Option<String>,
    }
    impl ::field_selector::FieldSelector for InsertTextRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LayoutPlaceholderIdMapping {
        #[doc = "The placeholder on a layout that will be applied to a slide. Only type and index are needed. For example, a\npredefined `TITLE_AND_BODY` layout may usually have a TITLE placeholder\nwith index 0 and a BODY placeholder with index 0."]
        #[serde(rename = "layoutPlaceholder", default)]
        pub layout_placeholder: Option<crate::schemas::Placeholder>,
        #[doc = "The object ID of the placeholder on a layout that will be applied\nto a slide."]
        #[serde(rename = "layoutPlaceholderObjectId", default)]
        pub layout_placeholder_object_id: Option<String>,
        #[doc = "A user-supplied object ID for the placeholder identified above that to be\ncreated onto a slide.\n\nIf you specify an ID, it must be unique among all pages and page elements\nin the presentation. The ID must start with an alphanumeric character or an\nunderscore (matches regex `[a-zA-Z0-9_]`); remaining characters\nmay include those as well as a hyphen or colon (matches regex\n`[a-zA-Z0-9_-:]`).\nThe length of the ID must not be less than 5 or greater than 50.\n\nIf you don't specify an ID, a unique one is generated."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for LayoutPlaceholderIdMapping {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LayoutProperties {
        #[doc = "The human-readable name of the layout."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[doc = "The object ID of the master that this layout is based on."]
        #[serde(rename = "masterObjectId", default)]
        pub master_object_id: Option<String>,
        #[doc = "The name of the layout."]
        #[serde(rename = "name", default)]
        pub name: Option<String>,
    }
    impl ::field_selector::FieldSelector for LayoutProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LayoutReferencePredefinedLayout {
        #[doc = "Unspecified layout."]
        PredefinedLayoutUnspecified,
        #[doc = "Blank layout, with no placeholders."]
        Blank,
        #[doc = "Layout with a caption at the bottom."]
        CaptionOnly,
        #[doc = "Layout with a title and a subtitle."]
        Title,
        #[doc = "Layout with a title and body."]
        TitleAndBody,
        #[doc = "Layout with a title and two columns."]
        TitleAndTwoColumns,
        #[doc = "Layout with only a title."]
        TitleOnly,
        #[doc = "Layout with a section title."]
        SectionHeader,
        #[doc = "Layout with a title and subtitle on one side and description on the other."]
        SectionTitleAndDescription,
        #[doc = "Layout with one title and one body, arranged in a single column."]
        OneColumnText,
        #[doc = "Layout with a main point."]
        MainPoint,
        #[doc = "Layout with a big number heading."]
        BigNumber,
    }
    impl LayoutReferencePredefinedLayout {
        pub fn as_str(self) -> &'static str {
            match self {
                LayoutReferencePredefinedLayout::PredefinedLayoutUnspecified => {
                    "PREDEFINED_LAYOUT_UNSPECIFIED"
                }
                LayoutReferencePredefinedLayout::Blank => "BLANK",
                LayoutReferencePredefinedLayout::CaptionOnly => "CAPTION_ONLY",
                LayoutReferencePredefinedLayout::Title => "TITLE",
                LayoutReferencePredefinedLayout::TitleAndBody => "TITLE_AND_BODY",
                LayoutReferencePredefinedLayout::TitleAndTwoColumns => "TITLE_AND_TWO_COLUMNS",
                LayoutReferencePredefinedLayout::TitleOnly => "TITLE_ONLY",
                LayoutReferencePredefinedLayout::SectionHeader => "SECTION_HEADER",
                LayoutReferencePredefinedLayout::SectionTitleAndDescription => {
                    "SECTION_TITLE_AND_DESCRIPTION"
                }
                LayoutReferencePredefinedLayout::OneColumnText => "ONE_COLUMN_TEXT",
                LayoutReferencePredefinedLayout::MainPoint => "MAIN_POINT",
                LayoutReferencePredefinedLayout::BigNumber => "BIG_NUMBER",
            }
        }
    }
    impl ::std::fmt::Display for LayoutReferencePredefinedLayout {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LayoutReferencePredefinedLayout {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LayoutReferencePredefinedLayout {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PREDEFINED_LAYOUT_UNSPECIFIED" => {
                    LayoutReferencePredefinedLayout::PredefinedLayoutUnspecified
                }
                "BLANK" => LayoutReferencePredefinedLayout::Blank,
                "CAPTION_ONLY" => LayoutReferencePredefinedLayout::CaptionOnly,
                "TITLE" => LayoutReferencePredefinedLayout::Title,
                "TITLE_AND_BODY" => LayoutReferencePredefinedLayout::TitleAndBody,
                "TITLE_AND_TWO_COLUMNS" => LayoutReferencePredefinedLayout::TitleAndTwoColumns,
                "TITLE_ONLY" => LayoutReferencePredefinedLayout::TitleOnly,
                "SECTION_HEADER" => LayoutReferencePredefinedLayout::SectionHeader,
                "SECTION_TITLE_AND_DESCRIPTION" => {
                    LayoutReferencePredefinedLayout::SectionTitleAndDescription
                }
                "ONE_COLUMN_TEXT" => LayoutReferencePredefinedLayout::OneColumnText,
                "MAIN_POINT" => LayoutReferencePredefinedLayout::MainPoint,
                "BIG_NUMBER" => LayoutReferencePredefinedLayout::BigNumber,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LayoutReference {
        #[doc = "Layout ID: the object ID of one of the layouts in the presentation."]
        #[serde(rename = "layoutId", default)]
        pub layout_id: Option<String>,
        #[doc = "Predefined layout."]
        #[serde(rename = "predefinedLayout", default)]
        pub predefined_layout: Option<crate::schemas::LayoutReferencePredefinedLayout>,
    }
    impl ::field_selector::FieldSelector for LayoutReference {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LineLineCategory {
        #[doc = "Unspecified line category."]
        LineCategoryUnspecified,
        #[doc = "Straight connectors, including straight connector 1."]
        Straight,
        #[doc = "Bent connectors, including bent connector 2 to 5."]
        Bent,
        #[doc = "Curved connectors, including curved connector 2 to 5."]
        Curved,
    }
    impl LineLineCategory {
        pub fn as_str(self) -> &'static str {
            match self {
                LineLineCategory::LineCategoryUnspecified => "LINE_CATEGORY_UNSPECIFIED",
                LineLineCategory::Straight => "STRAIGHT",
                LineLineCategory::Bent => "BENT",
                LineLineCategory::Curved => "CURVED",
            }
        }
    }
    impl ::std::fmt::Display for LineLineCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LineLineCategory {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LineLineCategory {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LINE_CATEGORY_UNSPECIFIED" => LineLineCategory::LineCategoryUnspecified,
                "STRAIGHT" => LineLineCategory::Straight,
                "BENT" => LineLineCategory::Bent,
                "CURVED" => LineLineCategory::Curved,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LineLineType {
        #[doc = "An unspecified line type."]
        TypeUnspecified,
        #[doc = "Straight connector 1 form. Corresponds to ECMA-376 ST_ShapeType\n'straightConnector1'."]
        StraightConnector1,
        #[doc = "Bent connector 2 form. Corresponds to ECMA-376 ST_ShapeType\n'bentConnector2'."]
        BentConnector2,
        #[doc = "Bent connector 3 form. Corresponds to ECMA-376 ST_ShapeType\n'bentConnector3'."]
        BentConnector3,
        #[doc = "Bent connector 4 form. Corresponds to ECMA-376 ST_ShapeType\n'bentConnector4'."]
        BentConnector4,
        #[doc = "Bent connector 5 form. Corresponds to ECMA-376 ST_ShapeType\n'bentConnector5'."]
        BentConnector5,
        #[doc = "Curved connector 2 form. Corresponds to ECMA-376 ST_ShapeType\n'curvedConnector2'."]
        CurvedConnector2,
        #[doc = "Curved connector 3 form. Corresponds to ECMA-376 ST_ShapeType\n'curvedConnector3'."]
        CurvedConnector3,
        #[doc = "Curved connector 4 form. Corresponds to ECMA-376 ST_ShapeType\n'curvedConnector4'."]
        CurvedConnector4,
        #[doc = "Curved connector 5 form. Corresponds to ECMA-376 ST_ShapeType\n'curvedConnector5'."]
        CurvedConnector5,
        #[doc = "Straight line. Corresponds to ECMA-376 ST_ShapeType 'line'. This line\ntype is not a connector."]
        StraightLine,
    }
    impl LineLineType {
        pub fn as_str(self) -> &'static str {
            match self {
                LineLineType::TypeUnspecified => "TYPE_UNSPECIFIED",
                LineLineType::StraightConnector1 => "STRAIGHT_CONNECTOR_1",
                LineLineType::BentConnector2 => "BENT_CONNECTOR_2",
                LineLineType::BentConnector3 => "BENT_CONNECTOR_3",
                LineLineType::BentConnector4 => "BENT_CONNECTOR_4",
                LineLineType::BentConnector5 => "BENT_CONNECTOR_5",
                LineLineType::CurvedConnector2 => "CURVED_CONNECTOR_2",
                LineLineType::CurvedConnector3 => "CURVED_CONNECTOR_3",
                LineLineType::CurvedConnector4 => "CURVED_CONNECTOR_4",
                LineLineType::CurvedConnector5 => "CURVED_CONNECTOR_5",
                LineLineType::StraightLine => "STRAIGHT_LINE",
            }
        }
    }
    impl ::std::fmt::Display for LineLineType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LineLineType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LineLineType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_UNSPECIFIED" => LineLineType::TypeUnspecified,
                "STRAIGHT_CONNECTOR_1" => LineLineType::StraightConnector1,
                "BENT_CONNECTOR_2" => LineLineType::BentConnector2,
                "BENT_CONNECTOR_3" => LineLineType::BentConnector3,
                "BENT_CONNECTOR_4" => LineLineType::BentConnector4,
                "BENT_CONNECTOR_5" => LineLineType::BentConnector5,
                "CURVED_CONNECTOR_2" => LineLineType::CurvedConnector2,
                "CURVED_CONNECTOR_3" => LineLineType::CurvedConnector3,
                "CURVED_CONNECTOR_4" => LineLineType::CurvedConnector4,
                "CURVED_CONNECTOR_5" => LineLineType::CurvedConnector5,
                "STRAIGHT_LINE" => LineLineType::StraightLine,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Line {
        #[doc = "The category of the line.\n\nIt matches the `category` specified in CreateLineRequest, and can be updated with\nUpdateLineCategoryRequest."]
        #[serde(rename = "lineCategory", default)]
        pub line_category: Option<crate::schemas::LineLineCategory>,
        #[doc = "The properties of the line."]
        #[serde(rename = "lineProperties", default)]
        pub line_properties: Option<crate::schemas::LineProperties>,
        #[doc = "The type of the line."]
        #[serde(rename = "lineType", default)]
        pub line_type: Option<crate::schemas::LineLineType>,
    }
    impl ::field_selector::FieldSelector for Line {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LineConnection {
        #[doc = "The object ID of the connected page element.\n\nSome page elements, such as groups, tables, and lines\ndo not have connection sites and therefore cannot be connected to a\nconnector line."]
        #[serde(rename = "connectedObjectId", default)]
        pub connected_object_id: Option<String>,
        #[doc = "The index of the connection site on the connected page element.\n\nIn most cases, it corresponds to the predefined connection site index from\nthe ECMA-376 standard. More information on those connection sites can be\nfound in the description of the \"cnx\" attribute in section 20.1.9.9 and\nAnnex H. \"Predefined DrawingML Shape and Text Geometries\" of \"Office Open\nXML File Formats-Fundamentals and Markup Language Reference\", part 1 of\n[ECMA-376 5th edition]\n(http://www.ecma-international.org/publications/standards/Ecma-376.htm).\n\nThe position of each connection site can also be viewed from Slides editor."]
        #[serde(rename = "connectionSiteIndex", default)]
        pub connection_site_index: Option<i32>,
    }
    impl ::field_selector::FieldSelector for LineConnection {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LineFill {
        #[doc = "Solid color fill."]
        #[serde(rename = "solidFill", default)]
        pub solid_fill: Option<crate::schemas::SolidFill>,
    }
    impl ::field_selector::FieldSelector for LineFill {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LinePropertiesDashStyle {
        #[doc = "Unspecified dash style."]
        DashStyleUnspecified,
        #[doc = "Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'.\nThis is the default dash style."]
        Solid,
        #[doc = "Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'."]
        Dot,
        #[doc = "Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'."]
        Dash,
        #[doc = "Alternating dashes and dots. Corresponds to ECMA-376 ST_PresetLineDashVal\nvalue 'dashDot'."]
        DashDot,
        #[doc = "Line with large dashes. Corresponds to ECMA-376 ST_PresetLineDashVal\nvalue 'lgDash'."]
        LongDash,
        #[doc = "Alternating large dashes and dots. Corresponds to ECMA-376\nST_PresetLineDashVal value 'lgDashDot'."]
        LongDashDot,
    }
    impl LinePropertiesDashStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                LinePropertiesDashStyle::DashStyleUnspecified => "DASH_STYLE_UNSPECIFIED",
                LinePropertiesDashStyle::Solid => "SOLID",
                LinePropertiesDashStyle::Dot => "DOT",
                LinePropertiesDashStyle::Dash => "DASH",
                LinePropertiesDashStyle::DashDot => "DASH_DOT",
                LinePropertiesDashStyle::LongDash => "LONG_DASH",
                LinePropertiesDashStyle::LongDashDot => "LONG_DASH_DOT",
            }
        }
    }
    impl ::std::fmt::Display for LinePropertiesDashStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LinePropertiesDashStyle {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LinePropertiesDashStyle {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DASH_STYLE_UNSPECIFIED" => LinePropertiesDashStyle::DashStyleUnspecified,
                "SOLID" => LinePropertiesDashStyle::Solid,
                "DOT" => LinePropertiesDashStyle::Dot,
                "DASH" => LinePropertiesDashStyle::Dash,
                "DASH_DOT" => LinePropertiesDashStyle::DashDot,
                "LONG_DASH" => LinePropertiesDashStyle::LongDash,
                "LONG_DASH_DOT" => LinePropertiesDashStyle::LongDashDot,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LinePropertiesEndArrow {
        #[doc = "An unspecified arrow style."]
        ArrowStyleUnspecified,
        #[doc = "No arrow."]
        None,
        #[doc = "Arrow with notched back. Corresponds to ECMA-376 ST_LineEndType value\n'stealth'."]
        StealthArrow,
        #[doc = "Filled arrow. Corresponds to ECMA-376 ST_LineEndType value 'triangle'."]
        FillArrow,
        #[doc = "Filled circle. Corresponds to ECMA-376 ST_LineEndType value 'oval'."]
        FillCircle,
        #[doc = "Filled square."]
        FillSquare,
        #[doc = "Filled diamond. Corresponds to ECMA-376 ST_LineEndType value 'diamond'."]
        FillDiamond,
        #[doc = "Hollow arrow."]
        OpenArrow,
        #[doc = "Hollow circle."]
        OpenCircle,
        #[doc = "Hollow square."]
        OpenSquare,
        #[doc = "Hollow diamond."]
        OpenDiamond,
    }
    impl LinePropertiesEndArrow {
        pub fn as_str(self) -> &'static str {
            match self {
                LinePropertiesEndArrow::ArrowStyleUnspecified => "ARROW_STYLE_UNSPECIFIED",
                LinePropertiesEndArrow::None => "NONE",
                LinePropertiesEndArrow::StealthArrow => "STEALTH_ARROW",
                LinePropertiesEndArrow::FillArrow => "FILL_ARROW",
                LinePropertiesEndArrow::FillCircle => "FILL_CIRCLE",
                LinePropertiesEndArrow::FillSquare => "FILL_SQUARE",
                LinePropertiesEndArrow::FillDiamond => "FILL_DIAMOND",
                LinePropertiesEndArrow::OpenArrow => "OPEN_ARROW",
                LinePropertiesEndArrow::OpenCircle => "OPEN_CIRCLE",
                LinePropertiesEndArrow::OpenSquare => "OPEN_SQUARE",
                LinePropertiesEndArrow::OpenDiamond => "OPEN_DIAMOND",
            }
        }
    }
    impl ::std::fmt::Display for LinePropertiesEndArrow {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LinePropertiesEndArrow {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LinePropertiesEndArrow {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARROW_STYLE_UNSPECIFIED" => LinePropertiesEndArrow::ArrowStyleUnspecified,
                "NONE" => LinePropertiesEndArrow::None,
                "STEALTH_ARROW" => LinePropertiesEndArrow::StealthArrow,
                "FILL_ARROW" => LinePropertiesEndArrow::FillArrow,
                "FILL_CIRCLE" => LinePropertiesEndArrow::FillCircle,
                "FILL_SQUARE" => LinePropertiesEndArrow::FillSquare,
                "FILL_DIAMOND" => LinePropertiesEndArrow::FillDiamond,
                "OPEN_ARROW" => LinePropertiesEndArrow::OpenArrow,
                "OPEN_CIRCLE" => LinePropertiesEndArrow::OpenCircle,
                "OPEN_SQUARE" => LinePropertiesEndArrow::OpenSquare,
                "OPEN_DIAMOND" => LinePropertiesEndArrow::OpenDiamond,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LinePropertiesStartArrow {
        #[doc = "An unspecified arrow style."]
        ArrowStyleUnspecified,
        #[doc = "No arrow."]
        None,
        #[doc = "Arrow with notched back. Corresponds to ECMA-376 ST_LineEndType value\n'stealth'."]
        StealthArrow,
        #[doc = "Filled arrow. Corresponds to ECMA-376 ST_LineEndType value 'triangle'."]
        FillArrow,
        #[doc = "Filled circle. Corresponds to ECMA-376 ST_LineEndType value 'oval'."]
        FillCircle,
        #[doc = "Filled square."]
        FillSquare,
        #[doc = "Filled diamond. Corresponds to ECMA-376 ST_LineEndType value 'diamond'."]
        FillDiamond,
        #[doc = "Hollow arrow."]
        OpenArrow,
        #[doc = "Hollow circle."]
        OpenCircle,
        #[doc = "Hollow square."]
        OpenSquare,
        #[doc = "Hollow diamond."]
        OpenDiamond,
    }
    impl LinePropertiesStartArrow {
        pub fn as_str(self) -> &'static str {
            match self {
                LinePropertiesStartArrow::ArrowStyleUnspecified => "ARROW_STYLE_UNSPECIFIED",
                LinePropertiesStartArrow::None => "NONE",
                LinePropertiesStartArrow::StealthArrow => "STEALTH_ARROW",
                LinePropertiesStartArrow::FillArrow => "FILL_ARROW",
                LinePropertiesStartArrow::FillCircle => "FILL_CIRCLE",
                LinePropertiesStartArrow::FillSquare => "FILL_SQUARE",
                LinePropertiesStartArrow::FillDiamond => "FILL_DIAMOND",
                LinePropertiesStartArrow::OpenArrow => "OPEN_ARROW",
                LinePropertiesStartArrow::OpenCircle => "OPEN_CIRCLE",
                LinePropertiesStartArrow::OpenSquare => "OPEN_SQUARE",
                LinePropertiesStartArrow::OpenDiamond => "OPEN_DIAMOND",
            }
        }
    }
    impl ::std::fmt::Display for LinePropertiesStartArrow {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LinePropertiesStartArrow {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LinePropertiesStartArrow {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARROW_STYLE_UNSPECIFIED" => LinePropertiesStartArrow::ArrowStyleUnspecified,
                "NONE" => LinePropertiesStartArrow::None,
                "STEALTH_ARROW" => LinePropertiesStartArrow::StealthArrow,
                "FILL_ARROW" => LinePropertiesStartArrow::FillArrow,
                "FILL_CIRCLE" => LinePropertiesStartArrow::FillCircle,
                "FILL_SQUARE" => LinePropertiesStartArrow::FillSquare,
                "FILL_DIAMOND" => LinePropertiesStartArrow::FillDiamond,
                "OPEN_ARROW" => LinePropertiesStartArrow::OpenArrow,
                "OPEN_CIRCLE" => LinePropertiesStartArrow::OpenCircle,
                "OPEN_SQUARE" => LinePropertiesStartArrow::OpenSquare,
                "OPEN_DIAMOND" => LinePropertiesStartArrow::OpenDiamond,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct LineProperties {
        #[doc = "The dash style of the line."]
        #[serde(rename = "dashStyle", default)]
        pub dash_style: Option<crate::schemas::LinePropertiesDashStyle>,
        #[doc = "The style of the arrow at the end of the line."]
        #[serde(rename = "endArrow", default)]
        pub end_arrow: Option<crate::schemas::LinePropertiesEndArrow>,
        #[doc = "The connection at the end of the line. If unset, there is no connection.\n\nOnly lines with a Type indicating it is\na \"connector\" can have an `end_connection`."]
        #[serde(rename = "endConnection", default)]
        pub end_connection: Option<crate::schemas::LineConnection>,
        #[doc = "The fill of the line. The default line fill matches the defaults for new\nlines created in the Slides editor."]
        #[serde(rename = "lineFill", default)]
        pub line_fill: Option<crate::schemas::LineFill>,
        #[doc = "The hyperlink destination of the line. If unset, there is no link."]
        #[serde(rename = "link", default)]
        pub link: Option<crate::schemas::Link>,
        #[doc = "The style of the arrow at the beginning of the line."]
        #[serde(rename = "startArrow", default)]
        pub start_arrow: Option<crate::schemas::LinePropertiesStartArrow>,
        #[doc = "The connection at the beginning of the line. If unset, there is no\nconnection.\n\nOnly lines with a Type indicating it is\na \"connector\" can have a `start_connection`."]
        #[serde(rename = "startConnection", default)]
        pub start_connection: Option<crate::schemas::LineConnection>,
        #[doc = "The thickness of the line."]
        #[serde(rename = "weight", default)]
        pub weight: Option<crate::schemas::Dimension>,
    }
    impl ::field_selector::FieldSelector for LineProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LinkRelativeLink {
        #[doc = "An unspecified relative slide link."]
        RelativeSlideLinkUnspecified,
        #[doc = "A link to the next slide."]
        NextSlide,
        #[doc = "A link to the previous slide."]
        PreviousSlide,
        #[doc = "A link to the first slide in the presentation."]
        FirstSlide,
        #[doc = "A link to the last slide in the presentation."]
        LastSlide,
    }
    impl LinkRelativeLink {
        pub fn as_str(self) -> &'static str {
            match self {
                LinkRelativeLink::RelativeSlideLinkUnspecified => "RELATIVE_SLIDE_LINK_UNSPECIFIED",
                LinkRelativeLink::NextSlide => "NEXT_SLIDE",
                LinkRelativeLink::PreviousSlide => "PREVIOUS_SLIDE",
                LinkRelativeLink::FirstSlide => "FIRST_SLIDE",
                LinkRelativeLink::LastSlide => "LAST_SLIDE",
            }
        }
    }
    impl ::std::fmt::Display for LinkRelativeLink {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LinkRelativeLink {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LinkRelativeLink {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "RELATIVE_SLIDE_LINK_UNSPECIFIED" => LinkRelativeLink::RelativeSlideLinkUnspecified,
                "NEXT_SLIDE" => LinkRelativeLink::NextSlide,
                "PREVIOUS_SLIDE" => LinkRelativeLink::PreviousSlide,
                "FIRST_SLIDE" => LinkRelativeLink::FirstSlide,
                "LAST_SLIDE" => LinkRelativeLink::LastSlide,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Link {
        #[doc = "If set, indicates this is a link to the specific page in this\npresentation with this ID. A page with this ID may not exist."]
        #[serde(rename = "pageObjectId", default)]
        pub page_object_id: Option<String>,
        #[doc = "If set, indicates this is a link to a slide in this presentation,\naddressed by its position."]
        #[serde(rename = "relativeLink", default)]
        pub relative_link: Option<crate::schemas::LinkRelativeLink>,
        #[doc = "If set, indicates this is a link to the slide at this zero-based index\nin the presentation. There may not be a slide at this index."]
        #[serde(rename = "slideIndex", default)]
        pub slide_index: Option<i32>,
        #[doc = "If set, indicates this is a link to the external web page at this URL."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
    }
    impl ::field_selector::FieldSelector for Link {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct List {
        #[doc = "The ID of the list."]
        #[serde(rename = "listId", default)]
        pub list_id: Option<String>,
        #[doc = "A map of nesting levels to the properties of bullets at the associated\nlevel. A list has at most nine levels of nesting, so the possible values\nfor the keys of this map are 0 through 8, inclusive."]
        #[serde(rename = "nestingLevel", default)]
        pub nesting_level:
            Option<::std::collections::BTreeMap<String, crate::schemas::NestingLevel>>,
    }
    impl ::field_selector::FieldSelector for List {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MasterProperties {
        #[doc = "The human-readable name of the master."]
        #[serde(rename = "displayName", default)]
        pub display_name: Option<String>,
    }
    impl ::field_selector::FieldSelector for MasterProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MergeTableCellsRequest {
        #[doc = "The object ID of the table."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The table range specifying which cells of the table to merge.\n\nAny text in the cells being merged will be concatenated and stored in the\nupper-left (\"head\") cell of the range. If the range is non-rectangular\n(which can occur in some cases where the range covers cells that are\nalready merged), a 400 bad request error is returned."]
        #[serde(rename = "tableRange", default)]
        pub table_range: Option<crate::schemas::TableRange>,
    }
    impl ::field_selector::FieldSelector for MergeTableCellsRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NestingLevel {
        #[doc = "The style of a bullet at this level of nesting."]
        #[serde(rename = "bulletStyle", default)]
        pub bullet_style: Option<crate::schemas::TextStyle>,
    }
    impl ::field_selector::FieldSelector for NestingLevel {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NotesProperties {
        #[doc = "The object ID of the shape on this notes page that contains the speaker\nnotes for the corresponding slide.\nThe actual shape may not always exist on the notes page. Inserting text\nusing this object ID will automatically create the shape. In this case, the\nactual shape may have different object ID. The `GetPresentation` or\n`GetPage` action will always return the latest object ID."]
        #[serde(rename = "speakerNotesObjectId", default)]
        pub speaker_notes_object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for NotesProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OpaqueColorThemeColor {
        #[doc = "Unspecified theme color. This value should not be used."]
        ThemeColorTypeUnspecified,
        #[doc = "Represents the first dark color."]
        Dark1,
        #[doc = "Represents the first light color."]
        Light1,
        #[doc = "Represents the second dark color."]
        Dark2,
        #[doc = "Represents the second light color."]
        Light2,
        #[doc = "Represents the first accent color."]
        Accent1,
        #[doc = "Represents the second accent color."]
        Accent2,
        #[doc = "Represents the third accent color."]
        Accent3,
        #[doc = "Represents the fourth accent color."]
        Accent4,
        #[doc = "Represents the fifth accent color."]
        Accent5,
        #[doc = "Represents the sixth accent color."]
        Accent6,
        #[doc = "Represents the color to use for hyperlinks."]
        Hyperlink,
        #[doc = "Represents the color to use for visited hyperlinks."]
        FollowedHyperlink,
        #[doc = "Represents the first text color."]
        Text1,
        #[doc = "Represents the first background color."]
        Background1,
        #[doc = "Represents the second text color."]
        Text2,
        #[doc = "Represents the second background color."]
        Background2,
    }
    impl OpaqueColorThemeColor {
        pub fn as_str(self) -> &'static str {
            match self {
                OpaqueColorThemeColor::ThemeColorTypeUnspecified => "THEME_COLOR_TYPE_UNSPECIFIED",
                OpaqueColorThemeColor::Dark1 => "DARK1",
                OpaqueColorThemeColor::Light1 => "LIGHT1",
                OpaqueColorThemeColor::Dark2 => "DARK2",
                OpaqueColorThemeColor::Light2 => "LIGHT2",
                OpaqueColorThemeColor::Accent1 => "ACCENT1",
                OpaqueColorThemeColor::Accent2 => "ACCENT2",
                OpaqueColorThemeColor::Accent3 => "ACCENT3",
                OpaqueColorThemeColor::Accent4 => "ACCENT4",
                OpaqueColorThemeColor::Accent5 => "ACCENT5",
                OpaqueColorThemeColor::Accent6 => "ACCENT6",
                OpaqueColorThemeColor::Hyperlink => "HYPERLINK",
                OpaqueColorThemeColor::FollowedHyperlink => "FOLLOWED_HYPERLINK",
                OpaqueColorThemeColor::Text1 => "TEXT1",
                OpaqueColorThemeColor::Background1 => "BACKGROUND1",
                OpaqueColorThemeColor::Text2 => "TEXT2",
                OpaqueColorThemeColor::Background2 => "BACKGROUND2",
            }
        }
    }
    impl ::std::fmt::Display for OpaqueColorThemeColor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OpaqueColorThemeColor {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OpaqueColorThemeColor {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "THEME_COLOR_TYPE_UNSPECIFIED" => OpaqueColorThemeColor::ThemeColorTypeUnspecified,
                "DARK1" => OpaqueColorThemeColor::Dark1,
                "LIGHT1" => OpaqueColorThemeColor::Light1,
                "DARK2" => OpaqueColorThemeColor::Dark2,
                "LIGHT2" => OpaqueColorThemeColor::Light2,
                "ACCENT1" => OpaqueColorThemeColor::Accent1,
                "ACCENT2" => OpaqueColorThemeColor::Accent2,
                "ACCENT3" => OpaqueColorThemeColor::Accent3,
                "ACCENT4" => OpaqueColorThemeColor::Accent4,
                "ACCENT5" => OpaqueColorThemeColor::Accent5,
                "ACCENT6" => OpaqueColorThemeColor::Accent6,
                "HYPERLINK" => OpaqueColorThemeColor::Hyperlink,
                "FOLLOWED_HYPERLINK" => OpaqueColorThemeColor::FollowedHyperlink,
                "TEXT1" => OpaqueColorThemeColor::Text1,
                "BACKGROUND1" => OpaqueColorThemeColor::Background1,
                "TEXT2" => OpaqueColorThemeColor::Text2,
                "BACKGROUND2" => OpaqueColorThemeColor::Background2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct OpaqueColor {
        #[doc = "An opaque RGB color."]
        #[serde(rename = "rgbColor", default)]
        pub rgb_color: Option<crate::schemas::RgbColor>,
        #[doc = "An opaque theme color."]
        #[serde(rename = "themeColor", default)]
        pub theme_color: Option<crate::schemas::OpaqueColorThemeColor>,
    }
    impl ::field_selector::FieldSelector for OpaqueColor {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct OptionalColor {
        #[doc = "If set, this will be used as an opaque color. If unset, this represents\na transparent color."]
        #[serde(rename = "opaqueColor", default)]
        pub opaque_color: Option<crate::schemas::OpaqueColor>,
    }
    impl ::field_selector::FieldSelector for OptionalColor {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OutlineDashStyle {
        #[doc = "Unspecified dash style."]
        DashStyleUnspecified,
        #[doc = "Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'.\nThis is the default dash style."]
        Solid,
        #[doc = "Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'."]
        Dot,
        #[doc = "Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'."]
        Dash,
        #[doc = "Alternating dashes and dots. Corresponds to ECMA-376 ST_PresetLineDashVal\nvalue 'dashDot'."]
        DashDot,
        #[doc = "Line with large dashes. Corresponds to ECMA-376 ST_PresetLineDashVal\nvalue 'lgDash'."]
        LongDash,
        #[doc = "Alternating large dashes and dots. Corresponds to ECMA-376\nST_PresetLineDashVal value 'lgDashDot'."]
        LongDashDot,
    }
    impl OutlineDashStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                OutlineDashStyle::DashStyleUnspecified => "DASH_STYLE_UNSPECIFIED",
                OutlineDashStyle::Solid => "SOLID",
                OutlineDashStyle::Dot => "DOT",
                OutlineDashStyle::Dash => "DASH",
                OutlineDashStyle::DashDot => "DASH_DOT",
                OutlineDashStyle::LongDash => "LONG_DASH",
                OutlineDashStyle::LongDashDot => "LONG_DASH_DOT",
            }
        }
    }
    impl ::std::fmt::Display for OutlineDashStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OutlineDashStyle {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OutlineDashStyle {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DASH_STYLE_UNSPECIFIED" => OutlineDashStyle::DashStyleUnspecified,
                "SOLID" => OutlineDashStyle::Solid,
                "DOT" => OutlineDashStyle::Dot,
                "DASH" => OutlineDashStyle::Dash,
                "DASH_DOT" => OutlineDashStyle::DashDot,
                "LONG_DASH" => OutlineDashStyle::LongDash,
                "LONG_DASH_DOT" => OutlineDashStyle::LongDashDot,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OutlinePropertyState {
        #[doc = "If a property's state is RENDERED, then the element has the corresponding\nproperty when rendered on a page. If the element is a placeholder shape as\ndetermined by the placeholder\nfield, and it inherits from a placeholder shape, the corresponding field\nmay be unset, meaning that the property value is inherited from a parent\nplaceholder. If the element does not inherit, then the field will contain\nthe rendered value. This is the default value."]
        Rendered,
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the\ncorresponding property when rendered on a page. However, the field may\nstill be set so it can be inherited by child shapes. To remove a property\nfrom a rendered element, set its property_state to NOT_RENDERED."]
        NotRendered,
        #[doc = "If a property's state is INHERIT, then the property state uses the value of\ncorresponding `property_state` field on the parent shape. Elements that do\nnot inherit will never have an INHERIT property state."]
        Inherit,
    }
    impl OutlinePropertyState {
        pub fn as_str(self) -> &'static str {
            match self {
                OutlinePropertyState::Rendered => "RENDERED",
                OutlinePropertyState::NotRendered => "NOT_RENDERED",
                OutlinePropertyState::Inherit => "INHERIT",
            }
        }
    }
    impl ::std::fmt::Display for OutlinePropertyState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OutlinePropertyState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OutlinePropertyState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "RENDERED" => OutlinePropertyState::Rendered,
                "NOT_RENDERED" => OutlinePropertyState::NotRendered,
                "INHERIT" => OutlinePropertyState::Inherit,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Outline {
        #[doc = "The dash style of the outline."]
        #[serde(rename = "dashStyle", default)]
        pub dash_style: Option<crate::schemas::OutlineDashStyle>,
        #[doc = "The fill of the outline."]
        #[serde(rename = "outlineFill", default)]
        pub outline_fill: Option<crate::schemas::OutlineFill>,
        #[doc = "The outline property state.\n\nUpdating the outline on a page element will implicitly update this field\nto `RENDERED`, unless another value is specified in the same request. To\nhave no outline on a page element, set this field to `NOT_RENDERED`. In\nthis case, any other outline fields set in the same request will be\nignored."]
        #[serde(rename = "propertyState", default)]
        pub property_state: Option<crate::schemas::OutlinePropertyState>,
        #[doc = "The thickness of the outline."]
        #[serde(rename = "weight", default)]
        pub weight: Option<crate::schemas::Dimension>,
    }
    impl ::field_selector::FieldSelector for Outline {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct OutlineFill {
        #[doc = "Solid color fill."]
        #[serde(rename = "solidFill", default)]
        pub solid_fill: Option<crate::schemas::SolidFill>,
    }
    impl ::field_selector::FieldSelector for OutlineFill {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PagePageType {
        #[doc = "A slide page."]
        Slide,
        #[doc = "A master slide page."]
        Master,
        #[doc = "A layout page."]
        Layout,
        #[doc = "A notes page."]
        Notes,
        #[doc = "A notes master page."]
        NotesMaster,
    }
    impl PagePageType {
        pub fn as_str(self) -> &'static str {
            match self {
                PagePageType::Slide => "SLIDE",
                PagePageType::Master => "MASTER",
                PagePageType::Layout => "LAYOUT",
                PagePageType::Notes => "NOTES",
                PagePageType::NotesMaster => "NOTES_MASTER",
            }
        }
    }
    impl ::std::fmt::Display for PagePageType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PagePageType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PagePageType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SLIDE" => PagePageType::Slide,
                "MASTER" => PagePageType::Master,
                "LAYOUT" => PagePageType::Layout,
                "NOTES" => PagePageType::Notes,
                "NOTES_MASTER" => PagePageType::NotesMaster,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Page {
        #[doc = "Layout specific properties. Only set if page_type = LAYOUT."]
        #[serde(rename = "layoutProperties", default)]
        pub layout_properties: Option<crate::schemas::LayoutProperties>,
        #[doc = "Master specific properties. Only set if page_type = MASTER."]
        #[serde(rename = "masterProperties", default)]
        pub master_properties: Option<crate::schemas::MasterProperties>,
        #[doc = "Notes specific properties. Only set if page_type = NOTES."]
        #[serde(rename = "notesProperties", default)]
        pub notes_properties: Option<crate::schemas::NotesProperties>,
        #[doc = "The object ID for this page. Object IDs used by\nPage and\nPageElement share the same namespace."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The page elements rendered on the page."]
        #[serde(rename = "pageElements", default)]
        pub page_elements: Option<Vec<crate::schemas::PageElement>>,
        #[doc = "The properties of the page."]
        #[serde(rename = "pageProperties", default)]
        pub page_properties: Option<crate::schemas::PageProperties>,
        #[doc = "The type of the page."]
        #[serde(rename = "pageType", default)]
        pub page_type: Option<crate::schemas::PagePageType>,
        #[doc = "The revision ID of the presentation containing this page. Can be used in\nupdate requests to assert that the presentation revision hasn't changed\nsince the last read operation. Only populated if the user has edit access\nto the presentation.\n\nThe format of the revision ID may change over time, so it should be treated\nopaquely. A returned revision ID is only guaranteed to be valid for 24\nhours after it has been returned and cannot be shared across users. If the\nrevision ID is unchanged between calls, then the presentation has not\nchanged. Conversely, a changed ID (for the same presentation and user)\nusually means the presentation has been updated; however, a changed ID can\nalso be due to internal factors such as ID format changes."]
        #[serde(rename = "revisionId", default)]
        pub revision_id: Option<String>,
        #[doc = "Slide specific properties. Only set if page_type = SLIDE."]
        #[serde(rename = "slideProperties", default)]
        pub slide_properties: Option<crate::schemas::SlideProperties>,
    }
    impl ::field_selector::FieldSelector for Page {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PageBackgroundFillPropertyState {
        #[doc = "If a property's state is RENDERED, then the element has the corresponding\nproperty when rendered on a page. If the element is a placeholder shape as\ndetermined by the placeholder\nfield, and it inherits from a placeholder shape, the corresponding field\nmay be unset, meaning that the property value is inherited from a parent\nplaceholder. If the element does not inherit, then the field will contain\nthe rendered value. This is the default value."]
        Rendered,
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the\ncorresponding property when rendered on a page. However, the field may\nstill be set so it can be inherited by child shapes. To remove a property\nfrom a rendered element, set its property_state to NOT_RENDERED."]
        NotRendered,
        #[doc = "If a property's state is INHERIT, then the property state uses the value of\ncorresponding `property_state` field on the parent shape. Elements that do\nnot inherit will never have an INHERIT property state."]
        Inherit,
    }
    impl PageBackgroundFillPropertyState {
        pub fn as_str(self) -> &'static str {
            match self {
                PageBackgroundFillPropertyState::Rendered => "RENDERED",
                PageBackgroundFillPropertyState::NotRendered => "NOT_RENDERED",
                PageBackgroundFillPropertyState::Inherit => "INHERIT",
            }
        }
    }
    impl ::std::fmt::Display for PageBackgroundFillPropertyState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PageBackgroundFillPropertyState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PageBackgroundFillPropertyState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "RENDERED" => PageBackgroundFillPropertyState::Rendered,
                "NOT_RENDERED" => PageBackgroundFillPropertyState::NotRendered,
                "INHERIT" => PageBackgroundFillPropertyState::Inherit,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PageBackgroundFill {
        #[doc = "The background fill property state.\n\nUpdating the fill on a page will implicitly update this field to\n`RENDERED`, unless another value is specified in the same request. To\nhave no fill on a page, set this field to `NOT_RENDERED`. In this case,\nany other fill fields set in the same request will be ignored."]
        #[serde(rename = "propertyState", default)]
        pub property_state: Option<crate::schemas::PageBackgroundFillPropertyState>,
        #[doc = "Solid color fill."]
        #[serde(rename = "solidFill", default)]
        pub solid_fill: Option<crate::schemas::SolidFill>,
        #[doc = "Stretched picture fill."]
        #[serde(rename = "stretchedPictureFill", default)]
        pub stretched_picture_fill: Option<crate::schemas::StretchedPictureFill>,
    }
    impl ::field_selector::FieldSelector for PageBackgroundFill {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PageElement {
        #[doc = "The description of the page element. Combined with title to display alt\ntext."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "A collection of page elements joined as a single unit."]
        #[serde(rename = "elementGroup", default)]
        pub element_group: Option<crate::schemas::Group>,
        #[doc = "An image page element."]
        #[serde(rename = "image", default)]
        pub image: Option<crate::schemas::Image>,
        #[doc = "A line page element."]
        #[serde(rename = "line", default)]
        pub line: Option<crate::schemas::Line>,
        #[doc = "The object ID for this page element. Object IDs used by\ngoogle.apps.slides.v1.Page and\ngoogle.apps.slides.v1.PageElement share the same namespace."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "A generic shape."]
        #[serde(rename = "shape", default)]
        pub shape: Option<crate::schemas::Shape>,
        #[doc = "A linked chart embedded from Google Sheets. Unlinked charts are\nrepresented as images."]
        #[serde(rename = "sheetsChart", default)]
        pub sheets_chart: Option<crate::schemas::SheetsChart>,
        #[doc = "The size of the page element."]
        #[serde(rename = "size", default)]
        pub size: Option<crate::schemas::Size>,
        #[doc = "A table page element."]
        #[serde(rename = "table", default)]
        pub table: Option<crate::schemas::Table>,
        #[doc = "The title of the page element. Combined with description to display alt\ntext."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
        #[doc = "The transform of the page element.\n\nThe visual appearance of the page element is determined by its absolute\ntransform. To compute the absolute transform, preconcatenate a page\nelement's transform with the transforms of all of its parent groups. If the\npage element is not in a group, its absolute transform is the same as the\nvalue in this field.\n\nThe initial transform for the newly created Group is always the identity transform."]
        #[serde(rename = "transform", default)]
        pub transform: Option<crate::schemas::AffineTransform>,
        #[doc = "A video page element."]
        #[serde(rename = "video", default)]
        pub video: Option<crate::schemas::Video>,
        #[doc = "A word art page element."]
        #[serde(rename = "wordArt", default)]
        pub word_art: Option<crate::schemas::WordArt>,
    }
    impl ::field_selector::FieldSelector for PageElement {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PageElementProperties {
        #[doc = "The object ID of the page where the element is located."]
        #[serde(rename = "pageObjectId", default)]
        pub page_object_id: Option<String>,
        #[doc = "The size of the element."]
        #[serde(rename = "size", default)]
        pub size: Option<crate::schemas::Size>,
        #[doc = "The transform for the element."]
        #[serde(rename = "transform", default)]
        pub transform: Option<crate::schemas::AffineTransform>,
    }
    impl ::field_selector::FieldSelector for PageElementProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PageProperties {
        #[doc = "The color scheme of the page. If unset, the color scheme is inherited from\na parent page. If the page has no parent, the color scheme uses a default\nSlides color scheme, matching the defaults in the Slides editor.\n\nOnly the concrete colors of the first 12 ThemeColorTypes are editable. In addition, only\nthe color scheme on `Master` pages can be updated. To update the field, a\ncolor scheme containing mappings from all the first 12 ThemeColorTypes to\ntheir concrete colors must be provided. Colors for the remaining\nThemeColorTypes will be ignored."]
        #[serde(rename = "colorScheme", default)]
        pub color_scheme: Option<crate::schemas::ColorScheme>,
        #[doc = "The background fill of the page. If unset, the background fill is inherited\nfrom a parent page if it exists. If the page has no parent, then the\nbackground fill defaults to the corresponding fill in the Slides editor."]
        #[serde(rename = "pageBackgroundFill", default)]
        pub page_background_fill: Option<crate::schemas::PageBackgroundFill>,
    }
    impl ::field_selector::FieldSelector for PageProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ParagraphMarker {
        #[doc = "The bullet for this paragraph. If not present, the paragraph does not\nbelong to a list."]
        #[serde(rename = "bullet", default)]
        pub bullet: Option<crate::schemas::Bullet>,
        #[doc = "The paragraph's style"]
        #[serde(rename = "style", default)]
        pub style: Option<crate::schemas::ParagraphStyle>,
    }
    impl ::field_selector::FieldSelector for ParagraphMarker {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ParagraphStyleAlignment {
        #[doc = "The paragraph alignment is inherited from the parent."]
        AlignmentUnspecified,
        #[doc = "The paragraph is aligned to the start of the line. Left-aligned for\nLTR text, right-aligned otherwise."]
        Start,
        #[doc = "The paragraph is centered."]
        Center,
        #[doc = "The paragraph is aligned to the end of the line. Right-aligned for\nLTR text, left-aligned otherwise."]
        End,
        #[doc = "The paragraph is justified."]
        Justified,
    }
    impl ParagraphStyleAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                ParagraphStyleAlignment::AlignmentUnspecified => "ALIGNMENT_UNSPECIFIED",
                ParagraphStyleAlignment::Start => "START",
                ParagraphStyleAlignment::Center => "CENTER",
                ParagraphStyleAlignment::End => "END",
                ParagraphStyleAlignment::Justified => "JUSTIFIED",
            }
        }
    }
    impl ::std::fmt::Display for ParagraphStyleAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ParagraphStyleAlignment {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ParagraphStyleAlignment {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALIGNMENT_UNSPECIFIED" => ParagraphStyleAlignment::AlignmentUnspecified,
                "START" => ParagraphStyleAlignment::Start,
                "CENTER" => ParagraphStyleAlignment::Center,
                "END" => ParagraphStyleAlignment::End,
                "JUSTIFIED" => ParagraphStyleAlignment::Justified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ParagraphStyleDirection {
        #[doc = "The text direction is inherited from the parent."]
        TextDirectionUnspecified,
        #[doc = "The text goes from left to right."]
        LeftToRight,
        #[doc = "The text goes from right to left."]
        RightToLeft,
    }
    impl ParagraphStyleDirection {
        pub fn as_str(self) -> &'static str {
            match self {
                ParagraphStyleDirection::TextDirectionUnspecified => "TEXT_DIRECTION_UNSPECIFIED",
                ParagraphStyleDirection::LeftToRight => "LEFT_TO_RIGHT",
                ParagraphStyleDirection::RightToLeft => "RIGHT_TO_LEFT",
            }
        }
    }
    impl ::std::fmt::Display for ParagraphStyleDirection {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ParagraphStyleDirection {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ParagraphStyleDirection {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TEXT_DIRECTION_UNSPECIFIED" => ParagraphStyleDirection::TextDirectionUnspecified,
                "LEFT_TO_RIGHT" => ParagraphStyleDirection::LeftToRight,
                "RIGHT_TO_LEFT" => ParagraphStyleDirection::RightToLeft,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ParagraphStyleSpacingMode {
        #[doc = "The spacing mode is inherited from the parent."]
        SpacingModeUnspecified,
        #[doc = "Paragraph spacing is always rendered."]
        NeverCollapse,
        #[doc = "Paragraph spacing is skipped between list elements."]
        CollapseLists,
    }
    impl ParagraphStyleSpacingMode {
        pub fn as_str(self) -> &'static str {
            match self {
                ParagraphStyleSpacingMode::SpacingModeUnspecified => "SPACING_MODE_UNSPECIFIED",
                ParagraphStyleSpacingMode::NeverCollapse => "NEVER_COLLAPSE",
                ParagraphStyleSpacingMode::CollapseLists => "COLLAPSE_LISTS",
            }
        }
    }
    impl ::std::fmt::Display for ParagraphStyleSpacingMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ParagraphStyleSpacingMode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ParagraphStyleSpacingMode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SPACING_MODE_UNSPECIFIED" => ParagraphStyleSpacingMode::SpacingModeUnspecified,
                "NEVER_COLLAPSE" => ParagraphStyleSpacingMode::NeverCollapse,
                "COLLAPSE_LISTS" => ParagraphStyleSpacingMode::CollapseLists,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ParagraphStyle {
        #[doc = "The text alignment for this paragraph."]
        #[serde(rename = "alignment", default)]
        pub alignment: Option<crate::schemas::ParagraphStyleAlignment>,
        #[doc = "The text direction of this paragraph. If unset, the value defaults to\nLEFT_TO_RIGHT since\ntext direction is not inherited."]
        #[serde(rename = "direction", default)]
        pub direction: Option<crate::schemas::ParagraphStyleDirection>,
        #[doc = "The amount indentation for the paragraph on the side that corresponds to\nthe end of the text, based on the current text direction. If unset, the\nvalue is inherited from the parent."]
        #[serde(rename = "indentEnd", default)]
        pub indent_end: Option<crate::schemas::Dimension>,
        #[doc = "The amount of indentation for the start of the first line of the paragraph.\nIf unset, the value is inherited from the parent."]
        #[serde(rename = "indentFirstLine", default)]
        pub indent_first_line: Option<crate::schemas::Dimension>,
        #[doc = "The amount indentation for the paragraph on the side that corresponds to\nthe start of the text, based on the current text direction. If unset, the\nvalue is inherited from the parent."]
        #[serde(rename = "indentStart", default)]
        pub indent_start: Option<crate::schemas::Dimension>,
        #[doc = "The amount of space between lines, as a percentage of normal, where normal\nis represented as 100.0. If unset, the value is inherited from the parent."]
        #[serde(rename = "lineSpacing", default)]
        pub line_spacing: Option<f32>,
        #[doc = "The amount of extra space above the paragraph. If unset, the value is\ninherited from the parent."]
        #[serde(rename = "spaceAbove", default)]
        pub space_above: Option<crate::schemas::Dimension>,
        #[doc = "The amount of extra space below the paragraph. If unset, the value is\ninherited from the parent."]
        #[serde(rename = "spaceBelow", default)]
        pub space_below: Option<crate::schemas::Dimension>,
        #[doc = "The spacing mode for the paragraph."]
        #[serde(rename = "spacingMode", default)]
        pub spacing_mode: Option<crate::schemas::ParagraphStyleSpacingMode>,
    }
    impl ::field_selector::FieldSelector for ParagraphStyle {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PlaceholderType {
        #[doc = "Default value, signifies it is not a placeholder."]
        None,
        #[doc = "Body text."]
        Body,
        #[doc = "Chart or graph."]
        Chart,
        #[doc = "Clip art image."]
        ClipArt,
        #[doc = "Title centered."]
        CenteredTitle,
        #[doc = "Diagram."]
        Diagram,
        #[doc = "Date and time."]
        DateAndTime,
        #[doc = "Footer text."]
        Footer,
        #[doc = "Header text."]
        Header,
        #[doc = "Multimedia."]
        Media,
        #[doc = "Any content type."]
        Object,
        #[doc = "Picture."]
        Picture,
        #[doc = "Number of a slide."]
        SlideNumber,
        #[doc = "Subtitle."]
        Subtitle,
        #[doc = "Table."]
        Table,
        #[doc = "Slide title."]
        Title,
        #[doc = "Slide image."]
        SlideImage,
    }
    impl PlaceholderType {
        pub fn as_str(self) -> &'static str {
            match self {
                PlaceholderType::None => "NONE",
                PlaceholderType::Body => "BODY",
                PlaceholderType::Chart => "CHART",
                PlaceholderType::ClipArt => "CLIP_ART",
                PlaceholderType::CenteredTitle => "CENTERED_TITLE",
                PlaceholderType::Diagram => "DIAGRAM",
                PlaceholderType::DateAndTime => "DATE_AND_TIME",
                PlaceholderType::Footer => "FOOTER",
                PlaceholderType::Header => "HEADER",
                PlaceholderType::Media => "MEDIA",
                PlaceholderType::Object => "OBJECT",
                PlaceholderType::Picture => "PICTURE",
                PlaceholderType::SlideNumber => "SLIDE_NUMBER",
                PlaceholderType::Subtitle => "SUBTITLE",
                PlaceholderType::Table => "TABLE",
                PlaceholderType::Title => "TITLE",
                PlaceholderType::SlideImage => "SLIDE_IMAGE",
            }
        }
    }
    impl ::std::fmt::Display for PlaceholderType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PlaceholderType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PlaceholderType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => PlaceholderType::None,
                "BODY" => PlaceholderType::Body,
                "CHART" => PlaceholderType::Chart,
                "CLIP_ART" => PlaceholderType::ClipArt,
                "CENTERED_TITLE" => PlaceholderType::CenteredTitle,
                "DIAGRAM" => PlaceholderType::Diagram,
                "DATE_AND_TIME" => PlaceholderType::DateAndTime,
                "FOOTER" => PlaceholderType::Footer,
                "HEADER" => PlaceholderType::Header,
                "MEDIA" => PlaceholderType::Media,
                "OBJECT" => PlaceholderType::Object,
                "PICTURE" => PlaceholderType::Picture,
                "SLIDE_NUMBER" => PlaceholderType::SlideNumber,
                "SUBTITLE" => PlaceholderType::Subtitle,
                "TABLE" => PlaceholderType::Table,
                "TITLE" => PlaceholderType::Title,
                "SLIDE_IMAGE" => PlaceholderType::SlideImage,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Placeholder {
        #[doc = "The index of the placeholder. If the same placeholder types are present in\nthe same page, they would have different index values."]
        #[serde(rename = "index", default)]
        pub index: Option<i32>,
        #[doc = "The object ID of this shape's parent placeholder.\nIf unset, the parent placeholder shape does not exist, so the shape does\nnot inherit properties from any other shape."]
        #[serde(rename = "parentObjectId", default)]
        pub parent_object_id: Option<String>,
        #[doc = "The type of the placeholder."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::PlaceholderType>,
    }
    impl ::field_selector::FieldSelector for Placeholder {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Presentation {
        #[doc = "The layouts in the presentation. A layout is a template that determines\nhow content is arranged and styled on the slides that inherit from that\nlayout."]
        #[serde(rename = "layouts", default)]
        pub layouts: Option<Vec<crate::schemas::Page>>,
        #[doc = "The locale of the presentation, as an IETF BCP 47 language tag."]
        #[serde(rename = "locale", default)]
        pub locale: Option<String>,
        #[doc = "The slide masters in the presentation. A slide master contains all common\npage elements and the common properties for a set of layouts. They serve\nthree purposes:\n\n- Placeholder shapes on a master contain the default text styles and shape\n  properties of all placeholder shapes on pages that use that master.\n- The master page properties define the common page properties inherited by\n  its layouts.\n- Any other shapes on the master slide will appear on all slides using that\n  master, regardless of their layout."]
        #[serde(rename = "masters", default)]
        pub masters: Option<Vec<crate::schemas::Page>>,
        #[doc = "The notes master in the presentation. It serves three purposes:\n\n- Placeholder shapes on a notes master contain the default text styles and\n  shape properties of all placeholder shapes on notes pages. Specifically,\n  a `SLIDE_IMAGE` placeholder shape contains the slide thumbnail, and a\n  `BODY` placeholder shape contains the speaker notes.\n- The notes master page properties define the common page properties\n  inherited by all notes pages.\n- Any other shapes on the notes master will appear on all notes pages.\n\nThe notes master is read-only."]
        #[serde(rename = "notesMaster", default)]
        pub notes_master: Option<crate::schemas::Page>,
        #[doc = "The size of pages in the presentation."]
        #[serde(rename = "pageSize", default)]
        pub page_size: Option<crate::schemas::Size>,
        #[doc = "The ID of the presentation."]
        #[serde(rename = "presentationId", default)]
        pub presentation_id: Option<String>,
        #[doc = "The revision ID of the presentation. Can be used in update requests\nto assert that the presentation revision hasn't changed since the last\nread operation. Only populated if the user has edit access to the\npresentation.\n\nThe format of the revision ID may change over time, so it should be treated\nopaquely. A returned revision ID is only guaranteed to be valid for 24\nhours after it has been returned and cannot be shared across users. If the\nrevision ID is unchanged between calls, then the presentation has not\nchanged. Conversely, a changed ID (for the same presentation and user)\nusually means the presentation has been updated; however, a changed ID can\nalso be due to internal factors such as ID format changes."]
        #[serde(rename = "revisionId", default)]
        pub revision_id: Option<String>,
        #[doc = "The slides in the presentation.\nA slide inherits properties from a slide layout."]
        #[serde(rename = "slides", default)]
        pub slides: Option<Vec<crate::schemas::Page>>,
        #[doc = "The title of the presentation."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for Presentation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RangeType {
        #[doc = "Unspecified range type. This value must not be used."]
        RangeTypeUnspecified,
        #[doc = "A fixed range. Both the `start_index` and\n`end_index` must be specified."]
        FixedRange,
        #[doc = "Starts the range at `start_index` and continues until the\nend of the collection. The `end_index` must not be specified."]
        FromStartIndex,
        #[doc = "Sets the range to be the whole length of the collection. Both the\n`start_index` and the `end_index` must not be\nspecified."]
        All,
    }
    impl RangeType {
        pub fn as_str(self) -> &'static str {
            match self {
                RangeType::RangeTypeUnspecified => "RANGE_TYPE_UNSPECIFIED",
                RangeType::FixedRange => "FIXED_RANGE",
                RangeType::FromStartIndex => "FROM_START_INDEX",
                RangeType::All => "ALL",
            }
        }
    }
    impl ::std::fmt::Display for RangeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RangeType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RangeType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "RANGE_TYPE_UNSPECIFIED" => RangeType::RangeTypeUnspecified,
                "FIXED_RANGE" => RangeType::FixedRange,
                "FROM_START_INDEX" => RangeType::FromStartIndex,
                "ALL" => RangeType::All,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Range {
        #[doc = "The optional zero-based index of the end of the collection.\nRequired for `FIXED_RANGE` ranges."]
        #[serde(rename = "endIndex", default)]
        pub end_index: Option<i32>,
        #[doc = "The type of range."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::RangeType>,
        #[doc = "The optional zero-based index of the beginning of the collection.\nRequired for `FIXED_RANGE` and `FROM_START_INDEX` ranges."]
        #[serde(rename = "startIndex", default)]
        pub start_index: Option<i32>,
    }
    impl ::field_selector::FieldSelector for Range {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RecolorName {
        #[doc = "No recolor effect. The default value."]
        None,
        #[doc = "A recolor effect that lightens the image using the page's first available\ncolor from its color scheme."]
        Light1,
        #[doc = "A recolor effect that lightens the image using the page's second\navailable color from its color scheme."]
        Light2,
        #[doc = "A recolor effect that lightens the image using the page's third available\ncolor from its color scheme."]
        Light3,
        #[doc = "A recolor effect that lightens the image using the page's forth available\ncolor from its color scheme."]
        Light4,
        #[doc = "A recolor effect that lightens the image using the page's fifth available\ncolor from its color scheme."]
        Light5,
        #[doc = "A recolor effect that lightens the image using the page's sixth available\ncolor from its color scheme."]
        Light6,
        #[doc = "A recolor effect that lightens the image using the page's seventh\navailable color from its color scheme."]
        Light7,
        #[doc = "A recolor effect that lightens the image using the page's eighth\navailable color from its color scheme."]
        Light8,
        #[doc = "A recolor effect that lightens the image using the page's ninth available\ncolor from its color scheme."]
        Light9,
        #[doc = "A recolor effect that lightens the image using the page's tenth available\ncolor from its color scheme."]
        Light10,
        #[doc = "A recolor effect that darkens the image using the page's first available\ncolor from its color scheme."]
        Dark1,
        #[doc = "A recolor effect that darkens the image using the page's second available\ncolor from its color scheme."]
        Dark2,
        #[doc = "A recolor effect that darkens the image using the page's third available\ncolor from its color scheme."]
        Dark3,
        #[doc = "A recolor effect that darkens the image using the page's fourth available\ncolor from its color scheme."]
        Dark4,
        #[doc = "A recolor effect that darkens the image using the page's fifth available\ncolor from its color scheme."]
        Dark5,
        #[doc = "A recolor effect that darkens the image using the page's sixth available\ncolor from its color scheme."]
        Dark6,
        #[doc = "A recolor effect that darkens the image using the page's seventh\navailable color from its color scheme."]
        Dark7,
        #[doc = "A recolor effect that darkens the image using the page's eighth available\ncolor from its color scheme."]
        Dark8,
        #[doc = "A recolor effect that darkens the image using the page's ninth available\ncolor from its color scheme."]
        Dark9,
        #[doc = "A recolor effect that darkens the image using the page's tenth available\ncolor from its color scheme."]
        Dark10,
        #[doc = "A recolor effect that recolors the image to grayscale."]
        Grayscale,
        #[doc = "A recolor effect that recolors the image to negative grayscale."]
        Negative,
        #[doc = "A recolor effect that recolors the image using the sepia color."]
        Sepia,
        #[doc = "Custom recolor effect. Refer to `recolor_stops` for the concrete\ngradient."]
        Custom,
    }
    impl RecolorName {
        pub fn as_str(self) -> &'static str {
            match self {
                RecolorName::None => "NONE",
                RecolorName::Light1 => "LIGHT1",
                RecolorName::Light2 => "LIGHT2",
                RecolorName::Light3 => "LIGHT3",
                RecolorName::Light4 => "LIGHT4",
                RecolorName::Light5 => "LIGHT5",
                RecolorName::Light6 => "LIGHT6",
                RecolorName::Light7 => "LIGHT7",
                RecolorName::Light8 => "LIGHT8",
                RecolorName::Light9 => "LIGHT9",
                RecolorName::Light10 => "LIGHT10",
                RecolorName::Dark1 => "DARK1",
                RecolorName::Dark2 => "DARK2",
                RecolorName::Dark3 => "DARK3",
                RecolorName::Dark4 => "DARK4",
                RecolorName::Dark5 => "DARK5",
                RecolorName::Dark6 => "DARK6",
                RecolorName::Dark7 => "DARK7",
                RecolorName::Dark8 => "DARK8",
                RecolorName::Dark9 => "DARK9",
                RecolorName::Dark10 => "DARK10",
                RecolorName::Grayscale => "GRAYSCALE",
                RecolorName::Negative => "NEGATIVE",
                RecolorName::Sepia => "SEPIA",
                RecolorName::Custom => "CUSTOM",
            }
        }
    }
    impl ::std::fmt::Display for RecolorName {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RecolorName {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RecolorName {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NONE" => RecolorName::None,
                "LIGHT1" => RecolorName::Light1,
                "LIGHT2" => RecolorName::Light2,
                "LIGHT3" => RecolorName::Light3,
                "LIGHT4" => RecolorName::Light4,
                "LIGHT5" => RecolorName::Light5,
                "LIGHT6" => RecolorName::Light6,
                "LIGHT7" => RecolorName::Light7,
                "LIGHT8" => RecolorName::Light8,
                "LIGHT9" => RecolorName::Light9,
                "LIGHT10" => RecolorName::Light10,
                "DARK1" => RecolorName::Dark1,
                "DARK2" => RecolorName::Dark2,
                "DARK3" => RecolorName::Dark3,
                "DARK4" => RecolorName::Dark4,
                "DARK5" => RecolorName::Dark5,
                "DARK6" => RecolorName::Dark6,
                "DARK7" => RecolorName::Dark7,
                "DARK8" => RecolorName::Dark8,
                "DARK9" => RecolorName::Dark9,
                "DARK10" => RecolorName::Dark10,
                "GRAYSCALE" => RecolorName::Grayscale,
                "NEGATIVE" => RecolorName::Negative,
                "SEPIA" => RecolorName::Sepia,
                "CUSTOM" => RecolorName::Custom,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Recolor {
        #[doc = "The name of the recolor effect.\n\nThe name is determined from the `recolor_stops` by matching the gradient\nagainst the colors in the page's current color scheme. This property is\nread-only."]
        #[serde(rename = "name", default)]
        pub name: Option<crate::schemas::RecolorName>,
        #[doc = "The recolor effect is represented by a gradient, which is a list of color\nstops.\n\nThe colors in the gradient will replace the corresponding colors at\nthe same position in the color palette and apply to the image. This\nproperty is read-only."]
        #[serde(rename = "recolorStops", default)]
        pub recolor_stops: Option<Vec<crate::schemas::ColorStop>>,
    }
    impl ::field_selector::FieldSelector for Recolor {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RefreshSheetsChartRequest {
        #[doc = "The object ID of the chart to refresh."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for RefreshSheetsChartRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ReplaceAllShapesWithImageRequestImageReplaceMethod {
        #[doc = "Unspecified image replace method. This value must not be used."]
        ImageReplaceMethodUnspecified,
        #[doc = "Scales and centers the image to fit within the bounds of the original\nshape and maintains the image's aspect ratio. The rendered size of the\nimage may be smaller than the size of the shape. This is the default\nmethod when one is not specified."]
        CenterInside,
        #[doc = "Scales and centers the image to fill the bounds of the original shape.\nThe image may be cropped in order to fill the shape. The rendered size of\nthe image will be the same as that of the original shape."]
        CenterCrop,
    }
    impl ReplaceAllShapesWithImageRequestImageReplaceMethod {
        pub fn as_str(self) -> &'static str {
            match self { ReplaceAllShapesWithImageRequestImageReplaceMethod :: ImageReplaceMethodUnspecified => "IMAGE_REPLACE_METHOD_UNSPECIFIED" , ReplaceAllShapesWithImageRequestImageReplaceMethod :: CenterInside => "CENTER_INSIDE" , ReplaceAllShapesWithImageRequestImageReplaceMethod :: CenterCrop => "CENTER_CROP" , }
        }
    }
    impl ::std::fmt::Display for ReplaceAllShapesWithImageRequestImageReplaceMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReplaceAllShapesWithImageRequestImageReplaceMethod {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReplaceAllShapesWithImageRequestImageReplaceMethod {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok ( match value { "IMAGE_REPLACE_METHOD_UNSPECIFIED" => ReplaceAllShapesWithImageRequestImageReplaceMethod :: ImageReplaceMethodUnspecified , "CENTER_INSIDE" => ReplaceAllShapesWithImageRequestImageReplaceMethod :: CenterInside , "CENTER_CROP" => ReplaceAllShapesWithImageRequestImageReplaceMethod :: CenterCrop , _ => return Err ( :: serde :: de :: Error :: custom ( format ! ( "invalid enum for #name: {}" , value ) ) ) , } )
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ReplaceAllShapesWithImageRequestReplaceMethod {
        #[doc = "Scales and centers the image to fit within the bounds of the original\nshape and maintains the image's aspect ratio. The rendered size of the\nimage may be smaller than the size of the shape. This is the default\nmethod when one is not specified."]
        CenterInside,
        #[doc = "Scales and centers the image to fill the bounds of the original shape.\nThe image may be cropped in order to fill the shape. The rendered size of\nthe image will be the same as that of the original shape."]
        CenterCrop,
    }
    impl ReplaceAllShapesWithImageRequestReplaceMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                ReplaceAllShapesWithImageRequestReplaceMethod::CenterInside => "CENTER_INSIDE",
                ReplaceAllShapesWithImageRequestReplaceMethod::CenterCrop => "CENTER_CROP",
            }
        }
    }
    impl ::std::fmt::Display for ReplaceAllShapesWithImageRequestReplaceMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReplaceAllShapesWithImageRequestReplaceMethod {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReplaceAllShapesWithImageRequestReplaceMethod {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CENTER_INSIDE" => ReplaceAllShapesWithImageRequestReplaceMethod::CenterInside,
                "CENTER_CROP" => ReplaceAllShapesWithImageRequestReplaceMethod::CenterCrop,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceAllShapesWithImageRequest {
        #[doc = "If set, this request will replace all of the shapes that contain the\ngiven text."]
        #[serde(rename = "containsText", default)]
        pub contains_text: Option<crate::schemas::SubstringMatchCriteria>,
        #[doc = "The image replace method.\n\nIf you specify both a `replace_method` and an `image_replace_method`, the\n`image_replace_method` takes precedence.\n\nIf you do not specify a value for `image_replace_method`, but specify a\nvalue for `replace_method`, then the specified `replace_method` value is\nused.\n\nIf you do not specify either, then CENTER_INSIDE is used."]
        #[serde(rename = "imageReplaceMethod", default)]
        pub image_replace_method:
            Option<crate::schemas::ReplaceAllShapesWithImageRequestImageReplaceMethod>,
        #[doc = "The image URL.\n\nThe image is fetched once at insertion time and a copy is stored for\ndisplay inside the presentation. Images must be less than 50MB in size,\ncannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF\nformat.\n\nThe provided URL can be at most 2 kB in length. The URL itself is saved\nwith the image, and exposed via the Image.source_url field."]
        #[serde(rename = "imageUrl", default)]
        pub image_url: Option<String>,
        #[doc = "If non-empty, limits the matches to page elements only on the given pages.\n\nReturns a 400 bad request error if given the page object ID of a\nnotes page or a\nnotes master, or if a\npage with that object ID doesn't exist in the presentation."]
        #[serde(rename = "pageObjectIds", default)]
        pub page_object_ids: Option<Vec<String>>,
        #[doc = "The replace method.\n\n<b>Deprecated</b>: use `image_replace_method` instead.\n\nIf you specify both a `replace_method` and an `image_replace_method`, the\n`image_replace_method` takes precedence."]
        #[serde(rename = "replaceMethod", default)]
        pub replace_method: Option<crate::schemas::ReplaceAllShapesWithImageRequestReplaceMethod>,
    }
    impl ::field_selector::FieldSelector for ReplaceAllShapesWithImageRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceAllShapesWithImageResponse {
        #[doc = "The number of shapes replaced with images."]
        #[serde(rename = "occurrencesChanged", default)]
        pub occurrences_changed: Option<i32>,
    }
    impl ::field_selector::FieldSelector for ReplaceAllShapesWithImageResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ReplaceAllShapesWithSheetsChartRequestLinkingMode {
        #[doc = "The chart is not associated with the source spreadsheet and cannot be\nupdated. A chart that is not linked will be inserted as an image."]
        NotLinkedImage,
        #[doc = "Linking the chart allows it to be updated, and other collaborators will\nsee a link to the spreadsheet."]
        Linked,
    }
    impl ReplaceAllShapesWithSheetsChartRequestLinkingMode {
        pub fn as_str(self) -> &'static str {
            match self {
                ReplaceAllShapesWithSheetsChartRequestLinkingMode::NotLinkedImage => {
                    "NOT_LINKED_IMAGE"
                }
                ReplaceAllShapesWithSheetsChartRequestLinkingMode::Linked => "LINKED",
            }
        }
    }
    impl ::std::fmt::Display for ReplaceAllShapesWithSheetsChartRequestLinkingMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReplaceAllShapesWithSheetsChartRequestLinkingMode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReplaceAllShapesWithSheetsChartRequestLinkingMode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NOT_LINKED_IMAGE" => {
                    ReplaceAllShapesWithSheetsChartRequestLinkingMode::NotLinkedImage
                }
                "LINKED" => ReplaceAllShapesWithSheetsChartRequestLinkingMode::Linked,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceAllShapesWithSheetsChartRequest {
        #[doc = "The ID of the specific chart in the Google Sheets spreadsheet."]
        #[serde(rename = "chartId", default)]
        pub chart_id: Option<i32>,
        #[doc = "The criteria that the shapes must match in order to be replaced. The\nrequest will replace all of the shapes that contain the given text."]
        #[serde(rename = "containsText", default)]
        pub contains_text: Option<crate::schemas::SubstringMatchCriteria>,
        #[doc = "The mode with which the chart is linked to the source spreadsheet. When\nnot specified, the chart will be an image that is not linked."]
        #[serde(rename = "linkingMode", default)]
        pub linking_mode: Option<crate::schemas::ReplaceAllShapesWithSheetsChartRequestLinkingMode>,
        #[doc = "If non-empty, limits the matches to page elements only on the given pages.\n\nReturns a 400 bad request error if given the page object ID of a\nnotes page or a\nnotes master, or if a\npage with that object ID doesn't exist in the presentation."]
        #[serde(rename = "pageObjectIds", default)]
        pub page_object_ids: Option<Vec<String>>,
        #[doc = "The ID of the Google Sheets spreadsheet that contains the chart."]
        #[serde(rename = "spreadsheetId", default)]
        pub spreadsheet_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for ReplaceAllShapesWithSheetsChartRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceAllShapesWithSheetsChartResponse {
        #[doc = "The number of shapes replaced with charts."]
        #[serde(rename = "occurrencesChanged", default)]
        pub occurrences_changed: Option<i32>,
    }
    impl ::field_selector::FieldSelector for ReplaceAllShapesWithSheetsChartResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceAllTextRequest {
        #[doc = "Finds text in a shape matching this substring."]
        #[serde(rename = "containsText", default)]
        pub contains_text: Option<crate::schemas::SubstringMatchCriteria>,
        #[doc = "If non-empty, limits the matches to page elements only on the given pages.\n\nReturns a 400 bad request error if given the page object ID of a\nnotes master,\nor if a page with that object ID doesn't exist in the presentation."]
        #[serde(rename = "pageObjectIds", default)]
        pub page_object_ids: Option<Vec<String>>,
        #[doc = "The text that will replace the matched text."]
        #[serde(rename = "replaceText", default)]
        pub replace_text: Option<String>,
    }
    impl ::field_selector::FieldSelector for ReplaceAllTextRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceAllTextResponse {
        #[doc = "The number of occurrences changed by replacing all text."]
        #[serde(rename = "occurrencesChanged", default)]
        pub occurrences_changed: Option<i32>,
    }
    impl ::field_selector::FieldSelector for ReplaceAllTextResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ReplaceImageRequestImageReplaceMethod {
        #[doc = "Unspecified image replace method. This value must not be used."]
        ImageReplaceMethodUnspecified,
        #[doc = "Scales and centers the image to fit within the bounds of the original\nshape and maintains the image's aspect ratio. The rendered size of the\nimage may be smaller than the size of the shape. This is the default\nmethod when one is not specified."]
        CenterInside,
        #[doc = "Scales and centers the image to fill the bounds of the original shape.\nThe image may be cropped in order to fill the shape. The rendered size of\nthe image will be the same as that of the original shape."]
        CenterCrop,
    }
    impl ReplaceImageRequestImageReplaceMethod {
        pub fn as_str(self) -> &'static str {
            match self {
                ReplaceImageRequestImageReplaceMethod::ImageReplaceMethodUnspecified => {
                    "IMAGE_REPLACE_METHOD_UNSPECIFIED"
                }
                ReplaceImageRequestImageReplaceMethod::CenterInside => "CENTER_INSIDE",
                ReplaceImageRequestImageReplaceMethod::CenterCrop => "CENTER_CROP",
            }
        }
    }
    impl ::std::fmt::Display for ReplaceImageRequestImageReplaceMethod {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReplaceImageRequestImageReplaceMethod {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReplaceImageRequestImageReplaceMethod {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "IMAGE_REPLACE_METHOD_UNSPECIFIED" => {
                    ReplaceImageRequestImageReplaceMethod::ImageReplaceMethodUnspecified
                }
                "CENTER_INSIDE" => ReplaceImageRequestImageReplaceMethod::CenterInside,
                "CENTER_CROP" => ReplaceImageRequestImageReplaceMethod::CenterCrop,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReplaceImageRequest {
        #[doc = "The ID of the existing image that will be replaced."]
        #[serde(rename = "imageObjectId", default)]
        pub image_object_id: Option<String>,
        #[doc = "The replacement method."]
        #[serde(rename = "imageReplaceMethod", default)]
        pub image_replace_method: Option<crate::schemas::ReplaceImageRequestImageReplaceMethod>,
        #[doc = "The URL of the new image.\n\nThe image is fetched once at insertion time and a copy is stored for\ndisplay inside the presentation. Images must be less than 50MB in size,\ncannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF\nformat.\n\nThe provided URL can be at most 2 kB in length. The URL itself is saved\nwith the image, and exposed via the Image.source_url field."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
    }
    impl ::field_selector::FieldSelector for ReplaceImageRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Request {
        #[doc = "Creates an image."]
        #[serde(rename = "createImage", default)]
        pub create_image: Option<crate::schemas::CreateImageRequest>,
        #[doc = "Creates a line."]
        #[serde(rename = "createLine", default)]
        pub create_line: Option<crate::schemas::CreateLineRequest>,
        #[doc = "Creates bullets for paragraphs."]
        #[serde(rename = "createParagraphBullets", default)]
        pub create_paragraph_bullets: Option<crate::schemas::CreateParagraphBulletsRequest>,
        #[doc = "Creates a new shape."]
        #[serde(rename = "createShape", default)]
        pub create_shape: Option<crate::schemas::CreateShapeRequest>,
        #[doc = "Creates an embedded Google Sheets chart."]
        #[serde(rename = "createSheetsChart", default)]
        pub create_sheets_chart: Option<crate::schemas::CreateSheetsChartRequest>,
        #[doc = "Creates a new slide."]
        #[serde(rename = "createSlide", default)]
        pub create_slide: Option<crate::schemas::CreateSlideRequest>,
        #[doc = "Creates a new table."]
        #[serde(rename = "createTable", default)]
        pub create_table: Option<crate::schemas::CreateTableRequest>,
        #[doc = "Creates a video."]
        #[serde(rename = "createVideo", default)]
        pub create_video: Option<crate::schemas::CreateVideoRequest>,
        #[doc = "Deletes a page or page element from the presentation."]
        #[serde(rename = "deleteObject", default)]
        pub delete_object: Option<crate::schemas::DeleteObjectRequest>,
        #[doc = "Deletes bullets from paragraphs."]
        #[serde(rename = "deleteParagraphBullets", default)]
        pub delete_paragraph_bullets: Option<crate::schemas::DeleteParagraphBulletsRequest>,
        #[doc = "Deletes a column from a table."]
        #[serde(rename = "deleteTableColumn", default)]
        pub delete_table_column: Option<crate::schemas::DeleteTableColumnRequest>,
        #[doc = "Deletes a row from a table."]
        #[serde(rename = "deleteTableRow", default)]
        pub delete_table_row: Option<crate::schemas::DeleteTableRowRequest>,
        #[doc = "Deletes text from a shape or a table cell."]
        #[serde(rename = "deleteText", default)]
        pub delete_text: Option<crate::schemas::DeleteTextRequest>,
        #[doc = "Duplicates a slide or page element."]
        #[serde(rename = "duplicateObject", default)]
        pub duplicate_object: Option<crate::schemas::DuplicateObjectRequest>,
        #[doc = "Groups objects, such as page elements."]
        #[serde(rename = "groupObjects", default)]
        pub group_objects: Option<crate::schemas::GroupObjectsRequest>,
        #[doc = "Inserts columns into a table."]
        #[serde(rename = "insertTableColumns", default)]
        pub insert_table_columns: Option<crate::schemas::InsertTableColumnsRequest>,
        #[doc = "Inserts rows into a table."]
        #[serde(rename = "insertTableRows", default)]
        pub insert_table_rows: Option<crate::schemas::InsertTableRowsRequest>,
        #[doc = "Inserts text into a shape or table cell."]
        #[serde(rename = "insertText", default)]
        pub insert_text: Option<crate::schemas::InsertTextRequest>,
        #[doc = "Merges cells in a Table."]
        #[serde(rename = "mergeTableCells", default)]
        pub merge_table_cells: Option<crate::schemas::MergeTableCellsRequest>,
        #[doc = "Refreshes a Google Sheets chart."]
        #[serde(rename = "refreshSheetsChart", default)]
        pub refresh_sheets_chart: Option<crate::schemas::RefreshSheetsChartRequest>,
        #[doc = "Replaces all shapes matching some criteria with an image."]
        #[serde(rename = "replaceAllShapesWithImage", default)]
        pub replace_all_shapes_with_image: Option<crate::schemas::ReplaceAllShapesWithImageRequest>,
        #[doc = "Replaces all shapes matching some criteria with a Google Sheets chart."]
        #[serde(rename = "replaceAllShapesWithSheetsChart", default)]
        pub replace_all_shapes_with_sheets_chart:
            Option<crate::schemas::ReplaceAllShapesWithSheetsChartRequest>,
        #[doc = "Replaces all instances of specified text."]
        #[serde(rename = "replaceAllText", default)]
        pub replace_all_text: Option<crate::schemas::ReplaceAllTextRequest>,
        #[doc = "Replaces an existing image with a new image."]
        #[serde(rename = "replaceImage", default)]
        pub replace_image: Option<crate::schemas::ReplaceImageRequest>,
        #[doc = "Reroutes a line such that it's connected\nat the two closest connection sites on the connected page elements."]
        #[serde(rename = "rerouteLine", default)]
        pub reroute_line: Option<crate::schemas::RerouteLineRequest>,
        #[doc = "Ungroups objects, such as groups."]
        #[serde(rename = "ungroupObjects", default)]
        pub ungroup_objects: Option<crate::schemas::UngroupObjectsRequest>,
        #[doc = "Unmerges cells in a Table."]
        #[serde(rename = "unmergeTableCells", default)]
        pub unmerge_table_cells: Option<crate::schemas::UnmergeTableCellsRequest>,
        #[doc = "Updates the properties of an Image."]
        #[serde(rename = "updateImageProperties", default)]
        pub update_image_properties: Option<crate::schemas::UpdateImagePropertiesRequest>,
        #[doc = "Updates the category of a line."]
        #[serde(rename = "updateLineCategory", default)]
        pub update_line_category: Option<crate::schemas::UpdateLineCategoryRequest>,
        #[doc = "Updates the properties of a Line."]
        #[serde(rename = "updateLineProperties", default)]
        pub update_line_properties: Option<crate::schemas::UpdateLinePropertiesRequest>,
        #[doc = "Updates the alt text title and/or description of a\npage element."]
        #[serde(rename = "updatePageElementAltText", default)]
        pub update_page_element_alt_text: Option<crate::schemas::UpdatePageElementAltTextRequest>,
        #[doc = "Updates the transform of a page element."]
        #[serde(rename = "updatePageElementTransform", default)]
        pub update_page_element_transform:
            Option<crate::schemas::UpdatePageElementTransformRequest>,
        #[doc = "Updates the Z-order of page elements."]
        #[serde(rename = "updatePageElementsZOrder", default)]
        pub update_page_elements_z_order: Option<crate::schemas::UpdatePageElementsZOrderRequest>,
        #[doc = "Updates the properties of a Page."]
        #[serde(rename = "updatePageProperties", default)]
        pub update_page_properties: Option<crate::schemas::UpdatePagePropertiesRequest>,
        #[doc = "Updates the styling of paragraphs within a Shape or Table."]
        #[serde(rename = "updateParagraphStyle", default)]
        pub update_paragraph_style: Option<crate::schemas::UpdateParagraphStyleRequest>,
        #[doc = "Updates the properties of a Shape."]
        #[serde(rename = "updateShapeProperties", default)]
        pub update_shape_properties: Option<crate::schemas::UpdateShapePropertiesRequest>,
        #[doc = "Updates the position of a set of slides in the presentation."]
        #[serde(rename = "updateSlidesPosition", default)]
        pub update_slides_position: Option<crate::schemas::UpdateSlidesPositionRequest>,
        #[doc = "Updates the properties of the table borders in a Table."]
        #[serde(rename = "updateTableBorderProperties", default)]
        pub update_table_border_properties:
            Option<crate::schemas::UpdateTableBorderPropertiesRequest>,
        #[doc = "Updates the properties of a TableCell."]
        #[serde(rename = "updateTableCellProperties", default)]
        pub update_table_cell_properties: Option<crate::schemas::UpdateTableCellPropertiesRequest>,
        #[doc = "Updates the properties of a Table\ncolumn."]
        #[serde(rename = "updateTableColumnProperties", default)]
        pub update_table_column_properties:
            Option<crate::schemas::UpdateTableColumnPropertiesRequest>,
        #[doc = "Updates the properties of a Table row."]
        #[serde(rename = "updateTableRowProperties", default)]
        pub update_table_row_properties: Option<crate::schemas::UpdateTableRowPropertiesRequest>,
        #[doc = "Updates the styling of text within a Shape or Table."]
        #[serde(rename = "updateTextStyle", default)]
        pub update_text_style: Option<crate::schemas::UpdateTextStyleRequest>,
        #[doc = "Updates the properties of a Video."]
        #[serde(rename = "updateVideoProperties", default)]
        pub update_video_properties: Option<crate::schemas::UpdateVideoPropertiesRequest>,
    }
    impl ::field_selector::FieldSelector for Request {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RerouteLineRequest {
        #[doc = "The object ID of the line to reroute.\n\nOnly a line with a category\nindicating it is a \"connector\" can be rerouted. The start and end\nconnections of the line must be on different page elements."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for RerouteLineRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "The result of creating an image."]
        #[serde(rename = "createImage", default)]
        pub create_image: Option<crate::schemas::CreateImageResponse>,
        #[doc = "The result of creating a line."]
        #[serde(rename = "createLine", default)]
        pub create_line: Option<crate::schemas::CreateLineResponse>,
        #[doc = "The result of creating a shape."]
        #[serde(rename = "createShape", default)]
        pub create_shape: Option<crate::schemas::CreateShapeResponse>,
        #[doc = "The result of creating a Google Sheets chart."]
        #[serde(rename = "createSheetsChart", default)]
        pub create_sheets_chart: Option<crate::schemas::CreateSheetsChartResponse>,
        #[doc = "The result of creating a slide."]
        #[serde(rename = "createSlide", default)]
        pub create_slide: Option<crate::schemas::CreateSlideResponse>,
        #[doc = "The result of creating a table."]
        #[serde(rename = "createTable", default)]
        pub create_table: Option<crate::schemas::CreateTableResponse>,
        #[doc = "The result of creating a video."]
        #[serde(rename = "createVideo", default)]
        pub create_video: Option<crate::schemas::CreateVideoResponse>,
        #[doc = "The result of duplicating an object."]
        #[serde(rename = "duplicateObject", default)]
        pub duplicate_object: Option<crate::schemas::DuplicateObjectResponse>,
        #[doc = "The result of grouping objects."]
        #[serde(rename = "groupObjects", default)]
        pub group_objects: Option<crate::schemas::GroupObjectsResponse>,
        #[doc = "The result of replacing all shapes matching some criteria with an\nimage."]
        #[serde(rename = "replaceAllShapesWithImage", default)]
        pub replace_all_shapes_with_image:
            Option<crate::schemas::ReplaceAllShapesWithImageResponse>,
        #[doc = "The result of replacing all shapes matching some criteria with a Google\nSheets chart."]
        #[serde(rename = "replaceAllShapesWithSheetsChart", default)]
        pub replace_all_shapes_with_sheets_chart:
            Option<crate::schemas::ReplaceAllShapesWithSheetsChartResponse>,
        #[doc = "The result of replacing text."]
        #[serde(rename = "replaceAllText", default)]
        pub replace_all_text: Option<crate::schemas::ReplaceAllTextResponse>,
    }
    impl ::field_selector::FieldSelector for Response {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RgbColor {
        #[doc = "The blue component of the color, from 0.0 to 1.0."]
        #[serde(rename = "blue", default)]
        pub blue: Option<f32>,
        #[doc = "The green component of the color, from 0.0 to 1.0."]
        #[serde(rename = "green", default)]
        pub green: Option<f32>,
        #[doc = "The red component of the color, from 0.0 to 1.0."]
        #[serde(rename = "red", default)]
        pub red: Option<f32>,
    }
    impl ::field_selector::FieldSelector for RgbColor {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ShadowAlignment {
        #[doc = "Unspecified."]
        RectanglePositionUnspecified,
        #[doc = "Top left."]
        TopLeft,
        #[doc = "Top center."]
        TopCenter,
        #[doc = "Top right."]
        TopRight,
        #[doc = "Left center."]
        LeftCenter,
        #[doc = "Center."]
        Center,
        #[doc = "Right center."]
        RightCenter,
        #[doc = "Bottom left."]
        BottomLeft,
        #[doc = "Bottom center."]
        BottomCenter,
        #[doc = "Bottom right."]
        BottomRight,
    }
    impl ShadowAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                ShadowAlignment::RectanglePositionUnspecified => "RECTANGLE_POSITION_UNSPECIFIED",
                ShadowAlignment::TopLeft => "TOP_LEFT",
                ShadowAlignment::TopCenter => "TOP_CENTER",
                ShadowAlignment::TopRight => "TOP_RIGHT",
                ShadowAlignment::LeftCenter => "LEFT_CENTER",
                ShadowAlignment::Center => "CENTER",
                ShadowAlignment::RightCenter => "RIGHT_CENTER",
                ShadowAlignment::BottomLeft => "BOTTOM_LEFT",
                ShadowAlignment::BottomCenter => "BOTTOM_CENTER",
                ShadowAlignment::BottomRight => "BOTTOM_RIGHT",
            }
        }
    }
    impl ::std::fmt::Display for ShadowAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ShadowAlignment {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ShadowAlignment {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "RECTANGLE_POSITION_UNSPECIFIED" => ShadowAlignment::RectanglePositionUnspecified,
                "TOP_LEFT" => ShadowAlignment::TopLeft,
                "TOP_CENTER" => ShadowAlignment::TopCenter,
                "TOP_RIGHT" => ShadowAlignment::TopRight,
                "LEFT_CENTER" => ShadowAlignment::LeftCenter,
                "CENTER" => ShadowAlignment::Center,
                "RIGHT_CENTER" => ShadowAlignment::RightCenter,
                "BOTTOM_LEFT" => ShadowAlignment::BottomLeft,
                "BOTTOM_CENTER" => ShadowAlignment::BottomCenter,
                "BOTTOM_RIGHT" => ShadowAlignment::BottomRight,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ShadowPropertyState {
        #[doc = "If a property's state is RENDERED, then the element has the corresponding\nproperty when rendered on a page. If the element is a placeholder shape as\ndetermined by the placeholder\nfield, and it inherits from a placeholder shape, the corresponding field\nmay be unset, meaning that the property value is inherited from a parent\nplaceholder. If the element does not inherit, then the field will contain\nthe rendered value. This is the default value."]
        Rendered,
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the\ncorresponding property when rendered on a page. However, the field may\nstill be set so it can be inherited by child shapes. To remove a property\nfrom a rendered element, set its property_state to NOT_RENDERED."]
        NotRendered,
        #[doc = "If a property's state is INHERIT, then the property state uses the value of\ncorresponding `property_state` field on the parent shape. Elements that do\nnot inherit will never have an INHERIT property state."]
        Inherit,
    }
    impl ShadowPropertyState {
        pub fn as_str(self) -> &'static str {
            match self {
                ShadowPropertyState::Rendered => "RENDERED",
                ShadowPropertyState::NotRendered => "NOT_RENDERED",
                ShadowPropertyState::Inherit => "INHERIT",
            }
        }
    }
    impl ::std::fmt::Display for ShadowPropertyState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ShadowPropertyState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ShadowPropertyState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "RENDERED" => ShadowPropertyState::Rendered,
                "NOT_RENDERED" => ShadowPropertyState::NotRendered,
                "INHERIT" => ShadowPropertyState::Inherit,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ShadowType {
        #[doc = "Unspecified shadow type."]
        ShadowTypeUnspecified,
        #[doc = "Outer shadow."]
        Outer,
    }
    impl ShadowType {
        pub fn as_str(self) -> &'static str {
            match self {
                ShadowType::ShadowTypeUnspecified => "SHADOW_TYPE_UNSPECIFIED",
                ShadowType::Outer => "OUTER",
            }
        }
    }
    impl ::std::fmt::Display for ShadowType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ShadowType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ShadowType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SHADOW_TYPE_UNSPECIFIED" => ShadowType::ShadowTypeUnspecified,
                "OUTER" => ShadowType::Outer,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Shadow {
        #[doc = "The alignment point of the shadow, that sets the origin for translate,\nscale and skew of the shadow. This property is read-only."]
        #[serde(rename = "alignment", default)]
        pub alignment: Option<crate::schemas::ShadowAlignment>,
        #[doc = "The alpha of the shadow's color, from 0.0 to 1.0."]
        #[serde(rename = "alpha", default)]
        pub alpha: Option<f32>,
        #[doc = "The radius of the shadow blur. The larger the radius, the more diffuse the\nshadow becomes."]
        #[serde(rename = "blurRadius", default)]
        pub blur_radius: Option<crate::schemas::Dimension>,
        #[doc = "The shadow color value."]
        #[serde(rename = "color", default)]
        pub color: Option<crate::schemas::OpaqueColor>,
        #[doc = "The shadow property state.\n\nUpdating the shadow on a page element will implicitly update this field to\n`RENDERED`, unless another value is specified in the same request. To have\nno shadow on a page element, set this field to `NOT_RENDERED`. In this\ncase, any other shadow fields set in the same request will be ignored."]
        #[serde(rename = "propertyState", default)]
        pub property_state: Option<crate::schemas::ShadowPropertyState>,
        #[doc = "The type of the shadow. This property is read-only."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::ShadowType>,
        #[doc = "Whether the shadow should rotate with the shape. This property is\nread-only."]
        #[serde(rename = "rotateWithShape", default)]
        pub rotate_with_shape: Option<bool>,
        #[doc = "Transform that encodes the translate, scale, and skew of the shadow,\nrelative to the alignment position."]
        #[serde(rename = "transform", default)]
        pub transform: Option<crate::schemas::AffineTransform>,
    }
    impl ::field_selector::FieldSelector for Shadow {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ShapeShapeType {
        #[doc = "The shape type that is not predefined."]
        TypeUnspecified,
        #[doc = "Text box shape."]
        TextBox,
        #[doc = "Rectangle shape. Corresponds to ECMA-376 ST_ShapeType 'rect'."]
        Rectangle,
        #[doc = "Round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'roundRect'"]
        RoundRectangle,
        #[doc = "Ellipse shape. Corresponds to ECMA-376 ST_ShapeType 'ellipse'"]
        Ellipse,
        #[doc = "Curved arc shape. Corresponds to ECMA-376 ST_ShapeType 'arc'"]
        Arc,
        #[doc = "Bent arrow shape. Corresponds to ECMA-376 ST_ShapeType 'bentArrow'"]
        BentArrow,
        #[doc = "Bent up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'bentUpArrow'"]
        BentUpArrow,
        #[doc = "Bevel shape. Corresponds to ECMA-376 ST_ShapeType 'bevel'"]
        Bevel,
        #[doc = "Block arc shape. Corresponds to ECMA-376 ST_ShapeType 'blockArc'"]
        BlockArc,
        #[doc = "Brace pair shape. Corresponds to ECMA-376 ST_ShapeType 'bracePair'"]
        BracePair,
        #[doc = "Bracket pair shape. Corresponds to ECMA-376 ST_ShapeType 'bracketPair'"]
        BracketPair,
        #[doc = "Can shape. Corresponds to ECMA-376 ST_ShapeType 'can'"]
        Can,
        #[doc = "Chevron shape. Corresponds to ECMA-376 ST_ShapeType 'chevron'"]
        Chevron,
        #[doc = "Chord shape. Corresponds to ECMA-376 ST_ShapeType 'chord'"]
        Chord,
        #[doc = "Cloud shape. Corresponds to ECMA-376 ST_ShapeType 'cloud'"]
        Cloud,
        #[doc = "Corner shape. Corresponds to ECMA-376 ST_ShapeType 'corner'"]
        Corner,
        #[doc = "Cube shape. Corresponds to ECMA-376 ST_ShapeType 'cube'"]
        Cube,
        #[doc = "Curved down arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedDownArrow'"]
        CurvedDownArrow,
        #[doc = "Curved left arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedLeftArrow'"]
        CurvedLeftArrow,
        #[doc = "Curved right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedRightArrow'"]
        CurvedRightArrow,
        #[doc = "Curved up arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'curvedUpArrow'"]
        CurvedUpArrow,
        #[doc = "Decagon shape. Corresponds to ECMA-376 ST_ShapeType 'decagon'"]
        Decagon,
        #[doc = "Diagonal stripe shape. Corresponds to ECMA-376 ST_ShapeType 'diagStripe'"]
        DiagonalStripe,
        #[doc = "Diamond shape. Corresponds to ECMA-376 ST_ShapeType 'diamond'"]
        Diamond,
        #[doc = "Dodecagon shape. Corresponds to ECMA-376 ST_ShapeType 'dodecagon'"]
        Dodecagon,
        #[doc = "Donut shape. Corresponds to ECMA-376 ST_ShapeType 'donut'"]
        Donut,
        #[doc = "Double wave shape. Corresponds to ECMA-376 ST_ShapeType 'doubleWave'"]
        DoubleWave,
        #[doc = "Down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'downArrow'"]
        DownArrow,
        #[doc = "Callout down arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'downArrowCallout'"]
        DownArrowCallout,
        #[doc = "Folded corner shape. Corresponds to ECMA-376 ST_ShapeType 'foldedCorner'"]
        FoldedCorner,
        #[doc = "Frame shape. Corresponds to ECMA-376 ST_ShapeType 'frame'"]
        Frame,
        #[doc = "Half frame shape. Corresponds to ECMA-376 ST_ShapeType 'halfFrame'"]
        HalfFrame,
        #[doc = "Heart shape. Corresponds to ECMA-376 ST_ShapeType 'heart'"]
        Heart,
        #[doc = "Heptagon shape. Corresponds to ECMA-376 ST_ShapeType 'heptagon'"]
        Heptagon,
        #[doc = "Hexagon shape. Corresponds to ECMA-376 ST_ShapeType 'hexagon'"]
        Hexagon,
        #[doc = "Home plate shape. Corresponds to ECMA-376 ST_ShapeType 'homePlate'"]
        HomePlate,
        #[doc = "Horizontal scroll shape. Corresponds to ECMA-376 ST_ShapeType\n'horizontalScroll'"]
        HorizontalScroll,
        #[doc = "Irregular seal 1 shape. Corresponds to ECMA-376 ST_ShapeType\n'irregularSeal1'"]
        IrregularSeal1,
        #[doc = "Irregular seal 2 shape. Corresponds to ECMA-376 ST_ShapeType\n'irregularSeal2'"]
        IrregularSeal2,
        #[doc = "Left arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftArrow'"]
        LeftArrow,
        #[doc = "Callout left arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftArrowCallout'"]
        LeftArrowCallout,
        #[doc = "Left brace shape. Corresponds to ECMA-376 ST_ShapeType 'leftBrace'"]
        LeftBrace,
        #[doc = "Left bracket shape. Corresponds to ECMA-376 ST_ShapeType 'leftBracket'"]
        LeftBracket,
        #[doc = "Left right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftRightArrow'"]
        LeftRightArrow,
        #[doc = "Callout left right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftRightArrowCallout'"]
        LeftRightArrowCallout,
        #[doc = "Left right up arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'leftRightUpArrow'"]
        LeftRightUpArrow,
        #[doc = "Left up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'leftUpArrow'"]
        LeftUpArrow,
        #[doc = "Lightning bolt shape. Corresponds to ECMA-376 ST_ShapeType\n'lightningBolt'"]
        LightningBolt,
        #[doc = "Divide math shape. Corresponds to ECMA-376 ST_ShapeType 'mathDivide'"]
        MathDivide,
        #[doc = "Equal math shape. Corresponds to ECMA-376 ST_ShapeType 'mathEqual'"]
        MathEqual,
        #[doc = "Minus math shape. Corresponds to ECMA-376 ST_ShapeType 'mathMinus'"]
        MathMinus,
        #[doc = "Multiply math shape. Corresponds to ECMA-376 ST_ShapeType 'mathMultiply'"]
        MathMultiply,
        #[doc = "Not equal math shape. Corresponds to ECMA-376 ST_ShapeType 'mathNotEqual'"]
        MathNotEqual,
        #[doc = "Plus math shape. Corresponds to ECMA-376 ST_ShapeType 'mathPlus'"]
        MathPlus,
        #[doc = "Moon shape. Corresponds to ECMA-376 ST_ShapeType 'moon'"]
        Moon,
        #[doc = "No smoking shape. Corresponds to ECMA-376 ST_ShapeType 'noSmoking'"]
        NoSmoking,
        #[doc = "Notched right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'notchedRightArrow'"]
        NotchedRightArrow,
        #[doc = "Octagon shape. Corresponds to ECMA-376 ST_ShapeType 'octagon'"]
        Octagon,
        #[doc = "Parallelogram shape. Corresponds to ECMA-376 ST_ShapeType 'parallelogram'"]
        Parallelogram,
        #[doc = "Pentagon shape. Corresponds to ECMA-376 ST_ShapeType 'pentagon'"]
        Pentagon,
        #[doc = "Pie shape. Corresponds to ECMA-376 ST_ShapeType 'pie'"]
        Pie,
        #[doc = "Plaque shape. Corresponds to ECMA-376 ST_ShapeType 'plaque'"]
        Plaque,
        #[doc = "Plus shape. Corresponds to ECMA-376 ST_ShapeType 'plus'"]
        Plus,
        #[doc = "Quad-arrow shape. Corresponds to ECMA-376 ST_ShapeType 'quadArrow'"]
        QuadArrow,
        #[doc = "Callout quad-arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'quadArrowCallout'"]
        QuadArrowCallout,
        #[doc = "Ribbon shape. Corresponds to ECMA-376 ST_ShapeType 'ribbon'"]
        Ribbon,
        #[doc = "Ribbon 2 shape. Corresponds to ECMA-376 ST_ShapeType 'ribbon2'"]
        Ribbon2,
        #[doc = "Right arrow shape. Corresponds to ECMA-376 ST_ShapeType 'rightArrow'"]
        RightArrow,
        #[doc = "Callout right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'rightArrowCallout'"]
        RightArrowCallout,
        #[doc = "Right brace shape. Corresponds to ECMA-376 ST_ShapeType 'rightBrace'"]
        RightBrace,
        #[doc = "Right bracket shape. Corresponds to ECMA-376 ST_ShapeType 'rightBracket'"]
        RightBracket,
        #[doc = "One round corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'round1Rect'"]
        Round1Rectangle,
        #[doc = "Two diagonal round corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'round2DiagRect'"]
        Round2DiagonalRectangle,
        #[doc = "Two same-side round corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'round2SameRect'"]
        Round2SameRectangle,
        #[doc = "Right triangle shape. Corresponds to ECMA-376 ST_ShapeType 'rtTriangle'"]
        RightTriangle,
        #[doc = "Smiley face shape. Corresponds to ECMA-376 ST_ShapeType 'smileyFace'"]
        SmileyFace,
        #[doc = "One snip corner rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'snip1Rect'"]
        Snip1Rectangle,
        #[doc = "Two diagonal snip corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'snip2DiagRect'"]
        Snip2DiagonalRectangle,
        #[doc = "Two same-side snip corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'snip2SameRect'"]
        Snip2SameRectangle,
        #[doc = "One snip one round corner rectangle shape. Corresponds to ECMA-376\nST_ShapeType 'snipRoundRect'"]
        SnipRoundRectangle,
        #[doc = "Ten pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star10'"]
        Star10,
        #[doc = "Twelve pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star12'"]
        Star12,
        #[doc = "Sixteen pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star16'"]
        Star16,
        #[doc = "Twenty four pointed star shape. Corresponds to ECMA-376 ST_ShapeType\n'star24'"]
        Star24,
        #[doc = "Thirty two pointed star shape. Corresponds to ECMA-376 ST_ShapeType\n'star32'"]
        Star32,
        #[doc = "Four pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star4'"]
        Star4,
        #[doc = "Five pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star5'"]
        Star5,
        #[doc = "Six pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star6'"]
        Star6,
        #[doc = "Seven pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star7'"]
        Star7,
        #[doc = "Eight pointed star shape. Corresponds to ECMA-376 ST_ShapeType 'star8'"]
        Star8,
        #[doc = "Striped right arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'stripedRightArrow'"]
        StripedRightArrow,
        #[doc = "Sun shape. Corresponds to ECMA-376 ST_ShapeType 'sun'"]
        Sun,
        #[doc = "Trapezoid shape. Corresponds to ECMA-376 ST_ShapeType 'trapezoid'"]
        Trapezoid,
        #[doc = "Triangle shape. Corresponds to ECMA-376 ST_ShapeType 'triangle'"]
        Triangle,
        #[doc = "Up arrow shape. Corresponds to ECMA-376 ST_ShapeType 'upArrow'"]
        UpArrow,
        #[doc = "Callout up arrow shape. Corresponds to ECMA-376 ST_ShapeType\n'upArrowCallout'"]
        UpArrowCallout,
        #[doc = "Up down arrow shape. Corresponds to ECMA-376 ST_ShapeType 'upDownArrow'"]
        UpDownArrow,
        #[doc = "U-turn arrow shape. Corresponds to ECMA-376 ST_ShapeType 'uturnArrow'"]
        UturnArrow,
        #[doc = "Vertical scroll shape. Corresponds to ECMA-376 ST_ShapeType\n'verticalScroll'"]
        VerticalScroll,
        #[doc = "Wave shape. Corresponds to ECMA-376 ST_ShapeType 'wave'"]
        Wave,
        #[doc = "Callout wedge ellipse shape. Corresponds to ECMA-376 ST_ShapeType\n'wedgeEllipseCallout'"]
        WedgeEllipseCallout,
        #[doc = "Callout wedge rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'wedgeRectCallout'"]
        WedgeRectangleCallout,
        #[doc = "Callout wedge round rectangle shape. Corresponds to ECMA-376 ST_ShapeType\n'wedgeRoundRectCallout'"]
        WedgeRoundRectangleCallout,
        #[doc = "Alternate process flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartAlternateProcess'"]
        FlowChartAlternateProcess,
        #[doc = "Collate flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartCollate'"]
        FlowChartCollate,
        #[doc = "Connector flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartConnector'"]
        FlowChartConnector,
        #[doc = "Decision flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartDecision'"]
        FlowChartDecision,
        #[doc = "Delay flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartDelay'"]
        FlowChartDelay,
        #[doc = "Display flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartDisplay'"]
        FlowChartDisplay,
        #[doc = "Document flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartDocument'"]
        FlowChartDocument,
        #[doc = "Extract flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartExtract'"]
        FlowChartExtract,
        #[doc = "Input output flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartInputOutput'"]
        FlowChartInputOutput,
        #[doc = "Internal storage flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartInternalStorage'"]
        FlowChartInternalStorage,
        #[doc = "Magnetic disk flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMagneticDisk'"]
        FlowChartMagneticDisk,
        #[doc = "Magnetic drum flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMagneticDrum'"]
        FlowChartMagneticDrum,
        #[doc = "Magnetic tape flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMagneticTape'"]
        FlowChartMagneticTape,
        #[doc = "Manual input flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartManualInput'"]
        FlowChartManualInput,
        #[doc = "Manual operation flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartManualOperation'"]
        FlowChartManualOperation,
        #[doc = "Merge flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartMerge'"]
        FlowChartMerge,
        #[doc = "Multi-document flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartMultidocument'"]
        FlowChartMultidocument,
        #[doc = "Offline storage flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartOfflineStorage'"]
        FlowChartOfflineStorage,
        #[doc = "Off-page connector flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartOffpageConnector'"]
        FlowChartOffpageConnector,
        #[doc = "Online storage flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartOnlineStorage'"]
        FlowChartOnlineStorage,
        #[doc = "Or flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartOr'"]
        FlowChartOr,
        #[doc = "Predefined process flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPredefinedProcess'"]
        FlowChartPredefinedProcess,
        #[doc = "Preparation flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPreparation'"]
        FlowChartPreparation,
        #[doc = "Process flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartProcess'"]
        FlowChartProcess,
        #[doc = "Punched card flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPunchedCard'"]
        FlowChartPunchedCard,
        #[doc = "Punched tape flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartPunchedTape'"]
        FlowChartPunchedTape,
        #[doc = "Sort flow shape. Corresponds to ECMA-376 ST_ShapeType 'flowChartSort'"]
        FlowChartSort,
        #[doc = "Summing junction flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartSummingJunction'"]
        FlowChartSummingJunction,
        #[doc = "Terminator flow shape. Corresponds to ECMA-376 ST_ShapeType\n'flowChartTerminator'"]
        FlowChartTerminator,
        #[doc = "East arrow shape."]
        ArrowEast,
        #[doc = "Northeast arrow shape."]
        ArrowNorthEast,
        #[doc = "North arrow shape."]
        ArrowNorth,
        #[doc = "Speech shape."]
        Speech,
        #[doc = "Star burst shape."]
        Starburst,
        #[doc = "Teardrop shape. Corresponds to ECMA-376 ST_ShapeType 'teardrop'"]
        Teardrop,
        #[doc = "Ellipse ribbon shape. Corresponds to ECMA-376 ST_ShapeType\n'ellipseRibbon'"]
        EllipseRibbon,
        #[doc = "Ellipse ribbon 2 shape. Corresponds to ECMA-376 ST_ShapeType\n'ellipseRibbon2'"]
        EllipseRibbon2,
        #[doc = "Callout cloud shape. Corresponds to ECMA-376 ST_ShapeType 'cloudCallout'"]
        CloudCallout,
        #[doc = "Custom shape."]
        Custom,
    }
    impl ShapeShapeType {
        pub fn as_str(self) -> &'static str {
            match self {
                ShapeShapeType::TypeUnspecified => "TYPE_UNSPECIFIED",
                ShapeShapeType::TextBox => "TEXT_BOX",
                ShapeShapeType::Rectangle => "RECTANGLE",
                ShapeShapeType::RoundRectangle => "ROUND_RECTANGLE",
                ShapeShapeType::Ellipse => "ELLIPSE",
                ShapeShapeType::Arc => "ARC",
                ShapeShapeType::BentArrow => "BENT_ARROW",
                ShapeShapeType::BentUpArrow => "BENT_UP_ARROW",
                ShapeShapeType::Bevel => "BEVEL",
                ShapeShapeType::BlockArc => "BLOCK_ARC",
                ShapeShapeType::BracePair => "BRACE_PAIR",
                ShapeShapeType::BracketPair => "BRACKET_PAIR",
                ShapeShapeType::Can => "CAN",
                ShapeShapeType::Chevron => "CHEVRON",
                ShapeShapeType::Chord => "CHORD",
                ShapeShapeType::Cloud => "CLOUD",
                ShapeShapeType::Corner => "CORNER",
                ShapeShapeType::Cube => "CUBE",
                ShapeShapeType::CurvedDownArrow => "CURVED_DOWN_ARROW",
                ShapeShapeType::CurvedLeftArrow => "CURVED_LEFT_ARROW",
                ShapeShapeType::CurvedRightArrow => "CURVED_RIGHT_ARROW",
                ShapeShapeType::CurvedUpArrow => "CURVED_UP_ARROW",
                ShapeShapeType::Decagon => "DECAGON",
                ShapeShapeType::DiagonalStripe => "DIAGONAL_STRIPE",
                ShapeShapeType::Diamond => "DIAMOND",
                ShapeShapeType::Dodecagon => "DODECAGON",
                ShapeShapeType::Donut => "DONUT",
                ShapeShapeType::DoubleWave => "DOUBLE_WAVE",
                ShapeShapeType::DownArrow => "DOWN_ARROW",
                ShapeShapeType::DownArrowCallout => "DOWN_ARROW_CALLOUT",
                ShapeShapeType::FoldedCorner => "FOLDED_CORNER",
                ShapeShapeType::Frame => "FRAME",
                ShapeShapeType::HalfFrame => "HALF_FRAME",
                ShapeShapeType::Heart => "HEART",
                ShapeShapeType::Heptagon => "HEPTAGON",
                ShapeShapeType::Hexagon => "HEXAGON",
                ShapeShapeType::HomePlate => "HOME_PLATE",
                ShapeShapeType::HorizontalScroll => "HORIZONTAL_SCROLL",
                ShapeShapeType::IrregularSeal1 => "IRREGULAR_SEAL_1",
                ShapeShapeType::IrregularSeal2 => "IRREGULAR_SEAL_2",
                ShapeShapeType::LeftArrow => "LEFT_ARROW",
                ShapeShapeType::LeftArrowCallout => "LEFT_ARROW_CALLOUT",
                ShapeShapeType::LeftBrace => "LEFT_BRACE",
                ShapeShapeType::LeftBracket => "LEFT_BRACKET",
                ShapeShapeType::LeftRightArrow => "LEFT_RIGHT_ARROW",
                ShapeShapeType::LeftRightArrowCallout => "LEFT_RIGHT_ARROW_CALLOUT",
                ShapeShapeType::LeftRightUpArrow => "LEFT_RIGHT_UP_ARROW",
                ShapeShapeType::LeftUpArrow => "LEFT_UP_ARROW",
                ShapeShapeType::LightningBolt => "LIGHTNING_BOLT",
                ShapeShapeType::MathDivide => "MATH_DIVIDE",
                ShapeShapeType::MathEqual => "MATH_EQUAL",
                ShapeShapeType::MathMinus => "MATH_MINUS",
                ShapeShapeType::MathMultiply => "MATH_MULTIPLY",
                ShapeShapeType::MathNotEqual => "MATH_NOT_EQUAL",
                ShapeShapeType::MathPlus => "MATH_PLUS",
                ShapeShapeType::Moon => "MOON",
                ShapeShapeType::NoSmoking => "NO_SMOKING",
                ShapeShapeType::NotchedRightArrow => "NOTCHED_RIGHT_ARROW",
                ShapeShapeType::Octagon => "OCTAGON",
                ShapeShapeType::Parallelogram => "PARALLELOGRAM",
                ShapeShapeType::Pentagon => "PENTAGON",
                ShapeShapeType::Pie => "PIE",
                ShapeShapeType::Plaque => "PLAQUE",
                ShapeShapeType::Plus => "PLUS",
                ShapeShapeType::QuadArrow => "QUAD_ARROW",
                ShapeShapeType::QuadArrowCallout => "QUAD_ARROW_CALLOUT",
                ShapeShapeType::Ribbon => "RIBBON",
                ShapeShapeType::Ribbon2 => "RIBBON_2",
                ShapeShapeType::RightArrow => "RIGHT_ARROW",
                ShapeShapeType::RightArrowCallout => "RIGHT_ARROW_CALLOUT",
                ShapeShapeType::RightBrace => "RIGHT_BRACE",
                ShapeShapeType::RightBracket => "RIGHT_BRACKET",
                ShapeShapeType::Round1Rectangle => "ROUND_1_RECTANGLE",
                ShapeShapeType::Round2DiagonalRectangle => "ROUND_2_DIAGONAL_RECTANGLE",
                ShapeShapeType::Round2SameRectangle => "ROUND_2_SAME_RECTANGLE",
                ShapeShapeType::RightTriangle => "RIGHT_TRIANGLE",
                ShapeShapeType::SmileyFace => "SMILEY_FACE",
                ShapeShapeType::Snip1Rectangle => "SNIP_1_RECTANGLE",
                ShapeShapeType::Snip2DiagonalRectangle => "SNIP_2_DIAGONAL_RECTANGLE",
                ShapeShapeType::Snip2SameRectangle => "SNIP_2_SAME_RECTANGLE",
                ShapeShapeType::SnipRoundRectangle => "SNIP_ROUND_RECTANGLE",
                ShapeShapeType::Star10 => "STAR_10",
                ShapeShapeType::Star12 => "STAR_12",
                ShapeShapeType::Star16 => "STAR_16",
                ShapeShapeType::Star24 => "STAR_24",
                ShapeShapeType::Star32 => "STAR_32",
                ShapeShapeType::Star4 => "STAR_4",
                ShapeShapeType::Star5 => "STAR_5",
                ShapeShapeType::Star6 => "STAR_6",
                ShapeShapeType::Star7 => "STAR_7",
                ShapeShapeType::Star8 => "STAR_8",
                ShapeShapeType::StripedRightArrow => "STRIPED_RIGHT_ARROW",
                ShapeShapeType::Sun => "SUN",
                ShapeShapeType::Trapezoid => "TRAPEZOID",
                ShapeShapeType::Triangle => "TRIANGLE",
                ShapeShapeType::UpArrow => "UP_ARROW",
                ShapeShapeType::UpArrowCallout => "UP_ARROW_CALLOUT",
                ShapeShapeType::UpDownArrow => "UP_DOWN_ARROW",
                ShapeShapeType::UturnArrow => "UTURN_ARROW",
                ShapeShapeType::VerticalScroll => "VERTICAL_SCROLL",
                ShapeShapeType::Wave => "WAVE",
                ShapeShapeType::WedgeEllipseCallout => "WEDGE_ELLIPSE_CALLOUT",
                ShapeShapeType::WedgeRectangleCallout => "WEDGE_RECTANGLE_CALLOUT",
                ShapeShapeType::WedgeRoundRectangleCallout => "WEDGE_ROUND_RECTANGLE_CALLOUT",
                ShapeShapeType::FlowChartAlternateProcess => "FLOW_CHART_ALTERNATE_PROCESS",
                ShapeShapeType::FlowChartCollate => "FLOW_CHART_COLLATE",
                ShapeShapeType::FlowChartConnector => "FLOW_CHART_CONNECTOR",
                ShapeShapeType::FlowChartDecision => "FLOW_CHART_DECISION",
                ShapeShapeType::FlowChartDelay => "FLOW_CHART_DELAY",
                ShapeShapeType::FlowChartDisplay => "FLOW_CHART_DISPLAY",
                ShapeShapeType::FlowChartDocument => "FLOW_CHART_DOCUMENT",
                ShapeShapeType::FlowChartExtract => "FLOW_CHART_EXTRACT",
                ShapeShapeType::FlowChartInputOutput => "FLOW_CHART_INPUT_OUTPUT",
                ShapeShapeType::FlowChartInternalStorage => "FLOW_CHART_INTERNAL_STORAGE",
                ShapeShapeType::FlowChartMagneticDisk => "FLOW_CHART_MAGNETIC_DISK",
                ShapeShapeType::FlowChartMagneticDrum => "FLOW_CHART_MAGNETIC_DRUM",
                ShapeShapeType::FlowChartMagneticTape => "FLOW_CHART_MAGNETIC_TAPE",
                ShapeShapeType::FlowChartManualInput => "FLOW_CHART_MANUAL_INPUT",
                ShapeShapeType::FlowChartManualOperation => "FLOW_CHART_MANUAL_OPERATION",
                ShapeShapeType::FlowChartMerge => "FLOW_CHART_MERGE",
                ShapeShapeType::FlowChartMultidocument => "FLOW_CHART_MULTIDOCUMENT",
                ShapeShapeType::FlowChartOfflineStorage => "FLOW_CHART_OFFLINE_STORAGE",
                ShapeShapeType::FlowChartOffpageConnector => "FLOW_CHART_OFFPAGE_CONNECTOR",
                ShapeShapeType::FlowChartOnlineStorage => "FLOW_CHART_ONLINE_STORAGE",
                ShapeShapeType::FlowChartOr => "FLOW_CHART_OR",
                ShapeShapeType::FlowChartPredefinedProcess => "FLOW_CHART_PREDEFINED_PROCESS",
                ShapeShapeType::FlowChartPreparation => "FLOW_CHART_PREPARATION",
                ShapeShapeType::FlowChartProcess => "FLOW_CHART_PROCESS",
                ShapeShapeType::FlowChartPunchedCard => "FLOW_CHART_PUNCHED_CARD",
                ShapeShapeType::FlowChartPunchedTape => "FLOW_CHART_PUNCHED_TAPE",
                ShapeShapeType::FlowChartSort => "FLOW_CHART_SORT",
                ShapeShapeType::FlowChartSummingJunction => "FLOW_CHART_SUMMING_JUNCTION",
                ShapeShapeType::FlowChartTerminator => "FLOW_CHART_TERMINATOR",
                ShapeShapeType::ArrowEast => "ARROW_EAST",
                ShapeShapeType::ArrowNorthEast => "ARROW_NORTH_EAST",
                ShapeShapeType::ArrowNorth => "ARROW_NORTH",
                ShapeShapeType::Speech => "SPEECH",
                ShapeShapeType::Starburst => "STARBURST",
                ShapeShapeType::Teardrop => "TEARDROP",
                ShapeShapeType::EllipseRibbon => "ELLIPSE_RIBBON",
                ShapeShapeType::EllipseRibbon2 => "ELLIPSE_RIBBON_2",
                ShapeShapeType::CloudCallout => "CLOUD_CALLOUT",
                ShapeShapeType::Custom => "CUSTOM",
            }
        }
    }
    impl ::std::fmt::Display for ShapeShapeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ShapeShapeType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ShapeShapeType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_UNSPECIFIED" => ShapeShapeType::TypeUnspecified,
                "TEXT_BOX" => ShapeShapeType::TextBox,
                "RECTANGLE" => ShapeShapeType::Rectangle,
                "ROUND_RECTANGLE" => ShapeShapeType::RoundRectangle,
                "ELLIPSE" => ShapeShapeType::Ellipse,
                "ARC" => ShapeShapeType::Arc,
                "BENT_ARROW" => ShapeShapeType::BentArrow,
                "BENT_UP_ARROW" => ShapeShapeType::BentUpArrow,
                "BEVEL" => ShapeShapeType::Bevel,
                "BLOCK_ARC" => ShapeShapeType::BlockArc,
                "BRACE_PAIR" => ShapeShapeType::BracePair,
                "BRACKET_PAIR" => ShapeShapeType::BracketPair,
                "CAN" => ShapeShapeType::Can,
                "CHEVRON" => ShapeShapeType::Chevron,
                "CHORD" => ShapeShapeType::Chord,
                "CLOUD" => ShapeShapeType::Cloud,
                "CORNER" => ShapeShapeType::Corner,
                "CUBE" => ShapeShapeType::Cube,
                "CURVED_DOWN_ARROW" => ShapeShapeType::CurvedDownArrow,
                "CURVED_LEFT_ARROW" => ShapeShapeType::CurvedLeftArrow,
                "CURVED_RIGHT_ARROW" => ShapeShapeType::CurvedRightArrow,
                "CURVED_UP_ARROW" => ShapeShapeType::CurvedUpArrow,
                "DECAGON" => ShapeShapeType::Decagon,
                "DIAGONAL_STRIPE" => ShapeShapeType::DiagonalStripe,
                "DIAMOND" => ShapeShapeType::Diamond,
                "DODECAGON" => ShapeShapeType::Dodecagon,
                "DONUT" => ShapeShapeType::Donut,
                "DOUBLE_WAVE" => ShapeShapeType::DoubleWave,
                "DOWN_ARROW" => ShapeShapeType::DownArrow,
                "DOWN_ARROW_CALLOUT" => ShapeShapeType::DownArrowCallout,
                "FOLDED_CORNER" => ShapeShapeType::FoldedCorner,
                "FRAME" => ShapeShapeType::Frame,
                "HALF_FRAME" => ShapeShapeType::HalfFrame,
                "HEART" => ShapeShapeType::Heart,
                "HEPTAGON" => ShapeShapeType::Heptagon,
                "HEXAGON" => ShapeShapeType::Hexagon,
                "HOME_PLATE" => ShapeShapeType::HomePlate,
                "HORIZONTAL_SCROLL" => ShapeShapeType::HorizontalScroll,
                "IRREGULAR_SEAL_1" => ShapeShapeType::IrregularSeal1,
                "IRREGULAR_SEAL_2" => ShapeShapeType::IrregularSeal2,
                "LEFT_ARROW" => ShapeShapeType::LeftArrow,
                "LEFT_ARROW_CALLOUT" => ShapeShapeType::LeftArrowCallout,
                "LEFT_BRACE" => ShapeShapeType::LeftBrace,
                "LEFT_BRACKET" => ShapeShapeType::LeftBracket,
                "LEFT_RIGHT_ARROW" => ShapeShapeType::LeftRightArrow,
                "LEFT_RIGHT_ARROW_CALLOUT" => ShapeShapeType::LeftRightArrowCallout,
                "LEFT_RIGHT_UP_ARROW" => ShapeShapeType::LeftRightUpArrow,
                "LEFT_UP_ARROW" => ShapeShapeType::LeftUpArrow,
                "LIGHTNING_BOLT" => ShapeShapeType::LightningBolt,
                "MATH_DIVIDE" => ShapeShapeType::MathDivide,
                "MATH_EQUAL" => ShapeShapeType::MathEqual,
                "MATH_MINUS" => ShapeShapeType::MathMinus,
                "MATH_MULTIPLY" => ShapeShapeType::MathMultiply,
                "MATH_NOT_EQUAL" => ShapeShapeType::MathNotEqual,
                "MATH_PLUS" => ShapeShapeType::MathPlus,
                "MOON" => ShapeShapeType::Moon,
                "NO_SMOKING" => ShapeShapeType::NoSmoking,
                "NOTCHED_RIGHT_ARROW" => ShapeShapeType::NotchedRightArrow,
                "OCTAGON" => ShapeShapeType::Octagon,
                "PARALLELOGRAM" => ShapeShapeType::Parallelogram,
                "PENTAGON" => ShapeShapeType::Pentagon,
                "PIE" => ShapeShapeType::Pie,
                "PLAQUE" => ShapeShapeType::Plaque,
                "PLUS" => ShapeShapeType::Plus,
                "QUAD_ARROW" => ShapeShapeType::QuadArrow,
                "QUAD_ARROW_CALLOUT" => ShapeShapeType::QuadArrowCallout,
                "RIBBON" => ShapeShapeType::Ribbon,
                "RIBBON_2" => ShapeShapeType::Ribbon2,
                "RIGHT_ARROW" => ShapeShapeType::RightArrow,
                "RIGHT_ARROW_CALLOUT" => ShapeShapeType::RightArrowCallout,
                "RIGHT_BRACE" => ShapeShapeType::RightBrace,
                "RIGHT_BRACKET" => ShapeShapeType::RightBracket,
                "ROUND_1_RECTANGLE" => ShapeShapeType::Round1Rectangle,
                "ROUND_2_DIAGONAL_RECTANGLE" => ShapeShapeType::Round2DiagonalRectangle,
                "ROUND_2_SAME_RECTANGLE" => ShapeShapeType::Round2SameRectangle,
                "RIGHT_TRIANGLE" => ShapeShapeType::RightTriangle,
                "SMILEY_FACE" => ShapeShapeType::SmileyFace,
                "SNIP_1_RECTANGLE" => ShapeShapeType::Snip1Rectangle,
                "SNIP_2_DIAGONAL_RECTANGLE" => ShapeShapeType::Snip2DiagonalRectangle,
                "SNIP_2_SAME_RECTANGLE" => ShapeShapeType::Snip2SameRectangle,
                "SNIP_ROUND_RECTANGLE" => ShapeShapeType::SnipRoundRectangle,
                "STAR_10" => ShapeShapeType::Star10,
                "STAR_12" => ShapeShapeType::Star12,
                "STAR_16" => ShapeShapeType::Star16,
                "STAR_24" => ShapeShapeType::Star24,
                "STAR_32" => ShapeShapeType::Star32,
                "STAR_4" => ShapeShapeType::Star4,
                "STAR_5" => ShapeShapeType::Star5,
                "STAR_6" => ShapeShapeType::Star6,
                "STAR_7" => ShapeShapeType::Star7,
                "STAR_8" => ShapeShapeType::Star8,
                "STRIPED_RIGHT_ARROW" => ShapeShapeType::StripedRightArrow,
                "SUN" => ShapeShapeType::Sun,
                "TRAPEZOID" => ShapeShapeType::Trapezoid,
                "TRIANGLE" => ShapeShapeType::Triangle,
                "UP_ARROW" => ShapeShapeType::UpArrow,
                "UP_ARROW_CALLOUT" => ShapeShapeType::UpArrowCallout,
                "UP_DOWN_ARROW" => ShapeShapeType::UpDownArrow,
                "UTURN_ARROW" => ShapeShapeType::UturnArrow,
                "VERTICAL_SCROLL" => ShapeShapeType::VerticalScroll,
                "WAVE" => ShapeShapeType::Wave,
                "WEDGE_ELLIPSE_CALLOUT" => ShapeShapeType::WedgeEllipseCallout,
                "WEDGE_RECTANGLE_CALLOUT" => ShapeShapeType::WedgeRectangleCallout,
                "WEDGE_ROUND_RECTANGLE_CALLOUT" => ShapeShapeType::WedgeRoundRectangleCallout,
                "FLOW_CHART_ALTERNATE_PROCESS" => ShapeShapeType::FlowChartAlternateProcess,
                "FLOW_CHART_COLLATE" => ShapeShapeType::FlowChartCollate,
                "FLOW_CHART_CONNECTOR" => ShapeShapeType::FlowChartConnector,
                "FLOW_CHART_DECISION" => ShapeShapeType::FlowChartDecision,
                "FLOW_CHART_DELAY" => ShapeShapeType::FlowChartDelay,
                "FLOW_CHART_DISPLAY" => ShapeShapeType::FlowChartDisplay,
                "FLOW_CHART_DOCUMENT" => ShapeShapeType::FlowChartDocument,
                "FLOW_CHART_EXTRACT" => ShapeShapeType::FlowChartExtract,
                "FLOW_CHART_INPUT_OUTPUT" => ShapeShapeType::FlowChartInputOutput,
                "FLOW_CHART_INTERNAL_STORAGE" => ShapeShapeType::FlowChartInternalStorage,
                "FLOW_CHART_MAGNETIC_DISK" => ShapeShapeType::FlowChartMagneticDisk,
                "FLOW_CHART_MAGNETIC_DRUM" => ShapeShapeType::FlowChartMagneticDrum,
                "FLOW_CHART_MAGNETIC_TAPE" => ShapeShapeType::FlowChartMagneticTape,
                "FLOW_CHART_MANUAL_INPUT" => ShapeShapeType::FlowChartManualInput,
                "FLOW_CHART_MANUAL_OPERATION" => ShapeShapeType::FlowChartManualOperation,
                "FLOW_CHART_MERGE" => ShapeShapeType::FlowChartMerge,
                "FLOW_CHART_MULTIDOCUMENT" => ShapeShapeType::FlowChartMultidocument,
                "FLOW_CHART_OFFLINE_STORAGE" => ShapeShapeType::FlowChartOfflineStorage,
                "FLOW_CHART_OFFPAGE_CONNECTOR" => ShapeShapeType::FlowChartOffpageConnector,
                "FLOW_CHART_ONLINE_STORAGE" => ShapeShapeType::FlowChartOnlineStorage,
                "FLOW_CHART_OR" => ShapeShapeType::FlowChartOr,
                "FLOW_CHART_PREDEFINED_PROCESS" => ShapeShapeType::FlowChartPredefinedProcess,
                "FLOW_CHART_PREPARATION" => ShapeShapeType::FlowChartPreparation,
                "FLOW_CHART_PROCESS" => ShapeShapeType::FlowChartProcess,
                "FLOW_CHART_PUNCHED_CARD" => ShapeShapeType::FlowChartPunchedCard,
                "FLOW_CHART_PUNCHED_TAPE" => ShapeShapeType::FlowChartPunchedTape,
                "FLOW_CHART_SORT" => ShapeShapeType::FlowChartSort,
                "FLOW_CHART_SUMMING_JUNCTION" => ShapeShapeType::FlowChartSummingJunction,
                "FLOW_CHART_TERMINATOR" => ShapeShapeType::FlowChartTerminator,
                "ARROW_EAST" => ShapeShapeType::ArrowEast,
                "ARROW_NORTH_EAST" => ShapeShapeType::ArrowNorthEast,
                "ARROW_NORTH" => ShapeShapeType::ArrowNorth,
                "SPEECH" => ShapeShapeType::Speech,
                "STARBURST" => ShapeShapeType::Starburst,
                "TEARDROP" => ShapeShapeType::Teardrop,
                "ELLIPSE_RIBBON" => ShapeShapeType::EllipseRibbon,
                "ELLIPSE_RIBBON_2" => ShapeShapeType::EllipseRibbon2,
                "CLOUD_CALLOUT" => ShapeShapeType::CloudCallout,
                "CUSTOM" => ShapeShapeType::Custom,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Shape {
        #[doc = "Placeholders are shapes that are inherit from corresponding placeholders on\nlayouts and masters.\n\nIf set, the shape is a placeholder shape and any inherited properties\ncan be resolved by looking at the parent placeholder identified by the\nPlaceholder.parent_object_id field."]
        #[serde(rename = "placeholder", default)]
        pub placeholder: Option<crate::schemas::Placeholder>,
        #[doc = "The properties of the shape."]
        #[serde(rename = "shapeProperties", default)]
        pub shape_properties: Option<crate::schemas::ShapeProperties>,
        #[doc = "The type of the shape."]
        #[serde(rename = "shapeType", default)]
        pub shape_type: Option<crate::schemas::ShapeShapeType>,
        #[doc = "The text content of the shape."]
        #[serde(rename = "text", default)]
        pub text: Option<crate::schemas::TextContent>,
    }
    impl ::field_selector::FieldSelector for Shape {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ShapeBackgroundFillPropertyState {
        #[doc = "If a property's state is RENDERED, then the element has the corresponding\nproperty when rendered on a page. If the element is a placeholder shape as\ndetermined by the placeholder\nfield, and it inherits from a placeholder shape, the corresponding field\nmay be unset, meaning that the property value is inherited from a parent\nplaceholder. If the element does not inherit, then the field will contain\nthe rendered value. This is the default value."]
        Rendered,
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the\ncorresponding property when rendered on a page. However, the field may\nstill be set so it can be inherited by child shapes. To remove a property\nfrom a rendered element, set its property_state to NOT_RENDERED."]
        NotRendered,
        #[doc = "If a property's state is INHERIT, then the property state uses the value of\ncorresponding `property_state` field on the parent shape. Elements that do\nnot inherit will never have an INHERIT property state."]
        Inherit,
    }
    impl ShapeBackgroundFillPropertyState {
        pub fn as_str(self) -> &'static str {
            match self {
                ShapeBackgroundFillPropertyState::Rendered => "RENDERED",
                ShapeBackgroundFillPropertyState::NotRendered => "NOT_RENDERED",
                ShapeBackgroundFillPropertyState::Inherit => "INHERIT",
            }
        }
    }
    impl ::std::fmt::Display for ShapeBackgroundFillPropertyState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ShapeBackgroundFillPropertyState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ShapeBackgroundFillPropertyState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "RENDERED" => ShapeBackgroundFillPropertyState::Rendered,
                "NOT_RENDERED" => ShapeBackgroundFillPropertyState::NotRendered,
                "INHERIT" => ShapeBackgroundFillPropertyState::Inherit,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ShapeBackgroundFill {
        #[doc = "The background fill property state.\n\nUpdating the fill on a shape will implicitly update this field to\n`RENDERED`, unless another value is specified in the same request. To\nhave no fill on a shape, set this field to `NOT_RENDERED`. In this case,\nany other fill fields set in the same request will be ignored."]
        #[serde(rename = "propertyState", default)]
        pub property_state: Option<crate::schemas::ShapeBackgroundFillPropertyState>,
        #[doc = "Solid color fill."]
        #[serde(rename = "solidFill", default)]
        pub solid_fill: Option<crate::schemas::SolidFill>,
    }
    impl ::field_selector::FieldSelector for ShapeBackgroundFill {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ShapePropertiesContentAlignment {
        #[doc = "An unspecified content alignment. The content alignment is inherited from\nthe parent if it exists."]
        ContentAlignmentUnspecified,
        #[doc = "An unsupported content alignment."]
        ContentAlignmentUnsupported,
        #[doc = "An alignment that aligns the content to the top of the content holder.\nCorresponds to ECMA-376 ST_TextAnchoringType 't'."]
        Top,
        #[doc = "An alignment that aligns the content to the middle of the content\nholder. Corresponds to ECMA-376 ST_TextAnchoringType 'ctr'."]
        Middle,
        #[doc = "An alignment that aligns the content to the bottom of the content\nholder. Corresponds to ECMA-376 ST_TextAnchoringType 'b'."]
        Bottom,
    }
    impl ShapePropertiesContentAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                ShapePropertiesContentAlignment::ContentAlignmentUnspecified => {
                    "CONTENT_ALIGNMENT_UNSPECIFIED"
                }
                ShapePropertiesContentAlignment::ContentAlignmentUnsupported => {
                    "CONTENT_ALIGNMENT_UNSUPPORTED"
                }
                ShapePropertiesContentAlignment::Top => "TOP",
                ShapePropertiesContentAlignment::Middle => "MIDDLE",
                ShapePropertiesContentAlignment::Bottom => "BOTTOM",
            }
        }
    }
    impl ::std::fmt::Display for ShapePropertiesContentAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ShapePropertiesContentAlignment {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ShapePropertiesContentAlignment {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTENT_ALIGNMENT_UNSPECIFIED" => {
                    ShapePropertiesContentAlignment::ContentAlignmentUnspecified
                }
                "CONTENT_ALIGNMENT_UNSUPPORTED" => {
                    ShapePropertiesContentAlignment::ContentAlignmentUnsupported
                }
                "TOP" => ShapePropertiesContentAlignment::Top,
                "MIDDLE" => ShapePropertiesContentAlignment::Middle,
                "BOTTOM" => ShapePropertiesContentAlignment::Bottom,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ShapeProperties {
        #[doc = "The alignment of the content in the shape. If unspecified,\nthe alignment is inherited from a parent placeholder if it exists. If the\nshape has no parent, the default alignment matches the alignment for new\nshapes created in the Slides editor."]
        #[serde(rename = "contentAlignment", default)]
        pub content_alignment: Option<crate::schemas::ShapePropertiesContentAlignment>,
        #[doc = "The hyperlink destination of the shape. If unset, there is no link. Links\nare not inherited from parent placeholders."]
        #[serde(rename = "link", default)]
        pub link: Option<crate::schemas::Link>,
        #[doc = "The outline of the shape. If unset, the outline is inherited from a\nparent placeholder if it exists. If the shape has no parent, then the\ndefault outline depends on the shape type, matching the defaults for\nnew shapes created in the Slides editor."]
        #[serde(rename = "outline", default)]
        pub outline: Option<crate::schemas::Outline>,
        #[doc = "The shadow properties of the shape. If unset, the shadow is inherited from\na parent placeholder if it exists. If the shape has no parent, then the\ndefault shadow matches the defaults for new shapes created in the Slides\neditor. This property is read-only."]
        #[serde(rename = "shadow", default)]
        pub shadow: Option<crate::schemas::Shadow>,
        #[doc = "The background fill of the shape. If unset, the background fill is\ninherited from a parent placeholder if it exists. If the shape has no\nparent, then the default background fill depends on the shape type,\nmatching the defaults for new shapes created in the Slides editor."]
        #[serde(rename = "shapeBackgroundFill", default)]
        pub shape_background_fill: Option<crate::schemas::ShapeBackgroundFill>,
    }
    impl ::field_selector::FieldSelector for ShapeProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SheetsChart {
        #[doc = "The ID of the specific chart in the Google Sheets spreadsheet that is\nembedded."]
        #[serde(rename = "chartId", default)]
        pub chart_id: Option<i32>,
        #[doc = "The URL of an image of the embedded chart, with a default lifetime of 30\nminutes. This URL is tagged with the account of the requester. Anyone with\nthe URL effectively accesses the image as the original requester. Access to\nthe image may be lost if the presentation's sharing settings change."]
        #[serde(rename = "contentUrl", default)]
        pub content_url: Option<String>,
        #[doc = "The properties of the Sheets chart."]
        #[serde(rename = "sheetsChartProperties", default)]
        pub sheets_chart_properties: Option<crate::schemas::SheetsChartProperties>,
        #[doc = "The ID of the Google Sheets spreadsheet that contains the source chart."]
        #[serde(rename = "spreadsheetId", default)]
        pub spreadsheet_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for SheetsChart {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SheetsChartProperties {
        #[doc = "The properties of the embedded chart image."]
        #[serde(rename = "chartImageProperties", default)]
        pub chart_image_properties: Option<crate::schemas::ImageProperties>,
    }
    impl ::field_selector::FieldSelector for SheetsChartProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Size {
        #[doc = "The height of the object."]
        #[serde(rename = "height", default)]
        pub height: Option<crate::schemas::Dimension>,
        #[doc = "The width of the object."]
        #[serde(rename = "width", default)]
        pub width: Option<crate::schemas::Dimension>,
    }
    impl ::field_selector::FieldSelector for Size {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SlideProperties {
        #[doc = "The object ID of the layout that this slide is based on. This property is\nread-only."]
        #[serde(rename = "layoutObjectId", default)]
        pub layout_object_id: Option<String>,
        #[doc = "The object ID of the master that this slide is based on. This property is\nread-only."]
        #[serde(rename = "masterObjectId", default)]
        pub master_object_id: Option<String>,
        #[doc = "The notes page that this slide is associated with. It defines the visual\nappearance of a notes page when printing or exporting slides with speaker\nnotes. A notes page inherits properties from the\nnotes master.\nThe placeholder shape with type BODY on the notes page contains the speaker\nnotes for this slide. The ID of this shape is identified by the\nspeakerNotesObjectId field.\nThe notes page is read-only except for the text content and styles of the\nspeaker notes shape. This property is read-only."]
        #[serde(rename = "notesPage", default)]
        pub notes_page: Option<crate::schemas::Page>,
    }
    impl ::field_selector::FieldSelector for SlideProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SolidFill {
        #[doc = "The fraction of this `color` that should be applied to the pixel.\nThat is, the final pixel color is defined by the equation:\n\n  pixel color = alpha * (color) + (1.0 - alpha) * (background color)\n\nThis means that a value of 1.0 corresponds to a solid color, whereas\na value of 0.0 corresponds to a completely transparent color."]
        #[serde(rename = "alpha", default)]
        pub alpha: Option<f32>,
        #[doc = "The color value of the solid fill."]
        #[serde(rename = "color", default)]
        pub color: Option<crate::schemas::OpaqueColor>,
    }
    impl ::field_selector::FieldSelector for SolidFill {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct StretchedPictureFill {
        #[doc = "Reading the content_url:\n\nAn URL to a picture with a default lifetime of 30 minutes.\nThis URL is tagged with the account of the requester. Anyone with the URL\neffectively accesses the picture as the original requester. Access to the\npicture may be lost if the presentation's sharing settings change.\n\nWriting the content_url:\n\nThe picture is fetched once at insertion time and a copy is stored for\ndisplay inside the presentation. Pictures must be less than 50MB in size,\ncannot exceed 25 megapixels, and must be in one of PNG, JPEG, or GIF\nformat.\n\nThe provided URL can be at most 2 kB in length."]
        #[serde(rename = "contentUrl", default)]
        pub content_url: Option<String>,
        #[doc = "The original size of the picture fill. This field is read-only."]
        #[serde(rename = "size", default)]
        pub size: Option<crate::schemas::Size>,
    }
    impl ::field_selector::FieldSelector for StretchedPictureFill {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SubstringMatchCriteria {
        #[doc = "Indicates whether the search should respect case:\n\n- `True`: the search is case sensitive.\n- `False`: the search is case insensitive."]
        #[serde(rename = "matchCase", default)]
        pub match_case: Option<bool>,
        #[doc = "The text to search for in the shape or table."]
        #[serde(rename = "text", default)]
        pub text: Option<String>,
    }
    impl ::field_selector::FieldSelector for SubstringMatchCriteria {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Table {
        #[doc = "Number of columns in the table."]
        #[serde(rename = "columns", default)]
        pub columns: Option<i32>,
        #[doc = "Properties of horizontal cell borders.\n\nA table's horizontal cell borders are represented as a grid. The grid has\none more row than the number of rows in the table and the same number of\ncolumns as the table. For example, if the table is 3 x 3, its horizontal\nborders will be represented as a grid with 4 rows and 3 columns."]
        #[serde(rename = "horizontalBorderRows", default)]
        pub horizontal_border_rows: Option<Vec<crate::schemas::TableBorderRow>>,
        #[doc = "Number of rows in the table."]
        #[serde(rename = "rows", default)]
        pub rows: Option<i32>,
        #[doc = "Properties of each column."]
        #[serde(rename = "tableColumns", default)]
        pub table_columns: Option<Vec<crate::schemas::TableColumnProperties>>,
        #[doc = "Properties and contents of each row.\n\nCells that span multiple rows are contained in only one of these rows and\nhave a row_span greater\nthan 1."]
        #[serde(rename = "tableRows", default)]
        pub table_rows: Option<Vec<crate::schemas::TableRow>>,
        #[doc = "Properties of vertical cell borders.\n\nA table's vertical cell borders are represented as a grid. The grid has the\nsame number of rows as the table and one more column than the number of\ncolumns in the table. For example, if the table is 3 x 3, its vertical\nborders will be represented as a grid with 3 rows and 4 columns."]
        #[serde(rename = "verticalBorderRows", default)]
        pub vertical_border_rows: Option<Vec<crate::schemas::TableBorderRow>>,
    }
    impl ::field_selector::FieldSelector for Table {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableBorderCell {
        #[doc = "The location of the border within the border table."]
        #[serde(rename = "location", default)]
        pub location: Option<crate::schemas::TableCellLocation>,
        #[doc = "The border properties."]
        #[serde(rename = "tableBorderProperties", default)]
        pub table_border_properties: Option<crate::schemas::TableBorderProperties>,
    }
    impl ::field_selector::FieldSelector for TableBorderCell {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableBorderFill {
        #[doc = "Solid fill."]
        #[serde(rename = "solidFill", default)]
        pub solid_fill: Option<crate::schemas::SolidFill>,
    }
    impl ::field_selector::FieldSelector for TableBorderFill {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TableBorderPropertiesDashStyle {
        #[doc = "Unspecified dash style."]
        DashStyleUnspecified,
        #[doc = "Solid line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'solid'.\nThis is the default dash style."]
        Solid,
        #[doc = "Dotted line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dot'."]
        Dot,
        #[doc = "Dashed line. Corresponds to ECMA-376 ST_PresetLineDashVal value 'dash'."]
        Dash,
        #[doc = "Alternating dashes and dots. Corresponds to ECMA-376 ST_PresetLineDashVal\nvalue 'dashDot'."]
        DashDot,
        #[doc = "Line with large dashes. Corresponds to ECMA-376 ST_PresetLineDashVal\nvalue 'lgDash'."]
        LongDash,
        #[doc = "Alternating large dashes and dots. Corresponds to ECMA-376\nST_PresetLineDashVal value 'lgDashDot'."]
        LongDashDot,
    }
    impl TableBorderPropertiesDashStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                TableBorderPropertiesDashStyle::DashStyleUnspecified => "DASH_STYLE_UNSPECIFIED",
                TableBorderPropertiesDashStyle::Solid => "SOLID",
                TableBorderPropertiesDashStyle::Dot => "DOT",
                TableBorderPropertiesDashStyle::Dash => "DASH",
                TableBorderPropertiesDashStyle::DashDot => "DASH_DOT",
                TableBorderPropertiesDashStyle::LongDash => "LONG_DASH",
                TableBorderPropertiesDashStyle::LongDashDot => "LONG_DASH_DOT",
            }
        }
    }
    impl ::std::fmt::Display for TableBorderPropertiesDashStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TableBorderPropertiesDashStyle {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TableBorderPropertiesDashStyle {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DASH_STYLE_UNSPECIFIED" => TableBorderPropertiesDashStyle::DashStyleUnspecified,
                "SOLID" => TableBorderPropertiesDashStyle::Solid,
                "DOT" => TableBorderPropertiesDashStyle::Dot,
                "DASH" => TableBorderPropertiesDashStyle::Dash,
                "DASH_DOT" => TableBorderPropertiesDashStyle::DashDot,
                "LONG_DASH" => TableBorderPropertiesDashStyle::LongDash,
                "LONG_DASH_DOT" => TableBorderPropertiesDashStyle::LongDashDot,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableBorderProperties {
        #[doc = "The dash style of the border."]
        #[serde(rename = "dashStyle", default)]
        pub dash_style: Option<crate::schemas::TableBorderPropertiesDashStyle>,
        #[doc = "The fill of the table border."]
        #[serde(rename = "tableBorderFill", default)]
        pub table_border_fill: Option<crate::schemas::TableBorderFill>,
        #[doc = "The thickness of the border."]
        #[serde(rename = "weight", default)]
        pub weight: Option<crate::schemas::Dimension>,
    }
    impl ::field_selector::FieldSelector for TableBorderProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableBorderRow {
        #[doc = "Properties of each border cell. When a border's adjacent table cells are\nmerged, it is not included in the response."]
        #[serde(rename = "tableBorderCells", default)]
        pub table_border_cells: Option<Vec<crate::schemas::TableBorderCell>>,
    }
    impl ::field_selector::FieldSelector for TableBorderRow {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableCell {
        #[doc = "Column span of the cell."]
        #[serde(rename = "columnSpan", default)]
        pub column_span: Option<i32>,
        #[doc = "The location of the cell within the table."]
        #[serde(rename = "location", default)]
        pub location: Option<crate::schemas::TableCellLocation>,
        #[doc = "Row span of the cell."]
        #[serde(rename = "rowSpan", default)]
        pub row_span: Option<i32>,
        #[doc = "The properties of the table cell."]
        #[serde(rename = "tableCellProperties", default)]
        pub table_cell_properties: Option<crate::schemas::TableCellProperties>,
        #[doc = "The text content of the cell."]
        #[serde(rename = "text", default)]
        pub text: Option<crate::schemas::TextContent>,
    }
    impl ::field_selector::FieldSelector for TableCell {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TableCellBackgroundFillPropertyState {
        #[doc = "If a property's state is RENDERED, then the element has the corresponding\nproperty when rendered on a page. If the element is a placeholder shape as\ndetermined by the placeholder\nfield, and it inherits from a placeholder shape, the corresponding field\nmay be unset, meaning that the property value is inherited from a parent\nplaceholder. If the element does not inherit, then the field will contain\nthe rendered value. This is the default value."]
        Rendered,
        #[doc = "If a property's state is NOT_RENDERED, then the element does not have the\ncorresponding property when rendered on a page. However, the field may\nstill be set so it can be inherited by child shapes. To remove a property\nfrom a rendered element, set its property_state to NOT_RENDERED."]
        NotRendered,
        #[doc = "If a property's state is INHERIT, then the property state uses the value of\ncorresponding `property_state` field on the parent shape. Elements that do\nnot inherit will never have an INHERIT property state."]
        Inherit,
    }
    impl TableCellBackgroundFillPropertyState {
        pub fn as_str(self) -> &'static str {
            match self {
                TableCellBackgroundFillPropertyState::Rendered => "RENDERED",
                TableCellBackgroundFillPropertyState::NotRendered => "NOT_RENDERED",
                TableCellBackgroundFillPropertyState::Inherit => "INHERIT",
            }
        }
    }
    impl ::std::fmt::Display for TableCellBackgroundFillPropertyState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TableCellBackgroundFillPropertyState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TableCellBackgroundFillPropertyState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "RENDERED" => TableCellBackgroundFillPropertyState::Rendered,
                "NOT_RENDERED" => TableCellBackgroundFillPropertyState::NotRendered,
                "INHERIT" => TableCellBackgroundFillPropertyState::Inherit,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableCellBackgroundFill {
        #[doc = "The background fill property state.\n\nUpdating the fill on a table cell will implicitly update this field\nto `RENDERED`, unless another value is specified in the same request. To\nhave no fill on a table cell, set this field to `NOT_RENDERED`. In this\ncase, any other fill fields set in the same request will be ignored."]
        #[serde(rename = "propertyState", default)]
        pub property_state: Option<crate::schemas::TableCellBackgroundFillPropertyState>,
        #[doc = "Solid color fill."]
        #[serde(rename = "solidFill", default)]
        pub solid_fill: Option<crate::schemas::SolidFill>,
    }
    impl ::field_selector::FieldSelector for TableCellBackgroundFill {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TableCellLocation {
        #[doc = "The 0-based column index."]
        #[serde(rename = "columnIndex", default)]
        pub column_index: Option<i32>,
        #[doc = "The 0-based row index."]
        #[serde(rename = "rowIndex", default)]
        pub row_index: Option<i32>,
    }
    impl ::field_selector::FieldSelector for TableCellLocation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TableCellPropertiesContentAlignment {
        #[doc = "An unspecified content alignment. The content alignment is inherited from\nthe parent if it exists."]
        ContentAlignmentUnspecified,
        #[doc = "An unsupported content alignment."]
        ContentAlignmentUnsupported,
        #[doc = "An alignment that aligns the content to the top of the content holder.\nCorresponds to ECMA-376 ST_TextAnchoringType 't'."]
        Top,
        #[doc = "An alignment that aligns the content to the middle of the content\nholder. Corresponds to ECMA-376 ST_TextAnchoringType 'ctr'."]
        Middle,
        #[doc = "An alignment that aligns the content to the bottom of the content\nholder. Corresponds to ECMA-376 ST_TextAnchoringType 'b'."]
        Bottom,
    }
    impl TableCellPropertiesContentAlignment {
        pub fn as_str(self) -> &'static str {
            match self {
                TableCellPropertiesContentAlignment::ContentAlignmentUnspecified => {
                    "CONTENT_ALIGNMENT_UNSPECIFIED"
                }
                TableCellPropertiesContentAlignment::ContentAlignmentUnsupported => {
                    "CONTENT_ALIGNMENT_UNSUPPORTED"
                }
                TableCellPropertiesContentAlignment::Top => "TOP",
                TableCellPropertiesContentAlignment::Middle => "MIDDLE",
                TableCellPropertiesContentAlignment::Bottom => "BOTTOM",
            }
        }
    }
    impl ::std::fmt::Display for TableCellPropertiesContentAlignment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TableCellPropertiesContentAlignment {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TableCellPropertiesContentAlignment {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTENT_ALIGNMENT_UNSPECIFIED" => {
                    TableCellPropertiesContentAlignment::ContentAlignmentUnspecified
                }
                "CONTENT_ALIGNMENT_UNSUPPORTED" => {
                    TableCellPropertiesContentAlignment::ContentAlignmentUnsupported
                }
                "TOP" => TableCellPropertiesContentAlignment::Top,
                "MIDDLE" => TableCellPropertiesContentAlignment::Middle,
                "BOTTOM" => TableCellPropertiesContentAlignment::Bottom,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableCellProperties {
        #[doc = "The alignment of the content in the table cell. The default alignment\nmatches the alignment for newly created table cells in the Slides editor."]
        #[serde(rename = "contentAlignment", default)]
        pub content_alignment: Option<crate::schemas::TableCellPropertiesContentAlignment>,
        #[doc = "The background fill of the table cell. The default fill matches the fill\nfor newly created table cells in the Slides editor."]
        #[serde(rename = "tableCellBackgroundFill", default)]
        pub table_cell_background_fill: Option<crate::schemas::TableCellBackgroundFill>,
    }
    impl ::field_selector::FieldSelector for TableCellProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableColumnProperties {
        #[doc = "Width of a column."]
        #[serde(rename = "columnWidth", default)]
        pub column_width: Option<crate::schemas::Dimension>,
    }
    impl ::field_selector::FieldSelector for TableColumnProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TableRange {
        #[doc = "The column span of the table range."]
        #[serde(rename = "columnSpan", default)]
        pub column_span: Option<i32>,
        #[doc = "The starting location of the table range."]
        #[serde(rename = "location", default)]
        pub location: Option<crate::schemas::TableCellLocation>,
        #[doc = "The row span of the table range."]
        #[serde(rename = "rowSpan", default)]
        pub row_span: Option<i32>,
    }
    impl ::field_selector::FieldSelector for TableRange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableRow {
        #[doc = "Height of a row."]
        #[serde(rename = "rowHeight", default)]
        pub row_height: Option<crate::schemas::Dimension>,
        #[doc = "Properties and contents of each cell.\n\nCells that span multiple columns are represented only once with a\ncolumn_span greater\nthan 1. As a result, the length of this collection does not always match\nthe number of columns of the entire table."]
        #[serde(rename = "tableCells", default)]
        pub table_cells: Option<Vec<crate::schemas::TableCell>>,
        #[doc = "Properties of the row."]
        #[serde(rename = "tableRowProperties", default)]
        pub table_row_properties: Option<crate::schemas::TableRowProperties>,
    }
    impl ::field_selector::FieldSelector for TableRow {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TableRowProperties {
        #[doc = "Minimum height of the row. The row will be rendered in the Slides editor at\na height equal to or greater than this value in order to show all the text\nin the row's cell(s)."]
        #[serde(rename = "minRowHeight", default)]
        pub min_row_height: Option<crate::schemas::Dimension>,
    }
    impl ::field_selector::FieldSelector for TableRowProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TextContent {
        #[doc = "The bulleted lists contained in this text, keyed by list ID."]
        #[serde(rename = "lists", default)]
        pub lists: Option<::std::collections::BTreeMap<String, crate::schemas::List>>,
        #[doc = "The text contents broken down into its component parts, including styling\ninformation. This property is read-only."]
        #[serde(rename = "textElements", default)]
        pub text_elements: Option<Vec<crate::schemas::TextElement>>,
    }
    impl ::field_selector::FieldSelector for TextContent {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TextElement {
        #[doc = "A TextElement representing a spot in the text that is dynamically\nreplaced with content that can change over time."]
        #[serde(rename = "autoText", default)]
        pub auto_text: Option<crate::schemas::AutoText>,
        #[doc = "The zero-based end index of this text element, exclusive, in Unicode code\nunits."]
        #[serde(rename = "endIndex", default)]
        pub end_index: Option<i32>,
        #[doc = "A marker representing the beginning of a new paragraph.\n\nThe `start_index` and `end_index` of this TextElement represent the\nrange of the paragraph. Other TextElements with an index range contained\ninside this paragraph's range are considered to be part of this\nparagraph. The range of indices of two separate paragraphs will never\noverlap."]
        #[serde(rename = "paragraphMarker", default)]
        pub paragraph_marker: Option<crate::schemas::ParagraphMarker>,
        #[doc = "The zero-based start index of this text element, in Unicode code units."]
        #[serde(rename = "startIndex", default)]
        pub start_index: Option<i32>,
        #[doc = "A TextElement representing a run of text where all of the characters\nin the run have the same TextStyle.\n\nThe `start_index` and `end_index` of TextRuns will always be fully\ncontained in the index range of a single `paragraph_marker` TextElement.\nIn other words, a TextRun will never span multiple paragraphs."]
        #[serde(rename = "textRun", default)]
        pub text_run: Option<crate::schemas::TextRun>,
    }
    impl ::field_selector::FieldSelector for TextElement {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TextRun {
        #[doc = "The text of this run."]
        #[serde(rename = "content", default)]
        pub content: Option<String>,
        #[doc = "The styling applied to this run."]
        #[serde(rename = "style", default)]
        pub style: Option<crate::schemas::TextStyle>,
    }
    impl ::field_selector::FieldSelector for TextRun {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TextStyleBaselineOffset {
        #[doc = "The text's baseline offset is inherited from the parent."]
        BaselineOffsetUnspecified,
        #[doc = "The text is not vertically offset."]
        None,
        #[doc = "The text is vertically offset upwards (superscript)."]
        Superscript,
        #[doc = "The text is vertically offset downwards (subscript)."]
        Subscript,
    }
    impl TextStyleBaselineOffset {
        pub fn as_str(self) -> &'static str {
            match self {
                TextStyleBaselineOffset::BaselineOffsetUnspecified => "BASELINE_OFFSET_UNSPECIFIED",
                TextStyleBaselineOffset::None => "NONE",
                TextStyleBaselineOffset::Superscript => "SUPERSCRIPT",
                TextStyleBaselineOffset::Subscript => "SUBSCRIPT",
            }
        }
    }
    impl ::std::fmt::Display for TextStyleBaselineOffset {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TextStyleBaselineOffset {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TextStyleBaselineOffset {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BASELINE_OFFSET_UNSPECIFIED" => TextStyleBaselineOffset::BaselineOffsetUnspecified,
                "NONE" => TextStyleBaselineOffset::None,
                "SUPERSCRIPT" => TextStyleBaselineOffset::Superscript,
                "SUBSCRIPT" => TextStyleBaselineOffset::Subscript,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct TextStyle {
        #[doc = "The background color of the text. If set, the color is either opaque or\ntransparent, depending on if the `opaque_color` field in it is set."]
        #[serde(rename = "backgroundColor", default)]
        pub background_color: Option<crate::schemas::OptionalColor>,
        #[doc = "The text's vertical offset from its normal position.\n\nText with `SUPERSCRIPT` or `SUBSCRIPT` baseline offsets is automatically\nrendered in a smaller font size, computed based on the `font_size` field.\nThe `font_size` itself is not affected by changes in this field."]
        #[serde(rename = "baselineOffset", default)]
        pub baseline_offset: Option<crate::schemas::TextStyleBaselineOffset>,
        #[doc = "Whether or not the text is rendered as bold."]
        #[serde(rename = "bold", default)]
        pub bold: Option<bool>,
        #[doc = "The font family of the text.\n\nThe font family can be any font from the Font menu in Slides or from\n[Google Fonts] (https://fonts.google.com/). If the font name is\nunrecognized, the text is rendered in `Arial`.\n\nSome fonts can affect the weight of the text. If an update request\nspecifies values for both `font_family` and `bold`, the explicitly-set\n`bold` value is used."]
        #[serde(rename = "fontFamily", default)]
        pub font_family: Option<String>,
        #[doc = "The size of the text's font. When read, the `font_size` will specified in\npoints."]
        #[serde(rename = "fontSize", default)]
        pub font_size: Option<crate::schemas::Dimension>,
        #[doc = "The color of the text itself. If set, the color is either opaque or\ntransparent, depending on if the `opaque_color` field in it is set."]
        #[serde(rename = "foregroundColor", default)]
        pub foreground_color: Option<crate::schemas::OptionalColor>,
        #[doc = "Whether or not the text is italicized."]
        #[serde(rename = "italic", default)]
        pub italic: Option<bool>,
        #[doc = "The hyperlink destination of the text. If unset, there is no link. Links\nare not inherited from parent text.\n\nChanging the link in an update request causes some other changes to the\ntext style of the range:\n\n* When setting a link, the text foreground color will be set to\n  ThemeColorType.HYPERLINK and the text will\n  be underlined. If these fields are modified in the same\n  request, those values will be used instead of the link defaults.\n* Setting a link on a text range that overlaps with an existing link will\n  also update the existing link to point to the new URL.\n* Links are not settable on newline characters. As a result, setting a link\n  on a text range that crosses a paragraph boundary, such as `\"ABC\\n123\"`,\n  will separate the newline character(s) into their own text runs. The\n  link will be applied separately to the runs before and after the newline.\n* Removing a link will update the text style of the range to match the\n  style of the preceding text (or the default text styles if the preceding\n  text is another link) unless different styles are being set in the same\n  request."]
        #[serde(rename = "link", default)]
        pub link: Option<crate::schemas::Link>,
        #[doc = "Whether or not the text is in small capital letters."]
        #[serde(rename = "smallCaps", default)]
        pub small_caps: Option<bool>,
        #[doc = "Whether or not the text is struck through."]
        #[serde(rename = "strikethrough", default)]
        pub strikethrough: Option<bool>,
        #[doc = "Whether or not the text is underlined."]
        #[serde(rename = "underline", default)]
        pub underline: Option<bool>,
        #[doc = "The font family and rendered weight of the text.\n\nThis field is an extension of `font_family` meant to support explicit font\nweights without breaking backwards compatibility. As such, when reading the\nstyle of a range of text, the value of `weighted_font_family#font_family`\nwill always be equal to that of `font_family`. However, when writing, if\nboth fields are included in the field mask (either explicitly or through\nthe wildcard `\"*\"`), their values are reconciled as follows:\n\n* If `font_family` is set and `weighted_font_family` is not, the value of\n  `font_family` is applied with weight `400` (\"normal\").\n* If both fields are set, the value of `font_family` must match that of\n  `weighted_font_family#font_family`. If so, the font family and weight of\n  `weighted_font_family` is applied. Otherwise, a 400 bad request error is\n  returned.\n* If `weighted_font_family` is set and `font_family` is not, the font\n  family and weight of `weighted_font_family` is applied.\n* If neither field is set, the font family and weight of the text inherit\n  from the parent. Note that these properties cannot inherit separately\n  from each other.\n\nIf an update request specifies values for both `weighted_font_family` and\n`bold`, the `weighted_font_family` is applied first, then `bold`.\n\nIf `weighted_font_family#weight` is not set, it defaults to `400`.\n\nIf `weighted_font_family` is set, then `weighted_font_family#font_family`\nmust also be set with a non-empty value. Otherwise, a 400 bad request error\nis returned."]
        #[serde(rename = "weightedFontFamily", default)]
        pub weighted_font_family: Option<crate::schemas::WeightedFontFamily>,
    }
    impl ::field_selector::FieldSelector for TextStyle {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ThemeColorPairType {
        #[doc = "Unspecified theme color. This value should not be used."]
        ThemeColorTypeUnspecified,
        #[doc = "Represents the first dark color."]
        Dark1,
        #[doc = "Represents the first light color."]
        Light1,
        #[doc = "Represents the second dark color."]
        Dark2,
        #[doc = "Represents the second light color."]
        Light2,
        #[doc = "Represents the first accent color."]
        Accent1,
        #[doc = "Represents the second accent color."]
        Accent2,
        #[doc = "Represents the third accent color."]
        Accent3,
        #[doc = "Represents the fourth accent color."]
        Accent4,
        #[doc = "Represents the fifth accent color."]
        Accent5,
        #[doc = "Represents the sixth accent color."]
        Accent6,
        #[doc = "Represents the color to use for hyperlinks."]
        Hyperlink,
        #[doc = "Represents the color to use for visited hyperlinks."]
        FollowedHyperlink,
        #[doc = "Represents the first text color."]
        Text1,
        #[doc = "Represents the first background color."]
        Background1,
        #[doc = "Represents the second text color."]
        Text2,
        #[doc = "Represents the second background color."]
        Background2,
    }
    impl ThemeColorPairType {
        pub fn as_str(self) -> &'static str {
            match self {
                ThemeColorPairType::ThemeColorTypeUnspecified => "THEME_COLOR_TYPE_UNSPECIFIED",
                ThemeColorPairType::Dark1 => "DARK1",
                ThemeColorPairType::Light1 => "LIGHT1",
                ThemeColorPairType::Dark2 => "DARK2",
                ThemeColorPairType::Light2 => "LIGHT2",
                ThemeColorPairType::Accent1 => "ACCENT1",
                ThemeColorPairType::Accent2 => "ACCENT2",
                ThemeColorPairType::Accent3 => "ACCENT3",
                ThemeColorPairType::Accent4 => "ACCENT4",
                ThemeColorPairType::Accent5 => "ACCENT5",
                ThemeColorPairType::Accent6 => "ACCENT6",
                ThemeColorPairType::Hyperlink => "HYPERLINK",
                ThemeColorPairType::FollowedHyperlink => "FOLLOWED_HYPERLINK",
                ThemeColorPairType::Text1 => "TEXT1",
                ThemeColorPairType::Background1 => "BACKGROUND1",
                ThemeColorPairType::Text2 => "TEXT2",
                ThemeColorPairType::Background2 => "BACKGROUND2",
            }
        }
    }
    impl ::std::fmt::Display for ThemeColorPairType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ThemeColorPairType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ThemeColorPairType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "THEME_COLOR_TYPE_UNSPECIFIED" => ThemeColorPairType::ThemeColorTypeUnspecified,
                "DARK1" => ThemeColorPairType::Dark1,
                "LIGHT1" => ThemeColorPairType::Light1,
                "DARK2" => ThemeColorPairType::Dark2,
                "LIGHT2" => ThemeColorPairType::Light2,
                "ACCENT1" => ThemeColorPairType::Accent1,
                "ACCENT2" => ThemeColorPairType::Accent2,
                "ACCENT3" => ThemeColorPairType::Accent3,
                "ACCENT4" => ThemeColorPairType::Accent4,
                "ACCENT5" => ThemeColorPairType::Accent5,
                "ACCENT6" => ThemeColorPairType::Accent6,
                "HYPERLINK" => ThemeColorPairType::Hyperlink,
                "FOLLOWED_HYPERLINK" => ThemeColorPairType::FollowedHyperlink,
                "TEXT1" => ThemeColorPairType::Text1,
                "BACKGROUND1" => ThemeColorPairType::Background1,
                "TEXT2" => ThemeColorPairType::Text2,
                "BACKGROUND2" => ThemeColorPairType::Background2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ThemeColorPair {
        #[doc = "The concrete color corresponding to the theme color type above."]
        #[serde(rename = "color", default)]
        pub color: Option<crate::schemas::RgbColor>,
        #[doc = "The type of the theme color."]
        #[serde(rename = "type", default)]
        pub r#type: Option<crate::schemas::ThemeColorPairType>,
    }
    impl ::field_selector::FieldSelector for ThemeColorPair {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Thumbnail {
        #[doc = "The content URL of the thumbnail image.\n\nThe URL to the image has a default lifetime of 30 minutes.\nThis URL is tagged with the account of the requester. Anyone with the URL\neffectively accesses the image as the original requester. Access to the\nimage may be lost if the presentation's sharing settings change.\nThe mime type of the thumbnail image is the same as specified in the\n`GetPageThumbnailRequest`."]
        #[serde(rename = "contentUrl", default)]
        pub content_url: Option<String>,
        #[doc = "The positive height in pixels of the thumbnail image."]
        #[serde(rename = "height", default)]
        pub height: Option<i32>,
        #[doc = "The positive width in pixels of the thumbnail image."]
        #[serde(rename = "width", default)]
        pub width: Option<i32>,
    }
    impl ::field_selector::FieldSelector for Thumbnail {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UngroupObjectsRequest {
        #[doc = "The object IDs of the objects to ungroup.\n\nOnly groups that are not inside other\ngroups can be ungrouped. All the groups\nshould be on the same page. The group itself is deleted. The visual sizes\nand positions of all the children are preserved."]
        #[serde(rename = "objectIds", default)]
        pub object_ids: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for UngroupObjectsRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UnmergeTableCellsRequest {
        #[doc = "The object ID of the table."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The table range specifying which cells of the table to unmerge.\n\nAll merged cells in this range will be unmerged, and cells that are already\nunmerged will not be affected. If the range has no merged cells, the\nrequest will do nothing. If there is text in any of the merged cells, the\ntext will remain in the upper-left (\"head\") cell of the resulting block of\nunmerged cells."]
        #[serde(rename = "tableRange", default)]
        pub table_range: Option<crate::schemas::TableRange>,
    }
    impl ::field_selector::FieldSelector for UnmergeTableCellsRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateImagePropertiesRequest {
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `imageProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the image outline color, set `fields` to\n`\"outline.outlineFill.solidFill.color\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(rename = "fields", default)]
        pub fields: Option<String>,
        #[doc = "The image properties to update."]
        #[serde(rename = "imageProperties", default)]
        pub image_properties: Option<crate::schemas::ImageProperties>,
        #[doc = "The object ID of the image the updates are applied to."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for UpdateImagePropertiesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UpdateLineCategoryRequestLineCategory {
        #[doc = "Unspecified line category."]
        LineCategoryUnspecified,
        #[doc = "Straight connectors, including straight connector 1."]
        Straight,
        #[doc = "Bent connectors, including bent connector 2 to 5."]
        Bent,
        #[doc = "Curved connectors, including curved connector 2 to 5."]
        Curved,
    }
    impl UpdateLineCategoryRequestLineCategory {
        pub fn as_str(self) -> &'static str {
            match self {
                UpdateLineCategoryRequestLineCategory::LineCategoryUnspecified => {
                    "LINE_CATEGORY_UNSPECIFIED"
                }
                UpdateLineCategoryRequestLineCategory::Straight => "STRAIGHT",
                UpdateLineCategoryRequestLineCategory::Bent => "BENT",
                UpdateLineCategoryRequestLineCategory::Curved => "CURVED",
            }
        }
    }
    impl ::std::fmt::Display for UpdateLineCategoryRequestLineCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UpdateLineCategoryRequestLineCategory {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UpdateLineCategoryRequestLineCategory {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LINE_CATEGORY_UNSPECIFIED" => {
                    UpdateLineCategoryRequestLineCategory::LineCategoryUnspecified
                }
                "STRAIGHT" => UpdateLineCategoryRequestLineCategory::Straight,
                "BENT" => UpdateLineCategoryRequestLineCategory::Bent,
                "CURVED" => UpdateLineCategoryRequestLineCategory::Curved,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UpdateLineCategoryRequest {
        #[doc = "The line category to update to.\n\nThe exact line type is determined based\non the category to update to and how it's routed to connect to other page\nelements."]
        #[serde(rename = "lineCategory", default)]
        pub line_category: Option<crate::schemas::UpdateLineCategoryRequestLineCategory>,
        #[doc = "The object ID of the line the update is applied to.\n\nOnly a line with a category\nindicating it is a \"connector\" can be updated.\n\nThe line may be rerouted after updating its category."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for UpdateLineCategoryRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateLinePropertiesRequest {
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `lineProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the line solid fill color, set `fields` to\n`\"lineFill.solidFill.color\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(rename = "fields", default)]
        pub fields: Option<String>,
        #[doc = "The line properties to update."]
        #[serde(rename = "lineProperties", default)]
        pub line_properties: Option<crate::schemas::LineProperties>,
        #[doc = "The object ID of the line the update is applied to."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for UpdateLinePropertiesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UpdatePageElementAltTextRequest {
        #[doc = "The updated alt text description of the page element. If unset the existing\nvalue will be maintained. The description is exposed to screen readers\nand other accessibility interfaces. Only use human readable values related\nto the content of the page element."]
        #[serde(rename = "description", default)]
        pub description: Option<String>,
        #[doc = "The object ID of the page element the updates are applied to."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The updated alt text title of the page element. If unset the\nexisting value will be maintained. The title is exposed to screen readers\nand other accessibility interfaces. Only use human readable values related\nto the content of the page element."]
        #[serde(rename = "title", default)]
        pub title: Option<String>,
    }
    impl ::field_selector::FieldSelector for UpdatePageElementAltTextRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UpdatePageElementTransformRequestApplyMode {
        #[doc = "Unspecified mode."]
        ApplyModeUnspecified,
        #[doc = "Applies the new AffineTransform matrix to the existing one, and\nreplaces the existing one with the resulting concatenation."]
        Relative,
        #[doc = "Replaces the existing AffineTransform matrix with the new one."]
        Absolute,
    }
    impl UpdatePageElementTransformRequestApplyMode {
        pub fn as_str(self) -> &'static str {
            match self {
                UpdatePageElementTransformRequestApplyMode::ApplyModeUnspecified => {
                    "APPLY_MODE_UNSPECIFIED"
                }
                UpdatePageElementTransformRequestApplyMode::Relative => "RELATIVE",
                UpdatePageElementTransformRequestApplyMode::Absolute => "ABSOLUTE",
            }
        }
    }
    impl ::std::fmt::Display for UpdatePageElementTransformRequestApplyMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UpdatePageElementTransformRequestApplyMode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UpdatePageElementTransformRequestApplyMode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APPLY_MODE_UNSPECIFIED" => {
                    UpdatePageElementTransformRequestApplyMode::ApplyModeUnspecified
                }
                "RELATIVE" => UpdatePageElementTransformRequestApplyMode::Relative,
                "ABSOLUTE" => UpdatePageElementTransformRequestApplyMode::Absolute,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdatePageElementTransformRequest {
        #[doc = "The apply mode of the transform update."]
        #[serde(rename = "applyMode", default)]
        pub apply_mode: Option<crate::schemas::UpdatePageElementTransformRequestApplyMode>,
        #[doc = "The object ID of the page element to update."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The input transform matrix used to update the page element."]
        #[serde(rename = "transform", default)]
        pub transform: Option<crate::schemas::AffineTransform>,
    }
    impl ::field_selector::FieldSelector for UpdatePageElementTransformRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UpdatePageElementsZOrderRequestOperation {
        #[doc = "Unspecified operation."]
        ZOrderOperationUnspecified,
        #[doc = "Brings the page elements to the front of the page."]
        BringToFront,
        #[doc = "Brings the page elements forward on the page by one element relative to the\nforwardmost one in the specified page elements."]
        BringForward,
        #[doc = "Sends the page elements backward on the page by one element relative to the\nfurthest behind one in the specified page elements."]
        SendBackward,
        #[doc = "Sends the page elements to the back of the page."]
        SendToBack,
    }
    impl UpdatePageElementsZOrderRequestOperation {
        pub fn as_str(self) -> &'static str {
            match self {
                UpdatePageElementsZOrderRequestOperation::ZOrderOperationUnspecified => {
                    "Z_ORDER_OPERATION_UNSPECIFIED"
                }
                UpdatePageElementsZOrderRequestOperation::BringToFront => "BRING_TO_FRONT",
                UpdatePageElementsZOrderRequestOperation::BringForward => "BRING_FORWARD",
                UpdatePageElementsZOrderRequestOperation::SendBackward => "SEND_BACKWARD",
                UpdatePageElementsZOrderRequestOperation::SendToBack => "SEND_TO_BACK",
            }
        }
    }
    impl ::std::fmt::Display for UpdatePageElementsZOrderRequestOperation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UpdatePageElementsZOrderRequestOperation {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UpdatePageElementsZOrderRequestOperation {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "Z_ORDER_OPERATION_UNSPECIFIED" => {
                    UpdatePageElementsZOrderRequestOperation::ZOrderOperationUnspecified
                }
                "BRING_TO_FRONT" => UpdatePageElementsZOrderRequestOperation::BringToFront,
                "BRING_FORWARD" => UpdatePageElementsZOrderRequestOperation::BringForward,
                "SEND_BACKWARD" => UpdatePageElementsZOrderRequestOperation::SendBackward,
                "SEND_TO_BACK" => UpdatePageElementsZOrderRequestOperation::SendToBack,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UpdatePageElementsZOrderRequest {
        #[doc = "The Z-order operation to apply on the page elements.\n\nWhen applying the operation on multiple page elements, the relative\nZ-orders within these page elements before the operation is maintained."]
        #[serde(rename = "operation", default)]
        pub operation: Option<crate::schemas::UpdatePageElementsZOrderRequestOperation>,
        #[doc = "The object IDs of the page elements to update.\n\nAll the page elements must be on the same page and must not be grouped."]
        #[serde(rename = "pageElementObjectIds", default)]
        pub page_element_object_ids: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for UpdatePageElementsZOrderRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdatePagePropertiesRequest {
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `pageProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the page background solid fill color, set `fields`\nto `\"pageBackgroundFill.solidFill.color\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(rename = "fields", default)]
        pub fields: Option<String>,
        #[doc = "The object ID of the page the update is applied to."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The page properties to update."]
        #[serde(rename = "pageProperties", default)]
        pub page_properties: Option<crate::schemas::PageProperties>,
    }
    impl ::field_selector::FieldSelector for UpdatePagePropertiesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateParagraphStyleRequest {
        #[doc = "The location of the cell in the table containing the paragraph(s) to\nstyle. If `object_id` refers to a table, `cell_location` must have a value.\nOtherwise, it must not."]
        #[serde(rename = "cellLocation", default)]
        pub cell_location: Option<crate::schemas::TableCellLocation>,
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `style` is implied and\nshould not be specified. A single `\"*\"` can be used as short-hand for\nlisting every field.\n\nFor example, to update the paragraph alignment, set `fields` to\n`\"alignment\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(rename = "fields", default)]
        pub fields: Option<String>,
        #[doc = "The object ID of the shape or table with the text to be styled."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The paragraph's style."]
        #[serde(rename = "style", default)]
        pub style: Option<crate::schemas::ParagraphStyle>,
        #[doc = "The range of text containing the paragraph(s) to style."]
        #[serde(rename = "textRange", default)]
        pub text_range: Option<crate::schemas::Range>,
    }
    impl ::field_selector::FieldSelector for UpdateParagraphStyleRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateShapePropertiesRequest {
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `shapeProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the shape background solid fill color, set `fields`\nto `\"shapeBackgroundFill.solidFill.color\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(rename = "fields", default)]
        pub fields: Option<String>,
        #[doc = "The object ID of the shape the updates are applied to."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The shape properties to update."]
        #[serde(rename = "shapeProperties", default)]
        pub shape_properties: Option<crate::schemas::ShapeProperties>,
    }
    impl ::field_selector::FieldSelector for UpdateShapePropertiesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UpdateSlidesPositionRequest {
        #[doc = "The index where the slides should be inserted, based on the slide\narrangement before the move takes place. Must be between zero and the\nnumber of slides in the presentation, inclusive."]
        #[serde(rename = "insertionIndex", default)]
        pub insertion_index: Option<i32>,
        #[doc = "The IDs of the slides in the presentation that should be moved.\nThe slides in this list must be in existing presentation order, without\nduplicates."]
        #[serde(rename = "slideObjectIds", default)]
        pub slide_object_ids: Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for UpdateSlidesPositionRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UpdateTableBorderPropertiesRequestBorderPosition {
        #[doc = "All borders in the range."]
        All,
        #[doc = "Borders at the bottom of the range."]
        Bottom,
        #[doc = "Borders on the inside of the range."]
        Inner,
        #[doc = "Horizontal borders on the inside of the range."]
        InnerHorizontal,
        #[doc = "Vertical borders on the inside of the range."]
        InnerVertical,
        #[doc = "Borders at the left of the range."]
        Left,
        #[doc = "Borders along the outside of the range."]
        Outer,
        #[doc = "Borders at the right of the range."]
        Right,
        #[doc = "Borders at the top of the range."]
        Top,
    }
    impl UpdateTableBorderPropertiesRequestBorderPosition {
        pub fn as_str(self) -> &'static str {
            match self {
                UpdateTableBorderPropertiesRequestBorderPosition::All => "ALL",
                UpdateTableBorderPropertiesRequestBorderPosition::Bottom => "BOTTOM",
                UpdateTableBorderPropertiesRequestBorderPosition::Inner => "INNER",
                UpdateTableBorderPropertiesRequestBorderPosition::InnerHorizontal => {
                    "INNER_HORIZONTAL"
                }
                UpdateTableBorderPropertiesRequestBorderPosition::InnerVertical => "INNER_VERTICAL",
                UpdateTableBorderPropertiesRequestBorderPosition::Left => "LEFT",
                UpdateTableBorderPropertiesRequestBorderPosition::Outer => "OUTER",
                UpdateTableBorderPropertiesRequestBorderPosition::Right => "RIGHT",
                UpdateTableBorderPropertiesRequestBorderPosition::Top => "TOP",
            }
        }
    }
    impl ::std::fmt::Display for UpdateTableBorderPropertiesRequestBorderPosition {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UpdateTableBorderPropertiesRequestBorderPosition {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UpdateTableBorderPropertiesRequestBorderPosition {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL" => UpdateTableBorderPropertiesRequestBorderPosition::All,
                "BOTTOM" => UpdateTableBorderPropertiesRequestBorderPosition::Bottom,
                "INNER" => UpdateTableBorderPropertiesRequestBorderPosition::Inner,
                "INNER_HORIZONTAL" => {
                    UpdateTableBorderPropertiesRequestBorderPosition::InnerHorizontal
                }
                "INNER_VERTICAL" => UpdateTableBorderPropertiesRequestBorderPosition::InnerVertical,
                "LEFT" => UpdateTableBorderPropertiesRequestBorderPosition::Left,
                "OUTER" => UpdateTableBorderPropertiesRequestBorderPosition::Outer,
                "RIGHT" => UpdateTableBorderPropertiesRequestBorderPosition::Right,
                "TOP" => UpdateTableBorderPropertiesRequestBorderPosition::Top,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateTableBorderPropertiesRequest {
        #[doc = "The border position in the table range the updates should apply to. If a\nborder position is not specified, the updates will apply to all borders in\nthe table range."]
        #[serde(rename = "borderPosition", default)]
        pub border_position:
            Option<crate::schemas::UpdateTableBorderPropertiesRequestBorderPosition>,
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `tableBorderProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the table border solid fill color, set\n`fields` to `\"tableBorderFill.solidFill.color\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(rename = "fields", default)]
        pub fields: Option<String>,
        #[doc = "The object ID of the table."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The table border properties to update."]
        #[serde(rename = "tableBorderProperties", default)]
        pub table_border_properties: Option<crate::schemas::TableBorderProperties>,
        #[doc = "The table range representing the subset of the table to which the updates\nare applied. If a table range is not specified, the updates will apply to\nthe entire table."]
        #[serde(rename = "tableRange", default)]
        pub table_range: Option<crate::schemas::TableRange>,
    }
    impl ::field_selector::FieldSelector for UpdateTableBorderPropertiesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateTableCellPropertiesRequest {
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `tableCellProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the table cell background solid fill color, set\n`fields` to `\"tableCellBackgroundFill.solidFill.color\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(rename = "fields", default)]
        pub fields: Option<String>,
        #[doc = "The object ID of the table."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The table cell properties to update."]
        #[serde(rename = "tableCellProperties", default)]
        pub table_cell_properties: Option<crate::schemas::TableCellProperties>,
        #[doc = "The table range representing the subset of the table to which the updates\nare applied. If a table range is not specified, the updates will apply to\nthe entire table."]
        #[serde(rename = "tableRange", default)]
        pub table_range: Option<crate::schemas::TableRange>,
    }
    impl ::field_selector::FieldSelector for UpdateTableCellPropertiesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateTableColumnPropertiesRequest {
        #[doc = "The list of zero-based indices specifying which columns to update. If no\nindices are provided, all columns in the table will be updated."]
        #[serde(rename = "columnIndices", default)]
        pub column_indices: Option<Vec<i32>>,
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `tableColumnProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the column width, set `fields` to `\"column_width\"`.\n\nIf '\"column_width\"' is included in the field mask but the property is left\nunset, the column width will default to 406,400 EMU (32 points)."]
        #[serde(rename = "fields", default)]
        pub fields: Option<String>,
        #[doc = "The object ID of the table."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The table column properties to update.\n\nIf the value of `table_column_properties#column_width` in the request is\nless than 406,400 EMU (32 points), a 400 bad request error is returned."]
        #[serde(rename = "tableColumnProperties", default)]
        pub table_column_properties: Option<crate::schemas::TableColumnProperties>,
    }
    impl ::field_selector::FieldSelector for UpdateTableColumnPropertiesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateTableRowPropertiesRequest {
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `tableRowProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the minimum row height, set `fields` to\n`\"min_row_height\"`.\n\nIf '\"min_row_height\"' is included in the field mask but the property is\nleft unset, the minimum row height will default to 0."]
        #[serde(rename = "fields", default)]
        pub fields: Option<String>,
        #[doc = "The object ID of the table."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The list of zero-based indices specifying which rows to update. If no\nindices are provided, all rows in the table will be updated."]
        #[serde(rename = "rowIndices", default)]
        pub row_indices: Option<Vec<i32>>,
        #[doc = "The table row properties to update."]
        #[serde(rename = "tableRowProperties", default)]
        pub table_row_properties: Option<crate::schemas::TableRowProperties>,
    }
    impl ::field_selector::FieldSelector for UpdateTableRowPropertiesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateTextStyleRequest {
        #[doc = "The location of the cell in the table containing the text to style. If\n`object_id` refers to a table, `cell_location` must have a value.\nOtherwise, it must not."]
        #[serde(rename = "cellLocation", default)]
        pub cell_location: Option<crate::schemas::TableCellLocation>,
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `style` is implied and\nshould not be specified. A single `\"*\"` can be used as short-hand for\nlisting every field.\n\nFor example, to update the text style to bold, set `fields` to `\"bold\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(rename = "fields", default)]
        pub fields: Option<String>,
        #[doc = "The object ID of the shape or table with the text to be styled."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The style(s) to set on the text.\n\nIf the value for a particular style matches that of the parent, that style\nwill be set to inherit.\n\nCertain text style changes may cause other changes meant to mirror the\nbehavior of the Slides editor. See the documentation of\nTextStyle for more information."]
        #[serde(rename = "style", default)]
        pub style: Option<crate::schemas::TextStyle>,
        #[doc = "The range of text to style.\n\nThe range may be extended to include adjacent newlines.\n\nIf the range fully contains a paragraph belonging to a list, the\nparagraph's bullet is also updated with the matching text style."]
        #[serde(rename = "textRange", default)]
        pub text_range: Option<crate::schemas::Range>,
    }
    impl ::field_selector::FieldSelector for UpdateTextStyleRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UpdateVideoPropertiesRequest {
        #[doc = "The fields that should be updated.\n\nAt least one field must be specified. The root `videoProperties` is\nimplied and should not be specified. A single `\"*\"` can be used as\nshort-hand for listing every field.\n\nFor example to update the video outline color, set `fields` to\n`\"outline.outlineFill.solidFill.color\"`.\n\nTo reset a property to its default value, include its field name in the\nfield mask but leave the field itself unset."]
        #[serde(rename = "fields", default)]
        pub fields: Option<String>,
        #[doc = "The object ID of the video the updates are applied to."]
        #[serde(rename = "objectId", default)]
        pub object_id: Option<String>,
        #[doc = "The video properties to update."]
        #[serde(rename = "videoProperties", default)]
        pub video_properties: Option<crate::schemas::VideoProperties>,
    }
    impl ::field_selector::FieldSelector for UpdateVideoPropertiesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VideoSource {
        #[doc = "The video source is unspecified."]
        SourceUnspecified,
        #[doc = "The video source is YouTube."]
        Youtube,
        #[doc = "The video source is Google Drive."]
        Drive,
    }
    impl VideoSource {
        pub fn as_str(self) -> &'static str {
            match self {
                VideoSource::SourceUnspecified => "SOURCE_UNSPECIFIED",
                VideoSource::Youtube => "YOUTUBE",
                VideoSource::Drive => "DRIVE",
            }
        }
    }
    impl ::std::fmt::Display for VideoSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VideoSource {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VideoSource {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SOURCE_UNSPECIFIED" => VideoSource::SourceUnspecified,
                "YOUTUBE" => VideoSource::Youtube,
                "DRIVE" => VideoSource::Drive,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Video {
        #[doc = "The video source's unique identifier for this video."]
        #[serde(rename = "id", default)]
        pub id: Option<String>,
        #[doc = "The video source."]
        #[serde(rename = "source", default)]
        pub source: Option<crate::schemas::VideoSource>,
        #[doc = "An URL to a video. The URL is valid as long as the source video exists and\nsharing settings do not change."]
        #[serde(rename = "url", default)]
        pub url: Option<String>,
        #[doc = "The properties of the video."]
        #[serde(rename = "videoProperties", default)]
        pub video_properties: Option<crate::schemas::VideoProperties>,
    }
    impl ::field_selector::FieldSelector for Video {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VideoProperties {
        #[doc = "Whether to enable video autoplay when the page is displayed in present\nmode. Defaults to false."]
        #[serde(rename = "autoPlay", default)]
        pub auto_play: Option<bool>,
        #[doc = "The time at which to end playback, measured in seconds from the beginning\nof the video.\nIf set, the end time should be after the start time.\nIf not set or if you set this to a value that exceeds the video's length,\nthe video will be played until its end."]
        #[serde(rename = "end", default)]
        pub end: Option<u32>,
        #[doc = "Whether to mute the audio during video playback. Defaults to false."]
        #[serde(rename = "mute", default)]
        pub mute: Option<bool>,
        #[doc = "The outline of the video. The default outline matches the defaults for new\nvideos created in the Slides editor."]
        #[serde(rename = "outline", default)]
        pub outline: Option<crate::schemas::Outline>,
        #[doc = "The time at which to start playback, measured in seconds from the beginning\nof the video.\nIf set, the start time should be before the end time.\nIf you set this to a value that exceeds the video's length in seconds, the\nvideo will be played from the last second.\nIf not set, the video will be played from the beginning."]
        #[serde(rename = "start", default)]
        pub start: Option<u32>,
    }
    impl ::field_selector::FieldSelector for VideoProperties {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct WeightedFontFamily {
        #[doc = "The font family of the text.\n\nThe font family can be any font from the Font menu in Slides or from\n[Google Fonts] (https://fonts.google.com/). If the font name is\nunrecognized, the text is rendered in `Arial`."]
        #[serde(rename = "fontFamily", default)]
        pub font_family: Option<String>,
        #[doc = "The rendered weight of the text. This field can have any value that is a\nmultiple of `100` between `100` and `900`, inclusive. This range\ncorresponds to the numerical values described in the CSS 2.1\nSpecification,\n[section 15.6](https://www.w3.org/TR/CSS21/fonts.html#font-boldness),\nwith non-numerical values disallowed. Weights greater than or equal to\n`700` are considered bold, and weights less than `700`are not bold. The\ndefault value is `400` (\"normal\")."]
        #[serde(rename = "weight", default)]
        pub weight: Option<i32>,
    }
    impl ::field_selector::FieldSelector for WeightedFontFamily {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct WordArt {
        #[doc = "The text rendered as word art."]
        #[serde(rename = "renderedText", default)]
        pub rendered_text: Option<String>,
    }
    impl ::field_selector::FieldSelector for WordArt {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct WriteControl {
        #[doc = "The revision ID of the presentation required for the write request. If\nspecified and the `required_revision_id` doesn't exactly match the\npresentation's current `revision_id`, the request will not be processed and\nwill return a 400 bad request error."]
        #[serde(rename = "requiredRevisionId", default)]
        pub required_revision_id: Option<String>,
    }
    impl ::field_selector::FieldSelector for WriteControl {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
            selector.push_str("*");
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
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
}
pub struct Client<A> {
    reqwest: ::reqwest::Client,
    auth: ::std::sync::Mutex<A>,
}
impl<A: yup_oauth2::GetToken> Client<A> {
    pub fn new(auth: A) -> Self {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: ::std::sync::Mutex::new(auth),
        }
    }
    #[doc = "Actions that can be performed on the presentations resource"]
    pub fn presentations(&self) -> crate::presentations::PresentationsActions<A> {
        crate::presentations::PresentationsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod presentations {
    pub mod params {}
    pub struct PresentationsActions<'a, A> {
        pub(super) reqwest: &'a reqwest::Client,
        pub(super) auth: &'a std::sync::Mutex<A>,
    }
    impl<'a, A: yup_oauth2::GetToken> PresentationsActions<'a, A> {
        #[doc = "Applies one or more updates to the presentation.\n\nEach request is validated before\nbeing applied. If any request is not valid, then the entire request will\nfail and nothing will be applied.\n\nSome requests have replies to\ngive you some information about how they are applied. Other requests do\nnot need to return information; these each return an empty reply.\nThe order of replies matches that of the requests.\n\nFor example, suppose you call batchUpdate with four updates, and only the\nthird one returns information. The response would have two empty replies:\nthe reply to the third request, and another empty reply, in that order.\n\nBecause other users may be editing the presentation, the presentation\nmight not exactly reflect your changes: your changes may\nbe altered with respect to collaborator changes. If there are no\ncollaborators, the presentation should reflect your changes. In any case,\nthe updates in your request are guaranteed to be applied together\natomically."]
        pub fn batch_update(
            &self,
            request: crate::schemas::BatchUpdatePresentationRequest,
            presentation_id: impl Into<String>,
        ) -> BatchUpdateRequestBuilder<A> {
            BatchUpdateRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
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
                presentation_id: presentation_id.into(),
            }
        }
        #[doc = "Creates a blank presentation using the title given in the request. If a\n`presentationId` is provided, it is used as the ID of the new presentation.\nOtherwise, a new ID is generated. Other fields in the request, including\nany provided content, are ignored.\nReturns the created presentation."]
        pub fn create(&self, request: crate::schemas::Presentation) -> CreateRequestBuilder<A> {
            CreateRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
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
        #[doc = "Gets the latest version of the specified presentation."]
        pub fn get(&self, presentation_id: impl Into<String>) -> GetRequestBuilder<A> {
            GetRequestBuilder {
                reqwest: &self.reqwest,
                auth: &self.auth,
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
                presentation_id: presentation_id.into(),
            }
        }
        #[doc = "Actions that can be performed on the pages resource"]
        pub fn pages(&self) -> pages::PagesActions<A> {
            pages::PagesActions
        }
    }
    #[derive(Debug, Clone)]
    pub struct BatchUpdateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::BatchUpdatePresentationRequest,
        presentation_id: String,
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
    impl<'a, A: yup_oauth2::GetToken> BatchUpdateRequestBuilder<'a, A> {
        #[doc = "OAuth access token."]
        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.access_token = Some(value.into());
            self
        }
        #[doc = "Data format for response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "JSONP"]
        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
            self.callback = Some(value.into());
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_protocol = Some(value.into());
            self
        }
        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_type = Some(value.into());
            self
        }
        #[doc = "V1 error format."]
        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
            self.xgafv = Some(value);
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::BatchUpdatePresentationResponse, Box<dyn ::std::error::Error>>
        {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://slides.googleapis.com/".to_owned();
            output.push_str("v1/presentations/");
            output.push_str(&self.presentation_id);
            output.push_str(":batchUpdate");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("access_token", &self.access_token)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("callback", &self.callback)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
            let req = req.query(&[("uploadType", &self.upload_type)]);
            let req = req.query(&[("$.xgafv", &self.xgafv)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct CreateRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        request: crate::schemas::Presentation,
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
    impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
        #[doc = "OAuth access token."]
        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.access_token = Some(value.into());
            self
        }
        #[doc = "Data format for response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "JSONP"]
        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
            self.callback = Some(value.into());
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_protocol = Some(value.into());
            self
        }
        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_type = Some(value.into());
            self
        }
        #[doc = "V1 error format."]
        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
            self.xgafv = Some(value);
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Presentation, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            let req = req.json(&self.request);
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://slides.googleapis.com/".to_owned();
            output.push_str("v1/presentations");
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::POST, path);
            let req = req.query(&[("access_token", &self.access_token)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("callback", &self.callback)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
            let req = req.query(&[("uploadType", &self.upload_type)]);
            let req = req.query(&[("$.xgafv", &self.xgafv)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    #[derive(Debug, Clone)]
    pub struct GetRequestBuilder<'a, A> {
        pub(crate) reqwest: &'a ::reqwest::Client,
        pub(crate) auth: &'a ::std::sync::Mutex<A>,
        presentation_id: String,
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
    impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
        #[doc = "OAuth access token."]
        pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.access_token = Some(value.into());
            self
        }
        #[doc = "Data format for response."]
        pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
            self.alt = Some(value);
            self
        }
        #[doc = "JSONP"]
        pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
            self.callback = Some(value.into());
            self
        }
        #[doc = "Selector specifying which fields to include in a partial response."]
        pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
            self.fields = Some(value.into());
            self
        }
        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
        pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
            self.key = Some(value.into());
            self
        }
        #[doc = "OAuth 2.0 token for the current user."]
        pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
            self.oauth_token = Some(value.into());
            self
        }
        #[doc = "Returns response with indentations and line breaks."]
        pub fn pretty_print(&mut self, value: bool) -> &mut Self {
            self.pretty_print = Some(value);
            self
        }
        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
        pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
            self.quota_user = Some(value.into());
            self
        }
        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
        pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_protocol = Some(value.into());
            self
        }
        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
        pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
            self.upload_type = Some(value.into());
            self
        }
        #[doc = "V1 error format."]
        pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
            self.xgafv = Some(value);
            self
        }
        pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            self._execute()
        }
        #[doc = r" TODO: Remove once development debugging is no longer a priority."]
        pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.text()?)
        }
        pub fn execute_debug(
            self,
        ) -> Result<crate::schemas::Presentation, Box<dyn ::std::error::Error>> {
            self.execute()
        }
        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
        {
            if self.fields.is_none() {
                self.fields = Some(T::field_selector());
            }
            let req = self._request(&self._path());
            Ok(req.send()?.error_for_status()?.json()?)
        }
        fn _path(&self) -> String {
            let mut output = "https://slides.googleapis.com/".to_owned();
            output.push_str("v1/presentations/");
            output.push_str(&self.presentation_id);
            output
        }
        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
            let req = self.reqwest.request(::reqwest::Method::GET, path);
            let req = req.query(&[("access_token", &self.access_token)]);
            let req = req.query(&[("alt", &self.alt)]);
            let req = req.query(&[("callback", &self.callback)]);
            let req = req.query(&[("fields", &self.fields)]);
            let req = req.query(&[("key", &self.key)]);
            let req = req.query(&[("oauth_token", &self.oauth_token)]);
            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
            let req = req.query(&[("quotaUser", &self.quota_user)]);
            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
            let req = req.query(&[("uploadType", &self.upload_type)]);
            let req = req.query(&[("$.xgafv", &self.xgafv)]);
            let mut auth = self.auth.lock().unwrap();
            let req = req.bearer_auth(
                auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.readonly"])
                    .unwrap()
                    .access_token,
            );
            req
        }
    }
    pub mod pages {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetThumbnailThumbnailPropertiesMimeType {}
            impl GetThumbnailThumbnailPropertiesMimeType {
                pub fn as_str(self) -> &'static str {
                    match self {}
                }
            }
            impl ::std::fmt::Display for GetThumbnailThumbnailPropertiesMimeType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetThumbnailThumbnailPropertiesMimeType {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetThumbnailThumbnailPropertiesMimeType {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetThumbnailThumbnailPropertiesThumbnailSize {}
            impl GetThumbnailThumbnailPropertiesThumbnailSize {
                pub fn as_str(self) -> &'static str {
                    match self {}
                }
            }
            impl ::std::fmt::Display for GetThumbnailThumbnailPropertiesThumbnailSize {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetThumbnailThumbnailPropertiesThumbnailSize {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetThumbnailThumbnailPropertiesThumbnailSize {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
        }
        pub struct PagesActions<'a, A> {
            pub(super) reqwest: &'a reqwest::Client,
            pub(super) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> PagesActions<'a, A> {
            #[doc = "Gets the latest version of the specified page in the presentation."]
            pub fn get(
                &self,
                presentation_id: impl Into<String>,
                page_object_id: impl Into<String>,
            ) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
                    presentation_id: presentation_id.into(),
                    page_object_id: page_object_id.into(),
                }
            }
            #[doc = "Generates a thumbnail of the latest version of the specified page in the\npresentation and returns a URL to the thumbnail image.\n\nThis request counts as an [expensive read request](/slides/limits) for\nquota purposes."]
            pub fn get_thumbnail(
                &self,
                presentation_id: impl Into<String>,
                page_object_id: impl Into<String>,
            ) -> GetThumbnailRequestBuilder<A> {
                GetThumbnailRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
                    presentation_id: presentation_id.into(),
                    page_object_id: page_object_id.into(),
                    thumbnail_properties_mime_type: None,
                    thumbnail_properties_thumbnail_size: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            presentation_id: String,
            page_object_id: String,
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
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Page, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://slides.googleapis.com/".to_owned();
                output.push_str("v1/presentations/");
                output.push_str(&self.presentation_id);
                output.push_str("/pages/");
                output.push_str(&self.page_object_id);
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.readonly"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetThumbnailRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            presentation_id: String,
            page_object_id: String,
            thumbnail_properties_mime_type:
                Option<crate::pages::params::GetThumbnailThumbnailPropertiesMimeType>,
            thumbnail_properties_thumbnail_size:
                Option<crate::pages::params::GetThumbnailThumbnailPropertiesThumbnailSize>,
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
        impl<'a, A: yup_oauth2::GetToken> GetThumbnailRequestBuilder<'a, A> {
            #[doc = "The optional mime type of the thumbnail image.\n\nIf you don't specify the mime type, the default mime type will be PNG."]
            pub fn thumbnail_properties_mime_type(
                &mut self,
                value: crate::pages::params::GetThumbnailThumbnailPropertiesMimeType,
            ) -> &mut Self {
                self.thumbnail_properties_mime_type = Some(value);
                self
            }
            #[doc = "The optional thumbnail image size.\n\nIf you don't specify the size, the server chooses a default size of the\nimage."]
            pub fn thumbnail_properties_thumbnail_size(
                &mut self,
                value: crate::pages::params::GetThumbnailThumbnailPropertiesThumbnailSize,
            ) -> &mut Self {
                self.thumbnail_properties_thumbnail_size = Some(value);
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "Data format for response."]
            pub fn alt(&mut self, value: crate::params::Alt) -> &mut Self {
                self.alt = Some(value);
                self
            }
            #[doc = "JSONP"]
            pub fn callback(&mut self, value: impl Into<String>) -> &mut Self {
                self.callback = Some(value.into());
                self
            }
            #[doc = "Selector specifying which fields to include in a partial response."]
            pub fn fields(&mut self, value: impl Into<String>) -> &mut Self {
                self.fields = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(&mut self, value: impl Into<String>) -> &mut Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(&mut self, value: impl Into<String>) -> &mut Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(&mut self, value: bool) -> &mut Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(&mut self, value: impl Into<String>) -> &mut Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(&mut self, value: impl Into<String>) -> &mut Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(&mut self, value: crate::params::Xgafv) -> &mut Self {
                self.xgafv = Some(value);
                self
            }
            pub fn execute<T>(mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                self._execute()
            }
            #[doc = r" TODO: Remove once development debugging is no longer a priority."]
            pub fn execute_text(self) -> Result<String, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.text()?)
            }
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Thumbnail, Box<dyn ::std::error::Error>> {
                self.execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://slides.googleapis.com/".to_owned();
                output.push_str("v1/presentations/");
                output.push_str(&self.presentation_id);
                output.push_str("/pages/");
                output.push_str(&self.page_object_id);
                output.push_str("/thumbnail");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[(
                    "thumbnailProperties.mimeType",
                    &self.thumbnail_properties_mime_type,
                )]);
                let req = req.query(&[(
                    "thumbnailProperties.thumbnailSize",
                    &self.thumbnail_properties_thumbnail_size,
                )]);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let req = req.bearer_auth(
                    auth.token::<_, &str>(&["https://www.googleapis.com/auth/drive.readonly"])
                        .unwrap()
                        .access_token,
                );
                req
            }
        }
    }
}
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
pub struct ResumableUpload {
    reqwest: ::reqwest::Client,
    url: String,
    progress: Option<i64>,
}

impl ResumableUpload {
    pub fn new(reqwest: ::reqwest::Client, url: String) -> Self {
        ResumableUpload {
            reqwest,
            url,
            progress: None,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn upload<R>(&mut self, mut reader: R) -> Result<(), Box<dyn ::std::error::Error>>
    where
        R: ::std::io::Read + ::std::io::Seek + Send + 'static,
    {
        let reader_len = {
            let start = reader.seek(::std::io::SeekFrom::Current(0))?;
            let end = reader.seek(::std::io::SeekFrom::End(0))?;
            reader.seek(::std::io::SeekFrom::Start(start))?;
            end
        };
        let progress = match self.progress {
            Some(progress) => progress,
            None => {
                let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
                let req = req.header(::reqwest::header::CONTENT_LENGTH, 0);
                let req = req.header(
                    ::reqwest::header::CONTENT_RANGE,
                    format!("bytes */{}", reader_len),
                );
                let resp = req.send()?.error_for_status()?;
                match resp.headers().get(::reqwest::header::RANGE) {
                    Some(range_header) => {
                        let (_, progress) = parse_range_header(range_header)
                            .map_err(|e| format!("invalid RANGE header: {}", e))?;
                        progress + 1
                    }
                    None => 0,
                }
            }
        };

        reader.seek(::std::io::SeekFrom::Start(progress as u64))?;
        let content_length = reader_len - progress as u64;
        let content_range = format!("bytes {}-{}/{}", progress, reader_len - 1, reader_len);
        let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
        let req = req.header(::reqwest::header::CONTENT_RANGE, content_range);
        let req = req.body(::reqwest::Body::sized(reader, content_length));
        req.send()?.error_for_status()?;
        Ok(())
    }
}

fn parse_range_header(
    range: &::reqwest::header::HeaderValue,
) -> Result<(i64, i64), Box<dyn ::std::error::Error>> {
    let range = range.to_str()?;
    if !range.starts_with("bytes ") {
        return Err(r#"does not begin with "bytes""#.to_owned().into());
    }
    let range = &range[6..];
    let slash_idx = range
        .find('/')
        .ok_or_else(|| r#"does not contain"#.to_owned())?;
    let (begin, end) = range.split_at(slash_idx);
    let end = &end[1..]; // remove '/'
    let begin: i64 = begin.parse()?;
    let end: i64 = end.parse()?;
    Ok((begin, end))
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
mod parsed_string {
    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
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

trait IterableMethod {
    fn set_page_token(&mut self, value: String);
    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
    where
        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector;
}

struct PageIter<'a, M, T> {
    method: &'a mut M,
    finished: bool,
    _phantom: ::std::marker::PhantomData<T>,
}

impl<'a, M, T> Iterator for PageIter<'a, M, T>
where
    M: IterableMethod,
    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
{
    type Item = Result<T, Box<dyn ::std::error::Error>>;

    fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
        use ::field_selector::FieldSelector;
        #[derive(::serde::Deserialize, FieldSelector)]
        struct PaginatedResult<T>
        where
            T: FieldSelector,
        {
            #[serde(rename = "nextPageToken")]
            next_page_token: Option<String>,

            #[serde(flatten)]
            page_contents: T,
        }

        if self.finished {
            return None;
        }

        let paginated_result: PaginatedResult<T> = match self.method.execute() {
            Ok(r) => r,
            Err(err) => return Some(Err(err)),
        };

        if let Some(next_page_token) = paginated_result.next_page_token {
            self.method.set_page_token(next_page_token);
        } else {
            self.finished = true;
        }

        Some(Ok(paginated_result.page_contents))
    }
}