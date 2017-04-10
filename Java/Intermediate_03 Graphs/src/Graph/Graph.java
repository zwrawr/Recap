package Graph;

import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;

public class Graph {

	private List<Node> nodes;

	private HashMap<Node,List<Edge>> edges;
	
	public Graph() {
		this.nodes = new LinkedList<>();
		this.edges = new HashMap<Node,List<Edge>>();
	}

	public List<Node> getNodes() {
		return nodes;
	}
	
	public void addNode(float value){
		Node n = new Node(value);
		
		nodes.add(n);
		edges.put(n, null);
	}
	
	public void addNode(){
		Node n = new Node();
		
		nodes.add(n);
		edges.put(n, null);
	}
	
	public void removeNode(Node n){
		nodes.remove(n);
		
		for( Edge e :edges.get(n)){
			removeEdge(e);
		}
		
		if (edges.containsKey(n)){
			System.out.println("Removed all know edges from a node, but it was still in the edge map");
			edges.remove(n);
		}
	}
	
	public void removeEdge(Edge e){
		
		// Find the nodes that this edge connects
		Node a = e.getA();
		Node b = e.getB();
		
		// Remove this edge from their lists of edges
		edges.get(a).remove(e);
		edges.get(b).remove(e);
		
	}
	
	public void addEdge(Node a, Node b, float cost ){
		Edge e = new Edge(a, b, cost);
		
		edges.get(a).add(e);
		edges.get(b).add(e);
	}
	
	
	public void addEdge(Node a, Node b){
		Edge e = new Edge(a, b);
		
		edges.get(a).add(e);
		edges.get(b).add(e);
	}

	public boolean areAdjacent(Node a, Node b){
		return edges.get(a).contains(b);
	}
	
	public Node[] getNeighbours(Node a){
		int size = edges.get(a).size();
		
		Node[] neighbours = new Node[size];
		
		Edge[] e = (Edge[]) edges.get(a).toArray(); 
		
		for ( int i = 0; i < size; i++)
		{	
			neighbours[i] = (e[i].getA() == a) ? e[i].getA() : e[i].getB();
		}
		
		return neighbours;
	}
	
	public float getEdgeCost(Edge e)
	{
		return e.getCost();
	}
	
	public void setEdgeCost(Edge e, float cost)
	{
		e.setCost(cost);
	}
	
	public float getNodeValue(Node n)
	{
		return n.getValue();
	}
	
	public void setNodeValue(Node n, float value)
	{
		n.setValue(value);
	}
	//get_vertex_value(G, x): returns the value associated with the vertex x;
	//set_vertex_value(G, x, v): sets the value associated with the vertex x to v.

}
