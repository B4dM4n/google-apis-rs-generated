#![doc = "# Resources and Methods\n* [divisions](resources/divisions/struct.DivisionsActions.html)\n  * [*search*](resources/divisions/struct.SearchRequestBuilder.html)\n* [elections](resources/elections/struct.ElectionsActions.html)\n  * [*electionQuery*](resources/elections/struct.ElectionQueryRequestBuilder.html), [*voterInfoQuery*](resources/elections/struct.VoterInfoQueryRequestBuilder.html)\n* [representatives](resources/representatives/struct.RepresentativesActions.html)\n  * [*representativeInfoByAddress*](resources/representatives/struct.RepresentativeInfoByAddressRequestBuilder.html), [*representativeInfoByDivision*](resources/representatives/struct.RepresentativeInfoByDivisionRequestBuilder.html)\n"]
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
    pub struct AdministrationRegion {
        #[doc = "The election administration body for this area."]
        #[serde(
            rename = "electionAdministrationBody",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub election_administration_body: ::std::option::Option<crate::schemas::AdministrativeBody>,
        #[doc = "The city or county that provides election information for this voter. This object can have the same elements as state."]
        #[serde(
            rename = "local_jurisdiction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_jurisdiction: ::std::option::Option<Box<crate::schemas::AdministrationRegion>>,
        #[doc = "The name of the jurisdiction."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A list of sources for this area. If multiple sources are listed the data has been aggregated from those sources."]
        #[serde(
            rename = "sources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sources: ::std::option::Option<Vec<crate::schemas::Source>>,
    }
    impl ::google_field_selector::FieldSelector for AdministrationRegion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdministrationRegion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AdministrativeBody {
        #[doc = "A URL provided by this administrative body for information on absentee voting."]
        #[serde(
            rename = "absenteeVotingInfoUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub absentee_voting_info_url: ::std::option::Option<String>,
        #[doc = "A URL provided by this administrative body to give contest information to the voter."]
        #[serde(
            rename = "ballotInfoUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ballot_info_url: ::std::option::Option<String>,
        #[doc = "The mailing address of this administrative body."]
        #[serde(
            rename = "correspondenceAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub correspondence_address: ::std::option::Option<crate::schemas::SimpleAddressType>,
        #[doc = "A URL provided by this administrative body for looking up general election information."]
        #[serde(
            rename = "electionInfoUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub election_info_url: ::std::option::Option<String>,
        #[doc = "A last minute or emergency notification text provided by this administrative body."]
        #[serde(
            rename = "electionNoticeText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub election_notice_text: ::std::option::Option<String>,
        #[doc = "A URL provided by this administrative body for additional information related to the last minute or emergency notification."]
        #[serde(
            rename = "electionNoticeUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub election_notice_url: ::std::option::Option<String>,
        #[doc = "The election officials for this election administrative body."]
        #[serde(
            rename = "electionOfficials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub election_officials: ::std::option::Option<Vec<crate::schemas::ElectionOfficial>>,
        #[doc = "A URL provided by this administrative body for confirming that the voter is registered to vote."]
        #[serde(
            rename = "electionRegistrationConfirmationUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub election_registration_confirmation_url: ::std::option::Option<String>,
        #[doc = "A URL provided by this administrative body for looking up how to register to vote."]
        #[serde(
            rename = "electionRegistrationUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub election_registration_url: ::std::option::Option<String>,
        #[doc = "A URL provided by this administrative body describing election rules to the voter."]
        #[serde(
            rename = "electionRulesUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub election_rules_url: ::std::option::Option<String>,
        #[doc = "A description of the hours of operation for this administrative body."]
        #[serde(
            rename = "hoursOfOperation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hours_of_operation: ::std::option::Option<String>,
        #[doc = "The name of this election administrative body."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The physical address of this administrative body."]
        #[serde(
            rename = "physicalAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub physical_address: ::std::option::Option<crate::schemas::SimpleAddressType>,
        #[doc = "A description of the services this administrative body may provide."]
        #[serde(
            rename = "voter_services",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub voter_services: ::std::option::Option<Vec<String>>,
        #[doc = "A URL provided by this administrative body for looking up where to vote."]
        #[serde(
            rename = "votingLocationFinderUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub voting_location_finder_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AdministrativeBody {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdministrativeBody {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Candidate {
        #[doc = "The URL for the candidate’s campaign web site."]
        #[serde(
            rename = "candidateUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub candidate_url: ::std::option::Option<String>,
        #[doc = "A list of known (social) media channels for this candidate."]
        #[serde(
            rename = "channels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub channels: ::std::option::Option<Vec<crate::schemas::Channel>>,
        #[doc = "The email address for the candidate’s campaign."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "The candidate’s name. If this is a joint ticket it will indicate the name of the candidate at the top of a ticket followed by a / and that name of candidate at the bottom of the ticket. e.g. “Mitt Romney / Paul Ryan”"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The order the candidate appears on the ballot for this contest."]
        #[serde(
            rename = "orderOnBallot",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub order_on_ballot: ::std::option::Option<i64>,
        #[doc = "The full name of the party the candidate is a member of."]
        #[serde(
            rename = "party",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub party: ::std::option::Option<String>,
        #[doc = "The voice phone number for the candidate’s campaign office."]
        #[serde(
            rename = "phone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone: ::std::option::Option<String>,
        #[doc = "A URL for a photo of the candidate."]
        #[serde(
            rename = "photoUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub photo_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Candidate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Candidate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Channel {
        #[doc = "The unique public identifier for the candidate’s channel."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The type of channel. The following is a list of types of channels, but is not exhaustive. More channel types may be added at a later time. One of: GooglePlus, YouTube, Facebook, Twitter"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Channel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Channel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Contest {
        #[doc = "A number specifying the position of this contest on the voter’s ballot."]
        #[serde(
            rename = "ballotPlacement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub ballot_placement: ::std::option::Option<i64>,
        #[doc = "The official title on the ballot for this contest, only where available."]
        #[serde(
            rename = "ballotTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ballot_title: ::std::option::Option<String>,
        #[doc = "The candidate choices for this contest."]
        #[serde(
            rename = "candidates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub candidates: ::std::option::Option<Vec<crate::schemas::Candidate>>,
        #[doc = "Information about the electoral district that this contest is in."]
        #[serde(
            rename = "district",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub district: ::std::option::Option<crate::schemas::ElectoralDistrict>,
        #[doc = "A description of any additional eligibility requirements for voting in this contest."]
        #[serde(
            rename = "electorateSpecifications",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub electorate_specifications: ::std::option::Option<String>,
        #[doc = "The levels of government of the office for this contest. There may be more than one in cases where a jurisdiction effectively acts at two different levels of government; for example, the mayor of the District of Columbia acts at “locality” level, but also effectively at both “administrative-area-2” and “administrative-area-1”."]
        #[serde(
            rename = "level",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub level: ::std::option::Option<Vec<crate::schemas::ContestLevelItems>>,
        #[doc = "The number of candidates that will be elected to office in this contest."]
        #[serde(
            rename = "numberElected",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub number_elected: ::std::option::Option<i64>,
        #[doc = "The number of candidates that a voter may vote for in this contest."]
        #[serde(
            rename = "numberVotingFor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub number_voting_for: ::std::option::Option<i64>,
        #[doc = "The name of the office for this contest."]
        #[serde(
            rename = "office",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub office: ::std::option::Option<String>,
        #[doc = "If this is a partisan election, the name of the party/parties it is for."]
        #[serde(
            rename = "primaryParties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primary_parties: ::std::option::Option<Vec<String>>,
        #[doc = "\\[DEPRECATED\\] If this is a partisan election, the name of the party it is for. This field as deprecated in favor of the array “primaryParties”, as contests may contain more than one party."]
        #[serde(
            rename = "primaryParty",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub primary_party: ::std::option::Option<String>,
        #[doc = "The type of contest. Usually this will be ‘General’, ‘Primary’, or ‘Run-off’ for contests with candidates. For referenda this will be ‘Referendum’. For Retention contests this will typically be ‘Retention’."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The set of ballot responses for the referendum. A ballot response represents a line on the ballot. Common examples might include “yes” or “no” for referenda. This field is only populated for contests of type ‘Referendum’."]
        #[serde(
            rename = "referendumBallotResponses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referendum_ballot_responses: ::std::option::Option<Vec<String>>,
        #[doc = "Specifies a short summary of the referendum that is typically on the ballot below the title but above the text. This field is only populated for contests of type ‘Referendum’."]
        #[serde(
            rename = "referendumBrief",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referendum_brief: ::std::option::Option<String>,
        #[doc = "A statement in opposition to the referendum. It does not necessarily appear on the ballot. This field is only populated for contests of type ‘Referendum’."]
        #[serde(
            rename = "referendumConStatement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referendum_con_statement: ::std::option::Option<String>,
        #[doc = "Specifies what effect abstaining (not voting) on the proposition will have (i.e. whether abstaining is considered a vote against it). This field is only populated for contests of type ‘Referendum’."]
        #[serde(
            rename = "referendumEffectOfAbstain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referendum_effect_of_abstain: ::std::option::Option<String>,
        #[doc = "The threshold of votes that the referendum needs in order to pass, e.g. “two-thirds”. This field is only populated for contests of type ‘Referendum’."]
        #[serde(
            rename = "referendumPassageThreshold",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referendum_passage_threshold: ::std::option::Option<String>,
        #[doc = "A statement in favor of the referendum. It does not necessarily appear on the ballot. This field is only populated for contests of type ‘Referendum’."]
        #[serde(
            rename = "referendumProStatement",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referendum_pro_statement: ::std::option::Option<String>,
        #[doc = "A brief description of the referendum. This field is only populated for contests of type ‘Referendum’."]
        #[serde(
            rename = "referendumSubtitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referendum_subtitle: ::std::option::Option<String>,
        #[doc = "The full text of the referendum. This field is only populated for contests of type ‘Referendum’."]
        #[serde(
            rename = "referendumText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referendum_text: ::std::option::Option<String>,
        #[doc = "The title of the referendum (e.g. ‘Proposition 42’). This field is only populated for contests of type ‘Referendum’."]
        #[serde(
            rename = "referendumTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referendum_title: ::std::option::Option<String>,
        #[doc = "A link to the referendum. This field is only populated for contests of type ‘Referendum’."]
        #[serde(
            rename = "referendumUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referendum_url: ::std::option::Option<String>,
        #[doc = "The roles which this office fulfills."]
        #[serde(
            rename = "roles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub roles: ::std::option::Option<Vec<crate::schemas::ContestRolesItems>>,
        #[doc = "A list of sources for this contest. If multiple sources are listed, the data has been aggregated from those sources."]
        #[serde(
            rename = "sources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sources: ::std::option::Option<Vec<crate::schemas::Source>>,
        #[doc = "“Yes” or “No” depending on whether this a contest being held outside the normal election cycle."]
        #[serde(
            rename = "special",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub special: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Contest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Contest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContestLevelItems {
        AdministrativeArea1,
        AdministrativeArea2,
        Country,
        International,
        Locality,
        Regional,
        Special,
        SubLocality1,
        SubLocality2,
    }
    impl ContestLevelItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ContestLevelItems::AdministrativeArea1 => "administrativeArea1",
                ContestLevelItems::AdministrativeArea2 => "administrativeArea2",
                ContestLevelItems::Country => "country",
                ContestLevelItems::International => "international",
                ContestLevelItems::Locality => "locality",
                ContestLevelItems::Regional => "regional",
                ContestLevelItems::Special => "special",
                ContestLevelItems::SubLocality1 => "subLocality1",
                ContestLevelItems::SubLocality2 => "subLocality2",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ContestLevelItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ContestLevelItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ContestLevelItems, ()> {
            Ok(match s {
                "administrativeArea1" => ContestLevelItems::AdministrativeArea1,
                "administrativeArea2" => ContestLevelItems::AdministrativeArea2,
                "country" => ContestLevelItems::Country,
                "international" => ContestLevelItems::International,
                "locality" => ContestLevelItems::Locality,
                "regional" => ContestLevelItems::Regional,
                "special" => ContestLevelItems::Special,
                "subLocality1" => ContestLevelItems::SubLocality1,
                "subLocality2" => ContestLevelItems::SubLocality2,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ContestLevelItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ContestLevelItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ContestLevelItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "administrativeArea1" => ContestLevelItems::AdministrativeArea1,
                "administrativeArea2" => ContestLevelItems::AdministrativeArea2,
                "country" => ContestLevelItems::Country,
                "international" => ContestLevelItems::International,
                "locality" => ContestLevelItems::Locality,
                "regional" => ContestLevelItems::Regional,
                "special" => ContestLevelItems::Special,
                "subLocality1" => ContestLevelItems::SubLocality1,
                "subLocality2" => ContestLevelItems::SubLocality2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ContestLevelItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContestLevelItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContestRolesItems {
        DeputyHeadOfGovernment,
        ExecutiveCouncil,
        GovernmentOfficer,
        HeadOfGovernment,
        HeadOfState,
        HighestCourtJudge,
        Judge,
        LegislatorLowerBody,
        LegislatorUpperBody,
        OtherRole,
        SchoolBoard,
        SpecialPurposeOfficer,
    }
    impl ContestRolesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                ContestRolesItems::DeputyHeadOfGovernment => "deputyHeadOfGovernment",
                ContestRolesItems::ExecutiveCouncil => "executiveCouncil",
                ContestRolesItems::GovernmentOfficer => "governmentOfficer",
                ContestRolesItems::HeadOfGovernment => "headOfGovernment",
                ContestRolesItems::HeadOfState => "headOfState",
                ContestRolesItems::HighestCourtJudge => "highestCourtJudge",
                ContestRolesItems::Judge => "judge",
                ContestRolesItems::LegislatorLowerBody => "legislatorLowerBody",
                ContestRolesItems::LegislatorUpperBody => "legislatorUpperBody",
                ContestRolesItems::OtherRole => "otherRole",
                ContestRolesItems::SchoolBoard => "schoolBoard",
                ContestRolesItems::SpecialPurposeOfficer => "specialPurposeOfficer",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ContestRolesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ContestRolesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ContestRolesItems, ()> {
            Ok(match s {
                "deputyHeadOfGovernment" => ContestRolesItems::DeputyHeadOfGovernment,
                "executiveCouncil" => ContestRolesItems::ExecutiveCouncil,
                "governmentOfficer" => ContestRolesItems::GovernmentOfficer,
                "headOfGovernment" => ContestRolesItems::HeadOfGovernment,
                "headOfState" => ContestRolesItems::HeadOfState,
                "highestCourtJudge" => ContestRolesItems::HighestCourtJudge,
                "judge" => ContestRolesItems::Judge,
                "legislatorLowerBody" => ContestRolesItems::LegislatorLowerBody,
                "legislatorUpperBody" => ContestRolesItems::LegislatorUpperBody,
                "otherRole" => ContestRolesItems::OtherRole,
                "schoolBoard" => ContestRolesItems::SchoolBoard,
                "specialPurposeOfficer" => ContestRolesItems::SpecialPurposeOfficer,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ContestRolesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ContestRolesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ContestRolesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "deputyHeadOfGovernment" => ContestRolesItems::DeputyHeadOfGovernment,
                "executiveCouncil" => ContestRolesItems::ExecutiveCouncil,
                "governmentOfficer" => ContestRolesItems::GovernmentOfficer,
                "headOfGovernment" => ContestRolesItems::HeadOfGovernment,
                "headOfState" => ContestRolesItems::HeadOfState,
                "highestCourtJudge" => ContestRolesItems::HighestCourtJudge,
                "judge" => ContestRolesItems::Judge,
                "legislatorLowerBody" => ContestRolesItems::LegislatorLowerBody,
                "legislatorUpperBody" => ContestRolesItems::LegislatorUpperBody,
                "otherRole" => ContestRolesItems::OtherRole,
                "schoolBoard" => ContestRolesItems::SchoolBoard,
                "specialPurposeOfficer" => ContestRolesItems::SpecialPurposeOfficer,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ContestRolesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContestRolesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DivisionSearchResponse {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string “civicinfo#divisionSearchResponse”."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[serde(
            rename = "results",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub results: ::std::option::Option<Vec<crate::schemas::DivisionSearchResult>>,
    }
    impl ::google_field_selector::FieldSelector for DivisionSearchResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DivisionSearchResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DivisionSearchResult {
        #[doc = "Other Open Civic Data identifiers that refer to the same division – for example, those that refer to other political divisions whose boundaries are defined to be coterminous with this one. For example, ocd-division/country:us/state:wy will include an alias of ocd-division/country:us/state:wy/cd:1, since Wyoming has only one Congressional district."]
        #[serde(
            rename = "aliases",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aliases: ::std::option::Option<Vec<String>>,
        #[doc = "The name of the division."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The unique Open Civic Data identifier for this division"]
        #[serde(
            rename = "ocdId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ocd_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DivisionSearchResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DivisionSearchResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Election {
        #[doc = "Day of the election in YYYY-MM-DD format."]
        #[serde(
            rename = "electionDay",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub election_day: ::std::option::Option<String>,
        #[doc = "The unique ID of this election."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<i64>,
        #[doc = "A displayable name for the election."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The political division of the election. Represented as an OCD Division ID. Voters within these political jurisdictions are covered by this election. This is typically a state such as ocd-division/country:us/state:ca or for the midterms or general election the entire US (i.e. ocd-division/country:us)."]
        #[serde(
            rename = "ocdDivisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ocd_division_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Election {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Election {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ElectionOfficial {
        #[doc = "The email address of the election official."]
        #[serde(
            rename = "emailAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email_address: ::std::option::Option<String>,
        #[doc = "The fax number of the election official."]
        #[serde(
            rename = "faxNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fax_number: ::std::option::Option<String>,
        #[doc = "The full name of the election official."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The office phone number of the election official."]
        #[serde(
            rename = "officePhoneNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub office_phone_number: ::std::option::Option<String>,
        #[doc = "The title of the election official."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ElectionOfficial {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ElectionOfficial {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ElectionsQueryResponse {
        #[doc = "A list of available elections"]
        #[serde(
            rename = "elections",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub elections: ::std::option::Option<Vec<crate::schemas::Election>>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string “civicinfo#electionsQueryResponse”."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ElectionsQueryResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ElectionsQueryResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ElectoralDistrict {
        #[doc = "An identifier for this district, relative to its scope. For example, the 34th State Senate district would have id “34” and a scope of stateUpper."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "The name of the district."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The geographic scope of this district. If unspecified the district’s geography is not known. One of: national, statewide, congressional, stateUpper, stateLower, countywide, judicial, schoolBoard, cityWide, township, countyCouncil, cityCouncil, ward, special"]
        #[serde(
            rename = "scope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scope: ::std::option::Option<crate::schemas::ElectoralDistrictScope>,
    }
    impl ::google_field_selector::FieldSelector for ElectoralDistrict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ElectoralDistrict {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ElectoralDistrictScope {
        CityCouncil,
        Citywide,
        Congressional,
        CountyCouncil,
        Countywide,
        Judicial,
        National,
        SchoolBoard,
        Special,
        StateLower,
        StateUpper,
        Statewide,
        Township,
        Ward,
    }
    impl ElectoralDistrictScope {
        pub fn as_str(self) -> &'static str {
            match self {
                ElectoralDistrictScope::CityCouncil => "cityCouncil",
                ElectoralDistrictScope::Citywide => "citywide",
                ElectoralDistrictScope::Congressional => "congressional",
                ElectoralDistrictScope::CountyCouncil => "countyCouncil",
                ElectoralDistrictScope::Countywide => "countywide",
                ElectoralDistrictScope::Judicial => "judicial",
                ElectoralDistrictScope::National => "national",
                ElectoralDistrictScope::SchoolBoard => "schoolBoard",
                ElectoralDistrictScope::Special => "special",
                ElectoralDistrictScope::StateLower => "stateLower",
                ElectoralDistrictScope::StateUpper => "stateUpper",
                ElectoralDistrictScope::Statewide => "statewide",
                ElectoralDistrictScope::Township => "township",
                ElectoralDistrictScope::Ward => "ward",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ElectoralDistrictScope {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ElectoralDistrictScope {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ElectoralDistrictScope, ()> {
            Ok(match s {
                "cityCouncil" => ElectoralDistrictScope::CityCouncil,
                "citywide" => ElectoralDistrictScope::Citywide,
                "congressional" => ElectoralDistrictScope::Congressional,
                "countyCouncil" => ElectoralDistrictScope::CountyCouncil,
                "countywide" => ElectoralDistrictScope::Countywide,
                "judicial" => ElectoralDistrictScope::Judicial,
                "national" => ElectoralDistrictScope::National,
                "schoolBoard" => ElectoralDistrictScope::SchoolBoard,
                "special" => ElectoralDistrictScope::Special,
                "stateLower" => ElectoralDistrictScope::StateLower,
                "stateUpper" => ElectoralDistrictScope::StateUpper,
                "statewide" => ElectoralDistrictScope::Statewide,
                "township" => ElectoralDistrictScope::Township,
                "ward" => ElectoralDistrictScope::Ward,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ElectoralDistrictScope {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ElectoralDistrictScope {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ElectoralDistrictScope {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "cityCouncil" => ElectoralDistrictScope::CityCouncil,
                "citywide" => ElectoralDistrictScope::Citywide,
                "congressional" => ElectoralDistrictScope::Congressional,
                "countyCouncil" => ElectoralDistrictScope::CountyCouncil,
                "countywide" => ElectoralDistrictScope::Countywide,
                "judicial" => ElectoralDistrictScope::Judicial,
                "national" => ElectoralDistrictScope::National,
                "schoolBoard" => ElectoralDistrictScope::SchoolBoard,
                "special" => ElectoralDistrictScope::Special,
                "stateLower" => ElectoralDistrictScope::StateLower,
                "stateUpper" => ElectoralDistrictScope::StateUpper,
                "statewide" => ElectoralDistrictScope::Statewide,
                "township" => ElectoralDistrictScope::Township,
                "ward" => ElectoralDistrictScope::Ward,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ElectoralDistrictScope {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ElectoralDistrictScope {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FeatureIdProto {
        #[doc = "The S2CellId corresponding to the approximate location of this feature as of when it was first created. This can be of variable accuracy, ranging from the exact centroid of the feature at creation, a very large S2 Cell, or even being completely randomized for locationless features. Cell ids have the nice property that they follow a space-filling curve over the surface of the earth. (See s2cellid.h for details.) WARNING: Clients should only use cell IDs to perform spatial locality optimizations. There is no strict guarantee that the cell ID of a feature is related to the current geometry of the feature in any way."]
        #[serde(
            rename = "cellId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub cell_id: ::std::option::Option<u64>,
        #[doc = "A 64-bit fingerprint used to identify features. Most clients should rely on MapFacts or OneRing to choose fingerprints. If creating new fprints, the strategy should be chosen so that the chance of collision is remote or non-existent, and the distribution should be reasonably uniform. For example, if the source data assigns unique ids to features, then a fingerprint of the provider name, version, and source id is sufficient."]
        #[serde(
            rename = "fprint",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub fprint: ::std::option::Option<u64>,
        #[doc = "A place for clients to attach arbitrary data to a feature ID. Never set in MapFacts."]
        #[serde(
            rename = "temporaryData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub temporary_data: ::std::option::Option<crate::schemas::MessageSet>,
    }
    impl ::google_field_selector::FieldSelector for FeatureIdProto {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FeatureIdProto {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GeocodingSummary {
        #[doc = "Represents the best estimate of whether or not the input address was fully understood and the address is correctly componentized. Mirrors the same-name field in geostore.staging.AddressLinkupScoringProto."]
        #[serde(
            rename = "addressUnderstood",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address_understood: ::std::option::Option<bool>,
        #[doc = "The ID of the FeatureProto returned by the geocoder"]
        #[serde(
            rename = "featureId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feature_id: ::std::option::Option<crate::schemas::FeatureIdProto>,
        #[doc = "The feature type for the FeatureProto returned by the geocoder"]
        #[serde(
            rename = "featureType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feature_type: ::std::option::Option<crate::schemas::GeocodingSummaryFeatureType>,
        #[doc = "Precision of the center point (lat/long) of the geocoded FeatureProto"]
        #[serde(
            rename = "positionPrecisionMeters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub position_precision_meters: ::std::option::Option<f64>,
        #[doc = "The query sent to the geocoder"]
        #[serde(
            rename = "queryString",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query_string: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GeocodingSummary {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GeocodingSummary {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GeocodingSummaryFeatureType {
        #[doc = "An address template feature provides region-specific conventions for structuring addresses. These features aren’t necessarily defined by physical geographic features, so they are classified as meta-features."]
        TypeAddressTemplate,
        #[doc = "ABSTRACT"]
        TypeAdministrativeArea,
        TypeAdministrativeArea1,
        TypeAdministrativeArea2,
        TypeAdministrativeArea3,
        TypeAdministrativeArea4,
        TypeAdministrativeArea5,
        TypeAdministrativeArea6,
        TypeAdministrativeArea7,
        TypeAdministrativeArea8,
        TypeAdministrativeArea9,
        TypeAgricultural,
        #[doc = "DEPRECATED"]
        TypeAirport,
        #[doc = "DEPRECATED"]
        TypeAirportCivil,
        #[doc = "DEPRECATED"]
        TypeAirportGrounds,
        #[doc = "DEPRECATED"]
        TypeAirportMilitary,
        #[doc = "DEPRECATED"]
        TypeAirportMixed,
        #[doc = "DEPRECATED"]
        TypeAirstrip,
        #[doc = "Subtype within a zoo - a cage or fenced-off or otherwise delineated area containing animals."]
        TypeAnimalEnclosure,
        #[doc = "ABSTRACT"]
        TypeAny,
        #[doc = "A feature representing a group or chain of islands. "]
        TypeArchipelago,
        TypeAtoll,
        #[doc = "DEPRECATED"]
        TypeBank,
        #[doc = "DEPRECATED"]
        TypeBar,
        #[doc = "An ocean subdivision formed by a coastal indentation. Includes coves and gulfs."]
        TypeBay,
        TypeBeach,
        #[doc = "A designated bicycle route, whose segments may consist of any combination of bicycle paths, bicycle lanes, or city streets."]
        TypeBicycleRoute,
        #[doc = "An open body of water formed by a slight coastal indentation."]
        TypeBight,
        #[doc = "DEPRECATED"]
        TypeBirdWatching,
        #[doc = "A line representing the boundary between two features. See border.proto for details."]
        TypeBorder,
        #[doc = "DEPRECATED"]
        TypeBorderCrossing,
        TypeBroadTrack,
        #[doc = "DEPRECATED"]
        TypeBuilding,
        #[doc = "DEPRECATED"]
        TypeBuildingGrounds,
        #[doc = "Terrain that looks populated."]
        TypeBuiltUpArea,
        #[doc = "DEPRECATED"]
        TypeBusStation,
        #[doc = "DEPRECATED"]
        TypeBusiness,
        #[doc = "A business chain feature is used to represent a chain, e.g. Starbucks, McDonald’s, etc. Other features representing specific stores/franchises of this chain may refer to one such feature via RELATION_MEMBER_OF_CHAIN. This is not strictly reserved to commercial chains but can also be used to model organizations such as the Red Cross or the United Nations."]
        TypeBusinessChain,
        #[doc = "A Business Corridor is a dense cluster of semantically similar establishments. TYPE_BUSINESS_CORRIDOR features are distinguished from TYPE_COLLOQUIAL_AREA features because the corridors are not under the political hierarchy, are allowed to be nameless, and may not correspond to well-known real world locations. For more details, see go/geo-corridors-schema."]
        TypeBusinessCorridor,
        #[doc = "DEPRECATED"]
        TypeCableCarStation,
        TypeCampfirePit,
        #[doc = "DEPRECATED"]
        TypeCampgrounds,
        #[doc = "DEPRECATED"]
        TypeCampingSite,
        #[doc = "A narrow passage used by boats. Normally artificial."]
        TypeCanal,
        #[doc = "DEPRECATED"]
        TypeCarRental,
        #[doc = "DEPRECATED"]
        TypeCarRepair,
        #[doc = "Cartographic features are used to capture real-world objects for which there is no current desire to model any specific attributes. These are only useful to make the map tiles look pretty."]
        TypeCartographic,
        #[doc = "A line for a cartographic detail. For example the international date line. Such features should have polyline geometry."]
        TypeCartographicLine,
        #[doc = "DEPRECATED"]
        TypeCashMachine,
        #[doc = "Don’t use 0xA7. Use 8 bits for additional types under TYPE_NATURAL_FEATURE, so we don’t run out of space. The following are miscellaneous natural features that don’t fit any of the categories above."]
        TypeCave,
        #[doc = "A small, low-elevation, sandy island formed on the surface of coral reefs"]
        TypeCay,
        #[doc = "The root of types of features that are in the sky, rather than on the earth. There will eventually be a hierarchy of types here."]
        TypeCelestial,
        #[doc = "DEPRECATED"]
        TypeCemetery,
        #[doc = "A deep part in a body of water that is suitable for navigation. Includes narrows."]
        TypeChannel,
        #[doc = "DEPRECATED"]
        TypeChurch,
        #[doc = "DEPRECATED"]
        TypeCityHall,
        #[doc = "A vertical or nearly vertical slope. Includes escarpments."]
        TypeCliff,
        #[doc = "DEPRECATED"]
        TypeCoffee,
        #[doc = "e.g. Silicon Valley"]
        TypeColloquialArea,
        #[doc = "An entity widely considered to be a city, that may itself be made up of smaller political entities, some of which are cities/towns/villages themselves. For example, the colloquial view of Sydney, Australia actually comprises many smaller cities, but is regarded as a city itself. This type is not suitable for modeling official metro-/micropolitan or other statistical areas."]
        TypeColloquialCity,
        #[doc = "ABSTRACT"]
        TypeCompound,
        #[doc = "e.g. single family dwelling, office building."]
        TypeCompoundBuilding,
        #[doc = "e.g. campus, compound, parcel."]
        TypeCompoundGrounds,
        #[doc = "e.g. suite, room, hallway, cubicle."]
        TypeCompoundSection,
        #[doc = "A place where two or more rivers join."]
        TypeConfluence,
        TypeConstituency,
        #[doc = "RESERVED"]
        TypeConstituencyFuture,
        TypeContinent,
        #[doc = "All the points on the polygon are at the same elevation."]
        TypeContourLine,
        TypeCountry,
        #[doc = "DEPRECATED"]
        TypeCourthouse,
        #[doc = "Depressions causes by impact, explosion, and sometimes sink-holes."]
        TypeCrater,
        #[doc = "Includes overfalls."]
        TypeCurrent,
        #[doc = "DEPRECATED"]
        TypeDam,
        #[doc = "Every data source used in constructing a data repository has a corresponding feature that provides more information about that data source. The extra information is stored in the optional data_source field below."]
        TypeDataSource,
        #[doc = "DEPRECATED"]
        TypeDentist,
        #[doc = "DEPRECATED"]
        TypeDeprecatedGolfShop,
        #[doc = "DEPRECATED"]
        TypeDeprecatedHighwayDoNotUse,
        #[doc = "DEPRECATED"]
        TypeDeprecatedTarmac,
        TypeDesert,
        TypeDesignatedBarbecuePit,
        TypeDesignatedCookingArea,
        #[doc = "Designated Market Areas (or DMAs) are used by marketing and ratings companies (such as the Nielsen Media Research company) to describe geographical regions (such as the greater New York metropolitan area) that are covered by a set of television stations. (See http://www.schooldata.com/pdfs/DMA.pdf) In the United States, DMAs should have a DMA numeric ID name, tagged with the FLAG_DESIGNATED_MARKET_AREA_ID flag."]
        TypeDesignatedMarketArea,
        #[doc = "RESERVED"]
        TypeDigitalElevationModel,
        #[doc = "Eventually we’ll have more data for disputed areas (e.g., who makes claims on the area, who has de facto control, etc.). For the moment, we just define a type so we can simply mark areas as disputed."]
        TypeDisputedArea,
        #[doc = "A branch which flows away from the main river. Includes deltas."]
        TypeDistributary,
        #[doc = "DEPRECATED"]
        TypeDoNotUseReservedToCatchGeneratedFiles,
        #[doc = "DEPRECATED"]
        TypeDoctor,
        #[doc = "DEPRECATED"]
        TypeDoodle,
        TypeDrinkingWater,
        TypeDune,
        #[doc = "DEPRECATED"]
        TypeEarthquake,
        #[doc = "DEPRECATED"]
        TypeEcoTouristDestination,
        #[doc = "Features that are notable for being high (or low), or for having sudden changes in elevation. These features might have an “elevation” extension to specify the actual elevation. See ElevationProto for more information."]
        TypeElevated,
        #[doc = "DEPRECATED"]
        TypeEmbassy,
        #[doc = "DEPRECATED"]
        TypeEmergency,
        #[doc = "DEPRECATED"]
        TypeEnclosedTrafficArea,
        #[doc = "A portal of entry or exit to another feature. Examples: - Subway station entrance. - Parking lot entrance."]
        TypeEntrance,
        #[doc = "ABSTRACT This type is being replaced by TYPE_COMPOUND_GROUNDS. For further details, see go/compounds-v2"]
        TypeEstablishment,
        #[doc = "DEPRECATED"]
        TypeEstablishmentBuilding,
        #[doc = "DEPRECATED This type has been replaced by TYPE_COMPOUND_BUILDING. For further details, see go/oyster-compounds"]
        TypeEstablishmentGrounds,
        #[doc = "Establishment POIs can be referenced by TYPE_COMPOUND features using the RELATION_PRIMARILY_OCCUPIED_BY. This is the reciprocal relation of the RELATION_OCCUPIES."]
        TypeEstablishmentPoi,
        #[doc = "Represents service-only establishments (those without a storefront location). NOTE(tcain): Using value 0xD441, since we could find ourselves with a need to differentiate service areas from online-only at this level in the future, but still benefit from being able to group those under a common parent, disjoint from TYPE_ESTABLISHMENT_POI."]
        TypeEstablishmentService,
        #[doc = "A place at the end of a river where fresh and salt water mix. Includes tidal creeks and limans."]
        TypeEstuary,
        #[doc = "DEPRECATED"]
        TypeEvent,
        TypeFault,
        #[doc = "ABSTRACT"]
        TypeFerry,
        #[doc = "The vast majority of ferries are ferry boats."]
        TypeFerryBoat,
        #[doc = "DEPRECATED"]
        TypeFerryTerminal,
        #[doc = "Also called a “car transport”, a ferry train is a rail service that carries passengers and their vehicles across undrivable terrain. The Channel Tunnel (“Chunnel”) is the most famous example, but they are also common in the Alps where they connect neighboring valleys otherwise separated by impassable mountains."]
        TypeFerryTrain,
        #[doc = "DEPRECATED"]
        TypeFire,
        #[doc = "DEPRECATED"]
        TypeFishing,
        TypeFissure,
        TypeFjord,
        #[doc = "A shallow place where water may be waded through."]
        TypeFord,
        #[doc = "DEPRECATED"]
        TypeFunicularStation,
        #[doc = "A feature whose geometry is planned to replace the geometry on another feature."]
        TypeFutureGeometry,
        #[doc = "DEPRECATED"]
        TypeGasStation,
        #[doc = "DEPRECATED"]
        TypeGbCountry,
        #[doc = "DEPRECATED"]
        TypeGbDependentLocality,
        #[doc = "DEPRECATED"]
        TypeGbDoubleDependentLocality,
        #[doc = "DEPRECATED"]
        TypeGbFormerPostalCounty,
        #[doc = "DEPRECATED"]
        TypeGbPostTown,
        #[doc = "DEPRECATED"]
        TypeGbTraditionalCounty,
        #[doc = "An association of a point with an address, with no other information."]
        TypeGeocodedAddress,
        TypeGeyser,
        TypeGlacier,
        #[doc = "DEPRECATED"]
        TypeGolf,
        #[doc = "DEPRECATED"]
        TypeGolfCourse,
        TypeGolfFairway,
        #[doc = "Use TYPE_ESTABLISHMENT_POI and gcid:golf_shop for golf shops instead."]
        TypeGolfHole,
        TypeGolfPuttingGreen,
        TypeGolfRough,
        TypeGolfSandBunker,
        #[doc = "Sub-types within a golf course."]
        TypeGolfTeeingGround,
        #[doc = "DEPRECATED"]
        TypeGondolaLiftStation,
        #[doc = "DEPRECATED"]
        TypeGovernment,
        TypeGrassland,
        #[doc = "DEPRECATED"]
        TypeGrocery,
        #[doc = "DEPRECATED"]
        TypeGrounds,
        #[doc = "DEPRECATED"]
        TypeGurudwara,
        #[doc = "A deep place near a shore where ships commonly drop anchor."]
        TypeHarbor,
        #[doc = "DEPRECATED"]
        TypeHeliport,
        TypeHighSpeedRail,
        #[doc = "DEPRECATED"]
        TypeHighTension,
        #[doc = "ABSTRACT"]
        TypeHighway,
        TypeHighway1,
        TypeHighway2,
        TypeHighway3,
        TypeHighway4,
        TypeHighway5,
        TypeHighway6,
        TypeHighway7,
        TypeHighway8,
        TypeHighway9,
        #[doc = "DEPRECATED"]
        TypeHikingArea,
        #[doc = "DEPRECATED"]
        TypeHinduTemple,
        #[doc = "DEPRECATED"]
        TypeHorseCarriageStation,
        #[doc = "DEPRECATED"]
        TypeHospital,
        #[doc = "DEPRECATED"]
        TypeHospitalGrounds,
        TypeHotSpring,
        #[doc = "DEPRECATED"]
        TypeHunting,
        #[doc = "DEPRECATED"]
        TypeHurricane,
        TypeIce,
        #[doc = "DEPRECATED"]
        TypeIndustrial,
        TypeInlet,
        #[doc = "An intersection consists of a collection of segments that terminate at the same location. This is topological definition: it may not match what a typical user would think of as an “intersection”. See TYPE_INTERSECTION_GROUP, below, for more information. Each segment terminating at an intersection has an “endpoint type” that specifies how that segment is terminated: stop sign, yield sign, three-way light, etc."]
        TypeIntersection,
        #[doc = "Our TYPE_INTERSECTION feature, above, models the point where one or more segments terminate. This is topological definition: it may not match what a typical user would think of as an “intersection”. Consider the intersections where Hayes, Market, Larkin, and 9th Street meet near (37.77765, -122.41638) in San Francisco. Most people would probably consider this a single feature, even though we model it as four separate TYPE_INTERSECTION features. This TYPE_INTERSECTION_GROUP is used to model the user’s concept of a complex intersection."]
        TypeIntersectionGroup,
        #[doc = "Man-made (and sometimes natural) channels used to move water. This type was used for both dam structures and water that is hold back by dams. We should use TYPE_COMPOUND_BUILDING for dam structures and TYPE_RESERVOIR for water."]
        TypeIrrigation,
        TypeIsland,
        #[doc = "A strip of land connecting two larger land masses, such as continents."]
        TypeIsthmus,
        #[doc = "DEPRECATED"]
        TypeJpChiban,
        #[doc = "DEPRECATED"]
        TypeJpEdaban,
        #[doc = "DEPRECATED"]
        TypeJpGaiku,
        #[doc = "DEPRECATED"]
        TypeJpGun,
        #[doc = "DEPRECATED"]
        TypeJpKoaza,
        #[doc = "DEPRECATED"]
        TypeJpOoaza,
        #[doc = "DEPRECATED"]
        TypeJpShikuchouson,
        #[doc = "DEPRECATED"]
        TypeJpSubShikuchouson,
        #[doc = "DEPRECATED"]
        TypeJpTodoufuken,
        TypeJrTrack,
        #[doc = "Topography formed on limestone and gypsum by dissolution with sinkholes, caves, etc."]
        TypeKarst,
        TypeLagoon,
        #[doc = "An inland body of standing water."]
        TypeLake,
        TypeLandMass,
        TypeLandParcel,
        TypeLavaField,
        #[doc = "A feature used to represent a logical level, e.g. floor."]
        TypeLevel,
        #[doc = "DEPRECATED"]
        TypeLibrary,
        TypeLightRailTrack,
        TypeLitterReceptacle,
        #[doc = "DEPRECATED"]
        TypeLocalPark,
        #[doc = "A locale feature provides region specific conventions such as preferred language and formatting details for time, date, and currency values. Locales aren’t necessary defined by physical geographic features, so they are classified as meta-features."]
        TypeLocale,
        TypeLocality,
        TypeLockerArea,
        #[doc = "DEPRECATED"]
        TypeLodging,
        #[doc = "ABSTRACT"]
        TypeMetaFeature,
        #[doc = "DEPRECATED"]
        TypeMilitary,
        #[doc = "DEPRECATED"]
        TypeMonorailStation,
        TypeMonorailTrack,
        #[doc = "DEPRECATED"]
        TypeMosque,
        #[doc = "A series of mountains or hills ranged in a line and connected by high ground. Mountain ranges usually consist of many smaller ridges. For example, the Himalayas, the Andes. the Alps, etc."]
        TypeMountainRange,
        #[doc = "DEPRECATED"]
        TypeMovieRental,
        TypeNarrowTrack,
        #[doc = "DEPRECATED"]
        TypeNationalForest,
        #[doc = "DEPRECATED"]
        TypeNationalPark,
        #[doc = "ABSTRACT"]
        TypeNaturalFeature,
        #[doc = "DEPRECATED"]
        TypeNatureReserve,
        TypeNeighborhood,
        #[doc = "A peak or ridge of a mountain that extends through a glacier."]
        TypeNunatak,
        #[doc = "One of the large salt-water bodies that covers most of the globe."]
        TypeOcean,
        #[doc = "An exposed rock in the water."]
        TypeOceanRockExposed,
        #[doc = "DEPRECATED"]
        TypeOffRoadArea,
        #[doc = "A near-level shallow, natural depression or basin, usually containing an intermittent lake, pond, or pool."]
        TypePan,
        #[doc = "DEPRECATED"]
        TypePark,
        #[doc = "DEPRECATED"]
        TypeParking,
        #[doc = "DEPRECATED"]
        TypeParkingGarage,
        #[doc = "DEPRECATED"]
        TypeParkingLot,
        #[doc = "A route over an otherwise difficult to traverse feature. Includes saddle."]
        TypePass,
        #[doc = "RESERVED"]
        TypePathway,
        #[doc = "Elevations that have a distinctive peak."]
        TypePeak,
        #[doc = "A stretch of land projecting into water. Includes capes and spits."]
        TypePeninsula,
        #[doc = "DEPRECATED"]
        TypePharmacy,
        #[doc = "A phone number area code is a prefix which also coincides with the area code, or national destination code, of a particular region."]
        TypePhoneNumberAreaCode,
        #[doc = "A phone number prefix feature is used to specify the region where phone numbers (typically fixed-line numbers) must begin with a certain prefix. Any phone number prefix down to any level of granularity could be represented by this type."]
        TypePhoneNumberPrefix,
        #[doc = "DEPRECATED"]
        TypePicnicArea,
        #[doc = "Elevations that are flat on top. Includes mesas and buttes."]
        TypePlateau,
        #[doc = "DEPRECATED"]
        TypePlayGround,
        #[doc = "DEPRECATED"]
        TypePolice,
        #[doc = "Boundaries representing the jurisdiction of a particular police station."]
        TypePoliceJurisdiction,
        #[doc = "ABSTRACT"]
        TypePolitical,
        TypePond,
        #[doc = "DEPRECATED"]
        TypePostOffice,
        #[doc = "The term “post town” is used for a locality-like-entity that is only used for postal addresses."]
        TypePostTown,
        #[doc = "ABSTRACT"]
        TypePostal,
        #[doc = "This is the type for postal codes which are complete and independent enough that there should be a feature for them (e.g. US 5-digit ZIP codes). For even more detailed suffixes that further subdivide a postal code (such as the +4 component in US ZIP codes), store the information in a TYPE_POSTAL_CODE_SUFFIX address component. When a range or set of postal codes share the same geographical area, e.g. because a precise subdivision does not exist or this subdivision is unknown, this type is used for each individual postal code."]
        TypePostalCode,
        #[doc = "A prefix portion of a postal code which does not meet the requirements for TYPE_POSTAL_CODE, but which is useful to search for, for example UK outcodes."]
        TypePostalCodePrefix,
        #[doc = "DEPRECATED"]
        TypePostalRound,
        #[doc = "DEPRECATED"]
        TypePremise,
        #[doc = "DEPRECATED"]
        TypeProvincialForest,
        #[doc = "DEPRECATED"]
        TypeProvincialPark,
        #[doc = "ABSTRACT"]
        TypePublicSpacesAndMonuments,
        #[doc = "Railroads use several different incompatible track types."]
        TypeRailway,
        TypeRapids,
        #[doc = "Steep declines usually carved by erosion. Includes valleys, canyons, ditches, and gorges."]
        TypeRavine,
        #[doc = "Rocks, coral, sandbars, or other features beneath the surface of the water that pose a hazard to passing ships. Includes shoals."]
        TypeReef,
        #[doc = "The full extent of the reef complex."]
        TypeReefExtent,
        #[doc = "A relatively shallow zone of the back reef located closest to the shore, that may be exposed at low tide."]
        TypeReefFlat,
        #[doc = "A small section of rocks, coral, sandbars, or other features beneath the surface of the water that forms part of a reef."]
        TypeReefGrowth,
        #[doc = "A submerged rock in the water."]
        TypeReefRockSubmerged,
        #[doc = "An area controlled in some way by an authoritative source, such as a government-designated COVID containment zone. Features of this type should have one or more gcids corresponding to their specific regulation."]
        TypeRegulatedArea,
        #[doc = "A reservation is a region collectively held or governed by indigenous people and officially recognized by the country’s government at the federal or state level. A reservation may be fully contained within an administrative feature or partially contained within two or more. These regions are referred to by different categorical names depending on country and even by state, including but not limited to: “Indian Reservations”, “Indian Reserves”, “Land Claim Settlement Lands”, “Indian Lands”, “Treaty Lands”, “Indigenous Territories”, etc. A reservation is not a historic indigenous territory boundary or a region which has applied for land rights but has not yet received official recognition."]
        TypeReservation,
        #[doc = "An artificial body of water, possibly created by a dam, often used for irrigation or house use."]
        TypeReservoir,
        #[doc = "DEPRECATED"]
        TypeRestArea,
        #[doc = "DEPRECATED"]
        TypeRestaurant,
        #[doc = "A restriction group describes a set of segment restrictions that belong together and have a name or an associated event. See also restriction_group.proto"]
        TypeRestrictionGroup,
        #[doc = "A ridge is a geographical feature consisting of a chain of mountains or hills that form a continuous elevated crest with a single ridgeline for some distance."]
        TypeRidge,
        #[doc = "An inland body of moving water, or parts associated with it in which there is little or no current (backwater)."]
        TypeRiver,
        TypeRoad,
        #[doc = "Features responsible for monitoring traffic on roads (usually for speed). Includes cameras at particular points as well as monitors that cover larger spans. Features of this type should have a corresponding gcid that specifies the correct subtype (e.g. gcid:road_camera or gcid:speed_camera_zone). This type was originally named as TYPE_ROAD_CAMERA."]
        TypeRoadMonitor,
        #[doc = "Road sign features have names, point geometry, etc. They also have segment_path data (see below) which lists the segments that refer to the sign. See segment.proto for the reference from the segment to the road sign."]
        TypeRoadSign,
        TypeRock,
        TypeRocky,
        #[doc = "A route is any section of road (or rails, etc.) that has a name. This includes city streets as well as highways. Road segments can belong to multiple routes (e.g. El Camino, CA-82)."]
        TypeRoute,
        #[doc = "A flat expanse of salt left by the evaporation of a body of salt water."]
        TypeSaltFlat,
        TypeSand,
        #[doc = "DEPRECATED"]
        TypeSchool,
        TypeSchoolDistrict,
        #[doc = "An ocean subdivision more or less confined by land and islands."]
        TypeSea,
        #[doc = "DEPRECATED"]
        TypeSeaplaneBase,
        #[doc = "DEPRECATED"]
        TypeSeaport,
        #[doc = "A lake that dries up part of the year."]
        TypeSeasonalLake,
        #[doc = "A river that dries up part of the year."]
        TypeSeasonalRiver,
        #[doc = "ABSTRACT"]
        TypeSegment,
        #[doc = "ABSTRACT"]
        TypeSegmentPath,
        #[doc = "DEPRECATED"]
        TypeShopping,
        #[doc = "DEPRECATED"]
        TypeShoppingCenter,
        TypeShrubbery,
        #[doc = "Also see skiboundary.proto"]
        TypeSkiBoundary,
        #[doc = "Also see skilift.proto"]
        TypeSkiLift,
        #[doc = "Also see skitrail.proto"]
        TypeSkiTrail,
        #[doc = "Land not so steep as a cliff, but changing elevation. Includes slides."]
        TypeSlope,
        #[doc = "DEPRECATED"]
        TypeSpecialStation,
        #[doc = "DEPRECATED"]
        TypeSportsComplex,
        #[doc = "A place where ground water flows naturally out of the ground."]
        TypeSpring,
        #[doc = "A subsidiary peak of a mountain."]
        TypeSpur,
        #[doc = "DEPRECATED"]
        TypeStadium,
        TypeStandardTrack,
        #[doc = "An area used for aggregating statistical data, eg, a census region. Note that TYPE_STATISTICAL_AREA has a third nibble so we can add an abstract parent above it later if need be at 0x2E1 (and rename TYPE_STATISTICAL_AREA as TYPE_STATISTICAL_AREA1)."]
        TypeStatisticalArea,
        #[doc = "Note that this type does not distinguish the nature of the statue (religious, historical, memorial, tourist, …)."]
        TypeStatue,
        #[doc = "A long narrow ocean subdivision. Includes sounds."]
        TypeStrait,
        #[doc = "DEPRECATED This is deprecated and we want to use TYPE_COMPOUND_SECTION instead."]
        TypeSubPremise,
        #[doc = "ABSTRACT"]
        TypeSublocality,
        TypeSublocality1,
        TypeSublocality2,
        TypeSublocality3,
        TypeSublocality4,
        TypeSublocality5,
        TypeSubmarineBasin,
        TypeSubmarineCliff,
        TypeSubmarineDeep,
        TypeSubmarineFractureZone,
        #[doc = "includes saddles"]
        TypeSubmarineGap,
        TypeSubmarinePlain,
        TypeSubmarinePlateau,
        TypeSubmarineRidge,
        #[doc = "includes peaks, ranges, and spurs"]
        TypeSubmarineSeamount,
        TypeSubmarineSlope,
        #[doc = "includes trenches and troughs"]
        TypeSubmarineValley,
        #[doc = "DEPRECATED"]
        TypeSubwayStation,
        TypeSubwayTrack,
        #[doc = "DEPRECATED"]
        TypeSuite,
        #[doc = "DEPRECATED"]
        TypeSynagogue,
        #[doc = "Starting with TYPE_TARMAC, we use longer IDs, so that we can expand the number of feature types under TYPE_CARTOGRAPHIC."]
        TypeTarmac,
        #[doc = "ABSTRACT This type is incorrectly under TYPE_TECTONIC instead of TYPE_WATER. This was a mistake and is now fixed. See TYPE_WATERING_HOLE for the replacement."]
        TypeTectonic,
        #[doc = "DEPRECATED"]
        TypeTemple,
        #[doc = "A terminal point represents a good location for a user to meet a taxi, ridesharing vehicle, or general driver."]
        TypeTerminalPoint,
        TypeTerrace,
        #[doc = "Expanses of land that share common surface attributes. These areas would look more or less uniform from a high altitude."]
        TypeTerrain,
        #[doc = "A timezone feature is used to specify the region covering an international timezone. When a point is covered by multiple timezone features, the most specific one can be used to compute the local time at this point. Most specific implies a much smaller region or the one that is closer to the center. A feature’s timezone can be specified in the repeated related_timezone field."]
        TypeTimezone,
        #[doc = "A toll cluster is either a single point on a segment (represented as a point at the end of the segment that has ENDPOINT_TOLL_BOOTH set) or a group of points on various road segments in MapFacts that represents one or more lanes passing through a toll fixture that all go to the same routing destination. Each toll cluster should have at most a single price per payment method. E.g. {CASH = $5, PASS = $1}. Note: If a toll fixture has different prices for multiple routing destinations, drivers need to be in the correct lane before passing through the toll fixture and hence such a fixture is represented by multiple toll clusters. A toll cluster does not necessarily represent a real-world entity, e.g. a particular plaza/structure as perceived by humans. This is because a plaza can be represented by more than one toll cluster. We require toll clusters to have names, but they might be non-unique. For example, a plaza might be represented by multiple toll clusters that may have the same plaza name. For further details, please see go/toll-cluster-schema."]
        TypeTollCluster,
        #[doc = "DEPRECATED"]
        TypeTouristDestination,
        #[doc = "Open space used for events, gathering, or as market-place."]
        TypeTownSquare,
        #[doc = "A designated trail, which may consist of paved walkways, dirt paths, fire road, streets or highways, etc."]
        TypeTrail,
        TypeTrailHead,
        #[doc = "DEPRECATED"]
        TypeTrainStation,
        #[doc = "DEPRECATED"]
        TypeTramwayStation,
        #[doc = "RESERVED"]
        TypeTransient,
        #[doc = "ABSTRACT"]
        TypeTransit,
        #[doc = "A transit agency operates a number of lines, typically all in the same city, region or country. See also transitagency.proto"]
        TypeTransitAgency,
        #[doc = "TYPE_TRANSIT_AGENCY was moved to 0xC91. This deprecated enum value still exists for debugging purposes only."]
        TypeTransitAgencyDeprecatedValue,
        #[doc = "DEPRECATED"]
        TypeTransitDeparture,
        #[doc = "DEPRECATED"]
        TypeTransitLeg,
        #[doc = "A transit line is a collection of transit legs, associated with some invariant properties of the trips that run over the legs. See also transitline.proto"]
        TypeTransitLine,
        #[doc = "DEPRECATED"]
        TypeTransitStation,
        #[doc = "DEPRECATED"]
        TypeTransitStop,
        #[doc = "DEPRECATED"]
        TypeTransitTransfer,
        #[doc = "DEPRECATED"]
        TypeTransitTrip,
        #[doc = "ABSTRACT"]
        TypeTransportation,
        #[doc = "DEPRECATED"]
        TypeTravelService,
        #[doc = "Tracks for streetcars, cable-cars, etc. Ferries are services that are part of the road network but are not roads. They typically involve fares and scheduled departure times."]
        TypeTrolleyTrack,
        TypeTundra,
        #[doc = "Features that are notable for being high (or low), or for having sudden changes in elevation. These features might have an “elevation” extension to specify the actual elevation. See ElevationProto for more information."]
        TypeUndersea,
        #[doc = "DEPRECATED"]
        TypeUniversity,
        #[doc = "DEPRECATED"]
        TypeUniversityGrounds,
        #[doc = "A feature of completely unknown type. This should only be used when absolutely necessary. One example in which this type is useful is in the Chinese importer, which must heuristically segment addresses into components - it often does not know what types to make those components. Please note that the Oyster address formatter does not currently support address components of TYPE_UNKNOWN well."]
        TypeUnknown,
        TypeUnstableHillside,
        #[doc = "Land along streams higher than the alluvial plain or stream terrace."]
        TypeUpland,
        #[doc = "DEPRECATED"]
        TypeUsBorough,
        #[doc = "DEPRECATED"]
        TypeUsNationalMonument,
        #[doc = "DEPRECATED"]
        TypeUsNationalPark,
        #[doc = "DEPRECATED"]
        TypeUsState,
        #[doc = "Terrain that is covered in vegetation."]
        TypeVegetation,
        #[doc = "DEPRECATED"]
        TypeVeterinarian,
        #[doc = "Any plausible 1-dimensional path through a 2+ dimensional space, for the purposes of making graph-search-based routing possible. Such segments can be used to model paths through parking lots, squares, floors of buildings and other areas."]
        TypeVirtualSegment,
        #[doc = "An elevated place that is notable for having a good view. Raster digital elevation data. This is not a type to be used by providers or consumed by clients."]
        TypeVista,
        TypeVolcano,
        #[doc = "A dry riverbed that occasionally receives flashfloods."]
        TypeWadi,
        #[doc = "Use TYPE_COMPOUND_GROUND and appropriate gcids for the next two."]
        TypeWall,
        #[doc = "Features can be TYPE_WATER if we don’t have enough information to properly type the body of water. TYPE_WATER is also used as the type for child features that compose a TYPE_RIVER feature."]
        TypeWater,
        TypeWaterFountain,
        TypeWaterNavigation,
        TypeWaterfall,
        #[doc = "A natural depression filled with water where animals come to drink."]
        TypeWateringHole,
        #[doc = "DEPRECATED"]
        TypeWateringHoleDeprecated,
        TypeWatershedBoundary,
        #[doc = "DEPRECATED"]
        TypeWeatherCondition,
        #[doc = "Land that is usually flooded. Includes bogs, marshes, flats, moors, and swamps."]
        TypeWetland,
        TypeWoods,
    }
    impl GeocodingSummaryFeatureType {
        pub fn as_str(self) -> &'static str {
            match self {
                GeocodingSummaryFeatureType::TypeAddressTemplate => "typeAddressTemplate",
                GeocodingSummaryFeatureType::TypeAdministrativeArea => "typeAdministrativeArea",
                GeocodingSummaryFeatureType::TypeAdministrativeArea1 => "typeAdministrativeArea1",
                GeocodingSummaryFeatureType::TypeAdministrativeArea2 => "typeAdministrativeArea2",
                GeocodingSummaryFeatureType::TypeAdministrativeArea3 => "typeAdministrativeArea3",
                GeocodingSummaryFeatureType::TypeAdministrativeArea4 => "typeAdministrativeArea4",
                GeocodingSummaryFeatureType::TypeAdministrativeArea5 => "typeAdministrativeArea5",
                GeocodingSummaryFeatureType::TypeAdministrativeArea6 => "typeAdministrativeArea6",
                GeocodingSummaryFeatureType::TypeAdministrativeArea7 => "typeAdministrativeArea7",
                GeocodingSummaryFeatureType::TypeAdministrativeArea8 => "typeAdministrativeArea8",
                GeocodingSummaryFeatureType::TypeAdministrativeArea9 => "typeAdministrativeArea9",
                GeocodingSummaryFeatureType::TypeAgricultural => "typeAgricultural",
                GeocodingSummaryFeatureType::TypeAirport => "typeAirport",
                GeocodingSummaryFeatureType::TypeAirportCivil => "typeAirportCivil",
                GeocodingSummaryFeatureType::TypeAirportGrounds => "typeAirportGrounds",
                GeocodingSummaryFeatureType::TypeAirportMilitary => "typeAirportMilitary",
                GeocodingSummaryFeatureType::TypeAirportMixed => "typeAirportMixed",
                GeocodingSummaryFeatureType::TypeAirstrip => "typeAirstrip",
                GeocodingSummaryFeatureType::TypeAnimalEnclosure => "typeAnimalEnclosure",
                GeocodingSummaryFeatureType::TypeAny => "typeAny",
                GeocodingSummaryFeatureType::TypeArchipelago => "typeArchipelago",
                GeocodingSummaryFeatureType::TypeAtoll => "typeAtoll",
                GeocodingSummaryFeatureType::TypeBank => "typeBank",
                GeocodingSummaryFeatureType::TypeBar => "typeBar",
                GeocodingSummaryFeatureType::TypeBay => "typeBay",
                GeocodingSummaryFeatureType::TypeBeach => "typeBeach",
                GeocodingSummaryFeatureType::TypeBicycleRoute => "typeBicycleRoute",
                GeocodingSummaryFeatureType::TypeBight => "typeBight",
                GeocodingSummaryFeatureType::TypeBirdWatching => "typeBirdWatching",
                GeocodingSummaryFeatureType::TypeBorder => "typeBorder",
                GeocodingSummaryFeatureType::TypeBorderCrossing => "typeBorderCrossing",
                GeocodingSummaryFeatureType::TypeBroadTrack => "typeBroadTrack",
                GeocodingSummaryFeatureType::TypeBuilding => "typeBuilding",
                GeocodingSummaryFeatureType::TypeBuildingGrounds => "typeBuildingGrounds",
                GeocodingSummaryFeatureType::TypeBuiltUpArea => "typeBuiltUpArea",
                GeocodingSummaryFeatureType::TypeBusStation => "typeBusStation",
                GeocodingSummaryFeatureType::TypeBusiness => "typeBusiness",
                GeocodingSummaryFeatureType::TypeBusinessChain => "typeBusinessChain",
                GeocodingSummaryFeatureType::TypeBusinessCorridor => "typeBusinessCorridor",
                GeocodingSummaryFeatureType::TypeCableCarStation => "typeCableCarStation",
                GeocodingSummaryFeatureType::TypeCampfirePit => "typeCampfirePit",
                GeocodingSummaryFeatureType::TypeCampgrounds => "typeCampgrounds",
                GeocodingSummaryFeatureType::TypeCampingSite => "typeCampingSite",
                GeocodingSummaryFeatureType::TypeCanal => "typeCanal",
                GeocodingSummaryFeatureType::TypeCarRental => "typeCarRental",
                GeocodingSummaryFeatureType::TypeCarRepair => "typeCarRepair",
                GeocodingSummaryFeatureType::TypeCartographic => "typeCartographic",
                GeocodingSummaryFeatureType::TypeCartographicLine => "typeCartographicLine",
                GeocodingSummaryFeatureType::TypeCashMachine => "typeCashMachine",
                GeocodingSummaryFeatureType::TypeCave => "typeCave",
                GeocodingSummaryFeatureType::TypeCay => "typeCay",
                GeocodingSummaryFeatureType::TypeCelestial => "typeCelestial",
                GeocodingSummaryFeatureType::TypeCemetery => "typeCemetery",
                GeocodingSummaryFeatureType::TypeChannel => "typeChannel",
                GeocodingSummaryFeatureType::TypeChurch => "typeChurch",
                GeocodingSummaryFeatureType::TypeCityHall => "typeCityHall",
                GeocodingSummaryFeatureType::TypeCliff => "typeCliff",
                GeocodingSummaryFeatureType::TypeCoffee => "typeCoffee",
                GeocodingSummaryFeatureType::TypeColloquialArea => "typeColloquialArea",
                GeocodingSummaryFeatureType::TypeColloquialCity => "typeColloquialCity",
                GeocodingSummaryFeatureType::TypeCompound => "typeCompound",
                GeocodingSummaryFeatureType::TypeCompoundBuilding => "typeCompoundBuilding",
                GeocodingSummaryFeatureType::TypeCompoundGrounds => "typeCompoundGrounds",
                GeocodingSummaryFeatureType::TypeCompoundSection => "typeCompoundSection",
                GeocodingSummaryFeatureType::TypeConfluence => "typeConfluence",
                GeocodingSummaryFeatureType::TypeConstituency => "typeConstituency",
                GeocodingSummaryFeatureType::TypeConstituencyFuture => "typeConstituencyFuture",
                GeocodingSummaryFeatureType::TypeContinent => "typeContinent",
                GeocodingSummaryFeatureType::TypeContourLine => "typeContourLine",
                GeocodingSummaryFeatureType::TypeCountry => "typeCountry",
                GeocodingSummaryFeatureType::TypeCourthouse => "typeCourthouse",
                GeocodingSummaryFeatureType::TypeCrater => "typeCrater",
                GeocodingSummaryFeatureType::TypeCurrent => "typeCurrent",
                GeocodingSummaryFeatureType::TypeDam => "typeDam",
                GeocodingSummaryFeatureType::TypeDataSource => "typeDataSource",
                GeocodingSummaryFeatureType::TypeDentist => "typeDentist",
                GeocodingSummaryFeatureType::TypeDeprecatedGolfShop => "typeDeprecatedGolfShop",
                GeocodingSummaryFeatureType::TypeDeprecatedHighwayDoNotUse => {
                    "typeDeprecatedHighwayDoNotUse"
                }
                GeocodingSummaryFeatureType::TypeDeprecatedTarmac => "typeDeprecatedTarmac",
                GeocodingSummaryFeatureType::TypeDesert => "typeDesert",
                GeocodingSummaryFeatureType::TypeDesignatedBarbecuePit => {
                    "typeDesignatedBarbecuePit"
                }
                GeocodingSummaryFeatureType::TypeDesignatedCookingArea => {
                    "typeDesignatedCookingArea"
                }
                GeocodingSummaryFeatureType::TypeDesignatedMarketArea => "typeDesignatedMarketArea",
                GeocodingSummaryFeatureType::TypeDigitalElevationModel => {
                    "typeDigitalElevationModel"
                }
                GeocodingSummaryFeatureType::TypeDisputedArea => "typeDisputedArea",
                GeocodingSummaryFeatureType::TypeDistributary => "typeDistributary",
                GeocodingSummaryFeatureType::TypeDoNotUseReservedToCatchGeneratedFiles => {
                    "typeDoNotUseReservedToCatchGeneratedFiles"
                }
                GeocodingSummaryFeatureType::TypeDoctor => "typeDoctor",
                GeocodingSummaryFeatureType::TypeDoodle => "typeDoodle",
                GeocodingSummaryFeatureType::TypeDrinkingWater => "typeDrinkingWater",
                GeocodingSummaryFeatureType::TypeDune => "typeDune",
                GeocodingSummaryFeatureType::TypeEarthquake => "typeEarthquake",
                GeocodingSummaryFeatureType::TypeEcoTouristDestination => {
                    "typeEcoTouristDestination"
                }
                GeocodingSummaryFeatureType::TypeElevated => "typeElevated",
                GeocodingSummaryFeatureType::TypeEmbassy => "typeEmbassy",
                GeocodingSummaryFeatureType::TypeEmergency => "typeEmergency",
                GeocodingSummaryFeatureType::TypeEnclosedTrafficArea => "typeEnclosedTrafficArea",
                GeocodingSummaryFeatureType::TypeEntrance => "typeEntrance",
                GeocodingSummaryFeatureType::TypeEstablishment => "typeEstablishment",
                GeocodingSummaryFeatureType::TypeEstablishmentBuilding => {
                    "typeEstablishmentBuilding"
                }
                GeocodingSummaryFeatureType::TypeEstablishmentGrounds => "typeEstablishmentGrounds",
                GeocodingSummaryFeatureType::TypeEstablishmentPoi => "typeEstablishmentPoi",
                GeocodingSummaryFeatureType::TypeEstablishmentService => "typeEstablishmentService",
                GeocodingSummaryFeatureType::TypeEstuary => "typeEstuary",
                GeocodingSummaryFeatureType::TypeEvent => "typeEvent",
                GeocodingSummaryFeatureType::TypeFault => "typeFault",
                GeocodingSummaryFeatureType::TypeFerry => "typeFerry",
                GeocodingSummaryFeatureType::TypeFerryBoat => "typeFerryBoat",
                GeocodingSummaryFeatureType::TypeFerryTerminal => "typeFerryTerminal",
                GeocodingSummaryFeatureType::TypeFerryTrain => "typeFerryTrain",
                GeocodingSummaryFeatureType::TypeFire => "typeFire",
                GeocodingSummaryFeatureType::TypeFishing => "typeFishing",
                GeocodingSummaryFeatureType::TypeFissure => "typeFissure",
                GeocodingSummaryFeatureType::TypeFjord => "typeFjord",
                GeocodingSummaryFeatureType::TypeFord => "typeFord",
                GeocodingSummaryFeatureType::TypeFunicularStation => "typeFunicularStation",
                GeocodingSummaryFeatureType::TypeFutureGeometry => "typeFutureGeometry",
                GeocodingSummaryFeatureType::TypeGasStation => "typeGasStation",
                GeocodingSummaryFeatureType::TypeGbCountry => "typeGbCountry",
                GeocodingSummaryFeatureType::TypeGbDependentLocality => "typeGbDependentLocality",
                GeocodingSummaryFeatureType::TypeGbDoubleDependentLocality => {
                    "typeGbDoubleDependentLocality"
                }
                GeocodingSummaryFeatureType::TypeGbFormerPostalCounty => "typeGbFormerPostalCounty",
                GeocodingSummaryFeatureType::TypeGbPostTown => "typeGbPostTown",
                GeocodingSummaryFeatureType::TypeGbTraditionalCounty => "typeGbTraditionalCounty",
                GeocodingSummaryFeatureType::TypeGeocodedAddress => "typeGeocodedAddress",
                GeocodingSummaryFeatureType::TypeGeyser => "typeGeyser",
                GeocodingSummaryFeatureType::TypeGlacier => "typeGlacier",
                GeocodingSummaryFeatureType::TypeGolf => "typeGolf",
                GeocodingSummaryFeatureType::TypeGolfCourse => "typeGolfCourse",
                GeocodingSummaryFeatureType::TypeGolfFairway => "typeGolfFairway",
                GeocodingSummaryFeatureType::TypeGolfHole => "typeGolfHole",
                GeocodingSummaryFeatureType::TypeGolfPuttingGreen => "typeGolfPuttingGreen",
                GeocodingSummaryFeatureType::TypeGolfRough => "typeGolfRough",
                GeocodingSummaryFeatureType::TypeGolfSandBunker => "typeGolfSandBunker",
                GeocodingSummaryFeatureType::TypeGolfTeeingGround => "typeGolfTeeingGround",
                GeocodingSummaryFeatureType::TypeGondolaLiftStation => "typeGondolaLiftStation",
                GeocodingSummaryFeatureType::TypeGovernment => "typeGovernment",
                GeocodingSummaryFeatureType::TypeGrassland => "typeGrassland",
                GeocodingSummaryFeatureType::TypeGrocery => "typeGrocery",
                GeocodingSummaryFeatureType::TypeGrounds => "typeGrounds",
                GeocodingSummaryFeatureType::TypeGurudwara => "typeGurudwara",
                GeocodingSummaryFeatureType::TypeHarbor => "typeHarbor",
                GeocodingSummaryFeatureType::TypeHeliport => "typeHeliport",
                GeocodingSummaryFeatureType::TypeHighSpeedRail => "typeHighSpeedRail",
                GeocodingSummaryFeatureType::TypeHighTension => "typeHighTension",
                GeocodingSummaryFeatureType::TypeHighway => "typeHighway",
                GeocodingSummaryFeatureType::TypeHighway1 => "typeHighway1",
                GeocodingSummaryFeatureType::TypeHighway2 => "typeHighway2",
                GeocodingSummaryFeatureType::TypeHighway3 => "typeHighway3",
                GeocodingSummaryFeatureType::TypeHighway4 => "typeHighway4",
                GeocodingSummaryFeatureType::TypeHighway5 => "typeHighway5",
                GeocodingSummaryFeatureType::TypeHighway6 => "typeHighway6",
                GeocodingSummaryFeatureType::TypeHighway7 => "typeHighway7",
                GeocodingSummaryFeatureType::TypeHighway8 => "typeHighway8",
                GeocodingSummaryFeatureType::TypeHighway9 => "typeHighway9",
                GeocodingSummaryFeatureType::TypeHikingArea => "typeHikingArea",
                GeocodingSummaryFeatureType::TypeHinduTemple => "typeHinduTemple",
                GeocodingSummaryFeatureType::TypeHorseCarriageStation => "typeHorseCarriageStation",
                GeocodingSummaryFeatureType::TypeHospital => "typeHospital",
                GeocodingSummaryFeatureType::TypeHospitalGrounds => "typeHospitalGrounds",
                GeocodingSummaryFeatureType::TypeHotSpring => "typeHotSpring",
                GeocodingSummaryFeatureType::TypeHunting => "typeHunting",
                GeocodingSummaryFeatureType::TypeHurricane => "typeHurricane",
                GeocodingSummaryFeatureType::TypeIce => "typeIce",
                GeocodingSummaryFeatureType::TypeIndustrial => "typeIndustrial",
                GeocodingSummaryFeatureType::TypeInlet => "typeInlet",
                GeocodingSummaryFeatureType::TypeIntersection => "typeIntersection",
                GeocodingSummaryFeatureType::TypeIntersectionGroup => "typeIntersectionGroup",
                GeocodingSummaryFeatureType::TypeIrrigation => "typeIrrigation",
                GeocodingSummaryFeatureType::TypeIsland => "typeIsland",
                GeocodingSummaryFeatureType::TypeIsthmus => "typeIsthmus",
                GeocodingSummaryFeatureType::TypeJpChiban => "typeJpChiban",
                GeocodingSummaryFeatureType::TypeJpEdaban => "typeJpEdaban",
                GeocodingSummaryFeatureType::TypeJpGaiku => "typeJpGaiku",
                GeocodingSummaryFeatureType::TypeJpGun => "typeJpGun",
                GeocodingSummaryFeatureType::TypeJpKoaza => "typeJpKoaza",
                GeocodingSummaryFeatureType::TypeJpOoaza => "typeJpOoaza",
                GeocodingSummaryFeatureType::TypeJpShikuchouson => "typeJpShikuchouson",
                GeocodingSummaryFeatureType::TypeJpSubShikuchouson => "typeJpSubShikuchouson",
                GeocodingSummaryFeatureType::TypeJpTodoufuken => "typeJpTodoufuken",
                GeocodingSummaryFeatureType::TypeJrTrack => "typeJrTrack",
                GeocodingSummaryFeatureType::TypeKarst => "typeKarst",
                GeocodingSummaryFeatureType::TypeLagoon => "typeLagoon",
                GeocodingSummaryFeatureType::TypeLake => "typeLake",
                GeocodingSummaryFeatureType::TypeLandMass => "typeLandMass",
                GeocodingSummaryFeatureType::TypeLandParcel => "typeLandParcel",
                GeocodingSummaryFeatureType::TypeLavaField => "typeLavaField",
                GeocodingSummaryFeatureType::TypeLevel => "typeLevel",
                GeocodingSummaryFeatureType::TypeLibrary => "typeLibrary",
                GeocodingSummaryFeatureType::TypeLightRailTrack => "typeLightRailTrack",
                GeocodingSummaryFeatureType::TypeLitterReceptacle => "typeLitterReceptacle",
                GeocodingSummaryFeatureType::TypeLocalPark => "typeLocalPark",
                GeocodingSummaryFeatureType::TypeLocale => "typeLocale",
                GeocodingSummaryFeatureType::TypeLocality => "typeLocality",
                GeocodingSummaryFeatureType::TypeLockerArea => "typeLockerArea",
                GeocodingSummaryFeatureType::TypeLodging => "typeLodging",
                GeocodingSummaryFeatureType::TypeMetaFeature => "typeMetaFeature",
                GeocodingSummaryFeatureType::TypeMilitary => "typeMilitary",
                GeocodingSummaryFeatureType::TypeMonorailStation => "typeMonorailStation",
                GeocodingSummaryFeatureType::TypeMonorailTrack => "typeMonorailTrack",
                GeocodingSummaryFeatureType::TypeMosque => "typeMosque",
                GeocodingSummaryFeatureType::TypeMountainRange => "typeMountainRange",
                GeocodingSummaryFeatureType::TypeMovieRental => "typeMovieRental",
                GeocodingSummaryFeatureType::TypeNarrowTrack => "typeNarrowTrack",
                GeocodingSummaryFeatureType::TypeNationalForest => "typeNationalForest",
                GeocodingSummaryFeatureType::TypeNationalPark => "typeNationalPark",
                GeocodingSummaryFeatureType::TypeNaturalFeature => "typeNaturalFeature",
                GeocodingSummaryFeatureType::TypeNatureReserve => "typeNatureReserve",
                GeocodingSummaryFeatureType::TypeNeighborhood => "typeNeighborhood",
                GeocodingSummaryFeatureType::TypeNunatak => "typeNunatak",
                GeocodingSummaryFeatureType::TypeOcean => "typeOcean",
                GeocodingSummaryFeatureType::TypeOceanRockExposed => "typeOceanRockExposed",
                GeocodingSummaryFeatureType::TypeOffRoadArea => "typeOffRoadArea",
                GeocodingSummaryFeatureType::TypePan => "typePan",
                GeocodingSummaryFeatureType::TypePark => "typePark",
                GeocodingSummaryFeatureType::TypeParking => "typeParking",
                GeocodingSummaryFeatureType::TypeParkingGarage => "typeParkingGarage",
                GeocodingSummaryFeatureType::TypeParkingLot => "typeParkingLot",
                GeocodingSummaryFeatureType::TypePass => "typePass",
                GeocodingSummaryFeatureType::TypePathway => "typePathway",
                GeocodingSummaryFeatureType::TypePeak => "typePeak",
                GeocodingSummaryFeatureType::TypePeninsula => "typePeninsula",
                GeocodingSummaryFeatureType::TypePharmacy => "typePharmacy",
                GeocodingSummaryFeatureType::TypePhoneNumberAreaCode => "typePhoneNumberAreaCode",
                GeocodingSummaryFeatureType::TypePhoneNumberPrefix => "typePhoneNumberPrefix",
                GeocodingSummaryFeatureType::TypePicnicArea => "typePicnicArea",
                GeocodingSummaryFeatureType::TypePlateau => "typePlateau",
                GeocodingSummaryFeatureType::TypePlayGround => "typePlayGround",
                GeocodingSummaryFeatureType::TypePolice => "typePolice",
                GeocodingSummaryFeatureType::TypePoliceJurisdiction => "typePoliceJurisdiction",
                GeocodingSummaryFeatureType::TypePolitical => "typePolitical",
                GeocodingSummaryFeatureType::TypePond => "typePond",
                GeocodingSummaryFeatureType::TypePostOffice => "typePostOffice",
                GeocodingSummaryFeatureType::TypePostTown => "typePostTown",
                GeocodingSummaryFeatureType::TypePostal => "typePostal",
                GeocodingSummaryFeatureType::TypePostalCode => "typePostalCode",
                GeocodingSummaryFeatureType::TypePostalCodePrefix => "typePostalCodePrefix",
                GeocodingSummaryFeatureType::TypePostalRound => "typePostalRound",
                GeocodingSummaryFeatureType::TypePremise => "typePremise",
                GeocodingSummaryFeatureType::TypeProvincialForest => "typeProvincialForest",
                GeocodingSummaryFeatureType::TypeProvincialPark => "typeProvincialPark",
                GeocodingSummaryFeatureType::TypePublicSpacesAndMonuments => {
                    "typePublicSpacesAndMonuments"
                }
                GeocodingSummaryFeatureType::TypeRailway => "typeRailway",
                GeocodingSummaryFeatureType::TypeRapids => "typeRapids",
                GeocodingSummaryFeatureType::TypeRavine => "typeRavine",
                GeocodingSummaryFeatureType::TypeReef => "typeReef",
                GeocodingSummaryFeatureType::TypeReefExtent => "typeReefExtent",
                GeocodingSummaryFeatureType::TypeReefFlat => "typeReefFlat",
                GeocodingSummaryFeatureType::TypeReefGrowth => "typeReefGrowth",
                GeocodingSummaryFeatureType::TypeReefRockSubmerged => "typeReefRockSubmerged",
                GeocodingSummaryFeatureType::TypeRegulatedArea => "typeRegulatedArea",
                GeocodingSummaryFeatureType::TypeReservation => "typeReservation",
                GeocodingSummaryFeatureType::TypeReservoir => "typeReservoir",
                GeocodingSummaryFeatureType::TypeRestArea => "typeRestArea",
                GeocodingSummaryFeatureType::TypeRestaurant => "typeRestaurant",
                GeocodingSummaryFeatureType::TypeRestrictionGroup => "typeRestrictionGroup",
                GeocodingSummaryFeatureType::TypeRidge => "typeRidge",
                GeocodingSummaryFeatureType::TypeRiver => "typeRiver",
                GeocodingSummaryFeatureType::TypeRoad => "typeRoad",
                GeocodingSummaryFeatureType::TypeRoadMonitor => "typeRoadMonitor",
                GeocodingSummaryFeatureType::TypeRoadSign => "typeRoadSign",
                GeocodingSummaryFeatureType::TypeRock => "typeRock",
                GeocodingSummaryFeatureType::TypeRocky => "typeRocky",
                GeocodingSummaryFeatureType::TypeRoute => "typeRoute",
                GeocodingSummaryFeatureType::TypeSaltFlat => "typeSaltFlat",
                GeocodingSummaryFeatureType::TypeSand => "typeSand",
                GeocodingSummaryFeatureType::TypeSchool => "typeSchool",
                GeocodingSummaryFeatureType::TypeSchoolDistrict => "typeSchoolDistrict",
                GeocodingSummaryFeatureType::TypeSea => "typeSea",
                GeocodingSummaryFeatureType::TypeSeaplaneBase => "typeSeaplaneBase",
                GeocodingSummaryFeatureType::TypeSeaport => "typeSeaport",
                GeocodingSummaryFeatureType::TypeSeasonalLake => "typeSeasonalLake",
                GeocodingSummaryFeatureType::TypeSeasonalRiver => "typeSeasonalRiver",
                GeocodingSummaryFeatureType::TypeSegment => "typeSegment",
                GeocodingSummaryFeatureType::TypeSegmentPath => "typeSegmentPath",
                GeocodingSummaryFeatureType::TypeShopping => "typeShopping",
                GeocodingSummaryFeatureType::TypeShoppingCenter => "typeShoppingCenter",
                GeocodingSummaryFeatureType::TypeShrubbery => "typeShrubbery",
                GeocodingSummaryFeatureType::TypeSkiBoundary => "typeSkiBoundary",
                GeocodingSummaryFeatureType::TypeSkiLift => "typeSkiLift",
                GeocodingSummaryFeatureType::TypeSkiTrail => "typeSkiTrail",
                GeocodingSummaryFeatureType::TypeSlope => "typeSlope",
                GeocodingSummaryFeatureType::TypeSpecialStation => "typeSpecialStation",
                GeocodingSummaryFeatureType::TypeSportsComplex => "typeSportsComplex",
                GeocodingSummaryFeatureType::TypeSpring => "typeSpring",
                GeocodingSummaryFeatureType::TypeSpur => "typeSpur",
                GeocodingSummaryFeatureType::TypeStadium => "typeStadium",
                GeocodingSummaryFeatureType::TypeStandardTrack => "typeStandardTrack",
                GeocodingSummaryFeatureType::TypeStatisticalArea => "typeStatisticalArea",
                GeocodingSummaryFeatureType::TypeStatue => "typeStatue",
                GeocodingSummaryFeatureType::TypeStrait => "typeStrait",
                GeocodingSummaryFeatureType::TypeSubPremise => "typeSubPremise",
                GeocodingSummaryFeatureType::TypeSublocality => "typeSublocality",
                GeocodingSummaryFeatureType::TypeSublocality1 => "typeSublocality1",
                GeocodingSummaryFeatureType::TypeSublocality2 => "typeSublocality2",
                GeocodingSummaryFeatureType::TypeSublocality3 => "typeSublocality3",
                GeocodingSummaryFeatureType::TypeSublocality4 => "typeSublocality4",
                GeocodingSummaryFeatureType::TypeSublocality5 => "typeSublocality5",
                GeocodingSummaryFeatureType::TypeSubmarineBasin => "typeSubmarineBasin",
                GeocodingSummaryFeatureType::TypeSubmarineCliff => "typeSubmarineCliff",
                GeocodingSummaryFeatureType::TypeSubmarineDeep => "typeSubmarineDeep",
                GeocodingSummaryFeatureType::TypeSubmarineFractureZone => {
                    "typeSubmarineFractureZone"
                }
                GeocodingSummaryFeatureType::TypeSubmarineGap => "typeSubmarineGap",
                GeocodingSummaryFeatureType::TypeSubmarinePlain => "typeSubmarinePlain",
                GeocodingSummaryFeatureType::TypeSubmarinePlateau => "typeSubmarinePlateau",
                GeocodingSummaryFeatureType::TypeSubmarineRidge => "typeSubmarineRidge",
                GeocodingSummaryFeatureType::TypeSubmarineSeamount => "typeSubmarineSeamount",
                GeocodingSummaryFeatureType::TypeSubmarineSlope => "typeSubmarineSlope",
                GeocodingSummaryFeatureType::TypeSubmarineValley => "typeSubmarineValley",
                GeocodingSummaryFeatureType::TypeSubwayStation => "typeSubwayStation",
                GeocodingSummaryFeatureType::TypeSubwayTrack => "typeSubwayTrack",
                GeocodingSummaryFeatureType::TypeSuite => "typeSuite",
                GeocodingSummaryFeatureType::TypeSynagogue => "typeSynagogue",
                GeocodingSummaryFeatureType::TypeTarmac => "typeTarmac",
                GeocodingSummaryFeatureType::TypeTectonic => "typeTectonic",
                GeocodingSummaryFeatureType::TypeTemple => "typeTemple",
                GeocodingSummaryFeatureType::TypeTerminalPoint => "typeTerminalPoint",
                GeocodingSummaryFeatureType::TypeTerrace => "typeTerrace",
                GeocodingSummaryFeatureType::TypeTerrain => "typeTerrain",
                GeocodingSummaryFeatureType::TypeTimezone => "typeTimezone",
                GeocodingSummaryFeatureType::TypeTollCluster => "typeTollCluster",
                GeocodingSummaryFeatureType::TypeTouristDestination => "typeTouristDestination",
                GeocodingSummaryFeatureType::TypeTownSquare => "typeTownSquare",
                GeocodingSummaryFeatureType::TypeTrail => "typeTrail",
                GeocodingSummaryFeatureType::TypeTrailHead => "typeTrailHead",
                GeocodingSummaryFeatureType::TypeTrainStation => "typeTrainStation",
                GeocodingSummaryFeatureType::TypeTramwayStation => "typeTramwayStation",
                GeocodingSummaryFeatureType::TypeTransient => "typeTransient",
                GeocodingSummaryFeatureType::TypeTransit => "typeTransit",
                GeocodingSummaryFeatureType::TypeTransitAgency => "typeTransitAgency",
                GeocodingSummaryFeatureType::TypeTransitAgencyDeprecatedValue => {
                    "typeTransitAgencyDeprecatedValue"
                }
                GeocodingSummaryFeatureType::TypeTransitDeparture => "typeTransitDeparture",
                GeocodingSummaryFeatureType::TypeTransitLeg => "typeTransitLeg",
                GeocodingSummaryFeatureType::TypeTransitLine => "typeTransitLine",
                GeocodingSummaryFeatureType::TypeTransitStation => "typeTransitStation",
                GeocodingSummaryFeatureType::TypeTransitStop => "typeTransitStop",
                GeocodingSummaryFeatureType::TypeTransitTransfer => "typeTransitTransfer",
                GeocodingSummaryFeatureType::TypeTransitTrip => "typeTransitTrip",
                GeocodingSummaryFeatureType::TypeTransportation => "typeTransportation",
                GeocodingSummaryFeatureType::TypeTravelService => "typeTravelService",
                GeocodingSummaryFeatureType::TypeTrolleyTrack => "typeTrolleyTrack",
                GeocodingSummaryFeatureType::TypeTundra => "typeTundra",
                GeocodingSummaryFeatureType::TypeUndersea => "typeUndersea",
                GeocodingSummaryFeatureType::TypeUniversity => "typeUniversity",
                GeocodingSummaryFeatureType::TypeUniversityGrounds => "typeUniversityGrounds",
                GeocodingSummaryFeatureType::TypeUnknown => "typeUnknown",
                GeocodingSummaryFeatureType::TypeUnstableHillside => "typeUnstableHillside",
                GeocodingSummaryFeatureType::TypeUpland => "typeUpland",
                GeocodingSummaryFeatureType::TypeUsBorough => "typeUsBorough",
                GeocodingSummaryFeatureType::TypeUsNationalMonument => "typeUsNationalMonument",
                GeocodingSummaryFeatureType::TypeUsNationalPark => "typeUsNationalPark",
                GeocodingSummaryFeatureType::TypeUsState => "typeUsState",
                GeocodingSummaryFeatureType::TypeVegetation => "typeVegetation",
                GeocodingSummaryFeatureType::TypeVeterinarian => "typeVeterinarian",
                GeocodingSummaryFeatureType::TypeVirtualSegment => "typeVirtualSegment",
                GeocodingSummaryFeatureType::TypeVista => "typeVista",
                GeocodingSummaryFeatureType::TypeVolcano => "typeVolcano",
                GeocodingSummaryFeatureType::TypeWadi => "typeWadi",
                GeocodingSummaryFeatureType::TypeWall => "typeWall",
                GeocodingSummaryFeatureType::TypeWater => "typeWater",
                GeocodingSummaryFeatureType::TypeWaterFountain => "typeWaterFountain",
                GeocodingSummaryFeatureType::TypeWaterNavigation => "typeWaterNavigation",
                GeocodingSummaryFeatureType::TypeWaterfall => "typeWaterfall",
                GeocodingSummaryFeatureType::TypeWateringHole => "typeWateringHole",
                GeocodingSummaryFeatureType::TypeWateringHoleDeprecated => {
                    "typeWateringHoleDeprecated"
                }
                GeocodingSummaryFeatureType::TypeWatershedBoundary => "typeWatershedBoundary",
                GeocodingSummaryFeatureType::TypeWeatherCondition => "typeWeatherCondition",
                GeocodingSummaryFeatureType::TypeWetland => "typeWetland",
                GeocodingSummaryFeatureType::TypeWoods => "typeWoods",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GeocodingSummaryFeatureType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GeocodingSummaryFeatureType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GeocodingSummaryFeatureType, ()> {
            Ok(match s {
                "typeAddressTemplate" => GeocodingSummaryFeatureType::TypeAddressTemplate,
                "typeAdministrativeArea" => GeocodingSummaryFeatureType::TypeAdministrativeArea,
                "typeAdministrativeArea1" => GeocodingSummaryFeatureType::TypeAdministrativeArea1,
                "typeAdministrativeArea2" => GeocodingSummaryFeatureType::TypeAdministrativeArea2,
                "typeAdministrativeArea3" => GeocodingSummaryFeatureType::TypeAdministrativeArea3,
                "typeAdministrativeArea4" => GeocodingSummaryFeatureType::TypeAdministrativeArea4,
                "typeAdministrativeArea5" => GeocodingSummaryFeatureType::TypeAdministrativeArea5,
                "typeAdministrativeArea6" => GeocodingSummaryFeatureType::TypeAdministrativeArea6,
                "typeAdministrativeArea7" => GeocodingSummaryFeatureType::TypeAdministrativeArea7,
                "typeAdministrativeArea8" => GeocodingSummaryFeatureType::TypeAdministrativeArea8,
                "typeAdministrativeArea9" => GeocodingSummaryFeatureType::TypeAdministrativeArea9,
                "typeAgricultural" => GeocodingSummaryFeatureType::TypeAgricultural,
                "typeAirport" => GeocodingSummaryFeatureType::TypeAirport,
                "typeAirportCivil" => GeocodingSummaryFeatureType::TypeAirportCivil,
                "typeAirportGrounds" => GeocodingSummaryFeatureType::TypeAirportGrounds,
                "typeAirportMilitary" => GeocodingSummaryFeatureType::TypeAirportMilitary,
                "typeAirportMixed" => GeocodingSummaryFeatureType::TypeAirportMixed,
                "typeAirstrip" => GeocodingSummaryFeatureType::TypeAirstrip,
                "typeAnimalEnclosure" => GeocodingSummaryFeatureType::TypeAnimalEnclosure,
                "typeAny" => GeocodingSummaryFeatureType::TypeAny,
                "typeArchipelago" => GeocodingSummaryFeatureType::TypeArchipelago,
                "typeAtoll" => GeocodingSummaryFeatureType::TypeAtoll,
                "typeBank" => GeocodingSummaryFeatureType::TypeBank,
                "typeBar" => GeocodingSummaryFeatureType::TypeBar,
                "typeBay" => GeocodingSummaryFeatureType::TypeBay,
                "typeBeach" => GeocodingSummaryFeatureType::TypeBeach,
                "typeBicycleRoute" => GeocodingSummaryFeatureType::TypeBicycleRoute,
                "typeBight" => GeocodingSummaryFeatureType::TypeBight,
                "typeBirdWatching" => GeocodingSummaryFeatureType::TypeBirdWatching,
                "typeBorder" => GeocodingSummaryFeatureType::TypeBorder,
                "typeBorderCrossing" => GeocodingSummaryFeatureType::TypeBorderCrossing,
                "typeBroadTrack" => GeocodingSummaryFeatureType::TypeBroadTrack,
                "typeBuilding" => GeocodingSummaryFeatureType::TypeBuilding,
                "typeBuildingGrounds" => GeocodingSummaryFeatureType::TypeBuildingGrounds,
                "typeBuiltUpArea" => GeocodingSummaryFeatureType::TypeBuiltUpArea,
                "typeBusStation" => GeocodingSummaryFeatureType::TypeBusStation,
                "typeBusiness" => GeocodingSummaryFeatureType::TypeBusiness,
                "typeBusinessChain" => GeocodingSummaryFeatureType::TypeBusinessChain,
                "typeBusinessCorridor" => GeocodingSummaryFeatureType::TypeBusinessCorridor,
                "typeCableCarStation" => GeocodingSummaryFeatureType::TypeCableCarStation,
                "typeCampfirePit" => GeocodingSummaryFeatureType::TypeCampfirePit,
                "typeCampgrounds" => GeocodingSummaryFeatureType::TypeCampgrounds,
                "typeCampingSite" => GeocodingSummaryFeatureType::TypeCampingSite,
                "typeCanal" => GeocodingSummaryFeatureType::TypeCanal,
                "typeCarRental" => GeocodingSummaryFeatureType::TypeCarRental,
                "typeCarRepair" => GeocodingSummaryFeatureType::TypeCarRepair,
                "typeCartographic" => GeocodingSummaryFeatureType::TypeCartographic,
                "typeCartographicLine" => GeocodingSummaryFeatureType::TypeCartographicLine,
                "typeCashMachine" => GeocodingSummaryFeatureType::TypeCashMachine,
                "typeCave" => GeocodingSummaryFeatureType::TypeCave,
                "typeCay" => GeocodingSummaryFeatureType::TypeCay,
                "typeCelestial" => GeocodingSummaryFeatureType::TypeCelestial,
                "typeCemetery" => GeocodingSummaryFeatureType::TypeCemetery,
                "typeChannel" => GeocodingSummaryFeatureType::TypeChannel,
                "typeChurch" => GeocodingSummaryFeatureType::TypeChurch,
                "typeCityHall" => GeocodingSummaryFeatureType::TypeCityHall,
                "typeCliff" => GeocodingSummaryFeatureType::TypeCliff,
                "typeCoffee" => GeocodingSummaryFeatureType::TypeCoffee,
                "typeColloquialArea" => GeocodingSummaryFeatureType::TypeColloquialArea,
                "typeColloquialCity" => GeocodingSummaryFeatureType::TypeColloquialCity,
                "typeCompound" => GeocodingSummaryFeatureType::TypeCompound,
                "typeCompoundBuilding" => GeocodingSummaryFeatureType::TypeCompoundBuilding,
                "typeCompoundGrounds" => GeocodingSummaryFeatureType::TypeCompoundGrounds,
                "typeCompoundSection" => GeocodingSummaryFeatureType::TypeCompoundSection,
                "typeConfluence" => GeocodingSummaryFeatureType::TypeConfluence,
                "typeConstituency" => GeocodingSummaryFeatureType::TypeConstituency,
                "typeConstituencyFuture" => GeocodingSummaryFeatureType::TypeConstituencyFuture,
                "typeContinent" => GeocodingSummaryFeatureType::TypeContinent,
                "typeContourLine" => GeocodingSummaryFeatureType::TypeContourLine,
                "typeCountry" => GeocodingSummaryFeatureType::TypeCountry,
                "typeCourthouse" => GeocodingSummaryFeatureType::TypeCourthouse,
                "typeCrater" => GeocodingSummaryFeatureType::TypeCrater,
                "typeCurrent" => GeocodingSummaryFeatureType::TypeCurrent,
                "typeDam" => GeocodingSummaryFeatureType::TypeDam,
                "typeDataSource" => GeocodingSummaryFeatureType::TypeDataSource,
                "typeDentist" => GeocodingSummaryFeatureType::TypeDentist,
                "typeDeprecatedGolfShop" => GeocodingSummaryFeatureType::TypeDeprecatedGolfShop,
                "typeDeprecatedHighwayDoNotUse" => {
                    GeocodingSummaryFeatureType::TypeDeprecatedHighwayDoNotUse
                }
                "typeDeprecatedTarmac" => GeocodingSummaryFeatureType::TypeDeprecatedTarmac,
                "typeDesert" => GeocodingSummaryFeatureType::TypeDesert,
                "typeDesignatedBarbecuePit" => {
                    GeocodingSummaryFeatureType::TypeDesignatedBarbecuePit
                }
                "typeDesignatedCookingArea" => {
                    GeocodingSummaryFeatureType::TypeDesignatedCookingArea
                }
                "typeDesignatedMarketArea" => GeocodingSummaryFeatureType::TypeDesignatedMarketArea,
                "typeDigitalElevationModel" => {
                    GeocodingSummaryFeatureType::TypeDigitalElevationModel
                }
                "typeDisputedArea" => GeocodingSummaryFeatureType::TypeDisputedArea,
                "typeDistributary" => GeocodingSummaryFeatureType::TypeDistributary,
                "typeDoNotUseReservedToCatchGeneratedFiles" => {
                    GeocodingSummaryFeatureType::TypeDoNotUseReservedToCatchGeneratedFiles
                }
                "typeDoctor" => GeocodingSummaryFeatureType::TypeDoctor,
                "typeDoodle" => GeocodingSummaryFeatureType::TypeDoodle,
                "typeDrinkingWater" => GeocodingSummaryFeatureType::TypeDrinkingWater,
                "typeDune" => GeocodingSummaryFeatureType::TypeDune,
                "typeEarthquake" => GeocodingSummaryFeatureType::TypeEarthquake,
                "typeEcoTouristDestination" => {
                    GeocodingSummaryFeatureType::TypeEcoTouristDestination
                }
                "typeElevated" => GeocodingSummaryFeatureType::TypeElevated,
                "typeEmbassy" => GeocodingSummaryFeatureType::TypeEmbassy,
                "typeEmergency" => GeocodingSummaryFeatureType::TypeEmergency,
                "typeEnclosedTrafficArea" => GeocodingSummaryFeatureType::TypeEnclosedTrafficArea,
                "typeEntrance" => GeocodingSummaryFeatureType::TypeEntrance,
                "typeEstablishment" => GeocodingSummaryFeatureType::TypeEstablishment,
                "typeEstablishmentBuilding" => {
                    GeocodingSummaryFeatureType::TypeEstablishmentBuilding
                }
                "typeEstablishmentGrounds" => GeocodingSummaryFeatureType::TypeEstablishmentGrounds,
                "typeEstablishmentPoi" => GeocodingSummaryFeatureType::TypeEstablishmentPoi,
                "typeEstablishmentService" => GeocodingSummaryFeatureType::TypeEstablishmentService,
                "typeEstuary" => GeocodingSummaryFeatureType::TypeEstuary,
                "typeEvent" => GeocodingSummaryFeatureType::TypeEvent,
                "typeFault" => GeocodingSummaryFeatureType::TypeFault,
                "typeFerry" => GeocodingSummaryFeatureType::TypeFerry,
                "typeFerryBoat" => GeocodingSummaryFeatureType::TypeFerryBoat,
                "typeFerryTerminal" => GeocodingSummaryFeatureType::TypeFerryTerminal,
                "typeFerryTrain" => GeocodingSummaryFeatureType::TypeFerryTrain,
                "typeFire" => GeocodingSummaryFeatureType::TypeFire,
                "typeFishing" => GeocodingSummaryFeatureType::TypeFishing,
                "typeFissure" => GeocodingSummaryFeatureType::TypeFissure,
                "typeFjord" => GeocodingSummaryFeatureType::TypeFjord,
                "typeFord" => GeocodingSummaryFeatureType::TypeFord,
                "typeFunicularStation" => GeocodingSummaryFeatureType::TypeFunicularStation,
                "typeFutureGeometry" => GeocodingSummaryFeatureType::TypeFutureGeometry,
                "typeGasStation" => GeocodingSummaryFeatureType::TypeGasStation,
                "typeGbCountry" => GeocodingSummaryFeatureType::TypeGbCountry,
                "typeGbDependentLocality" => GeocodingSummaryFeatureType::TypeGbDependentLocality,
                "typeGbDoubleDependentLocality" => {
                    GeocodingSummaryFeatureType::TypeGbDoubleDependentLocality
                }
                "typeGbFormerPostalCounty" => GeocodingSummaryFeatureType::TypeGbFormerPostalCounty,
                "typeGbPostTown" => GeocodingSummaryFeatureType::TypeGbPostTown,
                "typeGbTraditionalCounty" => GeocodingSummaryFeatureType::TypeGbTraditionalCounty,
                "typeGeocodedAddress" => GeocodingSummaryFeatureType::TypeGeocodedAddress,
                "typeGeyser" => GeocodingSummaryFeatureType::TypeGeyser,
                "typeGlacier" => GeocodingSummaryFeatureType::TypeGlacier,
                "typeGolf" => GeocodingSummaryFeatureType::TypeGolf,
                "typeGolfCourse" => GeocodingSummaryFeatureType::TypeGolfCourse,
                "typeGolfFairway" => GeocodingSummaryFeatureType::TypeGolfFairway,
                "typeGolfHole" => GeocodingSummaryFeatureType::TypeGolfHole,
                "typeGolfPuttingGreen" => GeocodingSummaryFeatureType::TypeGolfPuttingGreen,
                "typeGolfRough" => GeocodingSummaryFeatureType::TypeGolfRough,
                "typeGolfSandBunker" => GeocodingSummaryFeatureType::TypeGolfSandBunker,
                "typeGolfTeeingGround" => GeocodingSummaryFeatureType::TypeGolfTeeingGround,
                "typeGondolaLiftStation" => GeocodingSummaryFeatureType::TypeGondolaLiftStation,
                "typeGovernment" => GeocodingSummaryFeatureType::TypeGovernment,
                "typeGrassland" => GeocodingSummaryFeatureType::TypeGrassland,
                "typeGrocery" => GeocodingSummaryFeatureType::TypeGrocery,
                "typeGrounds" => GeocodingSummaryFeatureType::TypeGrounds,
                "typeGurudwara" => GeocodingSummaryFeatureType::TypeGurudwara,
                "typeHarbor" => GeocodingSummaryFeatureType::TypeHarbor,
                "typeHeliport" => GeocodingSummaryFeatureType::TypeHeliport,
                "typeHighSpeedRail" => GeocodingSummaryFeatureType::TypeHighSpeedRail,
                "typeHighTension" => GeocodingSummaryFeatureType::TypeHighTension,
                "typeHighway" => GeocodingSummaryFeatureType::TypeHighway,
                "typeHighway1" => GeocodingSummaryFeatureType::TypeHighway1,
                "typeHighway2" => GeocodingSummaryFeatureType::TypeHighway2,
                "typeHighway3" => GeocodingSummaryFeatureType::TypeHighway3,
                "typeHighway4" => GeocodingSummaryFeatureType::TypeHighway4,
                "typeHighway5" => GeocodingSummaryFeatureType::TypeHighway5,
                "typeHighway6" => GeocodingSummaryFeatureType::TypeHighway6,
                "typeHighway7" => GeocodingSummaryFeatureType::TypeHighway7,
                "typeHighway8" => GeocodingSummaryFeatureType::TypeHighway8,
                "typeHighway9" => GeocodingSummaryFeatureType::TypeHighway9,
                "typeHikingArea" => GeocodingSummaryFeatureType::TypeHikingArea,
                "typeHinduTemple" => GeocodingSummaryFeatureType::TypeHinduTemple,
                "typeHorseCarriageStation" => GeocodingSummaryFeatureType::TypeHorseCarriageStation,
                "typeHospital" => GeocodingSummaryFeatureType::TypeHospital,
                "typeHospitalGrounds" => GeocodingSummaryFeatureType::TypeHospitalGrounds,
                "typeHotSpring" => GeocodingSummaryFeatureType::TypeHotSpring,
                "typeHunting" => GeocodingSummaryFeatureType::TypeHunting,
                "typeHurricane" => GeocodingSummaryFeatureType::TypeHurricane,
                "typeIce" => GeocodingSummaryFeatureType::TypeIce,
                "typeIndustrial" => GeocodingSummaryFeatureType::TypeIndustrial,
                "typeInlet" => GeocodingSummaryFeatureType::TypeInlet,
                "typeIntersection" => GeocodingSummaryFeatureType::TypeIntersection,
                "typeIntersectionGroup" => GeocodingSummaryFeatureType::TypeIntersectionGroup,
                "typeIrrigation" => GeocodingSummaryFeatureType::TypeIrrigation,
                "typeIsland" => GeocodingSummaryFeatureType::TypeIsland,
                "typeIsthmus" => GeocodingSummaryFeatureType::TypeIsthmus,
                "typeJpChiban" => GeocodingSummaryFeatureType::TypeJpChiban,
                "typeJpEdaban" => GeocodingSummaryFeatureType::TypeJpEdaban,
                "typeJpGaiku" => GeocodingSummaryFeatureType::TypeJpGaiku,
                "typeJpGun" => GeocodingSummaryFeatureType::TypeJpGun,
                "typeJpKoaza" => GeocodingSummaryFeatureType::TypeJpKoaza,
                "typeJpOoaza" => GeocodingSummaryFeatureType::TypeJpOoaza,
                "typeJpShikuchouson" => GeocodingSummaryFeatureType::TypeJpShikuchouson,
                "typeJpSubShikuchouson" => GeocodingSummaryFeatureType::TypeJpSubShikuchouson,
                "typeJpTodoufuken" => GeocodingSummaryFeatureType::TypeJpTodoufuken,
                "typeJrTrack" => GeocodingSummaryFeatureType::TypeJrTrack,
                "typeKarst" => GeocodingSummaryFeatureType::TypeKarst,
                "typeLagoon" => GeocodingSummaryFeatureType::TypeLagoon,
                "typeLake" => GeocodingSummaryFeatureType::TypeLake,
                "typeLandMass" => GeocodingSummaryFeatureType::TypeLandMass,
                "typeLandParcel" => GeocodingSummaryFeatureType::TypeLandParcel,
                "typeLavaField" => GeocodingSummaryFeatureType::TypeLavaField,
                "typeLevel" => GeocodingSummaryFeatureType::TypeLevel,
                "typeLibrary" => GeocodingSummaryFeatureType::TypeLibrary,
                "typeLightRailTrack" => GeocodingSummaryFeatureType::TypeLightRailTrack,
                "typeLitterReceptacle" => GeocodingSummaryFeatureType::TypeLitterReceptacle,
                "typeLocalPark" => GeocodingSummaryFeatureType::TypeLocalPark,
                "typeLocale" => GeocodingSummaryFeatureType::TypeLocale,
                "typeLocality" => GeocodingSummaryFeatureType::TypeLocality,
                "typeLockerArea" => GeocodingSummaryFeatureType::TypeLockerArea,
                "typeLodging" => GeocodingSummaryFeatureType::TypeLodging,
                "typeMetaFeature" => GeocodingSummaryFeatureType::TypeMetaFeature,
                "typeMilitary" => GeocodingSummaryFeatureType::TypeMilitary,
                "typeMonorailStation" => GeocodingSummaryFeatureType::TypeMonorailStation,
                "typeMonorailTrack" => GeocodingSummaryFeatureType::TypeMonorailTrack,
                "typeMosque" => GeocodingSummaryFeatureType::TypeMosque,
                "typeMountainRange" => GeocodingSummaryFeatureType::TypeMountainRange,
                "typeMovieRental" => GeocodingSummaryFeatureType::TypeMovieRental,
                "typeNarrowTrack" => GeocodingSummaryFeatureType::TypeNarrowTrack,
                "typeNationalForest" => GeocodingSummaryFeatureType::TypeNationalForest,
                "typeNationalPark" => GeocodingSummaryFeatureType::TypeNationalPark,
                "typeNaturalFeature" => GeocodingSummaryFeatureType::TypeNaturalFeature,
                "typeNatureReserve" => GeocodingSummaryFeatureType::TypeNatureReserve,
                "typeNeighborhood" => GeocodingSummaryFeatureType::TypeNeighborhood,
                "typeNunatak" => GeocodingSummaryFeatureType::TypeNunatak,
                "typeOcean" => GeocodingSummaryFeatureType::TypeOcean,
                "typeOceanRockExposed" => GeocodingSummaryFeatureType::TypeOceanRockExposed,
                "typeOffRoadArea" => GeocodingSummaryFeatureType::TypeOffRoadArea,
                "typePan" => GeocodingSummaryFeatureType::TypePan,
                "typePark" => GeocodingSummaryFeatureType::TypePark,
                "typeParking" => GeocodingSummaryFeatureType::TypeParking,
                "typeParkingGarage" => GeocodingSummaryFeatureType::TypeParkingGarage,
                "typeParkingLot" => GeocodingSummaryFeatureType::TypeParkingLot,
                "typePass" => GeocodingSummaryFeatureType::TypePass,
                "typePathway" => GeocodingSummaryFeatureType::TypePathway,
                "typePeak" => GeocodingSummaryFeatureType::TypePeak,
                "typePeninsula" => GeocodingSummaryFeatureType::TypePeninsula,
                "typePharmacy" => GeocodingSummaryFeatureType::TypePharmacy,
                "typePhoneNumberAreaCode" => GeocodingSummaryFeatureType::TypePhoneNumberAreaCode,
                "typePhoneNumberPrefix" => GeocodingSummaryFeatureType::TypePhoneNumberPrefix,
                "typePicnicArea" => GeocodingSummaryFeatureType::TypePicnicArea,
                "typePlateau" => GeocodingSummaryFeatureType::TypePlateau,
                "typePlayGround" => GeocodingSummaryFeatureType::TypePlayGround,
                "typePolice" => GeocodingSummaryFeatureType::TypePolice,
                "typePoliceJurisdiction" => GeocodingSummaryFeatureType::TypePoliceJurisdiction,
                "typePolitical" => GeocodingSummaryFeatureType::TypePolitical,
                "typePond" => GeocodingSummaryFeatureType::TypePond,
                "typePostOffice" => GeocodingSummaryFeatureType::TypePostOffice,
                "typePostTown" => GeocodingSummaryFeatureType::TypePostTown,
                "typePostal" => GeocodingSummaryFeatureType::TypePostal,
                "typePostalCode" => GeocodingSummaryFeatureType::TypePostalCode,
                "typePostalCodePrefix" => GeocodingSummaryFeatureType::TypePostalCodePrefix,
                "typePostalRound" => GeocodingSummaryFeatureType::TypePostalRound,
                "typePremise" => GeocodingSummaryFeatureType::TypePremise,
                "typeProvincialForest" => GeocodingSummaryFeatureType::TypeProvincialForest,
                "typeProvincialPark" => GeocodingSummaryFeatureType::TypeProvincialPark,
                "typePublicSpacesAndMonuments" => {
                    GeocodingSummaryFeatureType::TypePublicSpacesAndMonuments
                }
                "typeRailway" => GeocodingSummaryFeatureType::TypeRailway,
                "typeRapids" => GeocodingSummaryFeatureType::TypeRapids,
                "typeRavine" => GeocodingSummaryFeatureType::TypeRavine,
                "typeReef" => GeocodingSummaryFeatureType::TypeReef,
                "typeReefExtent" => GeocodingSummaryFeatureType::TypeReefExtent,
                "typeReefFlat" => GeocodingSummaryFeatureType::TypeReefFlat,
                "typeReefGrowth" => GeocodingSummaryFeatureType::TypeReefGrowth,
                "typeReefRockSubmerged" => GeocodingSummaryFeatureType::TypeReefRockSubmerged,
                "typeRegulatedArea" => GeocodingSummaryFeatureType::TypeRegulatedArea,
                "typeReservation" => GeocodingSummaryFeatureType::TypeReservation,
                "typeReservoir" => GeocodingSummaryFeatureType::TypeReservoir,
                "typeRestArea" => GeocodingSummaryFeatureType::TypeRestArea,
                "typeRestaurant" => GeocodingSummaryFeatureType::TypeRestaurant,
                "typeRestrictionGroup" => GeocodingSummaryFeatureType::TypeRestrictionGroup,
                "typeRidge" => GeocodingSummaryFeatureType::TypeRidge,
                "typeRiver" => GeocodingSummaryFeatureType::TypeRiver,
                "typeRoad" => GeocodingSummaryFeatureType::TypeRoad,
                "typeRoadMonitor" => GeocodingSummaryFeatureType::TypeRoadMonitor,
                "typeRoadSign" => GeocodingSummaryFeatureType::TypeRoadSign,
                "typeRock" => GeocodingSummaryFeatureType::TypeRock,
                "typeRocky" => GeocodingSummaryFeatureType::TypeRocky,
                "typeRoute" => GeocodingSummaryFeatureType::TypeRoute,
                "typeSaltFlat" => GeocodingSummaryFeatureType::TypeSaltFlat,
                "typeSand" => GeocodingSummaryFeatureType::TypeSand,
                "typeSchool" => GeocodingSummaryFeatureType::TypeSchool,
                "typeSchoolDistrict" => GeocodingSummaryFeatureType::TypeSchoolDistrict,
                "typeSea" => GeocodingSummaryFeatureType::TypeSea,
                "typeSeaplaneBase" => GeocodingSummaryFeatureType::TypeSeaplaneBase,
                "typeSeaport" => GeocodingSummaryFeatureType::TypeSeaport,
                "typeSeasonalLake" => GeocodingSummaryFeatureType::TypeSeasonalLake,
                "typeSeasonalRiver" => GeocodingSummaryFeatureType::TypeSeasonalRiver,
                "typeSegment" => GeocodingSummaryFeatureType::TypeSegment,
                "typeSegmentPath" => GeocodingSummaryFeatureType::TypeSegmentPath,
                "typeShopping" => GeocodingSummaryFeatureType::TypeShopping,
                "typeShoppingCenter" => GeocodingSummaryFeatureType::TypeShoppingCenter,
                "typeShrubbery" => GeocodingSummaryFeatureType::TypeShrubbery,
                "typeSkiBoundary" => GeocodingSummaryFeatureType::TypeSkiBoundary,
                "typeSkiLift" => GeocodingSummaryFeatureType::TypeSkiLift,
                "typeSkiTrail" => GeocodingSummaryFeatureType::TypeSkiTrail,
                "typeSlope" => GeocodingSummaryFeatureType::TypeSlope,
                "typeSpecialStation" => GeocodingSummaryFeatureType::TypeSpecialStation,
                "typeSportsComplex" => GeocodingSummaryFeatureType::TypeSportsComplex,
                "typeSpring" => GeocodingSummaryFeatureType::TypeSpring,
                "typeSpur" => GeocodingSummaryFeatureType::TypeSpur,
                "typeStadium" => GeocodingSummaryFeatureType::TypeStadium,
                "typeStandardTrack" => GeocodingSummaryFeatureType::TypeStandardTrack,
                "typeStatisticalArea" => GeocodingSummaryFeatureType::TypeStatisticalArea,
                "typeStatue" => GeocodingSummaryFeatureType::TypeStatue,
                "typeStrait" => GeocodingSummaryFeatureType::TypeStrait,
                "typeSubPremise" => GeocodingSummaryFeatureType::TypeSubPremise,
                "typeSublocality" => GeocodingSummaryFeatureType::TypeSublocality,
                "typeSublocality1" => GeocodingSummaryFeatureType::TypeSublocality1,
                "typeSublocality2" => GeocodingSummaryFeatureType::TypeSublocality2,
                "typeSublocality3" => GeocodingSummaryFeatureType::TypeSublocality3,
                "typeSublocality4" => GeocodingSummaryFeatureType::TypeSublocality4,
                "typeSublocality5" => GeocodingSummaryFeatureType::TypeSublocality5,
                "typeSubmarineBasin" => GeocodingSummaryFeatureType::TypeSubmarineBasin,
                "typeSubmarineCliff" => GeocodingSummaryFeatureType::TypeSubmarineCliff,
                "typeSubmarineDeep" => GeocodingSummaryFeatureType::TypeSubmarineDeep,
                "typeSubmarineFractureZone" => {
                    GeocodingSummaryFeatureType::TypeSubmarineFractureZone
                }
                "typeSubmarineGap" => GeocodingSummaryFeatureType::TypeSubmarineGap,
                "typeSubmarinePlain" => GeocodingSummaryFeatureType::TypeSubmarinePlain,
                "typeSubmarinePlateau" => GeocodingSummaryFeatureType::TypeSubmarinePlateau,
                "typeSubmarineRidge" => GeocodingSummaryFeatureType::TypeSubmarineRidge,
                "typeSubmarineSeamount" => GeocodingSummaryFeatureType::TypeSubmarineSeamount,
                "typeSubmarineSlope" => GeocodingSummaryFeatureType::TypeSubmarineSlope,
                "typeSubmarineValley" => GeocodingSummaryFeatureType::TypeSubmarineValley,
                "typeSubwayStation" => GeocodingSummaryFeatureType::TypeSubwayStation,
                "typeSubwayTrack" => GeocodingSummaryFeatureType::TypeSubwayTrack,
                "typeSuite" => GeocodingSummaryFeatureType::TypeSuite,
                "typeSynagogue" => GeocodingSummaryFeatureType::TypeSynagogue,
                "typeTarmac" => GeocodingSummaryFeatureType::TypeTarmac,
                "typeTectonic" => GeocodingSummaryFeatureType::TypeTectonic,
                "typeTemple" => GeocodingSummaryFeatureType::TypeTemple,
                "typeTerminalPoint" => GeocodingSummaryFeatureType::TypeTerminalPoint,
                "typeTerrace" => GeocodingSummaryFeatureType::TypeTerrace,
                "typeTerrain" => GeocodingSummaryFeatureType::TypeTerrain,
                "typeTimezone" => GeocodingSummaryFeatureType::TypeTimezone,
                "typeTollCluster" => GeocodingSummaryFeatureType::TypeTollCluster,
                "typeTouristDestination" => GeocodingSummaryFeatureType::TypeTouristDestination,
                "typeTownSquare" => GeocodingSummaryFeatureType::TypeTownSquare,
                "typeTrail" => GeocodingSummaryFeatureType::TypeTrail,
                "typeTrailHead" => GeocodingSummaryFeatureType::TypeTrailHead,
                "typeTrainStation" => GeocodingSummaryFeatureType::TypeTrainStation,
                "typeTramwayStation" => GeocodingSummaryFeatureType::TypeTramwayStation,
                "typeTransient" => GeocodingSummaryFeatureType::TypeTransient,
                "typeTransit" => GeocodingSummaryFeatureType::TypeTransit,
                "typeTransitAgency" => GeocodingSummaryFeatureType::TypeTransitAgency,
                "typeTransitAgencyDeprecatedValue" => {
                    GeocodingSummaryFeatureType::TypeTransitAgencyDeprecatedValue
                }
                "typeTransitDeparture" => GeocodingSummaryFeatureType::TypeTransitDeparture,
                "typeTransitLeg" => GeocodingSummaryFeatureType::TypeTransitLeg,
                "typeTransitLine" => GeocodingSummaryFeatureType::TypeTransitLine,
                "typeTransitStation" => GeocodingSummaryFeatureType::TypeTransitStation,
                "typeTransitStop" => GeocodingSummaryFeatureType::TypeTransitStop,
                "typeTransitTransfer" => GeocodingSummaryFeatureType::TypeTransitTransfer,
                "typeTransitTrip" => GeocodingSummaryFeatureType::TypeTransitTrip,
                "typeTransportation" => GeocodingSummaryFeatureType::TypeTransportation,
                "typeTravelService" => GeocodingSummaryFeatureType::TypeTravelService,
                "typeTrolleyTrack" => GeocodingSummaryFeatureType::TypeTrolleyTrack,
                "typeTundra" => GeocodingSummaryFeatureType::TypeTundra,
                "typeUndersea" => GeocodingSummaryFeatureType::TypeUndersea,
                "typeUniversity" => GeocodingSummaryFeatureType::TypeUniversity,
                "typeUniversityGrounds" => GeocodingSummaryFeatureType::TypeUniversityGrounds,
                "typeUnknown" => GeocodingSummaryFeatureType::TypeUnknown,
                "typeUnstableHillside" => GeocodingSummaryFeatureType::TypeUnstableHillside,
                "typeUpland" => GeocodingSummaryFeatureType::TypeUpland,
                "typeUsBorough" => GeocodingSummaryFeatureType::TypeUsBorough,
                "typeUsNationalMonument" => GeocodingSummaryFeatureType::TypeUsNationalMonument,
                "typeUsNationalPark" => GeocodingSummaryFeatureType::TypeUsNationalPark,
                "typeUsState" => GeocodingSummaryFeatureType::TypeUsState,
                "typeVegetation" => GeocodingSummaryFeatureType::TypeVegetation,
                "typeVeterinarian" => GeocodingSummaryFeatureType::TypeVeterinarian,
                "typeVirtualSegment" => GeocodingSummaryFeatureType::TypeVirtualSegment,
                "typeVista" => GeocodingSummaryFeatureType::TypeVista,
                "typeVolcano" => GeocodingSummaryFeatureType::TypeVolcano,
                "typeWadi" => GeocodingSummaryFeatureType::TypeWadi,
                "typeWall" => GeocodingSummaryFeatureType::TypeWall,
                "typeWater" => GeocodingSummaryFeatureType::TypeWater,
                "typeWaterFountain" => GeocodingSummaryFeatureType::TypeWaterFountain,
                "typeWaterNavigation" => GeocodingSummaryFeatureType::TypeWaterNavigation,
                "typeWaterfall" => GeocodingSummaryFeatureType::TypeWaterfall,
                "typeWateringHole" => GeocodingSummaryFeatureType::TypeWateringHole,
                "typeWateringHoleDeprecated" => {
                    GeocodingSummaryFeatureType::TypeWateringHoleDeprecated
                }
                "typeWatershedBoundary" => GeocodingSummaryFeatureType::TypeWatershedBoundary,
                "typeWeatherCondition" => GeocodingSummaryFeatureType::TypeWeatherCondition,
                "typeWetland" => GeocodingSummaryFeatureType::TypeWetland,
                "typeWoods" => GeocodingSummaryFeatureType::TypeWoods,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GeocodingSummaryFeatureType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GeocodingSummaryFeatureType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GeocodingSummaryFeatureType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "typeAddressTemplate" => GeocodingSummaryFeatureType::TypeAddressTemplate,
                "typeAdministrativeArea" => GeocodingSummaryFeatureType::TypeAdministrativeArea,
                "typeAdministrativeArea1" => GeocodingSummaryFeatureType::TypeAdministrativeArea1,
                "typeAdministrativeArea2" => GeocodingSummaryFeatureType::TypeAdministrativeArea2,
                "typeAdministrativeArea3" => GeocodingSummaryFeatureType::TypeAdministrativeArea3,
                "typeAdministrativeArea4" => GeocodingSummaryFeatureType::TypeAdministrativeArea4,
                "typeAdministrativeArea5" => GeocodingSummaryFeatureType::TypeAdministrativeArea5,
                "typeAdministrativeArea6" => GeocodingSummaryFeatureType::TypeAdministrativeArea6,
                "typeAdministrativeArea7" => GeocodingSummaryFeatureType::TypeAdministrativeArea7,
                "typeAdministrativeArea8" => GeocodingSummaryFeatureType::TypeAdministrativeArea8,
                "typeAdministrativeArea9" => GeocodingSummaryFeatureType::TypeAdministrativeArea9,
                "typeAgricultural" => GeocodingSummaryFeatureType::TypeAgricultural,
                "typeAirport" => GeocodingSummaryFeatureType::TypeAirport,
                "typeAirportCivil" => GeocodingSummaryFeatureType::TypeAirportCivil,
                "typeAirportGrounds" => GeocodingSummaryFeatureType::TypeAirportGrounds,
                "typeAirportMilitary" => GeocodingSummaryFeatureType::TypeAirportMilitary,
                "typeAirportMixed" => GeocodingSummaryFeatureType::TypeAirportMixed,
                "typeAirstrip" => GeocodingSummaryFeatureType::TypeAirstrip,
                "typeAnimalEnclosure" => GeocodingSummaryFeatureType::TypeAnimalEnclosure,
                "typeAny" => GeocodingSummaryFeatureType::TypeAny,
                "typeArchipelago" => GeocodingSummaryFeatureType::TypeArchipelago,
                "typeAtoll" => GeocodingSummaryFeatureType::TypeAtoll,
                "typeBank" => GeocodingSummaryFeatureType::TypeBank,
                "typeBar" => GeocodingSummaryFeatureType::TypeBar,
                "typeBay" => GeocodingSummaryFeatureType::TypeBay,
                "typeBeach" => GeocodingSummaryFeatureType::TypeBeach,
                "typeBicycleRoute" => GeocodingSummaryFeatureType::TypeBicycleRoute,
                "typeBight" => GeocodingSummaryFeatureType::TypeBight,
                "typeBirdWatching" => GeocodingSummaryFeatureType::TypeBirdWatching,
                "typeBorder" => GeocodingSummaryFeatureType::TypeBorder,
                "typeBorderCrossing" => GeocodingSummaryFeatureType::TypeBorderCrossing,
                "typeBroadTrack" => GeocodingSummaryFeatureType::TypeBroadTrack,
                "typeBuilding" => GeocodingSummaryFeatureType::TypeBuilding,
                "typeBuildingGrounds" => GeocodingSummaryFeatureType::TypeBuildingGrounds,
                "typeBuiltUpArea" => GeocodingSummaryFeatureType::TypeBuiltUpArea,
                "typeBusStation" => GeocodingSummaryFeatureType::TypeBusStation,
                "typeBusiness" => GeocodingSummaryFeatureType::TypeBusiness,
                "typeBusinessChain" => GeocodingSummaryFeatureType::TypeBusinessChain,
                "typeBusinessCorridor" => GeocodingSummaryFeatureType::TypeBusinessCorridor,
                "typeCableCarStation" => GeocodingSummaryFeatureType::TypeCableCarStation,
                "typeCampfirePit" => GeocodingSummaryFeatureType::TypeCampfirePit,
                "typeCampgrounds" => GeocodingSummaryFeatureType::TypeCampgrounds,
                "typeCampingSite" => GeocodingSummaryFeatureType::TypeCampingSite,
                "typeCanal" => GeocodingSummaryFeatureType::TypeCanal,
                "typeCarRental" => GeocodingSummaryFeatureType::TypeCarRental,
                "typeCarRepair" => GeocodingSummaryFeatureType::TypeCarRepair,
                "typeCartographic" => GeocodingSummaryFeatureType::TypeCartographic,
                "typeCartographicLine" => GeocodingSummaryFeatureType::TypeCartographicLine,
                "typeCashMachine" => GeocodingSummaryFeatureType::TypeCashMachine,
                "typeCave" => GeocodingSummaryFeatureType::TypeCave,
                "typeCay" => GeocodingSummaryFeatureType::TypeCay,
                "typeCelestial" => GeocodingSummaryFeatureType::TypeCelestial,
                "typeCemetery" => GeocodingSummaryFeatureType::TypeCemetery,
                "typeChannel" => GeocodingSummaryFeatureType::TypeChannel,
                "typeChurch" => GeocodingSummaryFeatureType::TypeChurch,
                "typeCityHall" => GeocodingSummaryFeatureType::TypeCityHall,
                "typeCliff" => GeocodingSummaryFeatureType::TypeCliff,
                "typeCoffee" => GeocodingSummaryFeatureType::TypeCoffee,
                "typeColloquialArea" => GeocodingSummaryFeatureType::TypeColloquialArea,
                "typeColloquialCity" => GeocodingSummaryFeatureType::TypeColloquialCity,
                "typeCompound" => GeocodingSummaryFeatureType::TypeCompound,
                "typeCompoundBuilding" => GeocodingSummaryFeatureType::TypeCompoundBuilding,
                "typeCompoundGrounds" => GeocodingSummaryFeatureType::TypeCompoundGrounds,
                "typeCompoundSection" => GeocodingSummaryFeatureType::TypeCompoundSection,
                "typeConfluence" => GeocodingSummaryFeatureType::TypeConfluence,
                "typeConstituency" => GeocodingSummaryFeatureType::TypeConstituency,
                "typeConstituencyFuture" => GeocodingSummaryFeatureType::TypeConstituencyFuture,
                "typeContinent" => GeocodingSummaryFeatureType::TypeContinent,
                "typeContourLine" => GeocodingSummaryFeatureType::TypeContourLine,
                "typeCountry" => GeocodingSummaryFeatureType::TypeCountry,
                "typeCourthouse" => GeocodingSummaryFeatureType::TypeCourthouse,
                "typeCrater" => GeocodingSummaryFeatureType::TypeCrater,
                "typeCurrent" => GeocodingSummaryFeatureType::TypeCurrent,
                "typeDam" => GeocodingSummaryFeatureType::TypeDam,
                "typeDataSource" => GeocodingSummaryFeatureType::TypeDataSource,
                "typeDentist" => GeocodingSummaryFeatureType::TypeDentist,
                "typeDeprecatedGolfShop" => GeocodingSummaryFeatureType::TypeDeprecatedGolfShop,
                "typeDeprecatedHighwayDoNotUse" => {
                    GeocodingSummaryFeatureType::TypeDeprecatedHighwayDoNotUse
                }
                "typeDeprecatedTarmac" => GeocodingSummaryFeatureType::TypeDeprecatedTarmac,
                "typeDesert" => GeocodingSummaryFeatureType::TypeDesert,
                "typeDesignatedBarbecuePit" => {
                    GeocodingSummaryFeatureType::TypeDesignatedBarbecuePit
                }
                "typeDesignatedCookingArea" => {
                    GeocodingSummaryFeatureType::TypeDesignatedCookingArea
                }
                "typeDesignatedMarketArea" => GeocodingSummaryFeatureType::TypeDesignatedMarketArea,
                "typeDigitalElevationModel" => {
                    GeocodingSummaryFeatureType::TypeDigitalElevationModel
                }
                "typeDisputedArea" => GeocodingSummaryFeatureType::TypeDisputedArea,
                "typeDistributary" => GeocodingSummaryFeatureType::TypeDistributary,
                "typeDoNotUseReservedToCatchGeneratedFiles" => {
                    GeocodingSummaryFeatureType::TypeDoNotUseReservedToCatchGeneratedFiles
                }
                "typeDoctor" => GeocodingSummaryFeatureType::TypeDoctor,
                "typeDoodle" => GeocodingSummaryFeatureType::TypeDoodle,
                "typeDrinkingWater" => GeocodingSummaryFeatureType::TypeDrinkingWater,
                "typeDune" => GeocodingSummaryFeatureType::TypeDune,
                "typeEarthquake" => GeocodingSummaryFeatureType::TypeEarthquake,
                "typeEcoTouristDestination" => {
                    GeocodingSummaryFeatureType::TypeEcoTouristDestination
                }
                "typeElevated" => GeocodingSummaryFeatureType::TypeElevated,
                "typeEmbassy" => GeocodingSummaryFeatureType::TypeEmbassy,
                "typeEmergency" => GeocodingSummaryFeatureType::TypeEmergency,
                "typeEnclosedTrafficArea" => GeocodingSummaryFeatureType::TypeEnclosedTrafficArea,
                "typeEntrance" => GeocodingSummaryFeatureType::TypeEntrance,
                "typeEstablishment" => GeocodingSummaryFeatureType::TypeEstablishment,
                "typeEstablishmentBuilding" => {
                    GeocodingSummaryFeatureType::TypeEstablishmentBuilding
                }
                "typeEstablishmentGrounds" => GeocodingSummaryFeatureType::TypeEstablishmentGrounds,
                "typeEstablishmentPoi" => GeocodingSummaryFeatureType::TypeEstablishmentPoi,
                "typeEstablishmentService" => GeocodingSummaryFeatureType::TypeEstablishmentService,
                "typeEstuary" => GeocodingSummaryFeatureType::TypeEstuary,
                "typeEvent" => GeocodingSummaryFeatureType::TypeEvent,
                "typeFault" => GeocodingSummaryFeatureType::TypeFault,
                "typeFerry" => GeocodingSummaryFeatureType::TypeFerry,
                "typeFerryBoat" => GeocodingSummaryFeatureType::TypeFerryBoat,
                "typeFerryTerminal" => GeocodingSummaryFeatureType::TypeFerryTerminal,
                "typeFerryTrain" => GeocodingSummaryFeatureType::TypeFerryTrain,
                "typeFire" => GeocodingSummaryFeatureType::TypeFire,
                "typeFishing" => GeocodingSummaryFeatureType::TypeFishing,
                "typeFissure" => GeocodingSummaryFeatureType::TypeFissure,
                "typeFjord" => GeocodingSummaryFeatureType::TypeFjord,
                "typeFord" => GeocodingSummaryFeatureType::TypeFord,
                "typeFunicularStation" => GeocodingSummaryFeatureType::TypeFunicularStation,
                "typeFutureGeometry" => GeocodingSummaryFeatureType::TypeFutureGeometry,
                "typeGasStation" => GeocodingSummaryFeatureType::TypeGasStation,
                "typeGbCountry" => GeocodingSummaryFeatureType::TypeGbCountry,
                "typeGbDependentLocality" => GeocodingSummaryFeatureType::TypeGbDependentLocality,
                "typeGbDoubleDependentLocality" => {
                    GeocodingSummaryFeatureType::TypeGbDoubleDependentLocality
                }
                "typeGbFormerPostalCounty" => GeocodingSummaryFeatureType::TypeGbFormerPostalCounty,
                "typeGbPostTown" => GeocodingSummaryFeatureType::TypeGbPostTown,
                "typeGbTraditionalCounty" => GeocodingSummaryFeatureType::TypeGbTraditionalCounty,
                "typeGeocodedAddress" => GeocodingSummaryFeatureType::TypeGeocodedAddress,
                "typeGeyser" => GeocodingSummaryFeatureType::TypeGeyser,
                "typeGlacier" => GeocodingSummaryFeatureType::TypeGlacier,
                "typeGolf" => GeocodingSummaryFeatureType::TypeGolf,
                "typeGolfCourse" => GeocodingSummaryFeatureType::TypeGolfCourse,
                "typeGolfFairway" => GeocodingSummaryFeatureType::TypeGolfFairway,
                "typeGolfHole" => GeocodingSummaryFeatureType::TypeGolfHole,
                "typeGolfPuttingGreen" => GeocodingSummaryFeatureType::TypeGolfPuttingGreen,
                "typeGolfRough" => GeocodingSummaryFeatureType::TypeGolfRough,
                "typeGolfSandBunker" => GeocodingSummaryFeatureType::TypeGolfSandBunker,
                "typeGolfTeeingGround" => GeocodingSummaryFeatureType::TypeGolfTeeingGround,
                "typeGondolaLiftStation" => GeocodingSummaryFeatureType::TypeGondolaLiftStation,
                "typeGovernment" => GeocodingSummaryFeatureType::TypeGovernment,
                "typeGrassland" => GeocodingSummaryFeatureType::TypeGrassland,
                "typeGrocery" => GeocodingSummaryFeatureType::TypeGrocery,
                "typeGrounds" => GeocodingSummaryFeatureType::TypeGrounds,
                "typeGurudwara" => GeocodingSummaryFeatureType::TypeGurudwara,
                "typeHarbor" => GeocodingSummaryFeatureType::TypeHarbor,
                "typeHeliport" => GeocodingSummaryFeatureType::TypeHeliport,
                "typeHighSpeedRail" => GeocodingSummaryFeatureType::TypeHighSpeedRail,
                "typeHighTension" => GeocodingSummaryFeatureType::TypeHighTension,
                "typeHighway" => GeocodingSummaryFeatureType::TypeHighway,
                "typeHighway1" => GeocodingSummaryFeatureType::TypeHighway1,
                "typeHighway2" => GeocodingSummaryFeatureType::TypeHighway2,
                "typeHighway3" => GeocodingSummaryFeatureType::TypeHighway3,
                "typeHighway4" => GeocodingSummaryFeatureType::TypeHighway4,
                "typeHighway5" => GeocodingSummaryFeatureType::TypeHighway5,
                "typeHighway6" => GeocodingSummaryFeatureType::TypeHighway6,
                "typeHighway7" => GeocodingSummaryFeatureType::TypeHighway7,
                "typeHighway8" => GeocodingSummaryFeatureType::TypeHighway8,
                "typeHighway9" => GeocodingSummaryFeatureType::TypeHighway9,
                "typeHikingArea" => GeocodingSummaryFeatureType::TypeHikingArea,
                "typeHinduTemple" => GeocodingSummaryFeatureType::TypeHinduTemple,
                "typeHorseCarriageStation" => GeocodingSummaryFeatureType::TypeHorseCarriageStation,
                "typeHospital" => GeocodingSummaryFeatureType::TypeHospital,
                "typeHospitalGrounds" => GeocodingSummaryFeatureType::TypeHospitalGrounds,
                "typeHotSpring" => GeocodingSummaryFeatureType::TypeHotSpring,
                "typeHunting" => GeocodingSummaryFeatureType::TypeHunting,
                "typeHurricane" => GeocodingSummaryFeatureType::TypeHurricane,
                "typeIce" => GeocodingSummaryFeatureType::TypeIce,
                "typeIndustrial" => GeocodingSummaryFeatureType::TypeIndustrial,
                "typeInlet" => GeocodingSummaryFeatureType::TypeInlet,
                "typeIntersection" => GeocodingSummaryFeatureType::TypeIntersection,
                "typeIntersectionGroup" => GeocodingSummaryFeatureType::TypeIntersectionGroup,
                "typeIrrigation" => GeocodingSummaryFeatureType::TypeIrrigation,
                "typeIsland" => GeocodingSummaryFeatureType::TypeIsland,
                "typeIsthmus" => GeocodingSummaryFeatureType::TypeIsthmus,
                "typeJpChiban" => GeocodingSummaryFeatureType::TypeJpChiban,
                "typeJpEdaban" => GeocodingSummaryFeatureType::TypeJpEdaban,
                "typeJpGaiku" => GeocodingSummaryFeatureType::TypeJpGaiku,
                "typeJpGun" => GeocodingSummaryFeatureType::TypeJpGun,
                "typeJpKoaza" => GeocodingSummaryFeatureType::TypeJpKoaza,
                "typeJpOoaza" => GeocodingSummaryFeatureType::TypeJpOoaza,
                "typeJpShikuchouson" => GeocodingSummaryFeatureType::TypeJpShikuchouson,
                "typeJpSubShikuchouson" => GeocodingSummaryFeatureType::TypeJpSubShikuchouson,
                "typeJpTodoufuken" => GeocodingSummaryFeatureType::TypeJpTodoufuken,
                "typeJrTrack" => GeocodingSummaryFeatureType::TypeJrTrack,
                "typeKarst" => GeocodingSummaryFeatureType::TypeKarst,
                "typeLagoon" => GeocodingSummaryFeatureType::TypeLagoon,
                "typeLake" => GeocodingSummaryFeatureType::TypeLake,
                "typeLandMass" => GeocodingSummaryFeatureType::TypeLandMass,
                "typeLandParcel" => GeocodingSummaryFeatureType::TypeLandParcel,
                "typeLavaField" => GeocodingSummaryFeatureType::TypeLavaField,
                "typeLevel" => GeocodingSummaryFeatureType::TypeLevel,
                "typeLibrary" => GeocodingSummaryFeatureType::TypeLibrary,
                "typeLightRailTrack" => GeocodingSummaryFeatureType::TypeLightRailTrack,
                "typeLitterReceptacle" => GeocodingSummaryFeatureType::TypeLitterReceptacle,
                "typeLocalPark" => GeocodingSummaryFeatureType::TypeLocalPark,
                "typeLocale" => GeocodingSummaryFeatureType::TypeLocale,
                "typeLocality" => GeocodingSummaryFeatureType::TypeLocality,
                "typeLockerArea" => GeocodingSummaryFeatureType::TypeLockerArea,
                "typeLodging" => GeocodingSummaryFeatureType::TypeLodging,
                "typeMetaFeature" => GeocodingSummaryFeatureType::TypeMetaFeature,
                "typeMilitary" => GeocodingSummaryFeatureType::TypeMilitary,
                "typeMonorailStation" => GeocodingSummaryFeatureType::TypeMonorailStation,
                "typeMonorailTrack" => GeocodingSummaryFeatureType::TypeMonorailTrack,
                "typeMosque" => GeocodingSummaryFeatureType::TypeMosque,
                "typeMountainRange" => GeocodingSummaryFeatureType::TypeMountainRange,
                "typeMovieRental" => GeocodingSummaryFeatureType::TypeMovieRental,
                "typeNarrowTrack" => GeocodingSummaryFeatureType::TypeNarrowTrack,
                "typeNationalForest" => GeocodingSummaryFeatureType::TypeNationalForest,
                "typeNationalPark" => GeocodingSummaryFeatureType::TypeNationalPark,
                "typeNaturalFeature" => GeocodingSummaryFeatureType::TypeNaturalFeature,
                "typeNatureReserve" => GeocodingSummaryFeatureType::TypeNatureReserve,
                "typeNeighborhood" => GeocodingSummaryFeatureType::TypeNeighborhood,
                "typeNunatak" => GeocodingSummaryFeatureType::TypeNunatak,
                "typeOcean" => GeocodingSummaryFeatureType::TypeOcean,
                "typeOceanRockExposed" => GeocodingSummaryFeatureType::TypeOceanRockExposed,
                "typeOffRoadArea" => GeocodingSummaryFeatureType::TypeOffRoadArea,
                "typePan" => GeocodingSummaryFeatureType::TypePan,
                "typePark" => GeocodingSummaryFeatureType::TypePark,
                "typeParking" => GeocodingSummaryFeatureType::TypeParking,
                "typeParkingGarage" => GeocodingSummaryFeatureType::TypeParkingGarage,
                "typeParkingLot" => GeocodingSummaryFeatureType::TypeParkingLot,
                "typePass" => GeocodingSummaryFeatureType::TypePass,
                "typePathway" => GeocodingSummaryFeatureType::TypePathway,
                "typePeak" => GeocodingSummaryFeatureType::TypePeak,
                "typePeninsula" => GeocodingSummaryFeatureType::TypePeninsula,
                "typePharmacy" => GeocodingSummaryFeatureType::TypePharmacy,
                "typePhoneNumberAreaCode" => GeocodingSummaryFeatureType::TypePhoneNumberAreaCode,
                "typePhoneNumberPrefix" => GeocodingSummaryFeatureType::TypePhoneNumberPrefix,
                "typePicnicArea" => GeocodingSummaryFeatureType::TypePicnicArea,
                "typePlateau" => GeocodingSummaryFeatureType::TypePlateau,
                "typePlayGround" => GeocodingSummaryFeatureType::TypePlayGround,
                "typePolice" => GeocodingSummaryFeatureType::TypePolice,
                "typePoliceJurisdiction" => GeocodingSummaryFeatureType::TypePoliceJurisdiction,
                "typePolitical" => GeocodingSummaryFeatureType::TypePolitical,
                "typePond" => GeocodingSummaryFeatureType::TypePond,
                "typePostOffice" => GeocodingSummaryFeatureType::TypePostOffice,
                "typePostTown" => GeocodingSummaryFeatureType::TypePostTown,
                "typePostal" => GeocodingSummaryFeatureType::TypePostal,
                "typePostalCode" => GeocodingSummaryFeatureType::TypePostalCode,
                "typePostalCodePrefix" => GeocodingSummaryFeatureType::TypePostalCodePrefix,
                "typePostalRound" => GeocodingSummaryFeatureType::TypePostalRound,
                "typePremise" => GeocodingSummaryFeatureType::TypePremise,
                "typeProvincialForest" => GeocodingSummaryFeatureType::TypeProvincialForest,
                "typeProvincialPark" => GeocodingSummaryFeatureType::TypeProvincialPark,
                "typePublicSpacesAndMonuments" => {
                    GeocodingSummaryFeatureType::TypePublicSpacesAndMonuments
                }
                "typeRailway" => GeocodingSummaryFeatureType::TypeRailway,
                "typeRapids" => GeocodingSummaryFeatureType::TypeRapids,
                "typeRavine" => GeocodingSummaryFeatureType::TypeRavine,
                "typeReef" => GeocodingSummaryFeatureType::TypeReef,
                "typeReefExtent" => GeocodingSummaryFeatureType::TypeReefExtent,
                "typeReefFlat" => GeocodingSummaryFeatureType::TypeReefFlat,
                "typeReefGrowth" => GeocodingSummaryFeatureType::TypeReefGrowth,
                "typeReefRockSubmerged" => GeocodingSummaryFeatureType::TypeReefRockSubmerged,
                "typeRegulatedArea" => GeocodingSummaryFeatureType::TypeRegulatedArea,
                "typeReservation" => GeocodingSummaryFeatureType::TypeReservation,
                "typeReservoir" => GeocodingSummaryFeatureType::TypeReservoir,
                "typeRestArea" => GeocodingSummaryFeatureType::TypeRestArea,
                "typeRestaurant" => GeocodingSummaryFeatureType::TypeRestaurant,
                "typeRestrictionGroup" => GeocodingSummaryFeatureType::TypeRestrictionGroup,
                "typeRidge" => GeocodingSummaryFeatureType::TypeRidge,
                "typeRiver" => GeocodingSummaryFeatureType::TypeRiver,
                "typeRoad" => GeocodingSummaryFeatureType::TypeRoad,
                "typeRoadMonitor" => GeocodingSummaryFeatureType::TypeRoadMonitor,
                "typeRoadSign" => GeocodingSummaryFeatureType::TypeRoadSign,
                "typeRock" => GeocodingSummaryFeatureType::TypeRock,
                "typeRocky" => GeocodingSummaryFeatureType::TypeRocky,
                "typeRoute" => GeocodingSummaryFeatureType::TypeRoute,
                "typeSaltFlat" => GeocodingSummaryFeatureType::TypeSaltFlat,
                "typeSand" => GeocodingSummaryFeatureType::TypeSand,
                "typeSchool" => GeocodingSummaryFeatureType::TypeSchool,
                "typeSchoolDistrict" => GeocodingSummaryFeatureType::TypeSchoolDistrict,
                "typeSea" => GeocodingSummaryFeatureType::TypeSea,
                "typeSeaplaneBase" => GeocodingSummaryFeatureType::TypeSeaplaneBase,
                "typeSeaport" => GeocodingSummaryFeatureType::TypeSeaport,
                "typeSeasonalLake" => GeocodingSummaryFeatureType::TypeSeasonalLake,
                "typeSeasonalRiver" => GeocodingSummaryFeatureType::TypeSeasonalRiver,
                "typeSegment" => GeocodingSummaryFeatureType::TypeSegment,
                "typeSegmentPath" => GeocodingSummaryFeatureType::TypeSegmentPath,
                "typeShopping" => GeocodingSummaryFeatureType::TypeShopping,
                "typeShoppingCenter" => GeocodingSummaryFeatureType::TypeShoppingCenter,
                "typeShrubbery" => GeocodingSummaryFeatureType::TypeShrubbery,
                "typeSkiBoundary" => GeocodingSummaryFeatureType::TypeSkiBoundary,
                "typeSkiLift" => GeocodingSummaryFeatureType::TypeSkiLift,
                "typeSkiTrail" => GeocodingSummaryFeatureType::TypeSkiTrail,
                "typeSlope" => GeocodingSummaryFeatureType::TypeSlope,
                "typeSpecialStation" => GeocodingSummaryFeatureType::TypeSpecialStation,
                "typeSportsComplex" => GeocodingSummaryFeatureType::TypeSportsComplex,
                "typeSpring" => GeocodingSummaryFeatureType::TypeSpring,
                "typeSpur" => GeocodingSummaryFeatureType::TypeSpur,
                "typeStadium" => GeocodingSummaryFeatureType::TypeStadium,
                "typeStandardTrack" => GeocodingSummaryFeatureType::TypeStandardTrack,
                "typeStatisticalArea" => GeocodingSummaryFeatureType::TypeStatisticalArea,
                "typeStatue" => GeocodingSummaryFeatureType::TypeStatue,
                "typeStrait" => GeocodingSummaryFeatureType::TypeStrait,
                "typeSubPremise" => GeocodingSummaryFeatureType::TypeSubPremise,
                "typeSublocality" => GeocodingSummaryFeatureType::TypeSublocality,
                "typeSublocality1" => GeocodingSummaryFeatureType::TypeSublocality1,
                "typeSublocality2" => GeocodingSummaryFeatureType::TypeSublocality2,
                "typeSublocality3" => GeocodingSummaryFeatureType::TypeSublocality3,
                "typeSublocality4" => GeocodingSummaryFeatureType::TypeSublocality4,
                "typeSublocality5" => GeocodingSummaryFeatureType::TypeSublocality5,
                "typeSubmarineBasin" => GeocodingSummaryFeatureType::TypeSubmarineBasin,
                "typeSubmarineCliff" => GeocodingSummaryFeatureType::TypeSubmarineCliff,
                "typeSubmarineDeep" => GeocodingSummaryFeatureType::TypeSubmarineDeep,
                "typeSubmarineFractureZone" => {
                    GeocodingSummaryFeatureType::TypeSubmarineFractureZone
                }
                "typeSubmarineGap" => GeocodingSummaryFeatureType::TypeSubmarineGap,
                "typeSubmarinePlain" => GeocodingSummaryFeatureType::TypeSubmarinePlain,
                "typeSubmarinePlateau" => GeocodingSummaryFeatureType::TypeSubmarinePlateau,
                "typeSubmarineRidge" => GeocodingSummaryFeatureType::TypeSubmarineRidge,
                "typeSubmarineSeamount" => GeocodingSummaryFeatureType::TypeSubmarineSeamount,
                "typeSubmarineSlope" => GeocodingSummaryFeatureType::TypeSubmarineSlope,
                "typeSubmarineValley" => GeocodingSummaryFeatureType::TypeSubmarineValley,
                "typeSubwayStation" => GeocodingSummaryFeatureType::TypeSubwayStation,
                "typeSubwayTrack" => GeocodingSummaryFeatureType::TypeSubwayTrack,
                "typeSuite" => GeocodingSummaryFeatureType::TypeSuite,
                "typeSynagogue" => GeocodingSummaryFeatureType::TypeSynagogue,
                "typeTarmac" => GeocodingSummaryFeatureType::TypeTarmac,
                "typeTectonic" => GeocodingSummaryFeatureType::TypeTectonic,
                "typeTemple" => GeocodingSummaryFeatureType::TypeTemple,
                "typeTerminalPoint" => GeocodingSummaryFeatureType::TypeTerminalPoint,
                "typeTerrace" => GeocodingSummaryFeatureType::TypeTerrace,
                "typeTerrain" => GeocodingSummaryFeatureType::TypeTerrain,
                "typeTimezone" => GeocodingSummaryFeatureType::TypeTimezone,
                "typeTollCluster" => GeocodingSummaryFeatureType::TypeTollCluster,
                "typeTouristDestination" => GeocodingSummaryFeatureType::TypeTouristDestination,
                "typeTownSquare" => GeocodingSummaryFeatureType::TypeTownSquare,
                "typeTrail" => GeocodingSummaryFeatureType::TypeTrail,
                "typeTrailHead" => GeocodingSummaryFeatureType::TypeTrailHead,
                "typeTrainStation" => GeocodingSummaryFeatureType::TypeTrainStation,
                "typeTramwayStation" => GeocodingSummaryFeatureType::TypeTramwayStation,
                "typeTransient" => GeocodingSummaryFeatureType::TypeTransient,
                "typeTransit" => GeocodingSummaryFeatureType::TypeTransit,
                "typeTransitAgency" => GeocodingSummaryFeatureType::TypeTransitAgency,
                "typeTransitAgencyDeprecatedValue" => {
                    GeocodingSummaryFeatureType::TypeTransitAgencyDeprecatedValue
                }
                "typeTransitDeparture" => GeocodingSummaryFeatureType::TypeTransitDeparture,
                "typeTransitLeg" => GeocodingSummaryFeatureType::TypeTransitLeg,
                "typeTransitLine" => GeocodingSummaryFeatureType::TypeTransitLine,
                "typeTransitStation" => GeocodingSummaryFeatureType::TypeTransitStation,
                "typeTransitStop" => GeocodingSummaryFeatureType::TypeTransitStop,
                "typeTransitTransfer" => GeocodingSummaryFeatureType::TypeTransitTransfer,
                "typeTransitTrip" => GeocodingSummaryFeatureType::TypeTransitTrip,
                "typeTransportation" => GeocodingSummaryFeatureType::TypeTransportation,
                "typeTravelService" => GeocodingSummaryFeatureType::TypeTravelService,
                "typeTrolleyTrack" => GeocodingSummaryFeatureType::TypeTrolleyTrack,
                "typeTundra" => GeocodingSummaryFeatureType::TypeTundra,
                "typeUndersea" => GeocodingSummaryFeatureType::TypeUndersea,
                "typeUniversity" => GeocodingSummaryFeatureType::TypeUniversity,
                "typeUniversityGrounds" => GeocodingSummaryFeatureType::TypeUniversityGrounds,
                "typeUnknown" => GeocodingSummaryFeatureType::TypeUnknown,
                "typeUnstableHillside" => GeocodingSummaryFeatureType::TypeUnstableHillside,
                "typeUpland" => GeocodingSummaryFeatureType::TypeUpland,
                "typeUsBorough" => GeocodingSummaryFeatureType::TypeUsBorough,
                "typeUsNationalMonument" => GeocodingSummaryFeatureType::TypeUsNationalMonument,
                "typeUsNationalPark" => GeocodingSummaryFeatureType::TypeUsNationalPark,
                "typeUsState" => GeocodingSummaryFeatureType::TypeUsState,
                "typeVegetation" => GeocodingSummaryFeatureType::TypeVegetation,
                "typeVeterinarian" => GeocodingSummaryFeatureType::TypeVeterinarian,
                "typeVirtualSegment" => GeocodingSummaryFeatureType::TypeVirtualSegment,
                "typeVista" => GeocodingSummaryFeatureType::TypeVista,
                "typeVolcano" => GeocodingSummaryFeatureType::TypeVolcano,
                "typeWadi" => GeocodingSummaryFeatureType::TypeWadi,
                "typeWall" => GeocodingSummaryFeatureType::TypeWall,
                "typeWater" => GeocodingSummaryFeatureType::TypeWater,
                "typeWaterFountain" => GeocodingSummaryFeatureType::TypeWaterFountain,
                "typeWaterNavigation" => GeocodingSummaryFeatureType::TypeWaterNavigation,
                "typeWaterfall" => GeocodingSummaryFeatureType::TypeWaterfall,
                "typeWateringHole" => GeocodingSummaryFeatureType::TypeWateringHole,
                "typeWateringHoleDeprecated" => {
                    GeocodingSummaryFeatureType::TypeWateringHoleDeprecated
                }
                "typeWatershedBoundary" => GeocodingSummaryFeatureType::TypeWatershedBoundary,
                "typeWeatherCondition" => GeocodingSummaryFeatureType::TypeWeatherCondition,
                "typeWetland" => GeocodingSummaryFeatureType::TypeWetland,
                "typeWoods" => GeocodingSummaryFeatureType::TypeWoods,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GeocodingSummaryFeatureType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GeocodingSummaryFeatureType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GeographicDivision {
        #[doc = "Any other valid OCD IDs that refer to the same division.\\n\\nBecause OCD IDs are meant to be human-readable and at least somewhat predictable, there are occasionally several identifiers for a single division. These identifiers are defined to be equivalent to one another, and one is always indicated as the primary identifier. The primary identifier will be returned in ocd_id above, and any other equivalent valid identifiers will be returned in this list.\\n\\nFor example, if this division’s OCD ID is ocd-division/country:us/district:dc, this will contain ocd-division/country:us/state:dc."]
        #[serde(
            rename = "alsoKnownAs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub also_known_as: ::std::option::Option<Vec<String>>,
        #[doc = "The name of the division."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "List of indices in the offices array, one for each office elected from this division. Will only be present if includeOffices was true (or absent) in the request."]
        #[serde(
            rename = "officeIndices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub office_indices: ::std::option::Option<Vec<u32>>,
    }
    impl ::google_field_selector::FieldSelector for GeographicDivision {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GeographicDivision {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct MessageSet {}
    impl ::google_field_selector::FieldSelector for MessageSet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MessageSet {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Office {
        #[doc = "The OCD ID of the division with which this office is associated."]
        #[serde(
            rename = "divisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub division_id: ::std::option::Option<String>,
        #[doc = "The levels of government of which this office is part. There may be more than one in cases where a jurisdiction effectively acts at two different levels of government; for example, the mayor of the District of Columbia acts at “locality” level, but also effectively at both “administrative-area-2” and “administrative-area-1”."]
        #[serde(
            rename = "levels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub levels: ::std::option::Option<Vec<crate::schemas::OfficeLevelsItems>>,
        #[doc = "The human-readable name of the office."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "List of indices in the officials array of people who presently hold this office."]
        #[serde(
            rename = "officialIndices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub official_indices: ::std::option::Option<Vec<u32>>,
        #[doc = "The roles which this office fulfills. Roles are not meant to be exhaustive, or to exactly specify the entire set of responsibilities of a given office, but are meant to be rough categories that are useful for general selection from or sorting of a list of offices."]
        #[serde(
            rename = "roles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub roles: ::std::option::Option<Vec<crate::schemas::OfficeRolesItems>>,
        #[doc = "A list of sources for this office. If multiple sources are listed, the data has been aggregated from those sources."]
        #[serde(
            rename = "sources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sources: ::std::option::Option<Vec<crate::schemas::Source>>,
    }
    impl ::google_field_selector::FieldSelector for Office {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Office {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OfficeLevelsItems {
        AdministrativeArea1,
        AdministrativeArea2,
        Country,
        International,
        Locality,
        Regional,
        Special,
        SubLocality1,
        SubLocality2,
    }
    impl OfficeLevelsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                OfficeLevelsItems::AdministrativeArea1 => "administrativeArea1",
                OfficeLevelsItems::AdministrativeArea2 => "administrativeArea2",
                OfficeLevelsItems::Country => "country",
                OfficeLevelsItems::International => "international",
                OfficeLevelsItems::Locality => "locality",
                OfficeLevelsItems::Regional => "regional",
                OfficeLevelsItems::Special => "special",
                OfficeLevelsItems::SubLocality1 => "subLocality1",
                OfficeLevelsItems::SubLocality2 => "subLocality2",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OfficeLevelsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OfficeLevelsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OfficeLevelsItems, ()> {
            Ok(match s {
                "administrativeArea1" => OfficeLevelsItems::AdministrativeArea1,
                "administrativeArea2" => OfficeLevelsItems::AdministrativeArea2,
                "country" => OfficeLevelsItems::Country,
                "international" => OfficeLevelsItems::International,
                "locality" => OfficeLevelsItems::Locality,
                "regional" => OfficeLevelsItems::Regional,
                "special" => OfficeLevelsItems::Special,
                "subLocality1" => OfficeLevelsItems::SubLocality1,
                "subLocality2" => OfficeLevelsItems::SubLocality2,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OfficeLevelsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OfficeLevelsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OfficeLevelsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "administrativeArea1" => OfficeLevelsItems::AdministrativeArea1,
                "administrativeArea2" => OfficeLevelsItems::AdministrativeArea2,
                "country" => OfficeLevelsItems::Country,
                "international" => OfficeLevelsItems::International,
                "locality" => OfficeLevelsItems::Locality,
                "regional" => OfficeLevelsItems::Regional,
                "special" => OfficeLevelsItems::Special,
                "subLocality1" => OfficeLevelsItems::SubLocality1,
                "subLocality2" => OfficeLevelsItems::SubLocality2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OfficeLevelsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OfficeLevelsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum OfficeRolesItems {
        DeputyHeadOfGovernment,
        ExecutiveCouncil,
        GovernmentOfficer,
        HeadOfGovernment,
        HeadOfState,
        HighestCourtJudge,
        Judge,
        LegislatorLowerBody,
        LegislatorUpperBody,
        OtherRole,
        SchoolBoard,
        SpecialPurposeOfficer,
    }
    impl OfficeRolesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                OfficeRolesItems::DeputyHeadOfGovernment => "deputyHeadOfGovernment",
                OfficeRolesItems::ExecutiveCouncil => "executiveCouncil",
                OfficeRolesItems::GovernmentOfficer => "governmentOfficer",
                OfficeRolesItems::HeadOfGovernment => "headOfGovernment",
                OfficeRolesItems::HeadOfState => "headOfState",
                OfficeRolesItems::HighestCourtJudge => "highestCourtJudge",
                OfficeRolesItems::Judge => "judge",
                OfficeRolesItems::LegislatorLowerBody => "legislatorLowerBody",
                OfficeRolesItems::LegislatorUpperBody => "legislatorUpperBody",
                OfficeRolesItems::OtherRole => "otherRole",
                OfficeRolesItems::SchoolBoard => "schoolBoard",
                OfficeRolesItems::SpecialPurposeOfficer => "specialPurposeOfficer",
            }
        }
    }
    impl ::std::convert::AsRef<str> for OfficeRolesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for OfficeRolesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<OfficeRolesItems, ()> {
            Ok(match s {
                "deputyHeadOfGovernment" => OfficeRolesItems::DeputyHeadOfGovernment,
                "executiveCouncil" => OfficeRolesItems::ExecutiveCouncil,
                "governmentOfficer" => OfficeRolesItems::GovernmentOfficer,
                "headOfGovernment" => OfficeRolesItems::HeadOfGovernment,
                "headOfState" => OfficeRolesItems::HeadOfState,
                "highestCourtJudge" => OfficeRolesItems::HighestCourtJudge,
                "judge" => OfficeRolesItems::Judge,
                "legislatorLowerBody" => OfficeRolesItems::LegislatorLowerBody,
                "legislatorUpperBody" => OfficeRolesItems::LegislatorUpperBody,
                "otherRole" => OfficeRolesItems::OtherRole,
                "schoolBoard" => OfficeRolesItems::SchoolBoard,
                "specialPurposeOfficer" => OfficeRolesItems::SpecialPurposeOfficer,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for OfficeRolesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for OfficeRolesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for OfficeRolesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "deputyHeadOfGovernment" => OfficeRolesItems::DeputyHeadOfGovernment,
                "executiveCouncil" => OfficeRolesItems::ExecutiveCouncil,
                "governmentOfficer" => OfficeRolesItems::GovernmentOfficer,
                "headOfGovernment" => OfficeRolesItems::HeadOfGovernment,
                "headOfState" => OfficeRolesItems::HeadOfState,
                "highestCourtJudge" => OfficeRolesItems::HighestCourtJudge,
                "judge" => OfficeRolesItems::Judge,
                "legislatorLowerBody" => OfficeRolesItems::LegislatorLowerBody,
                "legislatorUpperBody" => OfficeRolesItems::LegislatorUpperBody,
                "otherRole" => OfficeRolesItems::OtherRole,
                "schoolBoard" => OfficeRolesItems::SchoolBoard,
                "specialPurposeOfficer" => OfficeRolesItems::SpecialPurposeOfficer,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for OfficeRolesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OfficeRolesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Official {
        #[doc = "Addresses at which to contact the official."]
        #[serde(
            rename = "address",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address: ::std::option::Option<Vec<crate::schemas::SimpleAddressType>>,
        #[doc = "A list of known (social) media channels for this official."]
        #[serde(
            rename = "channels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub channels: ::std::option::Option<Vec<crate::schemas::Channel>>,
        #[doc = "The direct email addresses for the official."]
        #[serde(
            rename = "emails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub emails: ::std::option::Option<Vec<String>>,
        #[doc = "Detailed summary about the official’s address’s geocoding"]
        #[serde(
            rename = "geocodingSummaries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub geocoding_summaries: ::std::option::Option<Vec<crate::schemas::GeocodingSummary>>,
        #[doc = "The official’s name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The full name of the party the official belongs to."]
        #[serde(
            rename = "party",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub party: ::std::option::Option<String>,
        #[doc = "The official’s public contact phone numbers."]
        #[serde(
            rename = "phones",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phones: ::std::option::Option<Vec<String>>,
        #[doc = "A URL for a photo of the official."]
        #[serde(
            rename = "photoUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub photo_url: ::std::option::Option<String>,
        #[doc = "The official’s public website URLs."]
        #[serde(
            rename = "urls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub urls: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Official {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Official {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PollingLocation {
        #[doc = "The address of the location."]
        #[serde(
            rename = "address",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub address: ::std::option::Option<crate::schemas::SimpleAddressType>,
        #[doc = "The last date that this early vote site or drop off location may be used. This field is not populated for polling locations."]
        #[serde(
            rename = "endDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_date: ::std::option::Option<String>,
        #[doc = "Latitude of the location, in degrees north of the equator. Note this field may not be available for some locations."]
        #[serde(
            rename = "latitude",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub latitude: ::std::option::Option<f64>,
        #[doc = "Longitude of the location, in degrees east of the Prime Meridian. Note this field may not be available for some locations."]
        #[serde(
            rename = "longitude",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub longitude: ::std::option::Option<f64>,
        #[doc = "The name of the early vote site or drop off location. This field is not populated for polling locations."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Notes about this location (e.g. accessibility ramp or entrance to use)."]
        #[serde(
            rename = "notes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub notes: ::std::option::Option<String>,
        #[doc = "A description of when this location is open."]
        #[serde(
            rename = "pollingHours",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub polling_hours: ::std::option::Option<String>,
        #[doc = "A list of sources for this location. If multiple sources are listed the data has been aggregated from those sources."]
        #[serde(
            rename = "sources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sources: ::std::option::Option<Vec<crate::schemas::Source>>,
        #[doc = "The first date that this early vote site or drop off location may be used. This field is not populated for polling locations."]
        #[serde(
            rename = "startDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_date: ::std::option::Option<String>,
        #[doc = "The services provided by this early vote site or drop off location. This field is not populated for polling locations."]
        #[serde(
            rename = "voterServices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub voter_services: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PollingLocation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PollingLocation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Precinct {
        #[doc = "ID of the AdministrationRegion message for this precinct. Corresponds to LocalityId xml tag."]
        #[serde(
            rename = "administrationRegionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub administration_region_id: ::std::option::Option<String>,
        #[doc = "ID(s) of the Contest message(s) for this precinct."]
        #[serde(
            rename = "contestId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contest_id: ::std::option::Option<Vec<String>>,
        #[doc = "Required. Dataset ID. What datasets our Precincts come from."]
        #[serde(
            rename = "datasetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub dataset_id: ::std::option::Option<i64>,
        #[doc = "ID(s) of the PollingLocation message(s) for this precinct."]
        #[serde(
            rename = "earlyVoteSiteId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub early_vote_site_id: ::std::option::Option<Vec<String>>,
        #[doc = "ID(s) of the ElectoralDistrict message(s) for this precinct."]
        #[serde(
            rename = "electoralDistrictId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub electoral_district_id: ::std::option::Option<Vec<String>>,
        #[doc = "Required. A unique identifier for this precinct."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Specifies if the precinct runs mail-only elections."]
        #[serde(
            rename = "mailOnly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mail_only: ::std::option::Option<bool>,
        #[doc = "Required. The name of the precinct."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The number of the precinct."]
        #[serde(
            rename = "number",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub number: ::std::option::Option<String>,
        #[doc = "Encouraged. The OCD ID of the precinct"]
        #[serde(
            rename = "ocdId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ocd_id: ::std::option::Option<Vec<String>>,
        #[doc = "ID(s) of the PollingLocation message(s) for this precinct."]
        #[serde(
            rename = "pollingLocationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub polling_location_id: ::std::option::Option<Vec<String>>,
        #[doc = "ID(s) of the SpatialBoundary message(s) for this precinct. Used to specify a geometrical boundary of the precinct."]
        #[serde(
            rename = "spatialBoundaryId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spatial_boundary_id: ::std::option::Option<Vec<String>>,
        #[doc = "If present, this proto corresponds to one portion of split precinct. Other portions of this precinct are guaranteed to have the same `name`. If not present, this proto represents a full precicnt."]
        #[serde(
            rename = "splitName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub split_name: ::std::option::Option<String>,
        #[doc = "Specifies the ward the precinct is contained within."]
        #[serde(
            rename = "ward",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ward: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Precinct {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Precinct {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RepresentativeInfoData {
        #[doc = "A map of political geographic divisions that contain the requested address, keyed by the unique Open Civic Data identifier for this division."]
        #[serde(
            rename = "divisions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub divisions: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::GeographicDivision>,
        >,
        #[doc = "Elected offices referenced by the divisions listed above. Will only be present if includeOffices was true in the request."]
        #[serde(
            rename = "offices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offices: ::std::option::Option<Vec<crate::schemas::Office>>,
        #[doc = "Officials holding the offices listed above. Will only be present if includeOffices was true in the request."]
        #[serde(
            rename = "officials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub officials: ::std::option::Option<Vec<crate::schemas::Official>>,
    }
    impl ::google_field_selector::FieldSelector for RepresentativeInfoData {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RepresentativeInfoData {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RepresentativeInfoResponse {
        #[doc = "A map of political geographic divisions that contain the requested address, keyed by the unique Open Civic Data identifier for this division."]
        #[serde(
            rename = "divisions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub divisions: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::GeographicDivision>,
        >,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string “civicinfo#representativeInfoResponse”."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The normalized version of the requested address"]
        #[serde(
            rename = "normalizedInput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_input: ::std::option::Option<crate::schemas::SimpleAddressType>,
        #[doc = "Elected offices referenced by the divisions listed above. Will only be present if includeOffices was true in the request."]
        #[serde(
            rename = "offices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offices: ::std::option::Option<Vec<crate::schemas::Office>>,
        #[doc = "Officials holding the offices listed above. Will only be present if includeOffices was true in the request."]
        #[serde(
            rename = "officials",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub officials: ::std::option::Option<Vec<crate::schemas::Official>>,
    }
    impl ::google_field_selector::FieldSelector for RepresentativeInfoResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RepresentativeInfoResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SimpleAddressType {
        #[doc = "The city or town for the address."]
        #[serde(
            rename = "city",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub city: ::std::option::Option<String>,
        #[doc = "The street name and number of this address."]
        #[serde(
            rename = "line1",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_1: ::std::option::Option<String>,
        #[doc = "The second line the address, if needed."]
        #[serde(
            rename = "line2",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_2: ::std::option::Option<String>,
        #[doc = "The third line of the address, if needed."]
        #[serde(
            rename = "line3",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub line_3: ::std::option::Option<String>,
        #[doc = "The name of the location."]
        #[serde(
            rename = "locationName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub location_name: ::std::option::Option<String>,
        #[doc = "The US two letter state abbreviation of the address."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<String>,
        #[doc = "The US Postal Zip Code of the address."]
        #[serde(
            rename = "zip",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub zip: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SimpleAddressType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SimpleAddressType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
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
        #[doc = "The name of the data source."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Whether this data comes from an official government source."]
        #[serde(
            rename = "official",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub official: ::std::option::Option<bool>,
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
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VoterInfoResponse {
        #[doc = "Contests that will appear on the voter’s ballot."]
        #[serde(
            rename = "contests",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contests: ::std::option::Option<Vec<crate::schemas::Contest>>,
        #[doc = "Locations where a voter is eligible to drop off a completed ballot. The voter must have received and completed a ballot prior to arriving at the location. The location may not have ballots available on the premises. These locations could be open on or before election day as indicated in the pollingHours field."]
        #[serde(
            rename = "dropOffLocations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drop_off_locations: ::std::option::Option<Vec<crate::schemas::PollingLocation>>,
        #[doc = "Locations where the voter is eligible to vote early, prior to election day."]
        #[serde(
            rename = "earlyVoteSites",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub early_vote_sites: ::std::option::Option<Vec<crate::schemas::PollingLocation>>,
        #[doc = "The election that was queried."]
        #[serde(
            rename = "election",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub election: ::std::option::Option<crate::schemas::Election>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string “civicinfo#voterInfoResponse”."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Specifies whether voters in the precinct vote only by mailing their ballots (with the possible option of dropping off their ballots as well)."]
        #[serde(
            rename = "mailOnly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mail_only: ::std::option::Option<bool>,
        #[doc = "The normalized version of the requested address"]
        #[serde(
            rename = "normalizedInput",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub normalized_input: ::std::option::Option<crate::schemas::SimpleAddressType>,
        #[doc = "When there are multiple elections for a voter address, the otherElections field is populated in the API response and there are two possibilities: 1. If the earliest election is not the intended election, specify the election ID of the desired election in a second API request using the electionId field. 2. If these elections occur on the same day, the API doesn?t return any polling location, contest, or election official information to ensure that an additional query is made. For user-facing applications, we recommend displaying these elections to the user to disambiguate. A second API request using the electionId field should be made for the election that is relevant to the user."]
        #[serde(
            rename = "otherElections",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub other_elections: ::std::option::Option<Vec<crate::schemas::Election>>,
        #[doc = "Locations where the voter is eligible to vote on election day."]
        #[serde(
            rename = "pollingLocations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub polling_locations: ::std::option::Option<Vec<crate::schemas::PollingLocation>>,
        #[serde(
            rename = "precinctId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub precinct_id: ::std::option::Option<String>,
        #[doc = "The precincts that match this voter’s address. Will only be returned for project IDs which have been whitelisted as “partner projects”."]
        #[serde(
            rename = "precincts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub precincts: ::std::option::Option<Vec<crate::schemas::Precinct>>,
        #[doc = "Local Election Information for the state that the voter votes in. For the US, there will only be one element in this array."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<Vec<crate::schemas::AdministrationRegion>>,
    }
    impl ::google_field_selector::FieldSelector for VoterInfoResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VoterInfoResponse {
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
    #[doc = "Actions that can be performed on the divisions resource"]
    pub fn divisions(&self) -> crate::resources::divisions::DivisionsActions {
        crate::resources::divisions::DivisionsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the elections resource"]
    pub fn elections(&self) -> crate::resources::elections::ElectionsActions {
        crate::resources::elections::ElectionsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the representatives resource"]
    pub fn representatives(&self) -> crate::resources::representatives::RepresentativesActions {
        crate::resources::representatives::RepresentativesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod divisions {
        pub mod params {}
        pub struct DivisionsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> DivisionsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Searches for political divisions by their natural name or OCD ID."]
            pub fn search(&self) -> SearchRequestBuilder {
                SearchRequestBuilder {
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
                    query: None,
                }
            }
        }
        #[doc = "Created via [DivisionsActions::search()](struct.DivisionsActions.html#method.search)"]
        #[derive(Debug, Clone)]
        pub struct SearchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            query: ::std::option::Option<String>,
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
            #[doc = "The search query. Queries can cover any parts of a OCD ID or a human readable division name. All words given in the query are treated as required patterns. In addition to that, most query operators of the Apache Lucene library are supported. See http://lucene.apache.org/core/2_9_4/queryparsersyntax.html"]
            pub fn query(mut self, value: impl Into<String>) -> Self {
                self.query = Some(value.into());
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
            ) -> Result<crate::schemas::DivisionSearchResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::DivisionSearchResponse, crate::Error> {
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
                let mut output = "https://civicinfo.googleapis.com/".to_owned();
                output.push_str("civicinfo/v2/divisions");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("query", &self.query)]);
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
    pub mod elections {
        pub mod params {}
        pub struct ElectionsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ElectionsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "List of available elections to query."]
            pub fn election_query(&self) -> ElectionQueryRequestBuilder {
                ElectionQueryRequestBuilder {
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
                }
            }
            #[doc = "Looks up information relevant to a voter based on the voter’s registered address."]
            pub fn voter_info_query(
                &self,
                address: impl Into<String>,
            ) -> VoterInfoQueryRequestBuilder {
                VoterInfoQueryRequestBuilder {
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
                    address: address.into(),
                    election_id: None,
                    official_only: None,
                    return_all_available_data: None,
                }
            }
        }
        #[doc = "Created via [ElectionsActions::election_query()](struct.ElectionsActions.html#method.election_query)"]
        #[derive(Debug, Clone)]
        pub struct ElectionQueryRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> ElectionQueryRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::ElectionsQueryResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ElectionsQueryResponse, crate::Error> {
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
                let mut output = "https://civicinfo.googleapis.com/".to_owned();
                output.push_str("civicinfo/v2/elections");
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
        #[doc = "Created via [ElectionsActions::voter_info_query()](struct.ElectionsActions.html#method.voter_info_query)"]
        #[derive(Debug, Clone)]
        pub struct VoterInfoQueryRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            address: String,
            election_id: ::std::option::Option<i64>,
            official_only: ::std::option::Option<bool>,
            return_all_available_data: ::std::option::Option<bool>,
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
        impl<'a> VoterInfoQueryRequestBuilder<'a> {
            #[doc = "The unique ID of the election to look up. A list of election IDs can be obtained at https://www.googleapis.com/civicinfo/{version}/elections. If no election ID is specified in the query and there is more than one election with data for the given voter, the additional elections are provided in the otherElections response field."]
            pub fn election_id(mut self, value: i64) -> Self {
                self.election_id = Some(value);
                self
            }
            #[doc = "If set to true, only data from official state sources will be returned."]
            pub fn official_only(mut self, value: bool) -> Self {
                self.official_only = Some(value);
                self
            }
            #[doc = "If set to true, the query will return the success code and include any partial information when it is unable to determine a matching address or unable to determine the election for electionId=0 queries."]
            pub fn return_all_available_data(mut self, value: bool) -> Self {
                self.return_all_available_data = Some(value);
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
            ) -> Result<crate::schemas::VoterInfoResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::VoterInfoResponse, crate::Error> {
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
                let mut output = "https://civicinfo.googleapis.com/".to_owned();
                output.push_str("civicinfo/v2/voterinfo");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("address", &self.address)]);
                req = req.query(&[("electionId", &self.election_id)]);
                req = req.query(&[("officialOnly", &self.official_only)]);
                req = req.query(&[("returnAllAvailableData", &self.return_all_available_data)]);
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
    pub mod representatives {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum RepresentativeInfoByAddressLevelsItems {
                AdministrativeArea1,
                AdministrativeArea2,
                Country,
                International,
                Locality,
                Regional,
                Special,
                SubLocality1,
                SubLocality2,
            }
            impl RepresentativeInfoByAddressLevelsItems {
                pub fn as_str(self) -> &'static str {
                    match self {
                        RepresentativeInfoByAddressLevelsItems::AdministrativeArea1 => {
                            "administrativeArea1"
                        }
                        RepresentativeInfoByAddressLevelsItems::AdministrativeArea2 => {
                            "administrativeArea2"
                        }
                        RepresentativeInfoByAddressLevelsItems::Country => "country",
                        RepresentativeInfoByAddressLevelsItems::International => "international",
                        RepresentativeInfoByAddressLevelsItems::Locality => "locality",
                        RepresentativeInfoByAddressLevelsItems::Regional => "regional",
                        RepresentativeInfoByAddressLevelsItems::Special => "special",
                        RepresentativeInfoByAddressLevelsItems::SubLocality1 => "subLocality1",
                        RepresentativeInfoByAddressLevelsItems::SubLocality2 => "subLocality2",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for RepresentativeInfoByAddressLevelsItems {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for RepresentativeInfoByAddressLevelsItems {
                type Err = ();
                fn from_str(
                    s: &str,
                ) -> ::std::result::Result<RepresentativeInfoByAddressLevelsItems, ()>
                {
                    Ok(match s {
                        "administrativeArea1" => {
                            RepresentativeInfoByAddressLevelsItems::AdministrativeArea1
                        }
                        "administrativeArea2" => {
                            RepresentativeInfoByAddressLevelsItems::AdministrativeArea2
                        }
                        "country" => RepresentativeInfoByAddressLevelsItems::Country,
                        "international" => RepresentativeInfoByAddressLevelsItems::International,
                        "locality" => RepresentativeInfoByAddressLevelsItems::Locality,
                        "regional" => RepresentativeInfoByAddressLevelsItems::Regional,
                        "special" => RepresentativeInfoByAddressLevelsItems::Special,
                        "subLocality1" => RepresentativeInfoByAddressLevelsItems::SubLocality1,
                        "subLocality2" => RepresentativeInfoByAddressLevelsItems::SubLocality2,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for RepresentativeInfoByAddressLevelsItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for RepresentativeInfoByAddressLevelsItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for RepresentativeInfoByAddressLevelsItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "administrativeArea1" => {
                            RepresentativeInfoByAddressLevelsItems::AdministrativeArea1
                        }
                        "administrativeArea2" => {
                            RepresentativeInfoByAddressLevelsItems::AdministrativeArea2
                        }
                        "country" => RepresentativeInfoByAddressLevelsItems::Country,
                        "international" => RepresentativeInfoByAddressLevelsItems::International,
                        "locality" => RepresentativeInfoByAddressLevelsItems::Locality,
                        "regional" => RepresentativeInfoByAddressLevelsItems::Regional,
                        "special" => RepresentativeInfoByAddressLevelsItems::Special,
                        "subLocality1" => RepresentativeInfoByAddressLevelsItems::SubLocality1,
                        "subLocality2" => RepresentativeInfoByAddressLevelsItems::SubLocality2,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for RepresentativeInfoByAddressLevelsItems {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for RepresentativeInfoByAddressLevelsItems {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum RepresentativeInfoByAddressRolesItems {
                DeputyHeadOfGovernment,
                ExecutiveCouncil,
                GovernmentOfficer,
                HeadOfGovernment,
                HeadOfState,
                HighestCourtJudge,
                Judge,
                LegislatorLowerBody,
                LegislatorUpperBody,
                OtherRole,
                SchoolBoard,
                SpecialPurposeOfficer,
            }
            impl RepresentativeInfoByAddressRolesItems {
                pub fn as_str(self) -> &'static str {
                    match self {
                        RepresentativeInfoByAddressRolesItems::DeputyHeadOfGovernment => {
                            "deputyHeadOfGovernment"
                        }
                        RepresentativeInfoByAddressRolesItems::ExecutiveCouncil => {
                            "executiveCouncil"
                        }
                        RepresentativeInfoByAddressRolesItems::GovernmentOfficer => {
                            "governmentOfficer"
                        }
                        RepresentativeInfoByAddressRolesItems::HeadOfGovernment => {
                            "headOfGovernment"
                        }
                        RepresentativeInfoByAddressRolesItems::HeadOfState => "headOfState",
                        RepresentativeInfoByAddressRolesItems::HighestCourtJudge => {
                            "highestCourtJudge"
                        }
                        RepresentativeInfoByAddressRolesItems::Judge => "judge",
                        RepresentativeInfoByAddressRolesItems::LegislatorLowerBody => {
                            "legislatorLowerBody"
                        }
                        RepresentativeInfoByAddressRolesItems::LegislatorUpperBody => {
                            "legislatorUpperBody"
                        }
                        RepresentativeInfoByAddressRolesItems::OtherRole => "otherRole",
                        RepresentativeInfoByAddressRolesItems::SchoolBoard => "schoolBoard",
                        RepresentativeInfoByAddressRolesItems::SpecialPurposeOfficer => {
                            "specialPurposeOfficer"
                        }
                    }
                }
            }
            impl ::std::convert::AsRef<str> for RepresentativeInfoByAddressRolesItems {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for RepresentativeInfoByAddressRolesItems {
                type Err = ();
                fn from_str(
                    s: &str,
                ) -> ::std::result::Result<RepresentativeInfoByAddressRolesItems, ()>
                {
                    Ok(match s {
                        "deputyHeadOfGovernment" => {
                            RepresentativeInfoByAddressRolesItems::DeputyHeadOfGovernment
                        }
                        "executiveCouncil" => {
                            RepresentativeInfoByAddressRolesItems::ExecutiveCouncil
                        }
                        "governmentOfficer" => {
                            RepresentativeInfoByAddressRolesItems::GovernmentOfficer
                        }
                        "headOfGovernment" => {
                            RepresentativeInfoByAddressRolesItems::HeadOfGovernment
                        }
                        "headOfState" => RepresentativeInfoByAddressRolesItems::HeadOfState,
                        "highestCourtJudge" => {
                            RepresentativeInfoByAddressRolesItems::HighestCourtJudge
                        }
                        "judge" => RepresentativeInfoByAddressRolesItems::Judge,
                        "legislatorLowerBody" => {
                            RepresentativeInfoByAddressRolesItems::LegislatorLowerBody
                        }
                        "legislatorUpperBody" => {
                            RepresentativeInfoByAddressRolesItems::LegislatorUpperBody
                        }
                        "otherRole" => RepresentativeInfoByAddressRolesItems::OtherRole,
                        "schoolBoard" => RepresentativeInfoByAddressRolesItems::SchoolBoard,
                        "specialPurposeOfficer" => {
                            RepresentativeInfoByAddressRolesItems::SpecialPurposeOfficer
                        }
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for RepresentativeInfoByAddressRolesItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for RepresentativeInfoByAddressRolesItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for RepresentativeInfoByAddressRolesItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "deputyHeadOfGovernment" => {
                            RepresentativeInfoByAddressRolesItems::DeputyHeadOfGovernment
                        }
                        "executiveCouncil" => {
                            RepresentativeInfoByAddressRolesItems::ExecutiveCouncil
                        }
                        "governmentOfficer" => {
                            RepresentativeInfoByAddressRolesItems::GovernmentOfficer
                        }
                        "headOfGovernment" => {
                            RepresentativeInfoByAddressRolesItems::HeadOfGovernment
                        }
                        "headOfState" => RepresentativeInfoByAddressRolesItems::HeadOfState,
                        "highestCourtJudge" => {
                            RepresentativeInfoByAddressRolesItems::HighestCourtJudge
                        }
                        "judge" => RepresentativeInfoByAddressRolesItems::Judge,
                        "legislatorLowerBody" => {
                            RepresentativeInfoByAddressRolesItems::LegislatorLowerBody
                        }
                        "legislatorUpperBody" => {
                            RepresentativeInfoByAddressRolesItems::LegislatorUpperBody
                        }
                        "otherRole" => RepresentativeInfoByAddressRolesItems::OtherRole,
                        "schoolBoard" => RepresentativeInfoByAddressRolesItems::SchoolBoard,
                        "specialPurposeOfficer" => {
                            RepresentativeInfoByAddressRolesItems::SpecialPurposeOfficer
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
            impl ::google_field_selector::FieldSelector for RepresentativeInfoByAddressRolesItems {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for RepresentativeInfoByAddressRolesItems {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum RepresentativeInfoByDivisionLevelsItems {
                AdministrativeArea1,
                AdministrativeArea2,
                Country,
                International,
                Locality,
                Regional,
                Special,
                SubLocality1,
                SubLocality2,
            }
            impl RepresentativeInfoByDivisionLevelsItems {
                pub fn as_str(self) -> &'static str {
                    match self {
                        RepresentativeInfoByDivisionLevelsItems::AdministrativeArea1 => {
                            "administrativeArea1"
                        }
                        RepresentativeInfoByDivisionLevelsItems::AdministrativeArea2 => {
                            "administrativeArea2"
                        }
                        RepresentativeInfoByDivisionLevelsItems::Country => "country",
                        RepresentativeInfoByDivisionLevelsItems::International => "international",
                        RepresentativeInfoByDivisionLevelsItems::Locality => "locality",
                        RepresentativeInfoByDivisionLevelsItems::Regional => "regional",
                        RepresentativeInfoByDivisionLevelsItems::Special => "special",
                        RepresentativeInfoByDivisionLevelsItems::SubLocality1 => "subLocality1",
                        RepresentativeInfoByDivisionLevelsItems::SubLocality2 => "subLocality2",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for RepresentativeInfoByDivisionLevelsItems {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for RepresentativeInfoByDivisionLevelsItems {
                type Err = ();
                fn from_str(
                    s: &str,
                ) -> ::std::result::Result<RepresentativeInfoByDivisionLevelsItems, ()>
                {
                    Ok(match s {
                        "administrativeArea1" => {
                            RepresentativeInfoByDivisionLevelsItems::AdministrativeArea1
                        }
                        "administrativeArea2" => {
                            RepresentativeInfoByDivisionLevelsItems::AdministrativeArea2
                        }
                        "country" => RepresentativeInfoByDivisionLevelsItems::Country,
                        "international" => RepresentativeInfoByDivisionLevelsItems::International,
                        "locality" => RepresentativeInfoByDivisionLevelsItems::Locality,
                        "regional" => RepresentativeInfoByDivisionLevelsItems::Regional,
                        "special" => RepresentativeInfoByDivisionLevelsItems::Special,
                        "subLocality1" => RepresentativeInfoByDivisionLevelsItems::SubLocality1,
                        "subLocality2" => RepresentativeInfoByDivisionLevelsItems::SubLocality2,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for RepresentativeInfoByDivisionLevelsItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for RepresentativeInfoByDivisionLevelsItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for RepresentativeInfoByDivisionLevelsItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "administrativeArea1" => {
                            RepresentativeInfoByDivisionLevelsItems::AdministrativeArea1
                        }
                        "administrativeArea2" => {
                            RepresentativeInfoByDivisionLevelsItems::AdministrativeArea2
                        }
                        "country" => RepresentativeInfoByDivisionLevelsItems::Country,
                        "international" => RepresentativeInfoByDivisionLevelsItems::International,
                        "locality" => RepresentativeInfoByDivisionLevelsItems::Locality,
                        "regional" => RepresentativeInfoByDivisionLevelsItems::Regional,
                        "special" => RepresentativeInfoByDivisionLevelsItems::Special,
                        "subLocality1" => RepresentativeInfoByDivisionLevelsItems::SubLocality1,
                        "subLocality2" => RepresentativeInfoByDivisionLevelsItems::SubLocality2,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for RepresentativeInfoByDivisionLevelsItems {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for RepresentativeInfoByDivisionLevelsItems {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum RepresentativeInfoByDivisionRolesItems {
                DeputyHeadOfGovernment,
                ExecutiveCouncil,
                GovernmentOfficer,
                HeadOfGovernment,
                HeadOfState,
                HighestCourtJudge,
                Judge,
                LegislatorLowerBody,
                LegislatorUpperBody,
                OtherRole,
                SchoolBoard,
                SpecialPurposeOfficer,
            }
            impl RepresentativeInfoByDivisionRolesItems {
                pub fn as_str(self) -> &'static str {
                    match self {
                        RepresentativeInfoByDivisionRolesItems::DeputyHeadOfGovernment => {
                            "deputyHeadOfGovernment"
                        }
                        RepresentativeInfoByDivisionRolesItems::ExecutiveCouncil => {
                            "executiveCouncil"
                        }
                        RepresentativeInfoByDivisionRolesItems::GovernmentOfficer => {
                            "governmentOfficer"
                        }
                        RepresentativeInfoByDivisionRolesItems::HeadOfGovernment => {
                            "headOfGovernment"
                        }
                        RepresentativeInfoByDivisionRolesItems::HeadOfState => "headOfState",
                        RepresentativeInfoByDivisionRolesItems::HighestCourtJudge => {
                            "highestCourtJudge"
                        }
                        RepresentativeInfoByDivisionRolesItems::Judge => "judge",
                        RepresentativeInfoByDivisionRolesItems::LegislatorLowerBody => {
                            "legislatorLowerBody"
                        }
                        RepresentativeInfoByDivisionRolesItems::LegislatorUpperBody => {
                            "legislatorUpperBody"
                        }
                        RepresentativeInfoByDivisionRolesItems::OtherRole => "otherRole",
                        RepresentativeInfoByDivisionRolesItems::SchoolBoard => "schoolBoard",
                        RepresentativeInfoByDivisionRolesItems::SpecialPurposeOfficer => {
                            "specialPurposeOfficer"
                        }
                    }
                }
            }
            impl ::std::convert::AsRef<str> for RepresentativeInfoByDivisionRolesItems {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for RepresentativeInfoByDivisionRolesItems {
                type Err = ();
                fn from_str(
                    s: &str,
                ) -> ::std::result::Result<RepresentativeInfoByDivisionRolesItems, ()>
                {
                    Ok(match s {
                        "deputyHeadOfGovernment" => {
                            RepresentativeInfoByDivisionRolesItems::DeputyHeadOfGovernment
                        }
                        "executiveCouncil" => {
                            RepresentativeInfoByDivisionRolesItems::ExecutiveCouncil
                        }
                        "governmentOfficer" => {
                            RepresentativeInfoByDivisionRolesItems::GovernmentOfficer
                        }
                        "headOfGovernment" => {
                            RepresentativeInfoByDivisionRolesItems::HeadOfGovernment
                        }
                        "headOfState" => RepresentativeInfoByDivisionRolesItems::HeadOfState,
                        "highestCourtJudge" => {
                            RepresentativeInfoByDivisionRolesItems::HighestCourtJudge
                        }
                        "judge" => RepresentativeInfoByDivisionRolesItems::Judge,
                        "legislatorLowerBody" => {
                            RepresentativeInfoByDivisionRolesItems::LegislatorLowerBody
                        }
                        "legislatorUpperBody" => {
                            RepresentativeInfoByDivisionRolesItems::LegislatorUpperBody
                        }
                        "otherRole" => RepresentativeInfoByDivisionRolesItems::OtherRole,
                        "schoolBoard" => RepresentativeInfoByDivisionRolesItems::SchoolBoard,
                        "specialPurposeOfficer" => {
                            RepresentativeInfoByDivisionRolesItems::SpecialPurposeOfficer
                        }
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for RepresentativeInfoByDivisionRolesItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for RepresentativeInfoByDivisionRolesItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for RepresentativeInfoByDivisionRolesItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "deputyHeadOfGovernment" => {
                            RepresentativeInfoByDivisionRolesItems::DeputyHeadOfGovernment
                        }
                        "executiveCouncil" => {
                            RepresentativeInfoByDivisionRolesItems::ExecutiveCouncil
                        }
                        "governmentOfficer" => {
                            RepresentativeInfoByDivisionRolesItems::GovernmentOfficer
                        }
                        "headOfGovernment" => {
                            RepresentativeInfoByDivisionRolesItems::HeadOfGovernment
                        }
                        "headOfState" => RepresentativeInfoByDivisionRolesItems::HeadOfState,
                        "highestCourtJudge" => {
                            RepresentativeInfoByDivisionRolesItems::HighestCourtJudge
                        }
                        "judge" => RepresentativeInfoByDivisionRolesItems::Judge,
                        "legislatorLowerBody" => {
                            RepresentativeInfoByDivisionRolesItems::LegislatorLowerBody
                        }
                        "legislatorUpperBody" => {
                            RepresentativeInfoByDivisionRolesItems::LegislatorUpperBody
                        }
                        "otherRole" => RepresentativeInfoByDivisionRolesItems::OtherRole,
                        "schoolBoard" => RepresentativeInfoByDivisionRolesItems::SchoolBoard,
                        "specialPurposeOfficer" => {
                            RepresentativeInfoByDivisionRolesItems::SpecialPurposeOfficer
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
            impl ::google_field_selector::FieldSelector for RepresentativeInfoByDivisionRolesItems {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for RepresentativeInfoByDivisionRolesItems {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct RepresentativesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> RepresentativesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Looks up political geography and representative information for a single address."]
            pub fn representative_info_by_address(
                &self,
            ) -> RepresentativeInfoByAddressRequestBuilder {
                RepresentativeInfoByAddressRequestBuilder {
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
                    address: None,
                    include_offices: None,
                    levels: None,
                    roles: None,
                }
            }
            #[doc = "Looks up representative information for a single geographic division."]
            pub fn representative_info_by_division(
                &self,
                ocd_id: impl Into<String>,
            ) -> RepresentativeInfoByDivisionRequestBuilder {
                RepresentativeInfoByDivisionRequestBuilder {
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
                    ocd_id: ocd_id.into(),
                    levels: None,
                    recursive: None,
                    roles: None,
                }
            }
        }
        #[doc = "Created via [RepresentativesActions::representative_info_by_address()](struct.RepresentativesActions.html#method.representative_info_by_address)"]
        #[derive(Debug, Clone)]
        pub struct RepresentativeInfoByAddressRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , address : :: std :: option :: Option < String > , include_offices : :: std :: option :: Option < bool > , levels : :: std :: option :: Option < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByAddressLevelsItems > > , roles : :: std :: option :: Option < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByAddressRolesItems > > , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
        impl<'a> RepresentativeInfoByAddressRequestBuilder<'a> {
            #[doc = "The address to look up. May only be specified if the field ocdId is not given in the URL"]
            pub fn address(mut self, value: impl Into<String>) -> Self {
                self.address = Some(value.into());
                self
            }
            #[doc = "Whether to return information about offices and officials. If false, only the top-level district information will be returned."]
            pub fn include_offices(mut self, value: bool) -> Self {
                self.include_offices = Some(value);
                self
            }
            #[doc = "A list of office levels to filter by. Only offices that serve at least one of these levels will be returned. Divisions that don’t contain a matching office will not be returned."]
            pub fn levels(
                mut self,
                value : impl Into < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByAddressLevelsItems > >,
            ) -> Self {
                self.levels = Some(value.into());
                self
            }
            #[doc = "A list of office roles to filter by. Only offices fulfilling one of these roles will be returned. Divisions that don’t contain a matching office will not be returned."]
            pub fn roles(
                mut self,
                value : impl Into < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByAddressRolesItems > >,
            ) -> Self {
                self.roles = Some(value.into());
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
            ) -> Result<crate::schemas::RepresentativeInfoResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::RepresentativeInfoResponse, crate::Error> {
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
                let mut output = "https://civicinfo.googleapis.com/".to_owned();
                output.push_str("civicinfo/v2/representatives");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("address", &self.address)]);
                req = req.query(&[("includeOffices", &self.include_offices)]);
                for value in self.levels.iter().flatten() {
                    req = req.query(&[("levels", value)]);
                }
                for value in self.roles.iter().flatten() {
                    req = req.query(&[("roles", value)]);
                }
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
        #[doc = "Created via [RepresentativesActions::representative_info_by_division()](struct.RepresentativesActions.html#method.representative_info_by_division)"]
        #[derive(Debug, Clone)]
        pub struct RepresentativeInfoByDivisionRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , ocd_id : String , levels : :: std :: option :: Option < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByDivisionLevelsItems > > , recursive : :: std :: option :: Option < bool > , roles : :: std :: option :: Option < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByDivisionRolesItems > > , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
        impl<'a> RepresentativeInfoByDivisionRequestBuilder<'a> {
            #[doc = "A list of office levels to filter by. Only offices that serve at least one of these levels will be returned. Divisions that don’t contain a matching office will not be returned."]
            pub fn levels(
                mut self,
                value : impl Into < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByDivisionLevelsItems > >,
            ) -> Self {
                self.levels = Some(value.into());
                self
            }
            #[doc = "If true, information about all divisions contained in the division requested will be included as well. For example, if querying ocd-division/country:us/district:dc, this would also return all DC’s wards and ANCs."]
            pub fn recursive(mut self, value: bool) -> Self {
                self.recursive = Some(value);
                self
            }
            #[doc = "A list of office roles to filter by. Only offices fulfilling one of these roles will be returned. Divisions that don’t contain a matching office will not be returned."]
            pub fn roles(
                mut self,
                value : impl Into < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByDivisionRolesItems > >,
            ) -> Self {
                self.roles = Some(value.into());
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
            ) -> Result<crate::schemas::RepresentativeInfoData, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::RepresentativeInfoData, crate::Error> {
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
                let mut output = "https://civicinfo.googleapis.com/".to_owned();
                output.push_str("civicinfo/v2/representatives/");
                {
                    let var_as_str = &self.ocd_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                for value in self.levels.iter().flatten() {
                    req = req.query(&[("levels", value)]);
                }
                req = req.query(&[("recursive", &self.recursive)]);
                for value in self.roles.iter().flatten() {
                    req = req.query(&[("roles", value)]);
                }
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
