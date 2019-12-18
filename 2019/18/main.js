const fs = require("fs");

const COLOUR_OFF = "\x1b[0m";
const COLOUR_RED = "\x1b[31m";
const COLOUR_BLUE = "\x1b[34m";
const COLOUR_GREEN = "\x1b[32m";

const DEBUG = false;
const DEPTH_LIMIT = Infinity;
let maxDepth = 0;

const assert = (b, s) => {
	if(b)
		console.log(`assertion passed: ${s}`);
	else
		console.error(`assertion failed: ${s}`);
};
const assertEq = (a, b, s) => {
	assert(a === b, `${s} (${a} === ${b})`);
};

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
/*Coord.prototype.unitdiff = function(other) {
	const { x, y } = this;

	return new Coord({ x, y });
};*/
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

function Map(grid) {
	this.grid = grid;

	this.height = grid.length;
	this.width = grid[0].length;
}
Map.prototype.coordOf = function(target) {
	for(let y = 0; y < this.grid.length; y++){
		const line = this.grid[y];

		for(let x = 0; x < line.length; x++){
			if(line[x] === target){
				return new Coord({ x, y });
			}
		}
	}
	return null;
};
Map.prototype.get = function({ x, y }) {
	if(x < 0 || x >= this.width)
		return null;
	if(y < 0 || y >= this.height)
		return null;
	return this.grid[y][x];
};
Map.prototype.show = function() {
	for(let y = 0; y < this.grid.length; y++){
		const line = this.grid[y];

		console.log(line);
		//for(let x = 0; x < line.length; x++){
		//}
	}
};
Map.prototype.keyList = function() {
	const keys = [];
	for(let y = 0; y < this.grid.length; y++){
		const line = this.grid[y];

		for(const ch of line){
			if(isKey(ch))
				keys.push(ch);
		}
	}
	return keys;
}

function PathTree(from, parent = null) {
	this.coord = from;
	this.parent = parent;
	this.depth = parent ? parent.depth + 1 : 0;

	if(this.depth > maxDepth)
		maxDepth = this.depth;

	this.nexts = {};
	this.keys = parent ? [...parent.keys] : [];
}
PathTree.prototype.visited = function(coord) {
	const k = coord.objkey();

	if(this.nexts[k])
		return true;

	return this.parent && this.parent.visited(coord);
};
PathTree.prototype.add = function(next) {
	const leaf = new PathTree(next, this);

	const k = next.objkey();
	if(this.nexts[k])
		throw new Error("already visited");

	this.nexts[k] = leaf;

	return leaf;
};
PathTree.prototype.del = function(next) {
	const k = next.objkey();
	delete this.nexts[k];
};
PathTree.prototype.children = function() {
	return Object.values(this.nexts);
};
PathTree.prototype.haveKey = function(needle) {
	return this.keys.indexOf(needle) >= 0;
};
PathTree.prototype.addKey = function(key) {
	this.keys.push(key);
};
PathTree.prototype.buildPath = function() {
	const path = [];

	for(let i = this; i; i = i.parent){
		path.push(i.coord);
	}

	return path.reverse();
};

const parse = path => {
	const s = fs.readFileSync(path, "utf8").trim();

	const grid =  s.split("\n");

	return new Map(grid);
};

const isKey = ch => 'a' <= ch && ch <= 'z';
const isDoor = ch => 'A' <= ch && ch <= 'Z';

const findDirs = (map, at, from, pathTree, pickedUpKey) => {
	// returns [{ coord, gotKey: true | undefined }, ...]
	const directions = [
		at.adj({ y: -1 }),
		at.adj({ y:  1 }),
		at.adj({ x: -1 }),
		at.adj({ x:  1 }),
	];

	return directions
		.map(coord => {
			const ent = map.get(coord);
			const justComeFromThere = !pickedUpKey && from && coord.equals(from);

			switch(ent){
				case null:
				case '#':
					return null;
				case '.':
				case '@': {
					if(justComeFromThere)
						return null;

					//console.log(`at ${at}, from: ${from}, accepted candidate: ${coord}`);

					return { coord }; // ok
				}
				default:
					if(isKey(ent)){
						const haveKey = pathTree.haveKey(ent);

						if(justComeFromThere && haveKey)
							return null;

						if(haveKey){
							// don't say we've got it again
							return { coord };
						}

						return {
							coord,
							gotKey: ent,
						};
					}

					if(isDoor(ent)){
						if(!justComeFromThere
						&& pathTree.haveKey(ent.toLowerCase())
						){
							return { coord };
						}
						return null;
					}

					throw new Error(`unexpected grid entry at ${coord}: '${ent}'`);
			}
		})
		.filter(x => x);
};

