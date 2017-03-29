import java.util.Scanner;

public class Multiples {

	public static void main(String[] args) {
		System.out.println("Enter an number");
		
		Scanner reader = new Scanner(System.in);
		int num = reader.nextInt();
		reader.close();
		
		int n = 0;
		while (n < num){
			n++;
			if ((n % 3) == 0 || (n % 5) == 0){
				System.out.println(n);
			}
		}
	}
}
