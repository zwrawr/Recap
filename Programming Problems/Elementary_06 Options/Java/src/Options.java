import java.util.Scanner;

public class Options {

	public static void main(String[] args) {
		System.out.println("Enter a number");
	
		Scanner reader = new Scanner(System.in);
		int n = reader.nextInt();
		
		System.out.println("Enter [1] to sum or [2] multi");
		int s = reader.nextInt();
		
		int res = (s == 1) ? sumToZero(n) : productsToZero(n);
		
		System.out.println(res);
		reader.close();
	}

	public static int sumToZero(int n){
		int sum = 0;
		while (n > 0)
		{
			sum += n;
			n--;
		}
		return sum;
	}
	
	public static int productsToZero(int n){
		int prod = 1;
		while (n > 1)
		{
			prod *= n;
			n--;
		}
		return prod;
	}
}
