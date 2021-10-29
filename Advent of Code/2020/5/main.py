
def parseSeatID(seatid):
	rowid = seatid[0:-3]
	colid = seatid[-3:]

	row = idToNum(rowid,"B")
	col = idToNum(colid,"R")

	return (row,col)

def idToNum(id,o):
	n = 0
	for i in id:
		n = n << 1
		if i == o:
			n+=1

	return n

def seatNumber(col, row):
	return row + 8 * col

def getdata(path):
	data = []
	with open(path) as my_file:
		for line in my_file:
			data.append(line.strip())
	return data

def getMaxSeatNum(data):

	max = 0
	for item in data:
		seat = parseSeatID(item)
		seatid = seatNumber(seat[0],seat[1])
		if seatid > max:
			print(seat)
			max = seatid
	return max

def getMissingNum(data):
	max = 0
	min = float('inf')
	sum = 0
	for item in data:
		seat = parseSeatID(item)
		seatid = seatNumber(seat[0],seat[1])

		if seatid > max:
			max = seatid
		if seatid < min:
			min = seatid
		sum += seatid


	zero2min = (min-1) * ((min-1)+1) / 2
	zero2max = max * (max + 1) / 2
	min2max = zero2max - zero2min
	missing = min2max -sum

	return (sum, min, missing ,max)


if __name__ == "__main__":

	data = getdata('../boardingpasses.txt')

	(sum, min, missing ,max) = getMissingNum(data)

	print("max seat number is " + str(max))
	print("min seat number is " + str(min))
	print("missing seat number is " + str(missing))

