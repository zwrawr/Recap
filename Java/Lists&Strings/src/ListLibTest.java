import static org.junit.Assert.*;

import java.util.Arrays;
import java.util.List;
import org.junit.Test;

public class ListLibTest {

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

}
