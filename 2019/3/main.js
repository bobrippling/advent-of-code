// The gravity assist was successful, and you're well on your way to the Venus refuelling station. During the rush back on Earth, the fuel management system wasn't completely installed, so that's next on the priority list.
//
// Opening the front panel reveals a jumble of wires. Specifically, two wires are connected to a central port and extend outward on a grid. You trace the path each wire takes as it leaves the central port, one wire per line of text (your puzzle input).
//
// The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, you need to find the intersection point closest to the central port. Because the wires are on a grid, use the Manhattan distance for this measurement. While the wires do technically cross right at the central port where they both start, this point does not count, nor does a wire count as crossing with itself.
//
// For example, if the first wire's path is R8,U5,L5,D3, then starting from the central port (o), it goes right 8, up 5, left 5, and finally down 3:
//
// ...........
// ...........
// ...........
// ....+----+.
// ....|....|.
// ....|....|.
// ....|....|.
// .........|.
// .o-------+.
// ...........
//
// Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down 4, and left 4:
//
// ...........
// .+-----+...
// .|.....|...
// .|..+--X-+.
// .|..|..|.|.
// .|.-X--+.|.
// .|..|....|.
// .|.......|.
// .o-------+.
// ...........
//
// These wires cross at two locations (marked X), but the lower-left one is closer to the central port: its distance is 3 + 3 = 6.
//
// Here are a few more examples:
//
//     R75,D30,R83,U83,L12,D49,R71,U7,L72
//     U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
//     R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
//
// What is the Manhattan distance from the central port to the closest intersection?

const fs = require("fs");

const SPACE = ".";
const ORIGIN = "o";
const SIZE = 5000;
let grid;

const origin = { x: SIZE / 2, y: SIZE / 2 };

const init = () => {
	grid = {};
	/*grid = [...Array(SIZE)]
		.map(x => [...Array(SIZE)].map(_ => SPACE));*/

	set(origin, ORIGIN);
};

const set = ({x, y}, v) => {
	if(!grid[x]){
		grid[x] = {};
	}
	grid[x][y] = v;
};

const get = ({x, y}) => {
	if(grid[x]){
		return grid[x][y];
	}
	return SPACE;
};

const showGrid = () => {
	let out = "";
	for(let y = 0; y < SIZE; y++){
		for(let x = 0; x < SIZE; x++)
			out += get({ x, y });

		out += "\n";
	}
	console.log(out);
};

const die = str => {
	console.error(str);
	process.exit(1);
};

const walk = (wire, mark, overlap) => {
	const pos = { ...origin };
	for(const entry of wire){
		const match = /([UDLR])(\d+)/.exec(entry);
		if(!match)
			die(`couldn't match against "${entry}"`);

		let [, dir, count] = match;
		let first = true;

		for(; count; first = false, count--){
			switch(dir){
				case 'U': pos.y -= 1; break;
				case 'D': pos.y += 1; break;
				case 'L': pos.x -= 1; break;
				case 'R': pos.x += 1; break;
			}

			const at = get(pos);
			if(overlap && at != SPACE)
				overlap(pos, at);

			set(pos, mark);
		}
	}
};

const manhat = (a, b) => {
	const dx = Math.abs(a.x - b.x);
	const dy = Math.abs(a.y - b.y);
	return dx + dy;
};

const closestDistance = (wires) => {
	const crosses = [];

	walk(wires[0], 'a');
	//showGrid();
	walk(wires[1], 'b', (pos, val) => {
		if(val != 'a')
			return;
		crosses.push({ ...pos });
	});
	//showGrid();

	let mindist = Infinity;
	let min;

	for(const x of crosses){
		const dist = manhat(x, origin);

		/*
		console.log("overlap @ ",
			x,
			"distance from",
			origin,
			":",
			dist);
		*/

		if(dist < mindist){
			mindist = dist;
			min = x;
		}
	}

	return { dist: mindist, point: min };
};

const expectDistance = (wires, expected) => {
	init();

	const { dist, point } = closestDistance(wires);

	if(dist != expected){
		console.error(`got ${dist}, expected ${expected}`);
	}
};

const lines = path => fs.readFileSync(path, "utf-8")
	.split("\n");

const eg0 = () => {
	const wire1 = "R8,U5,L5,D3".split(",");
	const wire2 = "U7,R6,D4,L4".split(",");

	expectDistance([wire1, wire2], 6);
};

const eg1 = () => {
	const wire1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72".split(",");
	const wire2 = "U62,R66,U55,R34,D71,R55,D58,R83".split(",");

	expectDistance([wire1, wire2], 159);
};

const eg2 = () => {
	const wire1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".split(",");
	const wire2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".split(",");

	expectDistance([wire1, wire2], 135);
};

//eg0();
//eg1();
//eg2();

init();

const wires = lines("./input")
	.filter(x => x)
	.map(s => s.split(","));

const { dist, point } = closestDistance(wires);

console.log(dist);
