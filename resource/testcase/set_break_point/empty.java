// javac -g empty.java
// java -agentpath:/path/librj_debug.so=config.json empty
/* Output:
[Breakpoint] getFull : 4
[Breakpoint] 1 : I
[Variable] 11
11
[Breakpoint] getEmpty : 18
[Breakpoint] 1 : I
[Variable] 12
12
*/

public class empty {
    public static int getFull(String s) {
        int len = s.length();
        if (len > 1) {
            return len;
        } else {
            return 0;
        }
    }

    public static int getEmpty(String s) {
        int len = s.length();
        if (len > 1) {
            len++;
        } else {
            len = 0;
        }
        return len;
       }

    public static void main(String[] args) {
        System.out.println(getFull("Hello world"));
        System.out.println(getEmpty("Hello world"));
    }
}