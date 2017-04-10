
public class AutoAdder {

	private static int[] numbers = { 1, 2, 3, 4, 5, 6, 7, 8, 9};
	private static int target = 100;
	
	private enum op{
		ADD,
		SUB,
		CONCAT
	}
	
	public static void main(String[] args) {
		TestPattern(new op[]{op.ADD,op.SUB,op.CONCAT,op.SUB,op.ADD,op.ADD,op.SUB,op.CONCAT}, numbers, target);
	}
	
	private static boolean TestPattern(op[] ops, int[] nums, int tar){
		
		// make sure the inputs make sense
		if((ops.length+1) != nums.length){
			throw new IllegalArgumentException("Wrong input lengths");
		}
		
		// apply any concats first
		for(int i = nums.length -1; i > 0; i--){
			if(ops[i-1] == op.CONCAT){
				nums[i-1] *= 10;
				nums[i-1] += nums[i];
				nums[i] = 0;
			}
		}
		
		// do additions and subtractions
		int result = nums[0];
		for (int i = 1; i < nums.length; i++){
			
			op curr = ops[i-1];
			switch(curr){
			case ADD:
				result += nums[i];
				break;
			case SUB:
				result -= nums[i];
				break;
			case CONCAT:
				break;
			}
			
			System.out.println(result);
		}
		
		// check result
		return (result == tar);
	}

}
