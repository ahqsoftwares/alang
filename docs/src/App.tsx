import React, { useEffect, useState } from "react";

interface Props {
    setPage: React.Dispatch<React.SetStateAction<string>>;
}

const pages: { [key: string]: (props: Props) => Promise<JSX.Element> } = {
    "/": async(props: Props) => {
        return (await import("./pages/Home")).default(props);
    },
    "/home": async(props: Props) => {
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
        let decrease = true;
        const listener = setInterval(() => {
            if (value == 100 || value == 0) {
                decrease = !decrease;
            }

            if (decrease) {
                value -= 2;
            } else {
                value += 1;
            }

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
    const pageUrl = window.location.pathname.replace("/alang", "");
    const pageElement = pages[pageUrl];

    const [currentPage, setPage] = useState(pageUrl);
    const [privateJsx, setJsx] = useState(<Loading />);

    if (!pageElement) {
        window.location.pathname = "/alang/404";
    }

    useEffect(() => {
        (async() => {
            const element = await pageElement({
                setPage
            });
            setJsx(element);
        })()
    }, [pageElement, currentPage]);

    return <div className="main-container">
        {privateJsx}
    </div>;
}

export type {
    Props
}