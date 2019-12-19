const fs = require("fs");

const COLOUR_OFF = "\x1b[0m";
const COLOUR_RED = "\x1b[31m";
const COLOUR_BLUE = "\x1b[34m";
const COLOUR_GREEN = "\x1b[32m";

const assert = (b, s) => {
	if(b)
		console.log(`${COLOUR_GREEN}assertion passed: ${s}${COLOUR_OFF}`);
	else
		console.log(`${COLOUR_RED}assertion failed: ${s}${COLOUR_OFF}`);
	return b;
};
const assertEq = (a, b, s) =>
	assert(a === b, `${s} (${a} === ${b})`);

function Coord({ x, y }) {
	this.x = x;
	this.y = y;
}
Coord.prototype.adj = function(adj) {
	const { x, y } = this;

	const coords = {
		x: x + (adj.x || 0),
		y: y + (adj.y || 0),
	};

	return new Coord(coords);
};
Coord.prototype.objkey = function() {
	return `${this.x}_${this.y}`;
};
Coord.prototype.toString = function() {
	const { x, y } = this;
	return `${x},${y}`;
};
Coord.prototype.equals = function(other) {
	const { x, y } = this;
	return x === other.x
		&& y === other.y;
};

function Grid(grid) {
	this.grid = grid;

	this.height = grid.length;
	this.width = grid[0].length;
}
Grid.prototype.coordOf = function(target, returnNull) {
	for(let y = 0; y < this.grid.length; y++){
		const line = this.grid[y];

		for(let x = 0; x < line.length; x++){
			if(line[x] === target){
				return new Coord({ x, y });
			}
		}
	}

	if(returnNull)
		return null;
	throw `couldn't find ${target}`;
};
Grid.prototype.get = function({ x, y }) {
	if(x < 0 || x >= this.width)
		return null;
	if(y < 0 || y >= this.height)
		return null;
	return this.grid[y][x];
};
Grid.prototype.set = function({ x, y }, to) {
	if(x < 0 || x >= this.width)
		throw "oob";
	if(y < 0 || y >= this.height)
		throw "oob";
	this.grid[y][x] = to;
};
Grid.parse = path => {
	const s = fs.readFileSync(path, "utf8").trim();

	const grid = s.split("\n")
		.map(line => line.split(""));

	return new Grid(grid);
};
Grid.prototype.clone = function() {
	return new Grid(
		[...this.grid.map(line => [...line])],
	);
};
Grid.prototype.keylist = function() {
	const keys = [];
	for(let y = 0; y < this.grid.length; y++){
		const line = this.grid[y];

		for(let x = 0; x < line.length; x++){
			if(isKey(line[x])){
				keys.push(line[x]);
			}
		}
	}
	return keys;
};

const isKey = ch => 'a' <= ch && ch <= 'z';
const isDoor = ch => 'A' <= ch && ch <= 'Z';

const indent = depth => "  ".repeat(depth);

const findAvailableKeys = (map, at, dist) => {
	const todo = [
		{ coord: at, dist },
	];
	const visited = new Set();
	const keys = [];

	//console.log(`starting todo for ${at}`);
	while(todo.length){
		const ent = todo.pop();
		const { coord, dist } = ent;

		const k = coord.objkey();
		if(visited.has(k))
			continue;
		visited.add(k);

		//console.log(`visiting ${coord}`);

		const ch = map.get(coord);
		if(ch === null)
			continue;

		if(isKey(ch)){
			keys.push({ key: ch, dist, at: coord });
			// FIXME: continue early?
			//continue;
		}else if(isDoor(ch) || ch === "#"){
			continue;
		}else if(ch === "." || ch === "@"){
			// ok
		}else{
			throw `unknown ent ${ch}`;
		}

		const nexts = [
			coord.adj({ x:  1, y:  0 }),
			coord.adj({ x: -1, y:  0 }),
			coord.adj({ x:  0, y:  1 }),
			coord.adj({ x:  0, y: -1 }),
		];
		todo.push(...nexts.map(coord => ({ coord, dist: dist + 1 })));
	}
	//console.log(`done todo for ${at}`);

	return keys;
};

const walk = (map, at, dist, seen /* shared for all? */) => {
	const availableKeys = findAvailableKeys(map, at, dist);

	if(availableKeys.length === 0)
		return dist;

	//console.log(`availableKeys from ${at}`, availableKeys);
	const sortedKeys = availableKeys.map(({ key }) => key).sort();
	const seenKey = `${at};${dist};${sortedKeys.join(",")}`;
	const seenGot = seen.get(seenKey);
	if(seenGot && seenGot.certain){
		return seenGot.dist;
	}

	const subdistances = availableKeys
		.map(({
			key,
			dist: keyDist,
			at: keyPos
		}) => {
			const map2 = map.clone();

			const door = key.toUpperCase();
			const doorPos = map.coordOf(door, true);

			if(doorPos !== null){
				map2.set(doorPos, ".");
			}

			map2.set(keyPos, ".");

			//const rem = map.keylist().join(",");
			//console.log(`recursive walk, from ${keyPos} with remaining keys: ${rem}`);

			const subwalk = walk(map2, keyPos, keyDist, seen);

			return subwalk;
		});

	//console.log("subdistances", subdistances);

	const least = subdistances.reduce((least, candidate) => {
		if(!least)
			return candidate;
		if(candidate < least)
			return candidate;
		return least;
	}, null);

	if(seen.has(seenKey)){
		const { dist: was } = seen.get(seenKey);
		if(was === least){
			seen.set(seenKey, {
				dist: least,
				certain: true,
			});
			//console.log(`overriding seen[${seenKey}] - was ${was}, now ${least}`);
		}
	} else {
		seen.set(seenKey, {
			dist: least,
			certain: false,
		});
	}

	return least;
};

const smallestRoute = map => {
	const me = map.coordOf("@");
	const seen = new Map();

	return walk(map, me, 0, seen);
};

const assertEg = (path, expected, desc) => {
	const dist = smallestRoute(Grid.parse(path));

	if(!assertEq(dist, expected, desc)){
		//console.log(`  map: ${map.join(" => ")}`);
	}
};

const checks = () => {
	const map = Grid.parse("./eg2");
	const map2 = map.clone();
	const c = new Coord({ y: 1, x: 5 });

	const was = map.get(c);
	map.set(c, "!");

	assertEq(map2.get(c), was, "map change");
};

const test = () => {
	checks();
	assertEg("./eg2", 86, "test eg2");
	assertEg("./eg3", 132, "test eg3");
	assertEg("./eg4", 136, "test eg4");
	assertEg("./eg5", 81, "test eg5");
};

const part1 = () => {
	//const map = parse("./input");
	//const map = parse("./simple");
	const map = Grid.parse("./testout.txt");

	//console.log("parsed:");
	//map.show();

	const min = smallestRoute(map);

	console.log(`shortest path, ${min}`);

	//console.log(`depth reached: ${maxDepth}`);
	//logPaths(pathTree, map);
};

test();
//part1();
