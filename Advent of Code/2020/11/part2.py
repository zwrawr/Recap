def getdata(path):
	data = []
	with open(path) as my_file:
		for line in my_file:
			data.append(list(line.strip()))
	return data
	
def adjecents(data,x,y):

	count = 0
	dirs = [
		[-1,1], # upright
		[-1,-1], #upleft
		[-1,0], # up
		[0,-1], #right
		[0,1], # left
		[1,0], # down
		[1,1], # downright
		[1,-1], # downleft
	]

	for dir in dirs:

		##offset current pos based on dir to check
		dx = x + dir[0]
		dy = y + dir[1]

		while (dx >= 0 and dx < len(data)) and (dy >=0 and dy < len(data[0])):
			#print([x,y,dx,dy,dir[0],dir[1]])
			if data[dx][dy] == '#':
				count += 1
				break
			if data[dx][dy] == 'L':
				break

			#if we don't find a chair keep chcking in that direction
			dx += dir[0]
			dy += dir[1]

	return count



def process(data):
	#	+-----> y
	#	|
	#	|
	#	v
	#	x

	changes = []

	for x in range(0,len(data)):
		for y in range(0,len(data[x])):
			if not data[x][y] == '.':
				#if (x < 3 and y < 3):
					#print([x,y])
				adj = adjecents(data,x,y)
				#f (x < 3 and y < 3):
					#print("  len(x) " + str(len(data)))
					#print("  len(y) " + str(len(data[0])))
					#print("  adjacent " + str(adj))
				if data[x][y] == 'L' and adj == 0:
					changes.append([x,y])
					#if (x < 3 and y < 3):
						#print("  changes L -> #")
				if data[x][y] == '#' and adj >= 5:
					changes.append([x,y])
					#if (x < 3 and y < 3):
						#print("  changes # -> L")


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
		print("run [" + str(runs) + "]")
		for d in data:
			print("".join(d))
		print("\n\n")
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

	#print(adjecents(data,0,0))

	changes = run(data)
	print(changes)

	occ = countOccupied(data)
	print(occ)