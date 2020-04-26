use js_sys::Math::{random, floor};
use web_sys::{CanvasRenderingContext2d, ImageData, console};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, Clamped};
use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::screen_buffer::{Color, ScreenBuffer};
use crate::maze::Maze;
use crate::draw_api::DrawApi;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

struct Api {
  ctx: CanvasRenderingContext2d,
  width: usize,
  height: usize,
  image_data_data: Option<wasm_bindgen::Clamped<Vec<u8>>>
}

impl Api {
  pub fn new(width: usize, height: usize) -> Self {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .map_err(|e| {
        console::log_1(&e.to_string().into());
      })
      .unwrap();

    let ctx = canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<CanvasRenderingContext2d>()
      .unwrap();
    Api {
      ctx,
      width,
      height,
      image_data_data: None
    }
  }

  pub fn draw_start(&mut self) {
    let clamped = self.ctx.create_image_data_with_sw_and_sh(
      self.width as f64,
      self.height as f64
    ).unwrap().data();
    self.image_data_data = Some(
      clamped
    );
  }
}

impl DrawApi for Api {
  fn draw_apply(&mut self) {
    if let Some(mut clamped) = self.image_data_data.take() {
      let cl: Clamped<&mut [u8]> = Clamped(&mut *clamped.as_mut_slice());
      let img_data = ImageData::new_with_u8_clamped_array(cl, self.width as u32).unwrap();
      /*for (idx, i) in img_data.data().iter().enumerate() {
        console::log_2(&(idx as i32).into(),&(i.clone() as i32).into());
      }*/

      self.ctx.put_image_data(&img_data, 0.0, 0.0)
        .map_err(|e| console::log_1(&e))
        .unwrap();
    }
    log("Draw applied");
  }

  fn draw_api(&mut self, x: usize, y: usize, color: Color) {
    /*let log_x: JsValue = (x.clone() as i32).into();
    let log_y: JsValue = (y.clone() as i32).into();
    let col_log: [u8; 4] = color.clone().into();*/
    /*console::log_6(
      &log_x,
      &log_y,
      &col_log[0].into(),
      &col_log[1].into(),
      &col_log[2].into(),
      &col_log[3].into()
    );*/
    if self.image_data_data.is_none() {
      self.draw_start();
    }
    if let Some(img_data) = &mut self.image_data_data {
      let start_idx = (y * self.width + x) * 4;
      let col: [u8; 4] = color.into();
      for (i, val) in col.iter().enumerate() {
        img_data[start_idx + i] = *val;
        // console::log_2(&((start_idx + i) as i32).into(), &img_data[start_idx + i].clone().into());
      }
    }
  }
}

static COLUMNS: usize = 60;
static ROWS: usize = 40;
static BLOCK_SIZE: usize = 20;

#[wasm_bindgen]
pub fn start() {
  log("WASM BOOT");
  let pixel_w = COLUMNS * BLOCK_SIZE + 1;
  let pixel_h = ROWS * BLOCK_SIZE + 1;
  let api_instance = Api::new(pixel_w, pixel_h);
  let rng = |min: usize, max: usize| -> usize {
    (floor(random() * max as f64 - min as f64) + min as f64) as usize
  };
  {
    let maze = Maze::new(
      COLUMNS,
      ROWS,
      Box::new(rng)
    );
    log("Maze generated");
    let mut buf = ScreenBuffer::new(
      BLOCK_SIZE,
      COLUMNS,
      ROWS,
      Box::new(api_instance)
    );
    log("Buffer created");
    maze.feed_whitespace(&mut buf);
    buf.draw();
    for i in maze.debug() {
      console::log_1(&(i.clone() as i32).into());
    }
  }
}
