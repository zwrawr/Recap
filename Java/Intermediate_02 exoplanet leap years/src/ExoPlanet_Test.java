import static org.junit.Assert.*;

import org.junit.Test;

public class ExoPlanet_Test {

	/*
	 * Lots of references for space stuff
	 * https://www.universetoday.com/37507/years-of-the-planets/
	 * https://en.wikipedia.org/wiki/Planet
	 * 
	 * Turns out no one seems to have complied a list of possible leap years
	 * for all the planets, or at least i couldn't find one.
	 */
	
	@Test
	public void test_calcYearsPer_a() {
		
		ExoPlanet exo = new ExoPlanet(100.25);
		assertEquals(4,exo.calcYearsPer());
	}
	
	@Test
	public void test_calcYearsPer_b() {
		
		ExoPlanet exo = new ExoPlanet(100.33);
		assertEquals(3,exo.calcYearsPer());
	}
	
	
	@Test
	public void test_calcYearsPer_c() {
		
		ExoPlanet exo = new ExoPlanet(100.001);
		assertEquals(1000,exo.calcYearsPer());
	}
	
	/*
	 * REAL PLANET TESTS . . .
	 */
	
	// NOTE: Mercury is weird its day is longer than its year
	
	// NOTE: a year on Venus is 1.92 venusian days
	
	// NOTE: a year on Earth is 365.2422 days
	@Test
	public void test_calcYearsPer_earth() {
		
		ExoPlanet exo = new ExoPlanet(365.2422);
		assertEquals(4,exo.calcYearsPer());
	}
	
	// NOTE: a year on mars is 668.5991 martian days
	// a proposed rule is if (year % 2 == 0 || year % 5 == 0)
	
	// NOTE: a year on Jupiter us 10,475.8 jovian days

	// NOTE: a year on Saturn is 24,491.07 Saturnian days
	@Test
	public void test_calcYearsPer_saturn() {
		
		ExoPlanet exo = new ExoPlanet(24491.07);
		assertEquals(14,exo.calcYearsPer());
	}
	
	// NOTE: a year on Uranus is 42,718 Uranian days
	
	// NOTE: a year on Neptune is 89,666 Neptunian days
}
