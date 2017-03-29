import java.util.Random;
import java.util.Scanner;
import java.util.ArrayList;
import java.util.List;

public class GuessingGame {

	static int bound = 100;
	
	public static void main(String[] args) {
		Random rand = new Random();
		int correct = rand.nextInt(bound);
		
		System.out.println("I've picked a number between 0 and 100, guess it");
		
		playGame(correct);
	}
	
	static void playGame(int correct)
	{
		
		Scanner reader = new Scanner(System.in);
		
		List<Integer> guesses = new ArrayList<Integer>();
		int guess;
		
		while(true)
		{
			guess = reader.nextInt();
			
			if (guess == correct){
				System.out.println("Correct, Yay! \nIt took you " + (guesses.size() + 1) + " guesses.");
				break;
			}
			
			if (!guesses.contains(guess))
			{
				if (guess > correct)
				{
					System.out.println("Too High");
				}
				else
				{
					System.out.println("Too Low");
				}
				guesses.add(guess);
			}
			else
			{
				System.out.println("You've already guessed that");
			}
			
		}
		
		reader.close();
	}
}
