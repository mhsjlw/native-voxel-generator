const VERSION = '1.8'

const World = require('prismarine-world')(VERSION)
const { Vec3 } = require('vec3')

const SEED = Math.floor(Math.random() * Math.pow(2, 31))
const cGenerator = require('../')({ version: VERSION, seed: SEED });
const world = new World(cGenerator)

world.getBlock(new Vec3(3, 50, 3))
  .then(block => console.log(JSON.stringify(block, null, 2)))
  .catch(e => console.log(e))
