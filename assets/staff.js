const VF = Vex.Flow;

export class Staff {
  constructor(parentEl, width, height) {
    this.renderer = new VF.Renderer(parentEl, VF.Renderer.Backends.SVG);
    this.renderer.resize(width, height);

    this.context = this.renderer.getContext();

    const stave = new VF.Stave(0, 0, width-1)
      .addClef("treble")
      .setContext(this.context);

    const note = new VF.StaveNote({
      keys:["E/3"],
      duration: "w",
    });

    note.setStave(stave);

    const tc = new VF.TickContext();
    note.setTickContext(tc);
    tc.setX(width / 4);

    this.noteGroup = this.context.openGroup();
    note.draw();
    this.context.closeGroup();

    stave.draw();
  }

  clear() {
    if (this.noteGroup) {
      this.context.svg.removeChild(this.noteGroup);
      this.noteGroup = null;
    }
  }
}
