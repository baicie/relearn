package com.baicie.obj;

public class Demo {
    public static void main(String[] args) {
        Student stu1 = new Student(1);
        stu1.name = "1";
        stu1.age = 2;
        Student stu2 = new Student(2);
        stu2.name = "2";
        stu2.age = 3;

        System.out.println(stu1.name);
        System.out.println(stu2.name);

        stu1.study();
        stu2.eat();
    }

}
