const { nativeGenerateChunk } = require('./native/index')

module.exports = function ({ version, seed }) {
  const Chunk = require('prismarine-chunk')(version)
  const Block = require('prismarine-block')(version)

  return function (chunkX, chunkZ) {
    const fromBlocks = nativeGenerateChunk(seed, chunkX, chunkZ)
    const chunk = new Chunk()

    chunk.initialize((x, y, z, n) => {
      const block = new Block(fromBlocks[n], 0, 0)
      block.skyLight = 15
      return block
    })

    return chunk
  }
}
