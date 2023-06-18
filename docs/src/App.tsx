import React, { useEffect, useState } from "react";
import Nav from "./components/Navigation";

interface Props {
    setPage: React.Dispatch<React.SetStateAction<string>>;
    platform: "Windows" | "Macos" | "Linux" | "Unsupported"
}

const pages: { [key: string]: (props: Props) => Promise<JSX.Element> } = {
    "/": async(props: Props) => {
        return (await import("./pages/Home")).default(props);
    },
    "/home": async(props: Props) => {
        return (await import("./pages/Home")).default(props);
    },
    "/install": async(props: Props) => {
        return (await import("./pages/Install")).default(props);
    },
    "/docs": async(props: Props) => {
        return (await import("./pages/Home")).default(props);
    },
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    "/404": async(_: Props) => {
        return <></>;
    }
};

function Loading() {
    useEffect(() => {
        const progress = document.getElementById("progress-2") as HTMLDivElement;

        let value = 0;
        const listener = setInterval(() => {
            if (value == 100) {
                value = 0;
            }

            value += 1;

            progress.setAttribute("style", `height: 100%;width: ${value}%`);
        }, 10);

        return () => clearInterval(listener)
    }, []);

    return <div id="progress-container">
        <h1>Loading</h1>
        <div id="progress">
            <div id="progress-2"></div>
        </div>
    </div>;
}

export default function App() {
    const platform = (() => {
        const platform = window.navigator.userAgent.toLowerCase();

        if (platform.includes('win')) {
            return "Windows";
        } else if (platform.includes('mac')) {
            return "Macos";
        } else if (platform.includes('linux') && !platform.includes('android')) {
            return "Linux";
        } else {
            return "Unsupported";
        }
    })();

    const pageUrl = window.location.pathname.replace("/alang", "");

    if (pageUrl == "/") {
        window.location.pathname = "/alang/home";
    }

    const pageElement = pages[pageUrl];

    const [currentPage, setPage] = useState(pageUrl);
    const [privateJsx, setJsx] = useState(<Loading />);

    if (!pageElement) {
        window.location.pathname = "/alang/404";
    }

    useEffect(() => {
        window.history.replaceState(null, "", "/alang/" + currentPage.replace("/", ""));
        (async() => {
            const element = await pageElement({
                setPage,
                platform
            });

            setJsx(element);
        })()
    }, [pageElement, currentPage, platform]);

    return <div className="main-container">
        <Nav
            page={currentPage}  
            pages={Object.keys(pages)}
            setPage={setPage}
        />
        {privateJsx}
    </div>;
}

export type {
    Props
}