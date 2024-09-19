package org.wiqs;

public class Main {

    static {
        System.loadLibrary("wiqs_rust");
    }

    public static native void run(Person person);

    public static void main(String[] args) {
        if (args.length != 2) {
            System.err.println("Required 2 args: name and age");
            System.exit(1);
        }

        try {
            Person person = new Person(args[0], Integer.parseInt(args[1]));
            run(person);
        } catch (NumberFormatException e) {
            System.err.println("Age must be an integer");
        }
    }
}
