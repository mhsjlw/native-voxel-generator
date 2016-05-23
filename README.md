node-voxel-worldgen
===================

[![NPM version](https://img.shields.io/npm/v/voxel-worldgen.svg)](https://www.npmjs.com/package/voxel-worldgen)

A voxel world generator written in Rust, with bindings for JavaScript

## Usage
You'll need [rust](https://www.rust-lang.org/) installed to compile the package. Once you've gotten it installed, just

    npm install voxel-worldgen
    
Then, if you are using it with flying-squid, simply add voxel-worldgen as the generator inside the `settings.json`!

## Thanks
- [dherman](https://github.com/dherman) for writing the [neon](https://github.com/rustbridge/neon) module that we use to compile the Rust bindings!
- [hansihe](https://github.com/hansihe) the original creator of [voxel_worldgen](https://github.com/hansihe/voxel_worldgen) the code that makes this module fly!
