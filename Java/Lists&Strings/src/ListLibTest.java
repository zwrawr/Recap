import static org.junit.Assert.*;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;
import java.util.function.Consumer;

import org.junit.Test;

public class ListLibTest {

	// Tests for ListLib.max_of
	@Test
	public void test_max_of_positve() {
		List<Integer> data = Arrays.asList(0, 8, 14, 3, 4, 2);
		assertEquals(14, (int)ListLib.max_of(data));
	}
	
	@Test
	public void test_max_of_negitives() {
		List<Integer> data = Arrays.asList(-1, -6, -3, - 11, -12);
		assertEquals(-1, (int)ListLib.max_of(data));
	}
	
	@Test
	public void test_max_of_strings() {
		List<String> data = Arrays.asList("a", "b", "bad", "egg", "eat");
		assertEquals("egg", ListLib.max_of(data));
	}
	
	// Tests for ListLib.reverse
	@Test
	public void test_reverse_odd() {
		Integer[] arr = {0, 1, 2, 3, 4, 5, 6};
		
		List<Integer> data = Arrays.asList(arr);
		
		Integer[] res = (Integer[])ListLib.reverse(data).toArray();

		assertArrayEquals(new Integer[]{6,5,4,3,2,1,0}, res);
	}
	
	@Test
	public void test_reverse_even() {
		Integer[] arr = {0, 1, 2, 3, 4, 5};

		List<Integer> data = Arrays.asList(arr);
		
		Integer[] res = (Integer[])ListLib.reverse(data).toArray();
		
		assertArrayEquals(new Integer[]{5,4,3,2,1,0}, res);
	}
	
	// Tests for ListLib.contains
	@Test
	public void test_contains_none(){
		List<Integer> data = Arrays.asList(-2, -1, 0, 1, 2);
		
		assertFalse(ListLib.contains(data,3));
	}
	
	@Test
	public void test_contains_one(){
		List<Integer> data = Arrays.asList(-2, -1, 0, 1, 2);
		
		assertTrue(ListLib.contains(data,-1));
	}
	
	@Test
	public void test_contains_many(){
		List<Integer> data = Arrays.asList(2, 1, 0, 1, 2);
		
		assertTrue(ListLib.contains(data,2));
	}
	
	// Test for ListLib.oddIndices_of
	@Test
	public void test_oddIndices_of_odd(){
		List<Integer> data = Arrays.asList(-2, -1, 0, 1, 2);
		assertArrayEquals(new Integer[]{-1,1},ListLib.oddIndices_of(data).toArray());
	}
	
	@Test
	public void test_oddIndices_of_even(){
		List<Integer> data = Arrays.asList(-2, -1, 0, 1, 2, 3);
		assertArrayEquals(new Integer[]{-1,1,3},ListLib.oddIndices_of(data).toArray());
	}
	
	@Test
	public void test_evenIndices_of_odd(){
		List<Integer> data = Arrays.asList(-2, -1, 0, 1, 2);
		assertArrayEquals(new Integer[]{-2,0,2},ListLib.evenIndices_of(data).toArray());
	}
	
	@Test
	public void test_evenIndices_of_even(){
		List<Integer> data = Arrays.asList(-2, -1, 0, 1, 2, 3);
		assertArrayEquals(new Integer[]{-2,0,2},ListLib.evenIndices_of(data).toArray());
	}
	
	// Test for ListLib.isPalindrome
	@Test
	public void test_isPalindrome_true(){
		List<Character> data = Arrays.asList('r','a','c','e','c','a','r');
		assertTrue(ListLib.isPalindrome(data));
	}
	
	@Test
	public void test_isPalindrome_false(){
		List<Character> data = Arrays.asList('a','v','a','c','a','r','d','o');
		assertFalse(ListLib.isPalindrome(data));
	}
	
