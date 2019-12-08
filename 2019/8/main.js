const fs = require("fs");

const BLACK = 0;
const WHITE = 1;
const TRANSP = 2;

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

const main = () => {
	const input = fs.readFileSync("./input", "utf8").trim();

	const img = parse(input, 25, 6);

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

eg1();
main();
