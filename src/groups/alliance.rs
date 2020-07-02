use crate::{Esi, EsiResult, RequestType};
use serde::Deserialize;

pub struct AllianceGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
pub struct AllianceInfo {
    pub creator_corporation_id: u64,
    pub creator_id: u64,
    pub date_founded: String,
    pub executor_corporation_id: u64,
    pub name: String,
    pub ticker: String,
}

#[derive(Debug, Deserialize)]
pub struct AllianceIcons {
    pub px128x128: String,
    pub px64x64: String,
}

impl<'a> AllianceGroup<'a> {
    /// Get a list of alliance IDs.
    pub async fn list_ids(&self) -> EsiResult<Vec<u64>> {
        let path = self.esi.get_endpoint_for_op_id("get_alliances")?;
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }

    /// Get public information about an alliance
    pub async fn get_info(&self, alliance_id: u64) -> EsiResult<AllianceInfo> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_alliances_alliance_id")?
            .replace("{alliance_id}", &alliance_id.to_string());
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }

    /// Get list of corporation IDs in an alliance
    pub async fn get_alliance_corporations(&self, alliance_id: u64) -> EsiResult<Vec<u64>> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_alliances_alliance_id_corporations")?
            .replace("{alliance_id}", &alliance_id.to_string());
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }

    /// Get paths to the alliance's icons on the image server.
    pub async fn get_alliance_icons(&self, alliance_id: u64) -> EsiResult<AllianceIcons> {
        let path = self
            .esi
            .get_endpoint_for_op_id("get_alliances_alliance_id_icons")?
            .replace("{alliance_id}", &alliance_id.to_string());
        self.esi
            .query("GET", RequestType::Public, &path, None, None)
            .await
    }
}
