
RESOURCES = ./fabric/src/main/resources/natives

build: java

# JAVA

java: natives
	cd ./fabric && ./gradlew build

# NATIVES

natives: linux windows osx

## LINUX

linux: linux_64

linux_64: x86_64-unknown-linux-gnu
	mkdir -p $(RESOURCES)/linux_64
	cp ./native/target/x86_64-unknown-linux-gnu/release/libnative.so $(RESOURCES)/linux_64

x86_64-unknown-linux-gnu: 
	@cd ./native && cargo build --target=$@ --release

## WINDOWS

windows: windows_64 windows_arm64

windows_64: x86_64-pc-windows-msvc
	mkdir -p $(RESOURCES)/windows_64
	cp ./native/target/x86_64-pc-windows-msvc/release/native.dll $(RESOURCES)/windows_64

windows_arm64: aarch64-pc-windows-msvc
	mkdir -p $(RESOURCES)/windows_arm64
	cp ./native/target/aarch64-pc-windows-msvc/release/native.dll $(RESOURCES)/windows_arm64

x86_64-pc-windows-msvc aarch64-pc-windows-msvc:
	@cd ./native && cargo xwin build --target=$@ --release

## OSX

osx: osx_64 osx_arm64

osx_64: x86_64-apple-darwin
	mkdir -p $(RESOURCES)/osx_64
	cp ./native/target/x86_64-apple-darwin/release/libnative.dylib $(RESOURCES)/osx_64
	
osx_arm64: aarch64-apple-darwin
	mkdir -p $(RESOURCES)/osx_arm64
	cp ./native/target/aarch64-apple-darwin/release/libnative.dylib $(RESOURCES)/osx_arm64

x86_64-apple-darwin aarch64-apple-darwin:
	@cd ./native && cargo zigbuild --target=$@ --release

## CLEAN

clean:
	cd ./native && cargo clean
	cd ./macros && cargo clean
	cd ./fabric && ./gradlew clean
