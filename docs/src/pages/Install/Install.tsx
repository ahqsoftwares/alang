import { useEffect, useState } from "react";
import { Props } from "../../App";

async function fetchReleases() {
  return await fetch("https://api.github.com/repos/ahqsoftwares/alang/releases/latest")
    .then(res => res.json())
    .then(data => data.assets)
    .then(assets => {
      function get(name: string) {
        return assets.find(({name: assetName}: {name: string}) => assetName == name).browser_download_url;
      }

      return {
        linux: get("alang_installer_linux"),
        macos: get("alang_installer_macos"),
        windows: get("alang_installer_windows.exe"),
      }
    });
}

export default function Install(props: Props) {
  const {
    platform
  } = props;

  const [releases, setReleases] = useState({linux: "", macos: "", windows: ""});

  useEffect(() => {
    fetchReleases()
      .then(setReleases)
      .catch(console.log)
  }, []);

  return <div className="install-modal">
    <h1>Install ALang for {platform}</h1>
    {
      platform == "Unsupported" 
        ? <All />
        : platform == "Linux"
        ? <Linux />
        : platform == "Macos"
        ? <Macos />
        : platform == "Windows"
        ? <Windows url={releases.windows} />
        : <All />
    }
  </div>;
}

function Windows(props: {url: string}) {
  return (<div className="install">
    <button onClick={() => window.location.href = props.url}>Download Installer</button>
  </div>);
}

function Macos() {
  return (<></>);
}

function Linux() {
  return (<></>);
}

function All() {
  return (<></>);
}