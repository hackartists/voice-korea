use dioxus::prelude::*;
use dioxus_translate::Language;
use models::prelude::{Field, OpinionInfo, OpinionInformation, PublicOpinionType};

use super::i18n::OpinionNewTranslate;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    current_step: Signal<CurrentStep>,
    public_opinion_sequences: Signal<Vec<OpinionInfo>>,
    total_option_types: Signal<Vec<String>>,

    //step 2
    total_fields: Signal<Vec<String>>,
    opinion_informations: Signal<OpinionInformation>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CurrentStep {
    PublicOpinionComposition, // 공론 구성 및 기간
    InputInformation,         //필수정보 입력
    CommitteeComposition,     //공론 위원회 구성
    PanelComposition,         //참여자 패널 구성
    DiscussionSetting,        //토론 설정
    Preview,                  //전체 미리보기
}

impl Controller {
    pub fn new(_lang: dioxus_translate::Language, translates: OpinionNewTranslate) -> Self {
        let ctrl = Self {
            current_step: use_signal(|| CurrentStep::PublicOpinionComposition),
            total_option_types: use_signal(|| {
                vec![
                    translates.regular_post.to_string(),
                    translates.video_conference.to_string(),
                    translates.post.to_string(),
                    translates.vote.to_string(),
                    translates.report.to_string(),
                ]
            }),
            public_opinion_sequences: use_signal(|| {
                vec![
                    OpinionInfo {
                        name: translates.information_provided.to_string(),
                        start_date: None,
                        end_date: None,
                        public_opinion_type: Some(PublicOpinionType::General),
                    },
                    OpinionInfo {
                        name: translates.discussion_and_deliberation.to_string(),
                        start_date: None,
                        end_date: None,
                        public_opinion_type: Some(PublicOpinionType::Video),
                    },
                    OpinionInfo {
                        name: translates.derive_opinions.to_string(),
                        start_date: None,
                        end_date: None,
                        public_opinion_type: Some(PublicOpinionType::Post),
                    },
                    OpinionInfo {
                        name: translates.reach_consensus.to_string(),
                        start_date: None,
                        end_date: None,
                        public_opinion_type: Some(PublicOpinionType::Vote),
                    },
                    OpinionInfo {
                        name: translates.analysis_result.to_string(),
                        start_date: None,
                        end_date: None,
                        public_opinion_type: Some(PublicOpinionType::Report),
                    },
                ]
            }),

            // step 2
            total_fields: use_signal(|| {
                vec![
                    "경제".to_string(),
                    "사회".to_string(),
                    "환경".to_string(),
                    "교육".to_string(),
                    "문화".to_string(),
                    "노동".to_string(),
                    "도시".to_string(),
                    "기술".to_string(),
                    "보건".to_string(),
                    "정치".to_string(),
                ]
            }),
            opinion_informations: use_signal(|| OpinionInformation {
                opinion_type: None,
                title: None,
                description: None,
                documents: vec![],
            }),
        };
        use_context_provider(|| ctrl);
        ctrl
    }

    pub fn update_opinion_info(&mut self, index: usize, opinion: OpinionInfo) {
        let mut sequences = self.get_public_opinion_sequences();
        sequences[index] = opinion;
        self.public_opinion_sequences.set(sequences);
    }

    pub fn delete_opinion_info(&mut self, index: usize) {
        let mut sequences = self.get_public_opinion_sequences();
        sequences.remove(index);
        self.public_opinion_sequences.set(sequences);
    }

    pub fn add_opinion_info(&mut self) {
        let mut sequences = self.get_public_opinion_sequences();
        sequences.push(OpinionInfo {
            name: "".to_string(),
            start_date: None,
            end_date: None,
            public_opinion_type: None,
        });
        self.public_opinion_sequences.set(sequences);
    }

    pub fn update_opinion_type_from_str(&self, opinion_type: String) -> Option<PublicOpinionType> {
        if opinion_type == "일반 게시글" {
            Some(PublicOpinionType::General)
        } else if opinion_type == "화상 회의" {
            Some(PublicOpinionType::Video)
        } else if opinion_type == "포스트형 게시글" {
            Some(PublicOpinionType::Post)
        } else if opinion_type == "투표" {
            Some(PublicOpinionType::Vote)
        } else if opinion_type == "보고서" {
            Some(PublicOpinionType::Report)
        } else {
            None
        }
    }

    pub fn project_opinion_type(
        &self,
        lang: Language,
        opinion_type: PublicOpinionType,
    ) -> &'static str {
        match lang {
            Language::En => match opinion_type {
                PublicOpinionType::General => "General",
                PublicOpinionType::Video => "Video",
                PublicOpinionType::Post => "Post",
                PublicOpinionType::Vote => "Vote",
                PublicOpinionType::Report => "Report",
            },
            Language::Ko => match opinion_type {
                PublicOpinionType::General => "일반 게시글",
                PublicOpinionType::Video => "화상 회의",
                PublicOpinionType::Post => "포스트형 게시글",
                PublicOpinionType::Vote => "투표",
                PublicOpinionType::Report => "보고서",
            },
        }
    }

    pub fn change_step(&mut self, step: CurrentStep) {
        self.current_step.set(step);
    }

    pub fn get_total_option_types(&self) -> Vec<String> {
        (self.total_option_types)()
    }

    pub fn get_public_opinion_sequences(&self) -> Vec<OpinionInfo> {
        (self.public_opinion_sequences)()
    }

    pub fn get_current_step(&self) -> CurrentStep {
        (self.current_step)()
    }

    pub fn use_service() -> Self {
        use_context()
    }

    // step 2
    pub fn get_total_fields(&self) -> Vec<String> {
        (self.total_fields)()
    }

    pub fn get_opinion_informations(&self) -> OpinionInformation {
        (self.opinion_informations)()
    }

    pub fn opinion_field_type_translate(
        &self,
        lang: Language,
        opinion_type: Field,
    ) -> &'static str {
        opinion_type.translate(&lang)
    }

    pub fn update_opinion_field_type_from_str(&self, opinion_field_type: String) -> Option<Field> {
        let field = opinion_field_type.parse::<Field>();

        match field {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub fn update_opinion_information(&mut self, information: OpinionInformation) {
        self.opinion_informations.set(information);
    }
}
