use http::Query;

// =============================================================================
// Collection Params
// =============================================================================

/// Parameters for collections queries.
pub struct CollectionParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl CollectionParams {
    pub fn new(limit: Option<u32>, offset: Option<u32>) -> Self {
        Self { limit, offset }
    }

    pub fn add_to_query(&self, query: &mut Query) {
        query.insert("limit".to_string(), self.limit.unwrap_or(25).to_string());
        query.insert("offset".to_string(), self.offset.unwrap_or(0).to_string());
    }

    pub fn to_query(&self) -> Query {
        let mut query = Query::new();
        self.add_to_query(&mut query);
        query
    }
}

impl Default for CollectionParams {
    fn default() -> Self {
        Self {
            limit: Some(25),
            offset: Some(0),
        }
    }
}