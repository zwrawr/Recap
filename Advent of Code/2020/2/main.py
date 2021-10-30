def getdata(path):
	data = []
	with open(path) as my_file:
		for line in my_file:
			parts = line.split(' ')
			rng = parts[0].split('-')

			data.append({
				"min" : int(rng[0]),
				"max" : int(rng[1]),
				"letter" : parts[1][0:-1],
				"password" : parts[2][0:-1]
			})
	return data


def isValid_range(d):
	return (d['min'] <= d['password'].count(d['letter']) <= d['max'])

def isValid_pos(d):

	checks = ['min','max']
	found = 0

	for check in checks:
		if (d[check] - 1) < len(d['password']) and (d['password'][d[check] - 1] == d['letter']):
			found += 1
	return found == 1 


def countValid(data, f):
	count = 0
	for i in range(0,len(data)):
		if f(data[i]):
			count += 1
	return count

if __name__ == "__main__":

	data = getdata('passwords.txt')

	res = countValid(data, isValid_range)
	print(str(res) + " passwords are valid for part one using the range policy")

	res = countValid(data, isValid_pos)
	print(str(res) + " passwords are valid for part two using the position policy")
