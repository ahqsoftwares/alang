import React from "react";
import ALang from "/alang.svg";

import { GiHamburgerMenu } from "react-icons/gi";
import Menu from "./Hamburger";

interface Navigation {
    page: string,
    setPage: React.Dispatch<React.SetStateAction<string>>,
    pages: string[],
}

const parsePage: {[key: string]:  string} = {

};

export default function Nav(props: Navigation) {
    const {
        page,
        setPage,
        pages
    } = props;

    const parsedPages = pages.filter((page) => page != "/404" && page != "/").map((page) => ({
        name: (() => {
            const name = (parsePage[page] || page).replace("/", "");

            return name[0].toUpperCase() + name.slice(1).toLowerCase();
        })(),
        url: page
    }));

    return (<div className="nav-container">
        <div className="nav">
            <Menu 
                open={true}
            />

            <div className="mobile">
                <GiHamburgerMenu
                    size={"35px"}
                />
            </div>

            <img
                src={ALang}
                width={"65px"}
                height={"65px"}
                onClick={() => setPage("home")}
            />

            <div className="desktop">
                {
                    parsedPages.map(({ name, url }) => 
                        <div id={name} className={page.replace("/", "") == url.replace("/", "") ? "active" : ""} onClick={() => setPage(url)}>
                            <h1>{name}</h1>
                        </div>
                    )
                }
            </div>
        </div>
    </div>);
}