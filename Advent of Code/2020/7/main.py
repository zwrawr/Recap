class Node:

	def __init__(self, color):
		self.color = color
		self.edges = set()

	def addEdge(self, edge):
		self.edges.add(edge)

	def __str__(self):
		res = "Node[" + self.color + "]\tEdges[" + str(len(self.edges)) + "]("
		for edge in self.edges:
			res += str(edge) + ", "
		res += ")"
		return res


class Edge:

	bag=None
	num=None

	def __init__(self,bag,num):
		self.bag=bag
		self.num=num
	
	def __str__(self):
		return self.bag.color + " : " + str(self.num)

def loadRules(path):
	data = {}
	with open(path) as my_file:
		for line in my_file:
			rule = parseRule(line[0:-1])
			data[rule['color']] = rule
			x = line.count(',')
			if x +1 != len(rule['contains']):
				print(rule)
	return data

def parseRule(rule):
	
	res = {}

	parts = rule.split('bags contain')

	k = parts[0].strip()

	res['color'] = k
	res['contains'] = {}
	for c in parts[1].split(','):
		if 'no other bags' not in c:
			res['contains'][c.strip()[2:].split('bag')[0].strip()] = int(c.strip()[0:1])
	return res

def createNodes(rules):
	nodes = {}
	for rule in rules:
		nodes[rule] = Node(rule)
	return nodes

def connectNodes(nodes, rules):
	connections = 0
	for key in nodes:
		rule = rules[nodes[key].color]

		for subbag in rule['contains']:
			nodes[key].addEdge(Edge(nodes[subbag],rule['contains'][subbag]))
			connections += 1


	return connections
def traverseNode(node, callback):
	return traverseNodeP(node, -1,callback)

def traverseNodeP(node,depth, callback):
	depth+=1

	sum = 0
	if len(node.edges) > 0:
		for edge in node.edges:
			sum += edge.num * traverseNodeP(edge.bag, depth, callback)
	sum += 1
	callback(node, sum, depth)
	return sum

def printnode(node, sum, depth):

	prefix = ''
	for x in range(0,depth-1):
		prefix += " | "
	if depth > 0:
		prefix += " |_"
	print(prefix + "[" + str(sum) + "] " + str(node))

def countGolds(node, sum, depth):
	if node.color == 'shiny gold':
		globals()['shinygold']+=1

def fingAllGolds():
	z = 0
	for node in nodes:
		globals()['shinygold']=0
		traverseNode(nodes[node],countGolds)
		if globals()['shinygold'] != 0 :
			z += 1
	return z


shinygold = 0


if __name__ == "__main__":
	
	rules = loadRules('rules.txt')
	#print(str(len(rules)))

	nodes = createNodes(rules)
	#print(str(len(nodes)))

	c = connectNodes(nodes, rules)
	#print(c)

	bags = traverseNode(nodes['shiny gold'],printnode)
	print(str(bags -1 )) # Q. wants number of bag contained not number of bags

	found = fingAllGolds()
	print(found - 1) # Q. wants number of bag contained not number of bags


