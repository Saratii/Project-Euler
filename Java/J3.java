
public class J3 {
    public static long greatestPrime(long number) {
        long greatest = 0;
        long i = 1;
        while(true) {
            if(number%i == 0) {
                if(i>greatest) {
                    greatest = i;
                }
                number /= i;
            }
            i+=1;
            if(i>number) { 
                return greatest;
            }
        }   
    }
    public static void main(String[] args) {
        System.out.println(greatestPrime(600851475143L));
        System.out.println("end");
    }
}



