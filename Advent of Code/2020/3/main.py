def getdata(path):
	data = []
	with open(path) as my_file:
		for line in my_file:
			data.append(line[0:-1])
	return data


def isEmpty(data, x, y):
	return data[x][y] == '.'

def traverse(data, x, y, trees, dx,dy):

	#print("["+str(x) + ":" + str(y) +"] "+str(trees))

	x+=dx
	y= (y + dy) % len(data[0])

	if x >= len(data):
		return trees

	return traverse(data, x, y, trees +  (0 if isEmpty(data, x, y) else 1), dx, dy)

if __name__ == "__main__":

	data = getdata('map.txt')

	res = traverse(data, 0, 0, 0, 1, 3)
	print(str(res) + " trees on the route, part 1")

	routes = [
		[1,1],
		[3,1],
		[5,1],
		[7,1],
		[1,2]
	]

	total = 1
	for route in routes	:
		total *= traverse(data, 0, 0, 0, route[1], route[0])

	print(str(total) + " trees on the route, part 2")

