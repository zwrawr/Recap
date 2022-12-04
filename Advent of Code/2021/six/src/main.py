def getdata(path):
	data = []
	with open(path) as my_file:
		for line in my_file:
			for n in line.strip().split(','):
				data.append(int(n))
		
	return data

def simple_process(data):
	
	i = 0
	for i in range(0, len(data)):

		if data[i] == 0:
			data[i] = 8
			data.append(6)
		else:
			data[i] -= 1
	
	#print(data)
	print(len(data))


def smart_process(data,limit):
	
	buf = [0,0,0,0,0,0,0,0,0,0]

	for d in data:
		buf[d] += 1

	print("0", total(buf), buf)

	for i in range(1,limit+1):

		buf[7] += buf[0]
		buf[9] += buf[0]

		for x in range(1,len(buf)):
			buf[x-1] = buf[x]
		
		buf[len(buf)-1] = 0

		print(i, total(buf), buf)

def total (arr) :
	sum = 0

	for a in arr:
		sum += a
	
	return sum

if __name__ == "__main__":

	data = getdata('../input.txt')

	smart_process(data,256)


