package com.baicie.oop;

public class Demo {
    public static void main(String[] args) {

    }

    static abstract class Fu {
        public abstract void demo();
    }

    public class Zi extends Fu {
        @Override
        public void demo() {
            System.out.println("zi1");
        }
    }

    public class Zi2 extends Fu {
        @Override
        public void demo() {
            System.out.println("zi2");
        }
    }
}
