plugins {
 id("fabric-loom")
 java
}

group = property("maven_group")!!
version = property("mod_version")!!

repositories {
 
}

dependencies {
 minecraft("com.mojang:minecraft:${property("minecraft_version")}")
 mappings("net.fabricmc:yarn:${property("yarn_mappings")}:v2")
 modImplementation("net.fabricmc:fabric-loader:${property("loader_version")}")

 // modImplementation("net.fabricmc.fabric-api:fabric-api:${property("fabric_api_version")}")
}


tasks {
 processResources {
  inputs.property("version", project.version)
  filesMatching("fabric.mod.json") {
   expand(mutableMapOf("version" to project.version))
  }
 }

 jar {
  inputs.property("archivesName", project.base.archivesName)
 
  from("LICENSE")
 }
}

java {
 
}
