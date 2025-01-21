use chrono::{TimeZone, Utc};
use dioxus::prelude::*;
use dioxus_translate::Language;
use models::prelude::{
    Field, MetadataAuthority, MetadataPurpose, MetadataSource, MetadataSummary, MetadataType,
};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    resources: Signal<Vec<MetadataSummary>>,
}

impl Controller {
    pub fn new(_lang: dioxus_translate::Language) -> Self {
        //FIXME: fix to api
        let resources = vec![
            MetadataSummary {
                id: "1".to_string(),
                name: "공론자료제목명".to_string(),
                urls: vec![
                    "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                        .to_string(),
                ],
                metadata_type: Some(MetadataType::Report),
                metadata_field: Some(Field::Economy),
                metadata_purpose: Some(MetadataPurpose::PublicDiscussion),
                metadata_source: Some(MetadataSource::Internal),
                metadata_authority: Some(MetadataAuthority::Public),
                public_opinion_projects: None,
                public_survey_projects: None,
                updated_at: 1759276800,
            },
            MetadataSummary {
                id: "2".to_string(),
                name: "공론자료제목명".to_string(),
                urls: vec![
                    "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                        .to_string(),
                ],
                metadata_type: Some(MetadataType::Statistics),
                metadata_field: Some(Field::Society),
                metadata_purpose: Some(MetadataPurpose::AcademicResearch),
                metadata_source: Some(MetadataSource::External),
                metadata_authority: Some(MetadataAuthority::Restricted),
                public_opinion_projects: None,
                public_survey_projects: None,
                updated_at: 1759276800,
            },
            MetadataSummary {
                id: "3".to_string(),
                name: "공론자료제목명".to_string(),
                urls: vec![
                    "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                        .to_string(),
                ],
                metadata_type: Some(MetadataType::Statistics),
                metadata_field: Some(Field::Environment),
                metadata_purpose: Some(MetadataPurpose::DevelopmentPolicy),
                metadata_source: Some(MetadataSource::Goverment),
                metadata_authority: Some(MetadataAuthority::Private),
                public_opinion_projects: None,
                public_survey_projects: None,
                updated_at: 1759276800,
            },
            MetadataSummary {
                id: "4".to_string(),
                name: "공론자료제목명".to_string(),
                urls: vec![
                    "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                        .to_string(),
                ],
                metadata_type: Some(MetadataType::Thesis),
                metadata_field: Some(Field::Education),
                metadata_purpose: Some(MetadataPurpose::Education),
                metadata_source: Some(MetadataSource::Company),
                metadata_authority: Some(MetadataAuthority::Public),
                public_opinion_projects: None,
                public_survey_projects: None,
                updated_at: 1759276800,
            },
            MetadataSummary {
                id: "5".to_string(),
                name: "공론자료제목명".to_string(),
                urls: vec![
                    "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                        .to_string(),
                ],
                metadata_type: Some(MetadataType::Presentation),
                metadata_field: Some(Field::Technology),
                metadata_purpose: Some(MetadataPurpose::PublicDiscussion),
                metadata_source: Some(MetadataSource::Internal),
                metadata_authority: Some(MetadataAuthority::Public),
                public_opinion_projects: None,
                public_survey_projects: None,
                updated_at: 1759276800,
            },
            MetadataSummary {
                id: "6".to_string(),
                name: "공론자료제목명".to_string(),
                urls: vec![
                    "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                        .to_string(),
                ],
                metadata_type: Some(MetadataType::Media),
                metadata_field: Some(Field::Health),
                metadata_purpose: Some(MetadataPurpose::PublicDiscussion),
                metadata_source: Some(MetadataSource::Internal),
                metadata_authority: Some(MetadataAuthority::Public),
                public_opinion_projects: None,
                public_survey_projects: None,
                updated_at: 1759276800,
            },
        ];
        let ctrl = Self {
            resources: use_signal(|| resources),
        };

        ctrl
    }

    pub fn get_resources(&self) -> Vec<MetadataSummary> {
        (self.resources)()
    }

    pub fn translate_metadata_type(
        &self,
        lang: Language,
        metadata_type: MetadataType,
    ) -> &'static str {
        metadata_type.translate(&lang)
    }

    pub fn translate_metadata_field(&self, lang: Language, metadata_field: Field) -> &'static str {
        metadata_field.translate(&lang)
    }

    pub fn translate_metadata_purpose(
        &self,
        lang: Language,
        metadata_purpose: MetadataPurpose,
    ) -> &'static str {
        metadata_purpose.translate(&lang)
    }

    pub fn translate_metadata_source(
        &self,
        lang: Language,
        metadata_source: MetadataSource,
    ) -> &'static str {
        metadata_source.translate(&lang)
    }

    pub fn translate_metadata_authority(
        &self,
        lang: Language,
        metadata_authority: MetadataAuthority,
    ) -> &'static str {
        metadata_authority.translate(&lang)
    }

    pub fn convert_timestamp_to_date(&self, timestamp: i64) -> String {
        let datetime = Utc.timestamp_opt(timestamp, 0).unwrap();
        let formatted_date = datetime.format("%Y.%m.%d").to_string();
        formatted_date
    }
}
