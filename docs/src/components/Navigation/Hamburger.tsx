interface MenuProps {
    open: boolean
}

export default function Menu(props: MenuProps) {
    const {
        open
    } = props;

    return (<div className={`nav-mobile ${open ? "open-mobile" : ""}`}>
    </div>);
}