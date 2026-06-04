use core::fmt;
use image::error::DecodingError;
use jwalk::WalkDir;
use rayon::prelude::*;
use std::collections::{BinaryHeap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use std::{cmp::Reverse, path::Path};

use image::{GenericImageView, ImageError};
use img_hash::{FilterType, Hasher, HasherConfig, ImageHash};

use crate::logger::LoggerSender;
use crate::state;

#[derive(Debug, Clone)]
pub struct PerceptualHash {
    pub path: String,
    pub dimensions: (u32, u32),
    hash: ImageHash,
}
impl PerceptualHash {
    pub fn new(path: String, hash: ImageHash, dimensions: (u32, u32)) -> Self {
        Self {
            path,
            hash,
            dimensions,
        }
    }
    pub fn dist(&self, other: &PerceptualHash) -> u32 {
        self.hash.dist(&other.hash)
    }
}

/// Offset dictates what index of the &[u8] it's targetting.
/// Signature dictates the random bits in the buckets it's scanning.
pub struct HammingBucket {
    pub signature: u8,
    pub offset: usize,
    pub items: Vec<PerceptualHash>,
}

impl HammingBucket {
    pub fn new(signature: u8, offset: usize) -> Self {
        Self {
            signature,
            offset,
            items: Vec::new(),
        }
    }

    pub fn get_key_similarity(&self, input: &[u8]) -> u32 {
        let input = input[self.offset];
        8 - (input ^ self.signature).count_ones()
    }
}

impl fmt::Debug for HammingBucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HammingBucket")
            .field("signature", &format_args!("{:#06b}", self.signature))
            .field("offset", &self.offset)
            .field("items", &self.items)
            .finish()
    }
}

/// This works by having buckets tagged with a random bit string.
/// The size of these bit strings are dictated by the resolution,
///     which should be __smaller__ than the image hash.
///
/// For insertion, a __subsection__ of the input (image hash)
///     scans each of these random bit strings.
/// It chooses the most similar one to itself, and goes into that bucket.
///
/// For search, a __subsection__ of the input (image hash)
///     scans each of these random bit strings again.
/// Only this time, when it finds the most similar one to itself,
///     it scans every single image in that bucket, and finds similar image hashes.
///
/// ====
/// The reason why I emphasize "a subsection" is because of the pigeonhole principle.
/// Too much bits to match with and you end up with no matches at all,
///     due to the opportunities for *many* bits to be different from one another.
///
/// We want as many collisions as possible, so we choose a smaller portion.
/// ====
///
/// This algorithm works on the assumption that two inputs
///     can derive their "hamming distance" from each other.
/// This means that the random bit strings can catch similar input string, even if not exact.
///
/// This enables fuzzy searching while ignoring
///     matches far away from the input, beating brute-force methods.
///
/// An improvement to this would be to handle an arbitrary number of buckets,
///     instead of being limited to only 8.
pub struct HammingClustererFinder {
    hasher: Hasher,
    buckets: Vec<HammingBucket>,
    sender: LoggerSender,
    cancelled: Arc<AtomicU8>,
}

impl HammingClustererFinder {
    pub fn new(sender: LoggerSender, cancelled: Arc<AtomicU8>) -> Self {
        let hasher = HasherConfig::new().to_hasher();
        let bucket_count = 8;
        let mut buckets: Vec<HammingBucket> = Vec::with_capacity(bucket_count as usize);

        for i in 0..8 {
            buckets.push(HammingBucket::new(0xFF, i));
        }

        Self {
            hasher,
            buckets,
            sender,
            cancelled,
        }
    }

    pub fn scan_directory(&mut self, directory: impl AsRef<Path>) {
        let hasher_config = HasherConfig::new().resize_filter(FilterType::CatmullRom);
        let cancelled = self.cancelled.clone();
        let map_cancelled = self.cancelled.clone();
        let files: Vec<_> = self.list_all_files(directory).collect();

        self.sender.total(files.len());

        self.sender
            .info(format!("Importing all images in directory."));

        let results: Vec<PerceptualHash> = files
            .par_iter()
            .take_any_while(move |_| cancelled.load(Ordering::Relaxed) != state::CANCELLED)
            .filter_map(|file| {
                let hasher = hasher_config.to_hasher();

                check_if_cancelled(map_cancelled.clone()).ok()?;

                self.sender.decoding(file.to_string_lossy());
                let img = image::open(&file).map_err(|s| (file.to_string_lossy().into_owned(), s));
                let Ok(img) = img else {
                    let (path, err) = img.unwrap_err();
                    self.sender.file_fail(path, err.to_string());
                    return None;
                };

                check_if_cancelled(map_cancelled.clone()).ok()?;

                self.sender.hash(file.to_string_lossy());
                let hash = hasher.hash_image(&img);

                check_if_cancelled(map_cancelled.clone()).ok()?;

                self.sender.finished(file.to_string_lossy());
                let dimensions = img.dimensions();

                check_if_cancelled(map_cancelled.clone()).ok()?;

                Some(PerceptualHash {
                    path: file.to_string_lossy().into_owned(),
                    hash,
                    dimensions,
                })
            })
            .collect();

        for result in results {
            self.insert(result);
        }
    }

