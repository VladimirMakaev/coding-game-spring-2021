cargo build --release --bins
java -jar tests/brutalizer.jar -r "java -jar tests/spring.jar" -p1 "target/release/beam" -p2 "target/release/beam" -t 8 -n 100 -l "./dist/logs/"