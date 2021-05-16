#![allow(unused)]

use crate::prelude::*;

/// Endpoints for Search
pub struct SearchGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Search {
    pub agent: Option<Vec<i32>>,
    pub alliance: Option<Vec<i32>>,
    pub constellation: Option<Vec<i32>>,
    pub character: Option<Vec<i32>>,
    pub corporation: Option<Vec<i32>>,
    pub faction: Option<Vec<i32>>,
    pub inventory_type: Option<Vec<i32>>,
    pub region: Option<Vec<i32>>,
    pub solar_system: Option<Vec<i32>>,
    pub station: Option<Vec<i32>>,
}

impl<'a> SearchGroup<'a> {
    api_get!(
        /// Get a list of constellation ids
        get_search,
        "get_search",
        RequestType::Public,
        Search,
        "strict", "true",
        ;
        (categories: String) => "categories",
        (search: String) => "search"
    );
}
