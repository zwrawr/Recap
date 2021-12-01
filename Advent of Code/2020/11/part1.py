def getdata(path):
	data = []
	with open(path) as my_file:
		for line in my_file:
			data.append(list(line.strip()))
	return data
	
def adjecents(data,x,y):
	count = 0
	for dx in range(-1,2):
		tx = x+dx
		if tx >= 0 and tx < len(data):
			for dy in range(-1,2):
				ty = y+dy
				#print((x,y,tx,ty))
				if ty >= 0 and ty < len(data[tx]):
					if not (ty == y and tx == x):
						count += 1 if data[tx][ty] == '#' else 0
	return count

def process(data):

	changes = []

	for x in range(0,len(data)):
		for y in range(0,len(data[x])):
			if not data[x][y] == '.':
				adj = adjecents(data,x,y)

				if data[x][y] == 'L' and adj == 0:
					changes.append([x,y])
				if data[x][y] == '#' and adj >= 4:
					changes.append([x,y])
	return changes

def mutate(data, changes):
	for change in changes:
		if data[change[0]][change[1]] == "#":
			data[change[0]][change[1]] = "L"
		else:
			data[change[0]][change[1]] = "#"

def run(data):
	runs = 0
	changes = process(data)
	while len(changes) > 0 :
		mutate(data,changes)

		#for d in data:
		#	print("".join(d))
		#print("\n\n")
		runs += 1
		changes = process(data)

	return runs

def countOccupied(data):
	o = 0
	for x in range(0,len(data)):
		for y in range(0,len(data[x])):
			if data[x][y] == '#':
				o += 1
	return o

if __name__ == "__main__":

	data = getdata('map.txt')

	#print(adjecents(data,0,1))

	changes = run(data)
	print(changes)

	occ = countOccupied(data)
	print(occ)