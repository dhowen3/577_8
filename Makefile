program8: program8.rs
	rustc -C opt-level=3 -o program8 program8.rs

build: program8

run: program8
	./program8

test_run: program8
	./program8 < GivenTest-5.txt
