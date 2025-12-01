//! ASCII Art sprites for Tamagochi display

use crate::core::LifeStage;

/// Get ASCII art for a given life stage and animation frame
pub fn get_sprite(stage: LifeStage, frame: u8) -> &'static str {
    match stage {
        LifeStage::Egg => get_egg_sprite(frame),
        LifeStage::Baby => get_baby_sprite(frame),
        LifeStage::Child => get_child_sprite(frame),
        LifeStage::Teenager => get_teenager_sprite(frame),
        LifeStage::Adult(form) => get_adult_sprite(form, frame),
    }
}

fn get_egg_sprite(frame: u8) -> &'static str {
    match frame % 2 {
        0 => r#"
    ___
   /   \
  |  o  |
  |     |
   \___/
"#,
        _ => r#"
    ___
   /   \
  | o o |
  |     |
   \___/
"#,
    }
}

fn get_baby_sprite(frame: u8) -> &'static str {
    match frame % 3 {
        0 => r#"
   /\_/\
  ( o.o )
   > ^ <
  /|   |\
   |___|
"#,
        1 => r#"
   /\_/\
  ( ^.^ )
   > ~ <
  /|   |\
   |___|
"#,
        _ => r#"
   /\_/\
  ( -.o )
   > ^ <
  /|   |\
   |___|
"#,
    }
}

fn get_child_sprite(frame: u8) -> &'static str {
    match frame % 4 {
        0 => r#"
    /\_/\
   ( o.o )
    > ^ <
   /|   |\
  / |___| \
 /  \___/  \
"#,
        1 => r#"
    /\_/\
   ( ^.^ )
    > v <
   /|   |\
  / |___| \
 /  \___/  \
"#,
        2 => r#"
    /\_/\
   ( o.o )
    > w <
   /|   |\
  / |___| \
 /  \___/  \
"#,
        _ => r#"
    /\_/\
   ( -.o )
    > ^ <
   /|   |\
  / |___| \
 /  \___/  \
"#,
    }
}

fn get_teenager_sprite(frame: u8) -> &'static str {
    match frame % 3 {
        0 => r#"
     /\_/\
    ( o.o )
     > ^ <
    /|   |\
   / |___| \
  /  |   |  \
 /___|___|___\
"#,
        1 => r#"
     /\_/\
    ( ^.^ )
     > v <
    /|   |\
   / |___| \
  /  |   |  \
 /___|___|___\
"#,
        _ => r#"
     /\_/\
    ( o.O )
     > w <
    /|   |\
   / |___| \
  /  |   |  \
 /___|___|___\
"#,
    }
}

fn get_adult_sprite(form: crate::core::AdultForm, frame: u8) -> &'static str {
    use crate::core::AdultForm;
    
    match form {
        AdultForm::Neglected => get_neglected_adult(frame),
        AdultForm::Normal => get_normal_adult(frame),
        AdultForm::Premium => get_premium_adult(frame),
    }
}

fn get_neglected_adult(frame: u8) -> &'static str {
    match frame % 2 {
        0 => r#"
      /\_/\
     ( -.- )
      > _ <
     /|   |\
    / |___| \
   /  |   |  \
  /___|   |___\
     |___|
"#,
        _ => r#"
      /\_/\
     ( x.x )
      > _ <
     /|   |\
    / |___| \
   /  |   |  \
  /___|   |___\
     |___|
"#,
    }
}

fn get_normal_adult(frame: u8) -> &'static str {
    match frame % 4 {
        0 => r#"
      /\_/\
     ( o.o )
      > ^ <
     /|   |\
    / |___| \
   /  |   |  \
  /___|   |___\
     |___|
"#,
        1 => r#"
      /\_/\
     ( ^.^ )
      > v <
     /|   |\
    / |___| \
   /  |   |  \
  /___|   |___\
     |___|
"#,
        2 => r#"
      /\_/\
     ( o.o )
      > w <
     /|   |\
    / |___| \
   /  |   |  \
  /___|   |___\
     |___|
"#,
        _ => r#"
      /\_/\
     ( -.o )
      > ^ <
     /|   |\
    / |___| \
   /  |   |  \
  /___|   |___\
     |___|
"#,
    }
}

fn get_premium_adult(frame: u8) -> &'static str {
    match frame % 5 {
        0 => r#"
    ★ /\_/\ ★
     ( ^.^ )
      > ♥ <
     /|   |\
    / |___| \
   /  |   |  \
  /___|   |___\
     |___|
"#,
        1 => r#"
    ✨ /\_/\ ✨
     ( o.o )
      > w <
     /|   |\
    / |___| \
   /  |   |  \
  /___|   |___\
     |___|
"#,
        2 => r#"
    ★ /\_/\ ★
     ( ^ω^ )
      > ♥ <
     /|   |\
    / |___| \
   /  |   |  \
  /___|   |___\
     |___|
"#,
        3 => r#"
    ✨ /\_/\ ✨
     ( ´∀` )
      > v <
     /|   |\
    / |___| \
   /  |   |  \
  /___|   |___\
     |___|
"#,
        _ => r#"
    ★ /\_/\ ★
     ( ^.^ )
      > ♪ <
     /|   |\
    / |___| \
   /  |   |  \
  /___|   |___\
     |___|
"#,
    }
}

/// Get death sprite
pub fn get_death_sprite() -> &'static str {
    r#"
      /\_/\
     ( x.x )
      > _ <
     /|   |\
    / |___| \
   /  |   |  \
  /___|   |___\
     |___|
     R.I.P.
"#
}

/// Get sleeping sprite (for idle states)
pub fn get_sleeping_sprite() -> &'static str {
    r#"
      /\_/\
     ( -.- )
      > z <
     /|   |\
    / |___| \
   /  Zzz..  \
  /_________\
"#
}
