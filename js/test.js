const jsonParsed = JSON.parse(
  `{"installer":"./alang_installer_linux","tools":"./tools-linux.zip","toolsName":"tools-linux.zip","installerName":"alang_installer_linux"}`
);

const fs = require("fs");
const uploadUrl = process.env.uurl;
const toUpload = [
  [jsonParsed["tools"], jsonParsed["toolsName"], "application/zip"],
  [
    jsonParsed["installer"],
    jsonParsed["installerName"],
    "application/octet-stream",
  ],
];

console.log(toUpload);