    pub fn get_clustered_duplicates(&self, threshold: u32) -> Vec<Vec<PerceptualHash>> {
        let mut nearest_matches: Vec<Vec<PerceptualHash>> = vec![];
        let mut seen_path = HashSet::<String>::new();
        let cancelled = self.cancelled.clone();

        for container in self.buckets.iter() {
            for img in container.items.iter() {
                if cancelled.load(Ordering::Relaxed) == state::CANCELLED {
                    return vec![];
                }
                if seen_path.contains(&img.path) {
                    continue;
                }

                seen_path.insert(img.path.clone());

                let mut best_score = threshold + 1;
                let mut best_idx: Option<usize> = None;

                for (i, similars) in nearest_matches.iter().enumerate() {
                    let score = similars[0].hash.dist(&img.hash);
                    if score <= threshold && score < best_score {
                        best_score = score;
                        best_idx = Some(i);
                    }
                }

                match best_idx {
                    Some(i) => nearest_matches[i].push(img.clone()),
                    None => nearest_matches.push(vec![img.clone()]),
                }
            }
        }

        nearest_matches
            .into_iter()
            .filter(|m| m.len() > 1)
            .collect()
    }

    pub fn image<P>(&self, path: P) -> Result<PerceptualHash, ImageError>
    where
        P: AsRef<std::path::Path>,
    {
        let path_ref = path.as_ref();

        let img = image::open(path_ref)?;
        let hash = self.hasher.hash_image(&img);
        let dimensions = img.dimensions();

        Ok(PerceptualHash {
            path: path_ref.to_string_lossy().into_owned(),
            hash,
            dimensions,
        })
    }

    pub fn get_similar(&self, item: &PerceptualHash, threshold: u32) -> Vec<&PerceptualHash> {
        let mut results = Vec::new();

        let bucket_indexes = self.top_k_buckets(item, 1);

        for idx in bucket_indexes {
            if let Some(bucket) = self.buckets.get(idx) {
                for candidate in &bucket.items {
                    let is_threshold_reached = item.hash.dist(&candidate.hash) <= threshold;
                    let is_path_same = candidate.path == item.path;
                    if !is_path_same && is_threshold_reached {
                        results.push(candidate);
                    }
                }
            }
        }

        results
    }
    pub fn top_k_buckets(&self, item: &PerceptualHash, top_k: usize) -> Vec<usize> {
        let mut heap = BinaryHeap::with_capacity(top_k + 1);

        for (i, b) in self.buckets.iter().enumerate() {
            let score = b.get_key_similarity(item.hash.as_bytes());
            heap.push(Reverse((score, i)));
            if heap.len() > top_k {
                heap.pop();
            }
        }

        heap.into_iter().map(|Reverse((_, i))| i).collect()
    }

    pub fn insert(&mut self, item: PerceptualHash) {
        self.top_k_insert(item);
    }

    fn top_k_insert(&mut self, item: PerceptualHash) {
        let mut bucket_indexes = self.top_k_buckets(&item, 2);

        for idx in bucket_indexes.iter_mut().take(2) {
            self.buckets[*idx].items.push(item.clone());
        }
    }
    fn best_insert(&mut self, item: PerceptualHash) {
        let bytes = item.hash.as_bytes();
        let best_idx = self
            .buckets
            .iter()
            .enumerate()
            .max_by_key(|(_, b)| b.get_key_similarity(bytes))
            .map(|(i, _)| i);
        if let Some(idx) = best_idx {
            self.buckets[idx].items.push(item);
        }
    }

    fn list_all_files(&self, path: impl AsRef<Path>) -> impl Iterator<Item = PathBuf> {
        let cancelled = self.cancelled.clone();
        WalkDir::new(path)
            .into_iter()
            .take_while(move |_| cancelled.load(Ordering::Relaxed) != state::CANCELLED)
            .filter_map(move |v| match v {
                Ok(e) => {
                    let p = e.path();
                    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");

                    matches!(ext, "jpg" | "jpeg" | "png" | "webp").then_some(p)
                }
                Err(err) => {
                    println!("[FILE-ERR]: {}", err);
                    None
                }
            })
    }
}

impl fmt::Debug for HammingClustererFinder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HammingClustererFinder")
            .field("hasher", &String::from("The hasher."))
            .field("buckets", &self.buckets)
            .finish()
    }
}

fn check_if_cancelled(map_cancelled: Arc<AtomicU8>) -> Result<(), (String, ImageError)> {
    let is_cancelled = map_cancelled.load(Ordering::Relaxed) == state::CANCELLED;
    if is_cancelled {
        let img_err = ImageError::Decoding(DecodingError::from_format_hint(
            image::error::ImageFormatHint::Unknown,
        ));
        return Err((String::from("Cancelled!"), img_err));
    } else {
        return Ok(());
    }
}
