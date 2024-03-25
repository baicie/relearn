package com.baicie.flow;

public class DoWhile {
    public static void main(String[] args) {
        int i = 0;
        do {
            System.out.println(Math.random());
            System.out.println(i++);
        } while (i < 5);
    }
}
