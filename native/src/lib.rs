#[macro_use]
extern crate neon;
extern crate voxel_worldgen;
extern crate rand;
extern crate nalgebra;
extern crate noise;

use voxel_worldgen::generators;
use voxel_worldgen::rnd::RngBuilder;

use nalgebra::Pnt2;

use neon::vm::{Call, JsResult};
use neon::js::{JsInteger, JsObject};
use neon::js::binary::{ JsBuffer };
use neon::mem::Handle;
use neon::js::Object;

fn generate_chunk(call: Call) -> JsResult<JsBuffer> {
  let scope = call.scope;

  let seed = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>()).value();
  let mut seed_rng = RngBuilder::init().mix(seed as u64).build();
  let world_gen_state = generators::vanilla::WorldGeneratorState::new(&mut seed_rng);

  let x = try!(try!(call.arguments.require(scope, 1)).check::<JsInteger>());
  let y = try!(try!(call.arguments.require(scope, 2)).check::<JsInteger>());
  let chunk_pos = Pnt2::new(x.value() as i32, y.value() as i32);

  let chunk = generators::vanilla::generate_chunk(&world_gen_state, chunk_pos);

  let data: Handle<JsBuffer> = try!(JsBuffer::new(scope, chunk.data.len() as u32));

  for (i, v) in chunk.data.iter().enumerate() {
      try!(data.set(i as u32, JsInteger::new(scope, *v as i32)));
  }

  Ok(data)
}

register_module!(m, {
    m.export("generate_chunk", generate_chunk)
});
