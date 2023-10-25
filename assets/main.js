import {Fretboard} from "./fretboard.js";

const fbContainer = document.querySelector("#fretboard-container");

function onClick(coord) {
    console.log("clicked", coord);
}

if (fbContainer) {
    const _fretboard = new Fretboard(
        fbContainer,
        {drawDotOnHover: true, onClick }
    );
}
