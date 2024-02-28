.PHONY: run

run:
	rm tmp/test.html
	cargo run > tmp/test.html
	open tmp/test.html
