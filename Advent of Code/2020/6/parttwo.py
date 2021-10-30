def getdata(path):
	data = []
	with open(path) as my_file:
		l = []
		for line in my_file:
			if len(line.strip()) == 0:
				data.append(l)
				l = []
			else:
				l.append(set(line.strip().replace(' ', '')))
		data.append(l)
	return data

def getAllYes(data):
	ans = []
	for d in data:
		ans.append(d[0].intersection(*d))
	return ans

def sumofcounts(data):
	sum = 0
	for d in data:
		sum += len(d)
	return sum

if __name__ == "__main__":

	data = getdata('quiz.txt')

	allyes = getAllYes(data)

	sc = sumofcounts(allyes)

	print(sc)

