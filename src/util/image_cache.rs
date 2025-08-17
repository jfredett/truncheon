use std::{collections::HashMap, fs, path::PathBuf, sync::{self, Arc, RwLock}};

use resvg::usvg;

pub type Image = Arc<usvg::ImageKind>;

#[derive(Default)]
pub struct ImageCache {
    content: RwLock<HashMap<PathBuf, Image>>
}

impl ImageCache {
    pub fn get(&self, path: &std::path::Path) -> Option<Image> {
        let cache = self.content.read().unwrap();
        let img_path = fs::canonicalize(path).expect("File not found");

        tracing::info!("Cache keys are {:?}", cache);
        tracing::info!("Checking cache for {}", img_path.display());

        cache.get(&img_path).cloned()
    }

    /// Adds or retrieves the image from the cache.
    ///
    /// FIXME: If the file does not exist, this will crash in some fun way, instead it should use a
    /// default fill image
    ///
    /// FIXME: Asyncify
    ///
    /// FIXME: allow for preloading.
    pub fn add(&self, path: &str) -> Option<Image> {
        let img_path = fs::canonicalize(path).expect("File not found");

        if let Some(x) = self.get(&img_path) {
            tracing::info!("Cache hit for {}", img_path.display());
            return Some(x)
        }

        tracing::info!("looking at {}", img_path.display());
        let image_data = fs::read(&img_path).expect("Failed to read");
        let len = image_data.len();

        let img = Arc::new(image_data);

        let entry = Arc::new(if path.ends_with("png") {
            tracing::info!("found png, loaded, {} bytes wide", len);
            usvg::ImageKind::PNG(img.clone())
        } else if path.ends_with("webp") {
            tracing::info!("found webp, loaded, {} bytes wide", len);
            usvg::ImageKind::WEBP(img.clone())
        } else {
            tracing::error!("unrecognized file format");
            return None
        });

        { // write to the cache
            let mut cache = self.content.write().unwrap();
            cache.insert(img_path, entry.clone());
        }

        Some(entry)
    }
}

pub static IMAGE_CACHE : sync::LazyLock<ImageCache> = sync::LazyLock::new(|| {
    ImageCache::default()
});

