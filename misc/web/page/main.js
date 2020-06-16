"use strict";

console.log("Module")

import init, { SChip8 } from "../pkg/chip8oxidizedweb.js";

async function run() {
    await init();

    var schip8 = new SChip8();
    console.log(schip8.screen_width);
}

run();
