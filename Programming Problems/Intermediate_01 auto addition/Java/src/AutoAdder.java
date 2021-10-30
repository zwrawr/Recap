
public class AutoAdder {

	private Integer[] numbers;
	private int target;
	private op[] pattern;
	private int result;
	
	private enum op{
		ADD,
		SUB,
		CONCAT
	}
	
	public static void main(String[] args) {
		
		AutoAdder AA = new AutoAdder();
		while(AA.incrementPattern()) {
			
			if (AA.TestPattern())
			{
				System.out.print(AA);
			}
	
		}
		
	}
	
	public AutoAdder(){
		numbers = new Integer[]{ 1, 2, 3, 4, 5, 6, 7, 8, 9};
		target = 100;
		pattern = null;
	}
	
	public boolean incrementPattern(){
				
		if (pattern == null){
			// this is the first pattern so we need to generate it
			
			pattern = new op[numbers.length-1];

			for (int i = 0; i < pattern.length; i++) {
				pattern[i] = op.ADD;
			}
			return true;
		}
		else
		{
			// need to increment the pattern, basically a base 3 increment
			for (int i = 0; i < pattern.length; i++) {
				boolean carry = false;

				switch(pattern[i])
				{
					case ADD:
						pattern[i] = op.SUB;
						break;
					case SUB:
						pattern[i] = op.CONCAT;
						break;
					case CONCAT:
						pattern[i] = op.ADD;
						carry = true;
						break;	
				}
				
				// If we don't need to change the next number we can exit early
				if (!carry){
					return true;
				}
			}
			
			// We've done every pattern
			return false;
		}
		
	}
	
	private boolean TestPattern(){
		
		Integer[] scratch = numbers.clone();
		
		// apply any concats first
		for(int i = 0; i < numbers.length-1; i++){
			if(pattern[i] == op.CONCAT){
				scratch[i] *= 10;
				scratch[i+1] += scratch[i];
				scratch[i] = 0;
			}
		}
		
		// Find the first item
		result = 0;
		int i = -1;
		while (result == 0){
			i++;
			result = scratch[i];
		}
		
		// Apply the operations to the values in the scratch pad
		for (i++; i < scratch.length; i++){

			// skip any empty elements
			
			switch(pattern[i-1]){
			case ADD:
				while(scratch[i] == 0){i++;}
				result += scratch[i];
				break;
			case SUB:
				while(scratch[i] == 0){i++;}
				result -= scratch[i];
				break;
			case CONCAT:
				break;
			}
			
		}
		
		// check result
		return (result == target);
	}

	@Override
	public String toString() {
		StringBuilder s = new StringBuilder();
		
		s.append(numbers[0]);
		
		for (int i = 1; i < numbers.length; i++) {
			
			switch(pattern[i-1]){
				case ADD:
					s.append(" + ");
					break;
				case SUB:
					s.append(" - ");
					break;
				case CONCAT:
					break;
			}
			s.append(numbers[i]);
		}
		
		s.append(" = " + result + "\n");
		
		return s.toString();
	}
	
	
}
