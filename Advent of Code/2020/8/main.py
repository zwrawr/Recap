from os import error


def getdata(path):
	data = []
	with open(path) as my_file:
		for line in my_file:
			parts = line.split(' ')

			data.append({
				"i" : parts[0].strip(),
				"n" : int(parts[1]),
				"v" : -1
			})
	return data

def run(data):
	pointer = 0
	acc = 0
	done = False
	while data[pointer]['v'] < 0 :
		data[pointer]['v'] = pointer

		match data[pointer]["i"] :
			case "nop":
				pass
			case "acc":
				acc += data[pointer]['n']
			case "jmp":
				pointer += data[pointer]['n'] - 1
			case _:
				error()

		pointer += 1

		if pointer == len(data):
			done = True
			break
	return (done,acc)

def findWrongI(data):
	
	pointer = 0

	while True:
		
		#reset data
		for d in data:
			d['v'] = -1

		match data[pointer]['i']:
			case 'acc':
				pass
			case 'jmp':
				data[pointer]['i'] = 'nop'
				print(data[pointer])
				(done,acc) = run(data)
				print([done,acc])
				if done : return acc
				data[pointer]['i'] = 'jmp'
			case 'nop':
				data[pointer]['i'] = 'jmp'
				print(data[pointer])
				(done,acc) = run(data)
				print([done,acc])
				if done : return acc
				data[pointer]['i'] = 'nop'

		pointer+=1

if __name__ == "__main__":

	data = getdata('program.txt')

	(done,acc) = run(data)
	print(acc)

	acc = findWrongI(data)
	print(acc)
