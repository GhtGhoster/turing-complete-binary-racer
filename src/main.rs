use std::{thread::sleep, time::Duration};

use enigo::{Coordinate, Enigo, Mouse, Settings};
use screenshots::{image::{io::Reader, GenericImageView, ImageBuffer}, Screen};

const ORANGE: [u8; 4] = [228, 159, 68, 255];
const WHITE: [u8; 4] = [255, 255, 255, 255];
const BLACK: [u8; 4] = [0, 0, 0, 255];

const BUTTON_Y: i32 = 955;
const BUTTON_X: i32 = 1920 + 590;
const BUTTON_SPACING: i32 = 95;

const SUBMIT_Y: i32 = 1030;
const SUBMIT_X: i32 = 1920 / 2 + 1920;

const SCREEN_X: i32 = 800;
const SCREEN_Y: i32 = 490;
const SCREEN_W: u32 = 300;
const SCREEN_H: u32 = 60;

fn main() {
    let screens = Screen::all().unwrap();
    let screen = screens[0];

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    loop {
        let mut image: ImageBuffer<screenshots::image::Rgba<u8>, Vec<u8>> = screen.capture_area(SCREEN_X, SCREEN_Y, SCREEN_W, SCREEN_H).unwrap();
        // image.save("tmp.png").unwrap();

        let mut subimages = vec![];

        let mut start = 0;
        for x in 0..SCREEN_W {
            let mut found = false;
            for y in 0..SCREEN_H {
                let pixel = image.get_pixel_mut(x, y);
                pixel.0 = if pixel.0 == ORANGE {
                    found = true;
                    BLACK
                } else {
                    WHITE
                }
            }
            if found {
                if start == 0 {
                    start = x;
                }
            } else {
                if start != 0 {
                    let subimage = screenshots::image::imageops::crop(&mut image, start, 0, x - start, 60).to_image();
                    subimages.push(subimage);
                    start = 0;
                }
            }
        }

        if subimages.is_empty() {
            break;
        }

        // for (i, subimage) in subimages.iter().enumerate() {
        //     subimage.save(format!("tmp{i}.png")).unwrap();
        // }

        let mut detected_number = 0;
        for subimage in subimages {
            detected_number *= 10;
            let mut best_match_index = 0;
            let mut best_match_difference = SCREEN_H * SCREEN_H;
            for i in 0..10 {
                let comparison_image = Reader::open(format!("{i}.png")).unwrap().decode().unwrap();
                let width = comparison_image.width().max(subimage.width());
                let mut difference_count = 0;
                for x in 0..width {
                    for y in 0..SCREEN_H {
                        let subimage_pixel = if x >= subimage.width() {
                            WHITE
                        } else {
                            subimage.get_pixel(x, y).0
                        };
                        let comparison_pixel = if x >= comparison_image.width() {
                            WHITE
                        } else {
                            comparison_image.get_pixel(x, y).0
                        };
                        if comparison_pixel != subimage_pixel {
                            difference_count += 1;
                        }
                    }
                }
                if difference_count < best_match_difference {
                    best_match_difference = difference_count;
                    best_match_index = i;
                }
            }
            detected_number += best_match_index;
        }

        println!("Detected number: {detected_number} ({detected_number:08b})");
        let string = format!("{detected_number:08b}");

        for (i, char) in string.char_indices() {
            if char == '1' {
                let button_x = BUTTON_X + (BUTTON_SPACING * i as i32);
                enigo.move_mouse(button_x, BUTTON_Y, Coordinate::Abs).unwrap();
                sleep(Duration::from_millis(50));
                enigo.button(enigo::Button::Left, enigo::Direction::Click).unwrap();
                sleep(Duration::from_millis(50));
            }
        }

        enigo.move_mouse(SUBMIT_X, SUBMIT_Y, Coordinate::Abs).unwrap();
        sleep(Duration::from_millis(50));
        enigo.button(enigo::Button::Left, enigo::Direction::Click).unwrap();
        sleep(Duration::from_millis(1000));
    }
}
