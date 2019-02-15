extern crate image;
extern crate rand;
use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
use image::Pixel;
use rand::Rng;

struct rgb {
    r: u32,
    g: u32,
    b: u32
}

pub fn threshold(mut img: DynamicImage, threshold: u32) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);

            let r: f32 = px.data[0].into();
            let g: f32 = px.data[1].into();
            let b: f32 = px.data[2].into();

            let mut v = (0.2126 * r + 0.7152 * g + 0.072 * b);

            if v >= threshold as f32 {
                v = 255.0;
            }
            else {
                v = 0.0;
            }
            px.data[0] = v as u8;
            px.data[1] = v as u8;
            px.data[2] = v as u8;

            img.put_pixel(x, y, px);
        }
    }
    return img;
}

fn grayscale(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            let mut avg = (r_val + g_val + b_val) / 3;
            if (avg >= 255) {
                avg = 255
            }
            px.data[0] = avg as u8;
            px.data[1] = avg as u8;
            px.data[2] = avg as u8;
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

pub mod channels {
    pub fn alter_channel(mut img: DynamicImage, channel: usize, offset: u8) -> DynamicImage {
        let (width, height) = img.dimensions();
        let mut rng = rand::thread_rng();

        for x in 0..width {
            for y in 0..height {
                let mut px = img.get_pixel(x, y);
                if px.data[channel] <= 255 - offset {
                    px.data[channel] += offset;
                }
                else {
                    px.data[channel] = 255;
                }
                img.put_pixel(x, y, px)
            }
        }
        return img;
    }

    fn alter_red_channel(mut img: DynamicImage, offset: u8) -> DynamicImage {
        let res_img = alter_channel(img, 0, offset);
        return res_img;
    }

    fn alter_green_channel(mut img: DynamicImage, offset: u8) -> DynamicImage {
        let res_img = alter_channel(img, 1, offset);
        return res_img;
    }

    fn alter_blue_channel(mut img: DynamicImage, offset: u8) -> DynamicImage {
        let res_img = alter_channel(img, 2, offset);
        return res_img;
    }
}

pub mod filters {
    // In-built image filters
    pub fn oceanic(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = alter_two_channels(img, 1, 9, 2, 173);
        return filtered_img;
    }

    pub fn islands(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = alter_two_channels(img, 1, 24, 2, 95);
        return filtered_img;
    }

    pub fn marine(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = alter_two_channels(img, 1, 14, 2, 119);
        return filtered_img;
    }

    pub fn seagreen(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = alter_two_channels(img, 1, 68, 2, 62);
        return filtered_img;
    }

    pub fn flagblue(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = alter_blue_channel(img, 131);
        return filtered_img;
    }

    pub fn diamante(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = alter_two_channels(img, 1, 82, 2, 87);
        return filtered_img;
    }

    pub fn liquid(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = alter_two_channels(img, 1, 10, 2, 75);
        return filtered_img;
    }

}