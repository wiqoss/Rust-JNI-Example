# Rust JNI Example

### Run:
~~~bash
./gradlew jar
cd src/main/rust && cargo build --release
for i in {1..3}; do cd ..; done
java -Djava.library.path=src/main/rust/target/release -jar build/libs/rust-example-1.0-SNAPSHOT.jar YourName YourAge
~~~