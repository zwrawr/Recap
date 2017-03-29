
public class TimesTables {

	public static void main(String[] args) {
		printTimesTables(12,12);
	}
	
	public static void printTimesTables(int base, int num){
		for (int i = 1; i <= num; i++){
			System.out.println(base*i);
		}
	}

}
