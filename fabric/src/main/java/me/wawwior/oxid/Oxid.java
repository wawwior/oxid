package me.wawwior.oxid;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import me.wawwior.oxid.jni.JNI;
import me.wawwior.oxid.jni.JNICallback;
import net.fabricmc.api.ModInitializer;

/**
 * Oxid
 */
public class Oxid implements ModInitializer {

  public static final String ID = "oxid";

  public static final Logger LOGGER = LoggerFactory.getLogger(ID);

  @Override
  public void onInitialize() {

    LOGGER.info("Java loaded!");

    JNI.INSTANCE.main(JNICallback.from(LOGGER::error, LOGGER::warn, LOGGER::info, LOGGER::debug, LOGGER::trace));
    
  }
  
}
