test:
	cargo check
	cargo test -- --nocapture --color=always
	cargo clippy
	# cargo tarpaulin

after-develop-merged:
	git switch main
	git pull --prune
	git branch --delete develop
	git switch --create develop

clean:
	rm -r node_modules/
	rm -r target/

init:
	poetry install
	pre-commit install
	direnv allow

lint:
	pre-commit run --all-files

open:
	gh repo view --web

switch:
	git switch --create develop
