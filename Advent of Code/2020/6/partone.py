def getdata(path):
	data = []
	with open(path) as my_file:
		l = ''
		for line in my_file:
			if len(line.strip()) == 0:
				data.append(removedups(l.strip()))
				l = ''
			else:
				l += " " + line[0:-1]
		data.append(removedups(l.strip()))
	return data


def removedups(line):
	# well thats easy in python
	return set(line.replace(' ', ''))


def sumofcounts(data):
	sum = 0
	for d in data:
		sum += len(d)
	return sum

if __name__ == "__main__":

	data = getdata('quiz.txt')

	sc = sumofcounts(data)

	print(sc)

