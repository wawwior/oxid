package me.wawwior.oxid;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import me.wawwior.oxid.jni.JNI;
import net.fabricmc.api.ModInitializer;

/**
 * Oxid
 */
public class Oxid implements ModInitializer {

  public static final String ID = "oxid";

  public static final Logger LOGGER = LoggerFactory.getLogger(ID);

  @Override
  public void onInitialize() {

    LOGGER.info("hello from oxid!");

    JNI.INSTANCE.main();
    
  }
  
}
