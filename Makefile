.PHONY: run
part ?= 1

run: 
	cd $(day) && cargo run -- --input input.txt --part $(part)
