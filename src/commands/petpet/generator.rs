use std::io::Cursor;
use reqwest;
use gif::{DisposalMethod, Encoder, Frame, Repeat};
use image::{
  DynamicImage, GenericImage, GenericImageView,
  ImageReader,
  ColorType, Rgba,
  imageops::{overlay, FilterType},
};

const SIZE: u16 = 140;
const NUM_FRAMES: usize = 5;
const FRAME_DELAY: u16 = 6;
const FRAME_OFFSETS: [[f64; 4]; NUM_FRAMES] = [
  [0.0, 0.0, 0.0, 0.0],
  [-5.6, 16.8, 5.6, -16.8],
  [-16.8, 25.2, 16.8, -25.2],
  [-16.8, 16.8, 5.6, -16.8],
  [-5.6, 0.0, 0.0, 0.0],
];

type SPRITES = [Option<DynamicImage>; NUM_FRAMES];
pub struct Generator {
  sprites: SPRITES,
}

impl Generator {
  // Creates a new `Generator`, initializing the sprite images
  pub fn new() -> anyhow::Result<Generator> {
    let sprites = Generator::get_sprites()?;
    Ok(Generator { sprites })
  }

  // Generates a petpet gif from a given avatar
  // See Generator::get_avatar to fetch and crop avatars from URLs
  pub fn generate_gif(&self, avatar: &DynamicImage) -> anyhow::Result<Vec<u8>> {
    let mut res: Vec<u8> = Vec::new();

    if let Ok(mut encoder) = Encoder::new(&mut res, SIZE, SIZE, &[]) {
      encoder.set_repeat(Repeat::Infinite)?;
  
      for frame_index in 0..NUM_FRAMES {
        // Create frame as a static image
        let frame_image = self.render_frame_image(frame_index, &avatar)?;
        let mut frame_data = frame_image.as_rgba8()
          .ok_or(anyhow::format_err!("could not export image to raw data"))?
          .to_vec();
  
        // Convert image to a gif frame and write it to the encoder
        let mut frame = Frame::from_rgba_speed(SIZE, SIZE, frame_data.as_mut_slice(), 10);
        frame.delay = FRAME_DELAY;
        frame.dispose = DisposalMethod::Background;
        encoder.write_frame(&frame)?;
      }
    }

    Ok(res)
  }

  // Renders a single frame of the petpet image
  fn render_frame_image(&self, frame_index: usize, avatar: &DynamicImage) -> anyhow::Result<DynamicImage> {
    // Base layer for the frame
    let mut image = DynamicImage::new(SIZE.into(), SIZE.into(), ColorType::Rgba8);

    // Avatar layer
    let (x, y, w, h) = Generator::calculate_frame_offsets(frame_index);
    let avatar = avatar.resize_exact(w, h, FilterType::Lanczos3);
    image.copy_from(&avatar, x, y)?;

    // Sprite layer
    let sprite = self.sprites[frame_index]
      .as_ref()
      .ok_or(anyhow::format_err!("uninitialized sprite"))?;
    overlay(&mut image, sprite, 0, 0);

    Ok(image)
  }

  // Fetches an avatar by URL and crops it to a circle
  // The circle crop is done by coping the pixels within the circle onto
  // a new pallete, to handle RGB -> RGBA conversion
  pub async fn get_avatar(&self, url: String) -> anyhow::Result<DynamicImage> {
    let source = {
      let response = reqwest::get(url).await?;
      let bytes = response.bytes().await?;

      ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?
    };

    let mut destination = DynamicImage::new(source.width(), source.height(), ColorType::Rgba8);

    let center = (source.width() as f64) / 2.0;
    for (x, y, pixel) in source.pixels() {
      let distance = f64::hypot(
        (x as f64) - center,
        (y as f64) - center
      );

      if (distance + 3.0) < center {
        destination.put_pixel(x, y, pixel)
      }
    }

    Ok(destination)
  }

  // Calculates the frame dimensions of the avatar for a given frame
  fn calculate_frame_offsets(frame_index: usize) -> (u32, u32, u32, u32) {
    let offsets = FRAME_OFFSETS[frame_index];
    let x = (22.0 + offsets[0]) as u32;
    let y = (22.0 + offsets[1]) as u32;
    let w = (115.0 + offsets[2]) as u32;
    let h = (115.0 + offsets[3]) as u32;

    (x, y, w, h)
  }

  // Reads and parses the sprites from disk into DynamicImage instances
  fn get_sprites() -> anyhow::Result<SPRITES> {
    const EMPTY: Option<DynamicImage> = None;
    let mut sprites: SPRITES = [EMPTY; NUM_FRAMES];

    for frame_index in 0..NUM_FRAMES {
      let sprite_path = format!("./src/commands/petpet/sprite/sprite-{frame_index}.png");
      let mut sprite = ImageReader::open(sprite_path)?.decode()?;

      for y in 0..sprite.height() {
        let start = sprite.width() - 10;
        let end = sprite.width();
        for x in start..end {
          sprite.put_pixel(x, y, Rgba([0, 0, 0, 0]))
        }
      }

      sprites[frame_index] = Some(sprite);
    }

    Ok(sprites)
  }
}
