import java.io.IOException;

public class Greeting {

	public static void main(String[] args) {
		
		byte[] name;
		do{
			System.out.println("Hello, Whats your name?\t");
			name = getInput();
		}
		while (name == null);
		
		System.out.println("Hello, " + new String(name) + "!");
	}
	
	public static byte[] getInput()
	{
		byte[] name = new byte[64];
		
		int len = 0;
		try {
			len = System.in.read(name);
		} catch (IOException e) {
			System.out.println("Oops, something went wrong!");
			return null;
		}
		
		// on windows a new line is 2 chars
		if (len < 3){
			return null;
		}
		
		return name;
	}
}
