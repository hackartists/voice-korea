#![allow(unused_variables)]
#[allow(unused)]
use crate::Result;
use crate::{
    attribute_v2::{AgeV2, GenderV2, RegionV2, SalaryV2},
    response::AgeV3,
};
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[derive(validator::Validate)]
#[api_model(base = "/organizations/v2/:org-id/panels", table = panels, iter_type=QueryResponse)]
pub struct PanelV2 {
    #[api_model(summary, primary_key, action = delete, read_action = [get_panel, find_by_id])]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = [create], action_by_id = update, query_action = search_by)]
    pub name: String,
    //TODO: remove this field with removal of ui dependency
    #[api_model(summary, action = [create], action_by_id = update)]
    pub user_count: u64,

    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    pub age: AgeV2,
    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    pub gender: GenderV2,
    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    pub region: RegionV2,
    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    pub salary: SalaryV2,

    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,
}

impl PartialEq<Vec<crate::survey::response::Attribute>> for PanelV2 {
    fn eq(&self, other: &Vec<crate::survey::response::Attribute>) -> bool {
        use crate::survey::response::Attribute;

        let mut cover_age = self.age == AgeV2::None;
        let mut cover_gender = self.gender == GenderV2::None;
        let mut cover_region = self.region == RegionV2::None;
        let mut cover_salary = self.salary == SalaryV2::None;

        for attr in other {
            match attr {
                Attribute::Age(AgeV3::Specific(age)) => {
                    let (min, max) = self.age.to_range();
                    if *age >= min && *age <= max {
                        cover_age = true;
                    }
                }
                Attribute::Age(AgeV3::Range {
                    inclusive_min,
                    inclusive_max,
                }) => {
                    let (min, max) = self.age.to_range();
                    if *inclusive_min >= min && *inclusive_max <= max {
                        cover_age = true;
                    }
                }
                Attribute::Age(AgeV3::None) => {}
                Attribute::Gender(gender) => {
                    if *gender == self.gender {
                        cover_gender = true;
                    }
                }
                Attribute::Region(region) => {
                    if *region == self.region {
                        cover_region = true;
                    }
                }
                Attribute::Salary(salary) => {
                    if *salary == self.salary {
                        cover_salary = true;
                    }
                }

                Attribute::None => {}
            }
        }

        cover_age && cover_gender && cover_region && cover_salary
    }
}
