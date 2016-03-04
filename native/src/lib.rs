#[macro_use]
extern crate neon;
extern crate voxel_worldgen;
extern crate rand;
extern crate nalgebra;
extern crate noise;

use voxel_worldgen::generators;
use voxel_worldgen::simplex_normalized::normalize_simplex;

use noise::{Seed, open_simplex2};
use voxel_worldgen::rnd::{ OctavesSeed, simplex3_octaves };
use rand::{ XorShiftRng, StdRng };
use nalgebra::{ Vec2, Pnt2 };

use neon::vm::{Call, JsResult, Module};
use neon::js::{JsInteger, JsString, JsObject, JsBuffer};
use neon::mem::Handle;
use neon::js::{Value, Object};

fn generate_chunk(call: Call) -> JsResult<JsObject> {
  let scope = call.scope;
  let object: Handle<JsObject> = JsObject::new(scope);
  let data: Handle<JsBuffer> = JsBuffer::new(scope);

  let seed = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>());
  let x = try!(try!(call.arguments.require(scope, 1)).check::<JsInteger>());
  let y = try!(try!(call.arguments.require(scope, 2)).check::<JsInteger>());

  let world_gen_state = generators::vanilla::WorldGeneratorState::new(&mut seed);
  let chunk = generators::vanilla::generate_chunk(&world_gen_state, Pnt2::new(x, y));

  let data: Handle<JsBuffer> = JsBuffer::new(scope, chunk.data.size());

  //try!(object.set("data", chunk.data));
  //try!(object.set("size", chunk.size));

  // try!(object.set("seed", seed));
  // try!(object.set("x", x));
  // try!(object.set("y", y));

  Ok(object)
}

register_module!(m, {
    m.export("generate_chunk", generate_chunk)
});