import java.util.ArrayList;
import java.util.List;

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
	
}