	// Test for sum
	@Test
	public void test_sum_f(){
		List<Integer> data = Arrays.asList(-2, -1, 0, 1, 2, 3, 5, 6);
		Integer sum = 14;
		
		assertEquals(sum,ListLib.sum_f(data));
	}
	
	@Test
	public void test_sum_fe(){
		List<Integer> data = Arrays.asList(-2, -1, 0, 1, 2, 3, 5, 6);
		Integer sum = 14;
		
		assertEquals(sum,ListLib.sum_fe(data));
	}
	
	@Test
	public void test_sum_r(){
		List<Integer> data = Arrays.asList(-2, -1, 0, 1, 2, 3, 5, 6);
		Integer sum = 14;
		
		assertEquals(sum,ListLib.sum_r(data));
	}
	
	//Test on all
	// TODO: better testing of this function
	@Test
	public void test_on_all(){
		List<Integer> data = new ArrayList<Integer>();
		
		for (int i = 0; i < 20; i++){data.add(i*i);}
		
		Consumer<Integer> consumer = x -> System.out.println(x);
		ListLib.on_all(data, consumer);
	}
	
	//Test concat
	@Test
	public void test_concat(){
		List<Integer> a = Arrays.asList(-3,-2,-1, 0);
		List<Integer> b = Arrays.asList( 1, 2, 3);
		
		assertArrayEquals(new Integer[]{-3,-2,-1, 0, 1, 2, 3}, ListLib.concat(a,b).toArray());
	}
	
	//Test combine
	@Test
	public void test_combine(){
		List<Integer> a = Arrays.asList(-3,-2,-1, 0);
		List<Integer> b = Arrays.asList( 3, 2, 1);
		
		assertArrayEquals(new Integer[]{-3, 3,-2, 2,-1, 1, 0}, ListLib.combine(a,b).toArray());
	}
	
	//Test merge
	@Test
	public void test_merge(){
		List<Integer> a = Arrays.asList(-3,-1, 2);
		List<Integer> b = Arrays.asList(-2, 0, 1, 3);
		
		assertArrayEquals(new Integer[]{-3,-2,-1, 0, 1, 2, 3}, ListLib.merge(a,b).toArray());
	}
	
	// Test rotate
	@Test
	public void test_rotate(){
		// asList returns an structurally unmodifiable list so . . . this has be ugly.
		List<Integer> data = new LinkedList<Integer>(Arrays.asList(7, 8, 0, 1, 2, 3, 5, 6));
		
		assertArrayEquals(new Integer[]{0, 1, 2, 3, 5, 6, 7, 8}, ListLib.rotate(data,2).toArray());
	}
	
	//Test gen_fib
	@Test
	public void gen_fib(){
		List<Integer> fib = ListLib.gen_fib(10);
		
		assertArrayEquals(new Integer[]{0, 1, 1, 2, 3, 5, 8, 13, 21, 34}, fib.toArray());
	}
	
	// Test digitise
	@Test
	public void test_digitise(){
		assertArrayEquals(new Integer[]{5, 4, 3, 2, 1, 0}, ListLib.digitise(543210).toArray());
	}
	
	// Test dig_add
	@Test
	public void test_dig_add(){
		List<Integer> a = ListLib.digitise(4321);
		List<Integer> b = ListLib.digitise(9123);

		List<Integer> sum = ListLib.dig_add(a, b);
		
		assertArrayEquals(new Integer[]{1, 3, 4, 4, 4}, sum.toArray());

	}
	
	// Test dig_sub
	@Test
	public void test_dig_sub(){
		List<Integer> a = ListLib.digitise(5532);
		List<Integer> b = ListLib.digitise(1631);

		List<Integer> sub = ListLib.dig_sub(a, b);
		
		assertArrayEquals(new Integer[]{3, 9, 0, 1}, sub.toArray());

	}
	
	@Test(expected = ArithmeticException.class)
	public void test_dig_sub_neg(){
		List<Integer> a = ListLib.digitise(100);
		List<Integer> b = ListLib.digitise(101);

		ListLib.dig_sub(a, b);		

	}
	
