export interface FluidPointerState {
	x: number;
	y: number;
	dx: number;
	dy: number;
	moved: boolean;
	initialized: boolean;
}

interface Vec2Like {
	set(x: number, y: number): unknown;
}

interface UpdateFluidPointerOptions {
	state: FluidPointerState;
	uv: Vec2Like;
	x: number;
	y: number;
	width: number;
	height: number;
	forceClamp: number;
	initialLerp: number;
	lerp: number;
	forceScale?: number;
}

const clamp = (value: number, min: number, max: number) =>
	Math.min(max, Math.max(min, value));

const mix = (a: number, b: number, t: number) => a + (b - a) * t;

export const updateFluidPointerState = ({
	state,
	uv,
	x,
	y,
	width,
	height,
	forceClamp,
	initialLerp,
	lerp,
	forceScale = 5,
}: UpdateFluidPointerOptions): void => {
	const prevX = state.x;
	const prevY = state.y;
	const targetDx = clamp(forceScale * (x - prevX), -forceClamp, forceClamp);
	const targetDy = clamp(forceScale * (y - prevY), -forceClamp, forceClamp);
	const lerpFactor = state.initialized ? lerp : initialLerp;

	state.moved = true;
	state.dx = mix(state.dx, targetDx, lerpFactor);
	state.dy = mix(state.dy, targetDy, lerpFactor);
	state.x = x;
	state.y = y;
	state.initialized = true;

	if (width > 0 && height > 0) {
		uv.set(x / width, 1 - y / height);
	}
};
