//! Sprite rendering for ESP32 OLED display
//! Converts ASCII art to pixel graphics

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
};
use tamagochi_core::LifeStage;

/// Draw sprite at given position
pub fn draw_sprite(
    display: &mut impl DrawTarget<Color = BinaryColor>,
    stage: LifeStage,
    frame: u8,
    position: Point,
) {
    match stage {
        LifeStage::Egg => draw_egg(display, position, frame),
        LifeStage::Baby => draw_baby(display, position, frame),
        LifeStage::Child => draw_child(display, position, frame),
        LifeStage::Teenager => draw_teenager(display, position, frame),
        LifeStage::Adult(form) => draw_adult(display, position, frame, form),
    }
}

/// Draw egg sprite (simple oval with animation)
fn draw_egg(display: &mut impl DrawTarget<Color = BinaryColor>, pos: Point, frame: u8) {
    // Egg body (oval approximation)
    Circle::new(pos, 20)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
        .draw(display)
        .ok();
    
    // Eyes that blink
    let eye_y = if frame % 10 < 8 { 2 } else { 0 }; // Blink occasionally
    
    if eye_y > 0 {
        // Left eye
        Circle::new(pos + Point::new(5, 8), 2)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(display)
            .ok();
        
        // Right eye
        Circle::new(pos + Point::new(13, 8), 2)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(display)
            .ok();
    } else {
        // Closed eyes (lines)
        Line::new(pos + Point::new(5, 9), pos + Point::new(7, 9))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(display)
            .ok();
        Line::new(pos + Point::new(13, 9), pos + Point::new(15, 9))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(display)
            .ok();
    }
}

/// Draw baby sprite
fn draw_baby(display: &mut impl DrawTarget<Color = BinaryColor>, pos: Point, frame: u8) {
    // Head
    Circle::new(pos, 16)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
        .draw(display)
        .ok();
    
    // Ears
    Circle::new(pos + Point::new(-2, 2), 4)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .ok();
    Circle::new(pos + Point::new(14, 2), 4)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .ok();
    
    // Eyes with animation
    let eye_offset = match frame % 4 {
        0 => 0,
        1 => 1,
        2 => 0,
        _ => -1,
    };
    
    Circle::new(pos + Point::new(4 + eye_offset, 6), 2)
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(display)
        .ok();
    Circle::new(pos + Point::new(10 + eye_offset, 6), 2)
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(display)
        .ok();
    
    // Mouth (smile)
    Line::new(pos + Point::new(5, 11), pos + Point::new(11, 11))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .ok();
    
    // Body (small)
    Rectangle::new(pos + Point::new(4, 18), Size::new(8, 8))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
        .draw(display)
        .ok();
}

/// Draw child sprite (bigger than baby)
fn draw_child(display: &mut impl DrawTarget<Color = BinaryColor>, pos: Point, frame: u8) {
    // Head
    Circle::new(pos, 18)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
        .draw(display)
        .ok();
    
    // Ears
    Circle::new(pos + Point::new(-2, 4), 5)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .ok();
    Circle::new(pos + Point::new(15, 4), 5)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .ok();
    
    // Animated eyes
    let eye_state = frame % 3;
    match eye_state {
        0 => {
            // Normal eyes
            Circle::new(pos + Point::new(5, 7), 2)
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                .draw(display)
                .ok();
            Circle::new(pos + Point::new(11, 7), 2)
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                .draw(display)
                .ok();
        }
        1 => {
            // Happy eyes (^_^)
            Line::new(pos + Point::new(4, 7), pos + Point::new(6, 9))
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                .draw(display)
                .ok();
            Line::new(pos + Point::new(6, 9), pos + Point::new(7, 7))
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                .draw(display)
                .ok();
            
            Line::new(pos + Point::new(11, 7), pos + Point::new(12, 9))
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                .draw(display)
                .ok();
            Line::new(pos + Point::new(12, 9), pos + Point::new(14, 7))
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                .draw(display)
                .ok();
        }
        _ => {
            // Wink
            Circle::new(pos + Point::new(5, 7), 2)
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                .draw(display)
                .ok();
            Line::new(pos + Point::new(11, 8), pos + Point::new(13, 8))
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                .draw(display)
                .ok();
        }
    }
    
    // Mouth
    Line::new(pos + Point::new(6, 13), pos + Point::new(12, 13))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .ok();
    
    // Body (larger)
    Rectangle::new(pos + Point::new(3, 20), Size::new(12, 10))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
        .draw(display)
        .ok();
}

/// Draw teenager sprite
fn draw_teenager(display: &mut impl DrawTarget<Color = BinaryColor>, pos: Point, frame: u8) {
    // Similar to child but taller
    draw_child(display, pos, frame);
    
    // Add feet
    Line::new(pos + Point::new(5, 32), pos + Point::new(5, 35))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
        .draw(display)
        .ok();
    Line::new(pos + Point::new(13, 32), pos + Point::new(13, 35))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
        .draw(display)
        .ok();
}

/// Draw adult sprite (varies by form)
fn draw_adult(
    display: &mut impl DrawTarget<Color = BinaryColor>,
    pos: Point,
    frame: u8,
    form: tamagochi_core::AdultForm,
) {
    use tamagochi_core::AdultForm;
    
    match form {
        AdultForm::Neglected => draw_neglected_adult(display, pos, frame),
        AdultForm::Normal => draw_normal_adult(display, pos, frame),
        AdultForm::Premium => draw_premium_adult(display, pos, frame),
    }
}

fn draw_neglected_adult(display: &mut impl DrawTarget<Color = BinaryColor>, pos: Point, frame: u8) {
    // Sad, droopy appearance
    draw_teenager(display, pos, frame);
    
    // Sad eyes (x_x)
    Line::new(pos + Point::new(4, 6), pos + Point::new(7, 9))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .ok();
    Line::new(pos + Point::new(7, 6), pos + Point::new(4, 9))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .ok();
    
    Line::new(pos + Point::new(11, 6), pos + Point::new(14, 9))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .ok();
    Line::new(pos + Point::new(14, 6), pos + Point::new(11, 9))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .ok();
}

fn draw_normal_adult(display: &mut impl DrawTarget<Color = BinaryColor>, pos: Point, frame: u8) {
    // Normal healthy adult
    draw_teenager(display, pos, frame);
}

fn draw_premium_adult(display: &mut impl DrawTarget<Color = BinaryColor>, pos: Point, frame: u8) {
    // Premium with sparkles
    draw_teenager(display, pos, frame);
    
    // Add sparkles around the head
    if frame % 2 == 0 {
        // Star left
        Line::new(pos + Point::new(-5, 5), pos + Point::new(-3, 5))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(display)
            .ok();
        Line::new(pos + Point::new(-4, 4), pos + Point::new(-4, 6))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(display)
            .ok();
        
        // Star right
        Line::new(pos + Point::new(21, 5), pos + Point::new(23, 5))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(display)
            .ok();
        Line::new(pos + Point::new(22, 4), pos + Point::new(22, 6))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(display)
            .ok();
    }
}
