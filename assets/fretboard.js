const DEFAULT_OPTS = {
  width: 200,
  height: 300,
  startFret: 0,
  endFret: 4,
  stringNames: "EBGDAE".split(""),
  dots: [],
  dotColor: "white",
  hoverDotColor: "white",
  showFretNums: true,
  showStringNames: false,
  drawDotOnHover: false,
  onClick: null,
}

export class Fretboard {
  constructor(parentEl, userOpts = {}) {
    this.opts = {...DEFAULT_OPTS, ...userOpts};

    this.xMargin = this.opts.width / this.numStrings;
    this.yMargin = this.opts.height / 8;
    this.neckWidth = this.opts.width - (this.xMargin * 2);
    this.neckHeight = this.opts.height - (this.yMargin * 2);
    this.fretHeight = this.neckHeight / this.numFrets;
    this.stringMargin = this.neckWidth / (this.numStrings - 1);
    this.dotRadius = this.fretHeight / 6;

    this.hoverDot = null;
    this.hoverCoord = null;

    this.svg = makeSvgElement(this.opts.width, this.opts.height);

    this.addStrings();
    this.addFrets();
    this.addDots();

    if (this.opts.drawDotOnHover) {
      this.svg.onmousemove = this.onMouseMove.bind(this);
    }

    if (this.opts.onClick) {
      this.svg.onclick = this.onClick.bind(this);
    }

    parentEl.appendChild(this.svg);
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
      const x1 = this.xMargin;
      const x2 = this.opts.width - this.xMargin;
      const y = (i * this.fretHeight) + this.yMargin;
      const line = makeLine(x1, y, x2, y);
      this.svg.appendChild(line);
    }
  }

  addDots() {
    for (const dot of this.opts.dots) {
      const elem = this.makeDot(dot.string, dot.fret, dot.color);
      this.svg.appendChild(elem);
    }
  }

  makeDot(string, fret, color) {
    const coord = this.fretCoord(string, fret);

    let radius = this.dotRadius;
    if (fret === 0) {
      radius -= radius / 4;
    }

    return makeCircle(coord.x, coord.y, radius, color);
  }

  remove() {
    this.svg.remove();
  }

  get numStrings() {
    return this.opts.stringNames.length;
  }

  get numFrets() {
    const offset = this.opts.startFret === 0 ? 0 : 1;
    return this.opts.endFret - this.opts.startFret + offset;
  }

  onClick(event) {
    const coord = this.closestFretCoord(event) ;
    this.opts.onClick(coord);
  }

  onMouseMove(event) {
    const coord = this.closestFretCoord(event);
    if (this.coordEqual(coord, this.hoverCoord)) return;
    this.hoverCoord = coord;

    if (this.hoverDot) this.hoverDot.remove();
    const dot = this.makeDot(coord.string, coord.fret, this.opts.hoverDotColor);
    this.hoverDot = dot;

    this.svg.appendChild(dot);
  }

  fretCoord(string, fret) {
    const stringOffset = Math.abs(string - this.numStrings);

    const x = (stringOffset * this.stringMargin) + this.xMargin;
    let y = ((fret * this.fretHeight) - (this.fretHeight / 2)) + this.yMargin;

    // place open string dots closer to the top of the fretboard
    if (fret === 0) {
      y += this.fretHeight / 5;
    }

    return {x, y};
  }

  closestFretCoord(mouseEvent) {
    const point = cursorPoint(this.svg, mouseEvent);
    const x = point.x - this.xMargin;
    const y = point.y - this.yMargin + (this.fretHeight / 2);

    let string = Math.abs(Math.round(x / this.stringMargin) - this.numStrings);
    if (string < 1) {
      string = 1;
    } else if (string > this.numStrings) {
      string = this.numStrings;
    }

    let fret = Math.round(y / this.fretHeight);
    if (fret > this.opts.endFret) {
      fret = this.opts.endFret;
    }

    return {string, fret};
  }

  coordEqual(c1, c2) {
    return c1 && c2 && c1.string === c2.string && c1.fret === c2.fret;
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

function cursorPoint(svgElem, mouseEvent) {
  const point = svgElem.createSVGPoint();
  point.x = mouseEvent.clientX;
  point.y = mouseEvent.clientY;

  const screenCTM = svgElem.getScreenCTM();
  if (!screenCTM) throw new Error(`could not get the screen ctm of ${svgElem}`);

  const matrix = screenCTM.inverse();
  return point.matrixTransform(matrix);
}

// function makeText(x, y, text, fontSize = 16) {
//   const textElem = document.createElementNS(SVG_NS, 'text');
//   textElem.setAttribute('x', x.toString());
//   textElem.setAttribute('y', y.toString());
//   textElem.setAttribute('text-anchor', 'middle');
//   textElem.setAttribute('font-size', fontSize.toString());
//
//   const textNode = document.createTextNode(text);
//   textElem.appendChild(textNode);
//
//   return textElem;
// }