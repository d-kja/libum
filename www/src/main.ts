import * as config from "./static-config";
import "./style.css";
import init, { World } from "snake_game";
import { keyboardControls } from "./hook"

await init();

const app = document.querySelector<HTMLDivElement>("#app") as HTMLDivElement;
app.innerHTML = `
  <span id="extra"></span>
  <canvas id="container"></canvas>
`;

const container = document.getElementById("container") as HTMLCanvasElement;
const extra = document.getElementById("extra") as HTMLCanvasElement;

const world = World.new(config.WORLD_SIZE, config.SNAKE_INITIAL_IDX, config.SNAKE_INITIAL_SIZE);

const context = container.getContext("2d") as CanvasRenderingContext2D;
const WORLD_SIZE = world.size();

container.height = WORLD_SIZE * config.CELL_SIZE;
container.width = WORLD_SIZE * config.CELL_SIZE;
context.strokeStyle = config.STROKE_COLOR;
context.fillStyle = config.FILL_COLOR;

const clearCanvas = () =>
	context.clearRect(0, 0, container.width, container.height);

const updateHeader = () => {
	extra.innerHTML = `<b>Snake game</b> | current index: ${world
		.snake_head_pos()
		.toString()
		.padStart(2, "0")}`;
}

const drawWorld = () => {
	context.beginPath();

	// Create grid columns
	for (let index = 0; index <= WORLD_SIZE; index++) {
		const posX = config.CELL_SIZE * index;
		const posY = config.CELL_SIZE * WORLD_SIZE;

		context.moveTo(posX, 0);
		context.lineTo(posX, posY);
	}

	// Create grid rows
	for (let index = 0; index <= WORLD_SIZE; index++) {
		context.moveTo(0, config.CELL_SIZE * index);
		context.lineTo(config.CELL_SIZE * WORLD_SIZE, config.CELL_SIZE * index);
	}

	context.stroke();
};

const drawSnake = () => {
	const snakeHeadIndex = world.snake_head_pos();

	const column = snakeHeadIndex % WORLD_SIZE;
	const row =
		Math.floor(
			// -> 0 / 8 = 0, 7 / 8 = 0 (still in the first row)... 8 / 8 = 1 (second row)
			snakeHeadIndex / WORLD_SIZE,
		);

	context.beginPath();

	const snake_x = config.CELL_SIZE * column;
	const snake_y = config.CELL_SIZE * row;

	context.fillRect(snake_x, snake_y, config.CELL_SIZE, config.CELL_SIZE);

	context.stroke();
};

const render = () => {
	updateHeader();
	clearCanvas();

	drawWorld();
	drawSnake();

	world.update();
};

addEventListener('keydown', (ctx) => {
	const direction = keyboardControls(ctx.key)
	world.update_snake_direction(direction);
})

setInterval(render, config.RENDER_TIMEOUT);
