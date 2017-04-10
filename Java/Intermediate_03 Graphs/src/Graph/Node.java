package Graph;

public class Node {
	
	private float value;
	
	public Node() {
		this.setValue(1f);
	}
	
	public Node(float value){
		this.setValue(value);
	}

	public float getValue() {
		return value;
	}

	public void setValue(float value) {
		this.value = value;
	}

}
