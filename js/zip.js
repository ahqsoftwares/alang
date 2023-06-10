const AdmZip = require("adm-zip");
const fs = require("fs");
const { Octokit } = require("octokit");

module.exports = (async () => {
  const osEs = {
    "ubuntu-latest": ["linux", "./target/release/alang"],
    "macos-latest": ["macos", "./target/release/alang"],
    "windows-latest": ["windows", "./target/release/alang.exe"],
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
      `${packloader}/packloader`,
    ],
    macos: [
      `${cli}/cli`,
      `${compiler}/compiler`,
      `${interpreter}/interpreter`,
      `${packager}/packager`,
      `${packloader}/packloader`,
    ],
    windows: [
      `${cli}/cli.exe`,
      `${compiler}/compiler.exe`,
      `${interpreter}/interpreter.exe`,
      `${packager}/packager.exe`,
      `${packloader}/packloader.exe`,
    ],
  };

  const files = filesToZip[osName];

  const file = new AdmZip();

  for (let i = 0; i < files.length; i++) {
    file.addLocalFile(files[i]);
  }

  const toolsName = `tools-${osName}.zip`;
  const toolsPath = `./${toolsName}`;

  file.writeZip(toolsPath);

  const installerName = `alang_installer_${osName}${
    osName == "windows" ? ".exe" : ""
  }`;
  const installerPath = `./${installerName}`;

  fs.copyFileSync(installer, installerPath);

  const data = {
    installer: installerPath,
    tools: toolsPath,
    name: {
      tools: toolsName,
      installer: installerName,
    },
  };

  const github = new Octokit({
    auth: process.env.token,
  });

  const base = {
    owner: "ahqsoftwares",
    repo: "alang",
    release_id: process.env.releaseid
  };

  await github.rest.repos.uploadReleaseAsset({
    ...base,
    name: data.name.tools,
    data: fs.readFileSync(data.tools),
    headers: {
      "Content-Type": "application/zip",
    }
  });

  await github.rest.repos.uploadReleaseAsset({
    ...base,
    name: data.name.installer,
    data: fs.readFileSync(data.installer),
    headers: {
      "Content-Type": "application/octet-stream",
    }
  });
})();
