program8: program8.rs
	rustc -o program8 program8.rs

build: program8

run: program8
	./program8

test_run:
	./program8 < GivenTest-5.txt
