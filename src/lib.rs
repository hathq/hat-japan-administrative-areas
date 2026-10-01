#![forbid(unsafe_code)]

mod areas;

pub use areas::ADMINISTRATIVE_AREAS;

pub const PACKAGE_JSON: &str = include_str!("../hat.package.json");
pub const PACKAGE_ID: &str = "hat/japan-administrative-areas";
pub const REPOSITORY_ID: &str = "hat-japan-administrative-areas";
pub const COUNTRY_CODE: &str = "JP";
pub const OPERATION_ID: &str = "hathq://vocabulary/action/list-administrative-areas/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdministrativeArea {
    pub code: &'static str,
    pub name_ja: &'static str,
    pub name_en: &'static str,
}

#[must_use]
pub fn list_for_country(country_code: &str) -> Option<&'static [AdministrativeArea]> {
    (country_code == COUNTRY_CODE).then_some(ADMINISTRATIVE_AREAS)
}

/// Selects one released subdivision using exact country and area codes.
///
/// # Errors
///
/// Returns a stable unresolved reason for an unsupported country or unknown
/// subdivision. This function never normalizes or guesses user input.
pub fn select_exact(
    country_code: &str,
    area_code: &str,
) -> Result<&'static AdministrativeArea, &'static str> {
    let areas = list_for_country(country_code).ok_or("residence-country-unsupported")?;
    areas
        .iter()
        .find(|area| area.code == area_code)
        .ok_or("administrative-area-unknown")
}
