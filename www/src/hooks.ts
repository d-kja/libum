import { Direction } from "libum";

export const keyboardControls = (key: string): Direction => {
	switch (key) {
		case "ArrowRight":
			return Direction.RIGHT;

		case "ArrowLeft":
			return Direction.LEFT;

		case "ArrowUp":
			return Direction.UP;

		case "ArrowDown":
			return Direction.DOWN;

		case "d":
			return Direction.RIGHT;

		case "a":
			return Direction.LEFT;

		case "w":
			return Direction.UP;

		case "s":
			return Direction.DOWN;

		default:
			return Direction.RIGHT;
	}
};
