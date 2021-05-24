cargo build --release --bins
java -jar tests/brutalizer.jar -r "java -jar tests/spring.jar" -p1 "target/release/main" -p2 "target/release/beam" -t 4 -n 10 -l "./dist/logs/"