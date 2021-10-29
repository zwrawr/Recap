
def find2(data, sum):
	for x in range(0,len(data)):
		for y in range(0,len(data)):
			if (data[x] + data[y] == sum) :
				return {
					"x" : data[x],
					"y" : data[y]
				}
	return -1

def find3(data,sum):
	for z in range(0,len(data)):
		res = find2(data, sum-data[z])
		if res != -1 :
			res["z"] = data[z]
			return res

if __name__ == "__main__":
	
	data = []
	with open('../inputs.csv') as my_file:
		for line in my_file:
			data.append(int(line))
	
	res = find2(data, 2020)
	print(res["x"]*res["y"])

	res = find3(data, 2020)

	print(res["x"]*res["y"]*res["z"])

