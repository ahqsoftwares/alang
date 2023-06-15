import fs from "fs";

const pages = [
    "home",
    "install",
    "docs",
    "404"
];

pages.forEach((page) => {
    fs.copyFileSync("./dist/index.html", `./dist/${page}.html`);
});