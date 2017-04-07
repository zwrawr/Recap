import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;
import java.util.function.Consumer;

public class ListLib {
	
	
	// Returns the maximum value from a List
	public static <T extends Comparable<T>> T max_of(List<T> data){
		
		T max = null;
		
		for (T t : data){
			if (max == null){
				max = t;
			}
			else if( t.compareTo(max) > 0){
				max = t;
			}
		}
		return max;
	}
	
	// Reverses an array.
	public static <T> List<T> reverse(List<T> data){
		
		T tmp = null;
		int size = data.size();
		
		for(int i = 0; i < Math.floor(size/2f); i++){
			tmp = data.get(i);
			data.set(i, data.get(size - (i+1)));
			data.set(size - (i+1), tmp);
		}
		
		return data;
	}
	
	public static <T extends Comparable<T>> boolean contains(List<T> data, T a){
		
		// Should use data.contains(a); but that defeats the points of this exercise
		for(T t : data){
			if (a.equals(t)){
				return true;
			}
		}
		return false;
	}
	
	public static <T> List<T> oddIndices_of(List<T> data){
		List<T> odds = new ArrayList<T>();
		
		for(int i = 1; i < data.size(); i+=2){
			odds.add(data.get(i));
		}
		
		return odds;
	}
	
	public static <T> List<T> evenIndices_of(List<T> data){
		List<T> even = new ArrayList<T>();
		
		for(int i = 0; i < data.size(); i+=2){
			even.add(data.get(i));
		}
		
		return even;
	}
	
	// List<T>.size()
	
	// Tests to see weather a list is a palindrome
	public static <T extends Comparable<T>> boolean isPalindrome(List<T> data){
		
		int size = data.size();

		for(int i = 0; i < Math.floor(size/2f); i++){
			
			T a = data.get(i);
			T b = data.get(size - (i+1));
			
			if (!a.equals(b))
			{
				return false;
			}
		}
		return true;
	}
	
	//Sum up the values in a list
	public static Integer sum_f(List<Integer> data){
		/* The nature of java's generic, operator overloading and Number type make it impossible to
		 * make this function generic whilst still being reasonable. 
		 */
		Integer sum = 0;
		
		for(int i = 0; i < data.size(); i++){
			sum += data.get(i);
		}
		
		return sum;
	}
	
	public static Integer sum_fe(List<Integer> data){
		/* The nature of java's generic, operator overloading and Number type make it impossible to
		 * make this function generic whilst still being reasonable. 
		 */
		Integer sum = 0;
		
		for(Integer i : data){
			sum += i;
		}
		
		return sum;
	}
	
	public static Integer sum_r(List<Integer> data){
		/* The nature of java's generic, operator overloading and Number type make it impossible to
		 * make this function generic whilst still being reasonable. 
		 */

		return (data.size() == 1) ? data.get(0): data.get(0) + sum_r(data.subList(1, data.size()));
		
	}
	
	// runs a function on all items in a list
	public static <T> void on_all(List<T> data, Consumer<T> f){
		for(T t : data){
			f.accept(t);
		}
	}
	
	// concatenates two lists, [a,b,c], [1,2,3] → [a,b,c,1,2,3]
	public static <T> List<T> concat(List<T> a, List<T> b){
		List<T> data = new ArrayList<T>(a);
		
		data.addAll(b);
		
		return data;
	} 
	
	// combines two lists by alternately taking elements, e.g. [a,b,c], [1,2,3] → [a,1,b,2,c,3].
	public static <T> List<T> combine(List<T> a, List<T> b){
		List<T> data = new ArrayList<T>();
		
		int aCount = 0;
		int bCount = 0;
		
		while(aCount < a.size() || bCount < b.size()){
			if(aCount < a.size()){
				data.add(a.get(aCount++));
			}
			if(bCount < b.size()){
				data.add(b.get(bCount++));
			}
		}
		
		return data;
	} 
	
