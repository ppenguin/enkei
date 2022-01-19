use std::collections::HashMap;

use log::debug;

use crate::image::{
    error::ImageError,
    image::Image,
    scaling::{Filter, Scaling},
};

pub struct ResourceLoader {
    loaded: HashMap<String, Image>,
}

impl ResourceLoader {
    pub fn new() -> Self {
        Self {
            loaded: HashMap::new(),
        }
    }

    pub fn load(
        &mut self,
        path: &str,
        scaling: Scaling,
        filter: Filter,
    ) -> Result<&Image, ImageError> {
        // workaround as this introduces nastier non-lexical lifetimes
        if self.loaded.contains_key(path) {
            // The scaling and filter cannot differ
            debug!("Fetching image from cache {{ path: {} }}", path);
            return Ok(self.loaded.get(path).unwrap());
        }

        let surface = Image::new(path, scaling, filter)?;
        self.loaded.insert(path.to_string(), surface);
        debug!("Caching image {{ path: {} }}", path);
        return Ok(self
            .loaded
            .get_mut(path)
            .expect("Insertion was somehow misreported."));
    }
}
