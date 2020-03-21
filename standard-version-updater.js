const TOML = require('@iarna/toml');

module.exports.readVersion = function(cargo_toml) {
  manifest = TOML.parse(cargo_toml);
  return manifest.package.version;
};


module.exports.writeVersion = function(cargo_toml, version) {
  manifest = TOML.parse(cargo_toml);
  manifest.package.version = version;
  return TOML.stringify(manifest);
};
