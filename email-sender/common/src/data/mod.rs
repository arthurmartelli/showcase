use google_sheets4::oauth2::{self, authenticator::Authenticator};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use std::collections::{HashMap, HashSet};

pub type Data = String;
pub type DataMap = HashMap<Data, Data>;
pub type DataSet<Data> = HashSet<Data>;
pub type AccessToken = oauth2::AccessToken;
pub type Auth = Authenticator<HttpsConnector<HttpConnector>>;

/// Copies entries from map_b into map_a (overwrites existing)
pub fn merge_data_maps(mut map_a: DataMap, map_b: &DataMap) -> DataMap {
    map_a.extend(map_b.iter().map(|(k, v)| (k.clone(), v.clone())));
    map_a
}

// sub-groups vector with smaller vectors of a given size
pub fn subgroup_vector<T: Clone>(vec: Vec<T>, section_size: usize) -> Vec<Vec<T>> {
    vec.chunks(section_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}
