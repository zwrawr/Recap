import java.util.Scanner;

public class ExoPlanet {

	public static void main(String[] args){
		
		System.out.println("Enter the number of fractions days a year this plant has.\n>> ");
		
		Scanner Reader = new Scanner(System.in);
		double days =  Reader.nextDouble();
		Reader.close();
		
		ExoPlanet exo = new ExoPlanet(days);
		
		int yearsPerLeap = exo.calcYearsPer();
		
		String method = exo.addYear()? "add on" : "take off" ;
		System.out.println( "A planet with "+ days +
				" days a year, would need a to " + method + " an extra day every "+
				yearsPerLeap + " years" );
	}

	private double days;
	
	public ExoPlanet(double days) {
		this.days = days;
	}

	public int calcYearsPer(){
		
		int yearsPerLeap = 2;
		int daysPerYear = (int) Math.floor(this.days);
		
		while(yearsPerLeap*this.days <= yearsPerLeap*daysPerYear + 1){
			yearsPerLeap++;
		}
		
		return yearsPerLeap-1;
	}
	
	public boolean addYear(){
		if (Math.round(this.days)==Math.floor(this.days)){
			return true;
		}
		else{
			return false;
		}
	}
}
