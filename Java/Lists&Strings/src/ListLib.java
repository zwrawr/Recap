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
}
