import java.util.Scanner;

public class Summing {

	public static void main(String[] args) {
		System.out.println("Enter an number");
		
		Scanner reader = new Scanner(System.in);
		int num = reader.nextInt();
		reader.close();
		
		int n = num;
		while (n > 0){
			System.out.println(num - --n);
		}
	}

}
