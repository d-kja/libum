const RENDER_TIMEOUT = 250;

const CELL_SIZE = 75;
const WORLD_SIZE = 10;
const SNAKE_INITIAL_IDX = Date.now() % (WORLD_SIZE * WORLD_SIZE);
const SNAKE_INITIAL_SIZE = 3;

const STROKE_COLOR = '#FFF';
const FILL_COLOR = '#3f3f46';

export {
  RENDER_TIMEOUT,
  CELL_SIZE,
  WORLD_SIZE,
  SNAKE_INITIAL_IDX,
  SNAKE_INITIAL_SIZE,
  STROKE_COLOR,
  FILL_COLOR
}