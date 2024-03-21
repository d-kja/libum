use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Copy)]
struct SnakeCell(usize);

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[wasm_bindgen]
pub struct Snake {
    position: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(snake_initial_pos: usize, snake_size: usize) -> Self {
        let mut snake_position = vec![];

        for index in 0..snake_size {
            snake_position.push(SnakeCell(snake_initial_pos - index));
        }

        Self {
            position: snake_position,
            direction: Direction::RIGHT,
        }
    }

    fn update_head(&mut self, idx: usize) {
        self.position[0].0 = idx;
    }

    pub fn update_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}

#[wasm_bindgen]
pub struct World {
    size: usize,
    total_size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(world_size: usize, initial_idx: usize, snake_size: usize) -> Self {
        let size = world_size;

        Self {
            size,
            total_size: size * size,
            snake: Snake::new(initial_idx, snake_size),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn snake_head_pos(&self) -> usize {
        self.snake.position[0].0
    }

    pub fn update(&mut self) {
        let snake_idx: usize = self.snake_head_pos();
        let row = snake_idx / self.size;

        fn is_negative(value: usize, fallback: usize) -> usize {
            match value as isize {
                n if (0..).contains(&n) => {
                    value
                },
                _ => fallback
            }
        }

        match self.snake.direction {
            Direction::RIGHT => {
                let next_column = (snake_idx + 1) % self.size;
                self.snake.update_head((row * self.size) + next_column);
            }
            Direction::LEFT => {
                let next_column = is_negative(snake_idx - 1, self.size - 1) % self.size;
                self.snake.update_head((row * self.size) + next_column);
            }
            Direction::UP => {
                let column = snake_idx % self.size;
                let next_row = is_negative(((row * self.size) + column) - self.size, (self.size - 1) * self.size + column);
                
                self.snake.update_head(next_row);
            }
            Direction::DOWN => {
                self.snake.update_head((snake_idx + self.size) % self.total_size);
            }
        };
    }

    pub fn update_snake_direction(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }
}
