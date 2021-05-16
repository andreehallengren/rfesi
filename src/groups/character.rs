use crate::prelude::*;

/// Endpoints for Character
pub struct CharacterGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CharacterPublicInfo {
    pub alliance_id: i32,
    pub ancestry_id: u16,
    pub birthday: String,
    pub corporation_id: i32,
    pub description: String,
    pub gender: String,
    pub name: String,
    pub race_id: u16,
    pub security_status: f64,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CorporationHistoryItem {
    pub corporation_id: i32,
    pub record_id: i32,
    pub start_date: String,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CharacterPortraitInfo {
    pub px128x128: String,
    pub px256x256: String,
    pub px512x512: String,
    pub px64x64: String,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CharacterAffiliation {
    pub alliance_id: i32,
    pub character: i32,
    pub corporation: i32,
}

impl<'a> CharacterGroup<'a> {
    api_get!(
        /// Get a character's public information.
        get_public_info,
        "get_characters_character_id",
        RequestType::Public,
        CharacterPublicInfo,
        (character_id: i32) => "{character_id}"
    );

    api_get!(
        /// Get a character's corporation history.
        get_history,
        "get_characters_character_id_corporationhistory",
        RequestType::Public,
        Vec<CorporationHistoryItem>,
        (character_id: i32) => "{character_id}"
    );

    api_get!(
        /// Get a character's portrait URLs on the image server.
        get_portrait,
        "get_characters_character_id_portrait",
        RequestType::Public,
        CharacterPortraitInfo,
        (character_id: i32) => "{character_id}"
    );

    api_post!(
        /// Get character affiliations.
        get_affiliation,
        "post_characters_affiliation",
        RequestType::Public,
        Vec<CharacterAffiliation>,
        ,
        character_ids: &[i32],
    );

    // more endpoints ...
}
