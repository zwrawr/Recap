
public class Primes {

	public static void main(String[] args) {
		long l = 2;
		while (l < Long.MAX_VALUE){
			if (isPrime(l)){
				System.out.println(l + " is prime!");
			}
			l++;
		}
	}
	
	// Implementation of pseudo code from wikipedia
	// https://en.wikipedia.org/wiki/Primality_test
	static Boolean isPrime(long n){
		
	    if (n <= 1){
	        return false;
	    }
	    else if (n <= 3){
	        return true;
	    }
	    else if ((n % 2) == 0 || (n % 3) == 0){
	        return false;
	    }
	    
	    long i = 5;
	    
	    while ((i * i) <= n ){
	        if ((n % i) == 0 || (n % (i + 2)) == 0){
	            return false;
	        }
	        i += 6;
	    }
	    
	    return true;
	}
}
