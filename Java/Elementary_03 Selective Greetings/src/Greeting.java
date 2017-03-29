import java.io.IOException;
import java.util.Arrays;
import java.util.Scanner;

public class Greeting {

	public static void main(String[] args) {
		
		String name;
		do{
			System.out.println("Hello, Whats your name?\t");
			name = getInput();
		}
		while (name == null || name.length() > 1);
				
		if ( name.equals("Alice") || name.equals("Bob"))
		{
			System.out.println("Hello, " + name + "!");
		}
		else{
			System.out.println("I don't know you!");
		}
	}
	
	public static String getInput()
	{
		Scanner reader = new Scanner(System.in);
		String data = reader.next();
		reader.close();
		
		return data;
	}
}
