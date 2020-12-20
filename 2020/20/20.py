import sys

with open(sys.argv[1], 'r') as f:
    tiles_raw = f.read().strip().split('\n\n')

tiles = {}

for raw_tile in tiles_raw:
    lines = raw_tile.strip('\n').split('\n')
    idn = int(lines[0].split()[1].strip(':'))
    grid = lines[1:]
    sides = [
			grid[0],
			grid[-1],
			''.join([g[0] for g in grid]),
			''.join([g[-1] for g in grid])
		]
    sides += [s[::-1] for s in sides]
    tiles[idn] = {"grid": grid, "sides": sides, 'neighbors': {}}

    for i, tile in tiles.items():
        if i == idn:
            continue
        shared = [s for s in tile['sides'] if s in sides]
        for s in shared:
            tiles[idn]['neighbors'][i] = s
            tiles[i]['neighbors'][idn] = s

# With that information, finding the corners is just finding the tiles with only two neighbors:

corners = list(map(int, [t for t in tiles if len(tiles[t]['neighbors']) == 2]))
res = 1
for c in corners:
    res *= c

print(f'Part 1: {res}')
