'use strict';

var addon = require('./native/index');

const Chunk = require('prismarine-chunk')("1.8");
const Block = require('prismarine-block')("1.8");

module.exports=function(options) {
  const seed=options.seed;

  function generateChunk(chunkX, chunkZ) {

    const fromBlocks=addon.generate_chunk(seed, chunkX, chunkZ);

    const chunk = new Chunk();

    chunk.initialize((x,y,z,n)=> {
      const block=new Block(fromBlocks[n],0,0);
      block.skyLight=15;
      return block;
    });
    return chunk;
  }
  return generateChunk;
};