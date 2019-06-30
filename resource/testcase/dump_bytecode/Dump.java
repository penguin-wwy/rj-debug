// javac -g empty.java
// java -agentpath:/path/librj_debug.so=config.json com.test.Dump
/* Output:
[method: <com.test.Dump.getLength:(Ljava/lang/String;)I>]:
"2A B6 00 02 AC"
Hello world
11
*/
package com.test;

public class Dump {
    public static int getLength(String s) {
        return s.length();
    }

    public static void main(String[] args) {
        System.out.println("Hello world");
        System.out.println(getLength("Hello world"));
    }
}