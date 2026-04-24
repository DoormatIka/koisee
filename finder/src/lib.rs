pub mod finder;
pub mod logger;

#[cfg(test)]
mod tests {

    use super::*;
    use image;
    use img_hash::{self, HasherConfig};

    #[test]
    fn testing_img_hash() {
        let img1 = image::open("assets/tenshi1.jpg").unwrap();
        let img2 = image::open("assets/tenshi2.jpg").unwrap();

        let hasher = HasherConfig::new().to_hasher();

        let hash1 = hasher.hash_image(&img1);
        let hash2 = hasher.hash_image(&img2);

        println!("Image1 hash: {}", hash1.to_base64());
        println!("Image1 hash array: {:?}", hash1.as_bytes());
        println!("Image2 hash: {}", hash2.to_base64());
        println!("Image1 hash array: {:?}", hash2.as_bytes());

        println!("Hamming Distance: {}", hash1.dist(&hash2));
    }
}
