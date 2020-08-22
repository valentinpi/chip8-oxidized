"use strict";

import * as wasm from "c8_ox_www";

// Timing
const DELAY = 1;
// How often the game should be updated in one tick
const REPEAT = 5;
// Keys
const KEY_MAP = {
    48: 0x0,
    49: 0x1,
    50: 0x2,
    51: 0x3,
    52: 0x4,
    53: 0x5,
    54: 0x6,
    55: 0x7,
    56: 0x8,
    57: 0x9,
    65: 0xA,
    66: 0xB,
    67: 0xC,
    68: 0xD,
    69: 0xE,
    70: 0xF,
    97: 0xA,
    98: 0xB,
    99: 0xC,
    100: 0xD,
    101: 0xE,
    102: 0xF,
};
// Paths of all available ROMs
const ROMPATH_PREFIX = "roms/";

// Load rom
let rom = null;

function init() {
    let romPath = prompt("Enter ROM title");

    let xhr = new XMLHttpRequest();
    xhr.open("GET", (ROMPATH_PREFIX + romPath).trim(), true);
    xhr.responseType = "arraybuffer";

    xhr.onload = function(e) {
        if (xhr.status == 200) {
            rom = new Uint8Array(xhr.response);
            start();
        }
        else {
            alert("File not found!");
            init();
        }
    }

    xhr.send();
}

// Implementation specific variables
let schip8 = null;
let canvas = document.getElementById("canvas");
let context = canvas.getContext("2d");
let audio = document.getElementById("audio");
let begin = window.performance.now();
let last_key = 0x10;
let interval = 0;

// Add event listeners
function keydown(e) {
    let key = KEY_MAP[e.key.charCodeAt(0)];
    schip8.set_key(key, true);
    last_key = key;
}

function keyup(e) {
    let key = KEY_MAP[e.key.charCodeAt(0)];
    schip8.set_key(key, false);
}

window.addEventListener("keydown", keydown);
window.addEventListener("keyup", keyup);

function start() {
    schip8 = new wasm.SChip8(rom);
    interval = window.setInterval(function() {
        for (let i = 0; i < 5; i++) {
            run();
        }
        window.requestAnimationFrame(render);
    }, DELAY);
}

// Emulator loop
function run() {
    let con = schip8.run(last_key);
    last_key = 0;
    window.performance.now();

    let end = window.performance.now();
    if (end - begin >= 16) {
        if (schip8.dt > 0) {
            schip8.dt -= 1;
        }
        if (schip8.st > 0) {
            schip8.st -= 1;
            audio.play();
            if (schip8.st == 0) {
                audio.pause();
            }
        }
        begin = window.performance.now();
    }

    if (!con) {
        window.cancelInterval(interval);
    }
}

function render() {
    let pixel_w = canvas.width / schip8.screen_width;
    let pixel_h = canvas.height / schip8.screen_height;

    for (let y = 0; y < schip8.screen_height; y++) {
        for (let x = 0; x < schip8.screen_width; x++) {
            let pixel = schip8.get_pixel(x, y);
    
            if (pixel == 0) {
                context.fillStyle = "black";
            }
            else {
                context.fillStyle = "white";
            }

            context.fillRect(
                x * pixel_w,
                y * pixel_h,
                pixel_w,
                pixel_h
            );
        }
    }
}

init();
