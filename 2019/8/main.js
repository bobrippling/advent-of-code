const fs = require("fs");

const BLACK = "0";
const WHITE = "1";
const TRANSP = "2";

const ESC_BLACK = "30";
const ESC_WHITE = "37";

const assert = (b, desc) => {
	if(!b){
		const e = new Error("failed assert\n" + (desc || ""));
		throw e;
	}
}

const parse = (str, width, height) => {
	let row = [];
	let rows = [];
	let layers = [];

	for(var i = 0; i < str.length; i++){
		const char = str[i];

		row.push(char);

		if (row.length === width){
			rows.push(row);
			row = [];
		}
		if (rows.length === height){
			layers.push(rows);
			rows = [];
		}
	}

	assert(rows.length == 0);
	assert(row.length == 0);

	return layers;
};

const countdigits = (layer, digit) => {
	let count = 0;
	for(const row of layer)
		for(const char of row)
			if(char === digit)
				count++;
	return count;
};

const resolve_pixel = (img, w, h) => {
	for(const layer of img){
		const pixel = layer[h][w];
		switch(pixel){
			case BLACK:
			case WHITE:
				return pixel;
			case TRANSP:
				break;
			default:
				assert(false, `unknown pixel "${layer[w][h]}"`);
		}
	}
	return BLACK;
};

const resolve = (img, width, height) => {
	const resolved = Array(height).fill(0).map(() => Array(width).fill(TRANSP));

	for(let w = 0; w < width; w++){
		for(let h = 0; h < height; h++){
			resolved[h][w] = resolve_pixel(img, w, h);
		}
	}

	return resolved;
};

const eg1 = () => {
	const img = parse("123456789012", 3, 2);

	assert(countdigits(img[0], "1") == 1);
	assert(countdigits(img[0], "2") == 1);
	assert(countdigits(img[0], "3") == 1);
	assert(countdigits(img[0], "4") == 1);
	assert(countdigits(img[0], "5") == 1);
	assert(countdigits(img[0], "6") == 1);

	assert(countdigits(img[1], "7") == 1);
	assert(countdigits(img[1], "8") == 1);
	assert(countdigits(img[1], "9") == 1);
	assert(countdigits(img[1], "0") == 1);
	assert(countdigits(img[1], "1") == 1);
	assert(countdigits(img[1], "2") == 1);
};

const eg2 = () => {
	const img = parse("0222112222120000", 2, 2);
	const r = resolve(img, 2, 2);

	assert(r[0][0] == BLACK);
	assert(r[0][1] == WHITE);
	assert(r[1][0] == WHITE);
	assert(r[1][1] == BLACK);
};

const part1 = img => {
	//console.log(JSON.stringify(img));//, 0, 2));

	let min = Infinity, minlayer;
	let i = 0;
	for(const layer of img){
		const nzeros = countdigits(layer, "0");

		console.log(`layer[${i}] nzeros = ${nzeros}`);

		if(!min || nzeros < min){
			min = nzeros;
			minlayer = layer;
		}
		i++;
	}

	const nones = countdigits(minlayer, "1");
	const ntwos = countdigits(minlayer, "2");

	console.log({nones, ntwos});

	console.log(nones * ntwos);
};

const part2 = (img, width, height) => {
	const r = resolve(img, width, height);

	for(let h = 0; h < height; h++){
		let s = "";
		for(let w = 0; w < width; w++){
			const c = r[h][w] == BLACK ? ESC_BLACK : ESC_WHITE;

			s += `\x1b[0;${c}mX`
		}
		console.log(s + "\x1b[0;0m");
	}
};

const main = () => {
	const input = fs.readFileSync("./input", "utf8").trim();

	const img = parse(input, 25, 6);

	//part1(img);
	part2(img, 25, 6);
};

eg1();
eg2();
main();
