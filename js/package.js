const { readFileSync } = require("node:fs");
const process = require("node:process");
const { Octokit } = require("octokit");
const toml = require('toml');

(async() => {try{
console.log(`
░█████╗░██╗░░░░░░█████╗░███╗░░██╗░██████╗░
██╔══██╗██║░░░░░██╔══██╗████╗░██║██╔════╝░
███████║██║░░░░░███████║██╔██╗██║██║░░██╗░
██╔══██║██║░░░░░██╔══██║██║╚████║██║░░╚██╗
██║░░██║███████╗██║░░██║██║░╚███║╚██████╔╝
╚═╝░░╚═╝╚══════╝╚═╝░░╚═╝╚═╝░░╚══╝░╚═════╝░
`);

const { package: { version } } = toml.parse(
    readFileSync(
    "./Cargo.toml"
    ).toString()
)

const app = new Octokit({
    userAgent: "Alang/Nodejs",
    auth: process.env.ghtoken,
});

await app.auth();

let release = await app.request("GET /repos/{owner}/{repo}/tags/releases/{tag}", {
    owner: "ahqsoftwares",
    repo: "alang",
    tag: version,
    headers: {
        "Accept": "application/json"
    }
}).catch(() => false);

if (release == false) {
    release = await app.request('POST /repos/{owner}/{repo}/releases', {
        owner: 'ahqsoftwares',
        repo: 'alang',
        tag_name: version,
        target_commitish: 'master',
        name: `Alang ${version}`,
        body: readFileSync("./latest.md"),
        draft: true,
        prerelease: false,
        generate_release_notes: false,
        headers: {
            'X-GitHub-Api-Version': '2022-11-28'
        }
    });
}

console.log(`
██╗░░░██╗██████╗░██╗░░░░░░█████╗░░█████╗░██████╗░██╗███╗░░██╗░██████╗░
██║░░░██║██╔══██╗██║░░░░░██╔══██╗██╔══██╗██╔══██╗██║████╗░██║██╔════╝░
██║░░░██║██████╔╝██║░░░░░██║░░██║███████║██║░░██║██║██╔██╗██║██║░░██╗░
██║░░░██║██╔═══╝░██║░░░░░██║░░██║██╔══██║██║░░██║██║██║╚████║██║░░╚██╗
╚██████╔╝██║░░░░░███████╗╚█████╔╝██║░░██║██████╔╝██║██║░╚███║╚██████╔╝
░╚═════╝░╚═╝░░░░░╚══════╝░╚════╝░╚═╝░░╚═╝╚═════╝░╚═╝╚═╝░░╚══╝░╚═════╝░`);

await app.rest.repos.uploadReleaseAsset({
    owner: 'ahqsoftwares',
    repo: 'alang',
    release_id: release.id,
    label: `${process.env.filename}${process.env.os == "windows-latest" ? ".exe" : `.bin.${Math.random()}`}`,
    data: readFileSync("./Cargo.toml"),
    headers: {
      'X-GitHub-Api-Version': '2022-11-28',
      "Contect-Type": "application/octet-stream"
    }
});

}catch(e){err(e)}})()

function err(e) {
    console.log(e);

    console.log(`
░██████╗░█████╗░███╗░░░███╗███████╗████████╗██╗░░██╗██╗███╗░░██╗░██████╗░  
██╔════╝██╔══██╗████╗░████║██╔════╝╚══██╔══╝██║░░██║██║████╗░██║██╔════╝░  
╚█████╗░██║░░██║██╔████╔██║█████╗░░░░░██║░░░███████║██║██╔██╗██║██║░░██╗░  
░╚═══██╗██║░░██║██║╚██╔╝██║██╔══╝░░░░░██║░░░██╔══██║██║██║╚████║██║░░╚██╗  
██████╔╝╚█████╔╝██║░╚═╝░██║███████╗░░░██║░░░██║░░██║██║██║░╚███║╚██████╔╝  
╚═════╝░░╚════╝░╚═╝░░░░░╚═╝╚══════╝░░░╚═╝░░░╚═╝░░╚═╝╚═╝╚═╝░░╚══╝░╚═════╝░  

               ░██╗░░░░░░░██╗███████╗███╗░░██╗████████╗
               ░██║░░██╗░░██║██╔════╝████╗░██║╚══██╔══╝
               ░╚██╗████╗██╔╝█████╗░░██╔██╗██║░░░██║░░░
               ░░████╔═████║░██╔══╝░░██║╚████║░░░██║░░░
               ░░╚██╔╝░╚██╔╝░███████╗██║░╚███║░░░██║░░░
               ░░░╚═╝░░░╚═╝░░╚══════╝╚═╝░░╚══╝░░░╚═╝░░░

          ░██╗░░░░░░░██╗██████╗░░█████╗░███╗░░██╗░██████╗░
          ░██║░░██╗░░██║██╔══██╗██╔══██╗████╗░██║██╔════╝░
          ░╚██╗████╗██╔╝██████╔╝██║░░██║██╔██╗██║██║░░██╗░
          ░░████╔═████║░██╔══██╗██║░░██║██║╚████║██║░░╚██╗
          ░░╚██╔╝░╚██╔╝░██║░░██║╚█████╔╝██║░╚███║╚██████╔╝
          ░░░╚═╝░░░╚═╝░░╚═╝░░╚═╝░╚════╝░╚═╝░░╚══╝░╚═════╝░`);

    process.exit(1);
}