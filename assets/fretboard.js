const DEFAULT_OPTS = {
    width: 200,
    height: 300,
    startFret: 1,
    endFret: 4,
    stringNames: "EBGDAE".split(""),
    dots: [],
    dotColor: "white",
    hoverDotColor: "white",
    showFretNums: true,
    showStringNames: false,
    drawDotOnHover: false,
}

export class Fretboard {
    constructor(parent, userOpts = {}) {
        this.opts = {...DEFAULT_OPTS, ...userOpts};

        this.xMargin = this.opts.width / this.numStrings;
        this.yMargin = this.opts.height / 8;
        this.neckWidth = this.opts.width - (this.xMargin * 2);
        this.neckHeight = this.opts.height - (this.yMargin * 2);
        this.fretHeight = this.neckHeight / this.numFrets;
        this.stringMargin = this.neckWidth / (this.numStrings - 1);

        // this.dotRadius = this.fretHeight / 6;

        this.svg = makeSvgElement(this.opts.width, this.opts.height);
        this.svg.style.border = "1px solid"

        this.addStrings();
        this.addFrets();

        parent.appendChild(this.svg);
    }

    addStrings() {
        for (let i = 0; i < this.numStrings; i++) {
            const x = (i * this.stringMargin) + this.xMargin;
            const y1 = this.yMargin;
            const y2 = this.yMargin + this.neckHeight;
            const line = makeLine(x, y1, x, y2);
            this.svg.appendChild(line);
        }
    }

    addFrets() {
        for (let i = 0; i <= this.numFrets; i++) {
            const y = (i * this.fretHeight) + this.yMargin;
            const x1 = this.xMargin;
            const x2 = this.opts.width - this.xMargin;
            const line = makeLine(x1, y, x2, y);
            this.svg.appendChild(line);
        }
    }

    remove() {
        this.svg.remove();
    }

    get numStrings() {
        return this.opts.stringNames.length;
    }

    get numFrets() {
        return this.opts.endFret - this.opts.startFret + 1;
    }
}

const SVG_NS = 'http://www.w3.org/2000/svg';

function makeSvgElement(width, height) {
    const elem = document.createElementNS(SVG_NS, 'svg');
    elem.setAttribute('width', width.toString());
    elem.setAttribute('height', height.toString());
    elem.setAttribute('viewBox', `0 0 ${width} ${height}`);
    return elem;
}

function makeLine(x1, y1, x2, y2, color = 'black') {
    const line = document.createElementNS(SVG_NS, 'line');
    line.setAttribute('x1', x1.toString());
    line.setAttribute('y1', y1.toString());
    line.setAttribute('x2', x2.toString());
    line.setAttribute('y2', y2.toString());
    line.setAttribute('stroke', color);
    return line;
}

function makeCircle(cx, cy, r, color = 'white') {
    const circle = document.createElementNS(SVG_NS, 'circle');
    circle.setAttribute('cx', cx.toString());
    circle.setAttribute('cy', cy.toString());
    circle.setAttribute('r', r.toString());
    circle.setAttribute('stroke', 'black');
    circle.setAttribute('fill', color);
    return circle;
}

function makeText(x, y, text, fontSize = 16) {
    const textElem = document.createElementNS(SVG_NS, 'text');
    textElem.setAttribute('x', x.toString());
    textElem.setAttribute('y', y.toString());
    textElem.setAttribute('text-anchor', 'middle');
    textElem.setAttribute('font-size', fontSize.toString());

    const textNode = document.createTextNode(text);
    textElem.appendChild(textNode);

    return textElem;
}
