use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/random.ts")]
extern "C" {
    fn random(max: usize) -> usize;
}

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub struct SnakeCell(usize);

impl PartialEq for SnakeCell {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    WON,
    LOST,
    STARTED,
}

#[wasm_bindgen]
pub struct Snake {
    position: Vec<SnakeCell>,
    direction: Direction,
    reward_cell: Option<usize>,
    status: Option<GameStatus>,
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
            reward_cell: None,
            status: None,
        }
    }

    fn update_cell(&mut self, snake_cell: SnakeCell, index_cell: usize) {
        self.position[index_cell] = snake_cell;
    }

    fn is_opposite_direction(&self, direction_to_compare: &Direction) -> bool {
        let current_direction = self.direction;

        match direction_to_compare {
            Direction::LEFT => {
                if current_direction == Direction::RIGHT {
                    return true;
                }
            }
            Direction::RIGHT => {
                if current_direction == Direction::LEFT {
                    return true;
                }
            }
            Direction::UP => {
                if current_direction == Direction::DOWN {
                    return true;
                }
            }
            Direction::DOWN => {
                if current_direction == Direction::UP {
                    return true;
                }
            }
        };

        false
    }

    pub fn update_direction(&mut self, direction: Direction) {
        let is_opposite_direction = self.is_opposite_direction(&direction);

        if !is_opposite_direction {
            self.direction = direction
        }
    }

    pub fn step(&mut self, world_size: usize) {
        match &self.status {
            Some(GameStatus::STARTED) => {
                let current_positions = self.position.clone();
                let mut last_cell_position: Option<usize> = None;

                for (idx, cell) in current_positions.iter().enumerate() {
                    let next_cell = if let Some(value) = last_cell_position {
                        SnakeCell(value)
                    } else {
                        self.generate_next_cell(cell.0, world_size)
                    };

                    last_cell_position = Some(cell.0);
                    self.update_cell(next_cell, idx);
                }

                let snake_head_idx = self.position[0].0;
                let reward_idx = self.reward_cell.unwrap_or(usize::MAX);

                // Crashed on it's own body
                if self.position[1..self.position.len()].contains(&self.position[0]) {
                    self.status = Some(GameStatus::LOST);
                    return;
                }

                if snake_head_idx == reward_idx {
                    if self.position.len() >= world_size {
                        self.status = Some(GameStatus::WON);
                        return;
                    }

                    self.position.push(SnakeCell(self.position[1].0));
                    self.reward_cell = None;
                }
            }
            _ => return,
        }
    }

    fn generate_next_cell(&self, snake_idx: usize, world_size: usize) -> SnakeCell {
        let world_total_size = world_size * world_size;
        let row = snake_idx / world_size;

        let rows_passed = row * world_size;

        fn is_negative(value: usize, fallback: usize) -> usize {
            match value as isize {
                n if (0..).contains(&n) => value,
                _ => fallback,
            }
        }

        let next_index = match self.direction {
            Direction::RIGHT => {
                let column = (snake_idx + 1) % world_size;
                rows_passed + column
            }
            Direction::LEFT => {
                let column = is_negative(snake_idx - 1, world_size - 1) % world_size;
                rows_passed + column
            }
            Direction::UP => {
                let current_column = snake_idx % world_size;
                let current_index = rows_passed + current_column;

                let next_index = current_index - world_size;
                let fallback_index = (world_size - 1) * world_size + current_column;

                is_negative(next_index, fallback_index)
            }
            Direction::DOWN => {
                let next_row = snake_idx + world_size;
                next_row % world_total_size
            }
        };

        SnakeCell(next_index)
    }
}

#[wasm_bindgen]
pub struct World {
    size: usize,
    snake: Snake,
    points: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(world_size: usize, initial_idx: usize, snake_size: usize) -> Self {
        let size = world_size;
        let mut snake = Snake::new(initial_idx, snake_size);
        let reward_cell = Self::generate_reward_cell(&snake.position, size);

        snake.reward_cell = reward_cell;

        Self {
            size,
            snake,
            points: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn reward_cell(&self) -> Option<usize> {
        self.snake.reward_cell
    }

    fn generate_reward_cell(positions: &Vec<SnakeCell>, size: usize) -> Option<usize> {
        let mut reward_cell;

        loop {
            reward_cell = World::generate_random_cell(&size);

            if !positions.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }

        Some(reward_cell)
    }

    fn generate_random_cell(size: &usize) -> usize {
        let total_size = size * size;

        random(total_size)
    }

    pub fn update_snake_direction(&mut self, direction: Direction) {
        self.snake.update_direction(direction);
    }

    pub fn update_step(&mut self) {
        self.snake.step(self.size);

        if self.snake.reward_cell.is_none() {
            let new_reward_cell = World::generate_reward_cell(&self.snake.position, self.size);
            self.points += 1;

            self.snake.reward_cell = new_reward_cell;
        }
    }

    pub fn change_status(&mut self, status: Option<GameStatus>) {
        self.snake.status = status;
    }

    pub fn get_status(&self) -> Option<GameStatus> {
        self.snake.status
    }

    pub fn get_points(&self) -> usize {
        self.points
    }

    /// Working with raw pointers
    /// *const is similar to &, although it claims
    /// that you're working with an actual pointer
    pub fn get_snake_ptr(&self) -> *const SnakeCell {
        self.snake.position.as_ptr()
    }

    pub fn get_snake_length(&self) -> usize {
        self.snake.position.len()
    }
}
