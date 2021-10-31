
def getdata(path):
	data = []
	with open(path) as my_file:
		for line in my_file:
			data.append(int(line))
	return data

def isValid(prev, cur):
	#print([cur,prev])
	for x in range(0,len(prev)):
		for y in range(0,len(prev)):
			if x == y : pass
			if prev[x] + prev[y] == cur:
				return True
	return False

def process(data,step):

	pointer = step+1
	while True:
		#print([pointer,data[pointer]])

		x = isValid(data[pointer-step:pointer],data[pointer])
		if x == True:
			pointer+=1
		else:
			break
	return pointer

def findWeekness(data, target):

	for x in range(0,len(data)):
		sum = 0

		max = 0
		min = float('inf')
		for y in range(x,len(data)):

			max = data[y] if data[y] > max else max
			min = data[y] if data[y] < min else min
			

			sum += data[y]
			#print([x,y,sum,target])

			if sum > target:
				break
			if sum == target:
				return (x,y,min,max)


if __name__ == "__main__":

	data = getdata('data.txt')

	res = process(data,25)
	print([res,data[res]])

	(x,y,min,max) = findWeekness(data, data[res])
	print(str(min+max))