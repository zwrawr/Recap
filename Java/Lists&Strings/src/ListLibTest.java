import static org.junit.Assert.*;

import java.util.ArrayList;
import java.util.Arrays;
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
	
	@Test
	public void test_on_all(){
		List<Integer> data = new ArrayList<Integer>();
		
		for (int i = 0; i < 20; i++){data.add(i*i);}
		
		Consumer<Integer> consumer = x -> System.out.println(x);
		ListLib.on_all(data, consumer);
	}
}
