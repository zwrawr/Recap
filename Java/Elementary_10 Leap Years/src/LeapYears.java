import java.util.Calendar;

public class LeapYears {

	public static void main(String[] args) {
		
		// Get a calendar that default to using the system time and date
		Calendar cal = Calendar.getInstance();
		
		// Get the current year
		int year = cal.get(Calendar.YEAR);
		
		searchForLeapYears(20, year);
	}
	
	public static void searchForLeapYears(int num, int start){

		int i = 0, year = start;
		while (i < num){
			if (isLeapYear(year)){
				System.out.println(year + " is a leap year!");
				year += 4;
				i++;
			}
			else
			{
				System.out.println(year + " nope");
				year++;
			}
		}
	}
	
	// From pseudo code at https://en.wikipedia.org/wiki/Leap_year
	public static boolean isLeapYear(int year){
		if (year % 4 != 0) {
			return false;
		}
		else if (year % 100 != 0) {
			return true;
		}
		else if (year % 400 != 0) {
			return false;
		}
		else {
			return true;
		}
	}

}
