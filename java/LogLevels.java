 public class LogLevels {
    
    public static String message(String logLine) {
        return logLine.split(":")[1].trim();
        //    logLine
    }

    public static String logLevel(String logLine) {
        System.out.println(logLine.split("["));
      return logLine.split("[")[0].split("]")[0].toLowerCase();
    }

    public static String reformat(String logLine) {
        throw new UnsupportedOperationException("Please implement the (static) LogLine.reformat() method");
    }
}
