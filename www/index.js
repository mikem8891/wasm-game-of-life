// @ts-check

import initWasm, {Universe, Cell} from "../pkg/wasm_game_of_life.js";

const initWasmPromise = initWasm();

const CELL_SIZE = 6; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
/** @type {HTMLCanvasElement} */
const canvas = document.getElementById("game-of-life-canvas");
/** @type {HTMLElement} */
const playPauseButton = document.getElementById("play-pause");
/** @type {HTMLInputElement} */
const speedSlider = document.getElementById("speed-slider");
let speed;
setSpeed();
/** @type {HTMLElement} */
const randButton = document.getElementById("randomize");
/** @type {HTMLElement} */
const stepButton = document.getElementById("step");
/** @type {HTMLElement} */
const clearButton = document.getElementById("clear");

const memory = (await initWasmPromise).memory;

// Construct the universe, and get its width and height.
const width  = 128;
const height = 80;
const universe = Universe.new(width, height);

// Give the canvas room for all of our cells and a 1px border
// around each of them.
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
/** @type {CanvasRenderingContext2D} */
const ctx = canvas.getContext('2d');

let animationId = null;

playPauseButton.addEventListener("click", togglePlayPauseButton);
canvas.addEventListener("click", clickCanvas);
speedSlider.addEventListener("change", setSpeed);
randButton.addEventListener("click", () => {
  universe.randomize();
  drawUniverse();
});
stepButton.addEventListener("click", () => {
  pause();
  universe.tick();
  drawUniverse();
});
clearButton.addEventListener("click", () => {
  universe.clear();
  drawUniverse();
});


drawUniverse();
pause();

function renderLoop() {
//  debugger;

  for (let i = 0; i < speed; i++){
    universe.tick();
  }  
  drawUniverse();

  animationId = requestAnimationFrame(renderLoop);
}

function drawGrid() {

  // Clear canvas
  ctx.fillStyle = DEAD_COLOR;
  ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);

  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
}

function drawCells() {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();
  ctx.fillStyle = ALIVE_COLOR;

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      if (cells[idx] === Cell.Alive) {
        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }
  }

  ctx.stroke();
}

function drawUniverse() {
  drawGrid();
  drawCells();
}

/**
 * @param {number} row
 * @param {number} column
 */
function getIndex(row, column) {
  return row * width + column;
};

function play() {
  playPauseButton.textContent = "⏸";
  playPauseButton.setAttribute("title", "pause");
  renderLoop();
}

function pause() {
  playPauseButton.textContent = "▶";
  playPauseButton.setAttribute("title", "play");
  cancelAnimationFrame(animationId);
  animationId = null;
}

function isPaused() {
  return animationId === null;
};

function togglePlayPauseButton() {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
}

/**
 * @param {MouseEvent} event
 */
function clickCanvas(event) {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width  / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop  / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width  - 1);

  if (event.ctrlKey) {
    universe.toggle_cell(row, col);
  } else if (event.shiftKey) {
    universe.insert_pulsar_at(row, col);
  }else {
  universe.insert_glider_at(row, col);
}

  drawUniverse();
}

function setSpeed() {
  speed = Number.parseInt(speedSlider.value);
}