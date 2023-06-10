const { readFileSync } = require("node:fs");
const toml = require('toml');

const { package: { version } } = toml.parse(
    readFileSync(
    "./Cargo.toml"
    ).toString()
)

const body = String(readFileSync("./latest.md"));

return {
    version,
    body
}