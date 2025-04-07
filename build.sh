set -eux
(cd native && cargo build --release --target=x86_64-unknown-linux-gnu)
cp ./native/target/x86_64-unknown-linux-gnu/release/libnative.so ./fabric/src/main/resources/natives/linux_64/
# (cd native && cargo build --release --target=aarch64-unknown-linux-gnu)
# cp ./native/target/aarch64-unknown-linux-gnu/release/native ./fabric/src/main/resources/aarch64-linux
(cd fabric && gradle build)
