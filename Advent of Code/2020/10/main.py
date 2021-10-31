def getdata(path):
	data = []
	max = 0
	with open(path) as my_file:
		for line in my_file:
			data.append(int(line))
			max = int(line) if int(line) > max else max
	data.append(0) # for the 'mains'
	data.append(max+3) # for the device
	

	return data

def process(data):
	dist = {1:0,2:0,3:0}
	for x in range(1,len(data)):
		diff = data[x] - data[x-1]
		dist[diff] += 1
	
	return dist

def comb(data, pointer, cache):
	#print(str(pointer))

	if not pointer < len(data) : 
		return 1

	#if pointer in cache:
	#	return cache[pointer]

	curr = data[pointer]
	#print('\t'+str(pointer+1))
	c = comb(data, pointer+1, cache) # first one is garenteed

	#print([pointer,pointer+2,curr,curr+3])
	if (pointer + 2 < len(data)) and (data[pointer+2] <= curr + 3) :
		#print('\t'+str(pointer+2))
		c += comb(data, pointer+2, cache)

	if (pointer + 3 < len(data)) and (data[pointer+3] <= curr + 3) :
		#print('\t'+str(pointer+3))
		c += comb(data, pointer+3, cache)

	cache[pointer] = c

	return c

if __name__ == "__main__":

	data = getdata('adapters.txt')

	data.sort()

	dist = process(data)

	print(dist)
	print(str(dist[1]*dist[3]))

	comb = comb(data,0,{})
	print(comb)