	// Test dig_mul
	@Test
	public void test_dig_mul(){
		List<Integer> a = ListLib.digitise(1234);
		List<Integer> b = ListLib.digitise(9876);

		List<Integer> sub = ListLib.dig_mul(a, b);
		
		assertArrayEquals(new Integer[]{1, 2, 1, 8, 6, 9, 8, 4}, sub.toArray());

	}
	
	//Test SelectionSort
	@Test
	public void test_SelectionSort(){
		List<Integer> a = Arrays.asList(1, 8, 54, 3, 45, 2, 34, 3, 2, 32, 12, 675, 43, 23, 4, 1);
		ListLib.selectionSort(a);
		
		assertArrayEquals(new Integer[]{1, 1, 2, 2, 3, 3, 4, 8, 12, 23, 32, 34, 43, 45, 54, 675}, a.toArray());
	}
	
	//Test InsertionSort
	@Test
	public void test_InsertionSort(){
		List<Integer> a = Arrays.asList(1, 8, 54, 3, 45, 2, 34, 3, 2, 32, 12, 675, 43, 23, 4, 1);
		ListLib.insertionSort(a);
		
		assertArrayEquals(new Integer[]{1, 1, 2, 2, 3, 3, 4, 8, 12, 23, 32, 34, 43, 45, 54, 675}, a.toArray());
	}
	
	//Test MergeSort
	@Test
	public void test_MergeSort(){
		List<Integer> a = Arrays.asList(1, 8, 54, 3, 45, 2, 34, 3, 2, 32, 12, 675, 43, 23, 4, 1);
		a = ListLib.mergeSort(a);
		
		for(Integer s : a){System.out.println(s);}

		assertArrayEquals(new Integer[]{1, 1, 2, 2, 3, 3, 4, 8, 12, 23, 32, 34, 43, 45, 54, 675}, a.toArray());
	}
	
	//Test QuickSort
	@Test
	public void test_QuickSort(){
		List<Integer> a = Arrays.asList(1, 8, 54, 3, 45, 2, 34, 3, 2, 32, 12, 675, 43, 23, 4, 1);
		ListLib.quickSort(a);
		
		for(Integer s : a){System.out.println(s);}

		assertArrayEquals(new Integer[]{1, 1, 2, 2, 3, 3, 4, 8, 12, 23, 32, 34, 43, 45, 54, 675}, a.toArray());
	}
	
	//Test StoogeSort
	@Test
	public void test_StoogeSort(){
		List<Integer> a = Arrays.asList(1, 8, 54, 3, 45, 2, 34, 3, 2, 32, 12, 675, 43, 23, 4, 1);
		ListLib.stoogeSort(a);
		
		for(Integer s : a){System.out.println(s);}

		assertArrayEquals(new Integer[]{1, 1, 2, 2, 3, 3, 4, 8, 12, 23, 32, 34, 43, 45, 54, 675}, a.toArray());
	}
	
	//Test BinarySearch
	@Test
	public void test_BinarySearch(){
		List<Integer> a = Arrays.asList(1, 1, 2, 2, 3, 3, 4, 8, 12, 23, 32, 34, 43, 45, 54, 675);

		assertEquals(6,ListLib.search(a, 4));
	}
	
	//Test PrintWordBox
	@Test
	public void test_PrintWordBox(){
		List<String> a = Arrays.asList("Hello", "World","in","a","box");

		ListLib.printWordBox(a);
	}
	
	//Test EngToPig
	@Test
	public void test_EngToPig(){

		assertEquals("hetay uickqay rownbay oxfay", ListLib.EngToPig("the quick brown fox"));
	}
	
	//Test PigToEng
	@Test
	public void test_PigToEng(){

		assertEquals("the quick brown fox", ListLib.PigToEng("hetay uickqay rownbay oxfay"));
	}
}
