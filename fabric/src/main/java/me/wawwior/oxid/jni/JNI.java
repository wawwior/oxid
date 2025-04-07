package me.wawwior.oxid.jni;

import java.io.IOException;

import org.scijava.nativelib.NativeLoader;

import com.sun.jna.Platform;

/**
 * JNI
 */
public class JNI {

  public static final JNI INSTANCE = new JNI();

  public JNI() {
    if (!Platform.isLinux() | !Platform.isGNU()) throw new UnsupportedOperationException("Your OS is not supported!");

    try {
      NativeLoader.loadLibrary("native");
    } catch (IOException e) {
      throw new UnsupportedOperationException(String.format("Platform %s is not supported!", Platform.ARCH));
    }
  }
  
  public native void main();
  
}
