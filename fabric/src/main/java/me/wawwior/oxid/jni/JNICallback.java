
package me.wawwior.oxid.jni;

import java.util.function.Consumer;

/**
 * JNICallback
 */
public interface JNICallback {

  void error(String s);

  void warn(String s);
  
  void info(String s);

  void debug(String s);

  void trace(String s);
  
  static JNICallback from(
    Consumer<String> error,
    Consumer<String> warn,
    Consumer<String> info,
    Consumer<String> debug,
    Consumer<String> trace) {

    return new JNICallback() {

    	@Override
    	public void error(String s) {
    	  error.accept(s);
    	}

    	@Override
    	public void warn(String s) {
    	  warn.accept(s);
    	}

    	@Override
    	public void info(String s) {
    	  info.accept(s);
    	}

    	@Override
    	public void debug(String s) {
    	  debug.accept(s);
    	}

    	@Override
    	public void trace(String s) {
    	  trace.accept(s);
    	}
    
    };

  }

}
