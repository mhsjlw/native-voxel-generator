var addon = require('../native');

module.exports = function(seed, x, y) {
  return addon.generate_chunk(seed, x, y);
}