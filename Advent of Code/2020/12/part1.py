import math


def getdata(path):
	data = []

	DIR = 90 # 0 north, 1 east, 2 south, 3 west

	with open(path) as my_file:
		for line in my_file:

			(DIR, i, n) = processInstruction(line, DIR)

			data.append({
				"i" : i,
				"n" : n,
				"l" : line.strip()
			})
	return data

def processInstruction(line, DIR):
	i  = line[0]
	n = int(line[1:])

	match i:
		case "R":
			DIR = (DIR + n)%360
			i = DIR
			n = 0
		case "L":
			DIR = (DIR - n)%360
			i = DIR
			n = 0
		case "F":
			i=DIR
		case "N":
			i=0
		case "E":
			i=90
		case "S":
			i=180
		case "W":
			i=270

	return (DIR, i, n)

def run(instructions):

	x = 0
	y = 0

	#i = 0
	for inst in instructions:
		#ox = x
		#oy = y
		x += math.cos(math.radians(inst['i']))*inst['n']
		y += math.sin(math.radians(inst['i']))*inst['n']
		#print(str(i) + "\t" + inst["l"] + "\t Old ["+str(round(ox)) + "," + str(round(oy)) + "] --> New["+str(round(x)) + "," + str(round(y)) + "] from a move in " + str(inst['i']) + " dir, by " + str(inst['n']))
		#i+=1


	print([x,y,abs(x)+abs(y)])

if __name__ == "__main__":

	data = getdata('instructions.txt')

	run(data)
