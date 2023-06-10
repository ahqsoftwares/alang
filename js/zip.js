const AdmZip = require("adm-zip");
const fs = require("fs");

const osEs = {
    "ubuntu-latest": ["linux", "./target/release/alang"],
    "macos-latest": ["macos", "./target/release/alang"],
    "windows-latest": ["windows", "./target/release/alang.exe"]
};

const [osName, installer] = osEs[process.env.OS.toLowerCase()];

const cli = "./cli/target/release";
const compiler = "./compiler/target/release";
const interpreter = "./interpreter/target/release";
const packager = "./packager/target/release";
const packloader = "./packloader/target/release";

const filesToZip = {
    linux: [
        `${cli}/cli`,
        `${compiler}/compiler`,
        `${interpreter}/interpreter`,
        `${packager}/packager`,
        `${packloader}/packloader`
    ],
    macos: [
        `${cli}/cli`,
        `${compiler}/compiler`,
        `${interpreter}/interpreter`,
        `${packager}/packager`,
        `${packloader}/packloader`
    ],
    windows: [
        `${cli}/cli.exe`,
        `${compiler}/compiler.exe`,
        `${interpreter}/interpreter.exe`,
        `${packager}/packager.exe`,
        `${packloader}/packloader.exe`
    ]
}

const files = filesToZip[osName];

const file = new AdmZip();

for (let i = 0; i < files.length; i++) {
    file.addLocalFile(files[i]);
}

const toolsPath = `./tools-${osName}.zip`

file.writeZip(toolsPath);

const installerPath = `./alang_installer_${osName}${osName == "windows" ? ".exe" : ""}`;

fs.copyFileSync(installer, installerPath);

module.exports = {
    installer: installerPath,
    tools: toolsPath
};