package com.baicie.student;

public class MethodDemo1 {
    public static void main(String[] args) {
        System.out.println("开始");
        int res = getMax(10, 20);
        System.out.println("结束" + res);
    }

    public static int getMax(int num1, int num2) {
        int max = Math.max(num2, num1);
        return max;
    }
}