	// merges two sorted lists into a new sorted list. [1,4,6],[2,3,5] → [1,2,3,4,5,6]
	public static <T extends Comparable<T>> List<T> merge(List<T> a, List<T> b){
		List<T> data = new ArrayList<T>();
		
		int aCount = 0;
		int bCount = 0;
		
		while(aCount < a.size() && bCount < b.size()){
			
			T aValue = a.get(aCount);
			T bValue = b.get(bCount);

			if(aValue.compareTo(bValue) < 0){
				data.add(a.get(aCount++));
			}
			else{
				data.add(b.get(bCount++));
			}
		}
		
		// if there were more items in a than b
		if (aCount < a.size()){
			data.addAll(a.subList(aCount, a.size()));
		}
		
		// if there were more items in b than a
		if (bCount < b.size()){
			data.addAll(b.subList(bCount, b.size()));
		}
		
		return data;
	}
	
	// rotate a list by k elements
	public static <T> List<T> rotate(List<T> data, int k){
		
		// rotating by a multiple of k is the same as doing nothing
		k = k % data.size(); 
		if (k == 0){ return data;}
		
		/* I would use an array here to store tmp, but java's generics don't know
		 * their class at runtime so there isn't a 'proper' way to create generic arrays
		 * T tmp[] = new T[k];
		*/
		
		// grab a temp copy of the first k items
		List<T> tmp = new ArrayList<T>(data.subList(0, k));
		
		// remove k items from the beginning
		for(int i = 0; i < k; i++){
			data.remove(0);
		}
		
		// add the first k items to the end of the list 
		data.addAll(tmp);
		
		return data;
	}
	
	// generate fib numbers
	public static List<Integer> gen_fib(int num){
		
		List<Integer> fib = new LinkedList<Integer>();
		
		// Have to hard code the first two fib numbers
		fib.add(0);
		fib.add(1);
				
		for(int i = 2; i < num; i++){
			// the next fib number is the sum of the last two.
			fib.add(fib.get(i-1) + fib.get(i-2));
		}
		
		return fib;
	}
	
	// takes a number and returns a list of its digits in base 10
	public static List<Integer> digitise(int i){
		List<Integer> digits = new LinkedList<Integer>();

		int pow = 1;
		
		while (i > 0){
			
			int mask = (int)Math.pow(10, pow);
			int a = i % mask;
			i -= a;
			
			digits.add(a/(mask / 10));
			pow++;
		}
		
		return ListLib.reverse(digits);
	}
	
	public static List<Integer> dig_add(List<Integer> a, List<Integer> b){
		
		int aSize = a.size(), bSize = b.size(), base = 10;
		
		List<Integer> sum = new LinkedList<Integer>();
		
		boolean carry = false;
		
		int i = 0;
		for(i = Math.max(aSize, bSize) -1; i >= 0 ; i--){
			
			int tmp = (carry) ? 1 : 0;
			
			if (i < aSize){
				tmp += a.get(i);
			}
			if (i < bSize){
				tmp += b.get(i);
			}
			
			carry = ( tmp > base );
			sum.add( ((carry)?tmp-base:tmp));
		}
		
		if (carry){
			sum.add(1);
		}
		
		return ListLib.reverse(sum);
	}
	
	
	public static List<Integer> dig_sub(List<Integer> a, List<Integer> b){
		
		int aSize = a.size(), bSize = b.size(), base = 10;
		
		List<Integer> sum = new LinkedList<Integer>();
		
		boolean carry = false;
		
		int i = 0;
		for(i = Math.max(aSize, bSize) -1; i >= 0 ; i--){
			
			int tmp = (carry) ? -1 : 0;
			
			if (i < aSize){
				tmp += a.get(i);
			}
			if (i < bSize){
				tmp -= b.get(i);
			}
			
			carry = ( tmp < 0 );
			sum.add( ((carry)?tmp+base:tmp));
		}
		
		if (carry){
			throw new ArithmeticException("Result was negitive");
		}
		
		return ListLib.reverse(sum);
	}
	
