package Graph;

public class Edge {
	
	protected Node a;
	protected Node b;
	
	protected float cost;
	
	public Edge(Node a, Node b){
		
		this.setCost(1);
		
		this.a = a;
		this.b = b;
	}
	
	public Edge(Node a, Node b, float cost){
		
		this.setCost(cost);
		
		this.a = a;
		this.b = b;
	}
	
	public boolean isConnectedTo(Node n){
		if (this.a == n || this.b == n){
			return true;
		}
		return false;
	}
	
	public float getCost() {
		return cost;
	}

	public void setCost(float cost) {
		this.cost = cost;
	}

	public Node getA() {
		return a;
	}

	public Node getB() {
		return b;
	}
}
