use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub struct SnakeCell(usize);

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

    fn update_cell(&mut self, snake_cell: SnakeCell, index_cell: usize) {
        self.position[index_cell] = snake_cell;
    }

    pub fn update_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn snake_head_pos(&self) -> usize {
        self.position[0].0
    }

    pub fn step(&mut self, world_size: usize) {
        let current_positions = self.position.clone();
        let mut last_cell_position: Option<SnakeCell> = None;  

        for (idx, cell) in current_positions.iter().enumerate() {
            let next_cell = if let Some(value) = last_cell_position {
                self.generate_next_cell(value.0, world_size)
            } else {
                self.generate_next_cell(cell.0, world_size)
            };
            
            self.update_cell(next_cell, idx);
            last_cell_position = Some(*cell);
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
}

#[wasm_bindgen]
impl World {
    pub fn new(world_size: usize, initial_idx: usize, snake_size: usize) -> Self {
        let size = world_size;

        Self {
            size,
            snake: Snake::new(initial_idx, snake_size),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn update_snake_direction(&mut self, direction: Direction) {
        self.snake.update_direction(direction);
    }

    pub fn update_step(&mut self) {
        self.snake.step(self.size);
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
