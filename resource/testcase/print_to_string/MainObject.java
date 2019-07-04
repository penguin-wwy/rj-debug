// javac -g MainObject.java
// java -agentpath:/path/librj_debug.so=config.json MainObject
/* Output:
[Breakpoint] run : 17
[Breakpoint] 0 : LMainObject;
[Variable] obj : MainObject
MainObject
*/

public class MainObject {
    @Override
    public String toString() {
        return "MainObject";
    }

    public static void run(MainObject obj) {
        System.out.println(obj);
    }

    public static void main(String[] args) {
        MainObject obj = new MainObject();
        run(obj);
    }
}