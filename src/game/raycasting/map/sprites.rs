use crate::core::sprites::Sprite;

pub const SPRITE_COUNT: usize = 19;

pub type SpriteMap = [Sprite; SPRITE_COUNT];
pub const SPRITE_MAP: SpriteMap = 
[
  Sprite::new(20.5, 11.5, 10), //green light in front of playerstart
  //green lights in every room
  Sprite::new(18.5,4.5, 10),
  Sprite::new(10.0,4.5, 10),
  Sprite::new(10.0,12.5,10),
  Sprite::new(3.5, 6.5, 10),
  Sprite::new(3.5, 20.5,10),
  Sprite::new(3.5, 14.5,10),
  Sprite::new(14.5,20.5,10),

  //row of pillars in front of wall: fisheye test
  Sprite::new(18.5, 10.5, 9),
  Sprite::new(18.5, 11.5, 9),
  Sprite::new(18.5, 12.5, 9),

  //some barrels around the map
  Sprite::new(21.5, 1.5, 8),
  Sprite::new(15.5, 1.5, 8),
  Sprite::new(16.0, 1.8, 8),
  Sprite::new(16.2, 1.2, 8),
  Sprite::new(3.5,  2.5, 8),
  Sprite::new(9.5, 15.5, 8),
  Sprite::new(10.0, 15.1,8),
  Sprite::new(10.5, 15.8,8),
];
