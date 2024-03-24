import * as config from "./static-config";
import "./style.css";
import init, { World } from "libum";
import { keyboardControls } from "./hooks";

const wasm = await init();

const app = document.querySelector<HTMLDivElement>("#app") as HTMLDivElement;
app.innerHTML = `
  <canvas id="container"></canvas>
`;

const container = document.getElementById("container") as HTMLCanvasElement;

const world = World.new(
	config.WORLD_SIZE,
	config.SNAKE_INITIAL_IDX,
	config.SNAKE_INITIAL_SIZE,
);

const context = container.getContext("2d") as CanvasRenderingContext2D;

const WORLD_SIZE = world.size();
const REWARD_CELL = world.reward_cell();
const SNAKE_POINTER = world.get_snake_ptr();
const SNAKE_LENGTH = world.get_snake_length();

container.height = WORLD_SIZE * config.CELL_SIZE;
container.width = WORLD_SIZE * config.CELL_SIZE;
context.strokeStyle = config.STROKE_COLOR;

const clearCanvas = () =>
	context.clearRect(0, 0, container.width, container.height);

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

const drawRewardCell = () => {
	if (!REWARD_CELL) return

	const row = Math.floor(REWARD_CELL / WORLD_SIZE);
	const column = REWARD_CELL % WORLD_SIZE;

	const padding = config.CELL_SIZE * 0.35;

	const rewardSize = config.CELL_SIZE - padding * 2;
	const rewardX = column * config.CELL_SIZE + padding;
	const rewardY = row * config.CELL_SIZE + padding;

	context.beginPath();

	context.fillStyle = "#a5b4fc";
	context.fillRect(
		rewardX - rewardSize * 0.15,
		rewardY - rewardSize * 0.15,
		rewardSize + rewardSize * 0.15 * 2,
		rewardSize + rewardSize * 0.15 * 2,
	);

	context.fillStyle = config.FILL_REWARD_COLOR;
	context.fillRect(rewardX, rewardY, rewardSize, rewardSize);

	context.stroke();
};

const drawSnake = () => {
	const snakeCells = new Uint32Array(
		wasm.memory.buffer,
		SNAKE_POINTER,
		SNAKE_LENGTH,
	);

	const first_index = snakeCells[0];

	for (const snakeIndex of snakeCells) {
		const column = snakeIndex % WORLD_SIZE;
		const row = Math.floor(
			// -> 0 / 8 = 0, 7 / 8 = 0 (still in the first row)... 8 / 8 = 1 (second row)
			snakeIndex / WORLD_SIZE,
		);

		context.beginPath();

		const snake_x = config.CELL_SIZE * column;
		const snake_y = config.CELL_SIZE * row;

		context.fillStyle = config.FILL_COLOR;
		if (first_index === snakeIndex) context.fillStyle = config.FILL_HEAD_COLOR;

		context.fillRect(snake_x, snake_y, config.CELL_SIZE, config.CELL_SIZE);
	}

	context.stroke();
};

const render = () => {
	clearCanvas();

	drawWorld();
	drawRewardCell();
	drawSnake();

	world.update_step();
};

addEventListener("keydown", (ctx) => {
	const direction = keyboardControls(ctx.key);
	world.update_snake_direction(direction);
});

setInterval(render, config.RENDER_TIMEOUT);