const walk = (map, at, from, pathTree, pickedUpKey, allKeys, found) => {
	if(pathTree.depth > DEPTH_LIMIT){
		if(DEBUG)
			console.log(`${indent(pathTree.depth)}reached DEPTH_LIMIT`);
		return;
	}

	const coords = findDirs(map, at, from, pathTree, pickedUpKey);

	if(coords.length < 1){
		if(DEBUG)
			console.log(`${indent(pathTree.depth)}${COLOUR_RED}no options from ${at}${COLOUR_OFF}`);
		return;
		//throw "should have at least one direction";
	}

	if(DEBUG)
		console.log(`${indent(pathTree.depth)}at ${at} (from ${from})`);

	if(DEBUG&&0){
		//console.log("coords", coords);
		if(from
		&& coords.length === 1
		&& coords[0].coord.equals(from))
		{
			throw "shouldn't get here";
			// nowhere to go
			console.log(`${indent(pathTree.depth)}nowhere to go`);
			return;
		}
	}

	/*
	// if we only have two directions (i.e. straight line),
	// keep moving - unless we got a key
	if(from
	&& coords.length === 2
	&& !coords.some(({ gotKey }) => gotKey)
	){
		const to = from.equals(coords[0].coord)
			? coords[1].coord
			: coords[0].coord;
		if(to.equals(from))
			throw "bad direction";

		console.log(`${indent(pathTree.depth)}single dir, towards ${to}`);

		const leaf = pathTree.add(to);
		walk(map, to, at, leaf);
		return;
	}
	*/

	const colour = coords.length > 1 ? COLOUR_BLUE : "";

	if(DEBUG)
		console.log(
			`${indent(pathTree.depth)}${colour}options:`,
			coords.map(({ coord }) => coord).join(", "),
			COLOUR_OFF);

	for(const { coord: newCoord, gotKey } of coords){
		/*if(pathTree.visited(newCoord))
			continue;*/
		const leaf = pathTree.add(newCoord);

		if(gotKey){
			if(DEBUG)
				console.log(`${indent(pathTree.depth+1)}got key '${gotKey}' at ${newCoord}`);
			leaf.addKey(gotKey);

			if(allKeys.every(k => leaf.haveKey(k))){
				//console.log(`${indent(pathTree.depth)}FOUND ALL KEYS, path:`);
				//console.log(leaf.buildPath().join(" -> "));
				found(leaf.buildPath());
				return;
			}
		}

		walk(map, newCoord, at, leaf, !!gotKey, allKeys, found);

		pathTree.del(newCoord);
		//console.log(`${indent(pathTree.depth+1)}done with option ${newCoord}`);
	}
};

const indent = depth => "  ".repeat(depth);

const logPaths = (pathTree, map, depth = 0) => {
	const pre = indent(depth);
	const c = pathTree.coord;

	console.log(`${pre}${c} ${map.get(c)}`);
	for(const next of pathTree.children()){
		logPaths(next, map, depth + 1);
	}
};

const smallestRoute = map => {
	const me = map.coordOf("@");
	if(!me)
		throw "couldn't find me";

	const allKeys = map.keyList();

	const pathTree = new PathTree(me);

	const paths = [];

	walk(map, me, null, pathTree, false, allKeys, path => {
		paths.push(path);
	});

	const min = paths.reduce((smallest, path) => {
		if(!smallest)
			return path;

		return path.length < smallest.length ? path : smallest;
	}, null);

	return min;
};

const test = () => {
	assertEq(smallestRoute(parse("./eg2")).length-1, 86, "test eg2");
	assertEq(smallestRoute(parse("./eg3")).length-1, 132, "test eg3");
	assertEq(smallestRoute(parse("./eg4")).length-1, 136, "test eg4");
	assertEq(smallestRoute(parse("./eg5")).length-1, 81, "test eg5");
};

const part1 = () => {
	//const map = parse("./input");
	//const map = parse("./simple");
	const map = parse("./input");

	//console.log("parsed:");
	//map.show();

	const min = smallestRoute(map);

	console.log(`shortest path, ${min.length-1} steps:`, min);

	//console.log(`depth reached: ${maxDepth}`);
	//logPaths(pathTree, map);
};

test();
//part1();
