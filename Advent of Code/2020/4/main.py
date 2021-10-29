def getdata(path):
	data = []
	with open(path) as my_file:
		l = ''
		for line in my_file:
			if len(line.strip()) == 0:
				data.append(parsepassport(l.strip()))
				l = ''
			else:
				l += " " + line[0:-1]
		data.append(parsepassport(l.strip()))
	return data

def parsepassport(passport):

	items = passport.split(' ')

	parsed = {}
	for item in items:
		kv = item.split(':')
		parsed[kv[0]] = kv[1]

	parsed
	return parsed

def byr_Valid(passport):
	if 'byr' in passport :
		y = passport['byr']
		if len(y) == 4:
			if 1920 <= int(y) <= 2002 :
				return True
	return False

def iyr_Valid(passport):
	if 'iyr' in passport :
		y = passport['iyr']
		if len(y) == 4:
			if 2010 <= int(y) <= 2020 :
				return True
	return False

def eyr_Valid(passport):
	if 'eyr' in passport :
		y = passport['eyr']
		if len(y) == 4:
			if 2020 <= int(y) <= 2030 :
				return True
	return False
				
def hgt_Valid(passport):
	if 'hgt' in passport :
		h = passport['hgt']
		if h[-2:] == 'cm':
			if 150 <= int(h[0:-2]) <= 193 :
				return True
		elif h[-2:] == 'in':
			if 59 <= int(h[0:-2]) <= 76 :
				return True
	return False

def hcl_Valid(passport):
	if 'hcl' in passport :
		c = passport['hcl']
		if c[0:1] == '#':
			if len(c[1:]) == 6:
				return True
	return False

def ecl_Valid(passport):
	if 'ecl' in passport :
		if passport['ecl'] in ['amb','blu','brn','gry','grn','hzl','oth']:
			return True
	return False

def pid_Valid(passport):
	if 'pid' in passport :
		if len(passport['pid']) == 9:
			return True
	return False

def isValid(passport):

	if (byr_Valid(passport) and 
		iyr_Valid(passport) and
		eyr_Valid(passport) and
		hgt_Valid(passport) and
		hcl_Valid(passport) and
		ecl_Valid(passport) and
		pid_Valid(passport)):
		return True


	return False

def checkPassports(data):
	
	total = 0
	for d in data:
		if isValid(d):
			total += 1

	return total

if __name__ == "__main__":

	data = getdata('../passports.txt')

	valid = checkPassports(data)

	print(valid)