	public static List<Integer> dig_mul(List<Integer> num1, List<Integer> num2){
	
		List<Integer> a = reverse(num1);
		List<Integer> b = reverse(num2);

		
		Integer[] result = new Integer[a.size() + b.size()];
		
		for (int r = 0; r < b.size(); r++) {
			int rDig = b.get(r);
			int tmp = 0;
			for (int l = 0; l < a.size(); l++) {
				tmp += (result[l+r] != null)?result[l+r]:0;
				tmp += rDig * a.get(l);
				result[l+r] = tmp % 10;
				tmp /= 10;
			}
			
			int dest = r + a.size();
			while(tmp !=0){
				tmp += (result[dest]!=null)?result[dest]:0;
				result[dest] = tmp % 10;
				tmp /= 10;
				dest++;
			}
		}
		
		return reverse(new ArrayList<Integer>(Arrays.asList(result)));
	}
	
	public static <T extends Comparable<T>> void selectionSort(List<T> data){
		int i, j, n = data.size();
		
		for(j = 0; j < n-1; j++){
			int min = j;
			
			for (i = j+1; i < n; i++) {

				if (data.get(i).compareTo(data.get(min)) < 0) {
		            min = i;
		        }
		    }

		    if(min != j) 
		    {
		        T tmp = data.get(j);
		        data.set(j, data.get(min));
		        data.set(min, tmp);
		    }
		}
	}
	
	public static <T extends Comparable<T>> void insertionSort(List<T> data){
		int i, j, n = data.size();
		
		for(i = 1; i < n; i++){
			T x = data.get(i);
			j = i - 1;
			
			while (j >= 0 && data.get(j).compareTo(x) > 0) {

				data.set(j+1, data.get(j));
				j--;
		    }

		    data.set(j+1, x);
		}
	}


	public static <T extends Comparable<T>> List<T> mergeSort(List<T> data){

		
		int size = data.size();
		
		if (size <= 1){
			return data;
		}
		
		int midpoint = size/2;
		
		List<T> left = data.subList(0, midpoint);
		List<T> right = data.subList(midpoint, size);
		
		left = mergeSort(left);
		right = mergeSort(right);
		
		return merge(left,right);
	}
	
	public static <T extends Comparable<T>> void quickSort(List<T> data){
		qSort(data, 1, data.size()-1);
	}
	
	private static <T extends Comparable<T>> void qSort(List<T> data, int lo, int hi){
		if(lo < hi){
			int p = partition(data, lo, hi);
			qSort(data,lo,p);
			qSort(data,p+1,hi);
		}
	}

	private static <T extends Comparable<T>> int partition(List<T> data, int lo, int hi){
		T piviot = data.get(lo);
		int i = lo -1 ;
		int j = hi + 1;
		
		while(true){
			do{
				i++;
			}while(data.get(i).compareTo(piviot) < 0);
			
			do{
				j--;
			}while(data.get(j).compareTo(piviot) > 0);
			
			if(i >= j){
				return j;
			}
			
			T tmp = data.get(i);
			data.set(i, data.get(j));
			data.set(j, tmp);
		}
	}

	public static <T extends Comparable<T>> void stoogeSort(List<T> data){
		int i = 0;
		int j = data.size()-1;
		
		sSort(data,i,j);
	}

	
	// never heard of stooge sort. Its suuuper inefficient O(n^2.7~)
	public static <T extends Comparable<T>> void sSort(List<T> data, int i, int j){

		if (data.get(j).compareTo(data.get(i)) < 0){
			T tmp = data.get(i);
			data.set(i, data.get(j));
			data.set(j, tmp);
		}
		
		if (j - i + 1 > 2){
			int t= (j - i + 1)/3;
			sSort(data, i, j-t);
			sSort(data, i+t, j);
			sSort(data, i, j-t);
		}
	}
	
	public static <T extends Comparable<T>> int search(List<T> data, T val){
		int left= 0, right = data.size();
		
		while (left <= right){
			
			int mid = left + (right - left)/2;
			T t = data.get(mid);
			
			if (t.equals(val)){
				return mid;
			}
			else if(t.compareTo(val)>0)
			{
				right = mid-1;
			}
			else if (t.compareTo(val)<0)
			{
				left = mid+1;
			}

		}
		
		return -1;
	}
}
