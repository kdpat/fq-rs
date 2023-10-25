import { Fretboard } from "./fretboard.js";

const fretboardContainer = document.querySelector("#fretboard-container");

if (fretboardContainer) {
    const _fretboard = new Fretboard(fretboardContainer);
}
